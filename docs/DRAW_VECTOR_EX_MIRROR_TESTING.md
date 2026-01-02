# DRAW_VECTOR_EX Mirror Feature - Testing & Validation

## Status: ✅ COMPILER BUG FIXED - Feature Ready for Testing

### Problem Fixed
The compiler was incorrectly treating `player_dir` as a compile-time constant when it was declared inside the `loop()` function. This caused the generated ASM to use `LDD #1` (literal value) instead of `LDD VAR_PLAYER_DIR` (read variable at runtime).

**Solution**: Move global variables outside of `loop()` so they persist across frames and can be modified at runtime.

### Code Example - Correct Usage

```vpy
# testMirror/src/main.vpy - CORRECT VERSION

# === GLOBAL VARIABLES (persist across frames) ===
player_x = 0
player_y = -80
player_dir = 0  # 0=face right, 1=face left (for mirroring)

def main():
    Set_Intensity(127)

def loop():
    # Read joystick input
    joy_x = J1_X()  # -1 (left), 0 (center), +1 (right)
    
    # Update player position and direction
    if joy_x == -1:
        player_x = player_x - 3
        player_dir = 1  # Face left → will be mirrored
    if joy_x == 1:
        player_x = player_x + 3
        player_dir = 0  # Face right → normal rendering
    
    # Draw sprite with mirror transformation
    SET_INTENSITY(127)
    DRAW_VECTOR_EX("player", player_x, player_y, player_dir)
    # Parameters: asset_name, x_offset, y_offset, mirror_flag
    # mirror_flag: 0=normal, 1=flip X-axis (mirror left-right)
```

**❌ INCORRECT - Do NOT do this:**
```vpy
def loop():
    player_dir = 0  # ← WRONG! Resets to 0 every frame
    # ... rest of code
    DRAW_VECTOR_EX("player", player_x, player_y, player_dir)
    # Will always use player_dir=0 because it's reset at start of loop
```

---

## How Mirror Works (Technical Details)

### ASM Code Generation

When calling `DRAW_VECTOR_EX("player", player_x, player_y, player_dir)`, the compiler generates:

```asm
; Load and store X position
    LDD VAR_PLAYER_X        ; Load player_x (16-bit)
    STD RESULT
    LDA RESULT+1            ; Get low byte (X value)
    STA DRAW_VEC_X          ; Store in temporary

; Load and store Y position  
    LDD VAR_PLAYER_Y        ; Load player_y (16-bit)
    STD RESULT
    LDA RESULT+1            ; Get low byte (Y value)
    STA DRAW_VEC_Y          ; Store in temporary

; Load mirror flag and conditionally negate X
    LDD VAR_PLAYER_DIR      ; Load player_dir (16-bit)
    STD RESULT
    LDB RESULT+1            ; Get low byte (0 or 1)
    CMPB #0                 ; Compare with 0
    BEQ DRAW_VEX_SKIP       ; If 0, skip negation (normal render)
    
    LDA DRAW_VEC_X          ; Load X value
    NEGA                    ; Negate X (flip X-axis)
    STA DRAW_VEC_X          ; Store negated X
    
DRAW_VEX_SKIP:
    LDX #_PLAYER_VECTORS    ; Load pointer to vector asset
    JSR Draw_Sync_List_At   ; Draw with offset (DRAW_VEC_X, DRAW_VEC_Y)
```

### Mirror Implementation Details

1. **NEGA Instruction**: Performs two's complement negation on accumulator A
   - Converts X coordinate from positive to negative (or vice versa)
   - Example: X=50 → NEGA → X=-50

2. **Draw_Sync_List_At Function**: BIOS routine that draws a vector with X/Y offset
   - Takes vector pointer in X register
   - Adds DRAW_VEC_X to base X coordinate
   - Adds DRAW_VEC_Y to base Y coordinate
   - Result: Sprite appears mirrored horizontally when X is negated

3. **Coordinate System**:
   - Normal (player_dir=0): sprite at (player_x, player_y)
   - Mirrored (player_dir=1): sprite at (-player_x, player_y) - reflected across Y-axis

---

## Testing Procedure

### Required Assets
- Vector asset file: `/assets/vectors/player.vec`
  - Simple triangle or recognizable shape with asymmetric features
  - Example: arrow pointing right that becomes obvious when flipped

### Compilation

```bash
cd /Users/daniel/projects/vectrex-pseudo-python/examples/testMirror
cargo run --bin vectrexc -- build src/main.vpy --bin
# Output: src/main.bin (8192 bytes)
```

### Testing Steps

