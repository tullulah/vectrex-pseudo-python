# vectrex-pseudo-python

## IDE (Electron Shell)

Para arrancar la IDE de escritorio (Electron + React + Monaco + LSP):

```
./run-ide.ps1
```

Esto levanta:
- Vite (frontend React) en `ide/frontend`
- Electron shell en `ide/electron` (menú nativo oculto; la UI expone su propio menú)

El antiguo runtime Tauri ha sido eliminado; Electron es ahora el único shell soportado.

### Nueva función: Run (Compilar y Cargar en Emulador)
Dentro del panel del emulador ahora hay un botón **Run** que:
1. Guarda (si está sucia) la pestaña activa `.vpy`.
2. Invoca el binario `vectrexc` con `build <archivo>.vpy --target vectrex --bin`.
3. Genera `<archivo>.asm` y `<archivo>.bin` (auto-pad a 8K si es necesario).
4. Carga el `.bin` resultante directamente en el emulador embebido y empieza a ejecutar (auto Play).

Salida / errores:
- STDOUT/STDERR del compilador se emiten a canales IPC (`run://stdout` / `run://stderr`). Un parsing simple de líneas `file:line:col: mensaje` se traduce a diagnósticos que se podrían integrar en el panel de errores (placeholder actual: consola).
- Conflictos de guardado (archivo cambiado en disco) devuelven `conflict` para evitar sobreescrituras inesperadas.

Requisitos:
- Haber compilado previamente el binario `vectrexc` (`cargo build -p vectrex_lang --bin vectrexc`). El resolvedor busca en `target/{debug,release}` en raíz y en `core/target/...`.

Limitaciones iniciales:
- No hay aún cancelación de compilación.
- Diagnósticos de runtime no se mezclan con los del LSP (pendiente unificar canal).
- Sólo target `vectrex` expuesto vía Run.
- Emulación de hardware (VIA/PSG) todavía no implementada; se usan atajos de BIOS para extraer vectores.

## Nueva Ruta: Núcleo Rust en WebAssembly (WASM)

Se está migrando el emulador 6809 a un único núcleo en Rust exportado a la IDE (Electron / navegador) vía WebAssembly, eliminando la duplicación del emulador TypeScript (`ide/electron/src/emu6809.ts`).

Estado actual:
- Se agregó dependencia opcional `wasm-bindgen` (feature `wasm`).
- Archivo `core/src/wasm_api.rs` expone un wrapper `WasmEmu` con métodos:
    - `new()`
    - `load_bios(&[u8])` (4K ó 8K)
    - `load_bin(base, &[u8])`
    - `reset()`
    - `step(count)`
    - `run_until_wait_recal(max_instr)` (heurística de frame usando `frame_count`)
    - `registers_json()`
    - `memory_ptr()` (para crear `Uint8Array(mem.buffer, ptr, 65536)` en JS)
    - `metrics()`

### Compilación a WASM
1. Instalar target:
```
rustup target add wasm32-unknown-unknown
```
2. Compilar:
```
cargo build -p vectrex_lang --target wasm32-unknown-unknown --release --features wasm
```
3. Generar envoltorio JS/TS con `wasm-bindgen` (ejemplo output en `dist-wasm/`):
```
wasm-bindgen --target web --out-dir dist-wasm target/wasm32-unknown-unknown/release/vectrex_lang.wasm
# Para Electron / Node:
wasm-bindgen --target nodejs --out-dir dist-wasm target/wasm32-unknown-unknown/release/vectrex_lang.wasm
```
4. Consumir en código (simplificado):
```ts
import init, { WasmEmu } from './dist-wasm/vectrex_lang.js';
await init();
const emu = new WasmEmu();
emu.load_bios(biosBytes); // Uint8Array
emu.reset();
// Bucle de frame aproximado
for(;;){
    emu.run_until_wait_recal(200000); // o un límite de seguridad
    const regs = JSON.parse(emu.registers_json());
    // Métricas (incluye integrator_*):
    const metrics = JSON.parse(emu.metrics_json());
    requestAnimationFrame(()=>{/* render */});
}
```

### Plan de Retirada del Emulador TypeScript
Fases previstas:
1. (Hecho) Exponer núcleo Rust vía WASM.
2. Añadir API adicional para obtención de segmentos vectoriales directamente desde Rust (hoy aún depende de intercepts TS).
3. Reemplazar todas las llamadas a `globalCpu` en `ide/electron` por `WasmEmu`.
4. Eliminar `emu6809.ts` y tests asociados, migrando los harness a Rust (tests de unidad) o a un wrapper JS mínimo.
5. Simplificar IPC: ya no se precisa sincronizar dos implementaciones.

Archivo de migración sugerido (pendiente): `MIGRATION_WASM.md` documentará las equivalencias de API.

Ventajas:
- Una sola fuente de verdad para flags, modos indexados, temporización.
- Menor divergencia futura al añadir opcodes o fidelidad VIA.
- Posibilidad de reutilizar el mismo binario en VSCode, Electron y navegador sin portar lógica.

