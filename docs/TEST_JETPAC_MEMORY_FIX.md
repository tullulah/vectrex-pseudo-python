# Test: Jetpac Memory Collision Fix

## Summary of Changes

### Problem
- **Jetpac**: Always moves up-right (joy_x=1, joy_y=1)
- **TestController**: Works correctly (moves all 4 directions)
- **Root Cause**: Joystick RAM addresses ($C81B/$C81C) colliding with Jetpac's global struct allocations

### Solution
Changed joystick RAM addresses from collision zone to safe high-RAM location:
- **OLD**: $C81B (Joy_1_X), $C81C (Joy_1_Y)
- **NEW**: $CF00 (Joy_1_X), $CF01 (Joy_1_Y)

### Why $CF00/$CF01 is safe
- Jetpac stack: starts at ~$CFFF, grows downward
- Jetpac work variables: typically use $C800-$CE00 range
- $CF00/$CF01 is between stack and work vars, less likely to be used for globals
- More importantly: Different from the collision zone where structs are allocating

## Files Modified

1. **core/src/backend/m6809/builtins.rs**
   - J1_X: Changed `LDB $C81B` → `LDB $CF00`
   - J1_Y: Changed `LDB $C81C` → `LDB $CF01`

2. **ide/frontend/src/components/panels/EmulatorPanel.tsx**
   - Changed `vecx.write8(0xC81B, ...)` → `vecx.write8(0xCF00, ...)`
   - Changed `vecx.write8(0xC81C, ...)` → `vecx.write8(0xCF01, ...)`

## Compiled Binaries

- ✅ Jetpac: 15276 bytes (recompiled at `examples/jetpac/src/main.bin`)
- ✅ TestController: 1577 bytes padded to 8192 (recompiled at `examples/TestController/src/main.bin`)
- ✅ Frontend: Built with unsigned formula and new addresses
- ✅ Electron IDE: Rebuilt TypeScript

## Verification in ASM

**Jetpac (jetpac.asm, line 925)**:
```asm
    LDB $CF00    ; Vec_Joy_1_X (0=left, 128=center, 255=right)
```

**TestController (main.asm, line 364)**:
```asm
    LDB $CF00    ; Vec_Joy_1_X (0=left, 128=center, 255=right)
```

## Test Procedure

1. Launch IDE: `npm start` from ide/
2. Load Jetpac ROM: `examples/jetpac/src/main.bin`
3. Test joystick movement:
   - Left/Right should move left/right (not stuck right)
   - Up/Down should move up/down (not stuck up)
   - Release should center (joy_x=0, joy_y=0)
4. Compare with TestController for reference behavior
5. Verify music/vectors still play (no regression from performance optimization)

## Expected Behavior After Fix

- ✅ Jetpac moves left/right when analog stick left/right
- ✅ Jetpac moves up/down when analog stick up/down  
- ✅ Jetpac centers when stick is neutral
- ✅ No "stuck up-right" behavior

## Technical Notes

- Frontend still uses unsigned 0-255 range with 128=center (unchanged)
- Formula: `Math.round((x + 1) * 127.5)` for range -1 to +1 (unchanged)
- Thresholds: <108=direction(-1), 108-148=center(0), >148=direction(+1) (unchanged)
- Only the target RAM address changed: $C81B/$C81C → $CF00/$CF01

## If This Doesn't Fix It

If Jetpac still shows stuck behavior, next steps:
1. Check if $CF00/$CF01 is also being used by Jetpac structs
2. Try $CD00/$CD01 (even further into stack area)
3. Instrument memory to find what IS overwriting $CF00/$CF01
4. Consider using JSVecx's `alg_jch0`/`alg_jch1` properties directly (would require WASM interface)