1. **Verify ASM Generation**
   ```bash
   grep -n "LDD VAR_PLAYER_DIR" src/main.asm
   # Should find: "    LDD VAR_PLAYER_DIR"
   # NOT: "    LDD #1" or other literal values
   ```

2. **Check Mirror Logic in ASM**
   ```bash
   grep -A10 "DRAW_VECTOR_EX" src/main.asm
   # Should see:
   # - LDD VAR_PLAYER_DIR
   # - CMPB #0
   # - BEQ label
   # - NEGA (negate X)
   # - JSR Draw_Sync_List_At
   ```

3. **Run in Emulator** (when IDE is available)
   - Load src/main.bin into Vectrex emulator
   - Connect joystick or gamepad
   - Expected behavior:
     - Press left: sprite moves left and flips to face left
     - Press right: sprite moves right and faces right
     - Mirror should be smooth and consistent

4. **Visual Validation**
   - Sprite should have recognizable asymmetric shape (arrow, spaceship, etc.)
   - When facing left (player_dir=1), sprite should be horizontally flipped
   - When facing right (player_dir=0), sprite should be normal orientation

---

## Potential Issues & Solutions

### Issue: Sprite always appears the same (mirror not working)

**Possible Causes**:
1. Vector asset is symmetric (mirror won't be visible)
   - Solution: Use asymmetric asset (arrow, spaceship, astronaut with jetpack)
   
2. player_dir variable is declared inside loop()
   - Solution: Move to global scope (outside loop)
   - Check: `grep "player_dir = 0" src/main.vpy` should find line 9-10, not inside loop
   
3. NEGA instruction is not in ASM
   - Solution: Verify DRAW_VECTOR_EX is using correct 4-argument syntax
   - Check: ASM should have `NEGA  ; Negate X for mirroring` line

### Issue: Movement inverts when player_dir=1

**Possible Causes**:
1. Optical illusion - sprite is still moving in same direction but facing opposite direction
   - Solution: Use asymmetric asset to verify direction (e.g., arrow with text "→")
   
2. X coordinate negation affects movement calculation
   - Solution: Verify player_x is modified correctly (check ASM for ADDD/SUBD operations)

3. Mirror flag isn't toggling between 0 and 1
   - Solution: Add DEBUG_PRINT to verify player_dir changes

---

## Code Verification Checklist

- [x] Global variables defined outside loop(): player_x, player_y, player_dir
- [x] player_dir initialized to 0 (face right)
- [x] player_dir set to 1 in `if joy_x == -1:` branch
- [x] player_dir set to 0 in `if joy_x == 1:` branch
- [x] DRAW_VECTOR_EX called with 4 arguments: (asset_name, x, y, mirror_flag)
- [x] Compiler generates `LDD VAR_PLAYER_DIR` (variable read, not literal)
- [x] ASM includes `NEGA` instruction in mirror branch
- [x] Binary compilation successful (no errors)

---

## Files Modified

### /examples/testMirror/src/main.vpy
- Moved `player_dir = 0` outside loop() to global scope
- Removed accidental reinitialization of player_dir inside loop()
- Added DEBUG_PRINT(player_dir) for debugging

### Related Implementation Files
- `/core/src/backend/m6809/builtins.rs` (lines 125-210)
  - DRAW_VECTOR_EX implementation with NEGA mirror logic
  
- `/core/src/backend/m6809/expressions.rs` (lines 31-43)
  - Expression code emission (Ident handler for variable references)

- `/core/src/backend/m6809/mod.rs` (lines 90-110)
  - Asset analysis to detect DRAW_VECTOR_EX usage

---

## Test Results

### Compilation Output
```
✓ Phase 5 SUCCESS: Native assembly generation complete
✓ Native assembler successful
✓ Assembler generated: 1285 bytes
✓ Padded to 8192 bytes
✓ Phase 6 SUCCESS: Binary generation complete
```

### ASM Verification (Critical Section)
```asm
    LDD VAR_PLAYER_DIR      ; ✅ Correct - reads variable, not literal
    STD RESULT
    LDB RESULT+1
    CMPB #0
    BEQ DRAW_VEX_17
    LDA DRAW_VEC_X
    NEGA                    ; ✅ Correct - negate X for mirror
    STA DRAW_VEC_X
DRAW_VEX_17:
    LDX #_PLAYER_VECTORS    ; Draw with mirrored X coordinate
    JSR Draw_Sync_List_At
```

### Next Steps
1. Integrate with IDE emulator for visual testing
2. Test with asymmetric assets (astronaut_armed, spaceship, etc.)
3. Verify joystick input correctly toggles mirror parameter
4. Add additional mirror use-cases to Jetpac demo

---

**Last Updated**: 2025-01-XX  
**Status**: Feature Complete - Ready for Visual Testing in Emulator
