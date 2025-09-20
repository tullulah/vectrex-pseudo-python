# Vectrex Pseudo-Python Project – Super Summary (Context Recovery Document)

> Purpose: Single authoritative, regularly updated high‑signal document to restore full project context after a lost session. Includes architecture, design decisions, build/runtime flows, IPC, emulator core specifics, frontend panels, conventions, troubleshooting, and short/medium backlog.
>
> Keep this file updated when making structural changes. Prefer additive updates with dated CHANGE NOTES at bottom.

---
## 1. High-Level Goal
Provide a modern toolchain + IDE for authoring Vectrex programs in a higher-level pseudo-Python / DSL that compiles to 6809 assembly/binaries, with a Rust/WASM 6809 + VIA + integrator emulator embedded in an Electron/React IDE. Real-time visualization of vector beam output plus introspection panels.

---
## 2. Repository Layout (Key Directories)
- `core/` – (Legacy / source generation pipeline pieces) Contains backend compile pipeline logic for pseudo language → 6809 assembly.
- `emulator/` – Rust crate implementing unified Motorola 6809 CPU, VIA timing, integrator (vector segment generator), wasm-bindgen exports.
- `ide/` – Electron + React frontend.
  - `electron/` – Main/Preload processes, IPC handlers (compile, assemble, source enumeration, file dialogs, etc.).
  - `frontend/` – React app (panels, Zustand stores, wasm glue, UI state, canvas rendering).
- `examples/` – Example `.vpy` (pseudo-python) and `.asm` sources with generated `.bin` outputs.
- `build/` – Some assembler output / scratch area (older flow).
- `include/` – Shared include assembly headers (if any) for lwasm stage.
- `runtime/` – BIOS or runtime assets (if present/mirrored – BIOS ultimately loaded from known search paths).
- `vscode-extension/` – (If present) Editor / language support (not the current primary UI path).

---
## 3. Emulator Core (Rust)
### 3.1 CPU 6809 Implementation
- Single authoritative CPU: `emulator/src/cpu6809.rs` (Legacy duplicates removed or deprecated – ensure no alternate CPU remains active).
- Tracks: registers (A,B,DP,X,Y,U,S,PC,CC flags), cycle counts, opcode execution counts, loop watch samples, BIOS frame counters, IRQ/FIRQ/NMI logic, WAI halt state.
- Metrics exported: total opcodes, unique unimplemented, hot spots (0x00 / 0xFF), VIA registers snapshots, cycles/frame, last intensity, vector counts.
- Instruction coverage: Large direct match over opcodes; missing / undefined opcodes mapped to lightweight NOP or recorded via `UNIMPL OP` branch.
- Recently added: Indexed RMW cluster (0x60–0x6F) fully implemented; placeholder NOP handlers for 0x7B & 0x8F to suppress noise in unimplemented list.
- Unimplemented tracking: `opcode_unimpl_bitmap[256]` + list aggregated into `unique_unimplemented` in metrics. Any opcode falling through final `op_unhandled` arm increments counter.
 - RAM Execution Detector (nuevo 2025-09-19, ampliado 2025-09-19b/c): Instrumentación ligera que observa ejecuciones prolongadas dentro de ventana RAM 0xC800–0xCFFF. Tras 512 iteraciones captura snapshot (regs, stack bytes, ventana ±24 bytes alrededor de PC, recent PCs, call_stack) sin abortar ejecución. Ampliación b: hook temprano en RTS/RTI que si el retorno cae directamente dentro de 0xC800–0xCFFF captura snapshot inmediato (`[RAM-EXEC EARLY][RTS-invalid-return]`). Ampliación c: cada snapshot incluye campo `reason` = `threshold` o etiqueta del disparo temprano (ej: `RTS-invalid-return`, `RTI-invalid-return`). No sobreescribe snapshot si ya estaba disparado. Volcado sólo si `trace` activo. Campo: `cpu.ram_exec.snapshot`.
 - Pseudo BIOS initial call: durante `reset()` si BIOS presente y `bios_calls` vacío, se registra una entrada inicial con la dirección de arranque (sin fabricar JSR) para trazas de secuencia inicial (facilita correlación con tests que esperan primera etiqueta BIOS).
#### 3.1.1 Execution Model (Simplified)
Inputs per instruction: PC, registers, flat memory + Bus (≥0xD000), pending interrupts. Output: mutated state, cycles, integrator side-effects, metrics.
Cycles: Approximate per opcode group; refined for taken branches. `advance_cycles()` updates VIA timers, integrator, frame counters.
Interrupt priority: NMI > FIRQ (if F clear) > IRQ (if I clear). WAI pre-pushes full frame once; first interrupt resumes execution.
WAIT_RECAL: BIOS call 0xF192 marks potential frame boundary; actual frame credited on corresponding RTS/RTI at same call depth. Cycle-based `cycle_frame` is authoritative; BIOS frame observational.
Coverage: `recompute_opcode_coverage()` clones CPU & executes each opcode (and valid extended sub-opcodes) to classify implementation; results persisted in `opcode_unimpl_bitmap` & `last_extended_unimplemented`.
Hotspot: Limited LFU sampling for opcodes 0x00 & 0xFF (potential tight loops).

#### 3.1.2 Simplifications / Divergences
- Cycle timings are grouped estimates (not cycle‑accurate per addressing mode yet).
- Some infrequent / BCD related instructions (e.g. DAA) not implemented.
- Undefined opcodes coerced to NOP for forward progress.
- FIRQ path present; VIA currently only asserts standard IRQ line.

##### 3.1.2.1 Policy Update (No New Heuristics / Stubs)
As of the Timer1 IRQ frame refactor (date: 2025-09-17) se adopta una política estricta:
- No se introducirán nuevas simulaciones heurísticas de frames, temporización o hardware.
- Cualquier subsistema ausente se reflejará explícitamente en `SIMULATION_LIMITATIONS.md`.
- Si el hardware (p.ej. Timer1) no está configurado por BIOS / programa, los contadores asociados permanecerán en 0 en lugar de activarse por una suposición.
- Eliminada la heurística `cycles_per_frame` (frame boundary ahora = IRQ Timer1 real, IFR6).
Objetivo: evitar diagnósticos confusos causados por "avance artificial" que enmascare fallos reales.

#### 3.1.3 Planned Enhancements
- Data-driven opcode table (mnemonic, cycles, flags) to shrink match arm duplication.
- Selective trace filters (PC allowlist) for low-noise debugging.
- Golden trace comparison harness.
 - Extender detector RAM a: (a) conteo por ventanas múltiples (b) heurística de detección de patrón cíclico usando período mínimo en ring PCs.
- Export WASM de snapshot (`bios_calls_json` paralelo) para UI (TODO 13 completado 2025-09-19).
  - Nota: Entrada inicial Init_OS (0xF18B) ahora etiquetada explícitamente; parte del TODO 11 (mapeo direcciones BIOS) actualizada.

### 3.2 VIA & Timing
- VIA memory-mapped region at 0xD000 range (simplified mapping). Timers tick via centralized `advance_cycles()`.
- Interrupt servicing order: NMI > FIRQ (if F flag clear) > IRQ (if I flag clear). WAI halts until serviced.

### 3.3 Integrator / Vector Generation

### 3.4 Shadow Call Stack & RAM Execution Diagnostics (2025-09-19)
Para diagnosticar el bucle espurio observado en RAM (0xC800–0xCFFF) se añadió una pila sombra (shadow stack) que rastrea entradas/salidas de frames de control y valida retornos.

ShadowKind (tipos de frame registrados):
- JSR, BSR, LBSR: retorno = PC tras la instrucción; push16 real ya efectuado (BSR/LBSR) o se añade para JSR.
- PshsPc / PshuPc: PSHS/PSHU con bit PC (0x80) empujan PC y se registra frame para validar PULS/PULU posteriores.
- IRQ, FIRQ, NMI, SWI, SWI2, SWI3: tras apilar el frame hardware de interrupción se añade frame sombra con el PC previo (ret) y SP resultante.

Validación:
- RTS: compara PC destino con frame (espera JSR/BSR/LBSR).
- PULS/PULU (bit PC): compara con frame PshsPc/PshuPc.
- RTI: compara con frame de tipo de interrupción correspondiente.

Snapshot Razones (cuando PC cae en 0xC800–0xCFFF o mismatch):
- `RTS-invalid-return`, `RTI-invalid-return`, `PULS-invalid-return`, `PULU-invalid-return` (ret directo a RAM sin mismatch shadow necesario).
- `shadow-mismatch-rts|puls|pulu|rti` cuando frame difiere.
- `shadow-underflow-*` si se intentó retornar sin frame.

Snapshot incluye: regs, ring de últimos PCs, bytes de stack (ventana), bytes alrededor de PC (±24), call_stack lógico, shadow_stack restante y reason.

