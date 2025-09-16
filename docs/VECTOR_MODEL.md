# Vector Generation (Integrator Backend Only)

The emulator now exposes a single, authoritative vector generation backend: a cycle‑integrated beam "Integrator". The former legacy parser (BIOS DRAW_VL interception) has been removed. This document describes the current model and planned fidelity improvements.

## Integrator Overview
Structure: `Integrator` maintains a `BeamState` and a list of `BeamSegment` objects:
```
BeamState { x, y, vx, vy, intensity, beam_on }
BeamSegment { x0, y0, x1, y1, intensity, frame }
```
Each instruction’s cycle count invokes `integrator.tick(cycles, current_cycle_frame)` from `CPU::advance_cycles`.

### Emission Rules
1. If `beam_on` AND `intensity > 0`: produce/extend a segment for the elapsed cycles.
2. Otherwise: only position integrates (blank slews) — no segment emitted.
3. Position update: `x += vx * cycles`, `y += vy * cycles` (linear, no decay yet).

### Line Merging Heuristic
With `merge_lines = true` (default):
- Two consecutive segments merge if same frame, same intensity, and are collinear & non‑reversing.
- Collinearity test: `|cross| < ε` with `cross = (dx_prev * dy_new) - (dy_prev * dx_new)` and `dot = dx_prev * dx_new + dy_prev * dy_new >= 0`.
- On merge, only `x1,y1` of the last segment extend; no new allocation.

### Auto‑Drain (Optional)
If `integrator_auto_drain` is enabled (default true in some builds), segments are cleared each authoritative frame rollover (cycle_frame increment) after statistics snapshot:
- `integrator_last_frame_segments`
- `integrator_max_frame_segments`
- `integrator_total_segments`

Disabling auto‑drain allows cumulative capture (e.g., for longer profiling sessions) — caller must then explicitly drain via API (e.g., `segments_json` or equivalent method when exposed).

### Intensity Mapping
Currently intensity is a direct 0–255 latch (taken from experimental VIA write mapping). Future improvements will introduce:
- Non‑linear brightness curve (gamma‑like) for display reproduction.
- Optional analog fade/ramp when beam_on toggles or velocity changes abruptly.

## Legacy Parser Removal
Previously, a heuristic parser intercepted BIOS routines (e.g. DRAW_VL) to synthesize semantic lines. It was limited by coarse timing granularity and inability to express mid‑line analog effects. Full reliance on the integrator eliminates divergence and simplifies testing. Any remaining code references to parser artifacts should be treated as bugs.

## Metrics & Diagnostics
| Metric Field | Meaning |
|--------------|---------|
| `integrator_last_frame_segments` | Segment count in the most recently completed authoritative frame |
| `integrator_max_frame_segments` | Peak per‑frame segment count since reset |
| `integrator_total_segments` | Cumulative emitted segments (after merges) |
| `integrator_auto_drain` | Boolean flag reflecting current drain policy |
| `lines_per_frame_avg` | Approx: `lines_per_frame_accum / samples` (same as segments when merges stable) |

## Testing Strategy
Implemented tests (`integrator_output.rs`):
- Single segment generation
- Collinear merge on
- Merge disabled behavior
- Beam blank (no segment) with motion
- Zero intensity (suppressed segment)

Planned future tests:
- Merge boundary edge (slight floating point divergence)
- Frame boundary segmentation (ensure segments carry correct frame index)
- Mixed intensity changes splitting segments

## Extensibility Roadmap
| Feature | Direction |
|---------|-----------|
| Brightness curve | Apply non‑linear mapping before segment record & UI export |
| Ramp delays | Insert synthetic micro‑segments or adjust first cycles after beam_on | 
| Analog jitter | Randomized sub‑pixel perturbation (seeded) for visual authenticity (optional) |
| Velocity saturation | Clamp or ease velocity transitions for hardware realism |

## Status
Legacy backend fully removed (Sept 2025). Update this document whenever merging tolerances, intensity mapping, or beam fidelity features change.
