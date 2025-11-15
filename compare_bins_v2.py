#!/usr/bin/env python3
"""Compare two binary files byte by byte"""
import sys

def compare_binaries(file1, file2):
    with open(file1, 'rb') as f1, open(file2, 'rb') as f2:
        data1 = f1.read()
        data2 = f2.read()
    
    print(f"File 1: {file1} ({len(data1)} bytes)")
    print(f"File 2: {file2} ({len(data2)} bytes)")
    print(f"Size difference: {abs(len(data1) - len(data2))} bytes")
    print()
    
    min_len = min(len(data1), len(data2))
    max_len = max(len(data1), len(data2))
    
    differences = []
    matches = 0
    
    for i in range(min_len):
        if data1[i] != data2[i]:
            differences.append((i, data1[i], data2[i]))
        else:
            matches += 1
    
    # Extra bytes in longer file
    if len(data1) > len(data2):
        for i in range(min_len, len(data1)):
            differences.append((i, data1[i], None))
    elif len(data2) > len(data1):
        for i in range(min_len, len(data2)):
            differences.append((i, None, data2[i]))
    
    print(f"Matching bytes: {matches}/{min_len} ({100*matches/min_len:.1f}%)")
    print(f"Different bytes: {len(differences)}")
    print()
    
    if differences:
        print("First 50 differences:")
        print("Offset    NativeV2  lwasm     ASCII")
        print("-" * 50)
        for i, (offset, b1, b2) in enumerate(differences[:50]):
            if b1 is None:
                print(f"${offset:04X}    --        ${b2:02X}       (extra in lwasm)")
            elif b2 is None:
                print(f"${offset:04X}    ${b1:02X}      --        (extra in native)")
            else:
                c1 = chr(b1) if 32 <= b1 < 127 else '.'
                c2 = chr(b2) if 32 <= b2 < 127 else '.'
                print(f"${offset:04X}    ${b1:02X}      ${b2:02X}       '{c1}' vs '{c2}'")
    else:
        print("✅ Files are IDENTICAL!")
    
    # Show hex dumps of first 128 bytes for context
    print("\n" + "="*60)
    print("First 128 bytes (hex dump):")
    print("="*60)
    print("\nNative V2:")
    print_hex_dump(data1[:128])
    print("\nlwasm:")
    print_hex_dump(data2[:128])

def print_hex_dump(data):
    for i in range(0, len(data), 16):
        chunk = data[i:i+16]
        hex_part = ' '.join(f'{b:02X}' for b in chunk)
        ascii_part = ''.join(chr(b) if 32 <= b < 127 else '.' for b in chunk)
        print(f"${i:04X}: {hex_part:<48} {ascii_part}")

if __name__ == '__main__':
    compare_binaries('test_debug_simple_native_v2.bin', 'test_debug_simple_lwasm.bin')