Notas de diseño:
- Cero efectos secundarios: la pila sombra no altera estado 6809.
- Los frames de interrupción se registran post-push para correlacionar SP exacto y detectar corrupción.
- Política “no heurísticas”: sólo se registra, nunca se fabrica estado (DP, intensidades, etc.).

Pendiente / Futuro:
- Exportar snapshot + shadow stack vía WASM (TODO 13: `bios_calls_json` paralelo o nuevo `bios_calls_and_shadow_json`).
- Tests para RTI con anidamiento (IRQ dentro de NMI, etc.).
- Detección de períodos repetitivos en ring PCs para clasificar tipo de bucle RAM.


---
## 4. WASM API
File: `emulator/src/wasm_api.rs`
Exports (selection):
- Initialization: `init()`, `load_bios(bytes)`, `load_program(bytes, base)`.
- Execution: `runFrame()`, (per-frame run advancing until frame boundary heuristics), `reset()`.
- Introspection: `registers()`, `metrics()`, `loopWatch()`.
- Vector data: `getSegmentsShared()` (preferred zero-copy) and `drainSegmentsJson()` (JSON fallback).
- Demo: `demoTriangle()` returns canned triangle segments (used for visual sanity test / demo mode).
- BIOS Call Stack: `bios_calls_json()` (añadido 2025-09-19, completa TODO 13) devuelve últimas ≤256 entradas "FFFF:LABEL"; `clear_bios_calls()` las borra.

---
## 5. Build / Compilation Pipeline
### 5.1 Source Types
- `.vpy` pseudo-python high level source → compiled into assembly/binary by backend compile path invoked through Electron IPC `runCompile`.
- `.asm` raw 6809 assembly assembled by `lwasm` (invoked through `emuAssemble` IPC) → direct binary bytes loaded.

### 5.1.1 Compilation Steps (VPY → BIN via WSL)
1. **Compile VPY to Assembly:**
  - The IDE or backend invokes the pseudo-python compiler, which parses the `.vpy` file and emits a `.asm` file (6809 assembly).
  - This step is handled by the backend compile pipeline, typically via Electron IPC (`runCompile`).
2. **Assemble with lwasm (via WSL):**
  - The generated `.asm` is assembled into a `.bin` using `lwasm` running inside WSL for compatibility.
  - The PowerShell script `tools/lwasm.ps1` is used to invoke lwasm, translating Windows paths to WSL `/mnt/...` form and ensuring the assembler runs in the project directory for correct relative includes.
  - Example usage:
    ```powershell
    ./tools/lwasm.ps1 --6809 --format=raw --output=build/game.bin build/game.asm
    ```
  - The script checks for lwasm at `/home/linuxbrew/.linuxbrew/bin/lwasm` in the Ubuntu WSL instance, and sets up the environment accordingly.
  - All arguments (including output and input paths) are normalized for WSL, and the command is executed via `wsl -d Ubuntu bash -lc ...`.
3. **Binary Output:**
  - The resulting `.bin` file is placed in the specified output location (e.g., `build/game.bin`).
  - This binary is then loaded into the emulator for execution and visualization.
4. **Direct Assembly Path:**
  - For raw `.asm` files, the same WSL-based lwasm flow is used, bypassing the VPY compiler step.
  - The IDE assembles and loads the binary as above.

### 5.2 Precedence for Building (Current Panel Logic)
1. Explicit Source dropdown selection.
2. Fallback to currently active editor document (if it has `.vpy` or `.asm`).
3. If neither available: build aborted with toast error.

### 5.3 Dirty Document Handling
- If active document matches selected path and is dirty, panel sends `saveIfDirty` payload (content + expected mtime) to `runCompile` so backend can save before compile (optimistic write model).

### 5.4 Output Placement
- Compiled `.vpy` assumed to produce sibling `.bin` (path substitution: change extension to `.bin`). Panel attempts to fetch that `.bin` and load into WASM core.
- Assembled `.asm` returns base64 bytes inline via IPC response; loaded immediately.

### 5.5 Base Load Address
Cartridge origin is now fixed at `0x0000`. The previous user-configurable base (historically default `0xC000`) caused binaries assembled for $0000 to execute incorrectly when mapped at $C000. The UI still shows a Base field for legacy flexibility, but if a Vectrex-style cartridge header pattern is detected at offset 0 and the entered base is not `0x0000`, it is auto-corrected (toast notifies user). Legacy persisted metadata with `base: 0xC000` is migrated to `0x0000`.

---
## 6. Electron IPC Surface
(Main file: `ide/electron/src/main.ts`)
- `runCompile({ path, saveIfDirty?, autoStart? })`: Runs high-level compile pipeline; may also auto-run under legacy path then we re-load for the WASM emulator.
- `emuAssemble({ asmPath })`: Runs lwasm, returns `{ ok, base64, size }` or error.
- `listSources({ limit })`: Enumerates `.vpy` and `.asm` across project & `examples/` (simple scan; currently non-recursive or shallow – extendable).
- Additional file dialogs (e.g., openBin) exposed via preload if integrated.

Preload (`ide/electron/src/preload.ts`) safely exposes the above to `window.electronAPI`.

---
## 7. Frontend Architecture
- Framework: React + Zustand stores.
- Panels Directory: `ide/frontend/src/components/panels/` (EmulatorPanel, OutputPanel, etc.).
- State Stores:
  - `emulatorStore`: status (`running|paused|stopped`), plus setters.
  - `editorStore`: tracks open documents `{ uri, diskPath, dirty, mtime, content? }` and `active` pointer.
- WASM Binding Wrapper: `emulatorWasm.ts` (globalEmu) used by panels for execution & drawing.

### 7.1 EmulatorPanel (Current Behavior)
- Controls: Build & Run, Load .bin (manual file), Play/Pause/Stop/Reset/Clear Stats, Reload last binary, Demo toggle.
- Source selection dropdown (populated from IPC), persists selection to `localStorage` key `emu_selected_source`.
- Removed features: Manual path text field and auto Detect button (now simplified).
- Canvas: 300×400 logical size, draws vector segments each frame (green glow lines, intensity mapped to alpha).
- Frame loop: requestAnimationFrame while status is `running` and not `demoMode`.
- Demo Mode: Use static `demoTriangle()` segments; suspends CPU frame loop.
- Persistence: Last binary metadata (path, base, size) preserved; quick reload button uses cached bytes. Base address is effectively fixed at `0x0000` for normal cartridges; mismatched entries are auto-normalized.
- Toast system: transient messages with auto-expire (4s).
- Loop Watch: Optional checkbox to display last captured loop samples from CPU (limited sampling every N iterations of certain BIOS loops).

### 7.2 OutputPanel
- Displays registers, cycle metrics, frames, number of unique unimplemented opcodes, vector list draw counts, top opcode histogram, and list of unimplemented opcodes.
- Refresh: manual or auto (1s interval).

---
## 8. Vector Rendering & Segment Flow
1. User builds program → binary loaded at base (default 0xC000) via `globalEmu.loadProgram(bytes, base)`.
2. `performFullReset()` resets CPU/integrator state.
3. `status` set to running; animation loop calls `runFrame()` repeatedly.
4. After each frame: fetch segments via `getSegmentsShared()` or fallback JSON drain.
5. Canvas rescales coordinates ([-1,1] normalized assumption) to centered viewport.
6. If no segments and BIOS not loaded → message prompts for BIOS.

Potential reasons for “no vectors”:
- Program stuck before WAIT_RECAL boundary (frameCount not incrementing meaningfully).
- BIOS missing (no vector driver routines invoked).
- Integrator not receiving DAC writes (program not poking VIA register addresses used by integrator model).
- Base load overlap / collision with BIOS area (incorrect base) causing corrupted flow.
- Unimplemented opcode early exit halting progression (check OutputPanel metrics).

Debug tips:
- Enable trace around suspected PC addresses (extend code to conditional trace block).
- Add temporary logging in integrator segment push path to confirm writes.
- Verify BIOS load (globalEmu.isBiosLoaded()).

---
## 9. BIOS Handling
- BIOS expected at one of: `bios.bin`, `/bios.bin`, `/core/src/bios/bios.bin` (fetched sequentially at init). Store in `ide/frontend/public/` for reliability.
- BIOS calls above `0xF000` logged for frame boundary detection.

---
## 10. Persistence & Local Storage Keys
- `emu_last_bin_meta` – JSON { path?, size?, base }.
- `emu_selected_source` – last chosen source path from dropdown.

---
## 11. Known Removed / Deprecated Elements
- Legacy EmulatorPanel duplicate (older location `ide/frontend/src/components/EmulatorPanel.tsx`) should be removed to avoid confusion (if still present, treat as dead code).
- `build.build` command: placeholder / logging only; panel bypasses it with direct IPC now.
- Manual path + Detect feature (removed to reduce state divergence risk).

