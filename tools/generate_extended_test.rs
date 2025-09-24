use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating extended comparison files (10,000 steps)...");
    
    // Crear archivo de test para generar comparación extendida
    let test_content = r#"
#[cfg(test)]
mod tests {
    use crate::vectrex::Vectrex;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn generate_extended_emulator_trace() {
        let mut vectrex = Vectrex::new();
        
        // Cargar BIOS
        let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
        match std::fs::read(bios_path) {
            Ok(bios_data) => {
                vectrex.bus.load_bios(&bios_data);
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
                vectrex.bus.load_cartridge(&rom_data);
                println!("ROM loaded successfully");
            }
            Err(e) => {
                panic!("Failed to load ROM: {}", e);
            }
        }
        
        let mut output = File::create("emulator_comparison_10000_steps.txt").unwrap();
        
        for step in 0..10000 {
            let pc_before = vectrex.cpu.pc;
            let x_before = vectrex.cpu.x;
            let a_before = vectrex.cpu.a;
            let b_before = vectrex.cpu.b;
            let dp_before = vectrex.cpu.dp;
            let cc_before = vectrex.cpu.cc;
            let s_before = vectrex.cpu.s;
            let u_before = vectrex.cpu.u;
            let y_before = vectrex.cpu.y;
            
            // Obtener información de la instrucción antes de ejecutar
            let opcode = vectrex.bus.read_byte(pc_before);
            let (instruction, _) = vectrex.cpu.disassemble_instruction(&vectrex.bus, pc_before);
            
            // Ejecutar un paso
            let cycles = vectrex.step();
            
            let pc_after = vectrex.cpu.pc;
            let x_after = vectrex.cpu.x;
            
            // Escribir estado completo
            writeln!(output, "Step {}: PC={:04X} executed: {} | PC={:04X} X={:04X} Y={:04X} U={:04X} S={:04X} A={:02X} B={:02X} DP={:02X} CC={:02X} cycles={}",
                step,
                pc_before,
                instruction,
                pc_after,
                x_after,
                y_before,
                u_before,
                s_before,
                a_before,
                b_before,
                dp_before,
                cc_before,
                cycles
            ).unwrap();
            
            // Detectar específicamente operaciones CLR indexed
            if instruction.contains("CLR") && instruction.contains(",X") {
                writeln!(output, "  -> CLR indexed detected: X {} -> {}", x_before, x_after).unwrap();
            }
            
            if step % 1000 == 0 {
                println!("Generated {} steps", step);
            }
        }
        
        println!("Extended emulator trace generated");
    }
}
"#;
    
    // Escribir el test en el archivo del emulador
    let test_file_path = "C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\emulator\\src\\extended_test.rs";
    let mut test_file = File::create(test_file_path)?;
    test_file.write_all(test_content.as_bytes())?;
    
    println!("Extended test file created at: {}", test_file_path);
    
    Ok(())
}
"#