Limitaciones actuales:
- API WASM de vectores ya disponible (ver sección "Exportación de Segmentos Vectoriales" más abajo); aún no hay modo zero‑copy sin copia intermedia ni ring buffer persistente.
- Sin soporte de PSG / audio todavía en el wrapper.
- Temporización de `run_until_wait_recal` es heurística (basada en BIOS call); se refinará con Timer1 auténtico.

Para activar la compilación WASM en integraciones CI, añadir un job que ejecute los pasos arriba y empaquete `dist-wasm`.


### BIOS Real (Vectrex)
La IDE carga la BIOS original (liberada) desde una ruta fija por ahora: `core/bios/vectrex.bin` (o el primer `.bin` si no hay exact match). Más adelante esta ruta será configurable.

Tamaños aceptados:
- 4KB (4096 bytes) estándar.
- 8KB (8192 bytes) dump duplicado/padding: se toma la mitad superior para mapear 0xF000-0xFFFF.

Al reset se reinstala la imagen y se usa el vector de arranque en 0xFFFE/0xFFFF para el PC inicial.

### Modo de Vectores (Transición)
El emulador mantiene dos modos internos (`vectorMode`):
- `intercept` (actual por defecto): Intercepta WAIT_RECAL / INTENSITY / DRAW_VL para extraer segmentos de forma directa (rápido, sin temporización).
- `via` (futuro): Permitirá ejecutar las rutinas BIOS completas y derivar los segmentos a partir de actividad de hardware (VIA 6522).

Actualmente el modo `via` es un placeholder—la selección manual todavía no está expuesta en UI. Cuando se implemente la temporización se retirarán las intercepciones en ese modo.

### VIA 6522 (Esqueleto Inicial)
Se ha añadido un array de 16 registros (0xD000-0xD00F) listo para simular:

Estado actual (parcial):
 - Escrituras experimentales (modo `via`): se interceptan accesos 0xD000-0xD00F y, de forma provisional, se interpretan:
     - 0xD000: delta X (signed 8-bit)
     - 0xD001: delta Y (signed 8-bit) — si el haz está en modo draw se emite un segmento (prev->nuevo)
     - 0xD002: control (bit0 = start draw, bit1 = stop draw)
     - 0xD003: intensidad (0..127)
     Estas asignaciones NO corresponden todavía al mapa real del VIA 6522; son un andamiaje para migrar la lógica a escritura hardware real antes de modelar integradores.
 - Se añadieron IFR (0xD00D) e IER (0xD00E) simplificados:
     - Lectura IFR devuelve bit7 = OR de bits 0-6 (cualquier flag activo)
     - Escritura IFR limpia los bits puestos a 1 (como en 6522 real)
     - Escritura IER con bit7=1 habilita bits marcados; bit7=0 deshabilita
     - De momento solo Timer1 (bit6) se marca en underflow.
 - Timer1: contador (prot) decrementa por ciclos; al underflow se recarga desde latch y marca IFR bit6; si `WAIT_RECAL` estaba pendiente, produce frameReady.
 - IRQ simplificado & WAI:
     - Se añadió flag I (mask IRQ) al registro de condición simplificado.
     - ORCC (0x1A) y ANDCC (0x1C) permiten modificar bits incluyendo I.
     - Cuando IFR bit6 (Timer1) coincide con IER bit6 y I=0, se atiende IRQ: se apila CC y PC (forma mínima) y se salta al vector 0xFFF8/0xFFF9.
     - WAI (0x3E) detiene la ejecución normal; el bucle sólo avanza timers hasta que llega un IRQ válido.
     - Modelo reducido: no se apilan A,B,DP,X,Y,U todavía; se añadirá más fidelidad luego.
    - Instrumentación VIA (modo `via`): se captura una lista limitada (<=5000) de eventos de escritura `{pc, reg, val}` por frame. La ruta provisional de deltas directas X/Y fue eliminada; ahora los segmentos se sintetizan heurísticamente.
    - Integrador inicial: ahora ORB (0xD000) fija velocidades (nibbles firmados) y cada instrucción integra posición según ciclos consumidos (tabla aproximada). ORA (0xD001) actúa como latch de intensidad y ACR (0xD00B) bits 0/1 activan/desactivan trazo. La heurística de síntesis al final de frame fue eliminada.
    - Flag `hardwareVectors`: activado automáticamente en modo `via`; la reconstrucción ya no usa listas vectoriales interceptadas.

Próximos pasos hacia precisión:
1. Simular decremento de T1 y generar evento de frame (vsync) -> reemplaza el flag directo en WAIT_RECAL.
2. Implementar acumulación analógica aproximada: writes a registros DAC/integradores generan deltas en coordenadas.
3. Capturar transiciones de blank/draw para dividir en segmentos con intensidad correcta.
4. Migrar DRAW_VL de intercept a ejecución BIOS genuina.
5. Jitter / normalización para representar limitaciones físicas.
6. (Posterior) Entrada & PSG.

Variables de entorno útiles:
- `VPY_IDE_VERBOSE_RUN=1` para logging adicional en consola (proceso Electron principal).

## Separación del Emulador (Refactor Reciente)

