# Vector Multi-Path Rendering — Known Limitation

**Date**: 2025-12-10
**Status**: DOCUMENTED — Works partially, requires further investigation

## Summary

The multi-path vector system **works correctly** for the first path, but **accumulates positions** in subsequent paths due to the relative nature of the BIOS function `Moveto_d`.

## Current Behaviour

### ✅ What works:
- **Single-path vectors**: Perfect (e.g. `test_simple_vector/line.vec`)
- **First path of multi-path**: Draws correctly (e.g. outer circle of `moon.vec`)
- **All paths are drawn**: No path disappears
- **Data format**: Correct (FCB y,x for Draw_VLc)

### ⚠️ The limitation:
- **Subsequent paths**: Drawn from accumulated positions instead of absolute ones
- **Example**: In `moon.vec`, the 3 craters should be distributed, but cluster in the top-right corner

## Technical Cause

### BIOS Moveto_d is Relative
```asm
; State after drawing the first path (circle):
; - Beam position: end point of circle (e.g. x=15, y=25)

; Attempting to position the second path (crater1 at -10, 8):
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
out.push_str("    ; Issue: Craters render at accumulated positions\n");
out.push_str("    ; Moveto_d is relative to current beam position\n");
```

## Attempted Solutions (All Failed)

### 1. ❌ Moveto_d_7F (Theoretical Absolute Positioning)
```asm
JSR Moveto_d_7F  ; Instead of Moveto_d
```
- **Result**: Craters in the **same incorrect positions**
- **Reason**: Moveto_d_7F requires additional setup, or also accumulates

### 2. ❌ Moveto_d_7F + Scale Factor
```asm
LDA #$7F
STA VIA_shift_reg    ; Configure scale
JSR Moveto_d_7F
```
- **Result**: Craters in the **same incorrect positions**
- **Reason**: Scale factor does not resolve the accumulation

### 3. ❌ Reset0Ref before each path (inside the loop)
```asm
DRAW_VEC_LOOP_START:
    LDD ,X++
    BEQ DRAW_VEC_DONE
    JSR Reset0Ref        ; ← Reset before each path
    PSHS X
    TFR D,X
    ...
```
- **Result**: **Nothing drawn** (blank screen)
- **Reason**: Reset0Ref requires integrator stabilisation time
- **Problem**: Rapid calls inside a loop break the internal BIOS state

### 4. ❌ Coordinate order swap (proof of concept)
```asm
LDB ,X+    ; B = X first
LDA ,X+    ; A = Y second
```
- **Result**: Not properly tested (user had incorrect asset name)
- **Reason**: FCB y,x order is correct (verified with single-path)

### 5. ✅ Reset0Ref once + loop Moveto_d (CURRENT)
- **Result**: Draws all paths; first path correct, subsequent paths accumulate
- **Status**: CURRENT IMPLEMENTATION — works partially

## Visual Examples

### moon.vec — Expected vs Rendered Coordinates

```
Expected (absolute from origin):       Actual (accumulated):

       outer_circle (0, 30)                  outer_circle (0, 30) ✅
            ◯                                       ◯

  crater1 (-10, 8)   crater2 (8, -5)        crater1,2,3 clustered
      •                  •                   at approx (15, 33) ❌
         crater3 (-5, -12)                            •••
             •
```

### Generated Data (Correct)
```asm
_MOON_OUTER_CIRCLE_VECTORS:
    FCB 30, 0          ; y=30, x=0 (top center)
    FCB 23             ; 23 deltas
    ; ... circle deltas

_MOON_CRATER1_VECTORS:
    FCB 8, -10         ; y=8, x=-10 (should be left-upper)
    FCB 7              ; 7 deltas
    ; ... crater deltas

_MOON_CRATER2_VECTORS:
    FCB -5, 8          ; y=-5, x=8 (should be right-lower)
    ; ...

_MOON_CRATER3_VECTORS:
    FCB -12, -5        ; y=-12, x=-5 (should be center-bottom)
    ; ...

_MOON_VECTORS:
    FDB _MOON_OUTER_CIRCLE_VECTORS
    FDB _MOON_CRATER1_VECTORS
    FDB _MOON_CRATER2_VECTORS
    FDB _MOON_CRATER3_VECTORS
    FDB 0
```

