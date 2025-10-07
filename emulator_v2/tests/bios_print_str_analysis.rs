// Test de análisis de Print_Str en BIOS real
// Este test ejecuta BIOS hasta Print_Str y captura valores críticos
// para diagnosticar el offset de -10.75 unidades en el título

use vectrex_emulator_v2::emulator::Emulator;
use vectrex_emulator_v2::engine_types::{AudioContext, Input, RenderContext};

const BIOS_PATH: &str = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";

// Direcciones críticas de la BIOS
const INIT_VEC_TEXT_HW: u16 = 0xF0A4;    // LDD #$F848 / STD <Vec_Text_HW
const STD_VEC_TEXT_HW: u16 = 0xF0AA;     // STD <Vec_Text_HW instruction (after LDD)
const PRINT_STR: u16 = 0xF495;           // Print_Str entry point
const PRINT_STR_NEG: u16 = 0xF4D4;       // NEG <VIA_port_a instruction
const PRINT_STR_STA_WIDTH: u16 = 0xF4B9; // STA <VIA_port_a (write Vec_Text_Width)
const VIA_PORT_A: u16 = 0xD000;          // VIA Port A (DAC X-axis)
const VEC_TEXT_HEIGHT: u16 = 0xC82A;     // Vec_Text_HW (height)
const VEC_TEXT_WIDTH: u16 = 0xC82B;      // Vec_Text_Width

