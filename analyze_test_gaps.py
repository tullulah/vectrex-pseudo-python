#!/usr/bin/env python3
"""
Script para identificar gaps específicos en la estructura de tests reorganizada
Analiza qué opcodes podrían necesitar tests específicos en la nueva organización
"""

import os
from pathlib import Path
import re

def get_existing_test_opcodes():
    """Extrae los opcodes que ya tienen tests en la estructura reorganizada"""
    test_dir = Path("emulator_v2/tests/opcodes")
    existing_tests = {}
    
    for test_file in test_dir.rglob("test_*.rs"):
        # Extraer opcode del nombre del archivo
        filename = test_file.stem
        
        # Leer contenido para buscar opcodes específicos
        try:
            with open(test_file, 'r', encoding='utf-8') as f:
                content = f.read()
                
            # Buscar patrones de opcodes en el contenido
            opcode_matches = re.findall(r'0x([0-9A-Fa-f]{2})', content)
            
            if opcode_matches:
                category = test_file.parent.name
                if category not in existing_tests:
                    existing_tests[category] = {}
                
                # Extraer el nombre base del test (ej: test_lda -> lda)
                base_name = filename.replace('test_', '')
                existing_tests[category][base_name] = {
                    'file': str(test_file.relative_to(Path.cwd())),
                    'opcodes': list(set(opcode_matches))
                }
        except Exception as e:
            print(f"Error leyendo {test_file}: {e}")
    
    return existing_tests

def analyze_potential_gaps():
    """Identifica opcodes importantes que podrían necesitar tests específicos"""
    # Definir opcodes importantes por categoría según MC6809
    important_opcodes = {
        'arithmetic': {
            'adda': ['0x8B', '0x9B', '0xAB', '0xBB'],  # immediate, direct, indexed, extended
            'addb': ['0xCB', '0xDB', '0xEB', '0xFB'],
            'addd': ['0xC3', '0xD3', '0xE3', '0xF3'],
            'suba': ['0x80', '0x90', '0xA0', '0xB0'],
            'subb': ['0xC0', '0xD0', '0xE0', '0xF0'],
            'subd': ['0x83', '0x93', '0xA3', '0xB3'],
            'mul': ['0x3D'],
            'daa': ['0x19'],
        },
        'logic': {
            'anda': ['0x84', '0x94', '0xA4', '0xB4'],
            'andb': ['0xC4', '0xD4', '0xE4', '0xF4'],
            'ora': ['0x8A', '0x9A', '0xAA', '0xBA'],
            'orb': ['0xCA', '0xDA', '0xEA', '0xFA'],
            'eora': ['0x88', '0x98', '0xA8', '0xB8'],
            'eorb': ['0xC8', '0xD8', '0xE8', '0xF8'],
            'bita': ['0x85', '0x95', '0xA5', '0xB5'],
            'bitb': ['0xC5', '0xD5', '0xE5', '0xF5'],
            'coma': ['0x43'],
            'comb': ['0x53'],
            'nega': ['0x40'],
            'negb': ['0x50'],
        },
        'branch': {
            'bra': ['0x20'],
            'brn': ['0x21'],
            'bhi': ['0x22'],
            'bls': ['0x23'],
            'bcc': ['0x24'],
            'bcs': ['0x25'],
            'bne': ['0x26'],
            'beq': ['0x27'],
            'bvc': ['0x28'],
            'bvs': ['0x29'],
            'bpl': ['0x2A'],
            'bmi': ['0x2B'],
            'bge': ['0x2C'],
            'blt': ['0x2D'],
            'bgt': ['0x2E'],
            'ble': ['0x2F'],
            'jmp': ['0x0E', '0x6E', '0x7E'],  # direct, indexed, extended
            'jsr': ['0x9D', '0xAD', '0xBD'],  # direct, indexed, extended
            'bsr': ['0x8D'],
            'rts': ['0x39'],
            'lbra': ['0x16'],
            'lbsr': ['0x17'],
        },
        'data_transfer': {
            'lda': ['0x86', '0x96', '0xA6', '0xB6'],
            'ldb': ['0xC6', '0xD6', '0xE6', '0xF6'],
            'ldd': ['0xCC', '0xDC', '0xEC', '0xFC'],
            'ldx': ['0x8E', '0x9E', '0xAE', '0xBE'],
            'ldy': ['0x108E', '0x109E', '0x10AE', '0x10BE'],  # Page 1
            'ldu': ['0xCE', '0xDE', '0xEE', '0xFE'],
            'lds': ['0x10CE', '0x10DE', '0x10EE', '0x10FE'],  # Page 1
            'sta': ['0x97', '0xA7', '0xB7'],
            'stb': ['0xD7', '0xE7', '0xF7'],
            'std': ['0xDD', '0xED', '0xFD'],
            'stx': ['0x9F', '0xAF', '0xBF'],
            'sty': ['0x109F', '0x10AF', '0x10BF'],  # Page 1
            'stu': ['0xDF', '0xEF', '0xFF'],
            'sts': ['0x10DF', '0x10EF', '0x10FF'],  # Page 1
            'tfr': ['0x1F'],
            'exg': ['0x1E'],
        },
        'register': {
            'inca': ['0x4C'],
            'incb': ['0x5C'],
            'inc': ['0x0C', '0x6C', '0x7C'],  # direct, indexed, extended
            'deca': ['0x4A'],
            'decb': ['0x5A'],
            'dec': ['0x0A', '0x6A', '0x7A'],
            'clra': ['0x4F'],
            'clrb': ['0x5F'],
            'clr': ['0x0F', '0x6F', '0x7F'],
            'tsta': ['0x4D'],
            'tstb': ['0x5D'],
            'tst': ['0x0D', '0x6D', '0x7D'],
        },
        'shift': {
            'lsra': ['0x44'],
            'lsrb': ['0x54'],
            'lsr': ['0x04', '0x64', '0x74'],
            'lsla': ['0x48'],  # Same as ASLA
            'lslb': ['0x58'],  # Same as ASLB
            'lsl': ['0x08', '0x68', '0x78'],  # Same as ASL
            'rora': ['0x46'],
            'rorb': ['0x56'],
            'ror': ['0x06', '0x66', '0x76'],
            'rola': ['0x49'],
            'rolb': ['0x59'],
            'rol': ['0x09', '0x69', '0x79'],
            'asra': ['0x47'],
            'asrb': ['0x57'],
            'asr': ['0x07', '0x67', '0x77'],
        },
        'system': {
            'nop': ['0x12'],
            'sync': ['0x13'],
            'daa': ['0x19'],
            'orcc': ['0x1A'],
            'andcc': ['0x1C'],
            'sex': ['0x1D'],
            'abx': ['0x3A'],
            'mul': ['0x3D'],
            'swi': ['0x3F'],
            'swi2': ['0x103F'],  # Page 1
            'swi3': ['0x113F'],  # Page 2
            'cwai': ['0x3C'],
            'rti': ['0x3B'],
        },
        'stack': {
            'pshs': ['0x34'],
            'puls': ['0x35'],
            'pshu': ['0x36'],
            'pulu': ['0x37'],
        },
        'memory': {
            'lea': ['0x30', '0x31', '0x32', '0x33'],  # LEAX, LEAY, LEAS, LEAU
        }
    }
    
    return important_opcodes

