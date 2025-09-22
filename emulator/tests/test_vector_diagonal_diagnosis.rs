// Test específico para diagnosticar vectores diagonales
// Ejecuta la BIOS real y rastrea las operaciones que afectan a los vectores

use vectrex_emulator::cpu6809::CPU;

fn load_real_bios(cpu: &mut CPU) {
    let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    match std::fs::read(path) {
        Ok(data) => {
            assert_eq!(data.len(), 8192, "BIOS size inesperado");
            for (i, b) in data.iter().enumerate() { 
                let addr = 0xE000 + i as u16; 
                cpu.bus.mem[addr as usize] = *b; 
            }
            cpu.bios_present = true;
            println!("BIOS cargada correctamente desde assets");
        }
        Err(_) => {
            // Fallback to dist path
            let alt_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
            let data = std::fs::read(alt_path).expect("BIOS real requerida para test");
            assert_eq!(data.len(), 8192, "BIOS size inesperado");
            for (i, b) in data.iter().enumerate() { 
                let addr = 0xE000 + i as u16; 
                cpu.bus.mem[addr as usize] = *b; 
            }
            cpu.bios_present = true;
            println!("BIOS cargada desde dist como fallback");
        }
    }
}

#[test]
fn diagnose_vector_coordinates() {
    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    cpu.trace = true;
    cpu.reset();
    
    println!("=== Diagnostic: Vector Coordinate Problem ===");
    
    // Set up a simple vector program in memory
    // This will draw a single vector from (0,0) to (100,100)
    
    // First, set up vector list in memory at 0x2000
    let vector_list_addr = 0x2000;
    
    // Vector 1: Move to (0, 0)
    cpu.bus.mem[vector_list_addr + 0] = 0x00; // Scale (dummy)
    cpu.bus.mem[vector_list_addr + 1] = 0x00; // Y coordinate high
    cpu.bus.mem[vector_list_addr + 2] = 0x00; // Y coordinate low  
    cpu.bus.mem[vector_list_addr + 3] = 0x00; // X coordinate high
    cpu.bus.mem[vector_list_addr + 4] = 0x00; // X coordinate low
    
    // Vector 2: Draw to (100, 100) with intensity
    cpu.bus.mem[vector_list_addr + 5] = 0x7F; // Scale + intensity
    cpu.bus.mem[vector_list_addr + 6] = 0x00; // Y coordinate high
    cpu.bus.mem[vector_list_addr + 7] = 0x64; // Y coordinate low (100)
    cpu.bus.mem[vector_list_addr + 8] = 0x00; // X coordinate high  
    cpu.bus.mem[vector_list_addr + 9] = 0x64; // X coordinate low (100)
    
    // End list
    cpu.bus.mem[vector_list_addr + 10] = 0x02; // End pattern
    cpu.bus.mem[vector_list_addr + 11] = 0x00;
    
    println!("Vector list setup at {:04X}:", vector_list_addr);
    for i in 0..12 {
        println!("  [{:04X}] = {:02X}", vector_list_addr + i, cpu.bus.mem[vector_list_addr + i]);
    }
    
    // Run BIOS until initialization complete
    let mut step_count = 0;
    let max_steps = 1000;
    
    println!("\n=== Running BIOS initialization ===");
    while step_count < max_steps {
        let pc_before = cpu.pc;
        let a_before = cpu.a;
        let b_before = cpu.b;
        let x_before = cpu.x;
        let _y_before = cpu.y;
        
        cpu.step();
        step_count += 1;
        
        // Monitor key vector-related operations
        if cpu.pc >= 0xF000 {
            // We're in vector processing BIOS code
            let d_val = (cpu.a as u16) << 8 | (cpu.b as u16);
            
            // Check for suspicious coordinate values
            if (a_before != cpu.a || b_before != cpu.b) && (d_val != 0) {
                println!("Step {}: PC {:04X} -> {:04X}", step_count, pc_before, cpu.pc);
                println!("  D changed: {:02X}{:02X} -> {:02X}{:02X} (D={:04X})", 
                        a_before, b_before, cpu.a, cpu.b, d_val);
                if d_val == 0x6464 {
                    println!("  *** DIAGONAL COORDINATES DETECTED! ***");
                }
            }
            
            // Check X register changes (often used for vector addresses)
            if x_before != cpu.x {
                println!("Step {}: PC {:04X}, X changed: {:04X} -> {:04X}", 
                        step_count, pc_before, x_before, cpu.x);
            }
        }
        
        // Stop if we hit a specific vector routine or loop
        if cpu.pc == 0xF192 { // Dot_ix_b routine
            println!("Reached Dot_ix_b at step {}", step_count);
            break;
        }
        if cpu.pc == 0xF1AA { // Dot_ix_b_no_move routine  
            println!("Reached Dot_ix_b_no_move at step {}", step_count);
            break;
        }
        if cpu.pc == 0xF15F { // Dot_ix_b_No_scale routine
            println!("Reached Dot_ix_b_No_scale at step {}", step_count);
            break;
        }
    }
    
    if step_count >= max_steps {
        println!("Timeout after {} steps at PC {:04X}", max_steps, cpu.pc);
    }
    
    println!("\n=== Final State Analysis ===");
    println!("Final PC: {:04X}", cpu.pc);
    println!("Final A: {:02X}, B: {:02X}, D: {:04X}", cpu.a, cpu.b, (cpu.a as u16) << 8 | (cpu.b as u16));
    println!("Final X: {:04X}, Y: {:04X}", cpu.x, cpu.y);
    println!("Steps executed: {}", step_count);
    
    // Check integrator state for diagonal bias
    let beam_segments = cpu.integrator.segments_slice();
    println!("\nIntegrator segments: {}", beam_segments.len());
    for (i, segment) in beam_segments.iter().take(5).enumerate() {
        let dx = segment.x1 - segment.x0;
        let dy = segment.y1 - segment.y0;
        println!("  Segment {}: ({:.2},{:.2}) -> ({:.2},{:.2}), dx={:.2}, dy={:.2}, intensity={:.2}", 
                i, segment.x0, segment.y0, segment.x1, segment.y1, dx, dy, segment.intensity);
        if dx != 0.0 && dy != 0.0 && (dx - dy).abs() < 0.01 {
            println!("    *** DIAGONAL SEGMENT DETECTED! ***");
        }
    }
    
    println!("=== Diagnostic Complete ===");
}