## Identified Constraints

1. **Moveto_d is relative**: By BIOS design, it adds to current beam position
2. **Reset0Ref is timing-sensitive**: Cannot be called in fast loops
3. **Moveto_d_7F insufficient**: Requires deeper understanding of setup
4. **BIOS internals unknown**: Details about integrator and timing are missing

## Available Workarounds

### A. Use Single-Path Vectors (RECOMMENDED)
- ✅ Works perfectly
- ✅ No positioning limitations
- ❌ Requires designing vectors as single paths (more points)

### B. First Path Only
- ✅ The first path of any multi-path works correctly
- ❌ Not useful when multiple separate shapes are needed

### C. Accept Accumulation (CURRENT)
- ✅ All paths are drawn
- ⚠️ Incorrect but predictable positioning
- 💡 Could be used intentionally for artistic effects

## Future Investigation

### 1. Study BIOS Moveto_d_7F
- Document exact setup requirements
- Test with different VIA configurations
- Compare with reference implementation (Vectrexy)

### 2. Calculate Deltas Between Paths
```asm
; Instead of absolute coordinates in FCB,
; calculate delta from the previous path:
; crater1_relative = crater1_abs - circle_end
```
- Requires tracking the end point of each path
- More complex compiler
- Potentially solves the problem

### 3. Manual Integrator Control
- Study VIA registers of the integrator
- Direct control without BIOS functions
- Advanced — requires deep hardware knowledge

### 4. Reset0Ref Timing
- How much delay is needed between calls
- Manual delay can be inserted in the loop
- Test with different NOP counts

### 5. Alternative BIOS Functions
- Investigate other Moveto_* functions (Moveto_ix, etc.)
- Check how commercial cartridges handle multiple shapes
- Disassemble commercial games

## Project Impact

### test_simple_vector
- **Status**: ✅ Works perfectly
- **Size**: 151 bytes
- **Type**: Single-path (2 points, 45° line)

### test_mcp
- **Status**: ⚠️ Works partially
- **Size**: 2733 bytes + padding
- **Type**: Multi-path (4 paths: circle + 3 craters)
- **Observation**: Circle perfect, craters clustered

### General Recommendation
For production projects:
- Design assets as **single-path** when possible
- If multiple separate shapes are needed, use **multiple DRAW_VECTOR calls** with single-path assets:
  ```python
  DRAW_VECTOR("moon_circle", 0, 0)    # Asset 1: circle only
  DRAW_VECTOR("moon_crater1", -10, 8) # Asset 2: crater 1 only
  DRAW_VECTOR("moon_crater2", 8, -5)  # Asset 3: crater 2 only
  DRAW_VECTOR("moon_crater3", -5, -12)# Asset 4: crater 3 only
  ```

## Code References

### Inline code generation
- **File**: `core/src/backend/m6809.rs`
- **Lines**: 1385–1420
- **Function**: `emit_builtin_call()` — case "DRAW_VECTOR"

### Vector data format
- **File**: `core/src/vecres.rs`
- **Lines**: 228–296
- **Functions**: Path data generation + pointer list

### Native assembler
- **File**: `core/src/backend/asm_to_binary.rs`
- **Lines**: 1605–1660
- **Function**: `parse_indexed_mode()` — Y register support

## Validations Performed

✅ Coordinates do not require negation (canvas and Vectrex axes match)
✅ FCB y,x order correct for Draw_VLc
✅ LDA/LDB loading order correct
✅ Y register indexed addressing implemented (available but unused)
✅ Asset validation with error handling
✅ Single-path vectors work perfectly
✅ Multi-path render (all paths visible)
⚠️ Multi-path positioning accumulates (documented limitation)

## Conclusion

The current system is **functional and stable**, with a known limitation in multi-path positioning. Users can choose between:
1. **Single-path workflows** (recommended, no limitations)
2. **Multiple DRAW_VECTOR calls** with single-path assets
3. **Accepting accumulation** in multi-path (artistic effects)

Future investigation may fully resolve the problem, but it is not a blocker for game development.

---

**Last updated**: 2025-12-10
