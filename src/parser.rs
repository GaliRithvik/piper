use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum BinOpKind {
    Add, Sub, Mul, Div, Mod, Pow,
    Eq, NotEq, Lt, Gt, LtEq, GtEq,
    And, Or,
    In, NotIn,
}

#[derive(Debug, Clone)]
pub enum UnaryOpKind { Neg, Not }

#[derive(Debug, Clone)]
pub enum FStringPart {
    Lit(String),
    Expr(Expr, Option<String>),   // expr, optional format spec (e.g. ".4f")
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    Ident(String),
    List(Vec<Expr>),
    Dict(Vec<(Expr, Expr)>),
    ListComp { expr: Box<Expr>, var: String, iter: Box<Expr>, cond: Option<Box<Expr>> },
    Index { object: Box<Expr>, index: Box<Expr> },
    Slice { object: Box<Expr>, start: Option<Box<Expr>>, end: Option<Box<Expr>> },
    Attribute { object: Box<Expr>, field: String },
    MethodCall { object: Box<Expr>, method: String, args: Vec<Expr> },
    FString(Vec<FStringPart>),
    BinOp { op: BinOpKind, left: Box<Expr>, right: Box<Expr> },
    UnaryOp { op: UnaryOpKind, expr: Box<Expr> },
    Call { name: String, args: Vec<Expr> },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let        { name: String, value: Expr },
    LetTuple   { names: Vec<String>, value: Expr },
    Assign     { name: String, value: Expr },
    IndexAssign{ name: String, index: Expr, value: Expr },
    If {
        cond: Expr,
        body: Vec<Stmt>,
        elif_branches: Vec<(Expr, Vec<Stmt>)>,
        else_body: Option<Vec<Stmt>>,
    },
    While  { cond: Expr, body: Vec<Stmt> },
    For    { var: String, iter: Expr, body: Vec<Stmt> },
    Fn     { name: String, params: Vec<(String, Option<Expr>)>, body: Vec<Stmt> },
    Return(Expr),
    Break,
    Continue,
    Try    { body: Vec<Stmt>, except_var: Option<String>, handler: Vec<Stmt> },
    Case     { expr: Expr, branches: Vec<(Expr, Vec<Stmt>)>, else_body: Option<Vec<Stmt>> },
    ClassDef { name: String, methods: Vec<Stmt> },
    SetAttr  { object: Expr, field: String, value: Expr },
    Expr(Expr),
}

