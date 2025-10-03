// Debug tests for MUL opcode C flag behavior
// Analyzing what the C flag should be according to MC6809 documentation

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
        let ram = Rc::new(RefCell::new(Ram::new()));
        Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
        Cpu6809::new(memory_bus)
    }

    #[test]
    fn debug_mul_step_by_step() {
        println!("\n=== DEBUG MUL OPCODE ===");
        
        // Test case 1: 12 * 13 = 156 (0x009C)
        let mut cpu = setup_cpu_with_memory();
        cpu.registers_mut().pc = 0xC800;
        cpu.registers_mut().a = 12;
        cpu.registers_mut().b = 13;
        cpu.memory_bus().borrow_mut().write(0xC800, 0x3D);
        
        println!("Test 1: 12 * 13");
        println!("Initial: A={}, B={}", cpu.registers().a, cpu.registers().b);
        
        cpu.execute_instruction(false, false);
        
        let d_result = ((cpu.registers().a as u16) << 8) | (cpu.registers().b as u16);
        println!("Result: D=0x{:04X} ({})", d_result, d_result);
        println!("Final: A=0x{:02X} ({}), B=0x{:02X} ({})", 
                 cpu.registers().a, cpu.registers().a,
                 cpu.registers().b, cpu.registers().b);
        println!("Bit 7 of result (0x80): {}", (d_result & 0x80) != 0);
        println!("Current C flag: {}", cpu.registers().cc.c);
        println!("Vectrexy expected: C = bit 7 of 16-bit result = {}", (156 & 0x80) != 0);
        
        // Test case 2: 255 * 255 = 65025 (0xFE01)
        let mut cpu2 = setup_cpu_with_memory();
        cpu2.registers_mut().pc = 0xC800;
        cpu2.registers_mut().a = 255;
        cpu2.registers_mut().b = 255;
        cpu2.memory_bus().borrow_mut().write(0xC800, 0x3D);
        
        println!("\nTest 2: 255 * 255");
        println!("Initial: A={}, B={}", cpu2.registers().a, cpu2.registers().b);
        
        cpu2.execute_instruction(false, false);
        
        let d_result2 = ((cpu2.registers().a as u16) << 8) | (cpu2.registers().b as u16);
        println!("Result: D=0x{:04X} ({})", d_result2, d_result2);
        println!("Final: A=0x{:02X} ({}), B=0x{:02X} ({})", 
                 cpu2.registers().a, cpu2.registers().a,
                 cpu2.registers().b, cpu2.registers().b);
        println!("Bit 7 of result (0x80): {}", (d_result2 & 0x80) != 0);
        println!("Current C flag: {}", cpu2.registers().cc.c);
        println!("Vectrexy expected: C = bit 7 of 16-bit result = {}", (65025 & 0x80) != 0);
        
        // Test case 3: 200 * 150 = 30000 (0x7530) - should have C=false since bit 7 is 0
        let mut cpu3 = setup_cpu_with_memory();
        cpu3.registers_mut().pc = 0xC800;
        cpu3.registers_mut().a = 200;
        cpu3.registers_mut().b = 150;
        cpu3.memory_bus().borrow_mut().write(0xC800, 0x3D);
        
        println!("\nTest 3: 200 * 150");
        println!("Initial: A={}, B={}", cpu3.registers().a, cpu3.registers().b);
        
        cpu3.execute_instruction(false, false);
        
        let d_result3 = ((cpu3.registers().a as u16) << 8) | (cpu3.registers().b as u16);
        println!("Result: D=0x{:04X} ({})", d_result3, d_result3);
        println!("Final: A=0x{:02X} ({}), B=0x{:02X} ({})", 
                 cpu3.registers().a, cpu3.registers().a,
                 cpu3.registers().b, cpu3.registers().b);
        println!("Bit 7 of result (0x80): {}", (d_result3 & 0x80) != 0);
        println!("Current C flag: {}", cpu3.registers().cc.c);
        println!("Vectrexy expected: C = bit 7 of 16-bit result = {}", (30000 & 0x80) != 0);
    }
}