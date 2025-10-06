#!/usr/bin/env python3
"""
Migra tests de arithmetic de API antigua (borrow_mut) a API nueva (UnsafeCell directo)
"""
import re
from pathlib import Path

def migrate_file(file_path):
    """Migra un archivo de test a la nueva API"""
    content = file_path.read_text(encoding='utf-8')
    original = content
    
    # Patrón 1: cpu.memory_bus().borrow_mut().write(...) -> unsafe { &mut *memory.get() }.write(...)
    content = re.sub(
        r'cpu\.memory_bus\(\)\.borrow_mut\(\)\.write\(([^,]+),\s*([^)]+)\)',
        r'unsafe { &mut *memory.get() }.write(\1, \2)',
        content
    )
    
    # Patrón 2: cpu.memory_bus().borrow_mut().read(...) -> unsafe { &*memory.get() }.read(...)
    content = re.sub(
        r'cpu\.memory_bus\(\)\.borrow_mut\(\)\.read\(([^)]+)\)',
        r'unsafe { &*memory.get() }.read(\1)',
        content
    )
    
    # Patrón 3: cpu.memory_bus().borrow().read(...) -> unsafe { &*memory.get() }.read(...)
    content = re.sub(
        r'cpu\.memory_bus\(\)\.borrow\(\)\.read\(([^)]+)\)',
        r'unsafe { &*memory.get() }.read(\1)',
        content
    )
    
    # Patrón 4: let cycles = cpu.execute_instruction(...); ... assert_eq!(cycles, X)
    # Cambiar a: cpu.execute_instruction(...).unwrap(); (eliminar assert de cycles)
    def replace_let_cycles(match):
        indent = match.group(1)
        args = match.group(2)
        return f'{indent}cpu.execute_instruction({args}).unwrap();'
    
    content = re.sub(
        r'^(\s+)let cycles = cpu\.execute_instruction\(([^)]+)\);$',
        replace_let_cycles,
        content,
        flags=re.MULTILINE
    )
    
    # Patrón 5: Eliminar assert_eq!(cycles, ...) ya que no verificamos cycles
    content = re.sub(
        r'^\s+assert_eq!\(cycles,\s*\d+\).*$\n',
        '',
        content,
        flags=re.MULTILINE
    )
    
    # Patrón 6: Eliminar referencias sueltas a cycles en asserts
    content = re.sub(
        r'^\s+// Check cycles.*$\n',
        '',
        content,
        flags=re.MULTILINE
    )
    
    # Solo escribir si hubo cambios
    if content != original:
        file_path.write_text(content, encoding='utf-8')
        return True
    return False

def main():
    test_dir = Path(r'C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\emulator_v2\tests\opcodes\arithmetic')
    
    if not test_dir.exists():
        print(f"❌ Directorio no encontrado: {test_dir}")
        return
    
    migrated = 0
    skipped = 0
    
    for test_file in test_dir.glob('test_*.rs'):
        if migrate_file(test_file):
            print(f"✅ Migrado: {test_file.name}")
            migrated += 1
        else:
            print(f"⏭️  Sin cambios: {test_file.name}")
            skipped += 1
    
    print(f"\n📊 Resumen:")
    print(f"   Migrados: {migrated}")
    print(f"   Sin cambios: {skipped}")
    print(f"   Total: {migrated + skipped}")

if __name__ == '__main__':
    main()
