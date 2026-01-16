#!/usr/bin/env python3

# El archivo multibank.bin tiene 512KB
with open("bin/test_multibank_pdb.bin", "rb") as f:
    data = f.read()

print(f"Tamaño total del binario: {len(data)} bytes (0x{len(data):X})")

# Buscar CC FF BA (LDD #-70) en el primer banco (0x0000-0x4000)
print("\n=== BANK 0 (0x0000-0x4000) ===")
print("Buscando CC FF BA (LDD #-70)...")
found_count = 0
for i in range(min(0x4000, len(data) - 2)):
    if data[i:i+3] == b'\xCC\xFF\xBA':
        print(f"✅ Encontrado en offset 0x{i:04X}: {data[i:i+3].hex()}")
        found_count += 1

if found_count == 0:
    print("❌ NO ENCONTRADO")

# Buscar CC 00 00 (LDD #0)
print("\nBuscando CC 00 00 (LDD #0) en BANK 0...")
count = 0
for i in range(min(0x4000, len(data) - 2)):
    if data[i:i+3] == b'\xCC\x00\x00':
        if count < 3:
            print(f"   En offset 0x{i:04X}: {data[i:i+3].hex()}")
        count += 1

print(f"Total LDD #0 encontrados: {count} veces")

# Mostrar región alrededor de LOOP_BODY (0x004C)
print("\n=== REGIÓN LOOP_BODY (0x0040-0x00A0) ===")
for addr in range(0x0040, min(0x00A0, len(data)), 16):
    hex_str = ' '.join(f'{b:02X}' for b in data[addr:addr+16])
    ascii_str = ''.join(chr(b) if 32 <= b < 127 else '.' for b in data[addr:addr+16])
    print(f"0x{addr:04X}: {hex_str}  | {ascii_str}")
