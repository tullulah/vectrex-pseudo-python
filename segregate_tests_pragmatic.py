#!/usr/bin/env python3
"""
Segregación pragmática de tests de opcodes.
Separa archivos que mezclan múltiples opcodes NO relacionados.
Mantiene juntos los relacionados (TFR/EXG, LBRA/LBSR, RTI/SWI/CWAI).
"""

import re
from pathlib import Path
from collections import defaultdict

# Directorio base
BASE_DIR = Path('emulator_v2/tests/opcodes')

# Archivos que se MANTIENEN juntos (opcodes relacionados)
KEEP_TOGETHER = {
    'misc/test_tfr_exg.rs',
    'misc/test_tfr_exg_correct.rs',
    'branch/test_lbra_lbsr.rs',
    'interrupt/test_rti_swi_cwai.rs',
}

def extract_tests_by_function(content):
    """
    Extrae funciones de test completas del contenido.
    Retorna dict {nombre_test: código_completo}
    """
    tests = {}
    
    # Pattern para capturar #[test] + función completa
    pattern = r'(#\[test\]\s*(?:#\[should_panic[^\]]*\])?\s*fn\s+(\w+)\s*\([^)]*\)\s*\{(?:[^{}]|\{(?:[^{}]|\{[^{}]*\})*\})*\})'
    
    for match in re.finditer(pattern, content, re.MULTILINE | re.DOTALL):
        full_test = match.group(1)
        test_name = match.group(2)
        tests[test_name] = full_test
    
    return tests

def extract_header(content):
    """Extrae los imports y comentarios del header"""
    lines = content.split('\n')
    header_lines = []
    
    for line in lines:
        # Detenerse al llegar al primer #[test]
        if line.strip().startswith('#[test]'):
            break
        header_lines.append(line)
    
    return '\n'.join(header_lines).strip()

def get_opcode_from_test_name(test_name):
    """Extrae el opcode del nombre del test"""
    # test_adda_immediate_0x8b -> adda
    # test_clr_a_basic -> clr
    # test_nop_0x12 -> nop
    
    parts = test_name.replace('test_', '').split('_')
    
    # Casos especiales
    if parts[0] in ['clr', 'inc', 'dec', 'tst', 'neg', 'com'] and len(parts) > 1 and parts[1] in ['a', 'b']:
        # test_clr_a_basic -> clra
        return parts[0] + parts[1]
    
    # Caso normal
    return parts[0]

def segregate_file(file_path):
    """Segrega un archivo en múltiples archivos por opcode"""
    full_path = BASE_DIR / file_path
    
    if not full_path.exists():
        print(f"  ⚠️  No existe: {file_path}")
        return []
    
    content = full_path.read_text(encoding='utf-8')
    header = extract_header(content)
    tests = extract_tests_by_function(content)
    
    if not tests:
        print(f"  ⚠️  No se encontraron tests en {file_path}")
        return []
    
    # Agrupar tests por opcode
    tests_by_opcode = defaultdict(list)
    for test_name, test_code in tests.items():
        opcode = get_opcode_from_test_name(test_name)
        tests_by_opcode[opcode].append((test_name, test_code))
    
    print(f"\n📁 {file_path}")
    print(f"   Encontrados {len(tests)} tests en {len(tests_by_opcode)} opcodes diferentes")
    
    # Si solo hay un opcode, no necesita segregación
    if len(tests_by_opcode) == 1:
        print(f"   ✅ Ya tiene un solo opcode, no requiere segregación")
        return []
    
    category = file_path.split('/')[0]
    created_files = []
    
    for opcode, opcode_tests in sorted(tests_by_opcode.items()):
        new_file_path = BASE_DIR / category / f"test_{opcode}.rs"
        
        print(f"   → {opcode}: {len(opcode_tests)} tests → test_{opcode}.rs")
        
        # Crear archivo
        with open(new_file_path, 'w', encoding='utf-8') as f:
            f.write(header + '\n\n')
            
            for test_name, test_code in sorted(opcode_tests):
                f.write(test_code + '\n\n')
        
        created_files.append(new_file_path)
    
    return created_files

def main():
    print("=" * 80)
    print("SEGREGACIÓN PRAGMÁTICA DE TESTS DE OPCODES")
    print("=" * 80)
    
    all_created = []
    files_to_delete = []
    
    # Buscar todos los archivos .rs (excepto mod.rs)
    for rs_file in BASE_DIR.rglob('*.rs'):
        if rs_file.name == 'mod.rs':
            continue
        
        rel_path = str(rs_file.relative_to(BASE_DIR)).replace('\\', '/')
        
        # Saltar archivos que queremos mantener juntos
        if rel_path in KEEP_TOGETHER:
            print(f"\n⏭️  Manteniendo junto: {rel_path}")
            continue
        
        created = segregate_file(rel_path)
        
        if created:
            all_created.extend(created)
            files_to_delete.append(rs_file)
    
    print("\n" + "=" * 80)
    print(f"✅ Segregación completada!")
    print(f"   Archivos creados: {len(all_created)}")
    print(f"   Archivos a eliminar: {len(files_to_delete)}")
    print("=" * 80)
    
    if files_to_delete:
        print("\n📋 ARCHIVOS PARA ELIMINAR (contienen tests ahora segregados):")
        for f in sorted(files_to_delete):
            print(f"   - {f.relative_to(BASE_DIR)}")
    
    print("\n📋 Próximos pasos:")
    print("1. Revisar archivos generados")
    print("2. Eliminar archivos originales duplicados")
    print("3. Actualizar mod.rs en cada categoría")
    print("4. Ejecutar cargo test")

if __name__ == '__main__':
    main()
