# Pang Hook Rope X Coordinate Bug - Root Cause Analysis and Fix

**Status**: ✅ FIXED (Session 2025-12-31)  
**Binary Size**: 23433 bytes (53 bytes saved from parameter handling optimization)

## Problem Statement

The hook rope in Pang was drawing from an incorrect X coordinate. Despite the function being called with:
- `start_x = 11` (hook_gun_x)
- `end_x = 11` (hook_x)

The rope was being drawn from X≈0 or with severe offset, appearing to aim toward the center rather than the gun position.

## Root Causes Identified and Fixed

### Bug #1: Parameter Stack Layout (Initial Investigation)
**Status**: Fixed but not the primary cause

**Problem**: Parameters were allocated in the stack based on their position in the `locals` list rather than their parameter order.

**Example**: If locals = [start_y, start_x, end_y, end_x]:
- Parameter 0 (start_x) would get offset 4,S instead of 0,S
- Parameter 1 (start_y) would get offset 6,S instead of 2,S

**Fix**: Changed to sequential allocation: `offset = param_index * 2`
- Now all parameters get correct stack offsets (0,S, 2,S, 4,S, 6,S)

**File**: `core/src/backend/m6809/mod.rs` lines 45-51

---

### Bug #2: Parameter Offset Calculation (Secondary Investigation)
**Status**: Fixed but not the primary cause

**Problem**: `offset_of()` function would search for variables in the `locals` list position, which was wrong for parameters accessed from within functions.

**Example**: When accessing parameter "start_x" from within a function, if parameter wasn't at position 0 in `locals`, it would return wrong offset.

**Fix**: Modified `offset_of()` to:
1. First check if variable is in `f.params` (parameter list)
2. Return fixed position: `param_index * 2` for parameters
3. Only calculate offset for non-parameters starting after parameter space

**Added**: `params: Vec<String>` field to FuncCtx to track function parameters

**Files Modified**:
- `core/src/backend/m6809/utils.rs` lines 279-315
- `core/src/backend/m6809/mod.rs` (4 FuncCtx initializers)

---

### Bug #3: DRAW_LINE Parameter Handling (Key Fix #1)
**Status**: ✅ FIXED (Primary issue found during investigation)

**Problem**: When passing non-constant arguments to DRAW_LINE, the code was:
```rust
emit_expr(arg, ...);          // Writes result to RESULT
out.push_str("LDD RESULT\n"); // Load from RESULT (redundant)
out.push_str("STD RESULT+offset\n"); // Store to final location
```

This created redundant reads and, more critically, when the SECOND argument was processed:
```asm
; Param 0
LDD 0,S
STD RESULT      ; Write param 0 to RESULT
LDD RESULT      ; Load param 0
STD RESULT+0    ; Store to RESULT+0 ✓

; Param 1
LDD 2,S
STD RESULT      ; ← OVERWRITES RESULT (which is same as RESULT+0!)
LDD RESULT      ; Load param 1 (correct)
STD RESULT+2    ; Store to RESULT+2 ✓
```

**Fix**: Removed the redundant `LDD RESULT` instruction. Since `emit_expr` returns the value in both RESULT and D register, we only need:
```rust
emit_expr(arg, ...);
out.push_str(&format!("STD RESULT+{}\n", offset)); // Direct store from D
```

**Result**: Value is stored directly from D register, avoiding the reload and accidental overwrite.

**File**: `core/src/backend/m6809/builtins.rs` lines 786-810

---

### Bug #4: DRAW_LINE_WRAPPER Moveto_d Coordinate Loading (ROOT CAUSE)
**Status**: ✅ FIXED (This was the actual visual bug!)

**Problem**: DRAW_LINE_WRAPPER was loading HIGH BYTES instead of LOW BYTES for coordinates:

```asm
; ❌ WRONG - Loading high bytes (sign extension)
LDA RESULT+2+1  ; = RESULT+3 (high byte of Y)
LDB RESULT+0+1  ; = RESULT+1 (high byte of X)

; ✅ CORRECT - Loading low bytes (actual coordinate values)
LDA RESULT+2    ; = Low byte of Y coordinate
LDB RESULT+0    ; = Low byte of X coordinate
```

**16-bit Value Memory Layout**:
```
For a 16-bit signed value stored at RESULT+0:
  RESULT+0 = Low byte (actual 8-bit signed value for Vectrex)
  RESULT+1 = High byte (sign extension, 0x00 for positive, 0xFF for negative)
```