// ── Parser ──────────────────────────────────────────────────────────────────

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self { Parser { tokens, pos: 0 } }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn peek2(&self) -> &Token {
        self.tokens.get(self.pos + 1).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens.get(self.pos).cloned().unwrap_or(Token::EOF);
        self.pos += 1;
        t
    }

    fn expect(&mut self, expected: &Token) {
        let t = self.advance();
        if &t != expected {
            panic!("Expected {:?}, got {:?}", expected, t);
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek(), Token::Newline) { self.advance(); }
    }

    // ── Top-level ────────────────────────────────────────────────────────────

    fn parse_program(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !matches!(self.peek(), Token::EOF) {
            stmts.push(self.parse_stmt());
            self.skip_newlines();
        }
        stmts
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.expect(&Token::Colon);
        // Single-line block: if x: stmt  or  fn f(x): return y
        if !matches!(self.peek(), Token::Newline | Token::EOF) {
            let stmt = self.parse_stmt();
            return vec![stmt];
        }
        self.expect(&Token::Newline);
        self.expect(&Token::Indent);
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !matches!(self.peek(), Token::Dedent | Token::EOF) {
            stmts.push(self.parse_stmt());
            self.skip_newlines();
        }
        if matches!(self.peek(), Token::Dedent) { self.advance(); }
        stmts
    }

    // ── Statements ───────────────────────────────────────────────────────────

    fn parse_stmt(&mut self) -> Stmt {
        match self.peek().clone() {
            Token::Let      => self.parse_let(),
            Token::Fn       => self.parse_fn(),
            Token::Return   => self.parse_return(),
            Token::If       => self.parse_if(),
            Token::While    => self.parse_while(),
            Token::For      => self.parse_for(),
            Token::Try      => self.parse_try(),
            Token::Case     => self.parse_case(),
            Token::Class    => self.parse_class(),
            Token::Break    => {
                self.advance();
                if matches!(self.peek(), Token::Newline) { self.advance(); }
                Stmt::Break
            }
            Token::Continue => {
                self.advance();
                if matches!(self.peek(), Token::Newline) { self.advance(); }
                Stmt::Continue
            }
            Token::Ident(_) => self.parse_ident_stmt(),
            _ => {
                let e = self.parse_expr();
                if matches!(self.peek(), Token::Newline) { self.advance(); }
                Stmt::Expr(e)
            }
        }
    }

    fn parse_let(&mut self) -> Stmt {
        self.advance(); // consume 'let'
        // Tuple destructuring: let (a, b) = expr
        if matches!(self.peek(), Token::LParen) {
            self.advance(); // consume '('
            let mut names = Vec::new();
            while !matches!(self.peek(), Token::RParen | Token::EOF) {
                if let Token::Ident(n) = self.advance() { names.push(n); }
                if matches!(self.peek(), Token::Comma) { self.advance(); }
            }
            self.expect(&Token::RParen);
            self.expect(&Token::Assign);
            let value = self.parse_expr();
            if matches!(self.peek(), Token::Newline) { self.advance(); }
            return Stmt::LetTuple { names, value };
        }
        let name = match self.advance() {
            Token::Ident(n) => n,
            t => panic!("Expected name after 'let', got {:?}", t),
        };
        self.expect(&Token::Assign);
        let value = self.parse_expr();
        if matches!(self.peek(), Token::Newline) { self.advance(); }
        Stmt::Let { name, value }
    }

    fn parse_fn(&mut self) -> Stmt {
        self.advance();
        let name = match self.advance() {
            Token::Ident(n) => n,
            t => panic!("Expected function name, got {:?}", t),
        };
        self.expect(&Token::LParen);
        let mut params = Vec::new();
        while !matches!(self.peek(), Token::RParen | Token::EOF) {
            if let Token::Ident(p) = self.advance() {
                let default = if matches!(self.peek(), Token::Assign) {
                    self.advance();
                    Some(self.parse_or())
                } else {
                    None
                };
                params.push((p, default));
            }
            if matches!(self.peek(), Token::Comma) { self.advance(); }
        }
        self.expect(&Token::RParen);
        let body = self.parse_block();
        Stmt::Fn { name, params, body }
    }

    fn parse_return(&mut self) -> Stmt {
        self.advance();
        let expr = if matches!(self.peek(), Token::Newline | Token::EOF) {
            Expr::Nil
        } else {
            self.parse_expr()
        };
        if matches!(self.peek(), Token::Newline) { self.advance(); }
        Stmt::Return(expr)
    }

    fn parse_if(&mut self) -> Stmt {
        self.advance();
        let cond = self.parse_expr();
        let body = self.parse_block();
        let mut elif_branches = Vec::new();
        let mut else_body = None;
        loop {
            self.skip_newlines();
            if matches!(self.peek(), Token::Elif) {
                self.advance();
                let ec = self.parse_expr();
                let eb = self.parse_block();
                elif_branches.push((ec, eb));
            } else if matches!(self.peek(), Token::Else) {
                self.advance();
                else_body = Some(self.parse_block());
                break;
            } else {
                break;
            }
        }
        Stmt::If { cond, body, elif_branches, else_body }
    }

    fn parse_while(&mut self) -> Stmt {
        self.advance();
        let cond = self.parse_expr();
        let body = self.parse_block();
        Stmt::While { cond, body }
    }

    fn parse_for(&mut self) -> Stmt {
        self.advance();
        let var = match self.advance() {
            Token::Ident(n) => n,
            t => panic!("Expected variable in for, got {:?}", t),
        };
        self.expect(&Token::In);
        let iter = self.parse_expr();
        let body = self.parse_block();
        Stmt::For { var, iter, body }
    }

    fn parse_try(&mut self) -> Stmt {
        self.advance(); // consume 'try'
        let body = self.parse_block();
        self.skip_newlines();
        if !matches!(self.peek(), Token::Except) {
            panic!("Expected 'except' after try block");
        }
        self.advance(); // consume 'except'
        let except_var = if let Token::Ident(n) = self.peek().clone() {
            self.advance();
            Some(n)
        } else {
            None
        };
        let handler = self.parse_block();
        Stmt::Try { body, except_var, handler }
    }

    fn parse_case(&mut self) -> Stmt {
        self.advance(); // consume 'case'
        let expr = self.parse_expr();
        self.expect(&Token::Colon);
        if matches!(self.peek(), Token::Newline) { self.advance(); }
        self.expect(&Token::Indent);
        self.skip_newlines();
        let mut branches: Vec<(Expr, Vec<Stmt>)> = Vec::new();
        let mut else_body: Option<Vec<Stmt>> = None;
        while !matches!(self.peek(), Token::Dedent | Token::EOF) {
            if matches!(self.peek(), Token::Of) {
                self.advance(); // consume 'of'
                let pattern = self.parse_expr();
                let body = self.parse_block();
                branches.push((pattern, body));
            } else if matches!(self.peek(), Token::Else) {
                self.advance(); // consume 'else'
                else_body = Some(self.parse_block());
                self.skip_newlines();
                break;
            } else {
                break;
            }
            self.skip_newlines();
        }
        if matches!(self.peek(), Token::Dedent) { self.advance(); }
        Stmt::Case { expr, branches, else_body }
    }

    fn parse_class(&mut self) -> Stmt {
        self.advance(); // consume 'class'
        let name = match self.advance() {
            Token::Ident(n) => n,
            t => panic!("Expected class name, got {:?}", t),
        };
        self.expect(&Token::Colon);
        if matches!(self.peek(), Token::Newline) { self.advance(); }
        self.expect(&Token::Indent);
        self.skip_newlines();
        let mut methods = Vec::new();
        while !matches!(self.peek(), Token::Dedent | Token::EOF) {
            methods.push(self.parse_fn());
            self.skip_newlines();
        }
        if matches!(self.peek(), Token::Dedent) { self.advance(); }
        Stmt::ClassDef { name, methods }
    }

    fn parse_ident_stmt(&mut self) -> Stmt {
        let name = match self.peek().clone() {
            Token::Ident(n) => n,
            _ => unreachable!(),
        };

        // name[index] = value  OR  name[start:end]
        if matches!(self.peek2(), Token::LBracket) {
            self.advance(); // consume name
            self.advance(); // consume [
            if matches!(self.peek(), Token::Colon) {
                self.advance(); // consume :
                let end = if matches!(self.peek(), Token::RBracket) { None }
                          else { Some(Box::new(self.parse_expr())) };
                self.expect(&Token::RBracket);
                let obj = Expr::Slice { object: Box::new(Expr::Ident(name)), start: None, end };
                if matches!(self.peek(), Token::Newline) { self.advance(); }
                return Stmt::Expr(obj);
            }
            let index = self.parse_expr();
            if matches!(self.peek(), Token::Colon) {
                self.advance(); // consume :
                let end = if matches!(self.peek(), Token::RBracket) { None }
                          else { Some(Box::new(self.parse_expr())) };
                self.expect(&Token::RBracket);
                let obj = Expr::Slice { object: Box::new(Expr::Ident(name)), start: Some(Box::new(index)), end };
                if matches!(self.peek(), Token::Newline) { self.advance(); }
                return Stmt::Expr(obj);
            }
            self.expect(&Token::RBracket);
            if matches!(self.peek(), Token::Assign) {
                self.advance();
                let value = self.parse_expr();
                if matches!(self.peek(), Token::Newline) { self.advance(); }
                return Stmt::IndexAssign { name, index, value };
            }
            // Not an assignment; wrap back as expression (index access)
            let obj = Expr::Index { object: Box::new(Expr::Ident(name)), index: Box::new(index) };
            if matches!(self.peek(), Token::Newline) { self.advance(); }
            return Stmt::Expr(obj);
        }

        // Augmented assignment: name += / -= / *= / /=
        match self.peek2().clone() {
            Token::PlusAssign | Token::MinusAssign |
            Token::StarAssign | Token::SlashAssign => {
                self.advance(); // name
                let op = self.advance();
                let rhs = self.parse_expr();
                let binop = match op {
                    Token::PlusAssign  => BinOpKind::Add,
                    Token::MinusAssign => BinOpKind::Sub,
                    Token::StarAssign  => BinOpKind::Mul,
                    Token::SlashAssign => BinOpKind::Div,
                    _ => unreachable!(),
                };
                let value = Expr::BinOp {
                    op: binop,
                    left: Box::new(Expr::Ident(name.clone())),
                    right: Box::new(rhs),
                };
                if matches!(self.peek(), Token::Newline) { self.advance(); }
                return Stmt::Assign { name, value };
            }
            _ => {}
        }

        // Plain assignment: name = value
        if matches!(self.peek2(), Token::Assign) {
            self.advance();
            self.advance();
            let value = self.parse_expr();
            if matches!(self.peek(), Token::Newline) { self.advance(); }
            return Stmt::Assign { name, value };
        }

        let expr = self.parse_expr();
        // Attribute assignment: obj.field = val  or  obj.field += val
        if matches!(expr, Expr::Attribute { .. }) {
            let op_tok = match self.peek().clone() {
                Token::Assign | Token::PlusAssign | Token::MinusAssign |
                Token::StarAssign | Token::SlashAssign => self.advance(),
                _ => {
                    if matches!(self.peek(), Token::Newline) { self.advance(); }
                    return Stmt::Expr(expr);
                }
            };
            if let Expr::Attribute { object, field } = expr {
                let rhs = self.parse_expr();
                let value = match op_tok {
                    Token::Assign => rhs,
                    tok => {
                        let binop = match tok {
                            Token::PlusAssign  => BinOpKind::Add,
                            Token::MinusAssign => BinOpKind::Sub,
                            Token::StarAssign  => BinOpKind::Mul,
                            Token::SlashAssign => BinOpKind::Div,
                            _ => unreachable!(),
                        };
                        Expr::BinOp {
                            op: binop,
                            left: Box::new(Expr::Attribute { object: object.clone(), field: field.clone() }),
                            right: Box::new(rhs),
                        }
                    }
                };
                if matches!(self.peek(), Token::Newline) { self.advance(); }
                return Stmt::SetAttr { object: *object, field, value };
            }
        }
        if matches!(self.peek(), Token::Newline) { self.advance(); }
        Stmt::Expr(expr)
    }

    // ── Expressions (precedence ladder) ──────────────────────────────────────

    fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_or();
        // pipe operator: left |> func  or  left |> func(extra_args)
        while matches!(self.peek(), Token::Pipe) {
            self.advance();
            let func_name = match self.advance() {
                Token::Ident(n) => n,
                t => panic!("Expected function name after |>, got {:?}", t),
            };
            let mut args = vec![left];
            if matches!(self.peek(), Token::LParen) {
                self.advance();
                while !matches!(self.peek(), Token::RParen | Token::EOF) {
                    args.push(self.parse_or());
                    if matches!(self.peek(), Token::Comma) { self.advance(); }
                }
                self.expect(&Token::RParen);
            }
            left = Expr::Call { name: func_name, args };
        }
        left
    }

    fn parse_or(&mut self) -> Expr {
        let mut l = self.parse_and();
        while matches!(self.peek(), Token::Or) {
            self.advance();
            let r = self.parse_and();
            l = Expr::BinOp { op: BinOpKind::Or, left: Box::new(l), right: Box::new(r) };
        }
        l
    }

    fn parse_and(&mut self) -> Expr {
        let mut l = self.parse_comparison();
        while matches!(self.peek(), Token::And) {
            self.advance();
            let r = self.parse_comparison();
            l = Expr::BinOp { op: BinOpKind::And, left: Box::new(l), right: Box::new(r) };
        }
        l
    }

    fn parse_range(&mut self) -> Expr {
        let l = self.parse_add();
        if matches!(self.peek(), Token::DotDot) {
            self.advance();
            let r = self.parse_add();
            // a..b is inclusive → range(a, b+1)
            let r_plus_one = Expr::BinOp {
                op: BinOpKind::Add,
                left: Box::new(r),
                right: Box::new(Expr::Number(1.0)),
            };
            return Expr::Call { name: "range".to_string(), args: vec![l, r_plus_one] };
        }
        l
    }

    fn parse_comparison(&mut self) -> Expr {
        let l = self.parse_range();
        // 'not in' is two tokens but one operator
        if matches!(self.peek(), Token::Not) && matches!(self.peek2(), Token::In) {
            self.advance(); // consume Not
            self.advance(); // consume In
            let r = self.parse_range();
            return Expr::BinOp { op: BinOpKind::NotIn, left: Box::new(l), right: Box::new(r) };
        }
        let op = match self.peek() {
            Token::Eq    => BinOpKind::Eq,
            Token::NotEq => BinOpKind::NotEq,
            Token::Lt    => BinOpKind::Lt,
            Token::Gt    => BinOpKind::Gt,
            Token::LtEq  => BinOpKind::LtEq,
            Token::GtEq  => BinOpKind::GtEq,
            Token::In    => BinOpKind::In,
            _ => return l,
        };
        self.advance();
        let r = self.parse_range();
        Expr::BinOp { op, left: Box::new(l), right: Box::new(r) }
    }

    fn parse_add(&mut self) -> Expr {
        let mut l = self.parse_mul();
        loop {
            let op = match self.peek() {
                Token::Plus  => BinOpKind::Add,
                Token::Minus => BinOpKind::Sub,
                _ => break,
            };
            self.advance();
            let r = self.parse_mul();
            l = Expr::BinOp { op, left: Box::new(l), right: Box::new(r) };
        }
        l
    }

    fn parse_mul(&mut self) -> Expr {
        let mut l = self.parse_power();
        loop {
            let op = match self.peek() {
                Token::Star    => BinOpKind::Mul,
                Token::Slash   => BinOpKind::Div,
                Token::Percent => BinOpKind::Mod,
                _ => break,
            };
            self.advance();
            let r = self.parse_power();
            l = Expr::BinOp { op, left: Box::new(l), right: Box::new(r) };
        }
        l
    }

    fn parse_power(&mut self) -> Expr {
        let base = self.parse_unary();
        if matches!(self.peek(), Token::StarStar) {
            self.advance();
            let exp = self.parse_power(); // right-associative
            Expr::BinOp { op: BinOpKind::Pow, left: Box::new(base), right: Box::new(exp) }
        } else {
            base
        }
    }

    fn parse_unary(&mut self) -> Expr {
        match self.peek().clone() {
            Token::Minus => { self.advance(); Expr::UnaryOp { op: UnaryOpKind::Neg, expr: Box::new(self.parse_postfix()) } }
            Token::Not   => { self.advance(); Expr::UnaryOp { op: UnaryOpKind::Not, expr: Box::new(self.parse_postfix()) } }
            _ => self.parse_postfix(),
        }
    }

    // Handles chained indexing, slicing, and attribute/method access
    fn parse_postfix(&mut self) -> Expr {
        let mut expr = self.parse_primary();
        loop {
            if matches!(self.peek(), Token::LBracket) {
                self.advance();
                if matches!(self.peek(), Token::Colon) {
                    self.advance();
                    let end = if matches!(self.peek(), Token::RBracket) { None }
                              else { Some(Box::new(self.parse_expr())) };
                    self.expect(&Token::RBracket);
                    expr = Expr::Slice { object: Box::new(expr), start: None, end };
                } else {
                    let idx = self.parse_expr();
                    if matches!(self.peek(), Token::Colon) {
                        self.advance();
                        let end = if matches!(self.peek(), Token::RBracket) { None }
                                  else { Some(Box::new(self.parse_expr())) };
                        self.expect(&Token::RBracket);
                        expr = Expr::Slice { object: Box::new(expr), start: Some(Box::new(idx)), end };
                    } else {
                        self.expect(&Token::RBracket);
                        expr = Expr::Index { object: Box::new(expr), index: Box::new(idx) };
                    }
                }
            } else if matches!(self.peek(), Token::Dot) {
                self.advance(); // consume .
                let field = match self.advance() {
                    Token::Ident(n) => n,
                    t => panic!("Expected field name after '.', got {:?}", t),
                };
                if matches!(self.peek(), Token::LParen) {
                    // method call: obj.method(args)
                    self.advance(); // consume (
                    let mut args = Vec::new();
                    while !matches!(self.peek(), Token::RParen | Token::EOF) {
                        args.push(self.parse_expr());
                        if matches!(self.peek(), Token::Comma) { self.advance(); }
                    }
                    self.expect(&Token::RParen);
                    expr = Expr::MethodCall { object: Box::new(expr), method: field, args };
                } else {
                    expr = Expr::Attribute { object: Box::new(expr), field };
                }
            } else {
                break;
            }
        }
        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.advance() {
            Token::Number(n)    => Expr::Number(n),
            Token::Str(s)       => Expr::Str(s),
            Token::Bool(b)      => Expr::Bool(b),
            Token::None         => Expr::Nil,
            Token::FStringLit(raw) => self.parse_fstring(&raw),

            Token::LBracket => {
                // empty list
                if matches!(self.peek(), Token::RBracket) {
                    self.advance();
                    return Expr::List(vec![]);
                }
                let first = self.parse_or();
                // list comprehension: [expr for var in iter]
                if matches!(self.peek(), Token::For) {
                    self.advance();
                    let var = match self.advance() {
                        Token::Ident(n) => n,
                        t => panic!("Expected variable in list comp, got {:?}", t),
                    };
                    self.expect(&Token::In);
                    let iter = self.parse_or();
                    let cond = if matches!(self.peek(), Token::If) {
                        self.advance();
                        Some(Box::new(self.parse_or()))
                    } else {
                        None
                    };
                    self.expect(&Token::RBracket);
                    return Expr::ListComp { expr: Box::new(first), var, iter: Box::new(iter), cond };
                }
                // regular list
                let mut items = vec![first];
                while matches!(self.peek(), Token::Comma) {
                    self.advance();
                    if matches!(self.peek(), Token::RBracket) { break; }
                    items.push(self.parse_or());
                }
                self.expect(&Token::RBracket);
                Expr::List(items)
            }

            Token::LBrace => {
                // Dict literal: {} or {key: val, ...}
                if matches!(self.peek(), Token::RBrace) {
                    self.advance();
                    return Expr::Dict(vec![]);
                }
                let mut pairs = Vec::new();
                loop {
                    let key = self.parse_or();
                    self.expect(&Token::Colon);
                    let val = self.parse_or();
                    pairs.push((key, val));
                    if matches!(self.peek(), Token::Comma) {
                        self.advance();
                        if matches!(self.peek(), Token::RBrace) { break; }
                    } else {
                        break;
                    }
                }
                self.expect(&Token::RBrace);
                Expr::Dict(pairs)
            }

            Token::Ident(name) => {
                if matches!(self.peek(), Token::LParen) {
                    self.advance();
                    let mut args = Vec::new();
                    while !matches!(self.peek(), Token::RParen | Token::EOF) {
                        args.push(self.parse_expr());
                        if matches!(self.peek(), Token::Comma) { self.advance(); }
                    }
                    self.expect(&Token::RParen);
                    Expr::Call { name, args }
                } else {
                    Expr::Ident(name)
                }
            }

            Token::LParen => {
                let first = self.parse_expr();
                if matches!(self.peek(), Token::Comma) {
                    // Tuple literal: (a, b, c) → list
                    let mut items = vec![first];
                    while matches!(self.peek(), Token::Comma) {
                        self.advance();
                        if matches!(self.peek(), Token::RParen) { break; }
                        items.push(self.parse_expr());
                    }
                    self.expect(&Token::RParen);
                    Expr::List(items)
                } else {
                    self.expect(&Token::RParen);
                    first
                }
            }

            t => panic!("Unexpected token: {:?}", t),
        }
    }

    fn parse_fstring(&self, raw: &str) -> Expr {
        let mut parts = Vec::new();
        let chars: Vec<char> = raw.chars().collect();
        let mut i = 0;
        let mut lit = String::new();

        while i < chars.len() {
            if chars[i] == '{' {
                if !lit.is_empty() {
                    parts.push(FStringPart::Lit(lit.clone()));
                    lit.clear();
                }
                i += 1;
                let mut inner = String::new();
                while i < chars.len() && chars[i] != '}' {
                    inner.push(chars[i]);
                    i += 1;
                }
                i += 1; // skip '}'

                // Split on ':' for format spec
                let (expr_str, fmt) = if let Some(colon) = inner.find(':') {
                    (&inner[..colon], Some(inner[colon+1..].to_string()))
                } else {
                    (inner.as_str(), None)
                };

                let tokens = crate::lexer::tokenize(expr_str.trim());
                let expr = Parser::new(tokens).parse_expr();
                parts.push(FStringPart::Expr(expr, fmt));
            } else {
                lit.push(chars[i]);
                i += 1;
            }
        }
        if !lit.is_empty() {
            parts.push(FStringPart::Lit(lit));
        }
        Expr::FString(parts)
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    Parser::new(tokens).parse_program()
}
