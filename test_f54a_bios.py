#!/usr/bin/env python3
"""
Test para verificar la línea F54A en el BIOS.
Debería ser SUBD #$0001 (83 00 01)
"""

def examine_bios_f54a():
    """Examinar los bytes en F54A en el BIOS"""
    import pathlib
    
    # Leer BIOS
    bios_path = pathlib.Path(r'C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin')
    data = bios_path.read_bytes()
    base = 0xE000
    
    # Examinar la secuencia completa
    print("Bytes en la secuencia F548-F550:")
    for addr in range(0xF548, 0xF550):
        offset = addr - base
        byte_val = data[offset]
        print(f"  {addr:04X}: {byte_val:02X}")
    
    # Analizar F54A específicamente
    f54a_offset = 0xF54A - base
    opcode = data[f54a_offset]
    operand1 = data[f54a_offset + 1]
    operand2 = data[f54a_offset + 2]
    
    print(f"\nInstrucción en F54A:")
    print(f"  Opcode: {opcode:02X}")
    print(f"  Operand 1: {operand1:02X}")
    print(f"  Operand 2: {operand2:02X}")
    
    if opcode == 0x83:
        value = (operand1 << 8) | operand2
        print(f"  Interpretación: SUBD #${value:04X}")
        print(f"  ✓ Esto es SUBD immediate, NO CMPD ni CMPU")
    else:
        print(f"  ❌ Opcode inesperado: {opcode:02X}")

if __name__ == "__main__":
    examine_bios_f54a()