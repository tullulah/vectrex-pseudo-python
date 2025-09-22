use vectrex_emulator::cpu6809::CPU;
use vectrex_emulator::opcode_meta::lookup_meta;

fn main() {
    // Test exacto de JSR de opcodes_all.rs línea 54
    let meta = lookup_meta(0xBD, None).unwrap();
    println!("JSR metadata: size={}, base_cycles={}", meta.size, meta.base_cycles);
    
    let mut cpu = CPU::default(); 
    cpu.pc = 0x0300; 
    cpu.test_write8(0x0300, 0xBD); 
    cpu.test_write8(0x0301, 0x12); 
    cpu.test_write8(0x0302, 0x34); 
    
    let before_cycles = cpu.cycles; 
    println!("Cycles before JSR: {}", before_cycles);
    
    let ok = cpu.step(); 
    assert!(ok, "JSR step failed"); 
    
    let cyc = (cpu.cycles - before_cycles) as u32; 
    println!("Cycles after JSR: {}, consumed: {}", cpu.cycles, cyc);
    
    assert_eq!(cyc as u8, meta.base_cycles, "JSR cycles mismatch: expected {}, got {}", meta.base_cycles, cyc); 
    assert_eq!(cpu.pc, 0x1234, "JSR target mismatch: expected 0x1234, got 0x{:04X}", cpu.pc);
    
    println!("✓ JSR test PASSED - cycles correctly applied!");
}