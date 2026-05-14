# 🐍 Piper — AI-First Programming Language

Piper is a programming language with Python-like syntax and a Rust backend, designed for AI and machine learning workflows.

![Rust](https://img.shields.io/badge/Backend-Rust-orange) ![Version](https://img.shields.io/badge/version-0.2.0-blue) ![License](https://img.shields.io/badge/license-MIT-green)

---

## Features

- **Python-like syntax** — clean, readable, easy to write
- **60+ AI/ML built-ins** — activations, vector ops, matrix math, loss functions
- **Pipe operator** `|>` — chain functions elegantly
- **List comprehensions** with `if` conditions
- **F-strings** with format specs
- **Short print** — `p(Hello World);` instead of `print("Hello World")`
- **Rust-powered** — fast tree-walk interpreter

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

## Built-in Functions

| Category | Functions |
|---|---|
| Math | `abs`, `sqrt`, `floor`, `ceil`, `round`, `pow`, `log`, `exp` |
| Stats | `sum`, `mean`, `std`, `variance`, `min`, `max`, `median` |
| AI Activations | `relu`, `sigmoid`, `tanh`, `softmax`, `gelu`, `selu` |
| Vectors | `dot`, `vadd`, `vsub`, `vmul`, `vdiv`, `vscale`, `norm`, `normalize`, `cross` |
| Matrices | `matmul`, `transpose`, `identity`, `reshape` |
| Arrays | `zeros`, `ones`, `linspace`, `arange` |
| Loss | `mse`, `cross_entropy`, `one_hot` |
| Lists | `append`, `pop`, `sort`, `reverse`, `slice`, `flatten`, `zip`, `map`, `filter`, `reduce`, `enumerate`, `range` |
| Strings | `len`, `str`, `int`, `float`, `split`, `join`, `upper`, `lower`, `trim` |

## Getting Started

### Requirements

- [Rust](https://rustup.rs/) installed

### Run a file

```bash
cargo run -- examples/ai_demo.piper
```

### Run the demo

```bash
cargo run -- examples/demo.piper
cargo run -- examples/ai_demo.piper
```

## Project Structure

```
piper/
├── Cargo.toml
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lexer.rs         # Tokenizer
│   ├── parser.rs        # AST + recursive descent parser
│   └── interpreter.rs   # Tree-walk interpreter (60+ built-ins)
└── examples/
    ├── demo.piper        # Basic language demo
    └── ai_demo.piper     # Full AI/ML demo
```

## License

MIT
