#!/usr/bin/env python3
"""
Analiza específicamente el comportamiento de decode_indexed para postbyte 0x8B
"""

def analyze_postbyte_0x8B():
    """
    Analiza cómo se debería procesar el postbyte 0x8B (CLR 11,X)
    """
    post = 0x8B
    
    print(f"Análisis del postbyte 0x8B:")
    print(f"post = 0x{post:02X} = {post:08b}")
    print()
    
    # Verificar las condiciones de la función decode_indexed
    if (post & 0x80) != 0:
        print("✓ (post & 0x80) != 0, entra al primer if")
        
        post_masked = post & 0x1F
        print(f"post & 0x1F = 0x{post_masked:02X} = {post_masked}")
        
        if post_masked in range(0x00, 0x10):  # 0x00 a 0x0F
            print(f"✓ {post_masked} está en rango 0x00-0x0F")
            print("→ DEBERÍA ejecutar: return self.decode_indexed_basic(post,self.x,self.y,self.u,self.s)")
            print()
            
            # Analizar qué hace decode_indexed_basic
            print("En decode_indexed_basic:")
            group = post & 0xE0
            offset = post & 0x1F
            print(f"group = post & 0xE0 = 0x{group:02X}")
            print(f"offset = post & 0x1F = 0x{offset:02X} = {offset}")
            
            # Determinar registro base
            base_reg = {0x80: "X", 0xA0: "Y", 0xC0: "U", 0xE0: "S"}.get(group, "?")
            print(f"Registro base: {base_reg}")
            
            # Para 0x05-0x0F: 5-bit signed offset
            if 0x05 <= offset <= 0x0F:
                print(f"✓ Offset {offset} en rango 5-bit signed (0x05-0x0F)")
                
                # Calcular offset con signo
                if offset & 0x10:  # bit 4 set = negativo
                    signed_offset = offset | 0xFFF0  # extender signo
                    signed_offset = signed_offset - 0x10000  # convertir a negativo
                else:
                    signed_offset = offset
                    
                print(f"Offset con signo: {signed_offset}")
                print(f"→ Dirección efectiva = {base_reg} + {signed_offset}")
                print(f"→ NO debe modificar {base_reg}")
            else:
                print(f"Offset {offset} NO en rango 5-bit signed")
        else:
            print(f"✗ {post_masked} NO está en rango 0x00-0x0F")
    else:
        print("✗ (post & 0x80) == 0, NO entra al primer if")
    
    print()
    print("CONCLUSIÓN:")
    print("Para CLR 11,X con postbyte 0x8B:")
    print("- Debería usar decode_indexed_basic")
    print("- Debería calcular X + 11 sin modificar X")
    print("- Si X se está incrementando, hay un bug en el código")

if __name__ == "__main__":
    analyze_postbyte_0x8B()