---
## 12. Coding Conventions & Practices
- Rust: Prefer explicit match arms over massive helper decode tables; unhandled opcodes fall through to final arm logging once (bitmap prevents spam).
- JS/TS: State via Zustand; avoid global window mutation (removed `window.__vpyEditorStore`).
- UI: Minimal styling inline; monochrome vector aesthetic.
- Error surfacing: Use toast for user-noticeable build/load results; console for diagnostics.

---
## 13. Troubleshooting Cheat Sheet
| Symptom | Likely Cause | Quick Check | Fix |
| ------- | ------------ | ----------- | --- |
| Status stuck running, no frames | WAIT_RECAL never returns | `cycleFrame` or `frameCount` stagnant | Verify BIOS + code path; add trace; ensure base correct |
| No segments, BIOS frames increment | Program not writing vector list ops or integrator decode mismatch | OutputPanel `draw_vl` | Inspect binary generation, integrator logic |
| Unimplemented opcodes appear (e.g. 0x7B,0x8F) | Placeholders / undefined | OutputPanel list | Implement or keep as NOP placeholders |
| Build & Run does nothing | Missing selection & no active doc | Toast error | Choose source from dropdown |
| Wrong file compiled | Stale dropdown selection | Active doc vs dropdown mismatch visible | Switch dropdown or open desired file then re-select |
| Reload fails | No prior binary cached | `Reload` disabled | Build or load a binary first |
| BIOS missing warning | bios.bin not found in search paths | Canvas message | Place `bios.bin` in `ide/frontend/public/` |

---
## 14. Backlog / Next Improvements
Short Term:
- Remove legacy panel file to avoid confusion.
- Add frame-step (single `runFrame()` button) for debugging.
- Integrator debug overlay (draw points/axes, show segment count live).
- Persist trace flag / targeted PC breakpoints (e.g. watch PC list input).
- IPC: Add `rebuildAll` to batch compile examples.

Medium Term:
- Proper symbol table emission from compiler to map PCs to function names in UI.
- Cycle-accurate VIA timing verification suite (test harness comparing reference traces).
- Integrator refinement: emulate phosphor decay & intensity curve.
- Source watcher: Auto-refresh source list when files added/removed.
- Migrate placeholder NOP opcodes into real implementations with correctness tests.

Long Term:
- Headless CLI build mode (no Electron) producing distributable `.bin`.
- Live code patching (hot swap compiled functions without full reset).
- Formal test coverage report for opcode implementation (auto diff vs reference emulator).
- Enhanced pseudo-language features (structured loops, macros, inline assembly interop).

---
## 15. Testing Strategy (Current State / Recommendations)
- Rust: Unit tests in `emulator/tests/` (cover memory map, interrupt masking, read-modify-write opcodes, nested IRQ/FIRQ semantics).
- Add: Golden trace compare (record known BIOS bootstrap trace and diff cycles/opcodes).
- JS: Minimal; consider adding Jest tests for path resolution & persistence logic.

---
## 16. Performance Notes
- Frame loop currently runs unthrottled under rAF; consider adding a max frame time slice or instrumentation for overrun detection.
- Segments retrieval uses shared memory first (avoid JSON parse). Keep segment vector reused across frames to limit allocations.
- Potential optimization: Batch draw using single path + moveTo/lineTo pairs or OffscreenCanvas for future.

---
## 17. Security / Stability Considerations
- IPC only exposes vetted operations (compile / assemble / list sources). Avoid arbitrary shell execution.
- Unsaved dirty buffer compile uses optimistic mtime match to prevent stomping external changes.
- Placeholder opcodes treated as NOP minimize risk of accidental infinite unimplemented growth.

---
## 18. Decision Log (Recent Key Decisions)
- Unified CPU (removed second implementation) to prevent divergence. (2025-09)
- Dropped manual path + detect UI to reduce complexity; rely on explicit dropdown & active doc. (2025-09)
- Added placeholder handlers for 0x7B & 0x8F instead of logging unimplemented repeatedly. (2025-09)
- Source enumeration moved to dedicated IPC `listSources` instead of heuristic guessing. (2025-09)
- Build pipeline decoupled from unused `build.build` command; direct IPC invocation. (2025-09)
- Hardened opcode sweep detection (bitmap+cycle delta+ok) and reduced fetch logging noise (trace-gated). (2025-09-19)

---
## 19. How to Start Fresh After Cloning
1. Place `bios.bin` in `ide/frontend/public/`.
2. Start IDE script (PowerShell): `./run-ide.ps1 -DevTools`.
3. Open a `.vpy` or `.asm` in editor; select it in EmulatorPanel dropdown if needed.
4. Click Build & Run.
5. If no vectors: open OutputPanel, verify metrics, adjust base address if custom.

---
## 20. Glossary
- **WAIT_RECAL**: BIOS routine marking frame boundaries (used for frame counting heuristics).
- **Integrator**: Component converting DAC / VIA register activity into normalized line segments for display.
- **Loop Watch**: Lightweight sampler capturing register snapshots in tight BIOS loops.
- **Hot 0x00 / 0xFF PCs**: Tracking of program counters executing certain opcodes unusually often (heuristic performance/bug clues).

---
## CHANGE NOTES
- 2025-09-16: Initial creation of SUPER_SUMMARY.md with full architecture & decisions.
 - 2025-09-16: Added deep dive (Sections 21–27), compiler & language spec draft, opcode appendix, expanded change log.
- 2025-09-18: Stack/return diagnostic instrumentation begun (call events capture `ret_addr`); buffer size (32) identified as insufficient; C++ parity test added (pending build); classification of drift vs mismatch deferred.
- 2025-09-18: Added opcode 0x14 & 0x15 to illegal/NOP handler set (previous test failure on 0x14 unimplemented classification resolved); `opcode_validity::illegal_opcodes_are_1_cycle_and_not_unimpl` now passes. Introduced temporary null-engine build path for C++ `vectrexy` to bypass outdated ImGui/SDL dependencies; created standalone `bios_callstack` tool (C++) mirroring Rust call stack trace (supports JSR direct/extended/indexed & BSR) but effort paused per focus shift back to Rust compiler/emulator. Illegal opcode list in Section 24 implicitly updated—ensure next comprehensive edit merges both enumerations (Section 24 & code). Warning: `unmapped_read_fallback` now dead code (candidate for removal or doc reference if open-bus fallback policy reintroduced).
- 2025-09-18: Reintroducido registro de instrucción (trace) para WASM: nueva función interna `trace_maybe_record` empuja `TraceEntry` al inicio de `step()` cuando `trace_enabled` está activo (habilitado vía export wasm `enable_trace(en,limit)`). Panel Trace requiere pulsar "Capture Init" (no auto-on) para evitar sobrecoste por defecto. Límite configurable (`trace_limit`) protege memoria (cap lado UI a 200k). Documentado para evitar confusión futura sobre ausencia de entries si no se activa.
 - 2025-09-18: Ampliado `TraceEntry` con campo `call_depth` (profundidad de pila de llamadas BIOS/JSR en el momento del fetch) y exportado en `trace_log_json()` como `depth`. No rompe compatibilidad: consumidores previos que ignoran campos extra siguen funcionando. Próximo paso: usar `depth` en UI para plegar/expandir trazas por nivel.
 - 2025-09-18: Ciclos afinados para familia CMPX y JSR indexed: 0x8C (CMPX imm) = 5 ciclos, 0xAC (CMPX indexed) = 6, 0xBC (CMPX extended) = 7; añadido handler explícito para JSR indexed (0xAD = 7 ciclos) y CMPX indexed separando seeds. Nuevos tests `audit_cmpx_*` verifican 5/6/7 y `audit_jsr_extended_cycles` permanece verde. Prueba de enforcement `enforce_no_unimplemented_primary_opcodes` confirma 100% cobertura primaria válida. Lista de ilegales ampliada (incluye 0x41,0x42,0x4B,0x51,0x55,0x5B,0x5E,0x62,0x65,0x6B,0x71,0x72,0x75,0x87,0xC7,0xCD) tratadas como NOP de 1 ciclo sin contaminar métrica de unimplemented.
