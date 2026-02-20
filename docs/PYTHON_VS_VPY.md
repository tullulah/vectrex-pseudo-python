# Python vs VPy

VPy is inspired by Python but is not Python. This document summarizes the key differences and what is/isn't supported.

---

## Key differences from Python

| | Python | VPy |
|--|--------|-----|
| Types | Dynamic, arbitrary-size ints | Static, 16-bit integers only |
| Runtime | Interpreted / JIT | Compiled to MC6809 assembly |
| Memory | Garbage collected | Static allocation, fixed RAM |
| Scope | `global` keyword needed | Globals accessed automatically |
| Float | Yes | No — integers only |
| Recursion | Safe (stack grows) | Unsafe (no stack overflow protection) |
| Error reporting | Multiple errors | First error only (per file) |

---

## What's implemented

### Control flow ✅

```python
if x > 0:
    ...
elif x < 0:
    ...
else:
    ...

while active:
    ...

for i in range(0, 10):
    ...

for i in range(0, 100, 5):   # with step
    ...

for item in my_array:         # array iteration
    ...

switch state:                  # not in Python (Python 3.10+ has match)
    case 0:
        ...
    default:
        ...

break
continue
pass
return value
```

### Variables ✅

```python
# Top-level = global (persists across frames)
player_x = 0
score = 0

# Inside function = local (stack, discarded on return)
def update():
    dx = joy_x * 2   # local

# Constants (compile-time)
const MAX = 8
const TABLE = [1, 2, 4, 8, 16]

# Compound assignment
x += 1
x -= 5
x *= 2
```

No `global` keyword needed — globals are automatically accessible from any function.

### Operators ✅

All standard arithmetic (`+`, `-`, `*`, `/`, `%`), bitwise (`&`, `|`, `^`, `~`, `<<`, `>>`), comparison (`==`, `!=`, `<`, `<=`, `>`, `>=`), and logical (`and`, `or`, `not`) operators.

Chained comparisons: `0 < x < 100` → works as in Python.

Not implemented: `**` (power), `//` (floor division, use `/`), `is`, `in` for membership.

### Functions ✅

```python
def add(a, b):
    return a + b

result = add(3, 5)
```

Limit: **4 parameters maximum**.

### Arrays ✅

```python
enemies = [0, 0, 0, 0, 0, 0, 0, 0]    # mutable
const coords = [10, 20, 30, 40]         # read-only
const names = ["LEVEL 1", "LEVEL 2"]   # string arrays

x = enemies[i]        # read
enemies[i] = x        # write
count = len(enemies)  # length

for item in enemies:  # iteration
    ...
```

### Math built-ins ✅

`abs(x)`, `min(a, b)`, `max(a, b)`, `clamp(v, lo, hi)`, `sin(a)`, `cos(a)`, `tan(a)`

Note: trig functions use 0..127 as full-circle argument, result is -127..127 (scaled).

### Strings ✅ (limited)

String literals work as arguments to built-in functions and in constant arrays. Runtime string manipulation (concatenation, indexing) is not supported.

---

## What's not implemented

| Feature | Python | VPy |
|---------|--------|-----|
| Float | `3.14` | ❌ — integers only |
| `True` / `False` | keywords | ❌ — use `1` / `0` |
| f-strings | `f"x={x}"` | ❌ |
| String concatenation | `"a" + "b"` | ❌ |
| Tuples | `(1, 2)` | ❌ |
| Dicts | `{"k": v}` | ❌ |
| Sets | `{1, 2, 3}` | ❌ |
| List slices | `a[1:3]` | ❌ |
| Exceptions | `try/except` | ❌ |
| `with` | context managers | ❌ |
| `lambda` | anonymous functions | ❌ |
| `*args`, `**kwargs` | variadic args | ❌ |
| Default args | `def f(x=10):` | ❌ |
| Keyword args | `f(y=5)` | ❌ |
| Generators | `yield` | ❌ |
| Classes / inheritance | `class Foo:` | ❌ |
| Multiline strings | `"""..."""` | ❌ |
| `assert` | `assert x > 0` | ❌ |
| `**` power operator | `x ** 2` | ❌ — use shifts for powers of 2 |
| Modules / packages | complex imports | ❌ — basic `import` only |

---

## Arithmetic is always 16-bit unsigned

Values wrap at 65536. Negative numbers work as two's complement but intermediate results can surprise you:

```python
# This may not do what you expect:
x = a - b    # if b > a, wraps to large positive number

# Safe pattern for clamped subtraction:
if a > b:
    diff = a - b
else:
    diff = 0
```

Use `clamp()`, `min()`, and `max()` defensively when dealing with values that could go negative.
