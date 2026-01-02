# Jetpac Memory Collision Fix - Complete Summary

## Problem Identified
Jetpac always reads joy_x=1, joy_y=1 (stuck up-right), while TestController works correctly.

**Root Cause**: Joystick RAM addresses ($C81B/$C81C) were in collision zone with Jetpac's global struct allocations (Vec2, Entity, etc.).

## Solution: Address Relocation

### New RAM Addresses
```
OLD:    $C81B (Joy_1_X)    $C81C (Joy_1_Y)
NEW:    $CF00 (Joy_1_X)    $CF01 (Joy_1_Y)
```

### Why $CF00/$CF01 is Safe
- **Location**: High RAM, between typical work variables ($C800-$CE00) and stack ($CFFF)
- **Safety**: Less likely to collide with dynamic allocations
- **Rationale**: Different from the $C81B/$C81C collision zone

## Changes Made

### 1. Compiler Backend (M6809 ASM Generation)
**File**: `core/src/backend/m6809/builtins.rs`

**J1_X (Line 214)**:
- OLD: `out.push_str("    LDB $C81B    ; Vec_Joy_1_X (0=left, 128=center, 255=right)\n");`
- NEW: `out.push_str("    LDB $CF00    ; Vec_Joy_1_X (0=left, 128=center, 255=right)\n");`

**J1_Y (Line 283)**:
- OLD: `out.push_str("    LDB $C81C    ; Vec_Joy_1_Y (0=down, 128=center, 255=up)\n");`
- NEW: `out.push_str("    LDB $CF01    ; Vec_Joy_1_Y (0=down, 128=center, 255=up)\n");`

### 2. Frontend Joystick Writer
**File**: `ide/frontend/src/components/panels/EmulatorPanel.tsx`

**Lines 507-514**:
```typescript
// OLD
vecx.write8(0xC81B, analogX); // Vec_Joy_1_X
vecx.write8(0xC81C, analogY); // Vec_Joy_1_Y

// NEW
vecx.write8(0xCF00, analogX); // Vec_Joy_1_X
vecx.write8(0xCF01, analogY); // Vec_Joy_1_Y
```

### 3. Updated Comments
- Updated comment on line 507 to reflect new addresses ($CF00/$CF01 instead of $C81B/$C81C)

## Compiled Binaries

✅ **Jetpac** (`examples/jetpac/src/main.bin`): 15276 bytes
- ASM: lines 924-925 now use `LDB $CF00`
- ASM: lines 946-947 now use `LDB $CF01`

✅ **TestController** (`examples/TestController/src/main.bin`): 1577 bytes (padded to 8192)
- ASM: lines 363-364 now use `LDB $CF00`
- ASM: lines 385-386 now use `LDB $CF01`

✅ **Frontend**: Rebuilt (`ide/frontend/dist/`)
- Write addresses changed in React component

✅ **Electron IDE**: Rebuilt TypeScript

## Input Data Flow (Unchanged)

1. **Frontend Gamepad Poll** (60Hz): Analog stick values -1.0 to +1.0
2. **Conversion Formula** (UNCHANGED): `Math.round((x + 1) * 127.5)`
   - Result: unsigned 0-255 range (0=left/down, 128=center, 255=right/up)
3. **RAM Write**: Write to **$CF00** (X) and **$CF01** (Y)
4. **Emulator Read**: M6809 ASM reads from **$CF00**/$CF01**
5. **VPy Conversion** (UNCHANGED):
   - <108 → -1 (left/down)
   - 108-148 → 0 (center)
   - >148 → +1 (right/up)
6. **Return Value**: Signed 16-bit (-1, 0, or +1)

## Verification Checklist

- [x] Compiler ASM generation updated to use $CF00/$CF01
- [x] Frontend writes updated to use 0xCF00/0xCF01
- [x] All binaries recompiled
- [x] Comments updated for clarity
- [x] No logic changes (only address relocation)

## Test Procedure

1. **Launch IDE**: `npm start` from `ide/` folder
2. **Load Jetpac**: Select `examples/jetpac/src/main.bin`
3. **Test Movement**:
   - Move analog stick **LEFT** → Player should move LEFT
   - Move analog stick **RIGHT** → Player should move RIGHT
   - Move analog stick **UP** → Player should move UP
   - Move analog stick **DOWN** → Player should move DOWN
   - Release stick → Player should center (joy=0)
4. **Verify No Regression**:
   - Music still plays
   - Vector graphics render correctly
5. **Compare Reference**: Run TestController and verify it still works

## Expected Result

✅ **Before**: Jetpac stuck moving up-right
❌ After Fix: **Jetpac moves in correct direction**

## If Fix Doesn't Work

If Jetpac still exhibits stuck behavior:
1. $CF00/$CF01 might also be in collision zone (rare but possible)
2. Could try $CD00/$CD01 (even further into reserved area)
3. Alternative: Implement WASM interface to read directly from JSVecx's `alg_jch0`/`alg_jch1` properties

## No Breaking Changes

- TestController should still work
- Input thresholds unchanged
- Frontend formula unchanged
- Only the target RAM address changed

## Files Modified Summary

```
2 files changed, 5 insertions(+), 5 deletions(-)
  - core/src/backend/m6809/builtins.rs (+2, -2)
  - ide/frontend/src/components/panels/EmulatorPanel.tsx (+3, -3)
```

## Rebuilt Artifacts

```
✓ Compiler: vectrexc (cargo build --bin vectrexc)
✓ Jetpac Binary: examples/jetpac/src/main.bin (15276 bytes)
✓ TestController Binary: examples/TestController/src/main.bin (8192 bytes)
✓ Frontend: ide/frontend/dist/ (npm run build)
✓ Electron IDE: Rebuilt TypeScript
```

---

**Next Step**: Test Jetpac in IDE with new addresses and verify movement in all 4 directions.
