use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== EXTENDED COMPARISON (10,000 steps) ===");
    
    // Leer archivos de comparación extendida
    let emulator_file = "C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\emulator_comparison_10000_steps.txt";
    let jsvecx_file = "C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\comparison_10000_steps.txt";
    
    let emulator_lines = read_lines(emulator_file)?;
    let jsvecx_lines = read_lines(jsvecx_file)?;
    
    println!("Emulator lines: {}, JSVecx lines: {}", emulator_lines.len(), jsvecx_lines.len());
    
    let mut register_differences = HashMap::new();
    let mut instruction_differences = 0;
    let mut total_comparisons = 0;
    let mut clr_indexed_operations = 0;
    let mut x_register_changes = 0;
    
    let min_len = emulator_lines.len().min(jsvecx_lines.len());
    
    for i in 0..min_len {
        let emulator_line = &emulator_lines[i];
        let jsvecx_line = &jsvecx_lines[i];
        
        if emulator_line != jsvecx_line {
            total_comparisons += 1;
            
            // Detectar operaciones CLR indexed específicamente
            if emulator_line.contains("CLR") && emulator_line.contains(",X") {
                clr_indexed_operations += 1;
                println!("CLR indexed en línea {}: {} vs {}", i, emulator_line.trim(), jsvecx_line.trim());
                
                // Verificar cambios en registro X
                if let (Some(emu_x), Some(jsx_x)) = (extract_x_register(emulator_line), extract_x_register(jsvecx_line)) {
                    if emu_x != jsx_x {
                        x_register_changes += 1;
                        println!("  X register difference: {} vs {}", emu_x, jsx_x);
                    }
                }
            }
            
            // Analizar diferencias por tipo de registro
            analyze_register_differences(emulator_line, jsvecx_line, &mut register_differences);
            
            if emulator_line.contains("executed:") && jsvecx_line.contains("executed:") {
                instruction_differences += 1;
            }
        }
    }
    
    println!("\n=== RESUMEN ANÁLISIS EXTENDIDO ===");
    println!("Total diferencias encontradas: {}", total_comparisons);
    println!("Diferencias en instrucciones: {}", instruction_differences);
    println!("Operaciones CLR indexed detectadas: {}", clr_indexed_operations);
    println!("Cambios en registro X durante CLR indexed: {}", x_register_changes);
    
    println!("\n=== DIFERENCIAS POR REGISTRO ===");
    for (register, count) in &register_differences {
        println!("{}: {} diferencias", register, count);
    }
    
    // Verificar patrones específicos en las últimas 100 líneas
    println!("\n=== ANÁLISIS ÚLTIMAS 100 LÍNEAS ===");
    let start_idx = if min_len >= 100 { min_len - 100 } else { 0 };
    
    let mut recent_differences = 0;
    for i in start_idx..min_len {
        if emulator_lines[i] != jsvecx_lines[i] {
            recent_differences += 1;
            if recent_differences <= 5 {  // Mostrar solo las primeras 5
                println!("Línea {}: {} vs {}", i, emulator_lines[i].trim(), jsvecx_lines[i].trim());
            }
        }
    }
    println!("Diferencias en últimas 100 líneas: {}", recent_differences);
    
    Ok(())
}

fn read_lines(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().collect::<Result<Vec<_>, _>>()?)
}

fn extract_x_register(line: &str) -> Option<u16> {
    if let Some(start) = line.find("X=") {
        let rest = &line[start + 2..];
        if let Some(end) = rest.find(|c: char| !c.is_ascii_hexdigit()) {
            if let Ok(val) = u16::from_str_radix(&rest[..end], 16) {
                return Some(val);
            }
        }
    }
    None
}

fn analyze_register_differences(emu_line: &str, jsx_line: &str, differences: &mut HashMap<String, usize>) {
    let registers = ["PC=", "X=", "Y=", "U=", "S=", "A=", "B=", "DP=", "CC="];
    
    for reg in &registers {
        let emu_val = extract_register_value(emu_line, reg);
        let jsx_val = extract_register_value(jsx_line, reg);
        
        if emu_val != jsx_val && emu_val.is_some() && jsx_val.is_some() {
            *differences.entry(reg.trim_end_matches('=').to_string()).or_insert(0) += 1;
        }
    }
}

fn extract_register_value(line: &str, register: &str) -> Option<String> {
    if let Some(start) = line.find(register) {
        let rest = &line[start + register.len()..];
        if let Some(end) = rest.find(|c: char| c.is_whitespace() || c == ',' || c == ')') {
            return Some(rest[..end].to_string());
        }
    }
    None
}