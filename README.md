# Piper — AI-First Programming Language

Piper is a programming language with Python-like syntax and a Rust backend, designed for AI and machine learning workflows.

![Rust](https://img.shields.io/badge/Backend-Rust-orange) ![Version](https://img.shields.io/badge/version-0.9.0-blue) ![License](https://img.shields.io/badge/license-MIT-green) ![WASM](https://img.shields.io/badge/runs%20in-browser%20(WASM)-purple)

🌐 **[Try it in your browser →](https://GaliRithvik.github.io/piper)** — no install needed

📖 **[Full Language Reference →](LANGUAGE.md)** — all syntax, operators, and built-in functions

⚡ **[Performance Analysis →](PERFORMANCE.md)** — benchmarks, Big-O zones, and where Piper beats Python

---

## Quick Start

> **Requirements:** [Rust](https://rustup.rs/) must be installed.

```bash
# 1. Clone the repo
git clone https://github.com/GaliRithvik/piper.git
cd piper

# 2. Run the demo
cargo run -- examples/demo.piper

# 3. Write your own code
cargo run -- myfile.piper

# 4. Start the interactive REPL
cargo run
```

**VS Code users** — run any `.piper` file with one shortcut:
```bash
# Install syntax highlighting (Mac/Linux)
cp -r vscode-extension ~/.vscode/extensions/piper-language
```
Then open the folder in VS Code and press **`Cmd+Shift+B`** (Mac) / **`Ctrl+Shift+B`** (Windows/Linux).

---

## Features

- **Python-like syntax** — clean, readable, easy to write
- **Lambda / anonymous functions** — `fn(x) => x * 2`, composable and passable
- **Closures** — lambdas capture their outer scope at creation time
- **Classes** — `class Foo:` with `init`, methods, `self`, `__str__`, field access and assignment
- **Inheritance** — `class Dog(Animal):` with `super.method(self, ...)` calls
- **Static methods** — `static fn name():` called on the class directly
- **90+ AI/ML built-ins** — activations, vector ops, matrix math, loss functions, random, data processing, metrics, ASCII visualization
- **Dictionary type** — `{"key": val}` literals, indexing, and dict built-ins
- **Dict comprehensions** — `{k: v for k in list}`
- **Pipe operator** `|>` — chain functions elegantly
- **List comprehensions** with `if` conditions
- **F-strings** with format specs
- **Triple-quoted strings** — `"""..."""` for multi-line literals
- **Escape sequences** — `\n`, `\t`, `\\`, `\"` inside strings
- **Membership operators** — `in` and `not in` for lists, strings, and dicts
- **Null coalescing** `??` — `val ?? "default"` returns right side when left is `none`
- **Optional chaining** `?.` — `obj?.field` returns `none` instead of crashing
- **`*args` variadic functions** — `fn f(*nums)` collects all extra args into a list
- **Named arguments** — `greet(name="Alice", greeting="Hi")`
- **Modules / import** — `import "file.piper"` loads another Piper file
- **Loop control** — `break` and `continue` in `for` and `while` loops
- **Error handling** — `try / except` blocks with line numbers in error messages
- **Negative indexing** — `arr[-1]`, `s[-2]` for lists and strings
- **Slicing with step** — `arr[1:4]`, `arr[::2]`, `arr[::-1]`, `arr[1:10:2]`
- **Default parameters** — `fn greet(name, msg="Hello"):`
- **`..` range syntax** — `for i in 1..10` instead of `range(1, 11)`
- **`case / of`** — clean pattern matching
- **Tuple unpacking** — `let (loss, acc) = train(X, y)`
- **`result` implicit return** — set `result = x` instead of `return x`
- **Short print** — `p(Hello World);` instead of `print("Hello World")`
- **HTTP server** — `serve("0.0.0.0", 8080)` starts a built-in server; `route()` and `honeypot()` register handlers
- **Deception layer** — `is_bot`, `tarpit`, `block_ip`, `canary_token`, `fake_account`, `deception_maze` for active defense
- **Rust-powered** — fast tree-walk interpreter, ~2× faster than CPython for compute-heavy loops
- **WebAssembly** — runs in the browser, no install required

---

## Syntax Showcase

```python
# Variables & math
let x = 42
let name = "Piper"

# Lambda functions
let double = fn(x) => x * 2
let square = fn(x) => x * x
print(double(5))                             # 10
print(map([1,2,3,4,5], fn(x) => x * x))     # [1, 4, 9, 16, 25]
print(filter([1,2,3,4,5], fn(x) => x % 2 == 0))  # [2, 4]

# List comprehension with condition
let evens = [x for x in range(1, 11) if x % 2 == 0]

# Functions
fn fib(n):
    if n <= 1: return n
    return fib(n - 1) + fib(n - 2)

# Pipe operator
let result = [1, 2, 3] |> sum |> str

# Slicing with step
let lst = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
print(lst[::2])    # [0, 2, 4, 6, 8]
print(lst[::-1])   # [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
print(lst[1:8:2])  # [1, 3, 5, 7]

# Dictionary
let person = {"name": "Alice", "age": 30}
print(person["name"])          # Alice
person["city"] = "NYC"
print(has_key(person, "city")) # true
print(keys(person))            # [age, city, name]

# in / not in
let fruits = ["apple", "banana", "cherry"]
print("apple" in fruits)       # true
print("grape" not in fruits)   # true
print("an" in "banana")        # true

# break / continue
for i in range(1, 10):
    if i == 5: break
    if i % 2 == 0: continue
    print(i)   # 1 3

# try / except with line numbers in errors
try:
    let x = undefined_var
except err:
    print(err)   # [line N] Undefined variable: 'undefined_var'

# .. range (inclusive)
for i in 1..5:
    print(i)     # 1 2 3 4 5

# case / of pattern matching
case activation:
    of "relu":    return relu(z)
    of "sigmoid": return sigmoid(z)
    else:         return z

# Tuple unpacking
fn min_max(data): return (min(data), max(data))
let (lo, hi) = min_max([3, 1, 9, 2])

# result implicit return
fn classify(score):
    if score >= 90: result = "A"
    elif score >= 75: result = "B"
    else: result = "C"

# Classes with OOP
class Dog:
    fn init(self, name, breed):
        self.name = name
        self.breed = breed
        self.tricks = []

    fn bark(self):
        print(self.name + " says: Woof!")

    fn learn(self, trick):
        append(self.tricks, trick)

let d = Dog("Rex", "Labrador")
d.bark()            # Rex says: Woof!
d.learn("sit")
print(d.name)       # Rex
print(type(d))      # Dog

# AI activations & matrix ops
let z = [-2, -1, 0, 1, 2]
let activated = relu(z)
let probs = softmax([1.5, 0.8, 2.3])
let A = [[1, 2], [3, 4]]
let B = [[5, 6], [7, 8]]
let C = matmul(A, B)   # [[19, 22], [43, 50]]
```

---

## Built-in Functions

| Category | Functions |
|---|---|
| I/O | `print`, `p`, `input` |
| Types | `str`, `int`, `float`, `bool`, `type` |
| Math | `abs`, `sqrt`, `cbrt`, `floor`, `ceil`, `round`, `pow`, `log`, `log2`, `log10`, `exp`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sign`, `clamp`, `pi`, `e`, `inf`, `nan` |
| Stats | `sum`, `mean`, `std`, `variance`, `min`, `max`, `median`, `argmax`, `argmin` |
| AI Activations | `relu`, `leaky_relu`, `sigmoid`, `tanh`, `softmax`, `gelu`, `selu` |
| Vectors | `dot`, `vadd`, `vsub`, `vmul`, `vdiv`, `vscale`, `norm`, `normalize`, `cross` |
| Matrices | `matmul`, `transpose`, `identity`, `reshape` |
| Arrays | `zeros`, `ones`, `linspace`, `arange` |
| Loss | `mse`, `cross_entropy`, `one_hot` |
| Random | `seed`, `rand`, `randn`, `randint`, `shuffle`, `choice` |
| Data | `batch`, `train_test_split`, `standardize`, `normalize_rows` |
| Metrics | `accuracy`, `precision`, `recall`, `f1_score`, `r2_score`, `confusion_matrix` |
| Viz | `plot`, `bar_chart` |
| Lists | `append`, `pop`, `sort`, `reverse`, `slice`, `flatten`, `zip`, `map`, `filter`, `reduce`, `enumerate`, `range` |
| Strings | `len`, `split`, `join`, `upper`, `lower`, `trim`, `contains`, `replace`, `startswith`, `endswith`, `char` |
| Dicts | `keys`, `values`, `items`, `has_key`, `get`, `del_key` |
| HTTP Server | `serve`, `route`, `honeypot`, `response` |
| Bot Detection | `is_bot`, `bot_score` |
| Deception | `tarpit`, `block_ip`, `unblock_ip`, `canary_token`, `log_threat`, `get_threats`, `get_blocked` |
| Fake Data | `fake_account`, `fake_user_list`, `fake_transaction`, `deception_maze` |

---

## Project Structure

```
piper/
├── Cargo.toml                    # Rust package config (bin + cdylib for WASM + tiny_http)
├── src/
│   ├── main.rs                   # CLI entry point + REPL; starts HTTP server if serve() called
│   ├── lib.rs                    # WASM entry point (wasm-bindgen)
│   ├── lexer.rs                  # Tokenizer — Token stream with line number markers
│   ├── parser.rs                 # AST definitions + recursive descent parser
│   ├── interpreter.rs            # Tree-walk interpreter — 90+ built-ins + server/deception state
│   └── server.rs                 # HTTP server (tiny_http) + deception layer — route dispatch,
│                                 #   bot fingerprinting, canary scanning, JSON serialisation
│
├── docs/                         # GitHub Pages — browser playground
│   ├── index.html                # Playground UI (editor + output split pane)
│   └── pkg/                      # Compiled WebAssembly (wasm-pack output)
│       ├── piper.js              # JS bindings generated by wasm-bindgen
│       ├── piper_bg.wasm         # Compiled Piper interpreter
│       └── piper.d.ts            # TypeScript type declarations
│
├── examples/
│   ├── demo.piper                # Full language demo
│   ├── ai_demo.piper             # AI/ML showcase
│   ├── messaging.piper           # PiperChat — CLI messaging app
│   ├── messaging app (demo).piper # Interactive messaging demo
│   ├── nn_forward.piper          # Neural net forward pass in Piper
│   ├── nn_forward.py             # Same in Python (comparison)
│   ├── nim_features_test.piper   # v0.5 features: .., case/of, tuples, result
│   ├── features_test.piper       # v0.4 features: slicing, negative index, defaults
│   ├── aiml_builtins_test.piper  # v0.6 AI/ML built-ins
│   ├── classes_test.piper        # v0.7 class syntax
│   ├── benchmark.piper           # Performance benchmark
│   ├── benchmark_pure.py         # Pure Python benchmark
│   ├── benchmark_numpy.py        # NumPy benchmark
│   └── generate_chart.py         # Generates performance_chart.png
│
├── vscode-extension/             # VS Code syntax highlighting (install locally)
│   ├── package.json
│   ├── language-configuration.json
│   └── syntaxes/
│       └── piper.tmLanguage.json
│
├── performance_chart.png         # Benchmark bar chart + Big-O zone chart
├── PERFORMANCE.md                # Benchmark results and Big-O analysis
├── LANGUAGE.md                   # Full language reference (20 sections)
└── .vscode/
    └── tasks.json                # Run current file with Cmd+Shift+B
```

---

## Browser Playground

Piper compiles to **WebAssembly** and runs entirely in the browser — no install, no server.

**[Try it → GaliRithvik.github.io/piper](https://GaliRithvik.github.io/piper)**

Built with `wasm-pack` + `wasm-bindgen`, hosted via GitHub Pages from the `docs/` folder.

To rebuild WASM locally after changing interpreter code:
```bash
wasm-pack build --target web --out-dir docs/pkg
```

---

## VS Code Integration

### Step 1 — Install syntax highlighting

**Mac/Linux:**
```bash
cp -r vscode-extension ~/.vscode/extensions/piper-language
```

**Windows:**
```bash
xcopy /E /I vscode-extension "%USERPROFILE%\.vscode\extensions\piper-language"
```

Reload VS Code (`Cmd+Shift+P` → **Reload Window**).

### Step 2 — Run your `.piper` file

Open the `piper/` folder in VS Code, open any `.piper` file, and press **`Cmd+Shift+B`** (Mac) or **`Ctrl+Shift+B`** (Windows/Linux).

---

## Changelog

### v0.9.0
- Added **HTTP server** — `serve(host, port)` + `route()` + `honeypot()` built-ins powered by Rust's `tiny_http`
- Added **deception / anti-bot layer** — `is_bot`, `bot_score`, `tarpit`, `block_ip`, `canary_token`, `log_threat`, `fake_account`, `fake_user_list`, `fake_transaction`, `deception_maze`
- Added **inheritance** — `class Dog(Animal):` extends a parent class
- Added **`super`** — call parent methods with `super.method(self, ...)`
- Added **static methods** — `static fn name():` called on the class directly
- Added **`__str__`** — custom string representation for instances
- Added **closures** — lambdas capture outer scope variables at creation time
- Added **`*args`** variadic parameters — `fn f(*nums)` collects all positional args into a list
- Added **named arguments** — `greet(name="Alice", greeting="Hi")`
- Added **`import`** — `import "file.piper"` loads another Piper source file
- Added **dict comprehensions** — `{k: v for k in iter if cond}`
- Added **triple-quoted strings** — `"""..."""` for multi-line string literals
- Added **escape sequences** — `\n`, `\t`, `\r`, `\\`, `\"`, `\'` inside strings
- Added **null coalescing** `??` — `val ?? fallback`
- Added **optional chaining** `?.` — `obj?.field` returns `none` safely
- Added **`sort` with key function** — `sort(list, fn(x) => x.name)`
- Improved **REPL** — multiline input: lines ending with `:` prompt for continuation

### v0.8.0
- Added **lambda / anonymous functions** — `fn(x) => x * 2`, works with `map`, `filter`, `reduce`
- Added **line numbers in error messages** — `[line 7] Undefined variable: 'foo'`
- Added **slice with step** — `list[::2]`, `list[::-1]`, `list[1:10:2]`
- Fixed **instance equality** — `==` now compares fields by value, not object identity
- Fixed **string comparison** — `<`, `>`, `<=`, `>=` use lexicographic order for strings
- Fixed **`f1_score`** internal argument duplication bug
- Launched **WebAssembly browser playground** at [GaliRithvik.github.io/piper](https://GaliRithvik.github.io/piper)

### v0.7.0
- Added **`class` keyword** — OOP with `init`, methods, `self`, field access and assignment
- Added **method chaining** — `obj.method().field`

### v0.6.0
- Added **Random built-ins** — `seed`, `rand`, `randn`, `randint`, `shuffle`, `choice`
- Added **Data processing** — `batch`, `train_test_split`, `standardize`, `normalize_rows`
- Added **Evaluation metrics** — `accuracy`, `precision`, `recall`, `f1_score`, `r2_score`, `confusion_matrix`
- Added **ASCII visualization** — `plot` (line chart), `bar_chart`

### v0.5.0
- Added **`..` range syntax** — `for i in 1..10` (inclusive)
- Added **`case / of`** — pattern matching
- Added **tuple unpacking** — `let (a, b) = fn()`
- Added **`result` implicit return**

### v0.4.0
- Added **negative indexing** — `arr[-1]`, `s[-2]`
- Added **slicing** — `arr[1:4]`, `s[:3]`, `arr[-3:]`
- Added **default parameters** — `fn greet(name, msg="Hello"):`

### v0.3.0
- Added **Dictionary** type with full built-in support
- Added **`in` / `not in`** membership operators
- Added **`break`** and **`continue`** loop control
- Added **`try / except`** error handling
- Added VS Code tasks for one-key file execution

### v0.2.0
- 60+ AI/ML built-in functions
- Pipe operator `|>`
- List comprehensions
- F-strings with format specs
- REPL mode

### v0.1.0
- Initial release

---

## License

MIT
