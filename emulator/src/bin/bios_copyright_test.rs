use vectrex_emulator::cpu6809::CPU;
use std::fs;

fn main() {
    println!("=== TEST BIOS COMPLETO HASTA COPYRIGHT ===");
    
    let bios_path = r"C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\dist\\bios.bin";
    let bios = fs::read(bios_path).expect("no se pudo leer bios.bin");
    
    let mut cpu = CPU::default();
    cpu.load_bios(&bios);
    cpu.reset();
    
    println!("🎯 Ejecutando BIOS hasta detección de copyright...");
    
    // Ejecutar con más pasos para ver la secuencia completa
    let mut step_count = 0;
    let max_steps = 1000000; // MUCHO más tiempo para ver el salto a Minestorm
    
    let mut in_wait_recal = false;
    let mut wait_recal_start = 0;
    let mut copyright_detection = false;
    let mut minestorm_detected = false;
    
    while step_count < max_steps {
        let pc_before = cpu.pc;
        cpu.step();
        step_count += 1;
        
        // Detectar entrada al Wait_Recal
        if !in_wait_recal && pc_before >= 0xF190 && pc_before <= 0xF1A5 {
            in_wait_recal = true;
            wait_recal_start = step_count;
            println!("📍 Entrando en Wait_Recal en paso {}, PC={:04X}", step_count, pc_before);
        }
        
        // Detectar salida de Wait_Recal
        if in_wait_recal && (pc_before < 0xF190 || pc_before > 0xF1A5) {
            println!("🎉 Saliendo de Wait_Recal en paso {}, PC={:04X} (duración: {} pasos)", 
                     step_count, pc_before, step_count - wait_recal_start);
            in_wait_recal = false;
        }
        
        // Buscar rutinas de copyright (probables en 0xF2xx-F3xx)
        if !copyright_detection && pc_before >= 0xF200 && pc_before <= 0xF400 {
            copyright_detection = true;
            println!("📝 Posible detección de copyright en PC={:04X}, paso {}", pc_before, step_count);
        }
        
        // Detectar copyright display específico
        if pc_before >= 0xF500 && pc_before <= 0xF520 && step_count > 10000 {
            println!("📺 Copyright display en PC={:04X}, paso {}", pc_before, step_count);
        }
        
        // Detectar chequeo de cartucho
        if pc_before >= 0xF06D && pc_before <= 0xF080 && step_count > 15000 {
            println!("🔍 Chequeo cartucho en PC={:04X}, paso {}", pc_before, step_count);
        }
        
        // Detectar salto a Minestorm (rango típico de Minestorm en BIOS)
        if !minestorm_detected && pc_before >= 0xF850 && step_count > 18000 {
            minestorm_detected = true;
            println!("🎮 MINESTORM DETECTADO! PC={:04X}, paso {}", pc_before, step_count);
            println!("🚀 Sin cartucho - saltando a juego integrado");
        }
        
        // Seguir monitoreando Minestorm por un rato
        if minestorm_detected && pc_before >= 0xF800 && step_count % 1000 == 0 {
            println!("🎯 Minestorm ejecutándose PC={:04X}, paso {}", pc_before, step_count);
        }
        
        // Instrumentación específica para el bucle de delay F4EB (menos verboso)
        if pc_before == 0xF4EB && step_count % 500 == 0 {
            let reg_b = cpu.b;
            println!("🔍 DELAY LOOP F4EB: registro B={:02X} ({}) en paso {}", reg_b, reg_b, step_count);
        }
        
        // Detectar algunos puntos clave de la BIOS
        match pc_before {
            0xF000 => println!("🔄 RESET vector en paso {}", step_count),
            0xF004 => println!("🚀 Después de Init_OS, verificando cold/warm start en paso {}", step_count),
            0xF006 => {
                let vec_cold_flag = ((cpu.bus.read8(0xC887) as u16) << 8) | (cpu.bus.read8(0xC888) as u16);
                println!("🌡️ Cold start check: Vec_Cold_Flag=${:04X} (esperado $7321) en paso {}", vec_cold_flag, step_count);
            },
            0xF008 => println!("✅ Branch a Warm_Start (es warm start) en paso {}", step_count),
            0xF00A => println!("❄️ Cold start - inicializando flags en paso {}", step_count),
            0xF018 => println!("📺 COLD START: First power-up loop (VECTREX) en paso {}", step_count),
            0xF06C => println!("🔥 WARM START en paso {}", step_count),
            0xF533 => println!("⚙️  Init_VIA completed en paso {}", step_count),
            0xF1A2 => println!("🕐 Set_Refresh en paso {}", step_count),
            0xF192 => println!("⏳ Wait_Recal start en paso {}", step_count),
            0xF1AF => println!("✅ Wait_Recal end (copyright check) en paso {}", step_count),
            0xF084 => println!("🔍 Verificación de cartucho COMIENZA en paso {}", step_count),
            0xF092 => println!("❌ Cartucho INVÁLIDO - saltando a Minestorm en paso {}", step_count),
            0xF09E => println!("🎵 Preparando música del juego en paso {}", step_count),
            0xF0A4 => {
                // CRÍTICO: Este es el main loop!
                let vec_music_flag = cpu.bus.read8(0xC888); // Vec_Music_Flag
                let vec_loop_count = ((cpu.bus.read8(0xC882) as u16) << 8) | (cpu.bus.read8(0xC883) as u16); // Vec_Loop_Count
                println!("🔄 COPYRIGHT MAIN LOOP F0A4 en paso {}. Music_Flag={}, Loop_Count={}", 
                         step_count, vec_music_flag, vec_loop_count);
            },
            0xF0D2 => {
                // Leer Vec_Run_Index para ver hacia dónde vamos (2 bytes)
                let vec_run_index = ((cpu.bus.read8(0xC880) as u16) << 8) | (cpu.bus.read8(0xC881) as u16);
                let vec_music_flag = cpu.bus.read8(0xC888);
                let vec_loop_count = ((cpu.bus.read8(0xC882) as u16) << 8) | (cpu.bus.read8(0xC883) as u16);
                println!("🎮 Preparando salto al juego en paso {}. Vec_Run_Index=${:04X}, Music_Flag={}, Loop_Count={}", 
                         step_count, vec_run_index, vec_music_flag, vec_loop_count);
            },
            0xF0DB => {
                // El salto final al juego!
                let vec_run_index = ((cpu.bus.read8(0xC880) as u16) << 8) | (cpu.bus.read8(0xC881) as u16);
                println!("🚀 SALTO FINAL AL JUEGO! JMP 1,U donde U=${:04X} en paso {}", vec_run_index, step_count);
            },
            _ => {}
        }
        
        // Mostrar progress cada 5000 pasos
        if step_count % 5000 == 0 {
            let ifr = cpu.bus.via_ifr();
            println!("📊 Paso {}: PC={:04X} IFR={:02X} ciclos={}", step_count, cpu.pc, ifr, cpu.cycles);
        }
        
        // Detectar si llegamos a una rutina de dibujo (probable en 0xF5xx+)
        if pc_before >= 0xF500 && pc_before < 0xF600 && step_count > 1000 {
            println!("🎨 Posible rutina de dibujo en PC={:04X}, paso {}", pc_before, step_count);
        }
        
        // Parar si hemos detectado Minestorm y llevamos un rato ejecutándolo
        if minestorm_detected && step_count > 30000 {
            println!("🏁 Stopping - Minestorm ejecutándose por {} pasos", step_count - 18000);
            break;
        }
    }
    
    if step_count >= max_steps {
        println!("⏰ Ejecutión terminada por timeout después de {} pasos", max_steps);
    }
    
    // Estado final
    let ifr = cpu.bus.via_ifr();
    println!("\n=== ESTADO FINAL ===");
    println!("📊 Pasos totales: {}", step_count);
    println!("📊 PC final: {:04X}", cpu.pc);
    println!("📊 Ciclos totales: {}", cpu.cycles);
    println!("📊 IFR final: {:02X}", ifr);
    
    // Verificar Timer2
    if (ifr & 0x20) != 0 {
        println!("✅ Timer2 expiró correctamente (IFR bit 5 set)");
    } else {
        println!("ℹ️  Timer2 estado normal (IFR bit 5 clear)");
    }
    
    // Mostrar algunos vectores integrator para ver si hay actividad gráfica
    let segments_count = cpu.integrator.segments.len();
    println!("🎨 Segmentos en integrator: {}", segments_count);
    
    if segments_count > 0 {
        println!("🎨 Primeros 5 segmentos:");
        for (i, seg) in cpu.integrator.segments.iter().take(5).enumerate() {
            println!("  {}. ({}, {}) → ({}, {}) intensidad={}", 
                     i+1, seg.x0, seg.y0, seg.x1, seg.y1, seg.intensity);
        }
    }
}