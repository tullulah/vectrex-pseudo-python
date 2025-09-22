#!/usr/bin/env python3
# Examinar el contexto del bucle F4EB-F4EF en la BIOS real

import pathlib

# Leer BIOS
bios_path = r'C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin'
data = pathlib.Path(bios_path).read_bytes()
base = 0xE000  # 8K mapeada en 0xE000

print("🔍 Contexto del bucle F4EB-F4EF:")
print()

# Examinar 16 bytes antes del bucle para ver el contexto
start_addr = 0xF4EB - 16
end_addr = 0xF4EB + 16

for addr in range(start_addr, end_addr):
    off = addr - base
    if 0 <= off < len(data):
        b = data[off]
        marker = ""
        if addr == 0xF4EB:
            marker = " <- LDA #$81 (INICIO BUCLE)"
        elif addr == 0xF4ED:
            marker = " <- NOP"
        elif addr == 0xF4EE:
            marker = " <- DECB"
        elif addr == 0xF4EF:
            marker = " <- BNE"
        elif addr == 0xF4F0:
            marker = " <- DESPUÉS DEL BUCLE"
        print(f"{addr:04X}: {b:02X}{marker}")
    else:
        print(f"{addr:04X}: --")

print()
print("📊 Análisis del BNE offset:")
bne_addr = 0xF4EF
if bne_addr - base < len(data):
    offset_byte = data[bne_addr + 1 - base]  # El byte después del BNE
    # Convertir a signed offset
    if offset_byte > 127:
        signed_offset = offset_byte - 256
    else:
        signed_offset = offset_byte
    
    target_addr = bne_addr + 2 + signed_offset  # PC después del BNE + offset
    print(f"BNE offset byte: 0x{offset_byte:02X}")
    print(f"Signed offset: {signed_offset}")
    print(f"Target address: 0x{target_addr:04X}")
    
    if target_addr == 0xF4EB:
        print("✅ BNE salta correctamente de vuelta al inicio del bucle")
    else:
        print(f"❌ BNE NO salta al inicio del bucle (0xF4EB), sino a 0x{target_addr:04X}")

print()
print("🎯 Lo que el bucle DEBERÍA hacer:")
print("1. DECB decrementa B")
print("2. Si B != 0, BNE salta de vuelta a F4EB")
print("3. Si B == 0, el bucle termina y continúa en F4F1")