## Nuevos Documentos de Referencia (Timing & Vectores)

Se añadieron documentos técnicos en `docs/` que describen el nuevo modelo determinista y el backend de vectores por haz:

- `docs/TIMING.md`: Explica `cycle_frame` (autoridad), `bios_frame` (observacional), acumulación de ciclos y estadísticas por frame. Incluye razones para eliminar heurísticas de IRQ y cómo se sincronizan los timers VIA.
- `docs/VECTOR_MODEL.md`: Detalla el integrador (haz analógico simplificado), reglas de fusión de segmentos, auto-drain, métricas expuestas y plan de migración para retirar el parser legacy de listas vectoriales.

Estos recursos reemplazan comentarios dispersos y sirven como base para futuras mejoras (curva de brillo, ramp delays). Actualizar siempre que cambien las heurísticas o estructuras principales.

El emulador 6809 + VIA se ha extraído del crate `core` (antes `vectrex_lang`) a un crate independiente `vectrex_emulator` ubicado en `emulator/` dentro del workspace. Cambios clave:

- Código antiguo en `core/src/emulator` fue retirado (dejado vacío) para evitar duplicación.
- Binarios de ejemplo (`core/src/bin/emu.rs`, `run_bios.rs`) ahora importan `CPU` desde `vectrex_emulator`.
- La lógica WASM y bindings deben apuntar al nuevo crate si se recompila para web; el wrapper anterior (`wasm_api.rs`) también se movió a `emulator/src/wasm_api.rs`.
- Eliminado el emulador TypeScript: ya no se mantiene `emu6809.ts`; toda la emulación vive únicamente en Rust.

### Motivos
1. Aislar la emulación para poder versionarla y testearla de forma independiente.
2. Reducir tiempos de compilación incrementales del compilador / LSP.
3. Preparar futura publicación en crates.io (`vectrex_emulator`).

### Impacto en Código Existente

Reemplaza imports:
```rust
// Antes
use vectrex_lang::emulator::CPU;
// Ahora
use vectrex_emulator::CPU;
```

Si tu código dependía de métodos de métricas (`metrics_pretty`, etc.) que no están expuestos todavía en el crate externo, tendrás que eliminarlos temporalmente o abrir un issue para volver a exportarlos de forma estable.

### Próximos pasos sugeridos
- Añadir API pública de métricas y snapshot estructurado (sin necesidad de strings).
- Exponer extracción de segmentos vectoriales por frame directamente desde Rust.
- Tests de regresión de opcodes en `emulator/tests/`.

Si encuentras referencias rotas al antiguo módulo, ejecuta búsqueda global de `vectrex_lang::emulator` y actualiza al nuevo path.

### Interrupciones 6809 (IRQ / FIRQ / SWI / NMI) – Estado Actual
Se ha implementado un modelo de interrupciones más fiel del 6809 distinguiendo entre IRQ (frame completo) y FIRQ (frame parcial), junto con el retorno por `RTI` y la instrucción `WAI`.

Tabla de vectores (direcciones altas en $FFxx):

| Vector | Dirección | Uso actual |
|--------|-----------|------------|
| SWI3   | $FFF0/$FFF1 | Implementado (frame completo, opcode 0x11 0x3F) |
| SWI2   | $FFF2/$FFF3 | Implementado (frame completo, opcode 0x10 0x3F) |
| FIRQ   | $FFF4/$FFF5 | Implementado (parcial) |
| IRQ    | $FFF6/$FFF7 | Implementado (completo) |
| SWI    | $FFF8/$FFF9 | Implementado (frame completo) |
| NMI    | $FFFA/$FFFB | Implementado (completo, prioridad máxima) |
| RESET  | $FFFC/$FFFD | Vector de reset (ya leído al arranque) |
| START* | $FFFE/$FFFF | Alias RESET (PC inicial) |

(*START se refiere al vector de arranque estándar del Vectrex.)

#### Apilado (Stack Frames)
El 6809 real define que `IRQ`, `SWI`, `NMI` apilan un frame completo (incluyendo todos los registros) mientras que `FIRQ` apila solo una parte. Nuestro modelo actual implementa:

Orden de push interno adoptado (de izquierda a derecha se realiza primero el push de PC y al final CC):
```
IRQ  : PC, U, Y, X, DP, B, A, CC   (E=1, I=1 tras servir la interrupción)
FIRQ : PC, CC                      (E=0 se mantiene, I=1 tras servir)
```
Notas:
- Para registros de 16 bits (PC, U, Y, X) se empuja primero el byte alto y luego el bajo ("high-first"). Esto simplifica la lógica de reconstrucción al hacer pops en orden inverso.
- El bit E (entendido aquí como bandera que indica frame extendido) se fuerza a 1 únicamente en IRQ (y lo estará también en futuras SWI/NMI). En FIRQ permanece 0 para señalar frame parcial.
- El bit I (mask IRQ) se activa (1) al entrar a cualquier servicio de interrupción para evitar reentradas inmediatas.