#[test]  
fn test_specific_vector_opcodes() {
    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    cpu.trace = true;
    
    println!("=== Testing Specific Vector-Related Opcodes ===");
    
    // Test indexed addressing modes that might affect vector calculations
    
    // Set up test values
    cpu.a = 0x00;
    cpu.b = 0x64; // 100 decimal
    cpu.x = 0x2000; // Base address
    
    // Test LDX immediate
    cpu.pc = 0x1000;
    cpu.bus.mem[0x1000] = 0x8E; // LDX immediate 
    cpu.bus.mem[0x1001] = 0x20; // High byte
    cpu.bus.mem[0x1002] = 0x00; // Low byte
    
    println!("Before LDX #$2000: X={:04X}", cpu.x);
    cpu.step();
    println!("After LDX #$2000: X={:04X}, PC={:04X}", cpu.x, cpu.pc);
    assert_eq!(cpu.x, 0x2000, "LDX should set X to 0x2000");
    
    // Test STB indexed
    cpu.bus.mem[0x1003] = 0xE7; // STB indexed 
    cpu.bus.mem[0x1004] = 0x05; // Offset 5
    
    println!("Before STB 5,X: X={:04X}, mem[{:04X}]={:02X}, B={:02X}", 
             cpu.x, cpu.x + 5, cpu.bus.mem[cpu.x as usize + 5], cpu.b);
    println!("Post-byte: {:02X}", cpu.bus.mem[0x1004]);
    cpu.step();
    println!("After STB 5,X: X={:04X}, mem[{:04X}]={:02X}, PC={:04X}", 
             cpu.x, cpu.x + 5, cpu.bus.mem[cpu.x as usize + 5], cpu.pc);
    assert_eq!(cpu.bus.mem[cpu.x as usize + 5], 0x64, "STB should store B at X+5");
    
    // Test LDB indexed
    cpu.bus.mem[0x1005] = 0xE6; // LDB indexed
    cpu.bus.mem[0x1006] = 0x05; // Offset 5
    cpu.b = 0x00; // Clear B first
    
    println!("Before LDB 5,X: B={:02X}, mem[{:04X}]={:02X}", 
             cpu.b, cpu.x + 5, cpu.bus.mem[cpu.x as usize + 5]);
    cpu.step();
    println!("After LDB 5,X: B={:02X}, PC={:04X}", cpu.b, cpu.pc);
    assert_eq!(cpu.b, 0x64, "LDB should load B from X+5");
    
    println!("=== Vector Opcode Test Complete ===");
}