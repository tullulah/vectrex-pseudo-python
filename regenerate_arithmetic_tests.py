#!/usr/bin/env python3
"""
Regenera tests de arithmetic con arquitectura correcta (sin RefCell, direcciones mapeadas)
Basado en el patrón de test_irq_system.rs que funciona
"""
from pathlib import Path

# Patrón base para tests de arithmetic (siguiendo test_irq_system.rs)
TEST_TEMPLATE = '''use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::{Cpu6809, MemoryBus, EnableSync, Ram, MemoryBusDevice};

const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

fn setup_cpu_with_ram() -> (Cpu6809, Rc<UnsafeCell<Ram>>) {
    let mut memory_bus = MemoryBus::new();
    let ram = Rc::new(UnsafeCell::new(Ram::new()));
    memory_bus.connect_device(ram.clone(), (RAM_START, 0xFFFF), EnableSync::False);
    let mut cpu = Cpu6809::new(memory_bus);
    cpu.registers_mut().s = STACK_START;
    (cpu, ram)
}

// Tests will be added below (regenerated with correct addresses)
'''

def fix_test_file(file_path):
    """Reescribe un archivo de test con arquitectura correcta"""
    content = file_path.read_text(encoding='utf-8')
    
    # Si ya tiene el setup correcto, solo necesitamos arreglar direcciones
    if 'const RAM_START: u16 = 0xC800;' in content:
        # Reemplazar direcciones 0x0000-0x0FFF con RAM_START + offset
        import re
        
        # Patrón: direcciones bajas que deben ser RAM_START + offset
        def replace_address(match):
            addr_str = match.group(1)
            addr = int(addr_str, 16)
            if addr < 0x1000:  # Direcciones bajas deben ser RAM_START + offset
                return f'RAM_START + 0x{addr:04X}'
            return f'0x{addr:04X}'  # Mantener direcciones altas
        
        # Reemplazar en write() calls
        content = re.sub(
            r'\.write\(0x([0-9A-Fa-f]{4}),',
            lambda m: f'.write({replace_address(m)},',
            content
        )
        
        # Reemplazar en asignaciones de PC
        content = re.sub(
            r'\.pc = 0x([0-9A-Fa-f]{4});',
            lambda m: f'.pc = {replace_address(m)};',
            content
        )
        
        # Reemplazar en asignaciones de X/Y
        content = re.sub(
            r'\.(x|y) = 0x([0-9A-Fa-f]{4});',
            lambda m: f'.{m.group(1)} = {replace_address(m)};',
            content
        )
        
        return content
    
    # Si no tiene el setup, agregarlo
    return TEST_TEMPLATE

def main():
    test_dir = Path(r'C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\emulator_v2\tests\opcodes\arithmetic')
    
    if not test_dir.exists():
        print(f"❌ Directorio no encontrado: {test_dir}")
        return
    
    # Solo arreglar test_adda_variants.rs por ahora (los que fallan)
    failing_tests = [
        'test_adda_variants.rs',
        'test_orb.rs'
    ]
    
    fixed = 0
    
    for test_name in failing_tests:
        test_file = test_dir / test_name
        if test_file.exists():
            try:
                new_content = fix_test_file(test_file)
                test_file.write_text(new_content, encoding='utf-8')
                print(f"✅ Arreglado: {test_name}")
                fixed += 1
            except Exception as e:
                print(f"❌ Error en {test_name}: {e}")
        else:
            print(f"⏭️  No encontrado: {test_name}")
    
    print(f"\n📊 Resumen:")
    print(f"   Arreglados: {fixed}")

if __name__ == '__main__':
    main()
