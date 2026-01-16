# Snapshot Feature Bug Fix - 2026-01-15

## Issue Identified ✅

User correctly identified a flaw in the ROM snapshot function:

**The Question**: 
- "vecx.cart contains the ENTIRE ROM?"
- "Why do you use this.cart for single-bank OR this.multibankRom for multibank?"

**The Answer**: 
Two different storage containers based on ROM size - and I was reading from the WRONG one in multibank mode.

## Root Cause ❌

In `onSnapshotROM()`, I was always reading from `vecx.cart`:

```typescript
const currentBankOffset = currentBankId * 0x4000;
const currentBankData = new Uint8Array(0x4000);

for (let i = 0; i < 0x4000; i++) {
    currentBankData[i] = vecx.cart[currentBankOffset + i] || 0;  // ❌ BUG!
}
```

**Problem**: In multibank mode, `vecx.cart` is **NEVER FILLED** - it stays as 0x01 initialization!

## Storage Architecture

### Single-Bank Cartridge (≤32KB)
- **Storage**: `vecx.cart` (32KB Array)
- **Size**: Up to 32KB
- **read8()**: Reads from `this.cart`

### Multibank Cartridge (>32KB)
- **Storage**: `vecx.multibankRom` (512KB Uint8Array)
- **Size**: 512KB (32 banks × 16KB)
- **read8()**: Reads from `this.multibankRom` with bank switching
- **Note**: `vecx.cart` is NOT filled (stays at 0x01)

## The Fix ✅

Check `vecx.isMultibank` and read from the correct source:

```typescript
if (vecx.isMultibank && vecx.multibankRom) {
    // Multibank: read from multibankRom
    const currentBankOffset = currentBankId * 0x4000;
    currentBankData = vecx.multibankRom.slice(
        currentBankOffset, 
        currentBankOffset + 0x4000
    );
    
    const bank31Offset = 31 * 0x4000;
    bank31Data = vecx.multibankRom.slice(
        bank31Offset, 
        bank31Offset + 0x4000
    );
} else {
    // Single-bank: read from cart
    currentBankData = new Uint8Array(vecx.cart.slice(0, 0x4000));
    bank31Data = new Uint8Array(0x4000);  // No Bank #31
}
```

## Files Modified

1. **EmulatorPanel.tsx** (line 1610)
   - ✅ Added `isMultibank` check
   - ✅ Reads from `vecx.multibankRom` for multibank
   - ✅ Reads from `vecx.cart` for single-bank

2. **inspect_multibank_console.js** (lines ~100-150)
   - ✅ Added `isMultibank` check for Bank #0 reading
   - ✅ Added `isMultibank` check for Bank #31 reading
   - ✅ Displays correct source (multibankRom vs cart)

3. **ROM_STORAGE_ARCHITECTURE.md** (NEW)
   - 📚 Comprehensive explanation of storage architecture
   - 📚 Why two different containers
   - 📚 When each is used
   - 📚 Verification checklist

## Impact

**Before**: Snapshot would download 32KB of garbage (0x01 bytes) for multibank
**After**: Snapshot correctly downloads the ROM data from multibankRom

## Verification

To verify the fix works:

```bash
# 1. Compile multibank program
cargo run --bin vectrexc -- build examples/test_callgraph/src/main.vpy --bin

# 2. Load in IDE, pause, download snapshot
npm run dev
# → Load File → main.bin
# → Press ▶️ then ⏸️
# → Click 💾

# 3. Analyze
python3 analyze_rom_snapshot.py rom_snapshot_*.bin
# Should show code (not all 0x01)
```

## Technical Details

See [ROM_STORAGE_ARCHITECTURE.md](ROM_STORAGE_ARCHITECTURE.md) for:
- Memory layout diagrams
- Storage path comparison table
- Why this design was chosen
- How read8() determines which source to use

---

**Status**: ✅ FIXED  
**Date**: 2026-01-15  
**Credit**: User identified the architectural issue  
**Thanks**: User's attention to detail caught a subtle but critical bug!
