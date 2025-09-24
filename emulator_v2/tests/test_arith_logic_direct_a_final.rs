// Test coverage for arithmetic/logic direct A operations (0x90-0x9B)
// Serie completa implementada: SUBA, ANDA, EORA, ORA, ADDA direct

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

fn setup_cpu_with_memory() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    Cpu6809::new(memory_bus)
}

#[cfg(test)]
mod suba_direct_tests {
    use super::*;

    #[test]
    fn test_suba_direct_0x90_basic() {
        let mut cpu = setup_cpu_with_memory();
        
        // Setup: A=0x50, [0xC810]=0x20
        cpu.registers_mut().a = 0x50;
        cpu.registers_mut().dp = 0xC8; // Direct page = 0xC8xx (in RAM range)
        cpu.registers_mut().pc = 0xC800;
        
        // Setup memory
        cpu.memory_bus().borrow_mut().write(0xC810, 0x20); // Data in RAM
        cpu.memory_bus().borrow_mut().write(0xC800, 0x90); // SUBA direct
        cpu.memory_bus().borrow_mut().write(0xC801, 0x10); // Direct address offset
        
        let _cycles = cpu.execute_instruction(false, false);
        
        // Result: A = 0x50 - 0x20 = 0x30
        assert_eq!(cpu.registers().a, 0x30);
        assert!(!cpu.registers().cc.z); // Not zero
        assert!(!cpu.registers().cc.n); // Not negative
        assert!(!cpu.registers().cc.c); // No borrow
        assert!(!cpu.registers().cc.v); // No overflow
        assert_eq!(cpu.registers().pc, 0xC802);
    }

    #[test]
    fn test_suba_direct_0x90_zero_result() {
        let mut cpu = setup_cpu_with_memory();
        
        // Setup: A=0x42, [0xC820]=0x42 (same value)
        cpu.registers_mut().a = 0x42;
        cpu.registers_mut().dp = 0xC8;
        cpu.registers_mut().pc = 0xC800;
        
        // Setup memory
        cpu.memory_bus().borrow_mut().write(0xC820, 0x42);
        cpu.memory_bus().borrow_mut().write(0xC800, 0x90);
        cpu.memory_bus().borrow_mut().write(0xC801, 0x20);
        
        let _cycles = cpu.execute_instruction(false, false);
        
        // Result: A = 0x42 - 0x42 = 0x00
        assert_eq!(cpu.registers().a, 0x00);
        assert!(cpu.registers().cc.z); // Zero flag set
        assert!(!cpu.registers().cc.n); // Not negative
        assert!(!cpu.registers().cc.c); // No borrow
        assert!(!cpu.registers().cc.v); // No overflow
    }

    #[test]
    fn test_suba_direct_0x90_borrow() {
        let mut cpu = setup_cpu_with_memory();
        
        // Setup: A=0x30, [0xC830]=0x50 (borrow case)
        cpu.registers_mut().a = 0x30;
        cpu.registers_mut().dp = 0xC8;
        cpu.registers_mut().pc = 0xC800;
        
        // Setup memory
        cpu.memory_bus().borrow_mut().write(0xC830, 0x50);
        cpu.memory_bus().borrow_mut().write(0xC800, 0x90);
        cpu.memory_bus().borrow_mut().write(0xC801, 0x30);
        
        let _cycles = cpu.execute_instruction(false, false);
        
        // Result: A = 0x30 - 0x50 = 0xE0 (with borrow)
        assert_eq!(cpu.registers().a, 0xE0);
        assert!(!cpu.registers().cc.z); // Not zero
        assert!(cpu.registers().cc.n); // Negative
        assert!(cpu.registers().cc.c); // Borrow occurred
        assert!(!cpu.registers().cc.v); // No overflow
    }
}

#[cfg(test)]
mod anda_direct_tests {
    use super::*;

    #[test]
    fn test_anda_direct_0x94_basic() {
        let mut cpu = setup_cpu_with_memory();
        
        // Setup: A=0xF0, [0xC840]=0x0F
        cpu.registers_mut().a = 0xF0;
        cpu.registers_mut().dp = 0xC8;
        cpu.registers_mut().pc = 0xC800;
        
        // Setup memory
        cpu.memory_bus().borrow_mut().write(0xC840, 0x0F);
        cpu.memory_bus().borrow_mut().write(0xC800, 0x94);
        cpu.memory_bus().borrow_mut().write(0xC801, 0x40);
        
        let _cycles = cpu.execute_instruction(false, false);
        
        // Result: A = 0xF0 & 0x0F = 0x00
        assert_eq!(cpu.registers().a, 0x00);
        assert!(cpu.registers().cc.z); // Zero flag set
        assert!(!cpu.registers().cc.n); // Not negative
        assert!(!cpu.registers().cc.v); // Overflow always cleared
    }

