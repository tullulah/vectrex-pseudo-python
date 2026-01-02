# Quick Fix Summary: Jetpac Memory Collision

## What Was Fixed
Jetpac always read joy_x=1, joy_y=1 (stuck up-right) due to RAM collision at $C81B/$C81C.

## How It Was Fixed
Changed joystick input RAM addresses to collision-free zone:
```
$C81B → $CF00  (Joy_1_X)
$C81C → $CF01  (Joy_1_Y)
```

## Files Changed
1. `core/src/backend/m6809/builtins.rs` - Compiler generates `LDB $CF00/$CF01`
2. `ide/frontend/src/components/panels/EmulatorPanel.tsx` - Frontend writes to `0xCF00`/`0xCF01`

## Status
- ✅ All files updated
- ✅ All binaries recompiled
- ✅ ASM verified correct (LDB $CF00 and LDB $CF01 present)
- ⏳ **Awaiting Test**: Jetpac movement in all 4 directions

## To Test
1. `npm start` from ide/
2. Load `examples/jetpac/src/main.bin`
3. Move analog stick in all directions
4. Verify: Left/Right/Up/Down work correctly (not stuck up-right)
