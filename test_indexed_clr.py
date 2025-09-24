#!/usr/bin/env python3
"""
Test mínimo para verificar el comportamiento de CLR indexed
"""

def test_clr_indexed():
    """
    Simular CLR 11,X donde X = 0xC800
    Debería acceder a dirección C800 + 11 = C80B
    X NO debería cambiar
    """
    print("TEST: CLR 11,X con X=C800")
    
    # Estado inicial
    x_initial = 0xC800
    print(f"X inicial: {x_initial:04X}")
    
    # Postbyte 0x8B = 10001011
    postbyte = 0x8B
    print(f"Postbyte: {postbyte:02X} = {postbyte:08b}")
    
    # Decodificación según 6809
    # Bit 7 = 1: indexed mode
    # post & 0x1F = 0x0B: 5-bit signed offset
    offset = postbyte & 0x1F  # 0x0B = 11
    if offset & 0x10:  # Sign extend si bit 4 está set
        offset = offset | 0xE0  # Sign extend to 8-bit
    
    print(f"Offset: {offset} (0x{offset:02X})")
    
    # Dirección efectiva
    effective_addr = (x_initial + offset) & 0xFFFF
    print(f"Dirección efectiva: {effective_addr:04X}")
    
    # X NO debería cambiar
    x_final = x_initial
    print(f"X final: {x_final:04X}")
    
    # Verificar resultado
    expected_addr = 0xC80B
    if effective_addr == expected_addr and x_final == x_initial:
        print("✅ TEST PASADO: CLR indexed funciona correctamente")
        return True
    else:
        print(f"❌ TEST FALLÓ: esperado addr={expected_addr:04X}, X={x_initial:04X}")
        print(f"                obtenido addr={effective_addr:04X}, X={x_final:04X}")
        return False

if __name__ == "__main__":
    test_clr_indexed()