    #[test]
    fn test_anda_direct_0x94_partial_mask() {
        let mut cpu = setup_cpu_with_memory();
        
        // Setup: A=0xFF, [0xC850]=0xAA
        cpu.registers_mut().a = 0xFF;
        cpu.registers_mut().dp = 0xC8;
        cpu.registers_mut().pc = 0xC800;
        
        // Setup memory
        cpu.memory_bus().borrow_mut().write(0xC850, 0xAA);
        cpu.memory_bus().borrow_mut().write(0xC800, 0x94);
        cpu.memory_bus().borrow_mut().write(0xC801, 0x50);
        
        let _cycles = cpu.execute_instruction(false, false);
        
        // Result: A = 0xFF & 0xAA = 0xAA
        assert_eq!(cpu.registers().a, 0xAA);
        assert!(!cpu.registers().cc.z); // Not zero
        assert!(cpu.registers().cc.n); // Negative
        assert!(!cpu.registers().cc.v); // Overflow always cleared
    }
}

#[cfg(test)]
mod eora_direct_tests {
    use super::*;

    #[test]
    fn test_eora_direct_0x98_basic() {
        let mut cpu = setup_cpu_with_memory();
        
        // Setup: A=0xF0, [0xC860]=0x0F
        cpu.registers_mut().a = 0xF0;
        cpu.registers_mut().dp = 0xC8;
        cpu.registers_mut().pc = 0xC800;
        
        // Setup memory
        cpu.memory_bus().borrow_mut().write(0xC860, 0x0F);
        cpu.memory_bus().borrow_mut().write(0xC800, 0x98);
        cpu.memory_bus().borrow_mut().write(0xC801, 0x60);
        
        let _cycles = cpu.execute_instruction(false, false);
        
        // Result: A = 0xF0 ^ 0x0F = 0xFF
        assert_eq!(cpu.registers().a, 0xFF);
        assert!(!cpu.registers().cc.z); // Not zero
        assert!(cpu.registers().cc.n); // Negative
        assert!(!cpu.registers().cc.v); // Overflow always cleared
    }

    #[test]
    fn test_eora_direct_0x98_self_cancel() {
        let mut cpu = setup_cpu_with_memory();
        
        // Setup: A=0x55, [0xC870]=0x55 (XOR with self = 0)
        cpu.registers_mut().a = 0x55;
        cpu.registers_mut().dp = 0xC8;
        cpu.registers_mut().pc = 0xC800;
        
        // Setup memory
        cpu.memory_bus().borrow_mut().write(0xC870, 0x55);
        cpu.memory_bus().borrow_mut().write(0xC800, 0x98);
        cpu.memory_bus().borrow_mut().write(0xC801, 0x70);
        
        let _cycles = cpu.execute_instruction(false, false);
        
        // Result: A = 0x55 ^ 0x55 = 0x00
        assert_eq!(cpu.registers().a, 0x00);
        assert!(cpu.registers().cc.z); // Zero flag set
        assert!(!cpu.registers().cc.n); // Not negative
        assert!(!cpu.registers().cc.v); // Overflow always cleared
    }
}

#[cfg(test)]
mod ora_direct_tests {
    use super::*;

    #[test]
    fn test_ora_direct_0x9a_basic() {
        let mut cpu = setup_cpu_with_memory();
        
        // Setup: A=0xF0, [0xC880]=0x0F
        cpu.registers_mut().a = 0xF0;
        cpu.registers_mut().dp = 0xC8;
        cpu.registers_mut().pc = 0xC800;
        
        // Setup memory
        cpu.memory_bus().borrow_mut().write(0xC880, 0x0F);
        cpu.memory_bus().borrow_mut().write(0xC800, 0x9A);
        cpu.memory_bus().borrow_mut().write(0xC801, 0x80);
        
        let _cycles = cpu.execute_instruction(false, false);
        
        // Result: A = 0xF0 | 0x0F = 0xFF
        assert_eq!(cpu.registers().a, 0xFF);
        assert!(!cpu.registers().cc.z); // Not zero
        assert!(cpu.registers().cc.n); // Negative
        assert!(!cpu.registers().cc.v); // Overflow always cleared
    }

