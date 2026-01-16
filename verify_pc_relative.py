#!/usr/bin/env python3
"""Verify PC-relative addressing bytecode in binary"""

import sys

# Read the binary
with open('/Users/daniel/projects/vectrex-pseudo-python/test_print_simple.bin', 'rb') as f:
    data = f.read()

# Search for LEAU [.,PC] pattern
# LEAU opcode is 0x33, postbyte 0x9C
found = False
for i in range(len(data) - 1):
    if data[i] == 0x33 and data[i+1] == 0x9C:
        print(f"✓ Found LEAU [.,PC] at offset 0x{i:04X}")
        print(f"  Bytes: 0x{data[i]:02X} 0x{data[i+1]:02X}")
        
        # Show context
        start = max(0, i - 10)
        end = min(len(data), i + 30)
        print(f"\n  Context (bytes around offset 0x{i:04X}):")
        for j in range(start, end, 16):
            hex_str = ' '.join(f'{data[k]:02X}' for k in range(j, min(j+16, end)))
            offset_str = f"0x{j:04X}"
            # Try to show ASCII
            ascii_str = ''
            for k in range(j, min(j+16, end)):
                c = chr(data[k])
                ascii_str += c if 32 <= ord(c) < 127 else '.'
            print(f"    {offset_str}: {hex_str:<48} {ascii_str}")
        found = True

if not found:
    print("✗ LEAU [.,PC] (0x33 0x9C) not found in binary")
    print("\nSearching for LEAU (0x33) patterns:")
    count = 0
    for i in range(len(data) - 1):
        if data[i] == 0x33:
            print(f"  Found LEAU at 0x{i:04X}, postbyte: 0x{data[i+1]:02X}")
            count += 1
            if count >= 10:
                print("  ...")
                break

