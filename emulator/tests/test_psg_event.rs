//! Test que ejecuta instrucciones que activan el PSG y verifica el flag de evento de sonido (psg_env_just_finished)

use vectrex_emulator::cpu6809::CPU;
use std::fs;

/// Test para activar el PSG y verificar el flag de evento de envolvente (psg_env_just_finished)
#[test]
fn test_psg_env_just_finished_flag() {
    // Cargar BIOS real (ruta primaria, ver instrucciones)
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = fs::read(bios_path).expect("No se pudo cargar la BIOS. Verificar ruta.");

    let mut cpu = CPU::default();
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;

    // Configurar reset vector desde BIOS
    let reset_vector = ((cpu.bus.read8(0xFFFE) as u16) << 8) | (cpu.bus.read8(0xFFFF) as u16);
    cpu.pc = reset_vector;

    // Simular escritura en PSG para activar envolvente (canal A)
    // Canal A: volumen controlado por envolvente
    cpu.bus.psg.write_reg(8, 0x10); // Canal A: envolvente
    cpu.bus.psg.write_reg(13, 0x09); // Envelope shape: 0x09 (one-shot, attack)
    cpu.bus.psg.write_reg(11, 0xFF); // Envelope fine
    cpu.bus.psg.write_reg(12, 0xFF); // Envelope coarse

    // Ejecutar suficientes ciclos para que termine la envolvente
    let mut found = false;
    for _ in 0..500_000 {
        cpu.bus.psg.tick(1); // Avanzar PSG manualmente (sin step CPU)
        if cpu.bus.psg.take_env_just_finished() {
            found = true;
            break;
        }
    }
    assert!(found, "psg_env_just_finished flag no se activó tras la envolvente");
}
