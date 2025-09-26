#!/usr/bin/env python3
"""
Script para comparar exactamente los opcodes implementados en Vectrexy vs emulator_v2
Genera una tabla HTML completa para verificar 1:1 la implementación
"""

import re
from pathlib import Path

def extract_vectrexy_opcodes():
    """Extrae todos los opcodes de Vectrexy desde CpuOpCodes.h"""
    vectrexy_file = Path("vectrexy_backup/libs/emulator/include/emulator/CpuOpCodes.h")
    
    if not vectrexy_file.exists():
        print(f"❌ No se encontró {vectrexy_file}")
        return {}, {}, {}
    
    with open(vectrexy_file, 'r') as f:
        content = f.read()
    
    # Extraer Page 0
    page0_pattern = r'inline constexpr CpuOp CpuOpsPage0\[\] = \{.*?\};'
    page0_match = re.search(page0_pattern, content, re.DOTALL)
    
    page0_opcodes = {}
    if page0_match:
        page0_content = page0_match.group(0)
        # Buscar líneas como: { 0x00, "NEG", AddressingMode::Direct, 6, 2, "Negate memory location" },
        opcode_pattern = r'\{\s*0x([0-9A-Fa-f]{2}),\s*"([^"]+)",\s*AddressingMode::(\w+)\s*,\s*(\d+),\s*(\d+),\s*"([^"]+)"\s*\}'
        matches = re.findall(opcode_pattern, page0_content)
        
        for match in matches:
            opcode = int(match[0], 16)
            name = match[1].strip()
            addr_mode = match[2]
            cycles = int(match[3])
            size = int(match[4])
            description = match[5]
            
            if addr_mode != "Illegal":  # Solo incluir opcodes válidos
                page0_opcodes[opcode] = {
                    'name': name,
                    'addr_mode': addr_mode,
                    'cycles': cycles,
                    'size': size,
                    'description': description
                }
    
    # Extraer Page 1
    page1_pattern = r'inline constexpr CpuOp CpuOpsPage1\[\] = \{.*?\};'
    page1_match = re.search(page1_pattern, content, re.DOTALL)
    
    page1_opcodes = {}
    if page1_match:
        page1_content = page1_match.group(0)
        matches = re.findall(opcode_pattern, page1_content)
        
        for match in matches:
            opcode = int(match[0], 16)
            name = match[1].strip()
            addr_mode = match[2]
            cycles = int(match[3])
            size = int(match[4])
            description = match[5]
            
            page1_opcodes[opcode] = {
                'name': name,
                'addr_mode': addr_mode,
                'cycles': cycles,
                'size': size,
                'description': description
            }
    
    # Extraer Page 2
    page2_pattern = r'inline constexpr CpuOp CpuOpsPage2\[\] = \{.*?\};'
    page2_match = re.search(page2_pattern, content, re.DOTALL)
    
    page2_opcodes = {}
    if page2_match:
        page2_content = page2_match.group(0)
        matches = re.findall(opcode_pattern, page2_content)
        
        for match in matches:
            opcode = int(match[0], 16)
            name = match[1].strip()
            addr_mode = match[2]
            cycles = int(match[3])
            size = int(match[4])
            description = match[5]
            
            page2_opcodes[opcode] = {
                'name': name,
                'addr_mode': addr_mode,
                'cycles': cycles,
                'size': size,
                'description': description
            }
    
    return page0_opcodes, page1_opcodes, page2_opcodes

def extract_emulator_v2_opcodes():
    """Extrae todos los opcodes implementados en emulator_v2 desde cpu6809.rs"""
    cpu_file = Path("emulator_v2/src/core/cpu6809.rs")
    
    if not cpu_file.exists():
        print(f"❌ No se encontró {cpu_file}")
        return set(), set(), set()
    
    with open(cpu_file, 'r') as f:
        content = f.read()
    
    # Búsqueda más simple: todos los opcodes con patrón 0xXX =>
    all_matches = re.findall(r'0x([0-9A-Fa-f]{2})\s*=>', content)
    
    # Separar por contexto de página
    page0_implemented = set()
    page1_implemented = set()
    page2_implemented = set()
    
    # Page 0: buscar después de "match cpu_op_page" y antes de "1 =>"
    lines = content.split('\n')
    current_page = -1
    
    for line in lines:
        line = line.strip()
        
        # Detectar cambio de página
        if 'match cpu_op_page' in line:
            current_page = -1
        elif line.startswith('0 => {'):
            current_page = 0
        elif line.startswith('1 => {'):
            current_page = 1  
        elif line.startswith('2 => {'):
            current_page = 2
        elif line.startswith('_ => {') and current_page >= 0:
            current_page = -1
            
        # Buscar opcodes en la página actual
        if current_page >= 0:
            opcode_match = re.search(r'0x([0-9A-Fa-f]{2})\s*=>', line)
            if opcode_match:
                opcode = int(opcode_match.group(1), 16)
                if current_page == 0:
                    page0_implemented.add(opcode)
                elif current_page == 1:
                    page1_implemented.add(opcode)
                elif current_page == 2:
                    page2_implemented.add(opcode)
    
    return page0_implemented, page1_implemented, page2_implemented

