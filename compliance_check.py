#!/usr/bin/env python3
"""
Análisis de Compliance entre emulator_v2 y Vectrexy
Este script compara los opcodes implementados entre ambos emuladores
"""

import re
import sys
from pathlib import Path

def parse_vectrexy_opcodes(file_path):
    """Parse Vectrexy C++ opcode definitions"""
    opcodes = {}
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        # Regex para capturar definiciones de opcodes C++
        pattern = r'{\s*0x([0-9A-Fa-f]+),\s*"([^"]+)",\s*AddressingMode::(\w+)\s*,\s*(\d+),\s*(\d+),\s*"([^"]+)"\s*}'
        matches = re.findall(pattern, content, re.MULTILINE)
        
        for match in matches:
            opcode = int(match[0], 16)
            name = match[1]
            addr_mode = match[2]
            cycles = int(match[3])
            size = int(match[4])
            description = match[5]
            
            opcodes[opcode] = {
                'name': name,
                'addr_mode': addr_mode,
                'cycles': cycles,
                'size': size,
                'description': description
            }
    except Exception as e:
        print(f"Error parsing Vectrexy opcodes: {e}")
        return {}
    
    return opcodes

def parse_rust_opcodes(file_path):
    """Parse emulator_v2 Rust opcode definitions"""
    opcodes = {}
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        # Regex para capturar definiciones de opcodes en Rust
        pattern = r'0x([0-9A-Fa-f]+)\s*=>\s*CpuOp\s*{\s*op_code:\s*0x[0-9A-Fa-f]+,\s*name:\s*"([^"]+)",\s*addr_mode:\s*AddressingMode::(\w+),\s*cycles:\s*(\d+),\s*size:\s*(\d+),\s*description:\s*"([^"]+)"\s*}'
        matches = re.findall(pattern, content)
        
        for match in matches:
            opcode = int(match[0], 16)
            name = match[1]
            addr_mode = match[2]
            cycles = int(match[3])
            size = int(match[4])
            description = match[5]
            
            opcodes[opcode] = {
                'name': name,
                'addr_mode': addr_mode,
                'cycles': cycles,
                'size': size,
                'description': description
            }
    except Exception as e:
        print(f"Error parsing Rust opcodes: {e}")
        return {}
    
    return opcodes

def main():
    # Rutas de archivos
    vectrexy_path = Path("vectrexy_backup/libs/emulator/include/emulator/CpuOpCodes.h")
    rust_path = Path("emulator_v2/src/core/cpu_op_codes.rs")
    
    if not vectrexy_path.exists():
        print(f"Error: No se encuentra {vectrexy_path}")
        return 1
        
    if not rust_path.exists():
        print(f"Error: No se encuentra {rust_path}")
        return 1
    
    print("=== ANÁLISIS DE COMPLIANCE VECTREXY vs EMULATOR_V2 ===\n")
    
    # Parse opcodes
    vectrexy_opcodes = parse_vectrexy_opcodes(vectrexy_path)
    rust_opcodes = parse_rust_opcodes(rust_path)
    
    print(f"Opcodes en Vectrexy: {len(vectrexy_opcodes)}")
    print(f"Opcodes en emulator_v2: {len(rust_opcodes)}")
    
    # Análisis de compliance
    implemented = set(rust_opcodes.keys())
    total_vectrexy = set(vectrexy_opcodes.keys())
    
    missing = total_vectrexy - implemented
    extra = implemented - total_vectrexy
    common = implemented & total_vectrexy
    
    print(f"\nComunes: {len(common)}")
    print(f"Faltantes en emulator_v2: {len(missing)}")
    print(f"Extra en emulator_v2: {len(extra)}")
    
    if len(total_vectrexy) > 0:
        compliance_percentage = (len(common) / len(total_vectrexy)) * 100
        print(f"\n🎯 COMPLIANCE TOTAL: {compliance_percentage:.1f}%")
    
    # Mostrar opcodes faltantes (solo algunos)
    if missing:
        print(f"\n❌ OPCODES FALTANTES (primeros 20):")
        sorted_missing = sorted(missing)
        for i, opcode in enumerate(sorted_missing[:20]):
            vectrexy_info = vectrexy_opcodes[opcode]
            print(f"  0x{opcode:02X}: {vectrexy_info['name']} - {vectrexy_info['description']}")
        if len(missing) > 20:
            print(f"  ... y {len(missing) - 20} más")
    
    # Verificar consistencia en opcodes comunes
    inconsistencies = []
    for opcode in common:
        v_op = vectrexy_opcodes[opcode]
        r_op = rust_opcodes[opcode]
        
        issues = []
        if v_op['name'] != r_op['name']:
            issues.append(f"name: '{v_op['name']}' vs '{r_op['name']}'")
        if v_op['addr_mode'] != r_op['addr_mode']:
            issues.append(f"addr_mode: '{v_op['addr_mode']}' vs '{r_op['addr_mode']}'")
        if v_op['cycles'] != r_op['cycles']:
            issues.append(f"cycles: {v_op['cycles']} vs {r_op['cycles']}")
        if v_op['size'] != r_op['size']:
            issues.append(f"size: {v_op['size']} vs {r_op['size']}")
            
        if issues:
            inconsistencies.append((opcode, issues))
    
    if inconsistencies:
        print(f"\n⚠️  INCONSISTENCIAS DETECTADAS ({len(inconsistencies)}):")
        for opcode, issues in inconsistencies[:10]:  # Solo primeros 10
            print(f"  0x{opcode:02X}: {', '.join(issues)}")
        if len(inconsistencies) > 10:
            print(f"  ... y {len(inconsistencies) - 10} más")
    else:
        print(f"\n✅ Todos los opcodes comunes son consistentes!")
    
    # Categorías de opcodes implementados
    categories = {}
    for opcode, info in rust_opcodes.items():
        name_base = info['name'].rstrip('ABXY0123')  # Remove register suffixes
        if name_base not in categories:
            categories[name_base] = 0
        categories[name_base] += 1
    
    print(f"\n📊 CATEGORÍAS DE INSTRUCCIONES IMPLEMENTADAS:")
    for category, count in sorted(categories.items()):
        print(f"  {category}: {count}")
    
    print(f"\n🏆 RESUMEN:")
    print(f"  • {len(common)} opcodes idénticos a Vectrexy")
    print(f"  • {len(inconsistencies)} inconsistencias menores")
    print(f"  • {len(missing)} opcodes por implementar")
    print(f"  • {compliance_percentage:.1f}% compliance total")
    
    if compliance_percentage >= 95:
        print("  🌟 EXCELENTE compliance!")
    elif compliance_percentage >= 85:
        print("  👍 Muy buena compliance")
    elif compliance_percentage >= 75:
        print("  👌 Buena compliance")
    else:
        print("  🔨 Necesita más trabajo")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())