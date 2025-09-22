#[cfg(test)]
mod extended_tests {
    use crate::emulator::Emulator;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn generate_extended_emulator_trace() {
        let mut emulator = Emulator::new();
        
        // Cargar BIOS
        let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
        match std::fs::read(bios_path) {
            Ok(bios_data) => {
                emulator.load_bios(&bios_data);
                println!("BIOS loaded successfully");
            }
            Err(e) => {
                panic!("Failed to load BIOS: {}", e);
            }
        }
        
        // Cargar ROM de prueba
        let rom_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\examples\triangle_aligned.bin";
        match std::fs::read(rom_path) {
            Ok(rom_data) => {
                emulator.load_cartridge(&rom_data);
                println!("ROM loaded successfully");
            }
            Err(e) => {
                panic!("Failed to load ROM: {}", e);
            }
        }
        
        let mut output = File::create("emulator_comparison_10000_steps.txt").unwrap();
        
        for step in 0..10000 {
            let pc_before = emulator.cpu.pc;
            let x_before = emulator.cpu.x;
            let a_before = emulator.cpu.a;
            let b_before = emulator.cpu.b;
            let dp_before = emulator.cpu.dp;
            let s_before = emulator.cpu.s;
            let u_before = emulator.cpu.u;
            let y_before = emulator.cpu.y;
            
            // Ejecutar un paso
            let success = emulator.step();
            if !success {
                println!("CPU halted at step {}", step);
                break;
            }
            
            let pc_after = emulator.cpu.pc;
            let x_after = emulator.cpu.x;
            
            // Escribir estado completo (sin CC por ahora)
            writeln!(output, "Step {}: PC={:04X} executed: Step{} | PC={:04X} X={:04X} Y={:04X} U={:04X} S={:04X} A={:02X} B={:02X} DP={:02X} cycles={}",
                step,
                pc_before,
                step,
                pc_after,
                x_after,
                y_before,
                u_before,
                s_before,
                a_before,
                b_before,
                dp_before,
                emulator.cpu.cycles
            ).unwrap();
            
            // Detectar cambios en registro X
            if x_before != x_after {
                writeln!(output, "  -> X register changed: {:04X} -> {:04X}", x_before, x_after).unwrap();
            }
            
            if step % 1000 == 0 {
                println!("Generated {} steps", step);
            }
        }
        
        println!("Extended emulator trace generated");
    }
}