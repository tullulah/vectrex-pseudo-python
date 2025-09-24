import pathlib

# Verificar la BIOS del sistema real
data = pathlib.Path(r'vectrexy\data\bios\System.bin').read_bytes()
print(f'System.bin size: {len(data)} bytes')

if len(data) >= 32:
    print(f'Last 32 bytes: {" ".join(f"{b:02X}" for b in data[-32:])}')
    
    # Los vectores están al final
    reset_vector = (data[-4] << 8) | data[-3]
    irq_vector = (data[-8] << 8) | data[-7] 
    firq_vector = (data[-6] << 8) | data[-5]
    nmi_vector = (data[-2] << 8) | data[-1]
    
    print(f'Reset vector: ${reset_vector:04X}')
    print(f'IRQ vector:   ${irq_vector:04X}')
    print(f'FIRQ vector:  ${firq_vector:04X}')
    print(f'NMI vector:   ${nmi_vector:04X}')

# Buscar todas las escrituras a VIA ($D000-$D00F) en la BIOS real
print('\nBuscando escrituras a VIA en System.bin:')
for i in range(len(data)-2):
    # STA extended $D00x
    if data[i] == 0xB7 and data[i+1] == 0xD0 and data[i+2] <= 0x0F:
        addr = 0xE000 + i if len(data) == 0x2000 else 0xF000 + i
        print(f'  STA $D0{data[i+2]:02X} at {addr:04X}')
    # STB extended $D00x  
    if data[i] == 0xF7 and data[i+1] == 0xD0 and data[i+2] <= 0x0F:
        addr = 0xE000 + i if len(data) == 0x2000 else 0xF000 + i
        print(f'  STB $D0{data[i+2]:02X} at {addr:04X}')