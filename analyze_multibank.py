#!/usr/bin/env python3
"""Analyze multibank ROM structure."""

with open('examples/test_callgraph/src/main.bin', 'rb') as f:
    # Bank #31 está en offset 0x7C000 (31 * 0x4000)
    f.seek(0x7C000)
    bank31_start = f.read(32)
    
    print("Bank #31 (offset 0x7C000) primeros 32 bytes:")
    for i in range(0, len(bank31_start), 16):
        chunk = bank31_start[i:i+16]
        hex_str = ' '.join(f'{b:02X}' for b in chunk)
        ascii_str = ''.join(chr(b) if 32 <= b < 127 else '.' for b in chunk)
        print(f"  +{i:04X}: {hex_str:48s} | {ascii_str}")
    
    # Bank #0
    f.seek(0x0000)
    bank0_start = f.read(32)
    print(f"\nBank #0 (offset 0x0000) primeros 32 bytes:")
    for i in range(0, len(bank0_start), 16):
        chunk = bank0_start[i:i+16]
        hex_str = ' '.join(f'{b:02X}' for b in chunk)
        ascii_str = ''.join(chr(b) if 32 <= b < 127 else '.' for b in chunk)
        print(f"  +{i:04X}: {hex_str:48s} | {ascii_str}")
    
    # Tamaño total
    f.seek(0, 2)
    size = f.tell()
    print(f"\nFile size: {size} bytes (0x{size:X})")
    print(f"Expected for multibank: 524288 bytes (0x{524288:X})")
    print(f"Is multibank: {size == 524288}")
