# Vector Multi-Path Rendering — Known Limitation

**Date**: 2025-12-10
**Status**: DOCUMENTED — Works partially, requires further investigation

## Summary

The multi-path vector system **works correctly** for the first path, but **accumulates positions** in subsequent paths due to the relative nature of the BIOS function `Moveto_d`.

## Current Behaviour

### ✅ What works:
- **Single-path vectors**: Perfect
- **First path of multi-path**: Draws correctly
- **All paths are drawn**: No path disappears
- **Data format**: Correct (FCB y,x for Draw_VLc)

### ⚠️ The limitation:
- **Subsequent paths**: Drawn from accumulated positions instead of absolute ones
- **Example**: In a multi-path `.vec` with a circle and craters, the craters cluster in the wrong position instead of being spread around the circle

## Technical Cause

### BIOS Moveto_d is Relative
```asm
; State after drawing the first path (circle):
; - Beam position: end point of circle (e.g. x=15, y=25)

; Attempting to position the second path (crater at -10, 8):
LDA #8          ; A = desired y (8)
LDB #-10        ; B = desired x (-10)
JSR Moveto_d    ; BUT: Moveto_d adds to current beam!
                ; Result: beam at (15-10, 25+8) = (5, 33) ❌
                ; Expected: beam at (-10, 8) from origin ✅
```

### Current Code (core/src/backend/m6809.rs lines 1385–1405)
```rust
out.push_str("    JSR Reset0Ref       ; Reset integrator origin to center\n");
out.push_str(&format!("    LDX #{}_VECTORS ; Load pointer list\n", symbol));
out.push_str("DRAW_VEC_LOOP_START:\n");
out.push_str("    LDD ,X++            ; Load next path pointer\n");
out.push_str("    BEQ DRAW_VEC_DONE   ; Exit if 0 (end of list)\n");
out.push_str("    PSHS X              ; Save list pointer\n");
out.push_str("    TFR D,X             ; X = path data pointer\n");
out.push_str("    LDA ,X+             ; A = Y0 (starting point)\n");
out.push_str("    LDB ,X+             ; B = X0 (starting point)\n");
out.push_str("    JSR Moveto_d        ; Move beam to starting point\n");
out.push_str("    JSR Draw_VLc        ; Draw this path\n");
out.push_str("    ; TODO: Multi-path positioning needs investigation\n");
out.push_str("    ; Issue: subsequent paths render at accumulated positions\n");
out.push_str("    ; Moveto_d is relative to current beam position\n");
```

## Attempted Solutions (All Failed)

### 1. ❌ Moveto_d_7F (Theoretical Absolute Positioning)
- **Result**: Same incorrect positions
- **Reason**: Moveto_d_7F requires additional setup, or also accumulates

### 2. ❌ Moveto_d_7F + Scale Factor
- **Result**: Same incorrect positions
- **Reason**: Scale factor does not resolve the accumulation

### 3. ❌ Reset0Ref before each path (inside the loop)
- **Result**: Nothing drawn (blank screen)
- **Reason**: Reset0Ref requires integrator stabilisation time; rapid calls in a loop break the internal BIOS state

### 4. ✅ Reset0Ref once + loop Moveto_d (CURRENT)
- **Result**: Draws all paths; first path correct, subsequent paths accumulate
- **Status**: CURRENT IMPLEMENTATION — works partially

## Identified Constraints

1. **Moveto_d is relative**: By BIOS design, it adds to the current beam position
2. **Reset0Ref is timing-sensitive**: Cannot be called in fast loops
3. **Moveto_d_7F insufficient**: Requires deeper understanding of setup
4. **BIOS internals unknown**: Details about integrator and timing are missing

## Workarounds

### A. Use Single-Path Vectors (RECOMMENDED)
- ✅ Works perfectly
- ✅ No positioning limitations
- ❌ Requires designing vectors as single paths (more points per file)

### B. First Path Only
- ✅ The first path of any multi-path works correctly
- ❌ Not useful when multiple separate shapes are needed

### C. Multiple DRAW_VECTOR Calls (RECOMMENDED for complex shapes)
Split a complex multi-path asset into separate single-path `.vec` files and call each individually:
```python
DRAW_VECTOR("shape_outline", 0, 0)
DRAW_VECTOR("shape_detail1", -10, 8)
DRAW_VECTOR("shape_detail2", 8, -5)
```
- ✅ Full positioning control
- ✅ Each part renders perfectly
- ❌ Slightly more draw calls per frame

## Future Investigation

### 1. Calculate Deltas Between Paths
Instead of absolute coordinates in FCB, calculate the delta from the previous path's end point. Requires tracking the end point of each path in the compiler — more complex but potentially solves the problem.

### 2. Manual Integrator Control
Study VIA registers of the integrator for direct control without BIOS functions. Advanced — requires deep hardware knowledge.

### 3. Reset0Ref Timing
Determine how much delay is needed between calls; test with NOP padding in the loop.

### 4. Alternative BIOS Functions
Investigate other `Moveto_*` variants (Moveto_ix, etc.) and study how commercial cartridges handle multiple shapes.

---

**Last updated**: 2025-12-10
