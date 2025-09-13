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
- Keywords: `def`, `return`, `for`, `in`, `range`, `if`, `elif`, `else`, `while`, `break`, `continue`, `and`, `or`, `not`, `let`
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

2. Local declaration (`let`):
```
let x = expression
```
Creates a local variable stored in the function's stack frame (one 16-bit slot currently rounded to 2 bytes on 6809, 4 bytes on ARM/Cortex-M for simplicity – may compact later). Lifetime = function invocation.

3. Assignment:
```
name = expression
```
Writes to an existing local if a `let name` appeared earlier in the same function; otherwise writes / creates a global variable.

3. For loop (range based):
```
for i in range(start, end, step):
    body
```
`step` is optional; default = 1 when omitted. Loop variable and bounds are reloaded each iteration for the comparison; loop runs while `i < end`.

For-loop induction variable locality:

- The loop variable (e.g. `i`) is always treated as an implicit local even without a preceding `let`. It is allocated a 2‑byte slot in the function's stack frame on all backends.
- It does not create or touch a `VAR_I` style global symbol, allowing shadowing and preventing global namespace pollution.
- Nested loops allocate additional slots (`i` at offset 0, `j` at offset 2, etc.).
- A specified `step` expression is evaluated after the body each iteration; if omitted, a constant 1 is synthesized.

Example with step:
```
for i in range(0, 6, 2):
    acc = acc + i
```
Generates a local slot for `i` and increments it by 2 each pass; no global `VAR_I` is emitted.

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
- Uses zero-page-like globals: `VAR_<NAME>` for globals, `VAR_ARGi` for call arguments, temporaries (`TMPLEFT`, `TMPRIGHT`, etc.).
- Locals (`let` and for-loop counters) allocated on stack via `LEAS -N,S` at function entry; each consumes 2 bytes; freed with `LEAS +N,S` before return.
- Multiplication / division call helper subroutines `MUL16`, `DIV16` (prototype) or inline shift peepholes for powers of two.

### ARM (PiTrex)
- Simple linear code, preserves no callee-saved registers (prototype). r4, r5 used as temporaries during binary ops.
- Locals allocated with `SUB sp, sp, #size` (2 bytes per local) and accessed by halfword ops `STRH/LDRH [sp,#off]`. Freed with corresponding `ADD sp, sp, #size` in epilogue.
- Masks results with `AND r0,r0,#0xFFFF` post arithmetic.
- Modulo synthesized via division helper + multiply-subtract sequence.
- Shifts emitted with `MOV r0,r4,LSL r5` / `MOV r0,r4,LSR r5` (prototype; assumes shift amount already in low bits).

### Cortex-M (VecFever / Vextreme)
- Similar to ARM backend; vector table stub plus infinite loop after `main` returns.
- Locals follow same stack frame scheme as ARM (2 bytes per local, STRH/LDRH).
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
- Shadowing across functions only (no nested scopes yet). Locals must be declared with `let` before first assignment to be local; otherwise assignment creates a global.
- No recursion guard or stack depth check.
- No negative integer literals (unary minus rewrite only).
- No string / data sections beyond variables.
- No constant folding across function boundaries.
 - Parser sólo reporta el primer error por archivo (multi-error recovery pendiente) aunque warnings heurísticos (como `POLYGON 2`) se agregan siempre.

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

## 15. Vectrex Minimal Mode & META Directives (Actual)

El backend Vectrex actualmente fuerza un modo "minimal clásico" para generar la cabecera estándar sin runtime adicional. Sólo se emite el código estrictamente necesario y las cadenas realmente usadas (sin duplicados manuales).

### 15.1 CLI Simplificado
Subcomando principal:
```
cargo run -- build fuente.vpy [--out salida] [--target vectrex] [--title T] [--bin]
```
- `--target` por defecto = `vectrex`.
- `--title` puede ser sobrescrito en el propio archivo con `META TITLE`.
- `--bin` genera además del `.asm` un `.bin` usando lwasm (o script fallback `tools/lwasm.ps1`).

Se han eliminado flags anteriores (blink, bank-size, debug, etc.) en este modo.

