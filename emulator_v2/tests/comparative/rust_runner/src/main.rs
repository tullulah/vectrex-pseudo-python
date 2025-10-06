// Rust Comparative Test Runner
// Ejecuta un .bin y serializa el estado a JSON para comparación con Vectrexy

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::process;
use vectrex_emulator_v2::core::emulator::Emulator;
use vectrex_emulator_v2::core::engine_types::{AudioContext, Input, RenderContext};

#[derive(Serialize, Deserialize)]
struct ConditionCodes {
    c: bool, // Carry
    v: bool, // Overflow
    z: bool, // Zero
    n: bool, // Negative
    i: bool, // IRQ mask
    h: bool, // Half carry
    f: bool, // FIRQ mask
    e: bool, // Entire flag
}

#[derive(Serialize, Deserialize)]
struct CpuState {
    pc: u16,
    a: u8,
    b: u8,
    x: u16,
    y: u16,
    u: u16,
    s: u16,
    dp: u8,
    cc: ConditionCodes,
}

#[derive(Serialize, Deserialize)]
struct ViaState {
    ifr: u8,
    ier: u8,
    timer1_counter: u16,
    timer2_counter: u16,
    port_a: u8,
    port_b: u8,
    shift_register: u8,
}

#[derive(Serialize, Deserialize)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize)]
struct Line {
    p0: Point,
    p1: Point,
    brightness: f32,
}

#[derive(Serialize, Deserialize)]
struct Vectors {
    count: usize,
    lines: Vec<Line>,
}

#[derive(Serialize, Deserialize)]
struct TestOutput {
    cycles: u64,
    cpu: CpuState,
    via: ViaState,
    vectors: Vectors,
    audio_samples: usize,
}

fn serialize_cpu(emulator: &mut Emulator) -> CpuState {
    let cpu = emulator.get_cpu();
    let regs = cpu.registers();
    
    CpuState {
        pc: regs.pc,
        a: regs.a,
        b: regs.b,
        x: regs.x,
        y: regs.y,
        u: regs.u,
        s: regs.s,
        dp: regs.dp,
        cc: ConditionCodes {
            c: regs.cc.c,
            v: regs.cc.v,
            z: regs.cc.z,
            n: regs.cc.n,
            i: regs.cc.i,
            h: regs.cc.h,
            f: regs.cc.f,
            e: regs.cc.e,
        },
    }
}

fn serialize_via(emulator: &mut Emulator) -> ViaState {
    // C++ Original: Via.cpp Read() via MemoryBus
    // Leer registros VIA igual que Vectrexy - via memory bus, NO getters directos
    let memory = emulator.get_memory_bus();
    
    // C++ Original: case Register::Timer1Low/Timer1High
    let timer1_low = memory.read(0xD004);
    let timer1_high = memory.read(0xD005);
    let timer1_counter = ((timer1_high as u16) << 8) | (timer1_low as u16);
    
    // C++ Original: case Register::Timer2Low/Timer2High
    let timer2_low = memory.read(0xD008);
    let timer2_high = memory.read(0xD009);
    let timer2_counter = ((timer2_high as u16) << 8) | (timer2_low as u16);
    
    // C++ Original: case Register::Shift
    let shift_register = memory.read(0xD00A);
    
    ViaState {
        ifr: memory.read(0xD00D),          // Register::InterruptFlag
        ier: memory.read(0xD00E),          // Register::InterruptEnable
        timer1_counter,                     // Combinación de Timer1Low + Timer1High
        timer2_counter,                     // Combinación de Timer2Low + Timer2High
        port_a: memory.read(0xD001),       // Register::PortA
        port_b: memory.read(0xD000),       // Register::PortB
        shift_register,                     // Register::Shift
    }
}