- 2025-09-19: Barrido 0x00–0xFF endurecido (ok + delta ciclos + bitmap) y fetch logging reducido (solo trace). Añadido helper público `opcode_marked_unimplemented`.
 - 2025-09-19: Añadidos barridos adicionales: (a) sweep extendido 0x10/0x11 validando que `extended_unimplemented_list()` está vacío y cada sub‑opcode válido avanza ciclos; (b) sweep básico VIA 0xD000–0xD00F verificando coherencia IFR bit7. Ref ref: tests `opcode_extended_and_via_sweeps.rs`.
 - 2025-09-19: Integrado mapeo exhaustivo de etiquetas BIOS en `bios_label_for()` y uso en `record_bios_call` (eliminando "BIOS_UNKNOWN" para rutinas estándar). Incluye Init_VIA, Warm_Start, Intensity_* variantes, Print_List*, Draw_VL, rotación (Rot_VL_*), sonido (Sound_Byte*, Clear_Sound, Do_Sound), contadores (Dec_*), y helpers de rotación/rise-run.
 - 2025-09-19: Ampliado mapeo BIOS: añadido scoreboard / score math (Strip_Zeros, Compare_Score, New_High_Score), colisiones (Obj_Will_Hit_u, Obj_Will_Hit, Obj_Hit), efectos (Explosion_Snd), más intensidad (Intensity_1F, Intensity_3F), variantes sonido (Sound_Bytes*, Do_Sound_x) para erradicar remanentes "BIOS_UNKNOWN".
 - 2025-09-19: Limpieza mapeo BIOS: eliminado duplicado `Moveto_d` (0xF312) que causaba warning de pattern inalcanzable y añadido test de regresión `bios_label_coverage` (archivo `emulator/tests/bios_labels.rs`) que valida presencia de etiquetas para todas las direcciones conocidas.
 - 2025-09-19: Ampliación final mapeo BIOS (fase Option A completa): añadidas rutinas restantes: Reset0Ref_D0, Check0Ref, Reset_Pen, Reset0Int, familia Print_* (Str_hwyx, Str_yx, Str_d, List_hw, List, List_chk, Ships_x, Ships, Str genérico), variantes combinadas Mov_Draw_VL*, variantes de dibujo Draw_VL* (c, b, cs, ab, a, principal, line), patrones Draw_Pat_VL*, modos Draw_VL_mode, variantes pre-move Draw_VLp*, random (Random_3, Random), inicialización música (Init_Music_Buf, Init_Music_chk, Init_Music, Init_Music_dft), clears de memoria (Clear_x_b, Clear_C8_RAM, Clear_x_256, Clear_x_d, Clear_x_b_80, Clear_x_b_a), contadores (Dec_6_Counters), suite de delays (Delay_3/2/1/0/b/RTS), utilidades Bitmask_a, Abs_a_b, Abs_b, Rise_Run_Angle, transformaciones Xform_* y Move_Mem_a*/_1. Test actualizado para cubrir todas. Objetivo: cero "BIOS_UNKNOWN" para llamadas legítimas.
 - 2025-09-19: Corrección frame IRQ y validación RTI: ajustado orden de push del frame de IRQ a la secuencia inversa de la restauración hardware (push: PC,U,Y,X,DP,B,A,CC) eliminando inversión de endian que causaba retorno 0x0001 en test. Añadido pop/validación de frame sombra en RTI (antes faltaba, produciendo fuga de shadow frames). Test `irq_rti_shadow_frame` ahora pasa con retorno exacto y pila sombra vaciada.

 - 2025-09-19: Intercept temprano Draw_VL / Draw_VLc (fase transitoria antes de emulación analógica completa VIA/DAC). Implementado en `record_bios_call` detectando direcciones $F3DD (Draw_VL) y $F3CE (Draw_VLc). Características:
   * Usa `X` como puntero de lista (corrección respecto al uso previo incorrecto de `U`).
   * Draw_VLc: primer byte = cuenta N; se leen N pares (dy,dx). Draw_VL: cuenta leída desde RAM $C823; se consumen ese número de pares (dy,dx) sin bit sentinela.
   * Escala aproximada aplicada por factor `scale = (VIA_T1_low / 255.0)` leyendo 0xD004; si 0 => 1.0.
   * Primer par tratado como movimiento (reposición del haz) sin emitir segmento (`Integrator::move_rel`), alineado con semántica de rutinas BIOS que posicionan antes de dibujar.
   * Segmentos posteriores emitidos con `line_to_rel` (sin integración temporal, un segmento por par) para acelerar representación.
   * Intensidad: se utiliza la intensidad vigente (`last_intensity`) ya gestionada por la CPU al cambiar registros BIOS; no se simula decaimiento ni ramp up.
   * Limitaciones actuales: (a) no se soportan variantes de patrón (Draw_Pat_VL*), (b) no se procesan modos/rotaciones fuera de que la BIOS ya haya transformado coordenadas en la RAM, (c) no modela timing real (no jitter, no distorsión por integrador analógico), (d) no respeta latencias de DAC ni blanking hardware entre movimiento y primer trazo.
   * Política de “No Sintético”: se evita introducir heurísticas como intensidades derivadas o escalas inventadas; la única aproximación temporal aceptada es el factor lineal de T1 low (valor ya configurado por BIOS). Cuando se implemente VIA/DAC real este intercept será eliminado y reemplazado por flujo de escritura de registros + integración por ciclos.
   * Impacto en métricas: conteo de segmentos por frame disminuye (primer par ya no genera segmento), mejorando concordancia esperada con arte de arranque original.
   * Próximo paso: remover intercept tras introducir pipeline real (writes a DAC X/Y, latch escala, blanking) y añadir trazado de patrones basándose en rutinas BIOS `Draw_Pat_VL*` sin stubs.


### 24.7 Actualización Ciclos CMPX / JSR (2025-09-18)
Resumen de ajuste puntual de temporización para mejorar fidelidad respecto a la tabla nominal MC6809:

| Opcode | Modo       | Ciclos Emu (antes) | Ciclos Emu (ahora) | Nominal | Nota |
|--------|------------|--------------------|--------------------|---------|------|
| 0x8C   | CMPX imm   | 6 (grupo genérico) | 5                  | 5       | Seed individual añadido |
| 0xAC   | CMPX indexed| 7 (grupo idx)     | 6                  | 6       | Handler + seed específica |
| 0xBC   | CMPX ext   | 5 (erróneo)        | 7                  | 7       | Corregido; se retiró de grupo de 5 |
| 0xAD   | JSR indexed| 5 (inexacto)       | 7                  | 7       | Handler dedicado (push + PC) |

Tests añadidos (módulo unified_audit en `opcodes_all.rs`):
- `audit_cmpx_immediate_cycles`
- `audit_cmpx_indexed_cycles`
- `audit_cmpx_extended_cycles`

Estos aseguran valores 5/6/7 correctos y actuarán como regresión si se altera la semántica. Se mantiene la política: no introducir heurísticas de efectos secundarios; sólo medir efectos reales de instrucciones. Próximo paso sugerido: extender auditoría de ciclos a conjunto completo de comparaciones y saltos para converger a exactitud total.

---
## 21. CPU / VIA / Integrator Deep Dive
### 21.1 CPU Flags & Registers
A,B (8-bit) forming D, X,Y,U,S (16-bit), DP (high byte for direct), CC bits EFHINZVC. E marks full frame pushed; F masks FIRQ; H reserved (half-carry pending proper BCD support); I masks IRQ.
### 21.2 Interrupt Entry Summary (POST-MIGRATION STANDARD MAP)
| Src | Frame | Sets E | Sets F | Sets I | Vector | Return |
|-----|-------|--------|--------|--------|--------|--------|
| FIRQ| Partial (CC+PC) | N | Y | Y | 0xFFF6 | RTI |
| IRQ | Full  | Y | N | Y | 0xFFF8 | RTI |
| SWI | Full  | Y | Y | Y | 0xFFFA | RTI |
| NMI | Full  | Y | N | Y | 0xFFFC | RTI |
| RESET | Full (hardware) | Y | N | Y | 0xFFFE | (fetch) |
| SWI2| Full  | Y | Y | Y | 0xFFF2 | RTI |
| SWI3| Full  | Y | Y | Y | 0xFFF4 | RTI |
| WAI | Pre (once) | Y | – | – | (next int) | RTI |

Nota: El orden de la tabla se ha ajustado para reflejar el mapa estándar ascendente de vectores (SWI2→RESET) y destacar la corrección aplicada el 2025-09-19 (ver subsección 21.2.1).

#### 21.2.1 Interrupt Vector Layout Migration (2025-09-19)
Histórico: Antes de esta fecha el emulador utilizaba un layout divergente heredado donde:
```
FIRQ = 0xFFF4 (bytes low,high invertidos en lectura)
IRQ  = 0xFFF6
SWI  = 0xFFF8
NMI  = 0xFFFA
RESET= 0xFFFC
```
Esto implicaba:
- FIRQ vector desplazado 2 bytes abajo respecto al estándar 6809.
- Lectura especial (endian invertido) para FIRQ.
- Potencial confusión al comparar trazas con otros emuladores (jsvecx / vectrexy) y documentación oficial.

Migración aplicada:
```
SWI2 = 0xFFF2
SWI3 = 0xFFF4
FIRQ = 0xFFF6
IRQ  = 0xFFF8
SWI  = 0xFFFA (alias SWI1)
NMI  = 0xFFFC
RESET= 0xFFFE
```
Cambios técnicos:
- Introducido helper `read_vector(base)` con lectura big-endian uniforme (hi=mem[base], lo=mem[base+1]).
- Eliminada ruta especial de FIRQ que invertía bytes.
- Actualizados tests (`irq_rti_shadow_test`, `firq_single_return_test`, `nested_irq_firq_test`) para escribir vectores en las nuevas direcciones.
- Añadido push de shadow frame consistente tras cada servicio (IRQ/FIRQ) con `sp_at_push` correcto.

