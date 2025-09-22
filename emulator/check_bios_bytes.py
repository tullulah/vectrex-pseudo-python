import pathlib

# Leer bytes de la BIOS
data = pathlib.Path(r'C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin').read_bytes()
base = 0xE000  # 8K mapeada en 0xE000

print("BIOS bytes en F4EB-F4EF:")
for addr in range(0xF4EB, 0xF4F0):
    off = addr - base
    b = data[off]
    print(f"{addr:04X}: {b:02X}")