#!/usr/bin/env python3
"""
Test directo en Python para simular el comportamiento esperado
"""

def test_decode_indexed_0x8B():
    """Simula el comportamiento esperado para postbyte 0x8B"""
    
    # Estado inicial
    x_reg = 0xC800
    postbyte = 0x8B
    
    print(f"=== Test decode_indexed para postbyte 0x{postbyte:02X} ===")
    print(f"Estado inicial: X = 0x{x_reg:04X}")
    print()
    
    # Analizar postbyte según la lógica del código
    print("Análisis del postbyte:")
    print(f"postbyte = 0x{postbyte:02X} = {postbyte:08b}")
    print(f"(postbyte & 0x80) = 0x{postbyte & 0x80:02X} {'!= 0' if (postbyte & 0x80) != 0 else '== 0'}")
    
    if (postbyte & 0x80) != 0:
        print("→ Entra al primer if (modo extendido)")
        
        low_bits = postbyte & 0x1F
        print(f"(postbyte & 0x1F) = 0x{low_bits:02X} = {low_bits}")
        
        if low_bits in range(0x00, 0x10):
            print(f"→ {low_bits} está en rango 0x00-0x0F, llama decode_indexed_basic")
            print()
            
            # Simular decode_indexed_basic
            print("En decode_indexed_basic:")
            group = postbyte & 0xE0
            offset = postbyte & 0x1F
            
            print(f"group = 0x{group:02X}")
            print(f"offset = 0x{offset:02X} = {offset}")
            
            # Determinar registro base
            base_reg_name = {0x80: "X", 0xA0: "Y", 0xC0: "U", 0xE0: "S"}.get(group, "?")
            base_reg_value = x_reg if group == 0x80 else 0
            
            print(f"Registro base: {base_reg_name} = 0x{base_reg_value:04X}")
            
            # Procesamiento según offset
            if offset in [0x00, 0x01]:
                print(f"→ Offset {offset}: AUTO-INCREMENT")
                if offset == 0x00:
                    new_x = x_reg + 1
                    print(f"   X += 1: 0x{x_reg:04X} → 0x{new_x:04X}")
                elif offset == 0x01:
                    new_x = x_reg + 2
                    print(f"   X += 2: 0x{x_reg:04X} → 0x{new_x:04X}")
                effective_addr = x_reg  # Pre-increment
                
            elif offset in [0x02, 0x03]:
                print(f"→ Offset {offset}: AUTO-DECREMENT")
                if offset == 0x02:
                    new_x = x_reg - 1
                    print(f"   X -= 1: 0x{x_reg:04X} → 0x{new_x:04X}")
                elif offset == 0x03:
                    new_x = x_reg - 2
                    print(f"   X -= 2: 0x{x_reg:04X} → 0x{new_x:04X}")
                effective_addr = new_x  # Post-decrement
                
            elif offset == 0x04:
                print(f"→ Offset {offset}: SIN OFFSET")
                effective_addr = x_reg
                new_x = x_reg  # Sin cambio
                
            elif 0x05 <= offset <= 0x0F:
                print(f"→ Offset {offset}: 5-BIT SIGNED OFFSET")
                # Convertir a signed
                if offset & 0x10:  # bit 4 = 1, negativo
                    signed_offset = offset | 0xFFF0  
                    signed_offset = signed_offset - 0x10000  # a python int negativo
                else:
                    signed_offset = offset
                
                print(f"   Offset con signo: {signed_offset}")
                effective_addr = (x_reg + signed_offset) & 0xFFFF
                new_x = x_reg  # SIN CAMBIO EN X
                print(f"   Dirección efectiva: 0x{x_reg:04X} + {signed_offset} = 0x{effective_addr:04X}")
                print(f"   X NO cambia: 0x{x_reg:04X}")
                
            else:
                print(f"→ Offset {offset}: DESCONOCIDO")
                effective_addr = x_reg
                new_x = x_reg
            
            print()
            print("RESULTADO:")
            print(f"Dirección efectiva: 0x{effective_addr:04X}")
            print(f"X antes: 0x{x_reg:04X}")
            print(f"X después: 0x{new_x:04X}")
            
            if new_x != x_reg:
                print("❌ X FUE MODIFICADO")
            else:
                print("✅ X NO FUE MODIFICADO (correcto)")
        else:
            print(f"→ {low_bits} NO está en rango 0x00-0x0F")
    else:
        print("→ NO entra al primer if")

if __name__ == "__main__":
    test_decode_indexed_0x8B()