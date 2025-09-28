# Simple ROM Test para Vectrex
# Este archivo genera un ROM mínimo para probar la funcionalidad de carga

# Crear un ROM de prueba de 8KB con un patrón simple
with open("test_rom.bin", "wb") as f:
    # Header mínimo del Vectrex ROM
    # Reset vector en 0x0000-0x0001 (apunta a 0x8000)
    f.write(bytes([0x80, 0x00]))  # Reset vector -> 0x8000
    
    # Padding hasta 0x8000 (32768 bytes) donde típicamente comienzan los ROMs
    padding = [0x00] * (0x8000 - 2)
    f.write(bytes(padding))
    
    # Código simple en 0x8000
    # LDX #$C800 (cargar X con dirección de RAM)
    f.write(bytes([0x8E, 0xC8, 0x00]))
    
    # LDA #$FF (cargar A con 0xFF)
    f.write(bytes([0x86, 0xFF]))
    
    # STA ,X+ (almacenar A en X y incrementar)
    f.write(bytes([0xA7, 0x80]))
    
    # BRA * (bucle infinito)
    f.write(bytes([0x20, 0xFE]))
    
    # Rellenar el resto hasta completar el ROM
    remaining = 0x2000 - (f.tell() - 0x8000)  # 8KB ROM size
    f.write(bytes([0x00] * remaining))

print("test_rom.bin creado exitosamente!")