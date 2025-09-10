# Multi-Target Pseudo-Python Vector Compiler (Prototype)

Rust prototype compiler turning a constrained Python-like subset into assembly for multiple vector platforms:

Targets:
- Vectrex (Motorola 6809)
- PiTrex (ARM)
- VecFever (Cortex-M)
- Vextreme (Cortex-M)

## Current Feature Set
- Functions (definitions, returns)
- Up to 4 positional parameters (simple prototype ABI)
- Statements: assignment, let (local declaration), for (range), while, if/elif/else, break, continue
- Expressions: literals, identifiers, calls, arithmetic (+ - * / %), bitwise (& | ^ << >> ~), comparisons (== != < <= > >=), chained comparisons (a < b < c), logical (and/or/not), unary +/-
- Literals: decimal, hexadecimal (0x...), binary (0b...)
- Comments: `#` to end of line
- Optimizations: constant folding (arithmetic, bitwise, shifts, modulo, bitnot), algebraic identities, constant propagation, dead code elimination, dead store elimination, backend peepholes (power-of-two mul/div, simple patterns)
- Uniform 16-bit unsigned arithmetic semantics across all backends
- Basic power-of-two multiply/divide lowering to shifts
- Bitwise / arithmetic identity simplifications (x&0, x|0, x^0, x&0xFFFF, x*1, x+0, etc.)

## Status Notes
- All arithmetic ops implemented for all backends (Add/Sub/Mul/Div with helper routines or shifts)
- Bitwise ops implemented and optimized
- Chained comparisons lowered to logical conjunction with short-circuiting
- Locals: `let name = expr` allocates a stack slot (ARM / Cortex-M now 2 bytes per 16-bit local via STRH/LDRH; 6809 uses 2 bytes). Non-`let` assignment to a new name creates/uses a global. Re-assigning a `let` variable stays local.
- No register allocation yet (globals + stack slots used for temps/params)

## Example (`tests/example.vpy`)

## Example Source (`tests/example.vpy`)
```
def main():
    x = 0
    for i in range(0, 16, 4):
        line(0, 0, i)
    if x:
        line(0,0,0)
```

Build (default target = vectrex):
```
cargo run -- build tests/example.vpy
```

Select explicit target:
```
cargo run -- build tests/example.vpy --target pitrex
cargo run -- build tests/example.vpy --target vecfever
cargo run -- build tests/example.vpy --target vextreme
cargo run -- build tests/example.vpy --target all    # produce los 4 ensamblados
```
Output file: `example.asm` (overwritten per target invocation unless you redirect).

Redirect to keep each:
```
cargo run -- build tests/example.vpy --target vectrex   > vectrex.asm
cargo run -- build tests/example.vpy --target pitrex    > pitrex.asm
cargo run -- build tests/example.vpy --target vecfever  > vecfever.asm
```

## Programming Manual
See `MANUAL.md` for the evolving language and ABI specification.

## Roadmap (Short-Term)
- Local vs global variable distinction / stack frame model
- Register allocation & temp reuse
- Arrays / structured data
- Strength reduce: modulo by power-of-two -> bitmask, combined shift+mask peepholes
- Engine / BIOS intrinsic hooks
- Test harness (golden assembly diffs)
- Improved diagnostics with spans

### Arithmetic / Helpers
6809 uses `MUL16` / `DIV16` helper routines (prototype) or shift peepholes for powers of two. ARM / Cortex-M use inline software loops for 32-bit widen-narrow mult/div then mask to 16 bits.

## License
MIT
