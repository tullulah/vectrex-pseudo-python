#!/usr/bin/env python3
"""
Análisis detallado de gaps entre definiciones de tablas y implementación funcional.
Este script compara lo que está definido en cpu_op_codes.rs vs lo que está implementado en cpu6809.rs
"""

import re
import os

def get_defined_opcodes():
    """Obtiene todos los opcodes definidos en las tablas de cpu_op_codes.rs"""
    opcodes_file = r'emulator_v2\src\core\cpu_op_codes.rs'
    
    if not os.path.exists(opcodes_file):
        print(f"Error: No se encuentra {opcodes_file}")
        return set(), set(), set()
    
    with open(opcodes_file, 'r') as f:
        content = f.read()
    
    defined_page0 = set()
    defined_page1 = set()
    defined_page2 = set()
    
    # Buscar definiciones en formato: 0xXX => CpuOp { op_code: 0xXX, ...
    pattern = r'0x([0-9A-Fa-f]{2})\s*=>\s*CpuOp\s*{'
    matches = re.findall(pattern, content)
    
    for match in matches:
        opcode = int(match, 16)
        # Determinar la página por contexto (esto es simplificado)
        if opcode <= 0xFF:
            defined_page0.add(opcode)
    
    # Para páginas 1 y 2, buscar en funciones específicas
    page1_pattern = r'fn lookup_cpu_op_page1.*?0x([0-9A-Fa-f]{2})\s*=>\s*CpuOp'
    page2_pattern = r'fn lookup_cpu_op_page2.*?0x([0-9A-Fa-f]{2})\s*=>\s*CpuOp'
    
    # Extraer sección de page1
    page1_section = re.search(r'fn lookup_cpu_op_page1.*?(?=fn|\Z)', content, re.DOTALL)
    if page1_section:
        page1_matches = re.findall(r'0x([0-9A-Fa-f]{2})\s*=>\s*CpuOp', page1_section.group(0))
        for match in page1_matches:
            defined_page1.add(int(match, 16))
    
    # Extraer sección de page2
    page2_section = re.search(r'fn lookup_cpu_op_page2.*?(?=fn|\Z)', content, re.DOTALL)
    if page2_section:
        page2_matches = re.findall(r'0x([0-9A-Fa-f]{2})\s*=>\s*CpuOp', page2_section.group(0))
        for match in page2_matches:
            defined_page2.add(int(match, 16))
    
    return defined_page0, defined_page1, defined_page2

def get_implemented_opcodes():
    """Obtiene todos los opcodes funcionalmente implementados en cpu6809.rs"""
    cpu_file = r'emulator_v2\src\core\cpu6809.rs'
    
    if not os.path.exists(cpu_file):
        print(f"Error: No se encuentra {cpu_file}")
        return set(), set(), set()
    
    with open(cpu_file, 'r') as f:
        content = f.read()
    
    implemented_page0 = set()
    implemented_page1 = set()
    implemented_page2 = set()
    
    # Buscar casos en formato: 0xXX => {
    pattern = r'0x([0-9A-Fa-f]{2})\s*=>\s*{'
    matches = re.findall(pattern, content)
    
    for match in matches:
        opcode = int(match, 16)
        implemented_page0.add(opcode)
    
    # Para pages 1 y 2, buscar en secciones específicas de manejo de páginas
    # (Este análisis sería más complejo - por ahora solo page 0)
    
    return implemented_page0, implemented_page1, implemented_page2

def main():
    print("=== ANÁLISIS DE GAP: DEFINICIONES vs IMPLEMENTACIONES ===\n")
    
    defined_p0, defined_p1, defined_p2 = get_defined_opcodes()
    implemented_p0, implemented_p1, implemented_p2 = get_implemented_opcodes()
    
    print(f"Page 0 - Definidos en tablas: {len(defined_p0)}")
    print(f"Page 0 - Implementados funcionalmente: {len(implemented_p0)}")
    print(f"Page 1 - Definidos en tablas: {len(defined_p1)}")
    print(f"Page 2 - Definidos en tablas: {len(defined_p2)}")
    
    # Encontrar opcodes definidos pero no implementados
    defined_not_implemented = defined_p0 - implemented_p0
    
    print(f"\n=== OPCODES DEFINIDOS PERO NO IMPLEMENTADOS (Page 0): {len(defined_not_implemented)} ===")
    for opcode in sorted(defined_not_implemented):
        print(f"  0x{opcode:02X}")
    
    # Encontrar opcodes implementados pero no definidos (probablemente error)
    implemented_not_defined = implemented_p0 - defined_p0
    
    print(f"\n=== OPCODES IMPLEMENTADOS PERO NO DEFINIDOS (Page 0): {len(implemented_not_defined)} ===")
    for opcode in sorted(implemented_not_defined):
        print(f"  0x{opcode:02X}")
    
    # Encontrar opcodes completamente implementados (definidos Y funcionalmente implementados)
    fully_implemented = defined_p0 & implemented_p0
    
    print(f"\n=== OPCODES COMPLETAMENTE IMPLEMENTADOS (Page 0): {len(fully_implemented)} ===")
    
    # Mostrar algunos ejemplos de missing importantes
    important_missing = {
        0x18: "Posible missing opcode",
        0x19: "DAA - Decimal Adjust A",
        0x1B: "SEX - Sign Extend", 
        0x1D: "SEX - Sign Extend (dup check)"
    }
    
    print(f"\n=== OPCODES FALTANTES IMPORTANTES ===")
    for opcode, desc in important_missing.items():
        if opcode in defined_not_implemented:
            print(f"  0x{opcode:02X}: {desc} - DEFINIDO pero NO implementado")
        elif opcode not in defined_p0:
            print(f"  0x{opcode:02X}: {desc} - NO definido en tablas")
        else:
            print(f"  0x{opcode:02X}: {desc} - OK (implementado)")

if __name__ == "__main__":
    main()