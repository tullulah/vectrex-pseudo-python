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

#### 3.1.3 Planned Enhancements
- Data-driven opcode table (mnemonic, cycles, flags) to shrink match arm duplication.
- Selective trace filters (PC allowlist) for low-noise debugging.
- Golden trace comparison harness.

### 3.2 VIA & Timing
- VIA memory-mapped region at 0xD000 range (simplified mapping). Timers tick via centralized `advance_cycles()`.
- Interrupt servicing order: NMI > FIRQ (if F flag clear) > IRQ (if I flag clear). WAI halts until serviced.

### 3.3 Integrator / Vector Generation
- Beam simulation converts DAC position & intensity changes into `Segment` records (x0,y0,x1,y1,intensity).
- Access from JS via wasm exports: `getSegmentsShared()` (shared memory view) + `drainSegmentsJson()` fallback.
- Frame boundary detection leverages BIOS WAIT_RECAL returns (tracking call depth) + internal cycle framing.

---
## 4. WASM API
File: `emulator/src/wasm_api.rs`
Exports (selection):
- Initialization: `init()`, `load_bios(bytes)`, `load_program(bytes, base)`.
- Execution: `runFrame()`, (per-frame run advancing until frame boundary heuristics), `reset()`.
- Introspection: `registers()`, `metrics()`, `loopWatch()`.
- Vector data: `getSegmentsShared()` (preferred zero-copy) and `drainSegmentsJson()` (JSON fallback).
- Demo: `demoTriangle()` returns canned triangle segments (used for visual sanity test / demo mode).

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

---
## 21. CPU / VIA / Integrator Deep Dive
### 21.1 CPU Flags & Registers
A,B (8-bit) forming D, X,Y,U,S (16-bit), DP (high byte for direct), CC bits EFHINZVC. E marks full frame pushed; F masks FIRQ; H reserved (half-carry pending proper BCD support); I masks IRQ.
### 21.2 Interrupt Entry Summary
| Src | Frame | Sets E | Sets F | Sets I | Vector | Return |
|-----|-------|--------|--------|--------|--------|--------|
| IRQ | Full  | Y | N | Y | 0xFFF6 | RTI |
| FIRQ| Partial (PC+CC) | N | Y | Y | 0xFFF4 | RTI |
| NMI | Full  | Y | N | Y | 0xFFFA | RTI |
| SWI | Full  | Y | Y | Y | 0xFFF8 | RTI |
| SWI2| Full  | Y | Y | Y | 0xFFF2 | RTI |
| SWI3| Full  | Y | Y | Y | 0xFFF0 | RTI |
| WAI | Pre (once) | Y | – | – | (next int) | RTI |
### 21.3 VIA 6522 Map (0xD000)
| Ofs | Reg | Notes |
|-----|-----|-------|
|00|ORB|Experimental horizontal velocity|
|01|ORA|Experimental vertical velocity|
|04|T1C-L|Read clears IFR6|
|05|T1C-H|Read clears IFR6 (if set) & reloads|
|08|T2C-L|No clear|
|09|T2C-H|Read clears IFR5|
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
Legend: [I]=Implemented [P]=Placeholder [U]=Undefined→NOP. Extended valid sub-opcodes listed in code (`VALID_PREFIX10/11`).
RMW Direct: 00,03,04,06,07,08,09,0C,0D,0E,0F [I]
RMW Indexed: 60,63,64,66,67,68,69,6A,6C,6D,6E,6F [I]
RMW Extended: 70,73,74,76,77,78,79,7A,7C,7D,7E,7F [I]
Placeholders: 7B,8F [P]
Branches short 20–2F [I]; Long branches prefix 0x10 (21–2F) [I].
Prefix 0x10 groups: CMPD, CMPY, LDY, STY, LDS, STS, SWI2.
Prefix 0x11 groups: CMPU, CMPS, SWI3.
Missing/Planned: DAA, any unimplemented math helpers (if needed by future codegen).

Coverage Tool: `recompute_opcode_coverage()` populates `opcode_unimpl_bitmap` + `last_extended_unimplemented` for diagnostics.

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
Pending work:
- Memory / I/O mapping: Decide addressing interface (BIOS expects VIA port lines & PSG latch writes; need abstraction layer).
- Implement PSG register model (16 regs: tone (A/B/C), noise, mixer, amplitudes, envelope period/shape, I/O port).
- Audio sample generation: Envelope + square wave + noise mixing; choose internal mixing rate (e.g. 44.1 kHz or 48 kHz) with simple linear interpolation to AudioContext rate.
- WASM <-> JS bridge: Ring buffer or AudioWorklet (preferred) for low-latency streaming.
- Volume scaling / mute toggle / enable flag.
- Performance: Batch generate per frame or fixed sample quantum (e.g. 512 samples) decoupled from video frames.
- Testing: Golden register write sequences producing deterministic short WAV snapshot for regression.

Acceptance criteria:
- BIOS sound test rom produces audible tones with stable pitch.
- No XRuns / underruns reported in AudioWorklet (buffer fullness > 50%).
- Latency < 100 ms end-to-end (host key -> audible tone) for interactive tests.

### 28.2 Controls / Input (Joystick & Buttons)
Status: Not implemented (no mapping of host inputs to emulated memory / VIA lines).
Vectrex joystick: Analog X/Y + 4 buttons (1–4). Tasks:
- Decide representation: Provide sampled values in expected BIOS polled addresses (likely via VIA port bits or PSG I/O port depending on hardware model).
- Host mapping: Keyboard (e.g. WASD / arrow keys) + Gamepad API fallback (first connected pad).
- Normalization: Convert digital key presses to centered analog (-128..127) with ramp for smooth motion or immediate edges (configurable).
- Debounce / repeat: Distinguish edge vs held states for buttons.
- UI config panel: Rebind keys, dead zone slider, analog sensitivity.
- Testing: Inject synthetic input sequence and verify BIOS reading (unit test verifying memory/register snapshots over frames).

Acceptance criteria:
- Example program reading joystick shows expected value range.
- Buttons register distinct edges; no ghost presses when idle.
- Gamepad connect/disconnect handled without crash.

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