### 15.2 Directivas META Soportadas
Se colocan al inicio del archivo `.vpy` (antes de funciones):
```
META TITLE = "MI JUEGO"        # <=24 chars, se fuerza MAYÚSCULAS y se limpian caracteres no válidos
META COPYRIGHT = "g GCE 2025"  # Reemplaza línea de copyright (default: g GCE 1998)
META MUSIC = "music1"          # Símbolo BIOS (por defecto music1)
META MUSIC = "0"               # Desactiva música (puntero $0000)
```
Sólo estas claves afectan la cabecera por ahora. Los bytes de parámetros ($F8,$50,$20,$AA) están fijos.

### 15.3 Cabecera Generada (Estructura)
```
FCC "<COPYRIGHT>"
FCB $80
FDB <music_pointer | $0000>
FCB $F8,$50,$20,$AA
FCC "<TITLE>"   ; título normalizado
FCB $80          ; terminador título
FCB 0            ; terminador lista vectores en este modo minimal
```

### 15.4 Ejemplo
Fuente:
```
META TITLE = "HELLO WORLD"
META COPYRIGHT = "g GCE 2025"
META MUSIC = "0"

def main():
    PRINT_TEXT(-0x50, 0x10, "HELLO WORLD")
```
Genera cabecera con música desactivada (FDB $0000) y título/copyright.

### 15.5 Notas Futuras
- Posibles META adicionales (coordenadas/escala) aún no implementados.
- Limpieza de opciones legacy en estructuras internas pendiente (campos no usados en CodegenOptions).

## 16. DSL de Vector Lists (Nuevo)

Bloques `vectorlist nombre:` permiten definir listas de vectores compactas interpretadas por el runtime `Run_VectorList`.

Comandos soportados (insensibles a mayúsculas):
- `MOVE x y` : punto inicial absoluto (CMD_START)
- `RECT x1 y1 x2 y2` : rectángulo (4 segmentos)
- `POLYGON N x0 y0 ... xN-1 yN-1` : polígono cerrado (N≥2) usando líneas sucesivas (START + N segmentos)
- `CIRCLE cx cy r segs` : aproximación poligonal (segs≥3)
- `ARC cx cy r start_deg sweep_deg segs` : arco abierto subdividido (segs≥2)
- `SPIRAL cx cy r_start r_end turns segs` : espiral abierta (segs≥4)
- `ORIGIN` : Reset0Ref (CMD_ZERO) recarga integradores a (0,0)
- `INTENSITY val` : inserta comando de intensidad; si val ∈ [0..7] se mapea a presets ($1F,$3F,$5F,$7F); otro valor se toma directo (8 bits)

Normalización backend:
1. Se elimina ZERO inicial redundante si inmediatamente va un START.
2. Se inserta un START (0,0) inicial si no existe.
3. La primera INTENSITY se mueve tras ese START.
4. START duplicados consecutivos en mismas coords se colapsan.
5. ZERO consecutivos se colapsan.

Salida:
```
VL_MAZE:
    FCB <count>
    FCB $yy,$xx,CMD_START ; START to (...)
    ...
    FCB $00,$00,CMD_END ; END
```
Los comentarios muestran deltas y coordenadas absolutas para depuración.

Ejemplo mínimo:
```
vectorlist demo:
    INTENSITY 0x5F
    MOVE -16 -16
    RECT -16 -16 16 16
    ORIGIN
    POLYGON 4 0 -16 16 0 0 16 -16 0
```

Uso en `main`:
```
def main():
    vectrex_draw_vectorlist("demo")
```

Limitaciones actuales:
- Sin clipping automático ni escalado.
- Coordenadas se truncan a 8 bits (signed) durante emisión (rango efectivo -128..127).
- Las figuras complejas (círculo/espiral) pueden generar muchos segmentos -> parpadeo; ajustar `segs`.
 - Diagnostics: line/col mostrados en el panel son 1-based (internamente 0-based). Ruta Windows con `C:` ya soportada por extractor robusto.

Buenas prácticas:
- Insertar `ORIGIN` entre grupos largos de segmentos para repartir brillo.
- Usar intensidades más bajas para listas densas.
- Dividir escenas grandes en varias `vectorlist` y dibujarlas en frames alternos si hay flicker.