#### RTI
La instrucción `RTI` realiza:
1. Pop de CC (siempre primero).
2. Si (CC.E == 1) entonces pop de A, B, DP, X, Y, U (en ese orden) y luego pop de PC.
3. Si (CC.E == 0) sólo pop de PC (frame parcial FIRQ).

De esta forma un handler FIRQ que modifica A/B (u otros registros) deja persistentes los cambios porque esos registros no se apilaron.

#### WAI
`WAI` coloca a la CPU en estado de espera. El bucle de emulación sigue avanzando hardware (timers VIA, etc.) y en cuanto se detecta una interrupción pendiente y habilitada:
- Se sirve inmediatamente (incluso si la petición ya estaba encolada antes del WAI) y la CPU retoma la ejecución en la rutina de servicio.
- Ya no se depende de artificios (p.ej. llamadas simuladas via stack manual) — se usa el frame real correspondiente.

#### Ejemplo mínimo de handler IRQ (ensamblador)
```
    ORG   $C800
IRQ_Handler:
    CLRA            ; trabajo del handler (ejemplo)
    RTI             ; restaura frame completo

    ORG   $FFF6
    FDB   IRQ_Handler
```

#### Ejemplo mínimo de handler FIRQ (parcial)
```
    ORG   $C820
FIRQ_Handler:
    INC   <SomeZeroPageVar  ; modifica B/A/etc. (persistirá tras RTI)
    RTI

    ORG   $FFF4
    FDB   FIRQ_Handler
```

#### Prioridad

## Vector Event Capture (Removed)
Legacy high-level vector event markers (MoveTo / Draw_VL) have been removed. Future visualization will rely on integrator segment drain APIs and metrics only.

## Running the BIOS with the New Emulator

To run the Vectrex BIOS using the refactored emulator crate:

```
cargo run -p vectrex_lang --bin run_bios
```

Optional environment variables:

| Variable | Default | Purpose |
|----------|---------|---------|
| `BIOS_FILE` | auto-search | Explicit path to BIOS image (4K or 8K) |
| `BIOS_MAX_STEPS` | 200000 | Max instruction steps before halting |
| `BIOS_TRACE_STEPS` | 128 | Disable trace after N instructions |
| `BIOS_DUMP_VECTORS` | off | Emit per-frame vector event summaries |

The trace initially logs opcode execution until the configured window elapses. When vector enrichment advances, a companion binary (or flag) will dump reconstructed line segments suitable for immediate rendering.

## Crate Split Recap (Quick Reference)

Old path: `vectrex_lang::emulator::{CPU,...}` → New path: `vectrex_emulator::{CPU, Bus, Via6522}`.

Add dependency in `Cargo.toml` of another crate inside the workspace:
```
vectrex_emulator = { path = "../emulator" }
```

WASM wrappers consuming the emulator should transition to import from the new crate or re-export through a façade module if version isolation needed.

## Planned Enhancements (Roadmap Snapshot)

- Full opcode parity (remaining indexed/extended variants, TFR/EXG, MUL, CCR edge cases).
- Cycle-accurate IRQ/FIRQ latency and better WAI timing alignment.
- VIA step modeling for precise beam path reconstruction.
- (Hecho) API de segmentos vectoriales (JSON + memoria compartida) para leer `{x0,y0,x1,y1,intensity,frame}` por frame.
- Optional deterministic mode (fixed cycle budget per frame) for reproducible tests.
- Metrics API (already internally counting opcodes) exposed as stable struct in public crate surface.

Orden de servicio actual: NMI > FIRQ > IRQ > (SWI/SWI2/SWI3 son sincrónicas al decodificar la instrucción, no vía polling).

#### Estado y Limitaciones Pendientes
- Ciclos exactos por instrucción: temporizado aún aproximado; muestreo de IRQ/FIRQ/NMI frente a límites de instrucción podría refinarse.
- Anidamiento complejo: no se han hecho pruebas exhaustivas con reactivación manual de I dentro de handlers para interrupciones anidadas.
- Verificación cruzada con un core 6809 de referencia sigue pendiente para confirmar orden y bits finos (H/V) en todos los casos.

#### Resumen Rápido
- IRQ: frame completo, E=1, preserva todos los registros del programa llamante.
- FIRQ: frame parcial, E=0, permite handlers rápidos que modifican registros sin coste de apilado extra.
- RTI: decide restauración completa observando E.
- Diseño de push high-first unifica la secuencia de pops y reduce código especial.

Notas adicionales:
- SWI (0x3F), SWI2 (0x10 0x3F) y SWI3 (0x11 0x3F) usan el mismo mecanismo interno `service_swi_generic` que apila frame completo y pone F=1, E=1, I=1.
- ORCC (0x1A) y ANDCC (0x1C) ya operan sobre la máscara completa EFHINZVC. Ejemplo: `ORCC #$10` activa I (enmascara IRQ); `ANDCC #$EF` limpia I.
- Próximo ajuste pendiente: temporización más precisa (ciclos reales por instrucción e interleaving de IRQ a mitad de instrucción cuando corresponda).

## IDE WASM Emulator Panel (Controls & Metrics)

The new React-based `EmulatorPanel` (WASM) now exposes basic runtime controls and richer metrics:

Controls (top-right of the panel):
- Play: resumes the per-frame loop (requestAnimationFrame driving `run_until_wait_recal`).
- Pause: halts frame execution but leaves state intact (registers, memory, metrics frozen until resumed).
- Reset: invokes `reset()` on the WASM core (re-runs BIOS reset vector) and clears recent vector events & frame counter in UI (underlying opcode counters are preserved by design—reset currently only resets CPU state; if a metrics clear is needed we can add a `metrics_reset()` function later).

Status Indicators:
- Status: running | stopped (derived from store state; paused is represented as `stopped` currently—future refinement may add a distinct `paused`).
- Frames: Reflects `registers.frame_count` (incremented each WAIT_RECAL boundary / BIOS frame heuristic).
- Last events: Count of vector events drained in the most recent frame (currently high-level markers MoveTo / DrawVectorList).
- BIOS: loaded | missing (panel attempts auto-load from `bios.bin`, `/bios.bin`, or `/core/src/bios/bios.bin`). If missing, the CPU may not progress meaningfully—drop a valid BIOS image into one of those paths to enable full execution.

Metrics Dashboard (Output panel):
- Cycles (cumulative) and average cycles per frame (cycles / frames).
- Draw VL count & last intensity (captured from intercepted BIOS routines).
- Unimplemented opcode counters with a unique opcode badge list.
- Top opcodes table (sorted by execution count, truncated to top N for clarity).

Extending Metrics:
To add more fields, edit `emulator/src/wasm_api.rs` `JsMetrics` struct and adjust `metrics_json()`; then update `MetricsSnapshot` in `ide/frontend/src/emulatorWasm.ts` and render logic in `OutputPanel.tsx`.

Program Loading (Future):
At present, the panel only auto-loads BIOS. A helper `loadProgram(bytes, base=0xC000)` is available on the `EmulatorService` to map cartridge binaries; UI wiring (drag & drop / Run integration) will be added in a subsequent iteration.

Performance Notes:
- JSON serialization is performed roughly once per second for the metrics polling loop; vector events are drained every frame. For higher-frequency metric updates, consider a shared memory snapshot struct and a typed array view to avoid repeated JSON parsing.

Troubleshooting:
1. BIOS shows as `missing`: ensure the file is served by the dev server (e.g. place `bios.bin` in `ide/frontend/public/`).
2. No frame increments: likely missing BIOS or an unimplemented opcode encountered early; inspect unimplemented list.
3. Large opcode unimplemented set: verify all extended opcodes are wired (check `cpu6809.rs` opcode dispatch). Prioritize opcodes reported earliest (the `first_unimpl` field).

## Exportación de Segmentos Vectoriales (Integrator Backend)

El backend único de vectores es ahora el integrador analógico simplificado. Cada trazo generado produce uno o más segmentos con:

```
struct BeamSegment {
        x0: f32, y0: f32,  // Coordenadas iniciales normalizadas (-1..1 aprox.)
        x1: f32, y1: f32,  // Coordenadas finales
        intensity: u8,     // 0..255 (curva lineal actual, sujeta a cambio)
        frame: u32         // Número de frame en que se generó
}
```

Se exponen dos rutas desde WASM (feature `wasm` activada en `vectrex_emulator`):

1. JSON (drain vs peek)
     - `integrator_segments_json()` (mutable) drena el buffer interno (lo deja vacío) y devuelve `"[[x0,y0,x1,y1,intensity,frame], ...]"`.
     - `integrator_segments_peek_json()` (inmutable) genera la misma estructura pero sin limpiar el buffer (útil para inspección / debugging).

2. Memoria compartida (copia estructurada eficiente)
     - `integrator_segments_ptr()` devuelve un puntero (`*const u8`) a un staging buffer interno (`Vec<BeamSegmentC>`) rellenado en la llamada.
     - `integrator_segments_len()` número de segmentos disponibles en ese staging buffer.
     - `integrator_segment_stride()` tamaño en bytes de cada entrada (estructura C empaquetada con padding explícito para alineación de 8 bytes principal: 24 bytes actualmente).
     - `integrator_drain_segments()` vacía el buffer interno del integrador (equivalente a drenar sin leer datos).

Estructura C (`repr(C)`) para lectura directa en JS/TS mediante `Uint8Array` + `DataView` / `Float32Array`:

```rust
#[repr(C)]
pub struct BeamSegmentC {
        pub x0: f32,
        pub y0: f32,
        pub x1: f32,
        pub y1: f32,
        pub intensity: u16, // mismo valor que u8, extendido para posible gamma futura
        pub frame: u32,
}
// stride reportado por integrator_segment_stride()
```

Ejemplo JS (asumiendo instancia `emu: WasmEmu`):

