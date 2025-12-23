# Fixes: PRINT_TEXT + Vector Drawing Issues

## Problem 1: PRINT_TEXT breaks subsequent vector drawing

### Symptom
After calling `PRINT_TEXT(x, y, "text")`, any subsequent `DRAW_VECTOR()` calls would not render.

### Root Cause
The `VECTREX_PRINT_TEXT` function was:
1. Setting `DP=$D0` (Direct Page) for BIOS text rendering
2. Calling `Print_Str_d` (BIOS text drawing function)
3. **Restoring `DP=$C8`** before returning

The problem: Vector drawing (and all BIOS calls) require `DP=$D0` to access Vectrex hardware registers. By restoring `DP=$C8`, subsequent drawing operations would fail silently.

### Solution
Remove the `DP` restoration. Keep `DP=$D0` after `PRINT_TEXT` returns, since:
- All Vectrex graphics operations need `DP=$D0`
- The main loop (WAIT_RECAL) sets `DP=$D0` before entering `loop()`
- Vector drawing, text drawing, and music updates all require `DP=$D0`

**Commit**: Fix in `core/src/backend/m6809/emission.rs` - removed `TFR A,DP` restoration lines after `Print_Str_d` call.

---

## Problem 2: Vector rectangles don't close all edges

### Symptom
When drawing a `closed: true` path that forms a rectangle, only 3 edges appear. The closing edge (returning to first point) is missing.

### Root Cause
The vector path termination was not properly completing the closed path before the `FCB 2` (end marker).

### Solution
Enhanced path termination logic in `core/src/vecres.rs`:
- Ensure `FCB $FF, dy, dx` is emitted for the closing line
- Verify that `closed: true` paths always emit the return-to-start delta
- Proper `FCB 2` terminator after all lines

### Testing
Created test cases:
- `test_rectangle.vec` - Simple 4-point closed rectangle
- `test_rectangle_draw.vpy` - Program to render it
- `test_print_text_vectors.vpy` - Mixed PRINT_TEXT + DRAW_VECTOR test

### Format Details
Vector path format (Malban Draw_Sync_List):
```
FCB intensity        ; 0-127 brightness
FCB y0, x0, 0, 0    ; Starting position (relative to center)
FCB $FF, dy0, dx0   ; Line 1: flag=-1 (draw), delta Y, delta X
FCB $FF, dy1, dx1   ; Line 2
...
FCB $FF, dyn, dxn   ; Closing line (if closed=true)
FCB 2               ; End marker
```

---

## Validation

### Before Fix
```
PRINT_TEXT("Test")   ; Text appears, DP now $C8 (WRONG)
DRAW_VECTOR(...)     ; Doesn't render (DP is $C8, not $D0)
```

### After Fix
```
PRINT_TEXT("Test")   ; Text appears, DP stays $D0
DRAW_VECTOR(...)     ; Renders correctly (DP is $D0)
Rectangle paths      ; All 4 edges visible, closed properly
```

---

## Files Modified
1. `core/src/backend/m6809/emission.rs` - PRINT_TEXT DP management
2. `core/src/vecres.rs` - Vector path closure (verification/improvement)

## Branch
`feature/fix-print-text-vectors`
