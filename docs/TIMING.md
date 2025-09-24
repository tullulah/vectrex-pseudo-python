# Timing & Frame Model

This document explains the deterministic timing system introduced in the Rust `vectrex_emulator` crate. It replaces earlier WAIT_RECAL/IRQ heuristics with a cycle‑driven authoritative frame counter while retaining BIOS observational metrics for tooling.

## Definitions
- `cycle_frame` (authoritative): Increments every time accumulated CPU cycles exceed `cycles_per_frame` (≈ `CPU_FREQ / 50Hz`). Drives per‑frame statistics (segment stats, averaging, etc.).
- `frame_count` (legacy mirror): Mirrors `cycle_frame` for frontends still reading the old field.
- `bios_frame` (observational): Increments only when the BIOS frame routine (WAIT_RECAL return path) is actually observed (or historically when a deprecated heuristic fired). Used to debug BIOS sequencing divergences.
- `cycles_per_frame`: Computed at reset from the effective CPU frequency (default 1_500_000 Hz / 50 → 30,000). Override via env `VPY_CPU_FREQ` (integer Hz) for experiments.

## Rationale
1. Determinism: Host pacing (wall clock / sleep) is avoided in core logic; simulation advances strictly by executed instruction cycle counts.
2. Diagnostics: Retaining `bios_frame` lets us detect when BIOS frame cadence desynchronizes (e.g., bad WAIT_RECAL path, missed interrupt, or custom homebrew bypassing BIOS loops).
3. Decoupling: Rendering is handled solely by the integrator backend; higher‑level UIs don’t depend on BIOS quirks, enabling testing with synthetic ROMs.

## Cycle Accumulator Flow
Pseudo‑logic (see `CPU::advance_cycles`):
```
cycle_accumulator += instruction_cycles
while cycle_accumulator >= cycles_per_frame:
    cycle_accumulator -= cycles_per_frame
    cycle_frame += 1
    frame_count = cycle_frame (legacy mirror)
    collect_integrator_stats()
    maybe_validate_cartridge_once()
```
All per‑frame instrumentation hooks piggyback on this rollover.

## VIA & Device Sync
- Each instruction’s base cycle cost (approximate per opcode table) is passed to `advance_cycles`, which in turn ticks the Bus & VIA timers.
- Timer IFR bits are sampled post‑tick to count expiries (`t1_expiries`, `t2_expiries`).
- Interrupt service routines increment `irq_count` / `firq_count` immediately on entry.

## BIOS Interaction
WAIT_RECAL is no longer responsible for frame increments; it is purely instrumented (counts calls, logs addresses) so BIOS irregularities can be surfaced without influencing simulation time.

## Deprecated / Removed Heuristics & Paths
- Legacy vector event parser / dual backend toggle removed (integrator only now).
- IRQ fallback frame forcing removed. Environment variable `TRACE_FRAME_FORCE` is inert.

## Integrator Coupling
On each frame rollover (cycle based):
- Segment statistics are captured: last frame segment count, max, cumulative total.
- Optional auto‑drain clears segments if `integrator_auto_drain` is enabled; otherwise, they accumulate until drained via API.
- Average lines (segments) per frame is computed as `lines_per_frame_accum / lines_per_frame_samples` and exposed through metrics JSON.

## Testing & Validation
Current tests cover:
- Instruction cycle seeding for representative opcodes.
- Memory map edge behaviors.
- Integrator output semantics (merge on/off, blanked beam, intensity zero).
Future timing validation tests will compare known small BIOS loops or synthetic delay loops against expected cycle_frame increments.

## Future Enhancements
| Area | Plan |
|------|------|
| Precise cycle costs | Refine per opcode table; add indexed mode micro‑adjustments. |
| Timer accuracy | Model VIA Timer1/Timer2 tick-by-tick with reload delays and IRQ jitter. |
| Vertical blank | Introduce explicit vblank window metrics derived from BIOS or timer events. |
| Throttling mode | Optional host pacing: sleep to approximate real time when embedded in UI (outside core). |

## Quick Reference
| Field | Meaning | Source |
|-------|---------|--------|
| `cycles` | Total executed CPU cycles | Sum of instruction costs |
| `cycles_per_frame` | Target cycles per 50Hz frame | Derived from CPU freq |
| `cycle_frame` | Authoritative frame index | Accumulator rollover |
| `bios_frame` | Observed BIOS frame count | WAIT_RECAL instrumentation |
| `t1_expiries` / `t2_expiries` | VIA timer expiry counters | IFR sampling |
| `irq_count` / `firq_count` | Interrupt service entries | IRQ/FIRQ handlers |

Maintain parity: update this doc if timing constants, frame rollover triggers, or BIOS instrumentation semantics change.
