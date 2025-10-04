#!/usr/bin/env python3
"""
Script para segregar tests de opcodes mezclados en archivos individuales.
Cada opcode debe tener su propio archivo test_[opcode].rs
"""

import re
import os
from pathlib import Path
from collections import defaultdict

# Directorio base
TESTS_DIR = Path('emulator_v2/tests/opcodes')

# Mapeo de archivos a segregar
FILES_TO_SEGREGATE = {
    # Arithmetic
    'arithmetic/test_add_sub_opcodes.rs': ['adda', 'addb', 'suba', 'subb'],
    'arithmetic/test_addb_subb.rs': ['addb', 'subb'],
    'arithmetic/test_and_eor_opcodes.rs': ['anda', 'andb', 'eora', 'eorb'],
    'arithmetic/test_arithmetic_corrected.rs': ['anda', 'adda', 'eora', 'oraa', 'suba'],
    'arithmetic/test_indexed_arithmetic.rs': ['anda', 'adda', 'eora', 'oraa', 'suba'],
    'arithmetic/test_indexed_arithmetic_fixed.rs': ['anda', 'adda', 'eora', 'oraa', 'suba'],
    'arithmetic/test_b_register_opcodes.rs': ['addb', 'andb', 'eorb', 'orb', 'subb'],
    'arithmetic/test_logic_b.rs': ['andb', 'eorb', 'orab'],
    'arithmetic/test_mul_sex_opcodes.rs': ['mul', 'sex'],
    'arithmetic/test_or_opcodes.rs': ['oraa', 'orab'],
    
    # Branch
    'branch/test_lbra_lbsr.rs': ['lbra', 'lbsr'],
    
    # Data transfer
    'data_transfer/test_lea_opcodes.rs': ['leax', 'leay', 'leas', 'leau'],
    'data_transfer/test_std_stu.rs': ['std', 'stu'],
    'data_transfer/test_store_16bit.rs': ['stx', 'std', 'stu'],
    'data_transfer/test_store_16bit_corrected.rs': ['stx', 'std', 'stu'],
    'data_transfer/test_store_16bit_fixed.rs': ['stx', 'std', 'stu'],
    
    # Interrupt
    'interrupt/test_rti_swi_cwai.rs': ['rti', 'swi', 'cwai'],
    
    # Misc
    'misc/test_basic_opcodes_fixed.rs': ['clr', 'inc', 'dec', 'tst', 'neg', 'com'],
    'misc/test_condition_code_opcodes.rs': ['andcc', 'orcc'],
    'misc/test_extended_addressing_opcodes.rs': ['ora', 'and', 'eor', 'add', 'sub'],
    'misc/test_minimal_opcodes.rs': ['nop', 'clra', 'inca', 'adda', 'suba'],
    'misc/test_tfr_exg.rs': ['tfr', 'exg'],
    'misc/test_tfr_exg_correct.rs': ['tfr', 'exg'],
}

def extract_tests_for_opcode(content, opcode):
    """Extrae todos los tests relacionados con un opcode específico"""
    pattern = rf'(#\[test\]\s*\n\s*fn test_{opcode}[^\n]*?\n(?:.*?\n)*?(?=\n#\[test\]|\nmod |\Z))'
    matches = re.findall(pattern, content, re.MULTILINE | re.DOTALL)
    return matches

def get_file_header(original_file):
    """Extrae el header del archivo original (imports, etc.)"""
    content = Path(TESTS_DIR / original_file).read_text(encoding='utf-8')
    # Extraer hasta el primer #[test]
    match = re.search(r'^(.*?)(?=#\[test\])', content, re.MULTILINE | re.DOTALL)
    return match.group(1).strip() if match else ""

def segregate_file(file_path, opcodes):
    """Segrega un archivo en múltiples archivos, uno por opcode"""
    full_path = TESTS_DIR / file_path
    if not full_path.exists():
        print(f"⚠️  Archivo no existe: {full_path}")
        return
    
    print(f"\n📁 Procesando: {file_path}")
    content = full_path.read_text(encoding='utf-8')
    header = get_file_header(file_path)
    
    category = file_path.split('/')[0]
    
    for opcode in opcodes:
        # Extraer tests de este opcode
        tests = extract_tests_for_opcode(content, opcode)
        
        if not tests:
            print(f"  ⚠️  No se encontraron tests para {opcode}")
            continue
        
        # Crear nuevo archivo
        new_file = TESTS_DIR / category / f"test_{opcode}.rs"
        
        if new_file.exists():
            print(f"  ⏭️  {opcode}: Ya existe test_{opcode}.rs, mergeando...")
            # Si ya existe, agregar al final
            existing_content = new_file.read_text(encoding='utf-8')
            with open(new_file, 'a', encoding='utf-8') as f:
                f.write('\n\n')
                for test in tests:
                    f.write(test.strip() + '\n\n')
        else:
            print(f"  ✅ {opcode}: Creando test_{opcode}.rs ({len(tests)} tests)")
            with open(new_file, 'w', encoding='utf-8') as f:
                f.write(header + '\n\n')
                for test in tests:
                    f.write(test.strip() + '\n\n')
    
    print(f"  🗑️  Marcando para eliminar: {file_path}")

def main():
    print("=" * 80)
    print("SEGREGACIÓN DE TESTS DE OPCODES")
    print("=" * 80)
    
    for file_path, opcodes in FILES_TO_SEGREGATE.items():
        segregate_file(file_path, opcodes)
    
    print("\n" + "=" * 80)
    print("✅ Segregación completada!")
    print("=" * 80)
    print("\nPróximos pasos:")
    print("1. Revisar los archivos generados")
    print("2. Eliminar archivos originales mezclados")
    print("3. Actualizar mod.rs en cada categoría")
    print("4. Ejecutar cargo test para verificar")

if __name__ == '__main__':
    main()