def main():
    print("=== ANÁLISIS DE GAPS EN ESTRUCTURA DE TESTS ===\n")
    
    existing_tests = get_existing_test_opcodes()
    important_opcodes = analyze_potential_gaps()
    
    print("📊 TESTS EXISTENTES POR CATEGORÍA:")
    for category, tests in existing_tests.items():
        print(f"\n{category.upper()} ({len(tests)} tests):")
        for test_name, info in tests.items():
            opcodes_str = ', '.join(f"0x{op}" for op in info['opcodes'][:5])
            if len(info['opcodes']) > 5:
                opcodes_str += f" (+ {len(info['opcodes']) - 5} más)"
            print(f"  ✅ {test_name:<20} -> {opcodes_str}")
    
    print(f"\n🔍 ANÁLISIS DE COBERTURA:")
    
    # Comparar con opcodes importantes
    for category, expected_opcodes in important_opcodes.items():
        existing_in_category = existing_tests.get(category, {})
        
        print(f"\n{category.upper()}:")
        missing_important = []
        
        for opcode_name, opcode_variants in expected_opcodes.items():
            if opcode_name not in existing_in_category:
                missing_important.append(opcode_name)
                print(f"  ❌ {opcode_name:<20} -> {', '.join(opcode_variants)}")
            else:
                print(f"  ✅ {opcode_name:<20} -> Cubierto")
        
        if not missing_important:
            print(f"  🎉 {category.upper()} - ¡Cobertura completa!")
    
    # Sugerencias de próximos tests
    print(f"\n🎯 RECOMENDACIONES DE PRÓXIMOS TESTS:")
    print("=" * 50)
    
    priority_tests = [
        ('data_transfer', 'sta', 'Store A en todos los modos'),
        ('data_transfer', 'stb', 'Store B en todos los modos'),
        ('data_transfer', 'std', 'Store D (16-bit)'),
        ('data_transfer', 'stx', 'Store X en todos los modos'),
        ('branch', 'bra', 'Branch Always - opcode fundamental'),
        ('branch', 'beq', 'Branch if Equal - muy usado'),
        ('branch', 'bne', 'Branch if Not Equal - muy usado'),
        ('register', 'inca', 'Increment A register'),
        ('register', 'incb', 'Increment B register'),
        ('register', 'deca', 'Decrement A register'),
        ('register', 'decb', 'Decrement B register'),
        ('system', 'nop', 'No Operation - test simple'),
        ('shift', 'lsra', 'Logical Shift Right A'),
        ('shift', 'lsrb', 'Logical Shift Right B'),
    ]
    
    for category, opcode, description in priority_tests:
        existing_in_category = existing_tests.get(category, {})
        if opcode not in existing_in_category:
            print(f"🔥 ALTA PRIORIDAD: tests/opcodes/{category}/test_{opcode}.rs")
            print(f"   📝 {description}")
            print(f"   📂 Ubicación sugerida: emulator_v2/tests/opcodes/{category}/")
            print()

if __name__ == "__main__":
    main()