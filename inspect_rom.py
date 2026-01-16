#!/usr/bin/env python3
import os
import re

# Load the snapshot
path = 'examples/test_multibank_pdb/rom_snapshot_bank0_and_31.bin'
data = open(path, 'rb').read()

print(f"Total size: {len(data)} bytes (0x{len(data):X})")
print(f"\n=== BANK 0 (0x0000-0x3FFF) ===")
print(f"Bank 0 size: {0x4000} bytes (16KB)")
bank0 = data[0:0x4000]

print(f"\n=== BANK 31 (0x4000-0x7FFF) ===")
print(f"Bank 31 size: {0x4000} bytes (16KB)")
bank31 = data[0x4000:0x8000]

# Inspect first 300 bytes of bank0
print(f"\n=== BANK 0: First 300 bytes (hex) ===")
for i in range(0, min(300, len(bank0)), 16):
    hex_part = ' '.join(f'{b:02X}' for b in bank0[i:i+16])
    ascii_part = ''.join(chr(b) if 32 <= b < 127 else '.' for b in bank0[i:i+16])
    print(f"0x{i:04X}: {hex_part:<48} {ascii_part}")

# Look for JSR $4000 pattern (BD 40 00)
print(f"\n=== SEARCHING for JSR $4000 (BD 40 00) in BANK 0 ===")
pattern = b'\xBD\x40\x00'
offsets = []
for i in range(len(bank0) - 2):
    if bank0[i:i+3] == pattern:
        offsets.append(i)
        print(f"  Found at offset 0x{i:04X}: {bank0[i:i+10].hex()}")

if not offsets:
    print("  NOT FOUND - this is suspicious!")

# Inspect bank31 first 150 bytes
print(f"\n=== BANK 31: First 150 bytes (hex) ===")
for i in range(0, min(150, len(bank31)), 16):
    hex_part = ' '.join(f'{b:02X}' for b in bank31[i:i+16])
    ascii_part = ''.join(chr(b) if 32 <= b < 127 else '.' for b in bank31[i:i+16])
    print(f"0x{i:04X}: {hex_part:<48} {ascii_part}")

# Look for specific VECTREX_PRINT_TEXT pattern (BD F1 AA = JSR $F1AA)
print(f"\n=== SEARCHING for BIOS call pattern (BD F1 AA) ===")
pattern_bios = b'\xBD\xF1\xAA'
for i in range(len(bank31) - 2):
    if bank31[i:i+3] == pattern_bios:
        print(f"  Found at bank31 offset 0x{i:04X}: {bank31[i:i+10].hex()}")

# Look for strings
print(f"\n=== LOOKING for readable strings (4+ chars) ===")
for bank_name, bank_data in [("BANK0", bank0), ("BANK31", bank31)]:
    strings = re.findall(rb'[\x20-\x7E]{4,}', bank_data)
    if strings:
        print(f"\n{bank_name}:")
        for s in strings[:15]:
            idx = bank_data.find(s)
            try:
                decoded = s.decode('ascii')
                print(f"  0x{idx:04X}: {repr(decoded)}")
            except:
                pass

print(f"\n=== ANALYSIS ===")
null_byte = b'\x00'
bank0_filled = len(bank0) - len(bank0.lstrip(null_byte)) - len(bank0.rstrip(null_byte))
bank31_filled = len(bank31) - len(bank31.lstrip(null_byte)) - len(bank31.rstrip(null_byte))
print(f"Bank 0 filled: {bank0_filled} bytes of actual code/data")
print(f"Bank 31 filled: {bank31_filled} bytes of actual code/data")
print(f"JSR $4000 calls found: {len(offsets)}")
