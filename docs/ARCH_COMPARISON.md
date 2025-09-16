# Architecture Comparison: Current Emulator vs vectrexy

## Overview
This document captures key differences between the current Rust/WASM Vectrex emulator implementation and the reference C++ project *vectrexy*. It motivates forthcoming refactors and serves as a living design baseline.

## Goals
- Increase determinism & hardware fidelity.
- Decouple frame timing from BIOS structure.
- Transition from BIOS vector list parsing to beam simulation.
- Improve observability & safety (memory map, diagnostics).
- Provide staged migration path with feature flags to avoid regressions.

## Comparison Matrix
| Concern | Current (Rust) | vectrexy (C++) | Planned Direction |
|---------|----------------|----------------|-------------------|
| Frame Boundary | WAIT_RECAL RTS/FIRQ heuristic; IRQ fallback | Host-driven 50Hz pacing + cycle budget | Add cycle-based frame counter; keep BIOS frame as instrumentation |
| Cycle Accounting | Approx per opcode; coarse timing integration | Per-instruction cycles -> bus sync | Introduce bus sync; tighten cycle counts incrementally |
| Vector Generation | Parsed DRAW_VL + events (MoveTo, etc.) | Analog beam/integrator simulation producing line list | Add beam backend w/ feature flag; later deprecate parser |
| VIA Timing | Partial IRQ + limited register semantics | Full timers, shift register, screen update each sync | Expand sync coverage post bus layer |
| Memory Map | Ad hoc fills (0xFF) for cartless state | Explicit mappings & illegal region device | Centralize map logic; add diagnostics counters |
| Unmapped Cart Reads | 0xFF from filled memory | Returns 0x01 + warning if OOB | Return 0x01 + counter; rate-limited log |
| Frame Metrics | Single frame counter (bios) | Engine frame concept only | Dual metrics: bios_frame & cycle_frame |
| Brightness/Intensity | Simple persistence | Curved brightness + ramp phases | Add curve & (optional) ramp after basic integrator |
| Testing | Minimal/informal | (Project specific tests) | Add unit tests: cycles, memory map, integrator output |
| Device Sync Granularity | Event-driven inside step | After each instruction via memory bus | Mirror vectrexy granularity initially |
| Cartridge Validation | None | Structural ROM heuristic | Lightweight validation, metrics flag |

## Migration Phases (Summary)
1. Cycle-based frames & dual counters.
2. Bus sync & per-instruction device advancement.
3. Minimal integrator (beam) backend.
4. Memory map enforcement & diagnostics.
5. UI/metrics expansion & deprecate IRQ fallback.
6. Testing harness & performance profiling.
7. Documentation & cleanup (drop legacy parser).
8. Fidelity upgrades (brightness curve, ramp delays).

## Risks & Mitigations
| Risk | Mitigation |
|------|------------|
| Performance drop from per-instruction sync | Profile; batch after correctness proven |
| Visual differences confuse users | Feature flag + side-by-side metrics |
| Legacy tools relying on frame counter semantics | Keep bios_frame; document new cycle_frame |
| Increased JSON payload size | Optional minimal metrics mode |

## Key Constants (Target Values)
| Concept | Value |
|---------|-------|
| CPU Frequency (default) | 1_500_000 Hz |
| Target Refresh | 50 Hz |
| Cycles Per Frame | 30_000 (approx) |

## Next Artifacts
- docs/TIMING.md (detailed frame loop)
- docs/VECTOR_MODEL.md (legacy vs beam mode)

---
*Maintained as implementation proceeds; update when major design choices change.*
