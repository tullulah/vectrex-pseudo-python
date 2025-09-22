//! Test to verify auto_demo is disabled by default

use vectrex_emulator::cpu6809::CPU;

#[test]
fn test_auto_demo_default_disabled() {
    let cpu = CPU::default();
    assert_eq!(cpu.auto_demo, false, "auto_demo should be disabled by default");
    println!("✅ auto_demo está correctamente deshabilitado por defecto");
}