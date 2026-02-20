# Vectrex Memory Map — VPy Compiler

This document describes all memory addresses used by the VPy compiler and their relationship to real Vectrex hardware.

## 1. Vectrex Hardware Memory Map (Official)

As documented in the official Vectrex reference (VECTREX.INC):

```
$C800-$C87F  BIOS RAM (128 bytes)
             System variables used by the BIOS

$C880-$CBEA  USER RAM (874 bytes available)
             ✅ Safe area for user programs

$CBEA-$CFFF  BIOS Stack and interrupt vectors
             Default stack at $CBEA

$D000-$D7FF  6522 VIA (shadowed 128 times)
             Hardware I/O (PSG, joystick, etc.)

$E000-$FFFF  System ROM (8KB BIOS)
             BIOS code
```

## 2. Real BIOS Variables (Used by VPy)

The VPy compiler uses these REAL BIOS addresses from Vectrex hardware:

### 2.1 Joystick (Joy_Analog BIOS)
```asm
$C81A  Vec_Joy_Resltn  ; Resolution (used in init)
$C81B  Vec_Joy_1_X     ; ✅ Joystick 1 X axis (0-255)
$C81C  Vec_Joy_1_Y     ; ✅ Joystick 1 Y axis (0-255)
$C81F  Vec_Joy_Mux_1_X ; X axis enable (used in init)
$C820  Vec_Joy_Mux_1_Y ; Y axis enable (used in init)
$C821  Vec_Joy_Mux_2_X ; Joystick 2 disable (used in init)
$C822  Vec_Joy_Mux_2_Y ; Joystick 2 disable (used in init)
$C823  Analog mode flag ; Cleared in init (Joy_Analog does DEC)
```

**IMPORTANT**: These are the ONLY valid addresses for joystick. Do NOT invent custom addresses.

### 2.2 Buttons (Read_Btns BIOS)
```asm
$C80F  Vec_Btn_State   ; Button state (written by Read_Btns)
```

## 3. VPy Compiler Variables

The VPy compiler uses the **`RamLayout`** system to assign variables automatically, guaranteeing ZERO collisions:

### 3.1 Automatic RamLayout (Perfect System ✅)
```rust
// ALL variables are assigned in this order automatically:
let mut ram = RamLayout::new(0xC880);

// 1. Runtime temporaries
ram.allocate("RESULT", 2, "Main result temporary");
ram.allocate("TMPPTR", 2, "Pointer temp");
// ...

// 2. PSG Music (if music assets present)
ram.allocate("PSG_MUSIC_PTR", 2, "...");
// ...

// 3. SFX (if SFX assets present)
ram.allocate("SFX_PTR", 2, "...");
// ...

// 4. PRINT_NUMBER buffer
ram.allocate("NUM_STR", 6, "...");

// 5. Vector list variables
ram.allocate("VL_PTR", 2, "Current position in vector list");
ram.allocate("VL_Y", 1, "Y position");
ram.allocate("VL_X", 1, "X position");
ram.allocate("VL_SCALE", 1, "Scale factor");

// 6. Drawing helpers (DRAW_VECTOR, DRAW_LINE, DRAW_CIRCLE)
ram.allocate("DRAW_VEC_X", 1, "...");
ram.allocate("MIRROR_X", 1, "...");
// ...

// 7. User variables (LAST — uses remaining space)
ram.allocate("VAR_PLAYER_X", 2, "...");
ram.allocate("VAR_ENEMIES_DATA", 6, "Array 3 elements");
// ...
```

### 3.2 ASM Output
```asm
; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 34 bytes (small program example)
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TEMP_YX              EQU $C880+$02   ; Temporary y,x storage (2 bytes)
VL_PTR               EQU $C880+$0C   ; Current position in vector list (2 bytes)
DRAW_VEC_X           EQU $C880+$11   ; DRAW_VECTOR X offset (1 bytes)
VAR_PLAYER_X         EQU $C880+$22   ; User variable (2 bytes)
...
```

**RamLayout advantages**:
- ✅ **Zero collisions**: Impossible for two variables to share an address
- ✅ **Compact**: No gaps or wasted space
- ✅ **Automatic**: No manual offset calculation needed
- ✅ **Flexible**: Adding/removing variables does not break anything
- ✅ **Safe**: Guarantees USER RAM limit ($CBEA) is never exceeded

## 4. Historical Bugs (Fixed)

