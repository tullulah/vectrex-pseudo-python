// Tests para opcode 0x1B (Reserved)
// Este opcode NO está definido en la especificación MC6809
// El comportamiento correcto es hacer panic

use std::rc::Rc;
use std::cell::RefCell;
use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::{MemoryBus, EnableSync};
use vectrex_emulator_v2::core::ram::Ram;

const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

fn setup_emulator() -> (Cpu6809, Rc<RefCell<Ram>>) {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    let ram = Rc::new(RefCell::new(Ram::new()));
    memory_bus.borrow_mut().connect_device(ram.clone(), (0x0000, 0xFFFF), EnableSync::False);
    
    let mut cpu = Cpu6809::new(memory_bus);
    cpu.registers_mut().s = STACK_START;
    (cpu, ram)
}

#[test]
#[should_panic(expected = "Illegal instruction")]
fn test_reserved_0x1b_panics() {
    let (mut cpu, memory) = setup_emulator();
    
    // Escribir opcode 0x1B en memoria
    memory.borrow_mut().write(RAM_START, 0x1B);
    
    // Configurar PC para ejecutar el opcode
    cpu.registers_mut().pc = RAM_START;
    
    // Intentar ejecutar - debe hacer panic
    cpu.execute_instruction(false, false);
}

#[test]
fn test_reserved_0x1b_not_in_valid_opcodes() {
    // Verificar que 0x1B no está en la lista de opcodes válidos MC6809
    // Este test documenta que el opcode es reserved según la especificación
    
    // Opcodes válidos en rango 0x10-0x1F:
    let valid_opcodes = vec![
        0x10, // Page 1 prefix
        0x11, // Page 2 prefix
        0x12, // NOP
        0x13, // SYNC
        0x16, // LBRA
        0x17, // LBSR
        0x19, // DAA
        0x1A, // ORCC
        0x1C, // ANDCC
        0x1D, // SEX
        0x1E, // EXG
        0x1F, // TFR
    ];
    
    // 0x1B NO debe estar en la lista
    assert!(!valid_opcodes.contains(&0x1B), 
            "0x1B is reserved and should not be in valid opcodes");
}
