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
- Math & trig built-ins: sin, cos, tan (also via math.sin etc.), abs/min/max/clamp
- Vectrex built-ins (prototype): vectrex.set_origin, vectrex.set_intensity, vectrex.move_to, vectrex.print_text, vectrex.draw_line (skeleton), vectrex.draw_to (TODO)

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
- Flesh out Vectrex drawing: implement draw_to and draw_line actual vector rendering
- Test harness (golden assembly diffs)
- Improved diagnostics with spans

### Arithmetic / Helpers
6809 uses `MUL16` / `DIV16` helper routines (prototype) or shift peepholes for powers of two. ARM / Cortex-M use inline software loops for 32-bit widen-narrow mult/div then mask to 16 bits.

### Built-ins Reference (Evolving)

General math:
- abs(x), min(a,b), max(a,b), clamp(v, lo, hi)
Trig (argument 0..127 covers full circle, 7-bit index):
- sin(a), cos(a), tan(a) (values scaled to -127..127). Namespace forms math.sin etc. are aliases.

Vectrex (6809 backend currently implemented / partial):
- vectrex.set_origin() : WAIT_RECAL + RESET0REF (stabilize + zero reference)
- vectrex.set_intensity(i) : sets beam intensity using low byte of i
- vectrex.move_to(x, y) : positions beam (absolute) using BIOS MOVETO_D (low bytes)
- vectrex.print_text(x, y, ptr) : prints null-terminated string at position
- vectrex.draw_line(x0,y0,x1,y1,intensity) : sets intensity, moves to start (line drawing TODO)
- vectrex.draw_to(x,y) : planned (draw from current to x,y)

Example drawing demo: `examples/vectrex_draw_demo.vpy`

### Tooling: Assembling to a Vectrex ROM

Assembler: LWTOOLS (`lwasm`). Two install paths on WSL:

1. Homebrew (fast, no source build):
```
pwsh ./tools/install_lwtools_wsl.ps1 -UseBrew
```
2. (Fallback – currently disabled until a public mirror is confirmed) Source clone & make.

Verify:
```
wsl lwasm --version
```

Assemble generated Vectrex assembly:
```
./tools/lwasm.ps1 --6809 --format=raw --output=game.bin tests/all_tests.asm
```

Pad to 32K (PowerShell):
```
$b = [IO.File]::ReadAllBytes('game.bin'); $pad = 32768 - $b.Length; if($pad -gt 0){[IO.File]::WriteAllBytes('game32k.bin', $b + (,[byte]0x00 * $pad))}
```

Load `game32k.bin` in a Vectrex emulator (VecX / ParaJVE / MAME).

## License
MIT