Resultados:
- Alineación con mapas estándar → simplifica correlación con desensamblados BIOS y otras implementaciones.
- Elimina fuente de divergencias en trazas y necesidad de comentarios aclaratorios en tests.
- Todos los tests existentes pasan tras actualización (suite completa verde al momento de la migración).

Riesgos mitigados:
- Posibles futuros bugs de salto a handler incorrecto por error de offset desaparecen al consolidar la convención.
- Evita confusión en documentación (tabla 21.2 ahora refleja layout estándar aceptado).

Acciones futuras (opcionales):
- Añadir test de invariantes que verifique en arranque que cada vector apunta dentro de rango válido (ej. BIOS presente o cart) y no a página 0x0000 accidental si la BIOS aún no fue cargada.
- Exportar por WASM (TODO ID 13) la pila de llamadas BIOS incluyendo identificación de interrupción y vector usado.
### 21.3 VIA 6522 Map (0xD000)
| Ofs | Reg | Notes |
|-----|-----|-------|
|00|ORB|Experimental horizontal velocity|
|01|ORA|Experimental vertical velocity|
|04|T1C-L|Read clears IFR6|
|05|T1C-H|No clear (counter high)|
|08|T2C-L|Read clears IFR5 (updated semantics)|
|09|T2C-H|No clear|
|0A|SR|Intensity latch (experimental) + shift mode|
|0B|ACR|Timer modes + PB7 toggle|
|0C|PCR|Control lines (pass-through)|
|0D|IFR|Bit7 master pending synthesized|
|0E|IER|Bit7 set/clear semantics|
Timers: T1 supports free-run (ACR bit6) with PB7 toggle (bit7). T2 one-shot. IRQ line when (IFR&IER&0x7F)!=0.
### 21.4 Integrator Algorithm
Integrates position: x += vx*cycles, y += vy*cycles; clamps to [-512,512]. If beam_on && intensity>0 emits segment (splitting > max_seg_len, merging collinear). Optional blank slews & intensity decay hooks.
### 21.5 Memory Map
| Range | Purpose |
|-------|---------|
|0000-BFFF|Cartridge/program|
|C000-CFFF|User RAM / default load base|
|D000-D00F|VIA|
|E000-EFFF|BIOS (8K) optional|
|F000-FFFF|BIOS / vectors|

---
## 22. Compiler Pipeline
Lex→Parse→Semantic (const fold, type inference)→Lowering→Optimization (dead label, constant propagation, macro expansion)→Emit ASM→Assemble `.bin`.
Artifacts: `<src>.asm`, `<src>.bin` (future: `.sym`, `.lst`). Error model to standardize file:line:col (TODO).
### 22.1 Language Features (from history)
- let declarations; arithmetic & bitwise ops.
- switch/case lowering.
- String literals with escapes.
- Builtins (vectrex.*) mapping to BIOS or runtime macros.
- Macros: DRAW_POLYGON / DRAW_CIRCLE / ARC / SPIRAL.
### 22.2 Planned
- Loops, inline asm, explicit type annotations, lints.
### 22.3 Parameters / Flags
| Flag | Source | Effect |
|------|--------|--------|
|VPY_CPU_FREQ|Env|Adjust cycles_per_frame|
|VPY_NO_MERGE=1|Env|Disable integrator line merge|
|TRACE_FRAME|Env (native)|Verbose frame logs|
|TRACE_FRAME_FORCE|Env (native)|Force frame if stuck|
|emitSymbols (future)|CLI|Generate .sym|
|optimizeMacros (future)|CLI|Segment dedupe & merges|

---
## 23. Pseudo-Python Mini-Spec
Identifiers `[A-Za-z_][A-Za-z0-9_]*`; ints (dec/hex). Strings with escapes. Expressions: unary > * / > + - > bitwise > comparisons > assign. Statements: let, switch/case, macro calls, builtin calls. Macros expand before lowering. Future: loops, inline asm.

---
## 24. Opcode Appendix (Do Not Remove)
Legend: [I]=Implemented, [NOP]=Illegal/Undefined but intentionally treated as NOP (counted implemented for coverage), [P]=Placeholder kept as NOP awaiting spec confirmation. Extended valid sub‑opcodes enumerated in `VALID_PREFIX10/11` in `cpu6809.rs` — all currently implemented.

Summary (UPDATED 2025-09-18 – incluye fix añadiendo handlers NOP explícitos para 0x14/0x15): 100% de los opcodes válidos implementados.
Summary (REVALIDATED 2025-09-19 – barrido endurecido + bitmap): Cobertura base permanece 100%; cualquier regresión fallará inmediatamente en `opcode_base_full_sweep_unimplemented`.

Illegal base (MC6809 no definidos) ahora centralizados en la constante `ILLEGAL_BASE_OPCODES` (archivo `cpu6809.rs`). Tests llaman a `is_illegal_base_opcode()` para evitar divergencia. Lista actual:
```
01 02 05 14 15 38 45 4E 52 61 7B 8F CF
41 42 4B 51 55 5B 5E 62 65 6B 71 72 75 87 C7 CD
```
Todos tratados como NOP mínimos (1 ciclo) con bandera de "illegal" (no cuentan como unimplemented). Prefijos 0x10/0x11: todos los sub‑opcodes válidos implementados; cualquier hueco fuera de `VALID_PREFIX10/11` clasificado como ilegal y no afecta cobertura.

RMW Direct: 00,03,04,06,07,08,09,0C,0D,0E,0F [I]
RMW Indexed: 60,63,64,66,67,68,69,6A,6C,6D,6E,6F [I]
RMW Extended: 70,73,74,76,77,78,79,7A,7C,7D,7E,7F [I]
Control / Interrupt: SWI(3F), SWI2(10 3F), SWI3(11 3F), WAI(3E), CWAI(3C), SYNC(13), MUL(3D), DAA(19) todos [I].
Placeholders (NOP): 7B,8F [P]
Branches short 20–2F [I]; long branches prefix 0x10 (21–2F) [I].
Extended groups: CMPD, CMPY, LDY, STY, LDS, STS, SWI2 (0x10) y CMPU, CMPS, SWI3 (0x11) completos.

Cycle Snapshot: ver `docs/6809_opcodes.md` sección "Tabla de Ciclos Emulados" y archivo generado `cycles.csv` (bin `gen_cycles`). Los prefijos 0x10/0x11 se registran 0 ciclos adicionales (coste absorbido en sub‑opcode) — pendiente refinar si se busca exactitud reloj.

Cycle Delta Audit: bin `gen_cycles_compare` + `docs/6809_cycles_nominal.json` produce `cycles_compare.csv` con columnas emu/nom/delta; sección 5.1 del doc de opcodes resume los desvíos prioritarios (JMP, SEX, WAI/CWAI, SYNC, BRN). Ajustar `cyc` en `step()` según roadmap.

Coverage Tool: `recompute_opcode_coverage()` mantiene `opcode_unimpl_bitmap` (vacío tras snapshot) y `last_extended_unimplemented` (lista vacía cuando completo).

### 24.1 Open Bus & Lecturas Fuera de Rango
- Ahora las lecturas en regiones no mapeadas / gaps (cart out-of-bounds, C000 gap, región ilegal) devuelven el último byte físicamente colocado en el bus (`last_bus_value`).
- El Bus captura cada write y cada read válido para actualizar `last_bus_value`.
- Eliminado retorno sintético 0x01 (cart OOB) y 0xFF (illegal). Esto alinea el comportamiento con hardware real y evita ocultar dependencias accidentales.

### 24.2 Semilla de RAM Power-On
- RAM (1 KB lógica espejada) se inicializa con patrón pseudo-aleatorio reproducible vía xorshift64*.
- Semilla derivada: FNV-1a(hash(bios_bytes || cart_bytes || constante)) para correlacionar con contenido de cartucho/BIOS.
- Modo determinista para tests disponible (flag `deterministic_power_on`).

### 24.3 Tablas de Ciclos Data-Driven
- Archivo `emulator/src/cycle_table.rs` define arrays const para opcodes base y prefijos 0x10/0x11.
- CPU usa lookup directo; heurística previa de ciclos por grupos eliminada en favor de datos.
- Sentinela 0xFF marca huecos ilegales (no se cargan como ciclos válidos).

### 24.4 Infraestructura Micro-Steps
- Estructura `MicroStep { stage, cycles }` + buffer circular añadida.
- Flag global `MICRO_BREAKDOWN_ENABLED=false` (desactivado hasta tener partición de ciclos verificada documentalmente).
- Cuando se active: permitirá descomponer Fetch / Decode / EA / Execute / WriteBack para integrador / timing VGA futuro.

### 24.5 Clasificación Final de OpCodes
- Lista ilegal consolidada en `ILLEGAL_BASE_OPCODES` (ver `cpu6809.rs`).
- Escaneo de cobertura unificada reporta UNIMPL VALID COUNT = 0.
- Cualquier cambio futuro debe actualizar simultáneamente: constante, esta sección y tests (bloques unified_* en `opcodes_all.rs`).

