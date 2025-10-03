// Tests for ORCC and ANDCC opcodes (Phase 9)
// C++ Original: ORCC and ANDCC operations in Vectrexy

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
    fn test_orcc_0x1a_set_flags() {
        let mut cpu = setup_cpu_with_memory();
        
        // C++ Original: ORCC with immediate operand
        cpu.registers_mut().pc = 0xC800;
        // Start with clear condition codes
        cpu.registers_mut().cc.from_u8(0x00);
        
        // Write ORCC #$54 (set Interrupt mask and Zero flag)
        cpu.memory_bus().borrow_mut().write(0xC800, 0x1A); // ORCC opcode
        cpu.memory_bus().borrow_mut().write(0xC801, 0x54); // I=1, Z=1
        
        let cycles = cpu.execute_instruction(false, false);
        
        // Verify flags are set
        assert_eq!(cpu.registers().cc.i, true, "I flag should be set");
        assert_eq!(cpu.registers().cc.z, true, "Z flag should be set");
        assert_eq!(cpu.registers().cc.c, false, "C flag should remain clear");
        assert_eq!(cpu.registers().cc.n, false, "N flag should remain clear");
        assert_eq!(cycles, 3, "ORCC should take 3 cycles");
    }

    #[test]
    fn test_orcc_0x1a_preserve_existing() {
        let mut cpu = setup_cpu_with_memory();
        
        // C++ Original: ORCC preserves existing flags
        cpu.registers_mut().pc = 0xC800;
        // Start with some flags already set
        cpu.registers_mut().cc.c = true;
        cpu.registers_mut().cc.n = true;
        
        // Write ORCC #$04 (set Zero flag)
        cpu.memory_bus().borrow_mut().write(0xC800, 0x1A); // ORCC opcode
        cpu.memory_bus().borrow_mut().write(0xC801, 0x04); // Z=1
        
        let cycles = cpu.execute_instruction(false, false);
        
        // Verify new flag is set and existing flags preserved
        assert_eq!(cpu.registers().cc.z, true, "Z flag should be set");
        assert_eq!(cpu.registers().cc.c, true, "C flag should be preserved");
        assert_eq!(cpu.registers().cc.n, true, "N flag should be preserved");
        assert_eq!(cycles, 3, "ORCC should take 3 cycles");
    }

    #[test]
    fn test_andcc_0x1c_clear_flags() {
        let mut cpu = setup_cpu_with_memory();
        
        // C++ Original: ANDCC clears specified flags
        cpu.registers_mut().pc = 0xC800;
        // Start with all flags set
        cpu.registers_mut().cc.from_u8(0xFF);
        
        // Write ANDCC #$AB (clear Interrupt and Zero flags: ~0x54)
        cpu.memory_bus().borrow_mut().write(0xC800, 0x1C); // ANDCC opcode
        cpu.memory_bus().borrow_mut().write(0xC801, 0xAB); // Clear I and Z
        
        let cycles = cpu.execute_instruction(false, false);
        
        // Verify specified flags are cleared
        assert_eq!(cpu.registers().cc.i, false, "I flag should be cleared");
        assert_eq!(cpu.registers().cc.z, false, "Z flag should be cleared");
        
        // Verify other flags remain set
        assert_eq!(cpu.registers().cc.c, true, "C flag should remain set");
        assert_eq!(cpu.registers().cc.n, true, "N flag should remain set");
        assert_eq!(cpu.registers().cc.v, true, "V flag should remain set");
        assert_eq!(cycles, 3, "ANDCC should take 3 cycles");
    }

    #[test]
    fn test_andcc_0x1c_preserve_flags() {
        let mut cpu = setup_cpu_with_memory();
        
        // C++ Original: ANDCC preserves specified flags
        cpu.registers_mut().pc = 0xC800;
        // Start with specific flags set
        cpu.registers_mut().cc.from_u8(0x00);
        cpu.registers_mut().cc.c = true;
        cpu.registers_mut().cc.z = true;
        
        // Write ANDCC #$05 (preserve only C and Z flags)
        cpu.memory_bus().borrow_mut().write(0xC800, 0x1C); // ANDCC opcode
        cpu.memory_bus().borrow_mut().write(0xC801, 0x05); // Keep C and Z
        
        let cycles = cpu.execute_instruction(false, false);
        
        // Verify only specified flags remain
        assert_eq!(cpu.registers().cc.c, true, "C flag should be preserved");
        assert_eq!(cpu.registers().cc.z, true, "Z flag should be preserved");
        assert_eq!(cpu.registers().cc.n, false, "N flag should be cleared");
        assert_eq!(cpu.registers().cc.i, false, "I flag should be cleared");
        assert_eq!(cycles, 3, "ANDCC should take 3 cycles");
    }

    #[test]
    fn test_andcc_0x1c_clear_all() {
        let mut cpu = setup_cpu_with_memory();
        
        // C++ Original: ANDCC can clear all flags
        cpu.registers_mut().pc = 0xC800;
        // Start with all flags set
        cpu.registers_mut().cc.from_u8(0xFF);
        
        // Write ANDCC #$00 (clear all flags)
        cpu.memory_bus().borrow_mut().write(0xC800, 0x1C); // ANDCC opcode
        cpu.memory_bus().borrow_mut().write(0xC801, 0x00); // Clear all
        
        let cycles = cpu.execute_instruction(false, false);
        
        // Verify all flags are cleared
        assert_eq!(cpu.registers().cc.c, false, "C flag should be cleared");
        assert_eq!(cpu.registers().cc.v, false, "V flag should be cleared");
        assert_eq!(cpu.registers().cc.z, false, "Z flag should be cleared");
        assert_eq!(cpu.registers().cc.n, false, "N flag should be cleared");
        assert_eq!(cpu.registers().cc.i, false, "I flag should be cleared");
        assert_eq!(cpu.registers().cc.h, false, "H flag should be cleared");
        assert_eq!(cpu.registers().cc.f, false, "F flag should be cleared");
        assert_eq!(cpu.registers().cc.e, false, "E flag should be cleared");
        assert_eq!(cycles, 3, "ANDCC should take 3 cycles");
    }
}