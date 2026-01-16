#!/usr/bin/env python3
import sys

snapshot_path = "/Users/daniel/projects/vectrex-pseudo-python/examples/test_callgraph/rom_snapshot_bank0_and_31.bin"

try:
    with open(snapshot_path, "rb") as f:
        data = f.read()
except FileNotFoundError:
    print(f"Error: File not found: {snapshot_path}")
    sys.exit(1)

print(f"Snapshot size: {len(data)} bytes")
print()

# Bank 0 analysis
print("=== BANK 0 ($0000-$3FFF) ===")
bank0 = data[0x0000:0x4000]
print(f"First 16 bytes: {' '.join(f'{b:02X}' for b in bank0[:16])}")
print(f"Non-zero bytes: {sum(1 for b in bank0 if b != 0)}")
jsr_b0 = bank0.count(b'\x8d')
rts_b0 = bank0.count(b'\x39')
print(f"JSR count (0x8D): {jsr_b0}")
print(f"RTS count (0x39): {rts_b0}")

print()

# Bank 31 analysis  
print("=== BANK 31 ($4000-$7FFF) ===")
bank31 = data[0x4000:0x8000]
print(f"First 16 bytes: {' '.join(f'{b:02X}' for b in bank31[:16])}")
print(f"Non-zero bytes: {sum(1 for b in bank31 if b != 0)}")
jsr_b31 = bank31.count(b'\x8d')
rts_b31 = bank31.count(b'\x39')
print(f"JSR count (0x8D): {jsr_b31}")
print(f"RTS count (0x39): {rts_b31}")

print()
print("=== VERDICT ===")

bank0_quality = sum(1 for b in bank0 if b != 0)
bank31_quality = sum(1 for b in bank31 if b != 0)

if bank0_quality > 100:
    print("✅ Bank 0: Contains real code")
else:
    print("❌ Bank 0: Looks like garbage or padding")

if bank31_quality > 100:
    print("✅ Bank 31: Contains real code")
else:
    print("❌ Bank 31: Looks like garbage or padding")
