#!/usr/bin/env python3
"""
Genera archivos mod.rs automáticamente basándose en los archivos test_*.rs presentes
"""

from pathlib import Path

BASE_DIR = Path('emulator_v2/tests/opcodes')

CATEGORIES = [
    'arithmetic',
    'branch',
    'data_transfer',
    'misc',
    'illegal',
    'reserved',
    'interrupt'
]

for category in CATEGORIES:
    category_path = BASE_DIR / category
    
    if not category_path.exists():
        print(f"⚠️  {category} no existe")
        continue
    
    # Buscar todos los test_*.rs
    test_files = sorted(category_path.glob('test_*.rs'))
    
    if not test_files:
        print(f"⚠️  {category} no tiene archivos test_*.rs")
        continue
    
    print(f"📁 {category}: {len(test_files)} archivos")
    
    # Generar mod.rs
    mod_rs_path = category_path / 'mod.rs'
    
    with open(mod_rs_path, 'w', encoding='utf-8') as f:
        # Header
        f.write(f"// {category.title()} opcode tests\n")
        f.write("// Auto-generated - one file per opcode\n\n")
        
        # Modules
        for test_file in test_files:
            module_name = test_file.stem  # test_adda -> test_adda
            f.write(f"pub mod {module_name};\n")
    
    print(f"   ✅ mod.rs generado con {len(test_files)} módulos")

print("\n✅ Todos los mod.rs generados!")