fn serialize_vectors(render_context: &RenderContext) -> Vectors {
    let lines: Vec<Line> = render_context
        .lines
        .iter()
        .map(|line| Line {
            p0: Point {
                x: line.p0.x,
                y: line.p0.y,
            },
            p1: Point {
                x: line.p1.x,
                y: line.p1.y,
            },
            brightness: line.brightness,
        })
        .collect();

    Vectors {
        count: lines.len(),
        lines,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <test.bin> <cycles> [--bios-only] [--trace]", args[0]);
        eprintln!("Example: {} irq_test.bin 1000", args[0]);
        eprintln!("         {} test.bin 2500000 --bios-only  (execute BIOS from reset vector)", args[0]);
        eprintln!("         {} test.bin 1000 --trace  (log every instruction to stderr)", args[0]);
        process::exit(1);
    }

    let test_bin_path = &args[1];
    let cycles_to_run: u64 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("Error: cycles must be a number");
        process::exit(1);
    });
    
    // Check for flags
    let mut bios_only_mode = false;
    let mut trace_mode = false;
    
    for arg in args.iter().skip(3) {
        match arg.as_str() {
            "--bios-only" => bios_only_mode = true,
            "--trace" => trace_mode = true,
            _ => {}
        }
    }

    // Cargar BIOS (8KB rom.dat copiada como bios.bin)
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    
    // Inicializar emulador  
    let mut emulator = Emulator::new();
    
    // CRITICAL: Llamar a init() para mapear dispositivos al memory bus
    emulator.init(bios_path);
    
    // CRITICAL: Llamar a reset() para inicializar VIA (port_b RampDisabled, timers)
    // Necesario si el test llama a funciones BIOS como Wait_Recal
    emulator.reset();

    if bios_only_mode {
        // BIOS-only mode: Do NOT load test.bin, do NOT override PC
        // PC is already set to 0xF000 by reset() (BIOS reset vector)
        eprintln!("BIOS-only mode: Executing from RESET vector (PC=0xF000)");
    } else {
        // Normal mode: Load test code to RAM and set PC to 0xC800
        // Cargar test binary
        let test_code = fs::read(test_bin_path).unwrap_or_else(|e| {
            eprintln!("Error loading test binary: {}", e);
            process::exit(1);
        });

        // Escribir test code a RAM (0xC800-0xCFFF)
        const RAM_START: u16 = 0xC800;
        for (i, &byte) in test_code.iter().enumerate() {
            emulator.get_memory_bus().write(RAM_START + i as u16, byte);
        }

        // Setear PC al inicio del test code
        emulator.get_cpu().registers_mut().pc = RAM_START;
    }

    // Preparar contextos
    let input = Input::default();
    let mut render_context = RenderContext::default();
    let mut audio_context = AudioContext::new(1.0); // Dummy sample rate

    // Ejecutar N ciclos
    let mut total_cycles: u64 = 0;
    let mut instruction_count: u64 = 0;

    while total_cycles < cycles_to_run {
        let pc_before = emulator.get_cpu().registers().pc;
        let opcode = emulator.get_memory_bus().read(pc_before);
        
        instruction_count += 1;  // Incrementar ANTES para empezar en #1 (como Vectrexy)
        
        if trace_mode {
            // Formato unificado con Vectrexy: minúsculas en hex, contador desde #1
            eprintln!("[TRACE] Instr #{}: PC=0x{:04x} Opcode=0x{:02x}", 
                      instruction_count, pc_before, opcode);
        }
        
        // CRITICAL DEBUG: Log VIA reads for BITB in critical zone (instr #17450-17470)
        let dp_before = emulator.get_cpu().registers().dp;
        let b_before = emulator.get_cpu().registers().b;
        
        match emulator.execute_instruction(&input, &mut render_context, &mut audio_context) {
            Ok(cycles) => {
                total_cycles += cycles;
                
                // CRITICAL DEBUG: Log BITB/BITA reads from VIA (opcodes 0xD5/0x95, instr 9500-17500)
                if trace_mode && instruction_count >= 9500 && instruction_count <= 17500 {
                    if (pc_before == 0xF33D && opcode == 0xD5) || (pc_before == 0xF19E && opcode == 0x95) {
                        // BITB/BITA reading from direct page address
                        // Next byte should be 0x0D (VIA Timer 1 Counter High at DP:0x0D = 0xD00D)
                        let addr_offset = emulator.get_memory_bus().read(pc_before + 1);
                        let full_addr = ((dp_before as u16) << 8) | (addr_offset as u16);
                        let via_value = emulator.get_memory_bus().read(full_addr);
                        let result = b_before & via_value;
                        let z_flag = emulator.get_cpu().registers().cc.z;
                        let opname = if opcode == 0xD5 { "BITB" } else { "BITA" };
                        let reg_val = if opcode == 0xD5 { b_before } else { emulator.get_cpu().registers().a };
                        eprintln!("  [VIA READ DEBUG] {} @0x{:04x}: Reg=0x{:02x} & VIA[0x{:04x}]=0x{:02x} = 0x{:02x}, Z={}", 
                            opname, pc_before, reg_val, full_addr, via_value, result, z_flag as u8);
                    }
                }
                
                // Log registros DESPUÉS de ejecutar si estamos en zona crítica
                if trace_mode && pc_before >= 0xF340 && pc_before <= 0xF350 {
                    let cpu = emulator.get_cpu();
                    let regs = cpu.registers();
                    eprintln!("  → A={:02x} B={:02x} X={:04x} Y={:04x} DP={:02x} CC=N{}Z{}V{}C{}", 
                        regs.a, regs.b, regs.x, regs.y, regs.dp,
                        regs.cc.n as u8, regs.cc.z as u8, regs.cc.v as u8, regs.cc.c as u8);
                }
            }
            Err(e) => {
                eprintln!("CPU Error at cycle {}: {:?}", total_cycles, e);
                if trace_mode {
                    eprintln!("[TRACE] ERROR at instr #{}: PC=0x{:04x} Opcode=0x{:02x}", 
                              instruction_count, pc_before, opcode);
                }
                break;
            }
        }
    }    // Serializar estado a JSON
    let output = TestOutput {
        cycles: total_cycles,
        cpu: serialize_cpu(&mut emulator),
        via: serialize_via(&mut emulator),
        vectors: serialize_vectors(&render_context),
        audio_samples: audio_context.samples.len(),
    };

    // Imprimir JSON a stdout
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}
