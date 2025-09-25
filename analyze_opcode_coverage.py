#!/usr/bin/env python3
"""
Script para analizar la cobertura de opcodes en el emulador v2.
Extrae los opcodes implementados en cpu6809.rs y mapea contra los tests existentes.
"""

import re
import os
import pathlib
from collections import defaultdict, Counter

def extract_implemented_opcodes(cpu_file):
    """Extrae todos los opcodes implementados del archivo cpu6809.rs"""
    opcodes = []
    
    with open(cpu_file, 'r', encoding='utf-8') as f:
        content = f.read()
        
    # Buscar patrones como "0x[HEX] => {"
    opcode_pattern = r'0x([0-9A-Fa-f]{2})\s*=>\s*{'
    matches = re.findall(opcode_pattern, content)
    
    for match in matches:
        opcodes.append(match.upper())
    
    return sorted(set(opcodes))

def extract_opcodes_from_tests(tests_dir):
    """Extrae opcodes mencionados en archivos de test"""
    test_opcodes = defaultdict(list)
    
    for test_file in pathlib.Path(tests_dir).glob('**/*test*.rs'):
        with open(test_file, 'r', encoding='utf-8') as f:
            content = f.read()
            
        # Buscar menciones de opcodes en comentarios y código
        opcode_patterns = [
            r'0x([0-9A-Fa-f]{2})',  # Formato 0xXX
            r'\\x([0-9A-Fa-f]{2})',  # Formato \xXX en bytes
        ]
        
        for pattern in opcode_patterns:
            matches = re.findall(pattern, content)
            for match in matches:
                test_opcodes[match.upper()].append(test_file.name)
    
    return test_opcodes

def categorize_opcodes(opcodes):
    """Categoriza opcodes por tipo de instrucción"""
    categories = {
        'Load/Store': [],
        'Arithmetic': [],
        'Logic': [],
        'Branch': [],
        'Stack': [],
        'System': [],
        'Transfer': [],
        'Test/Compare': [],
        'Shift/Rotate': [],
        'Other': []
    }
    
    # Mapeo básico basado en rangos conocidos del 6809
    for opcode in opcodes:
        hex_val = int(opcode, 16)
        
        if hex_val in [0x86, 0x96, 0xA6, 0xB6, 0xC6, 0xD6, 0xE6, 0xF6,  # LDA, LDB
                       0x8E, 0x9E, 0xAE, 0xBE, 0xCE, 0xDE, 0xEE, 0xFE,  # LDX, LDU
                       0x97, 0xA7, 0xB7, 0xD7, 0xE7, 0xF7,               # STA, STB
                       0x9F, 0xAF, 0xBF, 0xDD, 0xED, 0xFD,               # STX, STD
                       0xDF, 0xEF, 0xFF]:                                 # STU
            categories['Load/Store'].append(opcode)
        elif hex_val in [0x80, 0x81, 0x82, 0x8B, 0xC0, 0xC1, 0xC2, 0xCB,  # SUB, CMP, SBC, ADD
                         0x84, 0x85, 0x88, 0x89, 0x8A,                     # AND, BIT, EOR, ADC, OR
                         0xC4, 0xC5, 0xC8, 0xC9, 0xCA, 0xCC,               # AND, BIT, EOR, ADC, OR, LDD
                         0x8C, 0x90, 0x94, 0x98, 0x9C]:                    # CMP variations
            categories['Arithmetic'].append(opcode)
        elif (hex_val >= 0x20 and hex_val <= 0x2F):  # Branch instructions
            categories['Branch'].append(opcode)
        elif hex_val in [0x34, 0x35, 0x36, 0x37, 0x39]:  # PSHS, PULS, PSHU, PULU, RTS
            categories['Stack'].append(opcode)
        elif hex_val in [0x00, 0x03, 0x04, 0x12, 0x13, 0x16, 0x17, 0x19]:  # System opcodes
            categories['System'].append(opcode)
        elif hex_val in [0x30, 0x31, 0x32, 0x33, 0x8D]:  # LEA, BSR, JSR
            categories['Transfer'].append(opcode)
        elif (hex_val >= 0x40 and hex_val <= 0x7F):  # Shift/Rotate range
            categories['Shift/Rotate'].append(opcode)
        else:
            categories['Other'].append(opcode)
    
    return categories

def main():
    # Rutas
    cpu_file = r'c:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\emulator_v2\src\core\cpu6809.rs'
    tests_dir = r'c:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\emulator_v2\tests'
    
    print("=== ANÁLISIS DE COBERTURA DE OPCODES ===\n")
    
    # Extraer opcodes implementados
    implemented = extract_implemented_opcodes(cpu_file)
    print(f"📊 Opcodes implementados en cpu6809.rs: {len(implemented)}")
    
    # Extraer opcodes en tests
    test_opcodes = extract_opcodes_from_tests(tests_dir)
    print(f"📊 Opcodes mencionados en tests: {len(test_opcodes)}")
    
    # Categorizar opcodes
    categories = categorize_opcodes(implemented)
    
    print("\n=== CATEGORIZACIÓN DE OPCODES IMPLEMENTADOS ===")
    for category, opcodes in categories.items():
        if opcodes:
            print(f"\n{category} ({len(opcodes)}):")
            for opcode in opcodes:
                coverage = "✅" if opcode in test_opcodes else "❌"
                test_files = ", ".join(set(test_opcodes.get(opcode, [])))[:50]
                if test_files:
                    print(f"  0x{opcode} {coverage} ({test_files})")
                else:
                    print(f"  0x{opcode} {coverage}")
    
    # Opcodes sin cobertura
    uncovered = [op for op in implemented if op not in test_opcodes]
    print(f"\n=== OPCODES SIN COBERTURA DE TEST ({len(uncovered)}) ===")
    for opcode in uncovered:
        print(f"❌ 0x{opcode}")
    
    # Estadísticas finales
    covered = len(implemented) - len(uncovered)
    coverage_pct = (covered / len(implemented)) * 100 if implemented else 0
    
    print(f"\n=== RESUMEN FINAL ===")
    print(f"Opcodes implementados: {len(implemented)}")
    print(f"Opcodes con tests: {covered}")
    print(f"Opcodes sin tests: {len(uncovered)}")
    print(f"Cobertura de tests: {coverage_pct:.1f}%")
    
    # Lista completa de opcodes implementados ordenados
    print(f"\n=== TODOS LOS OPCODES IMPLEMENTADOS ===")
    for i, opcode in enumerate(implemented):
        if i % 8 == 0:
            print()
        print(f"0x{opcode}", end="  ")
    print()

if __name__ == "__main__":
    main()