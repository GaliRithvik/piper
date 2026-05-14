# Piper Language Reference

Complete guide to Piper's syntax and built-in functions.

---

## Table of Contents

1. [Variables](#1-variables)
2. [Data Types](#2-data-types)
3. [Operators](#3-operators)
4. [Strings](#4-strings)
5. [F-Strings](#5-f-strings)
6. [Lists](#6-lists)
7. [Dictionaries](#7-dictionaries)
8. [Conditionals](#8-conditionals)
9. [Loops](#9-loops)
10. [Functions](#10-functions)
11. [Case / Of](#11-case--of)
12. [Tuples & Unpacking](#12-tuples--unpacking)
13. [List Comprehensions](#13-list-comprehensions)
14. [Pipe Operator](#14-pipe-operator)
15. [Error Handling](#15-error-handling)
16. [Built-in Functions](#16-built-in-functions)

---

## 1. Variables

```python
let x = 10
let name = "Piper"
let flag = true

# Reassign
x = 20

# Augmented assignment
x += 5
x -= 2
x *= 3
x /= 2
```

---

## 2. Data Types

| Type | Example | Notes |
|---|---|---|
| Number | `42`, `3.14`, `-7` | All numbers are float64 internally |
| String | `"hello"`, `'world'` | Single or double quotes |
| Bool | `true`, `false` | Lowercase |
| None | `none` | Also `null`, `nil` |
| List | `[1, 2, 3]` | Mixed types allowed |
| Dict | `{"key": val}` | String keys |

```python
let n  = 42
let f  = 3.14
let s  = "hello"
let b  = true
let nothing = none
let nums = [1, 2, 3]
let info = {"name": "Piper", "version": 3}
```

---

## 3. Operators

### Arithmetic
| Operator | Description | Example |
|---|---|---|
| `+` | Add / string concat | `2 + 3` → `5` |
| `-` | Subtract | `10 - 4` → `6` |
| `*` | Multiply / list repeat | `3 * 4` → `12` |
| `/` | Divide | `10 / 4` → `2.5` |
| `%` | Modulo | `10 % 3` → `1` |
| `**` | Power | `2 ** 8` → `256` |

### Comparison
| Operator | Description |
|---|---|
| `==` | Equal |
| `!=` | Not equal |
| `<` | Less than |
| `>` | Greater than |
| `<=` | Less than or equal |
| `>=` | Greater than or equal |

### Logical
| Operator | Description |
|---|---|
| `and` | Logical AND |
| `or` | Logical OR |
| `not` | Logical NOT |

### Membership
| Operator | Description | Example |
|---|---|---|
| `in` | Check membership | `"a" in ["a","b"]` → `true` |
| `not in` | Check non-membership | `"x" not in "hello"` → `true` |

Works on **lists**, **strings** (substring), and **dicts** (key check).

### Assignment
```python
x += 1
x -= 1
x *= 2
x /= 2
```

---

## 4. Strings

```python
let s = "Hello, Piper!"

# Negative indexing
s[-1]                         # "!"
s[-6]                         # "P"

# Slicing
s[0:5]                        # "Hello"
s[:5]                         # "Hello"
s[-6:]                        # "Piper!"
s[7:-1]                       # "Piper"

len(s)                        # 13
upper(s)                      # "HELLO, PIPER!"
lower(s)                      # "hello, piper!"
trim("  hello  ")             # "hello"
split("a,b,c", ",")           # ["a", "b", "c"]
join("-", ["a", "b", "c"])    # "a-b-c"
contains(s, "Piper")          # true
replace(s, "Piper", "World")  # "Hello, World!"
startswith(s, "Hello")        # true
endswith(s, "!")              # true
str(42)                       # "42"
int("42")                     # 42
float("3.14")                 # 3.14
char(65)                      # "A"

# Concatenation
let full = "Hello" + " " + "World"

# Indexing
let first = s[0]              # "H"
```

---

## 5. F-Strings

Embed expressions directly inside strings.

```python
let name = "Alice"
let score = 95.678

print(f"Name: {name}")               # Name: Alice
print(f"Score: {score:.2f}")         # Score: 95.68
print(f"Int: {score:.0f}")           # Int: 96
print(f"Sci: {score:.2e}")           # Sci: 9.57e1
print(f"2 + 2 = {2 + 2}")           # 2 + 2 = 4
```

### Format Specs
| Spec | Meaning | Example |
|---|---|---|
| `.4f` | 4 decimal places | `3.1416` |
| `.2e` | Scientific notation | `3.14e0` |
| `d` | Integer | `42` |

---

## 6. Lists

```python
let nums = [1, 2, 3, 4, 5]

# Indexing & assignment
nums[0]           # 1
nums[0] = 99

# Negative indexing
nums[-1]          # 5  (last element)
nums[-2]          # 4  (second to last)
nums[-1] = 99     # assign via negative index

# Slicing
nums[1:4]         # [2, 3, 4]
nums[:3]          # [1, 2, 3]
nums[2:]          # [3, 4, 5]
nums[-3:]         # [3, 4, 5]
nums[1:-1]        # [2, 3, 4]

# Concatenation & repeat
[1, 2] + [3, 4]  # [1, 2, 3, 4]
[0] * 3           # [0, 0, 0]
```

### List Built-ins
| Function | Description | Example |
|---|---|---|
| `len(list)` | Length | `len([1,2,3])` → `3` |
| `append(list, val)` | Add to end | `append(nums, 6)` |
| `pop(list)` | Remove last | `pop(nums)` → `5` |
| `sort(list)` | Return sorted copy | `sort([3,1,2])` → `[1,2,3]` |
| `reverse(list)` | Return reversed copy | `reverse([1,2,3])` → `[3,2,1]` |
| `slice(list, lo, hi)` | Sublist | `slice(nums, 1, 3)` → `[2,3]` |
| `flatten(list)` | Flatten one level | `flatten([[1,2],[3]])` → `[1,2,3]` |
| `zip(a, b)` | Pair two lists | `zip([1,2],[3,4])` → `[[1,3],[2,4]]` |
| `enumerate(list)` | Index-value pairs | `enumerate(["a","b"])` → `[[0,"a"],[1,"b"]]` |
| `range(n)` | `0` to `n-1` | `range(3)` → `[0,1,2]` |
| `range(start, end)` | `start` to `end-1` | `range(2, 5)` → `[2,3,4]` |
| `range(start, end, step)` | With step | `range(0, 10, 2)` → `[0,2,4,6,8]` |
| `map(list, fn)` | Apply fn to each | `map(nums, double)` |
| `filter(list, fn)` | Keep where fn is true | `filter(nums, is_even)` |
| `reduce(list, fn, init)` | Fold to single value | `reduce(nums, add, 0)` |

---

## 7. Dictionaries

```python
let d = {"name": "Alice", "age": 30}

# Read
d["name"]                    # "Alice"

# Add / update
d["city"] = "NYC"

# Delete
del_key(d, "city")
```

### Dict Built-ins
| Function | Description | Example |
|---|---|---|
| `keys(d)` | List of keys | `["age", "name"]` |
| `values(d)` | List of values | `[30, "Alice"]` |
| `items(d)` | List of `[key, val]` pairs | `[["age",30],["name","Alice"]]` |
| `has_key(d, key)` | Check if key exists | `has_key(d, "age")` → `true` |
| `get(d, key, default)` | Safe read with default | `get(d, "x", 0)` → `0` |
| `del_key(d, key)` | Remove a key | `del_key(d, "age")` |
| `len(d)` | Number of keys | `len(d)` → `2` |

### Membership with `in`
```python
"name" in d        # true  — key exists
"city" not in d    # true  — key does not exist
```

---

## 8. Conditionals

```python
if x > 10:
    print("big")
elif x > 5:
    print("medium")
else:
    print("small")

# Single-line
if x > 0: print("positive")
```

---

## 9. Loops

### For loop
```python
for i in range(5):
    print(i)

for item in ["a", "b", "c"]:
    print(item)
```

### `..` range (inclusive)

A cleaner alternative to `range()` — `a..b` includes both endpoints.

```python
for i in 1..5:
    print(i)        # 1 2 3 4 5

for i in 0..9:
    print(i)        # 0 1 2 3 4 5 6 7 8 9

# Works in list comprehensions too
let squares = [i * i for i in 1..6]   # [1, 4, 9, 16, 25, 36]

let n = 10
let total = sum([i for i in 1..n])     # 55
```

| Syntax | Equivalent | Result |
|---|---|---|
| `1..5` | `range(1, 6)` | `[1, 2, 3, 4, 5]` |
| `0..9` | `range(0, 10)` | `[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]` |
| `a..b` | `range(a, b+1)` | `a` to `b` inclusive |

### While loop
```python
let i = 0
while i < 5:
    print(i)
    i += 1
```

### break / continue
```python
for i in range(10):
    if i == 5:
        break       # exit loop entirely

for i in range(10):
    if i % 2 == 0:
        continue    # skip to next iteration
    print(i)        # prints odd numbers only
```

---

## 10. Functions

```python
# Definition
fn add(a, b):
    return a + b

# Single-line
fn square(x): return x * x

# Default parameters
fn greet(name, msg = "Hello"):
    return msg + ", " + name + "!"

fn power(base, exp = 2):
    return base ** exp

print(greet("Piper"))          # Hello, Piper!
print(greet("Rithvik", "Hi"))  # Hi, Rithvik!
print(power(5))                # 25
print(power(2, 10))            # 1024

# Recursive
fn factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

# Call
print(add(3, 4))       # 7
print(factorial(5))    # 120

# Functions are values — pass them around
fn apply(f, x): return f(x)
print(apply(square, 5))  # 25
```

### `result` implicit return

Instead of `return val`, assign to the special variable `result`. It is automatically returned when the function ends.

```python
fn square(x):
    result = x * x       # no return needed

fn running_total(nums):
    result = 0
    for n in nums:
        result += n      # build up result, return at end

fn classify(score):
    if score >= 90:   result = "A"
    elif score >= 75: result = "B"
    elif score >= 60: result = "C"
    else:             result = "F"

print(square(7))              # 49
print(running_total([1..5]))  # 15
print(classify(82))           # B
```

> `return` still works and takes priority over `result` if both are used.

---

## 11. Case / Of

A cleaner alternative to long `if / elif` chains — match a value against a list of patterns.

```python
case activation:
    of "relu":    return relu(z)
    of "sigmoid": return sigmoid(z)
    of "tanh":    return tanh(z)
    else:         return z
```

```python
# Multi-line branch bodies work too
case command:
    of "train":
        print("Starting training...")
        result = train(X, y)
    of "eval":
        print("Evaluating...")
        result = evaluate(X, y)
    else:
        print("Unknown command: " + command)
```

```python
# Works with numbers
case status_code:
    of 200: print("OK")
    of 404: print("Not found")
    of 500: print("Server error")
    else:   print("Unknown: " + str(status_code))
```

**Rules:**
- `of` branches are checked top to bottom; first match wins
- `else` is optional and catches everything unmatched
- `else` must be the last branch

---

## 12. Tuples & Unpacking

Functions can return multiple values as a tuple `(a, b)`. Use `let (a, b) = ...` to unpack them.

```python
# Return a tuple
fn min_max(data):
    return (min(data), max(data))

# Unpack on the left side
let (lo, hi) = min_max([3, 1, 9, 2, 7])
print(lo)   # 1
print(hi)   # 9
```

```python
# Useful for ML — return loss and accuracy together
fn train_step(X, y):
    let loss = mean([v * v for v in X])
    let acc  = round(1.0 - loss, 3)
    return (loss, acc)

let (loss, acc) = train_step(X, y)
print("loss=" + str(loss) + "  acc=" + str(acc))
```

```python
# Tuple literal — also just a list under the hood
let point = (3.0, 4.0)
let (x, y) = point
print(sqrt(x*x + y*y))   # 5.0
```

---

## 13. List Comprehensions

```python
# Basic
let squares = [x * x for x in range(1, 6)]
# [1, 4, 9, 16, 25]

# With condition
let evens = [x for x in range(1, 11) if x % 2 == 0]
# [2, 4, 6, 8, 10]

# Transform strings
let words = ["hello", "world"]
let upper_words = [upper(w) for w in words]
# ["HELLO", "WORLD"]

# With .. range
let cubes = [i * i * i for i in 1..5]
# [1, 8, 27, 64, 125]
```

---

## 14. Pipe Operator

Chain function calls left to right — the output of each step becomes the first argument of the next.

```python
# Without pipe
print(str(sum([1, 2, 3])))

# With pipe
[1, 2, 3] |> sum |> str |> print

# With extra arguments
[3, 1, 2] |> sort |> reverse

# Multi-step AI pipeline
let result = data |> normalize |> mean
```

---

## 15. Error Handling

```python
# Basic try / except
try:
    let x = arr[99]
except err:
    print("Error: " + err)

# Without binding the error
try:
    risky_call()
except:
    print("something went wrong")

# Inside a function
fn safe_divide(a, b):
    try:
        return a / b
    except e:
        return 0
```

`err` is bound to the error message string, so you can inspect, log, or display it.

---

## 16. Built-in Functions

### I/O
| Function | Description |
|---|---|
| `print(val, ...)` | Print values separated by space |
| `p(Hello World)` | Print shorthand — no quotes needed for plain text |
| `input(prompt)` | Read a line from the user |

### Type Conversion
| Function | Description |
|---|---|
| `str(x)` | Convert to string |
| `int(x)` | Convert to integer |
| `float(x)` | Convert to float |
| `bool(x)` | Convert to bool |
| `type(x)` | Get type name as string |

### Math
| Function | Description |
|---|---|
| `abs(x)` | Absolute value |
| `sqrt(x)` | Square root |
| `cbrt(x)` | Cube root |
| `pow(x, y)` | `x` to the power `y` |
| `exp(x)` | e^x |
| `log(x)` | Natural log (base e) |
| `log(x, base)` | Log with custom base |
| `log2(x)` | Log base 2 |
| `log10(x)` | Log base 10 |
| `floor(x)` | Round down |
| `ceil(x)` | Round up |
| `round(x, places)` | Round to decimal places |
| `sign(x)` | `-1`, `0`, or `1` |
| `clamp(x, lo, hi)` | Clamp value between lo and hi |
| `sin(x)` | Sine |
| `cos(x)` | Cosine |
| `tan(x)` | Tangent |
| `asin(x)` | Arc sine |
| `acos(x)` | Arc cosine |
| `atan(x)` | Arc tangent |
| `atan2(y, x)` | Two-argument arc tangent |
| `pi` | 3.14159… |
| `e` | 2.71828… |
| `inf` | Infinity |
| `nan` | Not a Number |

### Statistics
| Function | Description |
|---|---|
| `sum(list)` | Sum of all elements |
| `mean(list)` | Average |
| `std(list)` | Standard deviation |
| `variance(list)` | Variance |
| `min(list)` | Minimum value |
| `max(list)` | Maximum value |
| `median(list)` | Median value |
| `argmax(list)` | Index of max value |
| `argmin(list)` | Index of min value |

### AI Activations
| Function | Description |
|---|---|
| `relu(x or list)` | ReLU activation |
| `leaky_relu(x, alpha)` | Leaky ReLU (default alpha=0.01) |
| `sigmoid(x or list)` | Sigmoid activation |
| `tanh(x or list)` | Tanh activation |
| `softmax(list)` | Softmax (normalised probabilities) |
| `gelu(x)` | GELU activation |
| `selu(x)` | SELU activation |

### Vector Operations
| Function | Description |
|---|---|
| `dot(a, b)` | Dot product |
| `vadd(a, b)` | Element-wise add |
| `vsub(a, b)` | Element-wise subtract |
| `vmul(a, b)` | Element-wise multiply |
| `vdiv(a, b)` | Element-wise divide |
| `vscale(v, s)` | Scale vector by scalar |
| `norm(v)` | L2 norm (magnitude) |
| `normalize(v)` | Unit vector |
| `cross(a, b)` | Cross product (3D only) |

### Matrix Operations
| Function | Description |
|---|---|
| `matmul(A, B)` | Matrix multiplication |
| `transpose(A)` | Transpose |
| `identity(n)` | n×n identity matrix |
| `reshape(list, rows, cols)` | Reshape flat list to 2D |

### Array Generators
| Function | Description |
|---|---|
| `zeros(n)` | List of n zeros |
| `zeros(n, m)` | n×m matrix of zeros |
| `ones(n)` | List of n ones |
| `ones(n, m)` | n×m matrix of ones |
| `linspace(start, end, n)` | n evenly spaced values |
| `arange(start, end, step)` | Range with float step |

### Loss Functions
| Function | Description |
|---|---|
| `mse(pred, true)` | Mean Squared Error |
| `cross_entropy(pred, true)` | Cross-entropy loss |
| `one_hot(index, depth)` | One-hot encode |

---

*For runnable examples see [`examples/demo.piper`](examples/demo.piper), [`examples/ai_demo.piper`](examples/ai_demo.piper), and [`examples/nim_features_test.piper`](examples/nim_features_test.piper).*
