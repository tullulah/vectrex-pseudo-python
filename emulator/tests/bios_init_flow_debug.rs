use std::collections::HashMap;
use vectrex_emulator::CPU;
use std::fs;

fn load_bios() -> Option<Vec<u8>> {
    let path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    match fs::read(path) { Ok(d)=>Some(d), Err(_)=>None }
}

#[test]
fn trace_bios_init_flow() {
    let bios = match load_bios() { 
        Some(b)=>b, 
        None => { 
            eprintln!("[SKIP] BIOS real no encontrada"); 
            return; 
        } 
    };
    
    let mut cpu = CPU::with_pc(0xF000);
    cpu.load_bios(&bios);
    
    let mut pc_counts = HashMap::new();
    let mut _last_pc = 0u16;
    let mut loop_detected = false;
    let mut step_count = 0;
    
    println!("=== BIOS Init Flow Trace ===");
    println!("Tracing PC addresses to detect infinite loops...");
    
    // Key addresses we expect to see:
    // F000: Start
    // F18B: Init_OS  
    // F164: Init_OS_RAM
    // F14C: Init_VIA
    // F1A2: Set_Refresh
    // F2E6: Recalibrate
    // F354: Reset0Ref
    
    let key_addresses = [
        (0xF000, "Start"),
        (0xF18B, "Init_OS"),
        (0xF164, "Init_OS_RAM"),
        (0xF14C, "Init_VIA"),
        (0xF1A2, "Set_Refresh"),
        (0xF2E6, "Recalibrate"),
        (0xF354, "Reset0Ref"),
        (0xF36B, "Reset0Int"),
        (0xF2F2, "Moveto_x_7F"),
    ];
    
    for _ in 0..50000 {
        let pc = cpu.pc;
        
        // Count PC occurrences
        *pc_counts.entry(pc).or_insert(0) += 1;
        
        // Check for key addresses
        for &(addr, name) in &key_addresses {
            if pc == addr {
                println!("Step {}: PC=${:04X} -> {}", step_count, pc, name);
            }
        }
        
        // Check for loops (PC visited more than 1000 times)
        if pc_counts[&pc] > 1000 && !loop_detected {
            println!("🔄 LOOP DETECTED at PC=${:04X} (visited {} times)", pc, pc_counts[&pc]);
            loop_detected = true;
            
            // Show surrounding addresses
            println!("Context around loop:");
            for addr in (pc.saturating_sub(10))..=(pc + 10) {
                if let Some(&count) = pc_counts.get(&addr) {
                    if count > 10 {
                        println!("  ${:04X}: {} visits", addr, count);
                    }
                }
            }
            break;
        }
        
        cpu.step();
        step_count += 1;
        _last_pc = pc;
    }
    
    println!("\n=== Summary ===");
    println!("Total steps: {}", step_count);
    println!("Final PC: ${:04X}", cpu.pc);
    
    // Show most visited addresses
    let mut sorted_pcs: Vec<_> = pc_counts.iter().collect();
    sorted_pcs.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\nTop 10 most visited addresses:");
    for (pc, count) in sorted_pcs.iter().take(10) {
        let label = key_addresses.iter()
            .find(|(addr, _)| addr == *pc)
            .map(|(_, name)| *name)
            .unwrap_or("Unknown");
        println!("  ${:04X}: {} visits ({})", pc, count, label);
    }
    
    if !loop_detected {
        panic!("No loop detected - BIOS should be stuck somewhere!");
    }
}

#[test]
fn trace_bios_init_detailed() {
    let bios = match load_bios() { 
        Some(b)=>b, 
        None => { 
            eprintln!("[SKIP] BIOS real no encontrada"); 
            return; 
        } 
    };
    
    let mut cpu = CPU::with_pc(0xF000);
    cpu.load_bios(&bios);
    
    println!("=== Detailed BIOS Init Trace ===");
    println!("Following exact execution path from Start through Init_OS...");
    
    // Target addresses for detailed tracing
    let init_sequence = [
        0xF000, // Start
        0xF18B, // Init_OS
        0xF164, // Init_OS_RAM 
        0xF14C, // Init_VIA
        0xF1A2, // Set_Refresh
        0xF2E6, // Recalibrate
    ];
    
    let mut current_target = 0;
    let mut step_count = 0;
    
    for _ in 0..10000 {
        let pc = cpu.pc;
        
        // Check if we've reached the next target
        if current_target < init_sequence.len() && pc == init_sequence[current_target] {
            println!("✓ Step {}: Reached ${:04X}", step_count, pc);
            current_target += 1;
        }
        
        // Stop tracing once we reach Recalibrate
        if pc == 0xF2E6 {
            println!("📍 Entered Recalibrate at step {}", step_count);
            
            // Trace next 50 steps inside Recalibrate
            for i in 0..50 {
                let pc = cpu.pc;
                let opcode = cpu.bus.mem[pc as usize];
                println!("  Cal+{}: PC=${:04X} op=${:02X}", i, pc, opcode);
                cpu.step();
            }
            break;
        }
        
        cpu.step();
        step_count += 1;
    }
    
    println!("\nInit sequence progress: {}/{}", current_target, init_sequence.len());
    if current_target < init_sequence.len() {
        println!("❌ Stuck before reaching ${:04X}", init_sequence[current_target]);
    }
}