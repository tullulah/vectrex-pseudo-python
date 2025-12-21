# Verification Checklist: Jetpac Memory Collision Fix

## Quick Verification Commands

### 1. Verify Compiler Changes
```bash
grep -n "LDB \$CF0" /Users/daniel/projects/vectrex-pseudo-python/examples/jetpac/src/main.asm
```
**Expected Output**:
```
925:    LDB $CF00    ; Vec_Joy_1_X (0=left, 128=center, 255=right)
947:    LDB $CF01    ; Vec_Joy_1_Y (0=down, 128=center, 255=up)
```

### 2. Verify Frontend Changes
```bash
grep -n "write8.*CF0" /Users/daniel/projects/vectrex-pseudo-python/ide/frontend/src/components/panels/EmulatorPanel.tsx
```
**Expected Output**:
```
513:        vecx.write8(0xCF00, analogX); // Vec_Joy_1_X
514:        vecx.write8(0xCF01, analogY); // Vec_Joy_1_Y
```

### 3. Verify Compiler Source Code
```bash
grep -n "\$CF00" /Users/daniel/projects/vectrex-pseudo-python/core/src/backend/m6809/builtins.rs
```
**Expected Output**:
```
214:        out.push_str("    LDB $CF00    ; Vec_Joy_1_X (0=left, 128=center, 255=right)\n");
283:        out.push_str("    LDB $CF01    ; Vec_Joy_1_Y (0=down, 128=center, 255=up)\n");
```

### 4. Verify Documentation
```bash
grep "19\. Joystick Input System" /Users/daniel/projects/vectrex-pseudo-python/.github/copilot-instructions.md
```
**Expected Output**: Should find Section 19 with full documentation

## Manual Verification

### Code File Checks

#### File 1: core/src/backend/m6809/builtins.rs
- **Line 214**: Should contain `LDB $CF00` (was `LDB $C81B`)
- **Line 283**: Should contain `LDB $CF01` (was `LDB $C81C`)

#### File 2: ide/frontend/src/components/panels/EmulatorPanel.tsx
- **Line 513**: Should contain `vecx.write8(0xCF00, analogX);`
- **Line 514**: Should contain `vecx.write8(0xCF01, analogY);`
- **Line 507**: Comment should say `$CF00/$CF01` (was `$C81B/$C81C`)

### Generated Binary Checks

#### ASM File 1: examples/jetpac/src/main.asm
- **Line 924-925**: Should show comment and `LDB $CF00`
- **Line 946-947**: Should show comment and `LDB $CF01`

#### ASM File 2: examples/TestController/src/main.asm
- **Line 363-364**: Should show comment and `LDB $CF00`
- **Line 385-386**: Should show comment and `LDB $CF01`

### Binary Files

#### examples/jetpac/src/main.bin
- **Size**: 15276 bytes (unchanged)
- **Timestamp**: Should be recent (just compiled)

#### examples/TestController/src/main.bin
- **Size**: 8192 bytes (unchanged)
- **Timestamp**: Should be recent (just compiled)

## Functional Verification (After Launch)

### Test Procedure in IDE

1. **Start IDE**:
   ```bash
   cd ide && npm start
   ```

2. **Load Jetpac**:
   - Open `examples/jetpac/src/main.bin`
   - Or drag-drop the file

3. **Test Movement**:
   - Push **LEFT** on analog stick
   - Verify: Player moves **LEFT** (not right)
   
   - Push **RIGHT** on analog stick
   - Verify: Player moves **RIGHT** (not left)
   
   - Push **UP** on analog stick
   - Verify: Player moves **UP** (not down)
   
   - Push **DOWN** on analog stick
   - Verify: Player moves **DOWN** (not up)
   
   - Release stick
   - Verify: Player **centers** (joy_x=0, joy_y=0)

4. **Compare with TestController**:
   - Load `examples/TestController/src/main.bin`
   - Verify same joystick behavior
   - Should move correctly in all directions

5. **Verify No Regression**:
   - Music still plays? ✓
   - Vectors render correctly? ✓
   - Menu navigation works? ✓

## Expected Results Summary

| Aspect | Status | Details |
|--------|--------|---------|
| Compiler ASM | ✅ SHOULD USE $CF00/$CF01 | Lines 214, 283 in builtins.rs |
| Frontend Writes | ✅ SHOULD USE 0xCF00/0xCF01 | Lines 513-514 in EmulatorPanel.tsx |
| Generated ASM | ✅ SHOULD SHOW LDB $CF00/01 | Jetpac/TestController main.asm |
| Jetpac Movement | ⏳ PENDING | Left/Right/Up/Down should work |
| No Regression | ⏳ PENDING | Music/vectors/menu still work |

## Troubleshooting

### If Jetpac STILL Reads joy_x=1 Always

**Debug Steps**:
1. Verify ASM actually uses $CF00 (check files above)
2. Check if Frontend is actually writing to $CF00 (add console.log)
3. Verify $CF00/$CF01 are not being overwritten by Jetpac structs
4. Try alternative addresses: $CD00/$CD01 or $CC00/$CC01

**Possible Causes**:
- Cache not cleared (unlikely, but clean build if needed)
- Frontend not rebuilt (run `npm run build` again)
- Addresses not matching (verify both files)
- $CF00/$CF01 also in collision zone (rare, try different addresses)

### If Code Changes Disappeared

**Recovery Steps**:
1. Re-run all changes listed above
2. Verify with grep commands
3. Rebuild compiler and frontend
4. Recompile Jetpac and TestController

## Files Modified (Summary)

```
2 files changed, 8 insertions(+), 8 deletions(-)

- core/src/backend/m6809/builtins.rs
  Line 214: LDB $C81B → LDB $CF00
  Line 283: LDB $C81C → LDB $CF01

- ide/frontend/src/components/panels/EmulatorPanel.tsx
  Line 507: Comment $C81B/$C81C → $CF00/$CF01
  Line 513: vecx.write8(0xC81B, ...) → vecx.write8(0xCF00, ...)
  Line 514: vecx.write8(0xC81C, ...) → vecx.write8(0xCF01, ...)
```

---

**Status**: Ready for testing. All code changes complete, binaries recompiled, documentation updated.
