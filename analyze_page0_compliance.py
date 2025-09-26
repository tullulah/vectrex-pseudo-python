#!/usr/bin/env python3
"""
Script para identificar exactamente qué opcodes de Page 0 faltan en emulator_v2
comparando con la referencia Vectrexy CpuOpCodes.h
"""

import re

def extract_vectrexy_page0_opcodes():
    """Extrae opcodes válidos (no Illegal) de Vectrexy Page 0"""
    vectrexy_path = "vectrexy_backup/libs/emulator/include/emulator/CpuOpCodes.h"
    
    valid_opcodes = []
    with open(vectrexy_path, 'r') as f:
        content = f.read()
    
    # Pattern para extraer opcodes de Page 0
    pattern = r'{\s*0x([0-9A-Fa-f]{2}),\s*"([^"]+)",\s*AddressingMode::(\w+)\s*,\s*(\d+),\s*(\d+),\s*"([^"]+)"\s*}'
    matches = re.findall(pattern, content)
    
    for match in matches:
        opcode = int(match[0], 16)
        name = match[1]
        addr_mode = match[2]
        cycles = int(match[3])
        size = int(match[4])
        description = match[5]
        
        # Solo incluir opcodes válidos (no Illegal y de Page 0, 0x00-0xFF)
        if addr_mode != "Illegal" and name != "Illegal" and opcode <= 0xFF:
            valid_opcodes.append({
                'opcode': opcode,
                'name': name,
                'addr_mode': addr_mode,
                'cycles': cycles,
                'size': size,
                'description': description
            })
    
    return sorted(valid_opcodes, key=lambda x: x['opcode'])

def extract_implemented_page0_opcodes():
    """Extrae opcodes implementados en cpu6809.rs"""
    cpu_path = "emulator_v2/src/core/cpu6809.rs"
    
    implemented = set()
    with open(cpu_path, 'r') as f:
        content = f.read()
    
    # Buscar patrones como "0x?? => {" en Page 0 switch
    # Necesitamos ser más específicos para encontrar solo Page 0
    page0_section = re.search(r'match cpu_op_page \{.*?0 => \{.*?match opcode_byte \{(.*?)\}.*?1 => \{', content, re.DOTALL)
    
    if page0_section:
        page0_matches = re.findall(r'0x([0-9A-Fa-f]{2})\s*=>', page0_section.group(1))
        implemented = set(int(m, 16) for m in page0_matches)
    
    return implemented

def main():
    print("=== ANÁLISIS DE COMPLIANCE PAGE 0 vs VECTREXY ===\n")
    
    vectrexy_opcodes = extract_vectrexy_page0_opcodes()
    implemented_opcodes = extract_implemented_page0_opcodes()
    
    print(f"Vectrexy Page 0 opcodes válidos: {len(vectrexy_opcodes)}")
    print(f"Emulator_v2 opcodes implementados: {len(implemented_opcodes)}")
    
    # Encontrar faltantes
    vectrexy_set = set(op['opcode'] for op in vectrexy_opcodes)
    missing = vectrexy_set - implemented_opcodes
    
    print(f"\n=== OPCODES FALTANTES EN PAGE 0: {len(missing)} ===")
    
    missing_details = []
    for op in vectrexy_opcodes:
        if op['opcode'] in missing:
            missing_details.append(op)
    
    for op in sorted(missing_details, key=lambda x: x['opcode']):
        print(f"  0x{op['opcode']:02X}: {op['name']:<12} {op['addr_mode']:<10} - {op['description']}")
    
    # Estadísticas finales
    compliance = (len(implemented_opcodes) / len(vectrexy_set)) * 100
    print(f"\n=== COMPLIANCE PAGE 0 ===")
    print(f"Implementados: {len(implemented_opcodes)}/{len(vectrexy_set)}")
    print(f"Compliance: {compliance:.1f}%")
    print(f"Faltantes: {len(missing)}")

if __name__ == "__main__":
    main()