**Example - start_x = 11 (0x000B)**:
```
RESULT+0 = 0x0B (low byte = 11)
RESULT+1 = 0x00 (high byte = 0)

❌ OLD CODE: LDB RESULT+0+1 = LDB RESULT+1 → loads 0x00 (zero, wrong!)
✅ NEW CODE: LDB RESULT+0 → loads 0x0B (11, correct!)
```

**Why This Caused the Bug**:
1. Parameter start_x = 11 correctly stored at RESULT+0
2. DRAW_LINE_WRAPPER read RESULT+1 (=0x00) instead of RESULT+0 (=0x0B)
3. Moveto_d received B=0x00 instead of B=0x0B
4. Rope drawn from X=0 instead of X=11

**Fix**: Load low bytes for coordinates:
```asm
; INTENSITY (byte value)
LDA RESULT+8    ; ← Low byte of intensity

; COORDINATES (8-bit signed -127..+127 for Vectrex)
LDA RESULT+2    ; ← Y coordinate (low byte)
LDB RESULT+0    ; ← X coordinate (low byte)
```

**File**: `core/src/backend/m6809/emission.rs` lines 260-271

---

## Technical Details

### DRAW_LINE_WRAPPER Argument Layout
```
RESULT+0 = x0 (start X coordinate, 16-bit signed)
RESULT+1 = x0 high byte
RESULT+2 = y0 (start Y coordinate, 16-bit signed)
RESULT+3 = y0 high byte
RESULT+4 = x1 (end X coordinate, 16-bit signed)
RESULT+5 = x1 high byte
RESULT+6 = y1 (end Y coordinate, 16-bit signed)
RESULT+7 = y1 high byte
RESULT+8 = intensity (16-bit value)
RESULT+9 = intensity high byte
```

### Vectrex Coordinate System
- Coordinates are **8-bit signed** values (-127 to +127)
- Moveto_d expects: A = Y (signed), B = X (signed)
- When loading from 16-bit storage, use **low byte** (actual coordinate value)
- High byte is sign extension: 0x00 for positive, 0xFF for negative

### Bug #4 vs Earlier Bugs
- **Bugs #1-3**: Parameter passing issues (found during investigation)
- **Bug #4**: Coordinate byte selection in DRAW_LINE_WRAPPER (ROOT CAUSE of visual bug)

Bugs #1-3 may not have affected this specific case because:
- Parameters were eventually reaching the right locations
- But they would have caused problems in other functions

Bug #4 was the actual cause of the rope drawing from wrong position.

---

## Testing

**Before Fix**:
- Hook rope drew from wrong X position (toward center)
- Parameter values correct at stack level
- DRAW_LINE_WRAPPER received correct values but misread them

**After Fix**:
- Hook rope draws from correct position (X=11, same as gun)
- DRAW_LINE_WRAPPER correctly loads 8-bit coordinate values
- Pang compiles to 23433 bytes (53 bytes saved)

---

## Files Changed

1. **core/src/backend/m6809/builtins.rs**
   - Lines 786-810: Removed redundant `LDD RESULT` in DRAW_LINE parameter handling

2. **core/src/backend/m6809/emission.rs**
   - Lines 260-271: Fixed Moveto_d call to load low bytes instead of high bytes
   - Changed from `RESULT+0+1` (high byte) to `RESULT+0` (low byte)
   - Changed from `RESULT+2+1` (high byte) to `RESULT+2` (low byte)
   - Changed from `RESULT+8+1` (high byte) to `RESULT+8` (low byte for intensity)

3. **core/src/backend/m6809/utils.rs**
   - Lines 279-315: Added parameter tracking to offset_of() function

4. **core/src/backend/m6809/mod.rs**
   - Lines 45-51: Fixed parameter stack layout calculation
   - 4 FuncCtx initializer updates

---

## Lessons Learned

1. **16-bit Value Storage**: When using 16-bit values for 8-bit hardware operations, always load the correct byte (low byte for actual values, high byte for sign extension)

2. **Parameter Handling**: Complex parameter allocation logic should be simplified and verified carefully

3. **Intermediate Values**: Using the same location (RESULT) for both input and output can cause overwrites - either use different locations or load the value immediately

4. **Code Review**: The double `STD` instruction should have been noticed earlier as a code smell (redundant write)

---

## Conclusion

The hook rope X coordinate bug was caused by **Bug #4: Loading the wrong bytes (high bytes instead of low bytes) from the coordinate storage in DRAW_LINE_WRAPPER**. While investigating this issue, we also found and fixed **Bugs #1-3** which were related to parameter handling and could have caused issues in other scenarios.

The root cause was a misunderstanding of how 16-bit values are stored in memory and which byte needs to be loaded for 8-bit hardware operations on the Vectrex.
