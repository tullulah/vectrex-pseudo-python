#!/usr/bin/env python3
import os

rom_path = "/Users/daniel/projects/vectrex-pseudo-python/examples/test_callgraph/build/test_callgraph.bin"
if os.path.exists(rom_path):
    data = open(rom_path, 'rb').read()
    print(f"Tamaño: {len(data)} bytes ({len(data)//1024}KB)")
    
    # Check offset 0
    first_bytes = data[0:11]
    header_str = first_bytes[:-1].decode('ascii', errors='replace')
    checksum = first_bytes[-1]
    print(f"Header en offset 0x0: {repr(first_bytes)}")
    print(f"  Signature: '{header_str}' + 0x{checksum:02X}")
    
    # Check if signature matches
    if header_str == "g GCE 1982" and checksum == 0x80:
        print("✅ HEADER VÁLIDO en offset 0x0000")
    else:
        print("❌ Header no válido")
        
    # Also check around offset 0x5A7 (old location)
    old_offset = 0x5A7
    if len(data) > old_offset + 11:
        old_bytes = data[old_offset:old_offset+11]
        old_str = old_bytes[:-1].decode('ascii', errors='replace')
        print(f"\nBúsqueda en offset 0x{old_offset:04X} (ubicación anterior):")
        if old_str == "g GCE 1982":
            print(f"  ⚠️  Encontrado header ANTIGUO - PROBLEMA NO RESUELTO")
        else:
            print(f"  ✅ Sin header antiguo - OK")
else:
    print(f"❌ No existe: {rom_path}")