### 24.6 Métricas Ajustadas
- `opcode_unimplemented` sólo cuenta verdaderos opcodes válidos sin handler (actualmente 0).
- Ilegales ejecutan como NOP de 1 ciclo y no tocan contador de unimplemented.

### 24.7 Próximos Pasos Relacionados
- Añadir tests específicos para ABX, LBSR, JSR indexed (edge cases de sign‑extend en branch largo ya cubiertos con LBSR base).
- Particionar ciclos por micro‑etapas (fetch vs memoria adicional) antes de habilitar microtraza.
- Validar tiempos exactos frente a referencia (doc / ciclo real BIOS) y ajustar tablas.

---
## 25. Expanded CHANGE NOTES
(Chronological – newest last)
- 2025-08-15: Initial compiler skeleton (pseudo→asm).
- 2025-08-16: Added bitwise ops & refined let handling.
- 2025-08-17: Added vectrex builtins & polygon macro.
- 2025-08-18: String literal escaping & print support.
- 2025-08-19: switch/case + circle/arc macros.
- 2025-08-20: Spiral macro & trig tables.
- 2025-08-22: Dead label pruning & constant folding pass.
- 2025-08-24: Unified Rust CPU; VIA timing hookup.
- 2025-08-26: Opcode coverage recompute + hotspot sampling.
- 2025-08-28: Integrator backend & WASM segment export.
- 2025-08-30: EmulatorPanel UX (base/pause/reload/toasts).
- 2025-09-01: `listSources` IPC + dropdown; removed manual detect.
- 2025-09-02: Indexed RMW cluster implemented.
- 2025-09-03: Placeholder opcodes 7B/8F.
- 2025-09-04: SUPER_SUMMARY initial.
- 2025-09-05: Deep dive + opcode appendix.
- 2025-09-17: Open bus unificado (`last_bus_value`), RAM power-on pseudo-aleatoria, tablas de ciclos data-driven, infraestructura micro-steps (desactivada), re-clasificación final de opcodes ilegales (cobertura válida = 100%), batch de implementación (LBSR, ABX, SBCB/ADCB variantes, ADDD direct, CMPX idx/ext, SUBA/SBCA/BITA/EORA/ORA extended, ADDB extended), tests `opcode_validity.rs` y `opcode_scan.rs` estabilizados.
 - 2025-09-19: Barrido 0x00–0xFF endurecido (ok + delta ciclos + bitmap) y fetch logging reducido (solo trace). Añadido helper público `opcode_marked_unimplemented`.
 - 2025-09-19: Migración layout vectores de interrupción a estándar 6809 (FIRQ=FFF6, IRQ=FFF8, SWI=FFFA, NMI=FFFC, RESET=FFFE) + helper `read_vector` big-endian; tests actualizados.

---
## 28. Línea de Tiempo de los 39 Pasos Recientes
1. (Audit) Auditoría inicial de heurísticas y simplificaciones (CPU/VIA/Integrator) con mapeo doc. [AUDIT]
2. (Heuristic Removal) Eliminación dependencia fuerte en WAIT_RECAL heurístico → autoridad pasa a ciclos reales / IRQ. [REMOVED_HEURISTIC]
3. (Synthetic Removal) Open bus para lecturas cart OOB (reemplaza byte sintético 0x01). [REMOVED_SYNTHETIC]
4. (Synthetic Removal) Open bus en regiones ilegales / gap (reemplaza 0xFF sintético). [REMOVED_SYNTHETIC]
5. (Realism) RAM power-on pseudo-aleatoria reproducible (sustituye patrón uniforme). [ADDED_REALISM]
6. (Refactor) Diseño tabla de ciclos base con sentinel para ilegales (prepara precisión). [STRUCTURAL]
7. (Heuristic Removal) Lookup de ciclos sustituye bloque condicional heurístico. [REMOVED_HEURISTIC]
8. (Infrastructure) Scaffold micro-etapas (desactivado hasta datos exactos). [NON_ACTIVE_INFRA]
9. (Spec Clarify) STOP evaluado y marcado N/A (evita stub falso). [AVOIDED_STUB]
10. (Classification) Separación opcodes inválidos vs válidos no implementados. [CLASSIFICATION]
11. (Testing Infra) `opcode_validity.rs` asegura ilegales=1 ciclo y no cuentan. [TEST_COVERAGE]
12. (Data Correctness) Ajuste lista ilegales (elimina falsos positivos p.ej. 0x0B). [CORRECTION]
13. (Coverage) Implementación LBSR (0x17). [ADDED_OPCODE]
14. (Coverage) Implementación ABX (0x3A). [ADDED_OPCODE]
15. (Coverage) Implementación CMPX indexed (0xAC) y extended (0xBC). [ADDED_OPCODE]
16. (Coverage) Implementación JSR indexed (0xAD). [ADDED_OPCODE]
17. (Coverage) Implementación SUBA extended (0xB0). [ADDED_OPCODE]
18. (Coverage) Implementación SBCA extended (0xB2). [ADDED_OPCODE]
19. (Coverage) Implementación BITA extended (0xB5). [ADDED_OPCODE]
20. (Coverage) Implementación EORA extended (0xB8). [ADDED_OPCODE]
21. (Coverage) Implementación ORA extended (0xBA). [ADDED_OPCODE]
22. (Coverage) Implementación SBCB immediate (0xC2). [ADDED_OPCODE]
23. (Coverage) Implementación SBCB direct (0xD2). [ADDED_OPCODE]
24. (Coverage) Implementación ADDD direct (0xD3). [ADDED_OPCODE]
25. (Coverage) Implementación ADCB direct (0xD9). [ADDED_OPCODE]
26. (Coverage) Implementación SBCB indexed (0xE2). [ADDED_OPCODE]
27. (Coverage) Implementación ADCB indexed (0xE9). [ADDED_OPCODE]
28. (Coverage) Implementación ADDB extended (0xFB). [ADDED_OPCODE]
29. (Coverage) BITB direct/extended y EORB direct/indexed añadidos. [ADDED_OPCODE]
30. (Heuristic/Synthetic Cleanup) Limpieza handlers redundantes / pseudo-NOPs. [REMOVED_STUB]
31. (Testing Infra) `opcode_scan.rs` para enumerar válidos unimplemented. [TEST_COVERAGE]
32. (Measurement) Primer scan: 35 válidos pendientes. [BASELINE]
33. (Iteration) Batch reduce a 16. [REDUCTION]
34. (Spec Validation) Auditoría mapa MC6809 confirma 16 huecos inválidos. [SPEC_VALIDATION]
35. (Classification Final) Añadidos 16 a `INVALID_BASE_OPCODES`. [CLASSIFICATION]
36. (Coverage Goal) Scan final 0 válidos unimplemented. [GOAL_ACHIEVED]
37. (Docs) Apéndice actualizado lista ilegal final. [DOC_UPDATE]
38. (Metric Integrity) Métrica ajustada: ilegales excluidos de unimplemented. [METRIC_FIX]
39. (Handoff) Preparación de SUPER_SUMMARY post-reinicio (este documento). [HANDOFF]

### 28.1 Checklist Eliminación de Heurísticas / Stubs / Sintéticos
| Nº | Paso | Tipo | Estado |
|----|------|------|--------|
|2|WAIT_RECAL heurístico|Heuristic|Completado|
|3|Cart OOB 0x01 sintético|Synthetic|Completado|
|4|Illegal/gap 0xFF sintético|Synthetic|Completado|
|7|Bloque heurístico ciclos|Heuristic|Completado|
|9|STOP stub evitado|Stub|Completado|
|12|Falsos ilegales removidos|Data fix|Completado|
|30|Pseudo-NOP redundantes|Stub/Synthetic|Completado|
|35|Últimos 16 inválidos reclasificados|Classification|Completado|
|38|Métrica contando ilegales|Heuristic metric|Completado|

Nota: Micro-steps (8) se creó infraestructura pero se mantuvo desactivada deliberadamente para no introducir heurística especulativa de partición de ciclos.

---
## 29. Próximos Focos (Post-Paso 39)
- Escribir tests dirigidos para nuevos opcodes críticos (LBSR path edge cases, ABX overflow, JSR idx EA variantes).
- Validar y refinar tiempos de ciclos contra referencias externas (añadir harness comparativo).
- Activar micro-breakdown tras validar partición de ciclos documentada.
- Añadir persistencia de semilla power-on y toggle determinista vía WASM API.
- Documentar política open-bus en `docs/` dedicada y vincular aquí (sección 24.1).

---
## 26. Maintenance Checklist Mapping
Change CPU opcode -> Sections 3.1 / 24.
Change VIA timing -> Sections 3.2 / 21.3.
Change integrator algorithm -> Sections 3.3 / 21.4.
Add language feature -> Sections 22 / 23 + CHANGE NOTES.
Modify IPC -> Section 6.
Panel UX -> Section 7.
Persistence semantics -> Sections 5 / 10.

