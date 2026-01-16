#!/usr/bin/env python3
# Inspect the updated snapshot
data = open('examples/test_multibank_pdb/rom_snapshot_bank0_and_31.bin', 'rb').read()
bank0 = data[0:0x4000]

print("=== SEARCHING IN UPDATED SNAPSHOT ===\n")

# Search for CC FF BA pattern (LDD #-70)
pattern_correct = bytes([0xCC, 0xFF, 0xBA])
found_correct = []
for i in range(len(bank0) - 2):
    if bank0[i:i+3] == pattern_correct:
        found_correct.append(i)

# Search for CC 00 00 pattern (LDD #0)
pattern_zero = bytes([0xCC, 0x00, 0x00])
found_zero = []
for i in range(len(bank0) - 2):
    if bank0[i:i+3] == pattern_zero:
        found_zero.append(i)

print(f"Found LDD #-70 (CC FF BA): {len(found_correct)} times")
for offset in found_correct:
    print(f"  0x{offset:04X}")

print(f"\nFound LDD #0 (CC 00 00): {len(found_zero)} times")
for offset in found_zero[:10]:
    print(f"  0x{offset:04X}")

# Inspect LOOP_BODY area specifically
print(f"\n=== LOOP_BODY area (0x0070-0x00B0) ===")
for i in range(0x0070, 0x00B0, 16):
    hex_part = ' '.join(f'{bank0[j]:02X}' for j in range(i, min(i+16, 0x00B0)))
    print(f"0x{i:04X}: {hex_part}")

print(f"\n=== DETAILED: First two LDD instructions ===")
# Find first JSR $4000 to locate PRINT_TEXT calls
for i in range(0x0070, 0x00B0):
    if bank0[i:i+3] == bytes([0xBD, 0x40, 0x00]):
        print(f"\nFound JSR $4000 at 0x{i:04X}")
        print(f"  Context (20 bytes before):")
        for j in range(max(0, i-20), i):
            print(f"    0x{j:04X}: {bank0[j]:02X}")
        break
