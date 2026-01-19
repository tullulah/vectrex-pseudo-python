# Comparison Report: Legacy vs New Compiler

## Setup
- **Test Asset**: `asym.vec` (Triangle: 0,0 -> 10,20 -> 20,0)
- **Legacy Compiler**: `vectrexc` (core crate, master branch)
- **New Compiler**: `vpy_cli` (buildtools crate, with `vecres.rs` reverted to matching logic)

## 1. Vector Data Integrity
Both compilers produced **identically** generated vector data blocks:

**Legacy (main_old.asm):**
```asm
_ASYM_PATH0:
    FCB 127              ; Intensity
    FCB $F6,$F6,0,0      ; Header: y=-10, x=-10 (Center-relative origin)
    FCB $FF,$14,$0A      ; Line 1: dy=20, dx=10  (Matches 0,0 -> 10,20)
    FCB $FF,$EC,$0A      ; Line 2: dy=-20, dx=10 (Matches 10,20 -> 20,0)
    FCB 2                ; End
```

**New (main_new.asm):**
```asm
_ASYM_PATH0:
    FCB 127
    FCB $F6,$F6,0,0
    FCB $FF,$14,$0A
    FCB $FF,$EC,$0A
    FCB 2
```

**Conclusion**: The logic for calculating Delta Y and Delta X is now consistent. `dy = y2 - y1` (Standard).

## 2. Drawing Logic
Both compilers utilize `Draw_Sync_List_At_With_Mirrors`.
- New compiler implements `NEGB` (for Y mirror) and `NEGA` (for X mirror).
- Given the standard data (`dy` positive = Up), negating it correctly produces a downward line (Vertical Flip).

## 3. Differences
The assembly files differ in:
- **Header**: "GCE 1982" vs "GCE 2025"
- **Variable Layout**: Addresses differ slightly ($C880 bases), labels renamed (e.g., `DRAW_VEC_X` vs `RESULT+offset`).
- **Helpers**: Legacy creates `Draw_Sync_List` (unused) and `Draw_Sync_List_At_With_Mirrors`. New compiler only emits the used `Draw_Sync_List_At_With_Mirrors` (Optimization).

## Final Verification
The "Revert" action successfully aligned the new compiler's vector math with the legacy compiler's proven logic.

## 4. Binary Artifacts
Generated binaries for runtime comparison:
- **Legacy**: `examples/test_mirror/src/main_old.bin`
- **New**: `examples/test_mirror/src/main_new.bin`

Both binaries are 32KB padded. Use these to verify runtime behavior in the emulator.
