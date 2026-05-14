use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::parser::{Stmt, Expr, FStringPart, BinOpKind, UnaryOpKind};

// ── Value ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    List(Rc<RefCell<Vec<Value>>>),
    Fn { params: Vec<String>, body: Vec<Stmt> },
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => {
                if *n == n.floor() && n.abs() < 1e15 { write!(f, "{}", *n as i64) }
                else { write!(f, "{:.6}", n) }
            }
            Value::Str(s)    => write!(f, "{}", s),
            Value::Bool(b)   => write!(f, "{}", b),
            Value::Nil       => write!(f, "none"),
            Value::Fn { .. } => write!(f, "<fn>"),
            Value::List(l)   => {
                let items: Vec<String> = l.borrow().iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", items.join(", "))
            }
        }
    }
}

pub(crate) enum Signal { Return(Value) }

// ── Standalone helpers (no self borrow) ─────────────────────────────────────

fn to_num(v: &Value) -> f64 {
    match v {
        Value::Number(n) => *n,
        Value::Bool(b)   => if *b { 1.0 } else { 0.0 },
        Value::Str(s)    => s.trim().parse().unwrap_or(0.0),
        _                => 0.0,
    }
}

fn is_truthy(v: &Value) -> bool {
    match v {
        Value::Bool(b)   => *b,
        Value::Nil       => false,
        Value::Number(n) => *n != 0.0,
        Value::Str(s)    => !s.is_empty(),
        Value::List(l)   => !l.borrow().is_empty(),
        _                => true,
    }
}

fn values_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => (x - y).abs() < 1e-12,
        (Value::Str(x),    Value::Str(y))    => x == y,
        (Value::Bool(x),   Value::Bool(y))   => x == y,
        (Value::Nil,       Value::Nil)        => true,
        _ => false,
    }
}

fn make_list(v: Vec<Value>) -> Value {
    Value::List(Rc::new(RefCell::new(v)))
}

fn format_val(v: &Value, spec: &str) -> String {
    let n = to_num(v);
    if let Some(inner) = spec.strip_prefix('.') {
        if let Some(rest) = inner.strip_suffix('f') {
            let prec: usize = rest.parse().unwrap_or(6);
            return format!("{:.prec$}", n, prec = prec);
        }
        if let Some(rest) = inner.strip_suffix('e') {
            let prec: usize = rest.parse().unwrap_or(6);
            return format!("{:.prec$e}", n, prec = prec);
        }
    }
    if spec == "d" || spec == "i" { return format!("{}", n as i64); }
    v.to_string()
}

// ── Interpreter ──────────────────────────────────────────────────────────────

