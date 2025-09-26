#!/usr/bin/env python3
"""
Script para identificar opcodes faltantes en emulator_v2 de manera precisa
Compara contra la referencia Vectrexy para encontrar gaps específicos
"""

import re
from pathlib import Path

def extract_cpu6809_implemented():
    """Extrae opcodes implementados del archivo cpu6809.rs actual"""
    cpu_path = Path("emulator_v2/src/core/cpu6809.rs")
    
    if not cpu_path.exists():
        print(f"Error: No se encuentra {cpu_path}")
        return set()
    
    implemented = set()
    with open(cpu_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Buscar todos los opcodes en el switch statement
    # Patrón más flexible para capturar todos los casos
    patterns = [
        r'0x([0-9A-Fa-f]{2})\s*=>\s*\{',  # Caso normal
        r'0x([0-9A-Fa-f]{2})\s*=>\s*//.*?\{',  # Con comentario
        r'0x([0-9A-Fa-f]{2})\s*=>\s*self\.op_',  # Llamada directa a función
    ]
    
    for pattern in patterns:
        matches = re.findall(pattern, content, re.MULTILINE)
        for match in matches:
            implemented.add(int(match, 16))
    
    return implemented

def extract_vectrexy_reference():
    """Extrae opcodes válidos de Vectrexy como referencia"""
    vectrexy_path = Path("vectrexy_backup/libs/emulator/include/emulator/CpuOpCodes.h")
    
    if not vectrexy_path.exists():
        print(f"Error: No se encuentra {vectrexy_path}")
        return []
    
    opcodes = []
    with open(vectrexy_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Buscar definiciones de opcodes válidos (no "Illegal")
    pattern = r'{\s*0x([0-9A-Fa-f]+),\s*"([^"]+)",\s*AddressingMode::(\w+)\s*,\s*(\d+),\s*(\d+),\s*"([^"]+)"\s*}'
    matches = re.findall(pattern, content)
    
    for match in matches:
        opcode = int(match[0], 16)
        name = match[1]
        addr_mode = match[2]
        cycles = int(match[3])
        size = int(match[4])
        description = match[5]
        
        # Excluir opcodes ilegales
        if name != "Illegal" and addr_mode != "Illegal":
            opcodes.append({
                'opcode': opcode,
                'name': name,
                'addr_mode': addr_mode,
                'cycles': cycles,
                'size': size,
                'description': description
            })
    
    return opcodes

def categorize_missing_opcodes(missing_opcodes):
    """Categoriza los opcodes faltantes por tipo de instrucción"""
    categories = {
        'Arithmetic': [],
        'Logic': [],
        'Branches': [],
        'Memory': [],
        'Stack': [],
        'System': [],
        'Loads': [],
        'Stores': [],
        'Jumps': [],
        'Other': []
    }
    
    for op in missing_opcodes:
        name = op['name']
        
        if any(x in name for x in ['ADD', 'SUB', 'MUL', 'ADC', 'SBC']):
            categories['Arithmetic'].append(op)
        elif any(x in name for x in ['AND', 'OR', 'EOR', 'BIT', 'COM', 'NEG']):
            categories['Logic'].append(op)
        elif any(x in name for x in ['B', 'LB']) and name not in ['ABX']:
            categories['Branches'].append(op)
        elif any(x in name for x in ['INC', 'DEC', 'CLR', 'TST', 'DAA']):
            categories['Memory'].append(op)
        elif any(x in name for x in ['PSH', 'PUL', 'RTS', 'RTI']):
            categories['Stack'].append(op)
        elif any(x in name for x in ['SWI', 'SYNC', 'CWAI', 'NOP', 'RESET', 'TFR', 'EXG', 'SEX', 'ANDCC', 'ORCC']):
            categories['System'].append(op)
        elif name.startswith('LD'):
            categories['Loads'].append(op)
        elif name.startswith('ST'):
            categories['Stores'].append(op)
        elif any(x in name for x in ['JMP', 'JSR', 'BSR']):
            categories['Jumps'].append(op)
        else:
            categories['Other'].append(op)
    
    return categories

def main():
    print("=== IDENTIFICACIÓN PRECISA DE OPCODES FALTANTES ===\n")
    
    # Extraer datos
    implemented = extract_cpu6809_implemented()
    vectrexy_opcodes = extract_vectrexy_reference()
    
    print(f"Opcodes implementados en cpu6809.rs: {len(implemented)}")
    print(f"Opcodes válidos en Vectrexy: {len(vectrexy_opcodes)}")
    
    # Identificar faltantes
    vectrexy_codes = {op['opcode'] for op in vectrexy_opcodes}
    missing_codes = vectrexy_codes - implemented
    
    missing_opcodes = [op for op in vectrexy_opcodes if op['opcode'] in missing_codes]
    missing_opcodes.sort(key=lambda x: x['opcode'])
    
    print(f"Opcodes faltantes: {len(missing_opcodes)}")
    
    if len(implemented) > 0 and len(vectrexy_codes) > 0:
        compliance = (len(implemented & vectrexy_codes) / len(vectrexy_codes)) * 100
        print(f"Compliance actual: {compliance:.1f}%")
    
    # Categorizar faltantes
    categories = categorize_missing_opcodes(missing_opcodes)
    
    print(f"\n=== OPCODES FALTANTES POR CATEGORÍA ===")
    
    for category, opcodes in categories.items():
        if opcodes:
            print(f"\n{category} ({len(opcodes)}):")
            for op in opcodes[:10]:  # Primeros 10 de cada categoría
                print(f"  0x{op['opcode']:02X}: {op['name']:<12} {op['addr_mode']:<10} - {op['description']}")
            if len(opcodes) > 10:
                print(f"  ... y {len(opcodes) - 10} más")
    
    # Prioridad alta - opcodes críticos
    high_priority = []
    critical_names = ['PSHS', 'PULS', 'PSHU', 'PULU', 'RTS', 'JSR', 'BSR', 'JMP', 
                      'BRA', 'BEQ', 'BNE', 'TFR', 'EXG', 'CWAI', 'SWI', 'RTI']
    
    for op in missing_opcodes:
        if any(crit in op['name'] for crit in critical_names):
            high_priority.append(op)
    
    if high_priority:
        print(f"\n=== OPCODES DE ALTA PRIORIDAD ({len(high_priority)}) ===")
        for op in high_priority:
            print(f"🔥 0x{op['opcode']:02X}: {op['name']:<12} {op['addr_mode']:<10}")
    
    # Opcodes simples para empezar
    simple_opcodes = []
    simple_names = ['NOP', 'CLRA', 'CLRB', 'TSTA', 'TSTB', 'INCA', 'INCB', 'DECA', 'DECB']
    
    for op in missing_opcodes:
        if op['name'] in simple_names:
            simple_opcodes.append(op)
    
    if simple_opcodes:
        print(f"\n=== OPCODES SIMPLES PARA EMPEZAR ({len(simple_opcodes)}) ===")
        for op in simple_opcodes:
            print(f"✅ 0x{op['opcode']:02X}: {op['name']:<12} {op['addr_mode']:<10}")

if __name__ == "__main__":
    main()