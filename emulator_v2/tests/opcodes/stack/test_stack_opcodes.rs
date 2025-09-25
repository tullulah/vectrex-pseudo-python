// test_stack_opcodes.rs - Tests for stack operation opcodes
// C++ Original Analysis from Vectrexy Cpu.cpp lines 627-682:
//
// OpPSH template: Pushes registers based on bit mask
// - Bit 7: PC (16-bit)
// - Bit 6: Other stack register (S or U, 16-bit) 
// - Bit 5: Y (16-bit)
// - Bit 4: X (16-bit)
// - Bit 3: DP (8-bit)
// - Bit 2: B (8-bit)
// - Bit 1: A (8-bit)
// - Bit 0: CC (8-bit)
//
// OpPUL template: Pulls registers in reverse order (opposite of push)
// - Same bit layout but pulls in reverse order (CC first, PC last)
//
// Timing: 1 cycle per 8-bit value, 2 cycles per 16-bit value
//
// Opcodes:
// 0x34: PSHS - Push to system stack (S)
// 0x35: PULS - Pull from system stack (S)  
// 0x36: PSHU - Push to user stack (U)
// 0x37: PULU - Pull from user stack (U)

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
fn test_pshs_single_register_a() {
    // C++ Original: if (value & BITS(1)) Push8(stackReg, A);
    let mut cpu = create_test_cpu();
    
    // Setup: A = 0x42, S = 0xD000 (top of RAM+1)
    cpu.registers_mut().a = 0x42;
    cpu.registers_mut().s = 0xD000;
    
    // Memory: 0x34 0x02 (PSHS with bit 1 set = push A)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x34);
    memory_bus.borrow_mut().write(0xC801, 0x02);
    cpu.registers_mut().pc = 0xC800;
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Verify: A pushed to stack, S decremented
    assert_eq!(cpu.registers().s, 0xCFFF); // S decremented by 1
    assert_eq!(memory_bus.borrow().read(0xCFFF), 0x42); // A value on stack
    assert_eq!(cpu.registers().pc, 0xC802); // PC advanced
}

#[test]
fn test_pshs_all_registers() {
    // C++ Original: Test all register combinations in push order
    let mut cpu = create_test_cpu();
    
    // Setup: Initialize all registers
    cpu.registers_mut().pc = 0xC800;  // Will be saved as current PC
    cpu.registers_mut().a = 0x11;
    cpu.registers_mut().b = 0x22;
    cpu.registers_mut().cc.from_u8(0x33);
    cpu.registers_mut().dp = 0x44;
    cpu.registers_mut().x = 0x5566;
    cpu.registers_mut().y = 0x7788;
    cpu.registers_mut().u = 0x99AA;
    cpu.registers_mut().s = 0xD000; // Start at top of RAM
    
    // Memory: 0x34 0xFF (PSHS with all bits set = push all registers)
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x34);
    memory_bus.borrow_mut().write(0xC801, 0xFF);
    
    let _cycles = cpu.execute_instruction(false, false);
    
    // Verify: All registers pushed in order: PC, U, Y, X, DP, B, A, CC
    // Stack grows downward, so last pushed is at lowest address
    let expected_s = 0xD000 - 12; // 8 bytes (4 x 16-bit) + 4 bytes (4 x 8-bit) = 12 bytes total
    assert_eq!(cpu.registers().s, expected_s);
    
    // Check stack contents (bottom to top, reverse push order)
    let mut stack_addr = expected_s;
    
    // CC (8-bit, last pushed, lowest address)
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x33);
    stack_addr += 1;
    
    // A (8-bit)
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x11);
    stack_addr += 1;
    
    // B (8-bit)
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x22);
    stack_addr += 1;
    
    // DP (8-bit)
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x44);
    stack_addr += 1;
    
    // X (16-bit, high byte first)
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x55); // X high
    stack_addr += 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x66); // X low
    stack_addr += 1;
    
    // Y (16-bit, high byte first)
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x77); // Y high
    stack_addr += 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x88); // Y low
    stack_addr += 1;
    
    // U (16-bit, high byte first)
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x99); // U high
    stack_addr += 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0xAA); // U low
    stack_addr += 1;
    
    // PC (16-bit, high byte first, first pushed, highest address)
    assert_eq!(memory_bus.borrow().read(stack_addr), 0xC8); // PC high (0xC800)
    stack_addr += 1;
    assert_eq!(memory_bus.borrow().read(stack_addr), 0x02); // PC low (0xC802, after instruction)
    
    // Verify PC advanced to next instruction
    assert_eq!(cpu.registers().pc, 0xC802);
}