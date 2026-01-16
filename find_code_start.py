#!/usr/bin/env python3
import pathlib

snapshot = pathlib.Path('/Users/daniel/projects/vectrex-pseudo-python/examples/test_callgraph/rom_snapshot_bank0_and_31.bin').read_bytes()

print("🔍 Buscando inicio de código en Bank 0...\n")

# Buscar primeros opcodes M6809
found_code = False
for i in range(0x0100):
    b = snapshot[i]
    if b in [0x86, 0xBD, 0xB7, 0x7E, 0x8D, 0x84]:
        next8 = ' '.join(f'{snapshot[i+j]:02X}' for j in range(8) if i+j < len(snapshot))
        print(f"0x{i:04X}: {next8}")
        if not found_code:
            print(f"  ✓ Primer código M6809 encontrado en offset 0x{i:04X}")
            found_code = True

print("\n=== MAPA DE CONTENIDO DE BANK 0 ===\n")
print("$0000: Strings/datos (g GCE 1982, CALL GRAPH TEST)")
print("$0100: Donde esperamos encontrar código")
print("$4000: Bank 31 (código real con JSR/RTS)\n")

print("=== PROBLEMA POSIBLE ===")
print("❌ Si el código multibank comienza en 0x0000 con strings")
print("   → Es el issue: Strings NO deben estar en 0x0000")
print("   → Deben estar en ROM data section")
print("   → Código debe comenzar en 0x0000\n")

print("⚠️  CONCLUSIÓN: El compilador está poniendo strings al inicio de Bank 0")
print("    Esto causa que la CPU execute 'g' (0x67) como código, ¡crash!")