pub struct Interpreter {
    scopes: Vec<HashMap<String, Value>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { scopes: vec![HashMap::new()] }
    }

    fn get(&self, name: &str) -> Value {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) { return v.clone(); }
        }
        panic!("Undefined variable: '{}'", name)
    }

    fn set(&mut self, name: String, value: Value) {
        self.scopes.last_mut().unwrap().insert(name, value);
    }

    fn set_existing(&mut self, name: &str, value: Value) {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) { scope.insert(name.to_string(), value); return; }
        }
        self.scopes.last_mut().unwrap().insert(name.to_string(), value);
    }

    fn push_scope(&mut self) { self.scopes.push(HashMap::new()); }
    fn pop_scope(&mut self)  { self.scopes.pop(); }

    pub fn exec(&mut self, stmts: &[Stmt]) -> Option<Signal> {
        for stmt in stmts {
            if let Some(s) = self.exec_stmt(stmt) { return Some(s); }
        }
        None
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> Option<Signal> {
        match stmt {
            Stmt::Let { name, value } => {
                let v = self.eval(value);
                self.set(name.clone(), v);
                None
            }
            Stmt::Assign { name, value } => {
                let v = self.eval(value);
                self.set_existing(name, v);
                None
            }
            Stmt::IndexAssign { name, index, value } => {
                let idx = self.eval(index);
                let val = self.eval(value);
                let list = self.get(name);
                match (&list, &idx) {
                    (Value::List(l), Value::Number(n)) => {
                        let i = *n as usize;
                        let mut items = l.borrow_mut();
                        if i < items.len() { items[i] = val; }
                        else { panic!("Index {} out of bounds (len {})", i, items.len()); }
                    }
                    _ => panic!("Cannot index-assign into non-list"),
                }
                None
            }
            Stmt::Expr(e) => { self.eval(e); None }
            Stmt::Return(e) => Some(Signal::Return(self.eval(e))),

            Stmt::Fn { name, params, body } => {
                self.set(name.clone(), Value::Fn { params: params.clone(), body: body.clone() });
                None
            }

            Stmt::If { cond, body, elif_branches, else_body } => {
                let cv = self.eval(cond);
                if is_truthy(&cv) { return self.exec(body); }
                for (ec, eb) in elif_branches {
                    let ev = self.eval(ec);
                    if is_truthy(&ev) { return self.exec(eb); }
                }
                if let Some(eb) = else_body { return self.exec(eb); }
                None
            }

            Stmt::While { cond, body } => {
                loop {
                    let cv = self.eval(cond);
                    if !is_truthy(&cv) { break; }
                    if let Some(s) = self.exec(body) { return Some(s); }
                }
                None
            }

            Stmt::For { var, iter, body } => {
                let iter_val = self.eval(iter);
                match iter_val {
                    Value::List(l) => {
                        let items: Vec<Value> = l.borrow().clone();
                        for item in items {
                            self.set(var.clone(), item);
                            if let Some(s) = self.exec(body) { return Some(s); }
                        }
                    }
                    _ => panic!("For loop requires a list (got {})", iter_val),
                }
                None
            }
        }
    }

    // ── Expression evaluation ─────────────────────────────────────────────────

    fn eval(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Number(n)  => Value::Number(*n),
            Expr::Str(s)     => Value::Str(s.clone()),
            Expr::Bool(b)    => Value::Bool(*b),
            Expr::Nil        => Value::Nil,
            Expr::Ident(n)   => self.get(n),

            Expr::List(items) => {
                let vals: Vec<Value> = items.iter().map(|e| self.eval(e)).collect();
                make_list(vals)
            }

            Expr::ListComp { expr, var, iter, cond } => {
                let iter_val = self.eval(iter);
                match iter_val {
                    Value::List(l) => {
                        let items: Vec<Value> = l.borrow().clone();
                        let mut result = Vec::new();
                        for item in items {
                            self.set(var.clone(), item);
                            if let Some(c) = cond {
                                let cv = self.eval(c);
                                if !is_truthy(&cv) { continue; }
                            }
                            result.push(self.eval(expr));
                        }
                        make_list(result)
                    }
                    _ => panic!("List comprehension requires a list"),
                }
            }

            Expr::Index { object, index } => {
                let obj = self.eval(object);
                let idx = self.eval(index);
                match (&obj, &idx) {
                    (Value::List(l), Value::Number(n)) => {
                        let items = l.borrow();
                        let i = *n as usize;
                        items.get(i).cloned().unwrap_or_else(|| panic!("Index {} out of bounds", i))
                    }
                    (Value::Str(s), Value::Number(n)) => {
                        let i = *n as usize;
                        let c = s.chars().nth(i).unwrap_or_else(|| panic!("Char index {} out of bounds", i));
                        Value::Str(c.to_string())
                    }
                    _ => panic!("Cannot index into {}", obj),
                }
            }

            Expr::FString(parts) => {
                let mut out = String::new();
                for part in parts {
                    match part {
                        FStringPart::Lit(s) => out.push_str(s),
                        FStringPart::Expr(e, fmt) => {
                            let v = self.eval(e);
                            let s = match fmt.as_deref() {
                                Some(spec) => format_val(&v, spec),
                                None => v.to_string(),
                            };
                            out.push_str(&s);
                        }
                    }
                }
                Value::Str(out)
            }

            Expr::BinOp { op, left, right } => {
                let l = self.eval(left);
                let r = self.eval(right);
                Self::apply_binop(op, l, r)
            }

            Expr::UnaryOp { op, expr } => {
                let v = self.eval(expr);
                match op {
                    UnaryOpKind::Neg => Value::Number(-to_num(&v)),
                    UnaryOpKind::Not => Value::Bool(!is_truthy(&v)),
                }
            }

            Expr::Call { name, args } => {
                let arg_vals: Vec<Value> = args.iter().map(|a| self.eval(a)).collect();
                self.call_builtin(name, arg_vals)
                    .unwrap_or_else(|av| self.call_user(name, av))
            }
        }
    }

    // ── Binary ops ───────────────────────────────────────────────────────────

    fn apply_binop(op: &BinOpKind, l: Value, r: Value) -> Value {
        match op {
            BinOpKind::Add => match (l, r) {
                (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                (Value::Str(a),    Value::Str(b))    => Value::Str(a + &b),
                (Value::Str(a),    b)                => Value::Str(a + &b.to_string()),
                (a,                Value::Str(b))    => Value::Str(a.to_string() + &b),
                (Value::List(a),   Value::List(b))   => {
                    let mut v = a.borrow().clone();
                    v.extend(b.borrow().iter().cloned());
                    make_list(v)
                }
                (a, b) => panic!("Cannot add {} and {}", a, b),
            },
            BinOpKind::Sub  => Value::Number(to_num(&l) - to_num(&r)),
            BinOpKind::Mul  => match (&l, &r) {
                (Value::List(lst), Value::Number(n)) => {
                    let items = lst.borrow().clone();
                    let n = *n as usize;
                    make_list(items.iter().cycle().take(items.len() * n).cloned().collect())
                }
                _ => Value::Number(to_num(&l) * to_num(&r)),
            },
            BinOpKind::Div  => Value::Number(to_num(&l) / to_num(&r)),
            BinOpKind::Mod  => Value::Number(to_num(&l) % to_num(&r)),
            BinOpKind::Pow  => Value::Number(to_num(&l).powf(to_num(&r))),
            BinOpKind::Eq    => Value::Bool(values_eq(&l, &r)),
            BinOpKind::NotEq => Value::Bool(!values_eq(&l, &r)),
            BinOpKind::Lt    => Value::Bool(to_num(&l) <  to_num(&r)),
            BinOpKind::Gt    => Value::Bool(to_num(&l) >  to_num(&r)),
            BinOpKind::LtEq  => Value::Bool(to_num(&l) <= to_num(&r)),
            BinOpKind::GtEq  => Value::Bool(to_num(&l) >= to_num(&r)),
            BinOpKind::And   => Value::Bool(is_truthy(&l) && is_truthy(&r)),
            BinOpKind::Or    => Value::Bool(is_truthy(&l) || is_truthy(&r)),
        }
    }

    // ── User function call ────────────────────────────────────────────────────

    fn call_user(&mut self, name: &str, args: Vec<Value>) -> Value {
        let func = self.get(name);
        match func {
            Value::Fn { params, body } => {
                self.push_scope();
                for (p, v) in params.iter().zip(args) { self.set(p.clone(), v); }
                let sig = self.exec(&body);
                self.pop_scope();
                match sig {
                    Some(Signal::Return(v)) => v,
                    None => Value::Nil,
                }
            }
            _ => panic!("'{}' is not a function", name),
        }
    }

    fn call_value(&mut self, func: &Value, args: Vec<Value>) -> Value {
        match func.clone() {
            Value::Fn { params, body } => {
                self.push_scope();
                for (p, v) in params.iter().zip(args) { self.set(p.clone(), v); }
                let sig = self.exec(&body);
                self.pop_scope();
                match sig {
                    Some(Signal::Return(v)) => v,
                    None => Value::Nil,
                }
            }
            _ => panic!("Value is not callable"),
        }
    }

    // ── Built-in functions ────────────────────────────────────────────────────
    // Returns Ok(Value) if handled, Err(args) to fall through to user fn.

    fn call_builtin(&mut self, name: &str, args: Vec<Value>) -> Result<Value, Vec<Value>> {
        let v = match name {

            // ── I/O ─────────────────────────────────────────────────────────
            "print" | "p" => {
                let parts: Vec<String> = args.iter().map(|v| v.to_string()).collect();
                println!("{}", parts.join(" "));
                Value::Nil
            }
            "input" => {
                use std::io::{self, Write, BufRead};
                if let Some(prompt) = args.first() { print!("{}", prompt); io::stdout().flush().ok(); }
                let mut line = String::new();
                io::stdin().lock().read_line(&mut line).ok();
                Value::Str(line.trim_end().to_string())
            }

            // ── Type conversion ──────────────────────────────────────────────
            "str"   => Value::Str(args.first().map(|v| v.to_string()).unwrap_or_default()),
            "int"   => Value::Number(args.first().map(|v| to_num(v).floor()).unwrap_or(0.0)),
            "float" => Value::Number(args.first().map(|v| to_num(v)).unwrap_or(0.0)),
            "bool"  => Value::Bool(args.first().map(|v| is_truthy(v)).unwrap_or(false)),
            "type"  => Value::Str(match args.first() {
                Some(Value::Number(_))  => "number",
                Some(Value::Str(_))     => "str",
                Some(Value::Bool(_))    => "bool",
                Some(Value::List(_))    => "list",
                Some(Value::Fn { .. })  => "fn",
                Some(Value::Nil) | None => "none",
            }.to_string()),

            // ── Math ─────────────────────────────────────────────────────────
            "sqrt"  => Value::Number(to_num(args.first().unwrap_or(&Value::Nil)).sqrt()),
            "cbrt"  => Value::Number(to_num(args.first().unwrap_or(&Value::Nil)).cbrt()),
            "exp"   => Value::Number(to_num(args.first().unwrap_or(&Value::Nil)).exp()),
            "log"   => {
                let x = to_num(args.get(0).unwrap_or(&Value::Nil));
                let base = args.get(1).map(|v| to_num(v)).unwrap_or(std::f64::consts::E);
                Value::Number(x.ln() / base.ln())
            }
            "log2"  => Value::Number(to_num(args.first().unwrap_or(&Value::Nil)).log2()),
            "log10" => Value::Number(to_num(args.first().unwrap_or(&Value::Nil)).log10()),
            "sin"   => Value::Number(to_num(args.first().unwrap_or(&Value::Nil)).sin()),
            "cos"   => Value::Number(to_num(args.first().unwrap_or(&Value::Nil)).cos()),
            "tan"   => Value::Number(to_num(args.first().unwrap_or(&Value::Nil)).tan()),
            "asin"  => Value::Number(to_num(args.first().unwrap_or(&Value::Nil)).asin()),
            "acos"  => Value::Number(to_num(args.first().unwrap_or(&Value::Nil)).acos()),
            "atan"  => Value::Number(to_num(args.first().unwrap_or(&Value::Nil)).atan()),
            "atan2" => Value::Number(to_num(args.get(0).unwrap_or(&Value::Nil)).atan2(to_num(args.get(1).unwrap_or(&Value::Nil)))),
            "pow"   => Value::Number(to_num(args.get(0).unwrap_or(&Value::Nil)).powf(to_num(args.get(1).unwrap_or(&Value::Nil)))),
            "abs"   => Value::Number(to_num(args.first().unwrap_or(&Value::Nil)).abs()),
            "floor" => Value::Number(to_num(args.first().unwrap_or(&Value::Nil)).floor()),
            "ceil"  => Value::Number(to_num(args.first().unwrap_or(&Value::Nil)).ceil()),
            "round" => {
                let n = to_num(args.get(0).unwrap_or(&Value::Nil));
                let p = args.get(1).map(|v| to_num(v)).unwrap_or(0.0) as i32;
                let factor = 10f64.powi(p);
                Value::Number((n * factor).round() / factor)
            }
            "sign"  => {
                let n = to_num(args.first().unwrap_or(&Value::Nil));
                Value::Number(n.signum())
            }
            "pi"    => Value::Number(std::f64::consts::PI),
            "e"     => Value::Number(std::f64::consts::E),
            "inf"   => Value::Number(f64::INFINITY),
            "nan"   => Value::Number(f64::NAN),
            "clamp" => {
                let n   = to_num(args.get(0).unwrap_or(&Value::Nil));
                let lo  = to_num(args.get(1).unwrap_or(&Value::Nil));
                let hi  = to_num(args.get(2).unwrap_or(&Value::Nil));
                Value::Number(n.clamp(lo, hi))
            }

            // ── AI / Activation functions ────────────────────────────────────
            "relu" => match args.first() {
                Some(Value::List(l)) => {
                    let v: Vec<Value> = l.borrow().iter().map(|x| Value::Number(to_num(x).max(0.0))).collect();
                    make_list(v)
                }
                Some(v) => Value::Number(to_num(v).max(0.0)),
                None => Value::Nil,
            },
            "leaky_relu" => {
                let x = to_num(args.get(0).unwrap_or(&Value::Nil));
                let alpha = args.get(1).map(|v| to_num(v)).unwrap_or(0.01);
                Value::Number(if x >= 0.0 { x } else { alpha * x })
            }
            "sigmoid" => match args.first() {
                Some(Value::List(l)) => {
                    let v: Vec<Value> = l.borrow().iter()
                        .map(|x| Value::Number(1.0 / (1.0 + (-to_num(x)).exp())))
                        .collect();
                    make_list(v)
                }
                Some(v) => Value::Number(1.0 / (1.0 + (-to_num(v)).exp())),
                None => Value::Nil,
            },
            "tanh" => match args.first() {
                Some(Value::List(l)) => {
                    let v: Vec<Value> = l.borrow().iter()
                        .map(|x| Value::Number(to_num(x).tanh()))
                        .collect();
                    make_list(v)
                }
                Some(v) => Value::Number(to_num(v).tanh()),
                None => Value::Nil,
            },
            "softmax" => {
                if let Some(Value::List(l)) = args.first() {
                    let items = l.borrow().clone();
                    let max_v = items.iter().map(|x| to_num(x)).fold(f64::NEG_INFINITY, f64::max);
                    let exps: Vec<f64> = items.iter().map(|x| (to_num(x) - max_v).exp()).collect();
                    let sum: f64 = exps.iter().sum();
                    make_list(exps.iter().map(|e| Value::Number(e / sum)).collect())
                } else { Value::Nil }
            }
            "gelu" => {
                let x = to_num(args.first().unwrap_or(&Value::Nil));
                Value::Number(0.5 * x * (1.0 + ((2.0 / std::f64::consts::PI).sqrt() * (x + 0.044715 * x.powi(3))).tanh()))
            }
            "selu" => {
                let x = to_num(args.first().unwrap_or(&Value::Nil));
                let alpha = 1.6732632423543772;
                let scale = 1.0507009873554805;
                Value::Number(scale * if x >= 0.0 { x } else { alpha * x.exp() - alpha })
            }

            // ── Vector operations ────────────────────────────────────────────
            "dot" => {
                match (args.get(0), args.get(1)) {
                    (Some(Value::List(a)), Some(Value::List(b))) => {
                        let av = a.borrow(); let bv = b.borrow();
                        let s: f64 = av.iter().zip(bv.iter()).map(|(x, y)| to_num(x) * to_num(y)).sum();
                        Value::Number(s)
                    }
                    _ => panic!("dot() expects two lists"),
                }
            }
            "vadd" | "vsub" | "vmul" | "vdiv" => {
                match (args.get(0), args.get(1)) {
                    (Some(Value::List(a)), Some(Value::List(b))) => {
                        let av = a.borrow(); let bv = b.borrow();
                        let v: Vec<Value> = av.iter().zip(bv.iter()).map(|(x, y)| {
                            let (xn, yn) = (to_num(x), to_num(y));
                            Value::Number(match name { "vadd" => xn+yn, "vsub" => xn-yn, "vmul" => xn*yn, _ => xn/yn })
                        }).collect();
                        make_list(v)
                    }
                    _ => panic!("{} expects two lists", name),
                }
            }
            "vscale" => {
                match (args.get(0), args.get(1)) {
                    (Some(Value::List(l)), Some(scalar)) => {
                        let s = to_num(scalar);
                        let v: Vec<Value> = l.borrow().iter().map(|x| Value::Number(to_num(x) * s)).collect();
                        make_list(v)
                    }
                    _ => panic!("vscale(list, scalar)"),
                }
            }
            "norm" => {
                if let Some(Value::List(l)) = args.first() {
                    let s: f64 = l.borrow().iter().map(|x| to_num(x).powi(2)).sum();
                    Value::Number(s.sqrt())
                } else { Value::Nil }
            }
            "normalize" => {
                if let Some(Value::List(l)) = args.first() {
                    let items = l.borrow().clone();
                    let n: f64 = items.iter().map(|x| to_num(x).powi(2)).sum::<f64>().sqrt();
                    if n == 0.0 { return Ok(make_list(items)); }
                    make_list(items.iter().map(|x| Value::Number(to_num(x) / n)).collect())
                } else { Value::Nil }
            }
            "cross" => {
                match (args.get(0), args.get(1)) {
                    (Some(Value::List(a)), Some(Value::List(b))) => {
                        let av = a.borrow(); let bv = b.borrow();
                        if av.len() != 3 || bv.len() != 3 { panic!("cross() needs 3D vectors"); }
                        let (a0,a1,a2) = (to_num(&av[0]), to_num(&av[1]), to_num(&av[2]));
                        let (b0,b1,b2) = (to_num(&bv[0]), to_num(&bv[1]), to_num(&bv[2]));
                        make_list(vec![
                            Value::Number(a1*b2 - a2*b1),
                            Value::Number(a2*b0 - a0*b2),
                            Value::Number(a0*b1 - a1*b0),
                        ])
                    }
                    _ => panic!("cross() expects two 3-element lists"),
                }
            }

            // ── Matrix operations ────────────────────────────────────────────
            "matmul" => {
                match (args.get(0), args.get(1)) {
                    (Some(Value::List(a)), Some(Value::List(b))) => {
                        let ar = a.borrow();
                        let br = b.borrow();
                        let rows_a = ar.len();
                        let cols_b = match &br[0] { Value::List(r) => r.borrow().len(), _ => panic!("matmul: B must be 2D") };
                        let cols_a = match &ar[0] { Value::List(r) => r.borrow().len(), _ => panic!("matmul: A must be 2D") };
                        let mut result = Vec::new();
                        for i in 0..rows_a {
                            let row_a = match &ar[i] { Value::List(r) => r.borrow().clone(), _ => panic!("matmul: row error") };
                            let mut row = Vec::new();
                            for j in 0..cols_b {
                                let mut s = 0.0f64;
                                for k in 0..cols_a {
                                    let bval = match &br[k] { Value::List(r) => r.borrow()[j].clone(), _ => panic!() };
                                    s += to_num(&row_a[k]) * to_num(&bval);
                                }
                                row.push(Value::Number(s));
                            }
                            result.push(make_list(row));
                        }
                        make_list(result)
                    }
                    _ => panic!("matmul(A, B) expects two 2D lists"),
                }
            }
            "transpose" => {
                if let Some(Value::List(m)) = args.first() {
                    let rows = m.borrow().clone();
                    if rows.is_empty() { return Ok(make_list(vec![])); }
                    let cols = match &rows[0] { Value::List(r) => r.borrow().len(), _ => panic!("transpose: 2D list required") };
                    let mut result = Vec::new();
                    for j in 0..cols {
                        let mut col = Vec::new();
                        for row in &rows {
                            if let Value::List(r) = row { col.push(r.borrow()[j].clone()); }
                        }
                        result.push(make_list(col));
                    }
                    make_list(result)
                } else { Value::Nil }
            }
            "identity" => {
                let n = to_num(args.first().unwrap_or(&Value::Nil)) as usize;
                let rows: Vec<Value> = (0..n).map(|i| {
                    make_list((0..n).map(|j| Value::Number(if i == j { 1.0 } else { 0.0 })).collect())
                }).collect();
                make_list(rows)
            }

            // ── Statistics ───────────────────────────────────────────────────
            "sum" => {
                if let Some(Value::List(l)) = args.first() {
                    Value::Number(l.borrow().iter().map(|v| to_num(v)).sum())
                } else { Value::Nil }
            }
            "mean" => {
                if let Some(Value::List(l)) = args.first() {
                    let items = l.borrow();
                    if items.is_empty() { return Ok(Value::Nil); }
                    Value::Number(items.iter().map(|v| to_num(v)).sum::<f64>() / items.len() as f64)
                } else { Value::Nil }
            }
            "std" | "stdev" => {
                if let Some(Value::List(l)) = args.first() {
                    let items = l.borrow();
                    if items.len() < 2 { return Ok(Value::Number(0.0)); }
                    let m = items.iter().map(|v| to_num(v)).sum::<f64>() / items.len() as f64;
                    let var = items.iter().map(|v| (to_num(v) - m).powi(2)).sum::<f64>() / items.len() as f64;
                    Value::Number(var.sqrt())
                } else { Value::Nil }
            }
            "variance" => {
                if let Some(Value::List(l)) = args.first() {
                    let items = l.borrow();
                    if items.is_empty() { return Ok(Value::Nil); }
                    let m = items.iter().map(|v| to_num(v)).sum::<f64>() / items.len() as f64;
                    Value::Number(items.iter().map(|v| (to_num(v) - m).powi(2)).sum::<f64>() / items.len() as f64)
                } else { Value::Nil }
            }
            "min" => {
                if let Some(Value::List(l)) = args.first() {
                    let v = l.borrow().iter().map(|v| to_num(v)).fold(f64::INFINITY, f64::min);
                    Value::Number(v)
                } else {
                    let a = to_num(args.get(0).unwrap_or(&Value::Nil));
                    let b = to_num(args.get(1).unwrap_or(&Value::Nil));
                    Value::Number(a.min(b))
                }
            }
            "max" => {
                if let Some(Value::List(l)) = args.first() {
                    let v = l.borrow().iter().map(|v| to_num(v)).fold(f64::NEG_INFINITY, f64::max);
                    Value::Number(v)
                } else {
                    let a = to_num(args.get(0).unwrap_or(&Value::Nil));
                    let b = to_num(args.get(1).unwrap_or(&Value::Nil));
                    Value::Number(a.max(b))
                }
            }
            "argmax" => {
                if let Some(Value::List(l)) = args.first() {
                    let items = l.borrow();
                    let idx = items.iter().enumerate()
                        .max_by(|a, b| to_num(a.1).partial_cmp(&to_num(b.1)).unwrap())
                        .map(|(i, _)| i).unwrap_or(0);
                    Value::Number(idx as f64)
                } else { Value::Nil }
            }
            "argmin" => {
                if let Some(Value::List(l)) = args.first() {
                    let items = l.borrow();
                    let idx = items.iter().enumerate()
                        .min_by(|a, b| to_num(a.1).partial_cmp(&to_num(b.1)).unwrap())
                        .map(|(i, _)| i).unwrap_or(0);
                    Value::Number(idx as f64)
                } else { Value::Nil }
            }
            "median" => {
                if let Some(Value::List(l)) = args.first() {
                    let mut vals: Vec<f64> = l.borrow().iter().map(|v| to_num(v)).collect();
                    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let n = vals.len();
                    if n == 0 { return Ok(Value::Nil); }
                    Value::Number(if n % 2 == 1 { vals[n/2] } else { (vals[n/2-1] + vals[n/2]) / 2.0 })
                } else { Value::Nil }
            }

            // ── List operations ──────────────────────────────────────────────
            "len" => match args.first() {
                Some(Value::List(l)) => Value::Number(l.borrow().len() as f64),
                Some(Value::Str(s))  => Value::Number(s.chars().count() as f64),
                _ => Value::Number(0.0),
            },
            "range" => {
                let nums: Vec<i64> = args.iter().map(|v| to_num(v) as i64).collect();
                let (start, end, step) = match nums.len() {
                    1 => (0, nums[0], 1),
                    2 => (nums[0], nums[1], 1),
                    3 => (nums[0], nums[1], nums[2]),
                    _ => panic!("range() takes 1-3 args"),
                };
                let mut v = Vec::new();
                let mut i = start;
                while if step > 0 { i < end } else { i > end } {
                    v.push(Value::Number(i as f64));
                    i += step;
                }
                make_list(v)
            }
            "append" => {
                if let (Some(Value::List(l)), Some(v)) = (args.get(0), args.get(1)) {
                    l.borrow_mut().push(v.clone());
                }
                Value::Nil
            }
            "pop" => {
                if let Some(Value::List(l)) = args.first() {
                    l.borrow_mut().pop().unwrap_or(Value::Nil)
                } else { Value::Nil }
            }
            "sort" => {
                if let Some(Value::List(l)) = args.first() {
                    let mut items = l.borrow().clone();
                    items.sort_by(|a, b| to_num(a).partial_cmp(&to_num(b)).unwrap_or(std::cmp::Ordering::Equal));
                    make_list(items)
                } else { Value::Nil }
            }
            "reverse" => {
                if let Some(Value::List(l)) = args.first() {
                    let mut items = l.borrow().clone();
                    items.reverse();
                    make_list(items)
                } else { Value::Nil }
            }
            "slice" => {
                if let Some(Value::List(l)) = args.get(0) {
                    let items = l.borrow().clone();
                    let lo = args.get(1).map(|v| to_num(v) as usize).unwrap_or(0);
                    let hi = args.get(2).map(|v| to_num(v) as usize).unwrap_or(items.len());
                    make_list(items[lo.min(items.len())..hi.min(items.len())].to_vec())
                } else { Value::Nil }
            }
            "flatten" => {
                if let Some(Value::List(l)) = args.first() {
                    let mut out = Vec::new();
                    for item in l.borrow().iter() {
                        match item {
                            Value::List(inner) => out.extend(inner.borrow().iter().cloned()),
                            v => out.push(v.clone()),
                        }
                    }
                    make_list(out)
                } else { Value::Nil }
            }
            "zip" => {
                match (args.get(0), args.get(1)) {
                    (Some(Value::List(a)), Some(Value::List(b))) => {
                        let av = a.borrow(); let bv = b.borrow();
                        let result: Vec<Value> = av.iter().zip(bv.iter())
                            .map(|(x, y)| make_list(vec![x.clone(), y.clone()]))
                            .collect();
                        make_list(result)
                    }
                    _ => panic!("zip() expects two lists"),
                }
            }
            "map" => {
                match (args.get(0), args.get(1)) {
                    (Some(Value::List(l)), Some(func)) => {
                        let items = l.borrow().clone();
                        let func = func.clone();
                        let mut result = Vec::new();
                        for item in items { result.push(self.call_value(&func, vec![item])); }
                        make_list(result)
                    }
                    _ => panic!("map(list, fn)"),
                }
            }
            "filter" => {
                match (args.get(0), args.get(1)) {
                    (Some(Value::List(l)), Some(func)) => {
                        let items = l.borrow().clone();
                        let func = func.clone();
                        let mut result = Vec::new();
                        for item in items {
                            let keep = self.call_value(&func, vec![item.clone()]);
                            if is_truthy(&keep) { result.push(item); }
                        }
                        make_list(result)
                    }
                    _ => panic!("filter(list, fn)"),
                }
            }
            "reduce" => {
                match (args.get(0), args.get(1), args.get(2)) {
                    (Some(Value::List(l)), Some(func), init) => {
                        let items = l.borrow().clone();
                        let func = func.clone();
                        let mut acc = init.cloned().unwrap_or_else(|| items.first().cloned().unwrap_or(Value::Nil));
                        let start = if init.is_none() { 1 } else { 0 };
                        for item in items.into_iter().skip(start) {
                            acc = self.call_value(&func, vec![acc, item]);
                        }
                        acc
                    }
                    _ => panic!("reduce(list, fn, ?initial)"),
                }
            }
            "enumerate" => {
                if let Some(Value::List(l)) = args.first() {
                    let result: Vec<Value> = l.borrow().iter().enumerate()
                        .map(|(i, v)| make_list(vec![Value::Number(i as f64), v.clone()]))
                        .collect();
                    make_list(result)
                } else { Value::Nil }
            }

            // ── Numpy-like ───────────────────────────────────────────────────
            "zeros" => {
                let n = to_num(args.get(0).unwrap_or(&Value::Nil)) as usize;
                let m = args.get(1).map(|v| to_num(v) as usize);
                if let Some(cols) = m {
                    make_list((0..n).map(|_| make_list(vec![Value::Number(0.0); cols])).collect())
                } else {
                    make_list(vec![Value::Number(0.0); n])
                }
            }
            "ones" => {
                let n = to_num(args.get(0).unwrap_or(&Value::Nil)) as usize;
                let m = args.get(1).map(|v| to_num(v) as usize);
                if let Some(cols) = m {
                    make_list((0..n).map(|_| make_list(vec![Value::Number(1.0); cols])).collect())
                } else {
                    make_list(vec![Value::Number(1.0); n])
                }
            }
            "linspace" => {
                let start = to_num(args.get(0).unwrap_or(&Value::Nil));
                let end   = to_num(args.get(1).unwrap_or(&Value::Nil));
                let n     = to_num(args.get(2).unwrap_or(&Value::Nil)) as usize;
                if n == 0 { return Ok(make_list(vec![])); }
                if n == 1 { return Ok(make_list(vec![Value::Number(start)])); }
                let step = (end - start) / (n - 1) as f64;
                make_list((0..n).map(|i| Value::Number(start + step * i as f64)).collect())
            }
            "arange" => {
                let start = to_num(args.get(0).unwrap_or(&Value::Nil));
                let end   = to_num(args.get(1).unwrap_or(&Value::Nil));
                let step  = args.get(2).map(|v| to_num(v)).unwrap_or(1.0);
                let mut v = Vec::new();
                let mut x = start;
                while if step > 0.0 { x < end } else { x > end } { v.push(Value::Number(x)); x += step; }
                make_list(v)
            }
            "reshape" => {
                if let (Some(Value::List(l)), Some(Value::Number(rows)), Some(Value::Number(cols))) =
                    (args.get(0), args.get(1), args.get(2))
                {
                    let flat = l.borrow().clone();
                    let rows = *rows as usize; let cols = *cols as usize;
                    if flat.len() != rows * cols { panic!("reshape: size mismatch {} != {}*{}", flat.len(), rows, cols); }
                    let result: Vec<Value> = (0..rows)
                        .map(|i| make_list(flat[i*cols..(i+1)*cols].to_vec()))
                        .collect();
                    make_list(result)
                } else { panic!("reshape(list, rows, cols)") }
            }

            // ── String functions ─────────────────────────────────────────────
            "upper"  => Value::Str(match args.first() { Some(Value::Str(s)) => s.to_uppercase(), _ => String::new() }),
            "lower"  => Value::Str(match args.first() { Some(Value::Str(s)) => s.to_lowercase(), _ => String::new() }),
            "trim"   => Value::Str(match args.first() { Some(Value::Str(s)) => s.trim().to_string(), _ => String::new() }),
            "split"  => {
                if let (Some(Value::Str(s)), Some(Value::Str(sep))) = (args.get(0), args.get(1)) {
                    make_list(s.split(sep.as_str()).map(|p| Value::Str(p.to_string())).collect())
                } else { make_list(vec![]) }
            }
            "join"   => {
                if let (Some(Value::Str(sep)), Some(Value::List(l))) = (args.get(0), args.get(1)) {
                    Value::Str(l.borrow().iter().map(|v| v.to_string()).collect::<Vec<_>>().join(sep))
                } else { Value::Nil }
            }
            "contains" => {
                match (args.get(0), args.get(1)) {
                    (Some(Value::Str(s)), Some(Value::Str(sub))) => Value::Bool(s.contains(sub.as_str())),
                    _ => Value::Bool(false),
                }
            }
            "replace" => {
                if let (Some(Value::Str(s)), Some(Value::Str(from)), Some(Value::Str(to))) =
                    (args.get(0), args.get(1), args.get(2))
                {
                    Value::Str(s.replace(from.as_str(), to.as_str()))
                } else { Value::Nil }
            }
            "startswith" => match (args.get(0), args.get(1)) {
                (Some(Value::Str(s)), Some(Value::Str(p))) => Value::Bool(s.starts_with(p.as_str())),
                _ => Value::Bool(false),
            },
            "endswith" => match (args.get(0), args.get(1)) {
                (Some(Value::Str(s)), Some(Value::Str(p))) => Value::Bool(s.ends_with(p.as_str())),
                _ => Value::Bool(false),
            },
            "char" => {
                let n = to_num(args.first().unwrap_or(&Value::Nil)) as u32;
                Value::Str(char::from_u32(n).map(|c| c.to_string()).unwrap_or_default())
            }

            // ── AI loss functions ────────────────────────────────────────────
            "mse" => {
                match (args.get(0), args.get(1)) {
                    (Some(Value::List(pred)), Some(Value::List(true_v))) => {
                        let p = pred.borrow(); let t = true_v.borrow();
                        let n = p.len() as f64;
                        let s: f64 = p.iter().zip(t.iter()).map(|(a, b)| (to_num(a) - to_num(b)).powi(2)).sum();
                        Value::Number(s / n)
                    }
                    _ => panic!("mse(predictions, targets)"),
                }
            }
            "cross_entropy" => {
                match (args.get(0), args.get(1)) {
                    (Some(Value::List(pred)), Some(Value::List(true_v))) => {
                        let p = pred.borrow(); let t = true_v.borrow();
                        let s: f64 = t.iter().zip(p.iter()).map(|(y, yhat)| {
                            let yhat_n = to_num(yhat).clamp(1e-12, 1.0 - 1e-12);
                            -to_num(y) * yhat_n.ln()
                        }).sum();
                        Value::Number(s)
                    }
                    _ => panic!("cross_entropy(predictions, targets)"),
                }
            }
            "one_hot" => {
                let idx   = to_num(args.get(0).unwrap_or(&Value::Nil)) as usize;
                let depth = to_num(args.get(1).unwrap_or(&Value::Nil)) as usize;
                make_list((0..depth).map(|i| Value::Number(if i == idx { 1.0 } else { 0.0 })).collect())
            }

            _ => return Err(args),
        };
        Ok(v)
    }
}

pub fn run(stmts: Vec<Stmt>) {
    Interpreter::new().exec(&stmts);
}