    #[test]
    fn test_ora_direct_0x9a_zero_input() {
        let mut cpu = setup_cpu_with_memory();
        
        // Setup: A=0x00, [0xC890]=0x00 (OR with zero = zero)
        cpu.registers_mut().a = 0x00;
        cpu.registers_mut().dp = 0xC8;
        cpu.registers_mut().pc = 0xC800;
        
        // Setup memory
        cpu.memory_bus().borrow_mut().write(0xC890, 0x00);
        cpu.memory_bus().borrow_mut().write(0xC800, 0x9A);
        cpu.memory_bus().borrow_mut().write(0xC801, 0x90);
        
        let _cycles = cpu.execute_instruction(false, false);
        
        // Result: A = 0x00 | 0x00 = 0x00
        assert_eq!(cpu.registers().a, 0x00);
        assert!(cpu.registers().cc.z); // Zero flag set
        assert!(!cpu.registers().cc.n); // Not negative
        assert!(!cpu.registers().cc.v); // Overflow always cleared
    }
}

#[cfg(test)]
mod adda_direct_tests {
    use super::*;

    #[test]
    fn test_adda_direct_0x9b_basic() {
        let mut cpu = setup_cpu_with_memory();
        
        // Setup: A=0x30, [0xC8A0]=0x20
        cpu.registers_mut().a = 0x30;
        cpu.registers_mut().dp = 0xC8;
        cpu.registers_mut().pc = 0xC800;
        
        // Setup memory
        cpu.memory_bus().borrow_mut().write(0xC8A0, 0x20);
        cpu.memory_bus().borrow_mut().write(0xC800, 0x9B);
        cpu.memory_bus().borrow_mut().write(0xC801, 0xA0);
        
        let _cycles = cpu.execute_instruction(false, false);
        
        // Result: A = 0x30 + 0x20 = 0x50
        assert_eq!(cpu.registers().a, 0x50);
        assert!(!cpu.registers().cc.z); // Not zero
        assert!(!cpu.registers().cc.n); // Not negative
        assert!(!cpu.registers().cc.c); // No carry
        assert!(!cpu.registers().cc.v); // No overflow
        assert_eq!(cpu.registers().pc, 0xC802);
    }

    #[test]
    fn test_adda_direct_0x9b_carry() {
        let mut cpu = setup_cpu_with_memory();
        
        // Setup: A=0xFF, [0xC8B0]=0x01 (carry case)
        cpu.registers_mut().a = 0xFF;
        cpu.registers_mut().dp = 0xC8;
        cpu.registers_mut().pc = 0xC800;
        
        // Setup memory
        cpu.memory_bus().borrow_mut().write(0xC8B0, 0x01);
        cpu.memory_bus().borrow_mut().write(0xC800, 0x9B);
        cpu.memory_bus().borrow_mut().write(0xC801, 0xB0);
        
        let _cycles = cpu.execute_instruction(false, false);
        
        // Result: A = 0xFF + 0x01 = 0x00 (with carry)
        assert_eq!(cpu.registers().a, 0x00);
        assert!(cpu.registers().cc.z); // Zero flag set
        assert!(!cpu.registers().cc.n); // Not negative
        assert!(cpu.registers().cc.c); // Carry occurred
        assert!(!cpu.registers().cc.v); // No overflow
    }

    #[test]
    fn test_adda_direct_0x9b_overflow() {
        let mut cpu = setup_cpu_with_memory();
        
        // Setup: A=0x7F, [0xC8C0]=0x01 (positive overflow case)
        cpu.registers_mut().a = 0x7F; // Max positive signed 8-bit
        cpu.registers_mut().dp = 0xC8;
        cpu.registers_mut().pc = 0xC800;
        
        // Setup memory
        cpu.memory_bus().borrow_mut().write(0xC8C0, 0x01);
        cpu.memory_bus().borrow_mut().write(0xC800, 0x9B);
        cpu.memory_bus().borrow_mut().write(0xC801, 0xC0);
        
        let _cycles = cpu.execute_instruction(false, false);
        
        // Result: A = 0x7F + 0x01 = 0x80 (signed overflow: +127 + 1 = -128)
        assert_eq!(cpu.registers().a, 0x80);
        assert!(!cpu.registers().cc.z); // Not zero
        assert!(cpu.registers().cc.n); // Negative (0x80 = -128 signed)
        assert!(!cpu.registers().cc.c); // No carry
        assert!(cpu.registers().cc.v); // Overflow occurred
    }
}

#[cfg(test)]
mod comprehensive_tests {
    use super::*;