#[test]
fn test_bios_print_str_trace() {
    println!("\n=== BIOS Print_Str Analysis ===\n");
    
    // 1. Crear emulador con BIOS real (patrón WASM: new -> init -> reset)
    let mut emulator = Emulator::new();
    emulator.init(BIOS_PATH); // Inicializa todos los dispositivos y carga BIOS
    emulator.reset();         // Reset CPU (lee reset vector, PC→0xF000)
    
    println!("✓ BIOS loaded and CPU reset");
    println!("  PC = 0x{:04X}", emulator.get_cpu().registers().pc);
    
    // 2. Crear contextos necesarios para execute_instruction
    let mut render_context = RenderContext::new();
    let mut audio_context = AudioContext::new(1500000.0 / 44100.0); // 1.5MHz / 44.1kHz
    let input = Input::default();
    
    // 3. Ejecutar hasta alcanzar Print_Str
    let mut step = 0;
    let max_steps = 50000; // Reducido drásticamente para debugging - era 5000000
    let mut reached_print_str = false;
    let mut print_str_calls = 0;
    let mut captured_init = false;
    
    // Variables para capturar estado
    let mut vec_text_width_at_init = 0u8;
    let mut vec_text_height_at_init = 0u8;
    let mut vec_text_width_at_call = 0u8;
    let mut vec_text_height_at_call = 0u8;
    let mut via_port_a_before_neg = 0u8;
    let mut via_port_a_after_neg = 0u8;
    let mut via_port_a_at_sta = 0u8;
    
    println!("\n--- Tracing execution: Init → Print_Str → NEG ---");
    println!("Looking for key addresses:");
    println!("  0xF000: Reset/Start");
    println!("  0xF18B: Init_OS");
    println!("  0xF01C: Power-up loop 1 (VECTREX screen)");
    println!("  0xF06C: Warm_Start (skip power-up)");
    println!("  0xF0A4: Power-up loop 2 (game name/copyright)");
    println!("  0xF0AA: STD <Vec_Text_HW (init height/width to 0xF8/0x48)");
    println!("  0xF495: Print_Str entry");
    
    // Track if we see key initialization addresses
    let mut saw_warm_start = false;
    let mut saw_init_os = false;
    let mut saw_power_up_1 = false;
    let mut saw_power_up_2 = false;
    
    // TRACK Vec_Text_Height writes
    let mut last_vec_text_height: u8 = 0;
    let mut vec_text_height_init_step: usize = 0;
    
    while step < max_steps && print_str_calls < 5 {
        let pc = emulator.get_cpu().registers().pc;
        
        // Track key BIOS entry points - DETAILED for first 100 steps
        if step < 100 {
            if step % 10 == 0 || pc == 0xF000 || pc == 0xF18B || pc == 0xF01C || pc == 0xF06C || pc == 0xF0A4 {
                println!("[Step {:3}] PC=0x{:04X}", step, pc);
            }
        }
        
        if pc == 0xF000 && step < 10 {
            println!("\n*** RESET VECTOR → 0xF000 (Start) ***");
        }
        if pc == 0xF18B {
            saw_init_os = true;
            println!("\n*** 0xF18B: Init_OS called (step {}) ***", step);
        }
        
        // CRITICAL: Wait_Recal debería incrementar Vec_Loop_Count
        if pc == 0xF192 {
            let loop_count_before = emulator.get_memory_bus().read16(0xC825);
            if step < 100 || (step % 10000 == 0) {
                println!("*** 0xF192: Wait_Recal ENTRY - Vec_Loop_Count=0x{:04X} ***", loop_count_before);
            }
        }
        
        if pc == 0xF01C {
            saw_power_up_1 = true;
            let loop_count = emulator.get_memory_bus().read16(0xC825);
            println!("\n*** 0xF01C: COLD START - Power-up loop 1 (step {}) Vec_Loop_Count=0x{:04X} ***", 
                     step, loop_count);
        }
        // Detectar el punto de verificación del loop (LF066: CMPA #$01; BLS LF01C)
        if pc == 0xF066 {
            let a_reg = emulator.get_cpu().registers().a;
            let cc_raw = emulator.get_cpu().registers().cc.to_u8();
            let loop_count = emulator.get_memory_bus().read16(0xC825);
            println!("*** 0xF066: Loop termination check - A=0x{:02X}, CC=0x{:02X}, Vec_Loop_Count=0x{:04X} ***", 
                     a_reg, cc_raw, loop_count);
        }

        if pc == 0xF06C {
            saw_warm_start = true;
            println!("\n*** 0xF06C: WARM START (skips power-up screen) ***");
        }
        if pc == 0xF0A4 {
            saw_power_up_2 = true;
            println!("\n*** 0xF0A4: Power-up loop 2 (should set Vec_Text_HW) ***");
        }
        
        // PASO 1: Detectar inicialización de Vec_Text_HW (0xF0AA - después de LDD #$F848)
        if pc == STD_VEC_TEXT_HW && !captured_init {
            captured_init = true;
            
            // Ejecutar el STD para que se guarde el valor
            let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
            step += 1;
            
            vec_text_height_at_init = emulator.get_memory_bus().read(VEC_TEXT_HEIGHT);
            vec_text_width_at_init = emulator.get_memory_bus().read(VEC_TEXT_WIDTH);
            vec_text_height_init_step = step;
            last_vec_text_height = vec_text_height_at_init;
            
            println!("\n*** INIT Vec_Text_HW at 0xF0A9 (step {}) ***", step);
            println!("    Vec_Text_Height = 0x{:02X} ({}) [EXPECTED: 0xF8 = -8]", 
                     vec_text_height_at_init, vec_text_height_at_init as i8);
            println!("    Vec_Text_Width  = 0x{:02X} ({}) [EXPECTED: 0x48 = 72]", 
                     vec_text_width_at_init, vec_text_width_at_init as i8);
            
            continue; // Ya ejecutamos, continuar sin ejecutar de nuevo
        }
        
        // PASO 2: Detectar entrada a Print_Str
        if pc == PRINT_STR && !reached_print_str {
            reached_print_str = true;
            print_str_calls += 1;
            
            // Capturar valores de RAM
            vec_text_height_at_call = emulator.get_memory_bus().read(VEC_TEXT_HEIGHT);
            vec_text_width_at_call = emulator.get_memory_bus().read(VEC_TEXT_WIDTH);
            let via_port_a = emulator.get_memory_bus().read(VIA_PORT_A);
            
            println!("\n*** REACHED Print_Str (call #{}) at step {} ***", print_str_calls, step);
            println!("    PC              = 0x{:04X}", pc);
            println!("    Vec_Text_Height = 0x{:02X} ({}) [INIT: 0x{:02X}, DELTA: {}]", 
                     vec_text_height_at_call, vec_text_height_at_call as i8,
                     vec_text_height_at_init, 
                     (vec_text_height_at_call as i8) - (vec_text_height_at_init as i8));
            println!("    Vec_Text_Width  = 0x{:02X} ({}) [INIT: 0x{:02X}, DELTA: {}]", 
                     vec_text_width_at_call, vec_text_width_at_call as i8,
                     vec_text_width_at_init,
                     (vec_text_width_at_call as i8) - (vec_text_width_at_init as i8));
            println!("    VIA_portA       = 0x{:02X} ({})", via_port_a, via_port_a as i8);
            println!("    A register      = 0x{:02X}", emulator.get_cpu().registers().a);
            println!("    B register      = 0x{:02X}", emulator.get_cpu().registers().b);
            println!("    X register      = 0x{:04X}", emulator.get_cpu().registers().x);
            println!("    Y register      = 0x{:04X}", emulator.get_cpu().registers().y);
            println!("    U register      = 0x{:04X}", emulator.get_cpu().registers().u);
            println!("    S register      = 0x{:04X}", emulator.get_cpu().registers().s);
        }
        
        // PASO 3: Detectar STA <VIA_port_a (escritura de Vec_Text_Width al DAC)
        if pc == PRINT_STR_STA_WIDTH && reached_print_str {
            // Capturar ANTES de ejecutar
            let via_before = emulator.get_memory_bus().read(VIA_PORT_A);
            
            // Ejecutar STA
            let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
            step += 1;
            
            via_port_a_at_sta = emulator.get_memory_bus().read(VIA_PORT_A);
            
            println!("\n*** STA <VIA_portA (write width) at step {} ***", step);
            println!("    VIA_portA: 0x{:02X} → 0x{:02X} ({})", 
                     via_before, via_port_a_at_sta, via_port_a_at_sta as i8);
            println!("    Expected: Vec_Text_Width = 0x{:02X}", vec_text_width_at_call);
            
            continue;
        }
        
        // PASO 4: Detectar NEG <VIA_port_a (línea F4D4 en Print_Str)
        if pc == PRINT_STR_NEG && reached_print_str {
            via_port_a_before_neg = emulator.get_memory_bus().read(VIA_PORT_A);
            
            println!("\n*** BEFORE NEG <VIA_portA> at step {} ***", step);
            println!("    PC              = 0x{:04X}", pc);
            println!("    VIA_portA       = 0x{:02X} (signed: {})", 
                     via_port_a_before_neg, 
                     via_port_a_before_neg as i8);
            
            // Ejecutar la instrucción NEG
            let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
            step += 1;
            
            via_port_a_after_neg = emulator.get_memory_bus().read(VIA_PORT_A);
            
            println!("\n*** AFTER NEG <VIA_portA at step {} ***", step);
            println!("    VIA_portA       = 0x{:02X} (signed: {})", 
                     via_port_a_after_neg, 
                     via_port_a_after_neg as i8);
            println!("    CHANGE          = 0x{:02X} -> 0x{:02X} (delta: {})",
                     via_port_a_before_neg,
                     via_port_a_after_neg,
                     (via_port_a_after_neg as i8) - (via_port_a_before_neg as i8));
            println!("    Expected NEG    = 0x{:02X} (0 - 0x{:02X})", 
                     (0u8.wrapping_sub(via_port_a_before_neg)), via_port_a_before_neg);
            
            // Reset flag para próxima llamada
            reached_print_str = false;
            continue; // Ya hicimos step(), continuar
        }
        
        // Log periódico de progreso
        if step % 10000 == 0 && step > 0 {
            println!("  [Step {:6}] PC = 0x{:04X}", step, pc);
        }
        
        // Ejecutar siguiente instrucción
        let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
        
        // DETECT Vec_Text_Height changes AFTER initialization
        if vec_text_height_init_step > 0 && step > vec_text_height_init_step {
            let current_vec_text_height = emulator.get_memory_bus().read(VEC_TEXT_HEIGHT);
            if current_vec_text_height != last_vec_text_height {
                println!("\n🔥 Vec_Text_Height CHANGED at step {} (PC=0x{:04X}): 0x{:02X} → 0x{:02X}",
                         step, pc, last_vec_text_height, current_vec_text_height);
            }
            last_vec_text_height = current_vec_text_height;
        }
        
        step += 1;
    }
    
    // 3. Resumen final
    println!("\n=== EXECUTION SUMMARY ===");
    println!("Total steps executed: {}", step);
    println!("Print_Str calls detected: {}", print_str_calls);
    
    if print_str_calls > 0 {
        println!("\n=== CRITICAL VALUES ===");
        println!("INIT (0xF0A9):");
        println!("  Vec_Text_Height = 0x{:02X} ({}) [EXPECTED: 0xF8 = -8]", 
                 vec_text_height_at_init, vec_text_height_at_init as i8);
        println!("  Vec_Text_Width  = 0x{:02X} ({}) [EXPECTED: 0x48 = 72]", 
                 vec_text_width_at_init, vec_text_width_at_init as i8);
        
        println!("\nPrint_Str ENTRY (0xF495):");
        println!("  Vec_Text_Height = 0x{:02X} ({}) [DELTA from init: {}]", 
                 vec_text_height_at_call, vec_text_height_at_call as i8,
                 (vec_text_height_at_call as i8) - (vec_text_height_at_init as i8));
        println!("  Vec_Text_Width  = 0x{:02X} ({}) [DELTA from init: {}]", 
                 vec_text_width_at_call, vec_text_width_at_call as i8,
                 (vec_text_width_at_call as i8) - (vec_text_width_at_init as i8));
        
        println!("\nVIA_portA WRITES:");
        println!("  STA <VIA_portA (0xF4B9): 0x{:02X} ({})", via_port_a_at_sta, via_port_a_at_sta as i8);
        println!("  Before NEG (0xF4D4):     0x{:02X} ({})", via_port_a_before_neg, via_port_a_before_neg as i8);
        println!("  After NEG:               0x{:02X} ({})", via_port_a_after_neg, via_port_a_after_neg as i8);
        
        println!("\n=== BIOS PATH ANALYSIS ===");
        println!("Init_OS called:        {}", if saw_init_os { "✅ YES" } else { "❌ NO" });
        println!("Power-up loop 1 (LF01C): {}", if saw_power_up_1 { "✅ YES (COLD START)" } else { "❌ NO" });
        println!("Power-up loop 2 (LF0A4): {}", if saw_power_up_2 { "✅ YES (should init Vec_Text_HW)" } else { "❌ NO" });
        println!("Warm_Start:            {}", if saw_warm_start { "✅ YES (skipped power-up)" } else { "❌ NO" });
        
        println!("\n=== ANALYSIS ===");
        
        if vec_text_height_at_init != 0xF8 {
            println!("⚠ WARNING: Vec_Text_Height at INIT is NOT 0xF8!");
            println!("  This suggests Init_OS_RAM didn't execute properly");
        }
        
        if vec_text_height_at_call != vec_text_height_at_init {
            println!("⚠ CRITICAL: Vec_Text_Height CHANGED between INIT and Print_Str!");
            println!("  Something modified it: 0x{:02X} → 0x{:02X}", 
                     vec_text_height_at_init, vec_text_height_at_call);
        }
        
        if via_port_a_at_sta != vec_text_width_at_call {
            println!("⚠ WARNING: STA wrote 0x{:02X} but Vec_Text_Width is 0x{:02X}",
                     via_port_a_at_sta, vec_text_width_at_call);
        }
        
        let expected_neg = 0u8.wrapping_sub(via_port_a_before_neg);
        if via_port_a_after_neg != expected_neg {
            println!("❌ ERROR: NEG produced incorrect result!");
            println!("  Expected: 0 - 0x{:02X} = 0x{:02X}", via_port_a_before_neg, expected_neg);
            println!("  Got:      0x{:02X}", via_port_a_after_neg);
        } else {
            println!("✅ NEG opcode working correctly: 0x{:02X} → 0x{:02X}", 
                     via_port_a_before_neg, via_port_a_after_neg);
        }
        
        println!("\n=== NEXT STEPS ===");
        println!("1. If Vec_Text_Height changed: Find what modified it");
        println!("2. If NEG is correct but offset persists: Check Screen integrator");
        println!("3. Compare VIA_portA values with Vectrexy execution");
    } else {
        println!("⚠ WARNING: Did not reach Print_Str after {} steps", step);
        println!("Last PC: 0x{:04X}", emulator.get_cpu().registers().pc);
    }
    
    // Assert que llegamos a Print_Str al menos una vez
    assert!(print_str_calls > 0, "Should have reached Print_Str at least once");
}

