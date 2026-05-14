# 🐍 Piper — AI-First Programming Language

Piper is a programming language with Python-like syntax and a Rust backend, designed for AI and machine learning workflows.

![Rust](https://img.shields.io/badge/Backend-Rust-orange) ![Version](https://img.shields.io/badge/version-0.3.0-blue) ![License](https://img.shields.io/badge/license-MIT-green)

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

Open the `piper/` folder in VS Code, then press **`Cmd+Shift+B`** to run the currently open `.piper` file. Output appears in the integrated terminal.

For syntax highlighting, install the bundled extension by copying `piper-language/` into `~/.vscode/extensions/` and reloading VS Code.

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
