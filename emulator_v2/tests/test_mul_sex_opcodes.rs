// Tests for MUL and SEX opcodes (Phase 8)
// C++ Original: multiply() and sex() functions in Vectrexy

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_cpu_with_memory() -> Cpu6809 {
        let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
        
        // Add RAM for test memory using the configured memory map
        let ram = Rc::new(RefCell::new(Ram::new()));
        Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
        
        Cpu6809::new(memory_bus)
    }

    #[test]
    fn test_mul_0x3d_basic() {
        let mut cpu = setup_cpu_with_memory();
        
        // C++ Original: multiply() - Test: 12 * 13 = 156 (0x9C)
        cpu.registers_mut().pc = 0xC800;
        cpu.registers_mut().a = 12;
        cpu.registers_mut().b = 13;
        
        // Write MUL opcode
        cpu.memory_bus().borrow_mut().write(0xC800, 0x3D);
        
        let cycles = cpu.execute_instruction(false, false);
        
        // Verify result in D register (A:B)
        let d_result = ((cpu.registers().a as u16) << 8) | (cpu.registers().b as u16);
        assert_eq!(d_result, 156, "MUL result should be 156");
        assert_eq!(cpu.registers().a, 0, "High byte should be 0");
        assert_eq!(cpu.registers().b, 156, "Low byte should be 156");
        
        // Check condition codes
        // C++ Original: CC.Carry = TestBits01(result, BITS(7)) where BITS(7) = 0x80
        // For 156 (0x009C), bit 7 is set (0x9C & 0x80 = 0x80), so C = true
        assert_eq!(cpu.registers().cc.z, false, "Z flag should be clear");
        assert_eq!(cpu.registers().cc.c, true, "C flag should be set (bit 7 of result)");
        assert_eq!(cycles, 11, "MUL should take 11 cycles");
    }

    #[test]
    fn test_mul_0x3d_overflow() {
        let mut cpu = setup_cpu_with_memory();
        
        // C++ Original: Test: 255 * 255 = 65025 (0xFE01)
        cpu.registers_mut().pc = 0xC800;
        cpu.registers_mut().a = 255;
        cpu.registers_mut().b = 255;
        
        cpu.memory_bus().borrow_mut().write(0xC800, 0x3D);
        
        let cycles = cpu.execute_instruction(false, false);
        
        // Verify result
        let d_result = ((cpu.registers().a as u16) << 8) | (cpu.registers().b as u16);
        assert_eq!(d_result, 65025, "MUL result should be 65025");
        assert_eq!(cpu.registers().a, 0xFE, "High byte should be 0xFE");
        assert_eq!(cpu.registers().b, 0x01, "Low byte should be 0x01");
        
        // Check condition codes 
        // C++ Original: CC.Carry = TestBits01(result, BITS(7)) where BITS(7) = 0x80
        // For 65025 (0xFE01), bit 7 is clear (0x01 & 0x80 = 0x00), so C = false
        assert_eq!(cpu.registers().cc.c, false, "C flag should be clear (bit 7 of result)");
        assert_eq!(cycles, 11, "MUL should take 11 cycles");
    }

    #[test]
    fn test_mul_0x3d_zero_result() {
        let mut cpu = setup_cpu_with_memory();
        
        // C++ Original: Test: 0 * 42 = 0
        cpu.registers_mut().pc = 0xC800;
        cpu.registers_mut().a = 0;
        cpu.registers_mut().b = 42;
        
        cpu.memory_bus().borrow_mut().write(0xC800, 0x3D);
        
        let cycles = cpu.execute_instruction(false, false);
        
        // Verify result
        let d_result = ((cpu.registers().a as u16) << 8) | (cpu.registers().b as u16);
        assert_eq!(d_result, 0, "MUL result should be 0");
        assert_eq!(cpu.registers().cc.z, true, "Z flag should be set");
        assert_eq!(cpu.registers().cc.c, false, "C flag should be clear");
        assert_eq!(cycles, 11, "MUL should take 11 cycles");
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
}