    #[test]
    fn test_all_direct_opcodes_sequence() {
        let mut cpu = setup_cpu_with_memory();
        
        // Setup memory for all operations
        cpu.registers_mut().dp = 0xC8;
        cpu.registers_mut().a = 0xFF; // Initial A register
        cpu.registers_mut().pc = 0xC800;
        
        // Setup memory locations
        cpu.memory_bus().borrow_mut().write(0xC810, 0x0F); // For SUBA: 0xFF - 0x0F = 0xF0
        cpu.memory_bus().borrow_mut().write(0xC820, 0xF0); // For ANDA: 0xF0 & 0xF0 = 0xF0  
        cpu.memory_bus().borrow_mut().write(0xC830, 0x0F); // For EORA: 0xF0 ^ 0x0F = 0xFF
        cpu.memory_bus().borrow_mut().write(0xC840, 0x00); // For ORA: 0xFF | 0x00 = 0xFF
        cpu.memory_bus().borrow_mut().write(0xC850, 0x01); // For ADDA: 0xFF + 0x01 = 0x00 (with carry)
        
        // Program sequence: SUBA, ANDA, EORA, ORA, ADDA
        let mut pc = 0xC800;
        
        // SUBA direct 0x10
        cpu.memory_bus().borrow_mut().write(pc, 0x90); 
        cpu.memory_bus().borrow_mut().write(pc + 1, 0x10); 
        pc += 2;
        // ANDA direct 0x20
        cpu.memory_bus().borrow_mut().write(pc, 0x94); 
        cpu.memory_bus().borrow_mut().write(pc + 1, 0x20); 
        pc += 2;
        // EORA direct 0x30
        cpu.memory_bus().borrow_mut().write(pc, 0x98); 
        cpu.memory_bus().borrow_mut().write(pc + 1, 0x30); 
        pc += 2;
        // ORA direct 0x40
        cpu.memory_bus().borrow_mut().write(pc, 0x9A); 
        cpu.memory_bus().borrow_mut().write(pc + 1, 0x40); 
        pc += 2;
        // ADDA direct 0x50
        cpu.memory_bus().borrow_mut().write(pc, 0x9B); 
        cpu.memory_bus().borrow_mut().write(pc + 1, 0x50);
        
        // Execute SUBA
        let _cycles = cpu.execute_instruction(false, false);
        assert_eq!(cpu.registers().a, 0xF0);
        assert!(!cpu.registers().cc.z);
        assert!(cpu.registers().cc.n);
        
        // Execute ANDA
        let _cycles = cpu.execute_instruction(false, false);
        assert_eq!(cpu.registers().a, 0xF0);
        assert!(!cpu.registers().cc.z);
        assert!(cpu.registers().cc.n);
        assert!(!cpu.registers().cc.v); // V always cleared in ANDA
        
        // Execute EORA
        let _cycles = cpu.execute_instruction(false, false);
        assert_eq!(cpu.registers().a, 0xFF);
        assert!(!cpu.registers().cc.z);
        assert!(cpu.registers().cc.n);
        assert!(!cpu.registers().cc.v); // V always cleared in EORA
        
        // Execute ORA
        let _cycles = cpu.execute_instruction(false, false);
        assert_eq!(cpu.registers().a, 0xFF);
        assert!(!cpu.registers().cc.z);
        assert!(cpu.registers().cc.n);
        assert!(!cpu.registers().cc.v); // V always cleared in ORA
        
        // Execute ADDA
        let _cycles = cpu.execute_instruction(false, false);
        assert_eq!(cpu.registers().a, 0x00);
        assert!(cpu.registers().cc.z); // Zero result
        assert!(!cpu.registers().cc.n); // Not negative
        assert!(cpu.registers().cc.c); // Carry occurred
        assert!(!cpu.registers().cc.v); // No signed overflow
        
        assert_eq!(cpu.registers().pc, 0xC80A); // All 5 instructions executed (2 bytes each)
    }

    #[test]
    fn test_direct_page_register_usage() {
        let mut cpu = setup_cpu_with_memory();
        
        // Test with different direct page register
        cpu.registers_mut().dp = 0xC9; // Direct page = 0xC9xx
        cpu.registers_mut().a = 0x42;
        cpu.registers_mut().pc = 0xC800;
        
        // Setup memory: value at 0xC950 (DP=0xC9 + offset=0x50)
        cpu.memory_bus().borrow_mut().write(0xC950, 0x33);
        cpu.memory_bus().borrow_mut().write(0xC800, 0x90);
        cpu.memory_bus().borrow_mut().write(0xC801, 0x50);
        
        let _cycles = cpu.execute_instruction(false, false);
        
        // Result: A = 0x42 - 0x33 = 0x0F
        assert_eq!(cpu.registers().a, 0x0F);
        assert!(!cpu.registers().cc.z);
        assert!(!cpu.registers().cc.n);
        assert!(!cpu.registers().cc.c);
    }
}