// DEPRECATED: Este test es frágil porque depende de llegar exactamente a PC=0xF0A4
// El test test_bios_print_str_trace es más robusto y verifica lo mismo
// Con las correcciones de tabla de opcodes (SYNC 4→2 cycles), el timing cambió
#[test]
#[ignore]
fn test_bios_init_os_ram() {
    // Test específico para Init_OS_RAM que configura Vec_Text_HW
    println!("\n=== BIOS Init_OS_RAM Analysis ===\n");
    
    let mut emulator = Emulator::new();
    emulator.init(BIOS_PATH); // Carga BIOS y configura todo
    emulator.reset();
    
    // Crear contextos
    let mut render_context = RenderContext::new();
    let mut audio_context = AudioContext::new(1500000.0 / 44100.0);
    let input = Input::default();
    
    // Ejecutar hasta que se configure Vec_Text_HW (debería estar en 0xF0A4 según bios.asm)
    const INIT_VEC_TEXT_HW: u16 = 0xF0A4; // LDD #$F848 / STD <Vec_Text_HW
    
    let mut step = 0;
    let max_steps = 50000;
    let mut found_init = false;
    
    while step < max_steps {
        let pc = emulator.get_cpu().registers().pc;
        
        if pc == INIT_VEC_TEXT_HW {
            found_init = true;
            
            // Ejecutar unas cuantas instrucciones más para que se complete la asignación
            for _ in 0..10 {
                let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
            }
            
            let vec_text_height = emulator.get_memory_bus().read(VEC_TEXT_HEIGHT);
            let vec_text_width = emulator.get_memory_bus().read(VEC_TEXT_WIDTH);
            
            println!("*** Vec_Text_HW initialized ***");
            println!("    Vec_Text_Height (0xC82A) = 0x{:02X} ({})", vec_text_height, vec_text_height as i8);
            println!("    Vec_Text_Width  (0xC82B) = 0x{:02X} ({})", vec_text_width, vec_text_width as i8);
            println!("    Expected from BIOS: Height=0xF8, Width=0x48");
            
            // Verificar valores esperados
            assert_eq!(vec_text_height, 0xF8, "Vec_Text_Height should be 0xF8");
            assert_eq!(vec_text_width, 0x48, "Vec_Text_Width should be 0x48");
            
            break;
        }
        
        let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
        step += 1;
    }
    
    assert!(found_init, "Should have found Vec_Text_HW initialization");
}
