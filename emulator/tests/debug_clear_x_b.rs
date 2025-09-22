use vectrex_emulator::cpu6809::CPU;

fn load_real_bios(cpu: &mut CPU) {
    let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let data = std::fs::read(path).expect("BIOS real requerida para test");
    assert_eq!(data.len(), 8192, "BIOS size inesperado");
    for (i, b) in data.iter().enumerate() { 
        let addr = 0xE000 + i as u16; 
        cpu.mem[addr as usize] = *b; 
        cpu.bus.mem[addr as usize] = *b; 
    }
    cpu.bios_present = true;
}

#[test]
fn debug_clear_x_b_infinite_loop() {
    let mut cpu = CPU::default();
    load_real_bios(&mut cpu);
    cpu.reset();
    
    // Run until we get to the Clear_x_b routine at F53F
    let mut steps = 0;
    while cpu.pc != 0xF53F && steps < 100 {
        cpu.step();
        steps += 1;
    }
    
    println!("=== Reached Clear_x_b at F53F after {} steps ===", steps);
    println!("PC: {:04X}, SP: {:04X}, DP: {:02X}", cpu.pc, cpu.s, cpu.dp);
    println!("A: {:02X}, B: {:02X} (D: {:04X}), X: {:04X}, Y: {:04X}, U: {:04X}", 
             cpu.a, cpu.b, ((cpu.a as u16) << 8) | (cpu.b as u16), cpu.x, cpu.y, cpu.u);
    println!("CC flags: Z:{} N:{} C:{} V:{}", cpu.cc_z, cpu.cc_n, cpu.cc_c, cpu.cc_v);
    
    // Step through the Clear_x_b routine instruction by instruction
    for i in 0..20 {
        let old_pc = cpu.pc;
        let old_d = ((cpu.a as u16) << 8) | (cpu.b as u16);
        let old_x = cpu.x;
        
        // Check what's the next instruction before stepping
        if cpu.pc == 0xF548 {
            let postbyte = cpu.mem[(cpu.pc + 1) as usize];
            println!("  CLR postbyte at {:04X}: {:02X} (reg_code={}, mode={})", 
                     cpu.pc + 1, postbyte, (postbyte >> 5) & 0x03, (postbyte >> 3) & 0x03);
        }
        
        cpu.step();
        
        let new_d = ((cpu.a as u16) << 8) | (cpu.b as u16);
        let new_x = cpu.x;
        
        println!("Step {}: {:04X} -> {:04X}, D: {:04X} -> {:04X}, X: {:04X} -> {:04X}, CC_N: {}", 
                 i, old_pc, cpu.pc, old_d, new_d, old_x, new_x, cpu.cc_n);
        
        // If we hit the infinite loop, show memory around X
        if cpu.pc == 0xF548 && i > 5 {
            println!("  Memory at X ({:04X}): {:02X}", cpu.x, cpu.mem[cpu.x as usize]);
            if new_d == 0 {
                println!("  *** D register is 0, this will loop forever! ***");
                break;
            }
            if new_d > 0x8000 {
                println!("  *** D register wrapped to large positive value! ***");
                break;
            }
        }
        
        if cpu.pc == old_pc {
            println!("  *** Stuck at same PC! ***");
            break;
        }
    }
    
    // Check what should be the correct setup for Clear_x_b
    println!("\n=== Analysis ===");
    let final_d = ((cpu.a as u16) << 8) | (cpu.b as u16);
    println!("Final D register: {:04X} (signed: {})", final_d, final_d as i16);
    println!("X register: {:04X}", cpu.x);
    println!("Expected: D should count down from some positive value to 0");
    println!("Memory at X: {:02X}", cpu.mem[cpu.x as usize]);
}