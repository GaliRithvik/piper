# 🐍 Piper — AI-First Programming Language

Piper is a programming language with Python-like syntax and a Rust backend, designed for AI and machine learning workflows.

![Rust](https://img.shields.io/badge/Backend-Rust-orange) ![Version](https://img.shields.io/badge/version-0.3.0-blue) ![License](https://img.shields.io/badge/license-MIT-green)

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
- **70+ AI/ML built-ins** — activations, vector ops, matrix math, loss functions
- **Dictionary type** — `{"key": val}` literals, indexing, and dict built-ins
- **Pipe operator** `|>` — chain functions elegantly
- **List comprehensions** with `if` conditions
- **F-strings** with format specs
- **Membership operators** — `in` and `not in` for lists, strings, and dicts
- **Loop control** — `break` and `continue` in `for` and `while` loops
- **Error handling** — `try / except` blocks
- **Short print** — `p(Hello World);` instead of `print("Hello World")`
- **Rust-powered** — fast tree-walk interpreter

---

## Syntax Showcase

```python
# Print shorthand
p(Hello Piper!);

# Variables & math
let x = 42
let name = "Piper"

# List comprehension with condition
let evens = [x for x in range(1, 11) if x % 2 == 0]

# Functions
fn square(x): return x * x

# Pipe operator
let result = [1, 2, 3] |> sum |> str

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
print("name" in person)        # true

# break / continue
for i in range(1, 10):
    if i == 5:
        break
    if i % 2 == 0:
        continue
    print(i)   # 1 3

# try / except
try:
    let arr = [1, 2, 3]
    let x = arr[99]
except err:
    print(err)   # Index 99 out of bounds

# AI activations
let z = [-2, -1, 0, 1, 2]
let activated = relu(z)
let probs     = softmax([1.5, 0.8, 2.3])

# Matrix operations
let A = [[1, 2], [3, 4]]
let B = [[5, 6], [7, 8]]
let C = matmul(A, B)   # [[19, 22], [43, 50]]

# Loss functions
let loss = mse(y_true, y_pred)
```

---

## demo.piper — What's Inside

Run it with `cargo run -- examples/demo.piper`

| # | Section | What it demonstrates | Example output |
|---|---|---|---|
| 1 | **Variables & Arithmetic** | `let`, math ops, `round()`, `str()` | `Rectangle: 12 x 5 = 60`, `Circle area: 153.94` |
| 2 | **Strings** | `len`, `upper`, `lower`, `trim`, `split`, `join` | `PIPER -> piper`, `Split: [one, two, three]` |
| 3 | **Conditionals** | `if / elif / else` | `Temperature: 28C → Warm and pleasant.` |
| 4 | **Functions** | `fn`, recursion, `greet()`, `factorial()` | `Hello, Piper!`, `7! = 5040` |
| 5 | **For Loop** | `for`, `range()`, prime sieve | `Primes up to 30: 2 3 5 7 11 13 17 19 23 29` |
| 6 | **While Loop** | `while`, FizzBuzz | `Fizz`, `Buzz`, `FizzBuzz` for 1–20 |
| 7 | **List Comprehensions** | `[x*x for x in ...]`, filter with `if` | `Squares: [1, 4, 9, 16, 25]` |
| 8 | **Dictionaries** | `{}`, indexing, `keys`, `has_key`, `get`, `del_key` | `Name: Rithvik`, `Country: not set` |
| 9 | **in / not in** | list membership, substring, dict key, inside loop | `apple in fruits: true`, `'an' in 'banana': true` |
| 10 | **break / continue** | skip evens, stop at multiple of 7 | `1 3 5 7`, `Stopped at 7, Total = 21` |
| 11 | **Higher-order Functions** | `map`, `filter`, `reduce`, `enumerate` | `map(double): [2,4,6,8...]`, `reduce(add): 36` |
| 12 | **try / except** | catch errors, `safe_divide` with error handling | `Caught: Index 99 out of bounds`, `10/2 = 5` |

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
| Lists | `append`, `pop`, `sort`, `reverse`, `slice`, `flatten`, `zip`, `map`, `filter`, `reduce`, `enumerate`, `range` |
| Strings | `len`, `split`, `join`, `upper`, `lower`, `trim`, `contains`, `replace`, `startswith`, `endswith`, `char` |
| Dicts | `keys`, `values`, `items`, `has_key`, `get`, `del_key` |

---

## Getting Started

### Requirements

- [Rust](https://rustup.rs/) installed

### Run a file

```bash
cargo run -- examples/demo.piper
cargo run -- examples/ai_demo.piper
```

### Start the REPL

```bash
cargo run
```

---

## VS Code Integration

### Step 1 — Install the syntax highlighting extension

**Mac/Linux:**
```bash
cp -r vscode-extension ~/.vscode/extensions/piper-language
```

**Windows:**
```bash
xcopy /E /I vscode-extension "%USERPROFILE%\.vscode\extensions\piper-language"
```

Then reload VS Code (`Cmd+Shift+P` → **Reload Window**).

### Step 2 — Run your `.piper` file

Open the `piper/` folder in VS Code, open any `.piper` file, and press **`Cmd+Shift+B`** (Mac) or **`Ctrl+Shift+B`** (Windows/Linux).

Output appears in the integrated terminal.

---

## Project Structure

```
piper/
├── Cargo.toml
├── src/
│   ├── main.rs          # CLI entry point + REPL
│   ├── lexer.rs         # Tokenizer
│   ├── parser.rs        # AST + recursive descent parser
│   └── interpreter.rs   # Tree-walk interpreter (70+ built-ins)
├── examples/
│   ├── demo.piper        # Basic language demo
│   └── ai_demo.piper     # Full AI/ML demo
└── .vscode/
    └── tasks.json        # Run current file with Cmd+Shift+B
```

---

## Changelog

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