def generate_comparison_table():
    """Genera una tabla HTML comparativa completa"""
    print("=== COMPARACIÓN COMPLETA: VECTREXY vs EMULATOR_V2 ===\n")
    
    # Extraer datos
    vectrexy_p0, vectrexy_p1, vectrexy_p2 = extract_vectrexy_opcodes()
    implemented_p0, implemented_p1, implemented_p2 = extract_emulator_v2_opcodes()
    
    print(f"Vectrexy - Page 0: {len(vectrexy_p0)} opcodes válidos")
    print(f"Vectrexy - Page 1: {len(vectrexy_p1)} opcodes válidos")
    print(f"Vectrexy - Page 2: {len(vectrexy_p2)} opcodes válidos")
    print(f"Implementados - Page 0: {len(implemented_p0)} opcodes")
    print(f"Implementados - Page 1: {len(implemented_p1)} opcodes")
    print(f"Implementados - Page 2: {len(implemented_p2)} opcodes")
    
    # Generar tabla HTML
    html_content = '''<!DOCTYPE html>
<html>
<head>
    <title>Comparación Opcodes: Vectrexy vs Emulator_v2</title>
    <style>
        body { font-family: monospace; font-size: 12px; }
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 4px; text-align: left; }
        th { background-color: #f2f2f2; }
        .implemented { background-color: #d4edda; }
        .missing { background-color: #f8d7da; }
        .illegal { background-color: #e2e3e5; }
        .page-header { background-color: #007bff; color: white; font-weight: bold; }
    </style>
</head>
<body>
    <h1>Comparación Opcodes: Vectrexy vs Emulator_v2</h1>
    <p>✅ = Implementado | ❌ = Faltante | ⚫ = Ilegal/No aplicable</p>
    
    <table>
        <tr>
            <th>Opcode</th>
            <th>Nombre</th>
            <th>Modo Dirección</th>
            <th>Ciclos</th>
            <th>Tamaño</th>
            <th>Estado</th>
            <th>Descripción</th>
        </tr>
'''
    
    # Page 0
    html_content += '        <tr class="page-header"><td colspan="7">PAGE 0 (0x00-0xFF)</td></tr>\n'
    
    for opcode in range(0x00, 0x100):
        if opcode in vectrexy_p0:
            op_info = vectrexy_p0[opcode]
            implemented = "✅" if opcode in implemented_p0 else "❌"
            css_class = "implemented" if opcode in implemented_p0 else "missing"
            
            html_content += f'''        <tr class="{css_class}">
            <td>0x{opcode:02X}</td>
            <td>{op_info['name']}</td>
            <td>{op_info['addr_mode']}</td>
            <td>{op_info['cycles']}</td>
            <td>{op_info['size']}</td>
            <td>{implemented}</td>
            <td>{op_info['description']}</td>
        </tr>
'''
        elif opcode in implemented_p0:
            # Opcode implementado pero no en Vectrexy (esto sería extraño)
            html_content += f'''        <tr class="missing">
            <td>0x{opcode:02X}</td>
            <td>???</td>
            <td>???</td>
            <td>???</td>
            <td>???</td>
            <td>⚠️</td>
            <td>Implementado pero no en Vectrexy</td>
        </tr>
'''
    
    # Page 1
    html_content += '        <tr class="page-header"><td colspan="7">PAGE 1 (0x10xx)</td></tr>\n'
    
    for opcode in sorted(set(vectrexy_p1.keys()) | implemented_p1):
        if opcode in vectrexy_p1:
            op_info = vectrexy_p1[opcode]
            implemented = "✅" if opcode in implemented_p1 else "❌"
            css_class = "implemented" if opcode in implemented_p1 else "missing"
            
            html_content += f'''        <tr class="{css_class}">
            <td>0x10{opcode:02X}</td>
            <td>{op_info['name']}</td>
            <td>{op_info['addr_mode']}</td>
            <td>{op_info['cycles']}</td>
            <td>{op_info['size']}</td>
            <td>{implemented}</td>
            <td>{op_info['description']}</td>
        </tr>
'''
        elif opcode in implemented_p1:
            html_content += f'''        <tr class="missing">
            <td>0x10{opcode:02X}</td>
            <td>???</td>
            <td>???</td>
            <td>???</td>
            <td>???</td>
            <td>⚠️</td>
            <td>Implementado pero no en Vectrexy</td>
        </tr>
'''
    
    # Page 2
    html_content += '        <tr class="page-header"><td colspan="7">PAGE 2 (0x11xx)</td></tr>\n'
    
    for opcode in sorted(set(vectrexy_p2.keys()) | implemented_p2):
        if opcode in vectrexy_p2:
            op_info = vectrexy_p2[opcode]
            implemented = "✅" if opcode in implemented_p2 else "❌"
            css_class = "implemented" if opcode in implemented_p2 else "missing"
            
            html_content += f'''        <tr class="{css_class}">
            <td>0x11{opcode:02X}</td>
            <td>{op_info['name']}</td>
            <td>{op_info['addr_mode']}</td>
            <td>{op_info['cycles']}</td>
            <td>{op_info['size']}</td>
            <td>{implemented}</td>
            <td>{op_info['description']}</td>
        </tr>
'''
        elif opcode in implemented_p2:
            html_content += f'''        <tr class="missing">
            <td>0x11{opcode:02X}</td>
            <td>???</td>
            <td>???</td>
            <td>???</td>
            <td>???</td>
            <td>⚠️</td>
            <td>Implementado pero no en Vectrexy</td>
        </tr>
'''
    
    html_content += '''    </table>
    
    <h2>Resumen</h2>
    <ul>
        <li>Vectrexy Page 0: ''' + str(len(vectrexy_p0)) + ''' opcodes válidos</li>
        <li>Implementados Page 0: ''' + str(len(implemented_p0)) + ''' opcodes</li>
        <li>Vectrexy Page 1: ''' + str(len(vectrexy_p1)) + ''' opcodes válidos</li>
        <li>Implementados Page 1: ''' + str(len(implemented_p1)) + ''' opcodes</li>
        <li>Vectrexy Page 2: ''' + str(len(vectrexy_p2)) + ''' opcodes válidos</li>
        <li>Implementados Page 2: ''' + str(len(implemented_p2)) + ''' opcodes</li>
    </ul>
    
    <h3>Análisis de Diferencias</h3>
'''
    
    # Análisis de diferencias
    missing_p0 = set(vectrexy_p0.keys()) - implemented_p0
    missing_p1 = set(vectrexy_p1.keys()) - implemented_p1
    missing_p2 = set(vectrexy_p2.keys()) - implemented_p2
    
    if missing_p0:
        html_content += '<p><strong>Page 0 faltantes:</strong> ' + ', '.join(f'0x{op:02X}' for op in sorted(missing_p0)) + '</p>\n'
    if missing_p1:
        html_content += '<p><strong>Page 1 faltantes:</strong> ' + ', '.join(f'0x10{op:02X}' for op in sorted(missing_p1)) + '</p>\n'
    if missing_p2:
        html_content += '<p><strong>Page 2 faltantes:</strong> ' + ', '.join(f'0x11{op:02X}' for op in sorted(missing_p2)) + '</p>\n'
    
    coverage_p0 = (len(implemented_p0) / len(vectrexy_p0)) * 100 if vectrexy_p0 else 0
    coverage_p1 = (len(implemented_p1) / len(vectrexy_p1)) * 100 if vectrexy_p1 else 0
    coverage_p2 = (len(implemented_p2) / len(vectrexy_p2)) * 100 if vectrexy_p2 else 0
    
    html_content += f'''
    <h3>Cobertura</h3>
    <ul>
        <li>Page 0: {coverage_p0:.1f}% ({len(implemented_p0)}/{len(vectrexy_p0)})</li>
        <li>Page 1: {coverage_p1:.1f}% ({len(implemented_p1)}/{len(vectrexy_p1)})</li>
        <li>Page 2: {coverage_p2:.1f}% ({len(implemented_p2)}/{len(vectrexy_p2)})</li>
    </ul>
    
</body>
</html>'''
    
    # Guardar archivo HTML
    output_file = Path("opcode_comparison_table.html")
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write(html_content)
    
    print(f"\n✅ Tabla HTML generada: {output_file}")
    
    # Resumen en consola
    print(f"\n=== RESUMEN ===")
    print(f"Page 0: {coverage_p0:.1f}% ({len(implemented_p0)}/{len(vectrexy_p0)})")
    print(f"Page 1: {coverage_p1:.1f}% ({len(implemented_p1)}/{len(vectrexy_p1)})")
    print(f"Page 2: {coverage_p2:.1f}% ({len(implemented_p2)}/{len(vectrexy_p2)})")
    
    total_vectrexy = len(vectrexy_p0) + len(vectrexy_p1) + len(vectrexy_p2)
    total_implemented = len(implemented_p0) + len(implemented_p1) + len(implemented_p2)
    total_coverage = (total_implemented / total_vectrexy) * 100 if total_vectrexy else 0
    
    print(f"TOTAL: {total_coverage:.1f}% ({total_implemented}/{total_vectrexy})")
    
    # Mostrar algunos faltantes críticos
    if missing_p0:
        print(f"\nPage 0 faltantes ({len(missing_p0)}):")
        for op in sorted(missing_p0)[:10]:  # Primeros 10
            name = vectrexy_p0[op]['name']
            print(f"  0x{op:02X}: {name}")
        if len(missing_p0) > 10:
            print(f"  ... y {len(missing_p0) - 10} más")
    
    if missing_p1:
        print(f"\nPage 1 faltantes ({len(missing_p1)}):")
        for op in sorted(missing_p1):
            name = vectrexy_p1[op]['name']
            print(f"  0x10{op:02X}: {name}")
    
    if missing_p2:
        print(f"\nPage 2 faltantes ({len(missing_p2)}):")
        for op in sorted(missing_p2):
            name = vectrexy_p2[op]['name']
            print(f"  0x11{op:02X}: {name}")

if __name__ == "__main__":
    generate_comparison_table()