---
## 27. Doc Backlog
- Add DAA instruction & tests.
- Export extended coverage via metrics for UI.
- EBNF grammar formalization.
- Auto-generated opcode table script (avoid drift).
- Add symbol emission & integrate with OutputPanel.


---
## 28. Pending Tasks (Audio, Controls, Opcode Gaps & Synthesized Behaviors)
This section consolidates cross-cutting functional gaps and planned work specifically requested: audio, input controls, any missing / placeholder opcodes (CPU & VIA), opcodes currently treated as NOP, and synthesized operations (emulator conveniences that do not exist as real hardware instructions/behaviors). Keep this updated as features land.

### 28.1 Audio (PSG) – Not Yet Implemented
Vectrex uses a General Instrument AY-3-8912 (PSG). Currently no sound path exists.

TEMP (2025-09-19): Se implementará un stub mínimo de avance musical (únicamente para permitir que la BIOS complete la intro y limpie Vec_Music_Flag). Este stub:
- No generará audio.
- Sólo replicará pasos estrictos (decremento de duración, avance de punteros, detección de terminador) basándose en datos reales escritos por la BIOS a las estructuras de música.
- Debe ELIMINARSE en cuanto se implemente emulación real del AY-3-8912. Añadir recordatorio aquí evita que el stub se perpetúe y viole la política de “no heurísticas permanentes”.

Checklist al retirar el stub:
1. Implementar temporización real de canales / envolventes ADSR según tablas BIOS.
2. Mapear registros shadow ($C800-$C80E) a estado interno del emulador de PSG.
3. Verificar que la intro BIOS (Mine Storm) progresa sin necesidad de lógica auxiliar.
4. Actualizar esta sección y eliminar este bloque TEMP.
Pending work:
- Memory / I/O mapping: Decide addressing interface (BIOS expects VIA port lines & PSG latch writes; need abstraction layer).
- Implement PSG register model (16 regs: tone (A/B/C), noise, mixer, amplitudes, envelope period/shape, I/O port).
- Audio sample generation: Envelope + square wave + noise mixing; choose internal mixing rate (e.g. 44.1 kHz or 48 kHz) with simple linear interpolation to AudioContext rate.
- WASM <-> JS bridge: Ring buffer or AudioWorklet (preferred) for low-latency streaming.
- Volume scaling / mute toggle / enable flag.
- Performance: Batch generate per frame or fixed sample quantum (e.g. 512 samples) decoupled from video frames.
- Testing: Golden register write sequences producing audible tones with stable pitch.

Acceptance criteria:
- BIOS sound test rom produces audible tones with stable pitch.
- No XRuns / underruns reported in AudioWorklet (buffer fullness > 50%).
- Latency < 100 ms end-to-end (host key -> audible tone) for interactive tests.

### 28.2 Controls / Input (Joystick & Buttons)
Status: Minimal implemented (host → emu write only). A primer input API expone `set_input_state(x,y,buttons)` vía WASM. Se escribe un snapshot en RAM $00F0..$00F2:
 - $00F0: X (unsigned biased, -128..127 mapeado a 0..255)
 - $00F1: Y (idéntico esquema)
 - $00F2: Bits de botones (bit0..bit3 = botones 1..4)

Pendiente (futuro): integrar con líneas VIA / PSG reales (puertos), soporte Gamepad API, configuración UI (dead‑zone, sensibilidad), detección de flancos (edge) y estados hold, así como mapping analógico progresivo en lugar de asignación directa.

Testing actual: test de unidad asegura escritura/masking correcto y clamp de rango. Próximos tests deberán validar lectura BIOS real cuando se use mapeo por puerto.

Acceptance criteria (extendida futura):
 - Demo lectura joystick devuelve valores estables y centrados en idle.
 - Botones reflejan transiciones edge y no rebotan espuriamente.
 - Reconexión de gamepad sin bloquear loop.

### 28.3 CPU Opcode Gaps / Accuracy
Currently implemented majority of core set; known omissions / approximations:
- DAA (0x19): Not implemented. Needed for BCD arithmetic (rare for typical Vectrex demos but required for completeness). Plan: Implement nibble adjust using A + correction based on lower & upper nibbles / carry & half-carry flags; add unit tests vs reference table.
- Half-Carry (H) flag semantics: Placeholder; ensure correct for ADC/DAA interplay once DAA added.
- Placeholders (treated as NOP): 0x7B, 0x8F (verify against authoritative 6809 opcode matrix; either implement if valid or formally classify as illegal/undefined and keep as NOP with comment referencing spec).
- Timing accuracy: Cycle counts are grouped approximations; future refinement may differentiate addressing modes for tighter demo timing (list to update when started).

Planned validation:
- Cross-check against published 6809 opcode table; produce generated JSON spec locked in repo.
- Add opcode unit tests: For every arithmetic/logical opcode, validate flags (N,Z,V,C) against reference emulator or precomputed vectors.

### 28.4 VIA / Peripheral Gaps
Current VIA model covers basic timers + IFR/IER bits; missing or simplified:
- Shift register full behavior (modes, clock source options, serial in/out) – presently intensity latch experiment only.
- PB7 audio toggle side-effects (currently only conceptual for integrator; tie into audio when PSG present).
- Precise timer underflow-to-reload timing (edge vs level IRQ timing nuance).
- FIRQ source support (if any planned) – presently only standard IRQ line asserted.
- Control line (CA1/CA2/CB1/CB2) handshake modes & latching.

Planned actions:
- Implement shift register mode state machine; verify IFR bit set/clear timing with test vectors.
- Introduce feature flags env (e.g. VIA_EXACT_TIMING) gating stricter cycle details.
- Provide debug dump of VIA registers over frame for profiling.

### 28.5 Opcodes Treated as NOP (Explicit List)
These opcodes currently execute as no-ops to keep execution flowing and suppress unimplemented spam:
- 0x7B – Placeholder (classification TBD)
- 0x8F – Placeholder (classification TBD)
Additionally: Any undefined opcode not matched in the main execution match arm falls back to a generic unimplemented handler that logs once and effectively behaves like NOP (after metrics update).

Action: Replace placeholders with accurate implementations or mark permanently illegal with assertion guards in debug builds.

### 28.6 Synthesized / Emulation Convenience Behaviors
These behaviors do not exist as literal hardware instructions but are introduced for practicality or metrics:
- Frame boundary synthesis: WAIT_RECAL BIOS call depth tracking to delimit frames alongside cycle budgeting.
- IFR bit7 synthesis: Master pending computed from other IFR bits (mirrors hardware logic but implemented explicitly).
- Opcode coverage recompute: Artificial single-step execution of all opcodes at startup to populate `opcode_unimpl_bitmap` (does not reflect real runtime path execution order).
- Demo triangle (`demoTriangle()`): Purely synthetic segments for UI sanity check (no CPU activity).
- Loop hotspot sampling: Lightweight sampling of PCs executing opcode 0x00/0xFF for heuristic loop detection – not a hardware feature.
- Placeholder opcodes acting as NOP to prevent halting behavior while awaiting spec confirmation.
- Potential future: Synthetic trace breakpoints & forced frame flush for stuck detection (planned; not yet implemented).

Documentation Requirements:
- Each synthesized behavior should have inline code comments + Section 28 reference tag `[Synth28]` for easier grep.

### 28.7 Risk / Priority Snapshot
High Priority (to unblock richer demos): Controls (input), DAA (for full spec compliance if needed by future codegen), Audio (if targeting full platform parity).
Medium: VIA shift register accuracy, placeholder opcode resolution.
Low: Fine-grained cycle timing, synthesized behavior documentation tags.

### 28.8 Tracking & Integration
Add CI task (future) to fail build if Section 28 list references opcode already implemented without updating status. Simple script: parse section, compare to generated opcode spec.

---
## Maintenance Guidance
When modifying core emulator or build system, update:
- Affected opcode coverage (Section 3.1)
- IPC additions (Section 6)
- Panel UX changes (Section 7.1 / 7.2)
- Decision Log (Section 18)
- Backlog if new tasks emerge

PR checklist suggestion (add to template):
- [ ] Updated SUPER_SUMMARY.md
- [ ] Ran emulator smoke test (Build & Run example + vectors drawn)
- [ ] Verified BIOS detection
- [ ] Confirmed no new unimplemented opcodes appear unintentionally

---
End of document.

---
## 30. Stack / Return Diagnostics (2025-09-18)

Context:
Early BIOS bootstrap traces revealed unexpected execution falling into regions of repeating `0x00` bytes (interpreted as `NEG` / NOP), suggesting corrupted or misaligned return addresses after `PULS/RTS/RTI` sequences.

