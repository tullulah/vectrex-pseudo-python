#!/usr/bin/env python3
import pathlib

# Single-bank (que funciona)
single = pathlib.Path('/Users/daniel/projects/vectrex-pseudo-python/examples/test_callgraph/build/test_callgraph.bin').read_bytes()

# Multibank snapshot Bank 0 (que no funciona)
multi_snapshot = pathlib.Path('/Users/daniel/projects/vectrex-pseudo-python/examples/test_callgraph/rom_snapshot_bank0_and_31.bin').read_bytes()
multi_bank0 = multi_snapshot[0x0000:0x4000]

print("=== COMPARACION: Single-Bank vs Multibank Bank 0 ===\n")

print("SINGLE-BANK (funciona):")
print(f"Size: {len(single)} bytes")
print(f"Primeros 64 bytes:")
for i in range(0, 64, 16):
    hex_str = ' '.join(f'{b:02X}' for b in single[i:i+16])
    ascii_str = ''.join(chr(b) if 32 <= b <= 126 else '.' for b in single[i:i+16])
    print(f"  0x{i:04X}: {hex_str}  {ascii_str}")

print("\nMULTIBANK Bank 0 (no funciona):")
print(f"Primeros 64 bytes:")
for i in range(0, 64, 16):
    hex_str = ' '.join(f'{b:02X}' for b in multi_bank0[i:i+16])
    ascii_str = ''.join(chr(b) if 32 <= b <= 126 else '.' for b in multi_bank0[i:i+16])
    print(f"  0x{i:04X}: {hex_str}  {ascii_str}")

print("\n=== SON IGUALES? ===")
if single[:len(multi_bank0)] == multi_bank0:
    print("SI - Son identicos en los primeros 16KB")
else:
    print("NO - Hay diferencias")
    for i in range(min(len(single), len(multi_bank0))):
        if single[i] != multi_bank0[i]:
            print(f"  Primera diferencia en 0x{i:04X}: Single=0x{single[i]:02X} vs Multi=0x{multi_bank0[i]:02X}")
            break