```ts
function readSegmentsShared() {
    const ptr = emu.integrator_segments_ptr();
    const len = emu.integrator_segments_len();
    const stride = emu.integrator_segment_stride(); // p.ej. 24
    const mem = (emu as any).memory || wasmMemory; // según glue de wasm-bindgen
    const bytes = new Uint8Array(mem.buffer, ptr, len * stride);
    const segments = [];
    for (let i=0; i<len; i++) {
        const off = i*stride;
        const dv = new DataView(bytes.buffer, bytes.byteOffset + off, stride);
        const x0 = dv.getFloat32(0, true);
        const y0 = dv.getFloat32(4, true);
        const x1 = dv.getFloat32(8, true);
        const y1 = dv.getFloat32(12, true);
        const intensity = dv.getUint16(16, true);
        const frame = dv.getUint32(20, true);
        segments.push({x0,y0,x1,y1,intensity,frame});
    }
    return segments;
}
```

Notas de uso / rendimiento:
- La llamada `integrator_segments_ptr()` crea (o reutiliza) un vector staging interno; si se vuelve a llamar invalida la vista anterior (no conserves referencias tras el siguiente frame).
- Para un flujo típico por frame: `run_until_wait_recal()` -> leer segmentos -> (opcional) `integrator_drain_segments()` si se usó `peek` previamente.
- Mezclar `peek_json` y shared memory está permitido; sólo el método `*_json()` variante drain limpia el buffer original.
- En un futuro se podría añadir modo "auto drain" (ya existe flag interno `integrator_auto_drain`) expuesto para vaciar automáticamente tras cada frame y reducir crecimiento no deseado.

Compatibilidad futura:
- El layout de `BeamSegmentC` puede ampliarse (p.ej. añadir `u8 flags`, `u8 pad`, gamma pre-aplicada). El stride es consultable dinámicamente para no depender de un tamaño fijo.
- Se planea incorporar correcciones físicas (ramp up/down, bloom). Eso podría aumentar la densidad de segmentos por frame.

Depuración rápida:
```ts
console.log('segments drain json', JSON.parse(emu.integrator_segments_json()).length);
console.log('segments peek json', JSON.parse(emu.integrator_segments_peek_json()).length); // igual que anterior si no se drenó
readSegmentsShared().slice(0,5).forEach(s=>console.log(s));
```

Si `integrator_segments_len()` devuelve 0 consistentemente con la BIOS corriendo, verifica que no se haya activado un modo de fusión agresivo (`set_integrator_merge_lines(false)` para desactivar merge) o que el frame loop realmente haya avanzado (`cycle_frame` incrementándose).


## Quick Start (WASM Dev Loop)

1. Build Rust (debug, native + wasm feature):
```
cargo build -p vectrex_emulator --features wasm
```
2. Ensure the generated `wasm_bindgen` JS glue (if using a manual build path) is placed or symlinked under `ide/frontend/src/wasm/` as `vectrex_emulator.js` plus the `.wasm` binary; alternatively use the existing workspace script that already stages these outputs.
3. Start the IDE shell:
```
./run-ide.ps1
```
4. Open the Emulator panel (docked) – it will attempt BIOS auto-load and begin execution.
5. Open Output panel to observe live metrics.

If the import path `./wasm/vectrex_emulator.js` fails, verify that the bundler (Vite) copied or built the WASM artifact into `src/wasm/` or adjust the import in `emulatorWasm.ts` to the actual relative path.

## Migration Appendix: Legacy TypeScript Emulator Removal

Removed elements:
- `ide/electron/src/emu6809.ts` (and associated IPC wiring for stats / stepping).
- Any `globalCpu` references replaced by `globalEmu` service.

Equivalent API Mapping:
| Legacy TS | New WASM Service |
|-----------|------------------|
| `globalCpu.step(n)` | `globalEmu.runFrame()` (until WAIT_RECAL) or future fine-grained step method |
| `globalCpu.getRegisters()` | `globalEmu.registers()` (JSON snapshot) |
| `globalCpu.getStats()` | `globalEmu.metrics()` |
| `globalCpu.reset()` | `globalEmu.reset()` |
| (none) load BIOS | `globalEmu.loadBios(bytes)` |

Pending parity tasks (not yet ported): per-instruction stepping UI, memory viewer, metrics reset, and vector segment coordinate extraction.

---
Revision: WASM panel controls & metrics integration (Sept 2025)

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
- Vectrex built-ins (prototype): vectrex.set_origin, vectrex.set_intensity, vectrex.move_to, vectrex.print_text, vectrex.draw_line (skeleton), vectrex.draw_to (TODO), draw_polygon macro (constante)
- Vectorlist DSL: embebido en `.vpy` mediante bloques `vectorlist nombre:` con comandos declarativos (MOVE, RECT, POLYGON, CIRCLE, ARC, SPIRAL, ORIGIN, INTENSITY) que se expanden a una lista compacta (count + triples y,x,cmd) interpretada por `Run_VectorList`.
- Runtime minimal: bucle de frame automático, Reset0Ref + intensidad fija ($5F) salvo que la lista incluya comandos INTENSITY propios.

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