### 4.1 Custom Joystick Addresses (Fixed 2025-12-30)
❌ **BUG (old)**:
```asm
$CF00  Joy_1_X  ; INVENTED ADDRESS — does not exist in hardware
$CF01  Joy_1_Y  ; INVENTED ADDRESS — does not exist in hardware
```

**Problem**: On real Vectrex hardware, these addresses contain garbage → random joystick values.

✅ **CORRECT (current)**:
```asm
$C81B  Vec_Joy_1_X  ; Real BIOS address (written by Joy_Analog $F1F5)
$C81C  Vec_Joy_1_Y  ; Real BIOS address (written by Joy_Analog $F1F5)
```

### 4.2 Vector List Variables in Stack Zone (Fixed 2025-12-30)
❌ **BUG (old)**:
```asm
$CF80  VL_PTR    ; In stack zone ($CBEA-$CFFF) — DANGEROUS
$CF82  VL_Y
$CF83  VL_X
$CF84  VL_SCALE
```

**Problem**: Stack grows downward from $CBEA → can overwrite these variables.

✅ **CORRECT (current — automatic RamLayout)**:
- All VL_*, DRAW_*, and VAR_* variables are assigned automatically
- RamLayout guarantees NO overlaps
- Example: In a small program (34 bytes total):
  - VL_PTR at $C880+$0C
  - DRAW_VEC_X at $C880+$11
  - User variables after everything else
- Collision with stack zone is impossible because RamLayout controls the upper bound

## 5. Memory Assignment Rules

### 5.1 RamLayout System (Automatic and Safe)
✅ **Automatic process**:
1. All variables assigned in sequential order
2. No gaps or overlaps
3. Total size calculated automatically
4. Guaranteed not to exceed USER RAM limit

✅ **Safe ranges**:
- Base: `$C880` (start of USER RAM)
- Variables assigned sequentially upward
- Limit: `$CBEA` (start of stack zone) — verified automatically by RamLayout

❌ **Forbidden**: Hardcoded addresses outside the RamLayout system
❌ **Forbidden**: `$C800-$C87F` (BIOS system variables)
❌ **Forbidden**: `$CBEA-$CFFF` (Stack zone)

### 5.2 Stack Considerations
The Vectrex stack starts at `$CBEA` by default and grows downward:
```
$CBEA  ← Initial stack pointer
$CBE9
$CBE8
...    ← Stack grows toward here
$C8XX  ← RamLayout ends here (depends on program)
```

**Available space**:
- Total USER RAM: 874 bytes ($C880-$CBEA)
- Small program: ~34 bytes → 840 bytes free
- Large program: ~200 bytes → 674 bytes free
- Stack: ~200 bytes assumed safe (normal usage)

## 6. Frontend Emulator Integration

The frontend (`EmulatorPanel.tsx`) must write joystick values to the REAL BIOS addresses:

```typescript
// ✅ CORRECT: Write to BIOS addresses
vecx.write8(0xC81B, analogX); // Vec_Joy_1_X
vecx.write8(0xC81C, analogY); // Vec_Joy_1_Y

// ❌ INCORRECT: Do NOT invent custom addresses
// vecx.write8(0xCF00, analogX); // This address does not exist in hardware!
```

## 7. Collision Verification

**RamLayout automatically guarantees ZERO collisions**:

1. **VPy variables**: Assigned sequentially by RamLayout
2. **VL_* variables**: Integrated into RamLayout (no longer hardcoded)
3. **Upper bound**: RamLayout verifies `total_size() + stack_size < 874 bytes`

**No manual verification is needed** — the RamLayout system prevents collisions by design.

### 7.1 Real Example (testsmallline.vpy)
```
Total RAM used: 34 bytes
Theoretical maximum: 874 bytes
Free space: 840 bytes (96% free!)
Safe stack: 200 bytes assumed → 640 bytes available for variables
```

**If a program needs more than ~674 bytes of variables**:
- ✅ RamLayout will detect the limit automatically
- ✅ Consider optimisations:
  - Use const arrays (stored in ROM, not RAM)
  - Reuse variables between functions
  - Use more efficient data structures

## 8. Debugging Memory Issues

If you experience memory corruption:

1. **Check variable size**: Are you using many large arrays?
2. **Check stack overflow**: Does your program have deep recursion or many nested calls?
3. **Memory Panel in IDE**: Use the search to inspect memory ranges
4. **Watch List**: Add critical variables to monitor changes

---

*Last updated: 2026-01-02 — RamLayout system implemented: all hardcoded addresses removed, automatic variable assignment guarantees zero collisions.*
