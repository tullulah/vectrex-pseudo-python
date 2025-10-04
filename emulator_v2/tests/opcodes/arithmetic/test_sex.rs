// Tests for MUL and SEX opcodes (Phase 8)
// C++ Original: multiply() and sex() functions in Vectrexy

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

    fn setup_cpu_with_memory() -> Cpu6809 {
        let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
        
        // Add RAM for test memory using the configured memory map
        let ram = Rc::new(RefCell::new(Ram::new()));
        Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
        
        Cpu6809::new(memory_bus)
    }

#[test]
    fn test_sex_0x1d_negative() {
        let mut cpu = setup_cpu_with_memory();
        
        // C++ Original: SEX with negative B (bit 7 = 1)
        cpu.registers_mut().pc = 0xC800;
        cpu.registers_mut().b = 0x80;  // Negative value
        
        cpu.memory_bus().borrow_mut().write(0xC800, 0x1D);
        
        let cycles = cpu.execute_instruction(false, false);
        
        // Verify result - A should be 0xFF for negative B
        assert_eq!(cpu.registers().a, 0xFF, "A should be 0xFF for negative B");
        assert_eq!(cpu.registers().b, 0x80, "B should remain unchanged");
        
        // Check condition codes
        assert_eq!(cpu.registers().cc.n, true, "N flag should be set");
        assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
        assert_eq!(cycles, 2, "SEX should take 2 cycles");
    }

#[test]
    fn test_sex_0x1d_positive() {
        let mut cpu = setup_cpu_with_memory();
        
        // C++ Original: SEX with positive B (bit 7 = 0)
        cpu.registers_mut().pc = 0xC800;
        cpu.registers_mut().b = 0x42;  // Positive value
        
        cpu.memory_bus().borrow_mut().write(0xC800, 0x1D);
        
        let cycles = cpu.execute_instruction(false, false);
        
        // Verify result - A should be 0x00 for positive B
        assert_eq!(cpu.registers().a, 0x00, "A should be 0x00 for positive B");
        assert_eq!(cpu.registers().b, 0x42, "B should remain unchanged");
        
        // Check condition codes
        assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
        assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
        assert_eq!(cycles, 2, "SEX should take 2 cycles");
    }

#[test]
    fn test_sex_0x1d_zero() {
        let mut cpu = setup_cpu_with_memory();
        
        // C++ Original: SEX with B = 0
        cpu.registers_mut().pc = 0xC800;
        cpu.registers_mut().b = 0x00;
        
        cpu.memory_bus().borrow_mut().write(0xC800, 0x1D);
        
        let cycles = cpu.execute_instruction(false, false);
        
        // Verify result
        assert_eq!(cpu.registers().a, 0x00, "A should be 0x00 for zero B");
        assert_eq!(cpu.registers().b, 0x00, "B should remain 0x00");
        
        // Check condition codes
        assert_eq!(cpu.registers().cc.n, false, "N flag should be clear");
        assert_eq!(cpu.registers().cc.z, true, "Z flag should be set");
        assert_eq!(cycles, 2, "SEX should take 2 cycles");
    }