### Built-ins & Vectorlist Reference (Evolving)

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
 vectrex.draw_line(x0,y0,x1,y1,intensity) : single segment using BIOS Draw_Line_d (delta 8‑bit)
 draw_polygon forms (compilation macro, argumentos constantes):
     Form A: DRAW_POLYGON(N, x0,y0, ..., xN-1,yN-1) usa intensidad $5F
     Form B: DRAW_POLYGON(N, INTENS, x0,y0, ..., xN-1,yN-1)
     Implementación optimizada: un solo Reset0Ref + Intensity al inicio, Moveto_d al primer vértice y luego N líneas (cierre automático). Menos flicker.
     Futuro: versión runtime con vértices dinámicos.
     draw_circle(xc,yc,diam[,intensity]) : macro constante que genera un 16-gon aproximando el círculo (formas A/B como polygon; B añade intensidad). Un solo Reset0Ref + intensidad.
     draw_circle_seg(nseg, xc,yc,diam[,intensity]) : variante con número de segmentos (3..64)
     draw_arc(nseg, xc,yc,radius,start_deg,sweep_deg[,intensity]) : arco abierto subdividido (1..128 segmentos)
     draw_spiral(nseg, xc,yc,r_start,r_end,turns[,intensity]) : espiral abierta interpolando radio y ángulo (1..160 segmentos)
 vectrex.draw_vl(ptr,intensity) : call BIOS Draw_VL with user vector list (y x y x ...; end flagged by bit7 in Y)
 vectrex.draw_to(x,y) : placeholder (updates current position only)

Vectorlist embedded DSL (simple, orden agnóstico entre comandos de forma):
```
vectorlist shapes:
    ORIGIN              # Reset0Ref (CMD_ZERO)
    INTENSITY 0x5F      # Inserta CMD_INT (traduce 0..7 a presets, o valor directo)
    MOVE -16 -16        # Inicio rectángulo (emite CMD_START absoluto)
    RECT -16 -16 16 16  # Cuadrado -> 4 segmentos (CMD_LINE)
    POLYGON 4 0 -16 16 0 0 16 -16 0  # Diamante cerrado
    CIRCLE 0 0 12 24    # Centro (cx,cy) radio=12, 24 segmentos
    ARC 0 -16 16 0 180 24   # Arco desde 0° a 180°
    SPIRAL 0 0 10 40 2 64   # r_start, r_end, turns, segs
```
Reglas:
- MOVE genera un START absoluto; RECT genera START + 4 líneas; POLYGON N genera START + N líneas cerrando; CIRCLE/ARC/SPIRAL generan aproximaciones poligonales.
- ORIGIN -> CMD_ZERO (Reset0Ref) que recentra el haz (se colapsan duplicados y se elimina un ZERO inicial redundante si tras él viene un START).
- El backend reordena para asegurar un START (0,0) inicial y mueve la primera INTENSITY justo después.
- Comentarios automáticos en el `.asm` indican coordenadas absolutas y deltas para depurar.

Ejemplo Pac-Man mini (fragmento):
```
vectorlist maze:
    INTENSITY 0x7F
    ORIGIN
    MOVE -68 -68
    RECT -68 -68 68 -67   # borde superior
    ...
```
Luego en `main()`:
```
def main():
    vectrex_draw_vectorlist("maze")
    vectrex_draw_vectorlist("pellets")
    vectrex_draw_vectorlist("actors")
```

 Runtime helpers actuales en modo minimal se reducen: bucle de frame + Wait_Recal + Reset0Ref + Intensity_5F (salvo override vía INTENSITY dentro de listas). Antiguas opciones (blink, bank-size, debug draw) han sido retiradas o aparcadas.

Example drawing demo: `examples/vectrex_draw_demo.vpy`
Polygon macro demo: `examples/triangle_text.vpy` (triángulo, cuadrado, hexágono con DRAW_POLYGON)

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

---

## IDE & Tooling (Desktop Prototype)

Además del compilador CLI y la extensión de VS Code, el repo incluye un prototipo de IDE de escritorio (Electron + React + Monaco + LSP Rust) ubicado en `ide/`:

### Estructura
    (El shell Tauri anterior ha sido retirado.)
- `ide/frontend`: UI React (Monaco Editor, layout docking, cliente LSP).
- `core/src/lsp.rs`: Servidor LSP (tower-lsp) compartiendo lexer/parser con el compilador.

### Script de desarrollo
`./run-ide.ps1` (PowerShell 5.1+) admite flags principales (ver script para opciones de CSP/DevTools). Ya no existen modos Tauri.

### Capacidades LSP actuales
- Initialize tolerante a respuestas espurias (-32600 inicial en algunas ejecuciones).
- Diagnostics (errores de parseo y heurísticas como POLYGON 2 -> warning).
- Completion: keywords, macros DRAW_*, comandos vectoriales, constantes básicas.
- Hover: documentación localizada (en/es) de comandos built-in y ubicación de funciones definidas por el usuario.
- Go to Definition: salto a definición de funciones creadas por el usuario.
- Semantic Tokens (full): keywords, funciones (usuario y built-in diferenciadas por modificador), variables, parámetros (reservado), números, strings, operadores, constantes I_*.

### Resaltado de sintaxis / tema
- Monaco Monarch para tokens léxicos básicos (keywords, macros, constantes).
- Semantic highlighting activado para refinar (enumMember para I_*). Tema custom `vpy-dark` definido en cada montaje para permitir hot reload.

