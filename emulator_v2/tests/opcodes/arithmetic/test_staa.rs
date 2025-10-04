// Tests para opcodes OR - Port 1:1 desde Vectrexy
// C++ Original: Vectrexy libs/emulator/src/Cpu.cpp opcodes 0x8A, 0xCA

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    Cpu6809::new(memory_bus)
}

#[test]
fn test_staa_direct_basic() {
    // Test STAA $50 - Store A to direct address
    // C++ Original: OpST writes sourceReg to EA, same flag updates as LD
    let mut cpu = create_test_cpu();
    
    cpu.registers_mut().a = 0x42; // Value to store
    cpu.registers_mut().dp = 0xC8; // Set DP to RAM page (0xC8xx)
    
    // Setup: STAA $50 instruction
    // Direct addressing: EA = DP:offset = 0xC8:0x50 = 0xC850 (within RAM range)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x97); // STAA direct opcode
    memory_bus.borrow_mut().write(0xC801, 0x50); // Direct page offset
    cpu.registers_mut().pc = 0xC800;
    
    cpu.execute_instruction(false, false);
    
    // Verify memory was written at effective address 0xC850
    assert_eq!(memory_bus.borrow().read(0xC850), 0x42, "Memory at 0xC850 should contain 0x42");
    
    // Verify flags based on stored value
    assert_eq!(cpu.registers().cc.n, false, "N flag should be clear (bit 7 = 0)");
    assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear (value != 0)");
    assert_eq!(cpu.registers().cc.v, false, "V flag should be clear (always cleared by ST)");
    assert_eq!(cpu.registers().pc, 0xC802, "PC should advance by 2");
}

