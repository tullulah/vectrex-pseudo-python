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
 - Vectrex runtime extras: automatic frame loop (unless --no-auto-loop), optional per‑frame audio silence, blink intensity toggle, debug initial vector draw, configurable bank padding

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

 Vectrex (6809 backend current built-ins & helpers):
 vectrex.frame_begin(intensity) : Wait_Recal + optional intensity + Reset0Ref (used manually only if auto loop disabled)
 vectrex.set_origin() : Reset0Ref (origin only)
 vectrex.set_intensity(i) : variable intensity (Intensity_a)
 vectrex.move_to(x, y) : absolute move (low bytes) via Moveto_d
 vectrex.print_text(x, y, ptr) : high-bit terminated string (last char bit7=1) via Print_Str_d
 vectrex.draw_line(x0,y0,x1,y1,intensity) : single segment using BIOS Draw_VL (delta saturated to -64..63)
 vectrex.draw_vl(ptr,intensity) : call BIOS Draw_VL with user vector list (y x y x ...; end flagged by bit7 in Y)
 vectrex.draw_to(x,y) : placeholder (updates current position only)

 Runtime injected helpers (no explicit Python call yet):
 - Blink intensity (enabled with --blink) toggles between $5F / $20 every frame.
 - Per-frame silence (default ON, disable with --no-per-frame-silence) writes zeros to AY registers to prevent random buzz on some emulators/hardware.
 - Debug init draw (small horizontal line) default ON; disable with --no-debug-draw.
 - Bank padding ( --bank-size N ) emits FILL/RMB so final ROM size == N (filled with $FF). Typical values: 4096, 8192.

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

Assemble generated Vectrex assembly (official BIOS/VIA/PSG symbols via always-included `../include/VECTREX.I`):
```
./tools/lwasm.ps1 --6809 --format=raw --output=game.bin tests/all_tests.asm
```

Bank padding:
If you pass `--bank-size 8192` (or another power-of-two) the emitted `.asm` auto-fills with $FF to reach that size, so the produced `*.bin` is already exactly the requested size (no external padding step). For multi-bank larger images you can concatenate banks or later introduce a mapper stage (future work).

Manual pad (only if you skipped --bank-size):
```
$b = [IO.File]::ReadAllBytes('game.bin'); $pad = 8192 - $b.Length; if($pad -gt 0){[IO.File]::WriteAllBytes('game8k.bin', $b + (,[byte]0xFF * $pad))}
```

Load the resulting `.bin` in a Vectrex emulator (VecX / ParaJVE / MAME).

## CLI (Simplificado)
Actualmente la herramienta expone un subcomando principal:
```
cargo run -- build <fuente.vpy> [--out <archivo>] [--target <vectrex|pitrex|vecfever|vextreme>] [--title T] [--bin]
```
En modo Vectrex minimal clásico la mayoría de flags antiguos fueron eliminados. Se generan:
- `<archivo>.asm`
- `<archivo>.bin` si se pasa `--bin` (usa lwasm local o script fallback `tools/lwasm.ps1`).

El `--title` del CLI puede ser sobrescrito desde el propio código fuente con directivas META (ver abajo).

## Directivas META (Vectrex)
Al inicio del `.vpy` puedes definir metadatos que sustituyen partes de la cabecera ROM:
```
META TITLE = "MI JUEGO"        # Máx 24 chars, se fuerza a MAYÚSCULAS y se limpian caracteres no alfanum/espacio
META COPYRIGHT = "g GCE 2025"  # Cadena mostrada en la primera línea (por defecto: g GCE 1998)
META MUSIC = "music1"          # Símbolo BIOS de música (por defecto music1)
META MUSIC = "0"               # Desactiva música (FDB $0000)
```
Sólo estos META afectan la cabecera actualmente. Altura/anchura/coords ($F8,$50,$20,$AA) están fijos.

Ejemplo mínimo hello:
```
META TITLE = "HELLO WORLD"
META COPYRIGHT = "g GCE 2025"
META MUSIC = "0"

def main():
    PRINT_TEXT(-0x50, 0x10, "HELLO WORLD")
```

Salida de cabecera generada (simplificada):
```
FCC "g GCE 2025"
FCB $80
FDB $0000
FCB $F8,$50,$20,$AA
FCC "HELLO WORLD"
FCB $80
FCB 0
```

## Estado de funcionalidades Vectrex recortadas
Se eliminó runtime extra, wrappers y padding automático para el modo clásico minimal; sólo se emiten llamadas BIOS directas y la cadena usada en PRINT_TEXT.

## License
MIT