### Docking / Layout
Se usa `flexlayout-react` para un workspace con pestañas: Files | Editor | Emulator | Debug | Errors.

En Web: drag & drop nativo HTML5 permite reordenar/redistribuir pestañas.

Layout con `flexlayout-react` gestionado por drag & drop nativo (no se requiere workaround específico de WebView2).

### Menú (nuevo)
Se reemplazó la fila de botones por una barra de menú minimal:

- Menú File: Reset Layout, (placeholders para New/Open), Exit.
- Menú View: toggles para mostrar/ocultar Files, Emulator, Debug, Errors. (Editor es fijo y no puede ocultarse / cerrarse).
- Cada ítem de View muestra un check si la pestaña está presente; Errors añade badge con `nE` o `nW` (errores o warnings) si existen diagnostics.

Las pestañas (excepto Editor) ahora se pueden cerrar con la X de la propia tab; se restauran desde View > (Nombre).

Desde la versión reciente el Editor también puede cerrarse; si hay documentos con cambios sin guardar se mostrará un prompt de confirmación antes de cerrar. (Guardado real/flujo de persistencia de archivos pendiente de implementación: el prompt actualmente sólo avisa/cancela.)

Persistencia: el layout sigue almacenándose en `localStorage` (`vpy_dock_model_v2`). Cerrar una pestaña y reiniciar respeta el estado; Reset Layout restaura el layout por defecto e inserta de nuevo la pestaña Errors si falta.

LIMITACIONES (prototipo):
- El fallback no genera todavía un "ghost" visual ni crea nuevos tabsets dinámicamente arrastrando al borde.
- La heurística de identificación de tabset se basa en hash de los títulos visibles (suficiente mientras los nombres difieran).

### Próximas mejoras posibles (IDE)
- Crear tabsets nuevos al soltar en bordes (split vertical/horizontal).
- Persistencia inmediata tras cada move (forzar `onModelChange`).
- Mejora de accesibilidad del drag (teclado / alta precisión).
- Formateador y rename symbol en LSP.

### Problemas conocidos
| Área | Descripción | Estado |
|------|-------------|--------|
| WebView2 DnD | Cursor prohibido con HTML5 drag nativo | Mitigado con fallback custom |
| Semantic tokens | Requiere sincronizar legend y tema para nuevos tipos | Documentado |
| Reordenar cross-tabset (estético) | Falta ghost y drop zones más ricas | Pendiente |
| Multi-error parse | Actualmente sólo primer error: parser hace bail temprano | En diseño |
| Live i18n en tabs | Cambiar idioma no refresca títulos existentes | Pendiente |

### Cómo arrancar el IDE rápidamente
1. `cargo build --bin vpy_lsp` (opcional; el script lo hará si no existe).
2. `pwsh ./run-ide.ps1`
3. Esperar a que Vite sirva `http://localhost:5173` y Electron abra ventana.

Si sólo deseas la experiencia web (sin wrapper desktop), entra a `ide/frontend` y ejecuta `npm run dev`.

---

## Changelog (Extracto Reciente)
- Panel "Errors" agregado: listado global de diagnostics (error/warning/info) con doble click de navegación.
- Añadido servidor LSP con semantic tokens y hover localizado.
- Integrado Monaco Editor con completado, hover, definición y resaltado semántico.
- Tema oscuro `vpy-dark` + reglas para enumMember (constantes I_*).
- Script PowerShell robusto para lanzar IDE (`run-ide.ps1`).
 (El fallback de drag específico de Tauri fue eliminado al migrar definitivamente a Electron.)
 - Fix: hover -32601 (se reemplazó binario minimal por wrapper del servidor completo).
 - Fix: duplicación de tooltips hover (gestión de disposable al registrar proveedor).
 - Fix: normalización URIs Windows (file:///C:/...) para alinear markers y panel de errores.
 - Mejora: parser error line/col parsing robusto ante rutas con colon (Windows drive); evita inversión de coordenadas.
 - Tests: añadidos `diagnostics_positions` (warning línea 20, error línea 30) y `diagnostics_windows_path` (línea 187) para blindar mapping.
 - Hover docs: añadidas entradas para `DRAW_VECTORLIST` y alias `VECTREX_DRAW_VECTORLIST`.
 - Signature Help: parámetros activos por conteo de comas.
 - Logging: instrumentación selectiva `[vpy_lsp][hover]` para depurar ubicaciones en hovers.

## Diagnostics & Line/Col Mapping
El servidor produce:
1. Errores de parseo (ERROR) con rango mínimo (start..start+1) usando line/column 0-based.
2. Warning heurístico cuando se detecta `POLYGON 2` (lista degenerada); se marca al inicio de la línea.

Pipeline:
Parser -> Mensaje `filename:line:col: error: detalle` -> extractor robusto (retrocede desde `: error:`) -> LSP `publishDiagnostics` -> Store (mantiene 0-based) -> Panel (`line+1:col+1`).

Tests automatizados verifican posiciones (incluyendo rutas Windows con `C:`) para evitar regresiones.

