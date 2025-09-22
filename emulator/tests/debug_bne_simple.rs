// Debug simple para BNE
use vectrex_emulator::cpu6809::CPU;

#[test]
fn debug_bne_simple() {
    println!("🔍 Debug BNE simple");
    
    let mut cpu = CPU::default();
    
    // Setup simple: B=1, DECB, BNE
    cpu.pc = 0x1000;
    cpu.b = 0x01;
    cpu.cc_z = false;
    
    // DECB + BNE que debe NOT take
    cpu.bus.mem[0x1000] = 0x5A;  // DECB
    cpu.bus.mem[0x1001] = 0x26;  // BNE
    cpu.bus.mem[0x1002] = 0xFC;  // offset -4
    cpu.bus.mem[0x1003] = 0x12;  // NOP después del BNE
    
    println!("Antes DECB: PC={:04X}, B={:02X}, Z={}", cpu.pc, cpu.b, cpu.cc_z);
    
    // Step 1: DECB
    let step1 = cpu.step();
    println!("Después DECB: step={}, PC={:04X}, B={:02X}, Z={}", step1, cpu.pc, cpu.b, cpu.cc_z);
    
    // Step 2: BNE (debe NOT take porque Z=true)
    let step2 = cpu.step();
    println!("Después BNE: step={}, PC={:04X}, B={:02X}, Z={}", step2, cpu.pc, cpu.b, cpu.cc_z);
    
    // Verificar que PC avanzó correctamente
    assert_eq!(cpu.pc, 0x1003, "PC debe estar en 1003 después de BNE not taken, pero está en {:04X}", cpu.pc);
}