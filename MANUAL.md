# Pseudo-Python Vector Compiler Programming Manual (Draft)

This manual describes the current prototype language supported by the multi-target pseudo-Python compiler for Vectrex (6809), PiTrex (ARM), VecFever / Vextreme (Cortex-M).

## 1. Design Goals
- Very small, explicit subset inspired by Python for fast iteration on vector hardware.
- Deterministic, no dynamic allocation, no recursion (currently not prevented but discouraged).
- Uniform 16-bit unsigned integer arithmetic semantics across all targets (values masked to 0xFFFF after operations where needed).

## 2. Source File Basics
- File extension: `.vpy` (convention).
- Encoding: UTF-8.
- Indentation: exactly 4 spaces per block level. Tabs are rejected. Mixed indentation levels must be multiples of 4.
- One module = one file. No imports yet.

## 3. Lexical Elements
Tokens currently supported:
- Keywords: `def`, `return`, `for`, `in`, `range`, `if`, `elif`, `else`, `while`, `break`, `continue`, `and`, `or`, `not`
- Operators / punctuation: `+ - * / % << >> & | ^ ~ == != < <= > >= = ( ) , :`
- Literals: 
    - Decimal integers (no sign inside literal; unary `-` handled syntactically)
    - Hexadecimal: `0x` followed by hex digits
    - Binary: `0b` followed by binary digits
- Identifiers: `[a-zA-Z_][a-zA-Z0-9_]*`
- Comments: `#` to end of line (ignored by lexer)
- Boolean literals: (not separate; any non-zero is truthy, zero is false). `True/False` may be added later.
- End of line creates statement boundaries (like Python). Blank lines are ignored.

## 4. Types & Values
- Single primitive: 16-bit unsigned integer (0..65535). Negative intermediate results wrap modulo 65536.
- Truthiness: `0` is false; any other value is true.

## 5. Statements
1. Function definition:
```
def name(param0, param1, ...):
    body
```
Up to 4 parameters participate in the current simple ABI (more will be ignored / future error).

2. Assignment:
```
name = expression
```
Creates or updates a global variable (function locals are treated as globals for now; later lexical scopes may be added).

3. For loop (range based):
```
for i in range(start, end, step):
    body
```
`step` is optional; default assumed 1 if omitted (currently explicit step often required in codegen). Loop variable and bounds evaluated once per iteration check (simple semantics). Loop runs while `i < end`.

4. While loop:
```
while condition:
    body
```

5. If / Elif / Else:
```
if cond:
    ...
elif other:
    ...
else:
    ...
```
`elif` chain short-circuits; only one block executes.

6. Break / Continue inside loops.

7. Return (optional expression). If last statement not an explicit return, backends emit a function epilogue returning (value in result register is unspecified unless you return explicitly).

8. Expression statement: just an expression on a line (mainly function calls).

## 6. Expressions
- Literals: decimal / hex / binary (`123`, `0x1F`, `0b1010`)
- Identifiers: `foo`
- Binary arithmetic: `+ - * / %` (unsigned 16-bit; division or modulo by zero currently leaves left operand — placeholder UB)
- Bitwise: `& | ^ << >> ~` (shift amounts masked to low 4 bits during folding / codegen to keep 0..15 range)
- Comparisons: `== != < <= > >=`
- Chained comparisons: `a < b < c` expands to `(a < b) and (b < c)` with short-circuiting
- Logical: `and`, `or` (short-circuit) and unary `not`
- Unary plus and minus: `-x` lowers to `0 - x`; `+x` is identity.
- Bitwise not: `~x` -> mask applied yielding 16-bit result.
- Function call: `name(arg0, arg1, ...)` evaluating up to 4 arguments.

Operator precedence (high to low):
1. Unary: `- + not ~`
2. Multiplicative: `* / %`
3. Additive: `+ -`
4. Shifts: `<< >>`
5. Bitwise AND: `&`
6. Bitwise XOR: `^`
7. Bitwise OR: `|`
8. Comparison (including chained)
9. Logical AND: `and`
10. Logical OR: `or`

