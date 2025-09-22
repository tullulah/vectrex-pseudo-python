import pathlib

data = pathlib.Path(r'ide\frontend\dist\bios.bin').read_bytes()
base = 0xE000  # 8K BIOS mapeada en 0xE000
reset_addr = 0xF000
offset = reset_addr - base

print(f'Code at RESET vector ${reset_addr:04X}:')
for i in range(20):
    addr = reset_addr + i
    off = addr - base
    if off < len(data):
        byte = data[off]
        print(f'  ${addr:04X}: ${byte:02X}')
    else:
        break
        
# Verificar también si hay código de inicialización típico
print('\nBuscando código de inicialización común cerca del reset:')        
for start_offset in [0x1000, 0x1200, 0x1400, 0x1800, 0x1C00]:  # F000, F200, F400, F800, FC00
    addr = 0xE000 + start_offset
    if start_offset < len(data):
        byte = data[start_offset]
        if byte != 0x00:  # No vacío
            print(f'  ${addr:04X}: ${byte:02X} {"(possible code)" if byte in [0x10, 0x8E, 0xCE, 0x1A, 0x12] else ""}')