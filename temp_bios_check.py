import pathlib

data = pathlib.Path(r'C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin').read_bytes()

# Buscar todas las escrituras a VIA ($D000-$D00F)
print('Buscando todas las escrituras a VIA ($D000-$D00F):')
for i in range(len(data)-2):
    # STA extended $D00x
    if data[i] == 0xB7 and data[i+1] == 0xD0 and data[i+2] <= 0x0F:
        addr = 0xE000 + i if len(data) == 0x2000 else 0xF000 + i
        print(f'  STA $D0{data[i+2]:02X} at {addr:04X}')
    # STB extended $D00x  
    if data[i] == 0xF7 and data[i+1] == 0xD0 and data[i+2] <= 0x0F:
        addr = 0xE000 + i if len(data) == 0x2000 else 0xF000 + i
        print(f'  STB $D0{data[i+2]:02X} at {addr:04X}')

print('\nVerificando secuencias con Timer initialization:')
# Buscar secuencias que tengan $C0 (set bit 7+6 para IER)
for i in range(len(data)-10):
    if data[i] == 0x86 and data[i+1] == 0xC0:  # LDA #$C0
        addr = 0xE000 + i if len(data) == 0x2000 else 0xF000 + i
        # Mostrar el contexto
        context = data[i:i+10]
        print(f'  LDA #$C0 at {addr:04X}: {" ".join(f"{b:02X}" for b in context)}')

# Buscar también patrones de inicialización específicos de Vectrex
print('\nBuscando secuencias de inicialización comunes:')
for i in range(len(data)-20):
    # Secuencia típica: Clear todos los registros VIA
    if (data[i:i+3] == bytes([0x4F]) and  # CLRA 
        any(data[i+j:i+j+3] == bytes([0xB7, 0xD0, reg]) for j in range(1,10) for reg in range(16))):
        addr = 0xE000 + i if len(data) == 0x2000 else 0xF000 + i
        context = data[i:i+20]
        print(f'  Clear sequence at {addr:04X}: {" ".join(f"{b:02X}" for b in context)}')
        break