Instrumentation Added:
- Call opcodes (`BSR`, `JSR` (idx/ext), `LBSR`) now record a `pending_ret_addr` before pushing onto the stack; this is stored as `ret_addr` in each `StackEvent`.
- Event structure extended with diagnostic flags placeholders (overwrite, alias, drift, destroyed, mismatch) to enable fine-grained classification.
- Rust BIOS startup test relaxed (warns instead of asserting) when logical call stack depth exceeds retained events due to fixed buffer (32 entries).
- C++ test `BiosStartup.EarlyInstructionTrace` added in `vectrexy` for cross-implementation parity (build pending until CMake available locally).

Current Limitations / Issues:
1. Event buffer capacity (32) truncates early BIOS call history → older frames overwritten, impeding accurate drift/mismatch analysis.
2. Classifier logic does not yet use `ret_addr` to distinguish genuine stack corruption from stack pointer displacement (e.g., interrupt frames).
3. Heuristic to detect interrupt frames "below" a return may mislabel legitimate nested frames as drift.
4. Visibility warning: event list exposure vs `StackEvent` privacy unresolved (API stability concern for future UI export).

Planned Immediate Work:
1. Expand buffer (e.g., 256) or convert to ring with generation counter & loss metrics.
2. Implement return classifier:
  - If popped PC == stored `ret_addr` → confirm (no issue).
  - If popped PC appears in a deeper frame but `ret_addr` still present lower → classify as drift.
  - Else if bytes at expected stack slot differ from saved pattern → mismatch.
3. Refine interrupt-below detection using original frame `low_addr` snapshot instead of current `S`.
4. Export aggregated counts (`ret_mismatch_count`, `ret_drift_count`, `stack_overwrite_count`) through metrics JSON/WASM.
5. Resolve visibility (public struct vs accessor methods returning immutable slice + counts).
6. Run C++ test post toolchain install to confirm parity and reproduce any mismatches across implementations.

Success Criteria:
- BIOS first ~128 instructions execute with zero mismatches; any drift events are justified (documented interrupts) and count stable.
- No buffer overflow (or, with ring, zero lost events) during early bootstrap interval.
- Metrics visible and stable between Rust & C++ traces.

Follow-Up (Post Success):
- Optional: selective event filtering (e.g., ignore balanced returns) to reduce telemetry noise.
- Add regression test asserting zero mismatches on BIOS bootstrap for deterministic BIOS image.

---
## 31. External Tools / C++ Parity Status (2025-09-18)
Purpose: Track auxiliary C++ implementation efforts (`vectrexy`) used for parity checks and investigative tooling.

### 31.1 Null Engine Build Path
- Configured to bypass SDL2/ImGui legacy backend friction by selecting `ENGINE_TYPE=null` and disabling tests.
- Resulting `vectrexy.exe` builds successfully; runtime provides minimal loop suitable for headless validation.
- Rationale: Decouple emulator parity investigations from stalled UI dependency updates.

### 31.2 `bios_callstack` Tool
- Standalone executable added under C++ libs/emulator/tools capturing BIOS call stack frames.
- Supports: JSR direct (0xBD), JSR extended (0xBD), JSR indexed (0xAD), BSR (0x8D). Logs target addresses and maintains depth.
- Added heuristic for indexed JSR detection via stack pointer delta.
- Instrumentation (first 64 opcodes trace) revealed early BIOS loop (PC cycling F548–F54D) prior to premature process exit (code 1) — root cause undiagnosed (likely external abort or exit path in harness).
- Work Paused: Per strategic refocus on Rust core and compiler.

### 31.3 Pending / Future Parity Work
- Re-run C++ tool after upgrading ImGui + backend to modern version (remove deprecated fields).
- Align C++ CPU step timing table with Rust `cycle_table.rs` for apples-to-apples cycle diff.
- Optionally export call stack JSON and compare against Rust stack event metrics once classifier implemented (see Section 30).

### 31.4 De-scoping Justification
- Rust emulator now authoritative; maintaining two active CPU cores risks divergence.
- C++ path retained only for: (a) comparative debugging if Rust trace anomalies arise, (b) potential future native instrumentation.

### 31.5 Cleanup Candidates
- If no parity regressions appear over next milestone, archive or mark C++ tooling experimental in repository docs.

Reference Cross-links: Sections 3.1 (CPU), 30 (Stack Diagnostics).

### 32. Opcode Metadata Scaffold (2025-09-18)
Context: Inicio de migración hacia tablas data-driven de longitud y ciclos por instrucción.

Added:
- `emulator/src/opcode_meta.rs` con `OpcodeMeta` (opcode completo con prefijo, tamaño bytes, ciclos base, flags de branch).
- Subconjunto inicial: LDS inmediato (10 CE), JSR extendido (BD), BRA, BSR, RTS, SUBB inmediato.
- Test `opcode_meta_subset.rs` valida que PC delta y ciclos reales coinciden (BRA tomado = base+1 se comprueba aparte de base_cycles almacenado).

Motivación:
- Separar semántica estática (tamaño/ciclos base) de la lógica de ejecución para permitir futura verificación automatizada y simplificación del gran `match` en `cpu6809.rs`.

Estado:
- Pasivo: no modifica la ejecución; sólo provee superficie de consulta para tests.
- Siguiente paso previsto: expandir cobertura (todas las cargas/ALU básicas) y unir con `cycle_table.rs` o fusionar en una única fuente de verdad.

Riesgos / ToDo:
- Duplicación temporal de valores (inline seeds vs meta). Mitigación: añadir verificación incremental hasta migrar.
- Aún no modela variaciones por postbyte indexado ni máscaras de pila (PSHS/PULS).

### 32.1 Actualización Semántica Stack 6809 (2025-09-18)
Correcciones aplicadas:
1. IRQ/FIRQ frame order: ahora el emulador empuja en IRQ exactamente `CC, A, B, DP, X, Y, U, PC` y en FIRQ únicamente `CC, PC` (sin registros extra) respetando la referencia MC6809. Antes el orden estaba invertido provocando retorno PC corrupto tras `PULS` durante inicialización BIOS.
2. PSHS (mask) push order normalizado a la secuencia hardware al procesar bits 7→0: cuando una máscara incluye múltiples registros se almacenan en orden `PC, U, Y, X, DP, B, A, CC` (PC high primero en la dirección más alta). Tests anteriores asumían un orden lógico invertido (CC primero) y fueron actualizados.
3. PULS pop order ahora procesa bits ascendentes (0→7), extrayendo en orden `CC, A, B, DP, X, Y, U, PC`. Se revertió un cambio temporal que intercambió A/B para satisfacer un test, alineando definitivamente con la especificación hardware y ajustando los tests (`opcode_puls_ab`, `pshs_full_mask_and_puls_restore`).

Impacto en pruebas:
- `bios_puls_return_valido` y `bios_puls_rango_irq` validan que la corrección de frame IRQ produce PC esperado (F7CC rango válido) tras la secuencia de BIOS inicial.
- `pshs_full_mask_and_puls_restore` reescrito para verificar layout exacto de pila siguiendo el nuevo orden documentado.

Documentación / Próximos pasos:
- Pendiente incorporar a la futura tabla de metadatos de opcodes la variación de ciclos dependiente del número de registros en PSHS/PULS.
- Añadir sección de ejemplo de volcado de pila antes/después de IRQ para trazas WASM cuando se implemente export de call stack (TODO ID 13).

### 32.2 Hooks de Trazado Adicionales (2025-09-19)
Actualización: Sistema de logging unificado para entradas de interrupción.

Hooks activos (solo cuando `trace=true`):
1. `[INT ENTER kind=K prev_pc=PPPP sp=SSSS vec=VVVV]` – Empleado ahora para todas las rutas de servicio (`IRQ`, `FIRQ`, `NMI`, `SWI`, `SWI2`, `SWI3`). Reemplaza los mensajes separados `[IRQ ENTER ...]`, `[FIRQ ENTER ...]`, etc. Formato estable para parsing automático futuro.
2. `[BIOS->CART] handoff pc=XXXX` – Emitido una sola vez al primer retorno (`RTS`, `PULS` con bit PC o `RTI`) que cruza de BIOS (>=0xE000) a cartucho (<0xE000). Conservado sin cambios.

Detalles:
- Helper interno `log_interrupt_enter(kind, prev_pc, sp_before, vec)` centraliza formato y gating.
- Todas las rutas (incluyendo NMI y SWI/SWI2/SWI3) ahora usan `read_vector()` big-endian para coherencia tras la migración del layout de vectores (ver 21.2.1).
- Mensajes anteriores específicos (`[IRQ ENTER pc=...]`, `[FIRQ ENTER pc=...]`, `[NMI SERVICE]`, `[SWI SERVICE]`) fueron retirados para reducir ruido y facilitar regex único.

Consideraciones futuras:
- Exportar estos eventos en JSON (junto a `trace_log_json`) o integrarlos en el planned `bios_calls_json()` (TODO ID 13) con tipo y vector.
- Añadir flag incremental que permita filtrar únicamente tipos específicos (`trace_int_mask`).

