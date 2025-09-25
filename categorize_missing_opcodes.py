#!/usr/bin/env python3
"""
Análisis detallado de opcodes faltantes con categorización por tipo
"""

import re

def analyze_vectrexy_opcodes():
    """Analiza los opcodes en Vectrexy para categorizar los faltantes"""
    
    # Lista de opcodes faltantes del análisis anterior
    missing_opcodes = [
        0x10, 0x11, 0xA0, 0xA2, 0xA4, 0xA9, 0xAB, 0xB2, 0xB9, 
        0xC2, 0xC3, 0xC9, 0xCC, 0xD2, 0xD3, 0xD9, 0xDB, 0xDC,
        0xE2, 0xE3, 0xE4, 0xE8, 0xE9, 0xEA, 0xEB, 0xEC,
        0xF2, 0xF3, 0xF8, 0xF9, 0xFB, 0xFC
    ]
    
    categories = {
        'ARITHMETIC': [],
        'COMPARE': [], 
        'LOGIC': [],
        'MEMORY': [],
        'BRANCH': [],
        'OTHER': []
    }
    
    try:
        with open('vectrexy_backup/libs/emulator/src/Cpu.cpp', 'r') as f:
            content = f.read()
            
        for opcode in missing_opcodes:
            opcode_hex = f"0x{opcode:02X}"
            
            # Buscar el patrón case 0xXX: seguido de la operación
            pattern = rf"case {opcode_hex}:\s*(\w+)<[^>]+>\([^)]+\);"
            match = re.search(pattern, content, re.MULTILINE)
            
            if match:
                operation = match.group(1)
                
                # Categorizar por tipo de operación
                if operation in ['OpSUB', 'OpSBC', 'OpADD', 'OpADC']:
                    categories['ARITHMETIC'].append((opcode_hex, operation))
                elif operation in ['OpCMP']:
                    categories['COMPARE'].append((opcode_hex, operation))
                elif operation in ['OpAND', 'OpOR', 'OpEOR']:
                    categories['LOGIC'].append((opcode_hex, operation))
                elif operation in ['OpLD', 'OpST']:
                    categories['MEMORY'].append((opcode_hex, operation))
                else:
                    categories['OTHER'].append((opcode_hex, operation))
            else:
                # Si no encontramos el patrón, buscar cualquier mención
                if f"case {opcode_hex}:" in content:
                    categories['OTHER'].append((opcode_hex, "FOUND_BUT_UNKNOWN"))
                else:
                    categories['OTHER'].append((opcode_hex, "NOT_FOUND"))
    
    except FileNotFoundError:
        print("Error: No se encontró el archivo Vectrexy")
        return None
    
    return categories

def print_categorized_opcodes(categories):
    """Imprime los opcodes categorizados"""
    print("\n=== OPCODES FALTANTES CATEGORIZADOS ===")
    
    for category, opcodes in categories.items():
        if opcodes:
            print(f"\n{category}:")
            for opcode, operation in opcodes:
                print(f"  {opcode}: {operation}")
    
    # Sugerir grupos prioritarios
    print("\n=== SUGERENCIAS DE GRUPOS PRIORITARIOS ===")
    
    total_arithmetic = len(categories['ARITHMETIC'])
    total_compare = len(categories['COMPARE'])  
    total_logic = len(categories['LOGIC'])
    total_memory = len(categories['MEMORY'])
    
    if total_arithmetic > 0:
        print(f"1. GRUPO ARITMÉTICO: {total_arithmetic} opcodes - Prioridad ALTA")
        for opcode, op in categories['ARITHMETIC']:
            print(f"   {opcode} ({op})")
    
    if total_compare > 0:
        print(f"2. GRUPO COMPARACIÓN: {total_compare} opcodes - Prioridad ALTA")
        for opcode, op in categories['COMPARE']:
            print(f"   {opcode} ({op})")
    
    if total_logic > 0:
        print(f"3. GRUPO LÓGICO: {total_logic} opcodes - Prioridad MEDIA")
        for opcode, op in categories['LOGIC']:
            print(f"   {opcode} ({op})")
    
    if total_memory > 0:
        print(f"4. GRUPO MEMORIA: {total_memory} opcodes - Prioridad MEDIA")
        for opcode, op in categories['MEMORY']:
            print(f"   {opcode} ({op})")

if __name__ == "__main__":
    categories = analyze_vectrexy_opcodes()
    if categories:
        print_categorized_opcodes(categories)