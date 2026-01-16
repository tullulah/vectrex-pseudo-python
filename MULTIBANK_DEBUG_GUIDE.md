# Multibank ROM Debugging Guide

## Overview
The emulator now has a **ROM Snapshot** button (💾) that appears when the emulator is **paused**. This allows you to download and inspect the ROM contents from the emulator's memory to diagnose why multibank execution is hanging.

## Step-by-Step Debugging Process

### Phase 1: Verify Multibank File Generation
```bash
# 1. Compile multibank program
cargo run --bin vectrexc -- build examples/test_callgraph/src/main.vpy --bin

# 2. Verify file size (should be 524288 bytes = 512KB)
ls -la examples/test_callgraph/src/main.bin
wc -c examples/test_callgraph/src/main.bin

# Expected output:
# 524288 bytes for 512KB multibank (32 banks × 16KB each)
```

### Phase 2: Load ROM in Emulator & Pause
1. **Open IDE**: `npm run dev` (or similar)
2. **Load ROM**: Use "Load File..." button to load the 512KB `.bin` file
3. **Start Emulation**: Press ▶️ button
4. **Wait**: Let it run for a few seconds
5. **Pause**: Press ⏸️ button
   - Expected: 💾 button now appears (next to overlay button)

### Phase 3: Download ROM Snapshot
1. **Click 💾 button** when emulator is paused
2. **File saves**: `rom_snapshot_bank0_and_31.bin` (32KB)
   - Contains current bank (Bank #0) at offset 0x0000-0x3FFF
   - Contains Bank #31 (fixed) at offset 0x4000-0x7FFF

### Phase 4: Analyze Snapshot with Python
```bash
# Run analysis script
python3 analyze_rom_snapshot.py rom_snapshot_bank0_and_31.bin

# Expected output:
# ✅ File size: 32KB (correct)
# ✅ Bank 0 starts with code (not garbage)
# ✅ Bank 31 starts with "LDA #0" (CUSTOM_RESET pattern)
```

### Phase 5: Browser Console Inspection
If you want more detailed inspection:

1. **Open Browser DevTools**: F12 or Right-click → Inspect
2. **Go to Console tab**
3. **Paste & run**: Contents of `inspect_multibank_console.js`
   ```javascript
   // Paste entire inspect_multibank_console.js here
   ```
4. **Check output**:
   - `✅ CORRECT: Cartdata is 512KB (multibank)`
   - `✅ CORRECT: Cart array is >= 524288 bytes`
   - `✅ Bank #0 starts with code (not garbage)`
   - `✅ Bank #31 starts with "LDA #0" (CUSTOM_RESET)`

### Phase 6: Interpret Results

#### SUCCESS CASE ✅
```
File size:     32KB ✓
Bank 0:        Has code (non-garbage) ✓
Bank 31:       Has CUSTOM_RESET (LDA #0) ✓
Console:       Cartdata 512KB ✓
Console:       Cart array initialized ✓
```
**Conclusion**: ROMs are loading correctly. Problem is NOT in array mounting.
**Next step**: Trace BIOS execution with debugger (why is it hanging at 0xF33D?)

#### FAILURE CASE ❌
```
File size:     32KB ✗ (shows different size)
Bank 0:        Garbage (all zeros or random) ✗
Bank 31:       Wrong pattern ✗
Console:       Cartdata 32KB (wrong, should be 512KB) ✗
Console:       Cart array too small ✗
```
**Conclusion**: Arrays NOT mounting correctly in multibank mode.
**Next step**: Fix cartdata loading or file parsing

## What Each File Contains

### rom_snapshot_bank0_and_31.bin (32KB)
```
Bytes 0x0000-0x3FFF: Current bank (16KB)
  - CPU can read this via address 0x0000-0x3FFF
  - Content determined by value in 0xD000 (bank switch register)
  - In snapshot: contents of Bank #0 (initial bank)

Bytes 0x4000-0x7FFF: Bank #31 (fixed 16KB)
  - CPU always reads this via address 0x4000-0x7FFF
  - Never changes (fixed bank)
  - Should contain CUSTOM_RESET or stable code
```

### analyze_rom_snapshot.py
Python script that:
- Verifies file size (must be 32KB)
- Shows hex dump of first 64 bytes of each bank
- Detects GCE header and M6809 opcodes
- Checks for CUSTOM_RESET pattern (86 00 = LDA #0)
- Reports byte usage statistics

### inspect_multibank_console.js
JavaScript snippet for browser console that:
- Checks `Globals.cartdata` length (should be 524288)
- Checks `vecx.cart` array size (should be ≥ 524288)
- Shows first 64 bytes of Bank #0 and Bank #31 in hex
- Displays current CPU registers and state
- Exports diagnostic JSON for analysis

## Troubleshooting

### "Snapshot button doesn't appear"
- **Issue**: Not paused
- **Solution**: Click ⏸️ to pause emulator first

### "Downloaded file is 0 bytes"
- **Issue**: vecx instance not available
- **Solution**: Check browser console for errors, reload IDE

### "Bank 0 shows all zeros"
- **Issue**: Bank not loaded into cart array
- **Solution**: Check if Globals.cartdata is being copied (console check)

### "Bank 31 shows wrong pattern"
- **Issue**: File offset calculation wrong (should be offset 0x7C000 = 31×0x4000)
- **Solution**: Verify vecx.cart[] is large enough to hold 512KB

## Expected Results for Multibank

### Good Multibank Loading
```
Python analysis output:
✅ Correct size (32KB)
✅ Bank 0: non-zero bytes = 2345 / 16384
✅ Bank 31: non-zero bytes = 1234 / 16384
✅ Found 'LDA #0' at offset 0x0000 (CUSTOM_RESET start)

Browser console output:
✅ CORRECT: Cartdata is 512KB (multibank)
✅ CORRECT: Cart array is >= 524288 bytes
✅ Bank #0 starts with code (not garbage)
✅ Bank #31 starts with "LDA #0" (CUSTOM_RESET)
```

### Bad Multibank Loading
```
Python analysis output:
⚠️  Expected 32KB (32768 bytes), got 8192  ❌
⚠️  Bank 31 starts with 0x00 0x00 (expected 86 00)  ❌

Browser console output:
❌ ERROR: Cartdata is 32768 bytes (unexpected)  ❌ Should be 524288
❌ ERROR: Cart array is 0 bytes (too small)  ❌ Should be ≥ 524288
```

## When to Use Snapshot Button

| Scenario | Action |
|----------|--------|
| Program compiles, runs for a moment, then hangs | ✅ Use snapshot to see if ROM loaded |
| BIOS boots but program doesn't start | ✅ Use snapshot to verify Bank #0 content |
| Execution stuck at 0xF33D (BIOS) | ✅ Use snapshot to see if CUSTOM_RESET in Bank #31 |
| Program works but produces garbage graphics | ⚠️ Snapshot might not help (content issue) |

## Next Steps After Diagnosis

If ROM loading is **GOOD** ✅:
- Problem is in BIOS execution flow or CPU state
- Use debugger to trace from BIOS RESET through CUSTOM_RESET
- Check if BIOS jumps to 0x0000 (Bank #0 start)

If ROM loading is **BAD** ❌:
- Problem is in cartdata initialization or array mounting
- Check EmulatorPanel.tsx cartridge loading code
- Verify JSVecx cart[] initialization
- Check if 512KB file is being read completely

## Files Provided

1. **analyze_rom_snapshot.py** - Python analysis tool
   - Usage: `python3 analyze_rom_snapshot.py rom_snapshot_*.bin`
   - Shows hex dumps, header detection, pattern matching

2. **inspect_multibank_console.js** - Browser console script
   - Run in browser DevTools console (F12)
   - Shows Globals.cartdata and vecx.cart state
   - Exports diagnostic JSON

3. **EmulatorPanel.tsx** - Updated with onSnapshotROM function
   - Added 💾 button (visible only when paused)
   - Triggers download of 32KB snapshot

## Quick Start

```bash
# Compile multibank
cargo run --bin vectrexc -- build examples/test_callgraph/src/main.vpy --bin

# Verify file size
ls -l examples/test_callgraph/src/main.bin  # Should be 524288 bytes

# Start IDE and load ROM, pause, download snapshot
# Then analyze:
python3 analyze_rom_snapshot.py rom_snapshot_*.bin

# If needed, run JavaScript console check (paste into F12 console)
```

---

**Created**: 2026-01-15  
**Purpose**: Diagnose multibank ROM loading issues  
**Status**: Ready to debug
