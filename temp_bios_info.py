import pathlib

data = pathlib.Path(r'ide\frontend\dist\bios.bin').read_bytes()
print(f'BIOS size: {len(data)} bytes')
print(f'First 32 bytes: {" ".join(f"{b:02X}" for b in data[:32])}')

# Verificar vectores de interrupción
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