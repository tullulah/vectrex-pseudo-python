/// Test JSR cycles - verifica que JSR absolute (0xBD) realmente consume 7 ciclos
/// Basado en el patrón de opcodes_all.rs línea 54
use vectrex_emulator::cpu6809::CPU;

#[test]
fn test_jsr_absolute_cycles() {
    // Usamos el patrón exact de opcodes_all.rs para JSR
    let mut cpu = CPU::default(); 
    cpu.pc = 0x0300; 
    cpu.test_write8(0x0300, 0xBD);  // JSR absolute
    cpu.test_write8(0x0301, 0x12);  // hi byte target
    cpu.test_write8(0x0302, 0x34);  // lo byte target
    
    let before_cycles = cpu.cycles; 
    let ok = cpu.step(); 
    assert!(ok, "JSR step() debería retornar true"); 
    
    let cyc = (cpu.cycles - before_cycles) as u32; 
    
    // Verificación principal: JSR debe consumir exactamente 7 ciclos
    assert_eq!(cyc, 7, "JSR absolute debería consumir 7 ciclos, pero consumió {}", cyc);
    
    // Verificación secundaria: PC debería saltar a target
    assert_eq!(cpu.pc, 0x1234, "JSR target debería ser $1234");

    println!("✓ JSR absolute consume correctamente 7 ciclos (antes del fix: {}, después: {})", 
             before_cycles, cpu.cycles);
}