Short-circuit rules preserve left-to-right evaluation for logical ops and chained comparisons.

## 7. Function Calling & ABI (Prototype)
- Caller evaluates arguments right-to-left ensuring leftmost ends in r0 (ARM/Cortex-M) or placed sequentially (6809 temporary slots) before the call.
- ARM / Cortex-M: Up to 4 args in r0..r3. Callee prologue stores each to a global `VAR_<PARAM>` location.
- 6809: Caller stores each evaluated arg into `VAR_ARGi`. Callee copies into `VAR_<PARAM>` at entry.
- Return value: r0 (ARM/Cortex-M), D register (6809) and also stored in a pseudo global `RESULT` (6809) during expression evaluation.

## 8. Optimization Passes
Executed iteratively (up to 5 rounds) until stable:
1. Constant folding & algebraic simplifications (e.g., `x+0 -> x`, `x*1 -> x`, bitwise identities like `x|0 -> x`, `x&0 -> 0`, `x^0 -> x`).
2. Dead code elimination (unreachable removal after constant conditions become false/true, prunes empty constructs).
3. Forward constant propagation with branch merging (simple environment merge — losing knowledge on divergence).
4. Dead store elimination (drops assignments whose values are never subsequently read and have no side effects).
5. Peepholes in backends (e.g., multiply/divide by power-of-two becomes shifts, `x+x` duplicates to single ADD/shift sequences, 6809 special patterns).

All arithmetic coerced via mask to 16 bits after operations where necessary.

## 9. Backend Notes
### 6809 (Vectrex)
- Uses zero-page-like globals: `VAR_<NAME>` for variables, `VAR_ARGi` for call arguments, temporaries (`TMPLEFT`, `TMPRIGHT`, etc.).
- Multiplication / division call helper subroutines `MUL16`, `DIV16` (prototype) or inline shift peepholes for powers of two.

### ARM (PiTrex)
- Simple linear code, preserves no callee-saved registers (prototype). r4, r5 used as temporaries during binary ops.
- Masks results with `AND r0,r0,#0xFFFF` post arithmetic.
- Modulo synthesized via division helper + multiply-subtract sequence.
- Shifts emitted with `MOV r0,r4,LSL r5` / `MOV r0,r4,LSR r5` (prototype; assumes shift amount already in low bits).

### Cortex-M (VecFever / Vextreme)
- Similar to ARM backend; vector table stub plus infinite loop after `main` returns.
- Same modulo / shift strategy as ARM (uses MVN for bitwise not).

## 10. Example
```
def add(a, b):
    return a + b

def main():
    s = add(3,5)
    n = -7
    t = 0
    if 1 < 2 < 3:
        t = s & 255
    bw = (s & 0x00FF) | (t ^ ~n)
    m = (bw << 3) % 16
    z = 0b1010 ^ m
    return s + t + n + bw + z
```

## 11. Current Limitations / Undefined Behavior
- Division by zero: leaves left operand (subject to change).
- More than 4 function parameters: ignored / presently unsafe.
- No scoping: all variables effectively module globals.
- No recursion guard or stack depth check.
- No negative integer literals (unary minus rewrite only).
- No string / data sections beyond variables.
- No constant folding across function boundaries.

## 12. Roadmap (Potential)
- Local variable allocation vs globals.
- Basic register allocation to reduce memory traffic.
- Array / vector primitives.
- Inline line-drawing API intrinsics.
- Improved error messages with spans.
- Test harness & golden assembly diffs.
- Optional signed arithmetic mode.

## 13. Contributing (Draft)
- Keep new language constructs minimal and consistent across all backends.
- Always implement feature in: lexer -> parser -> AST -> optimizer -> all backends -> example + manual update.
- Write deterministic code (avoid randomness, time-based data).

## 14. License
MIT (see `README.md`).

---
(End of Draft Section: This manual is a living document; update with each feature.)
