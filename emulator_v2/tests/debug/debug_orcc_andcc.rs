// Debug test for ORCC and ANDCC opcodes
use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod debug_tests {
    use super::*;

    fn setup_cpu_with_memory() -> Cpu6809 {
        let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
        
        // Add RAM for test memory using the configured memory map
        let ram = Rc::new(RefCell::new(Ram::new()));
        Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
        
        Cpu6809::new(memory_bus)
    }

    #[test]
    fn debug_orcc_step_by_step() {
        let mut cpu = setup_cpu_with_memory();
        
        println!("=== DEBUGGING ORCC STEP BY STEP ===");
        
        // Setup
        cpu.registers_mut().pc = 0xC800;
        cpu.registers_mut().cc.from_u8(0x00); // Clear all flags
        
        println!("Initial CC: {:02X}", cpu.registers().cc.to_u8());
        println!("Initial Z flag: {}", cpu.registers().cc.z);
        
        // Write ORCC #$04 (set only Z flag)
        cpu.memory_bus().borrow_mut().write(0xC800, 0x1A); // ORCC opcode
        cpu.memory_bus().borrow_mut().write(0xC801, 0x04); // Z=1 (bit 2)
        
        println!("Operand written: 0x04");
        println!("Expected after OR: {:02X}", 0x00 | 0x04);
        
        // Execute instruction
        let cycles = cpu.execute_instruction(false, false);
        
        println!("Final CC: {:02X}", cpu.registers().cc.to_u8());
        println!("Final Z flag: {}", cpu.registers().cc.z);
        println!("Cycles: {}", cycles);
        
        // Debug: manually check each bit
        let cc_value = cpu.registers().cc.to_u8();
        println!("CC bits: C={} V={} Z={} N={} I={} H={} F={} E={}", 
                 cc_value & 0x01 != 0,
                 cc_value & 0x02 != 0,
                 cc_value & 0x04 != 0,
                 cc_value & 0x08 != 0,
                 cc_value & 0x10 != 0,
                 cc_value & 0x20 != 0,
                 cc_value & 0x40 != 0,
                 cc_value & 0x80 != 0);
    }

    #[test] 
    fn debug_andcc_step_by_step() {
        let mut cpu = setup_cpu_with_memory();
        
        println!("=== DEBUGGING ANDCC STEP BY STEP ===");
        
        // Setup
        cpu.registers_mut().pc = 0xC800;
        cpu.registers_mut().cc.from_u8(0xFF); // Set all flags
        
        println!("Initial CC: {:02X}", cpu.registers().cc.to_u8());
        println!("Initial Z flag: {}", cpu.registers().cc.z);
        
        // Write ANDCC #$FB (clear only Z flag, keep others: ~0x04 = 0xFB)
        cpu.memory_bus().borrow_mut().write(0xC800, 0x1C); // ANDCC opcode
        cpu.memory_bus().borrow_mut().write(0xC801, 0xFB); // Clear Z (bit 2)
        
        println!("Operand written: 0xFB");
        println!("Expected after AND: {:02X}", 0xFF & 0xFB);
        
        // Execute instruction
        let cycles = cpu.execute_instruction(false, false);
        
        println!("Final CC: {:02X}", cpu.registers().cc.to_u8());
        println!("Final Z flag: {}", cpu.registers().cc.z);
        println!("Cycles: {}", cycles);
        
        // Debug: manually check each bit
        let cc_value = cpu.registers().cc.to_u8();
        println!("CC bits: C={} V={} Z={} N={} I={} H={} F={} E={}", 
                 cc_value & 0x01 != 0,
                 cc_value & 0x02 != 0,
                 cc_value & 0x04 != 0,
                 cc_value & 0x08 != 0,
                 cc_value & 0x10 != 0,
                 cc_value & 0x20 != 0,
                 cc_value & 0x40 != 0,
                 cc_value & 0x80 != 0);
    }
}