# ASM Comparison: Legacy vs New Compiler

## Status: RESOLVED (2026-01-18)

The memory collision issue has been successfully identified and fixed.

## 1. The Collision (Root Cause)

### Legacy Compiler (Working)
* **Storage**: Uses `$C886` for temporary coordinate storage (MoveTo results).
* **Flags**: Uses `$C88E` and `$C88F` for `MIRROR_X` and `MIRROR_Y`.
* **Result**: Flags are safely stored far away from the coordinate scratchpad.

### New Compiler (Broken Build)
* **Storage**: Uses `$C886` for temporary coordinate storage (same as legacy).
* **Collision**: Allocates `MIRROR_Y` at **`$C886`**.
* **Effect**: The very instruction that calculates the vector position **OVERWRITES** the Mirror Y flag with the Y-coordinate.

## 2. The Solution

A safety padding block was introduced in the RAM layout definition (`core/src/backend/m6809/mod.rs`).

```rust
// CRITICAL FIX: Add padding to prevent collision with TEMP_YX
ram.allocate("MIRROR_PAD", 16, "Safety padding to prevent MIRROR flag corruption");
```

### Verified Memory Map (Fixed Build)
* **TEMP_YX**: `$C886` (Offset 6)
* **MIRROR_PAD**: `$C88E` (Offset 14, 16 bytes)
* **MIRROR_X**: **`$C89E`** (Offset 30) - Safe distance!
* **MIRROR_Y**: **`$C89F`** (Offset 31) - Safe distance!

## 3. Verification

The user confirmed that `main_new.bin` (compiled with padding) works correctly in the emulator, fixing the erratic mirror behavior. The collision at `$C886` is no longer possible.

## 4. VPy CLI Update (Fixed)

The `vpy_cli` tool (used by the IDE) depended on `vpy_codegen` which had a separate implementation of the RAM layout.
* **Fix Applied**: Updated `buildtools/vpy_codegen/src/m6809/helpers.rs` to include `MIRROR_PAD` and removed hardcoded aliases from `variables.rs`.
* **Verification**: `vpy_cli build examples/test_mirror/src/main.vpy` now generates `MIRROR_Y EQU $C880+$1C` ($C89C), which is safe from `TEMP_YX` ($C886).

---
**Old Analysis (For Reference)**

## 3. Root Cause

The new compiler likely defines `MIRROR_X` and `MIRROR_Y` as offsets within the `RESULT` block (e.g., `RESULT+4`, `RESULT+6`), but the `Draw_Sync_List` implementation *also* uses `RESULT+6` as a scratch register for the MoveTo coordinates.

**Fix Required**: Move `MIRROR_X` and `MIRROR_Y` to dedicated RAM addresses (global variables) or offsets safely outside the `RESULT` scratchpad range (e.g., `$C890+`).
