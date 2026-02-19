# ROM Snapshot Debugging Feature - Quick Start

## What's New ✨

The emulator now has a **💾 Snapshot** button that appears when paused. This lets you download and inspect the ROM state to diagnose multibank issues.

**Implementation**: Uses `vecx.read8()` to read CPU address space (0x0000-0x7FFF) - tests that the address mapping works correctly!

## How to Use

### 1. Load Multibank Program
```bash
cargo run --bin vectrexc -- build examples/test_callgraph/src/main.vpy --bin
```
Verify file size: `ls -l examples/test_callgraph/src/main.bin` → **524288 bytes** (512KB) ✓

### 2. Open IDE & Load ROM
```bash
npm run dev
```
- Click **"Load File..."** button
- Select the `.bin` file (512KB)
- Click **▶️** to start emulation
- Wait a few seconds
- Click **⏸️** to pause

### 3. Download Snapshot
- **💾** button now visible (next to overlay button)
- Click to download: `rom_snapshot_bank0_and_31.bin` (32KB)

### 4. Analyze Snapshot
```bash
# Quick hex analysis
python3 analyze_rom_snapshot.py rom_snapshot_bank0_and_31.bin

# Expected output:
# ✅ File size: 32KB
# ✅ Bank 0: [code or data]
# ✅ Bank 31: starts with "LDA #0" (CUSTOM_RESET)
```

### 5. Browser Console Check (Optional)
1. Press **F12** → Console tab
2. Paste the entire contents of `inspect_multibank_console.js`
3. Check output:
   - `✅ CORRECT: Cartdata is 512KB (multibank)` - cartdata loaded correctly
   - `✅ CORRECT: Cart array is >= 524288 bytes` - emulator memory OK
   - `✅ Bank #0 starts with code` - Bank 0 has code
   - `✅ Bank #31 starts with "LDA #0"` - Bank 31 has CUSTOM_RESET

## What You're Checking

| Item | Expected | Means |
|------|----------|-------|
| Snapshot file size | 32KB (32768 bytes) | Correct extraction |
| Bank 0 content | Has code (non-zero) | Bank 0 loaded correctly |
| Bank 31 first bytes | `86 00` (LDA #0) | Bank 31 has CUSTOM_RESET |
| Console: cartdata | 524288 bytes | Full 512KB file loaded |
| Console: cart array | ≥524288 bytes | Emulator memory large enough |

## Interpretation

### All ✅ (Good News!)
```
ROM loads correctly → Problem is in BIOS execution or CPU state
→ Use debugger to trace RESET → CUSTOM_RESET → START flow
```

### Some ❌ (Array Mounting Problem!)
```
ROM doesn't load correctly → Problem is in cartdata loading
→ Check EmulatorPanel.tsx file reading or JSVecx initialization
→ Verify 512KB file is being parsed completely
```

## File Descriptions

| File | Purpose |
|------|---------|
| `analyze_rom_snapshot.py` | Python tool to inspect downloaded binary |
| `inspect_multibank_console.js` | JavaScript console script (run in F12) - **FIXED** to read from correct source |
| `MULTIBANK_DEBUG_GUIDE.md` | Detailed troubleshooting guide |
| `ROM_STORAGE_ARCHITECTURE.md` | **NEW** - Explains single-bank vs multibank storage (vecx.cart vs vecx.multibankRom) |
| EmulatorPanel.tsx (updated) | Added `onSnapshotROM()` function + 💾 button - **FIXED** to check isMultibank |

## Example Output

### python3 analyze_rom_snapshot.py
```
📦 ROM Snapshot Analysis: rom_snapshot_bank0_and_31.bin
   Size: 32768 bytes (32KB)
   ✅ Correct size (32KB)

BANK 0 (Current Bank) - Offset 0x0000-0x3FFF
📍 First 32 bytes (hex):
   0000: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 | ................
   0010: 00 00 00 00 00 00 00 00 80 2F 80 30 80 2F 06 04 | ........./..0../..

✅ Bank #0 starts with code (not garbage)

BANK #31 (Fixed Bank) - Offset 0x4000-0x7FFF
📍 First 32 bytes (hex):
   0000: 86 00 B7 DF 00 B7 80 00 7E 40 00 39 00 00 00 00 | ......~@..9....
   0010: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 | ................

✅ Found 'LDA #0' at offset 0x0000 (CUSTOM_RESET start)
```

### Browser Console Output
```
=== MULTIBANK CARTRIDGE LOADING CHECK ===
Globals.cartdata size: 524288 bytes (512.0KB)
✅ CORRECT: Cartdata is 512KB (multibank)

=== EMULATOR CART ARRAY CHECK ===
vecx.cart array size: 524288 bytes (512.0KB)
vecx.current_bank: 0
✅ CORRECT: Cart array is >= 524288 bytes

=== BANK #0 CONTENTS CHECK ===
Bank #0 first 64 bytes (hex):
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 80 2F 80 30 80 2F 06 04
...
✅ Bank #0 starts with code (not garbage)

=== BANK #31 CONTENTS CHECK ===
Bank #31 first 64 bytes (offset 0x7C000):
86 00 B7 DF 00 B7 80 00 7E 40 00 39 00 00 00 00
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
...
✅ Bank #31 starts with "LDA #0" (CUSTOM_RESET)
```

## Troubleshooting

| Problem | Cause | Fix |
|---------|-------|-----|
| Snapshot button doesn't appear | Not paused | Press ⏸️ button first |
| Downloaded file is 0 bytes | JS error | Check F12 console for errors |
| Bank 0 is all zeros | Not loaded | Check `Globals.cartdata` in console |
| Bank 31 wrong pattern | File offset wrong | Verify JSVecx address mapping |
| Cartdata shows 32KB in console | File loading issue | Check if full 512KB file selected |

## Commands Reference

```bash
# Compile multibank program
cargo run --bin vectrexc -- build examples/test_callgraph/src/main.vpy --bin

# Check file size (should be 524288)
ls -la examples/test_callgraph/src/main.bin

# Analyze downloaded snapshot
python3 analyze_rom_snapshot.py rom_snapshot_*.bin

# Start IDE
npm run dev
```

## Next Steps

1. **If ROM loads ✅**: Use IDE debugger to trace BIOS → CUSTOM_RESET → START
2. **If ROM fails ❌**: Check file reading in `ide/frontend/src/components/panels/EmulatorPanel.tsx` line ~1800

---

Created: 2026-01-15  
Feature: ROM Snapshot button for multibank debugging  
Status: Ready to use
