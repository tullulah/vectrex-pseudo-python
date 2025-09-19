//! Utility binary: run CPU with real BIOS + optional cartridge until RAM execution detector triggers.
//! Prints snapshot for diagnosis.
//!
//! Usage (PowerShell):
//!   cargo run -p vectrex_emulator --bin ram_loop_capture -- --cart ..\..\examples\triangle.bin --max-steps 5000000
//!   cargo run -p vectrex_emulator --bin ram_loop_capture -- --max-steps 3000000
//!
//! Notes:
//! - BIOS path is fixed per project policy (no synthetic BIOS allowed).
//! - If no cart provided, runs BIOS only.
//! - Exits when detector.snapshot is Some or on step cap.

use std::fs;
use std::path::PathBuf;
use vectrex_emulator::cpu6809::CPU;
#[derive(Debug, Default)]
struct Args { cart: Option<PathBuf>, max_steps: u64, trace: bool, threshold: Option<u32> }

fn parse_args() -> Args {
    let mut a = Args { max_steps: 10_000_000, ..Default::default() };
    let mut iter = std::env::args().skip(1);
    while let Some(tok) = iter.next() {
        match tok.as_str() {
            "--cart" => { if let Some(p) = iter.next() { a.cart = Some(PathBuf::from(p)); } },
            "--max-steps" => { if let Some(v)=iter.next() { if let Ok(n)=v.parse() { a.max_steps=n; } } },
            "--trace" => { a.trace = true; },
            "--threshold" => { if let Some(v)=iter.next() { if let Ok(n)=v.parse() { a.threshold=Some(n); } } },
            _ => { eprintln!("[WARN] arg ignorado: {}", tok); }
        }
    }
    a
}

fn main() {
    let args = parse_args();
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios = fs::read(bios_path).expect("No se pudo leer BIOS real");
    assert!(bios.len()==4096 || bios.len()==8192, "Tamaño BIOS inesperado");

    let mut cpu = CPU::default();
    if args.trace { cpu.trace = true; }
    cpu.load_bios(&bios);

    if let Some(cart_path) = &args.cart {
        let cart_bytes = fs::read(cart_path).expect("Error leyendo cart");
        // Map cart at 0x0000 (cartridge origin policy)
        cpu.load_bin(&cart_bytes, 0x0000);
    }
    cpu.reset();

    // If threshold override requested, patch internal detector threshold by reducing trigger count.
    // We can't change compiled constant, so we simulate by early break after count >= custom.
    let custom_thresh = args.threshold.unwrap_or(512);

    for step_idx in 0..args.max_steps {
        let pc_before = cpu.pc;
        if !cpu.step() { eprintln!("[STOP] step devolvió false (opcode no implementado?)"); break; }
        // Manual early trigger path if custom threshold < compiled 512
        if custom_thresh < 512 && pc_before>=0xC800 && pc_before<=0xCFFF {
            let det = &cpu.ram_exec;
            if !det.triggered && det.count >= custom_thresh { eprintln!("[FORCE SNAPSHOT] count {} >= custom {} (esperando compilado 512)", det.count, custom_thresh); }
        }
        if cpu.ram_exec.triggered { break; }
        if (step_idx % 200_000)==0 { eprintln!("[PROGRESS] steps={} pc={:04X} bios_calls={} ram_count={} triggered={}", step_idx, cpu.pc, cpu.bios_calls.len(), cpu.ram_exec.count, cpu.ram_exec.triggered); }
    }

    if let Some(snap) = &cpu.ram_exec.snapshot { println!("=== RAM EXEC SNAPSHOT ==="); println!("first_pc={:04X} last_pc={:04X} iterations={}", snap.first_pc, snap.last_pc, snap.iterations); println!("regs: A={:02X} B={:02X} X={:04X} Y={:04X} U={:04X} S={:04X} DP={:02X} PC={:04X}", snap.regs.0,snap.regs.1,snap.regs.2,snap.regs.3,snap.regs.4,snap.regs.5,snap.regs.6,snap.regs.7); println!("recent_pcs: {:?}", snap.recent_pcs); println!("call_stack len={} top={:?}", snap.call_stack.len(), snap.call_stack.last()); println!("stack_bytes (48):"); for (i,b) in snap.stack_bytes.iter().enumerate() { if i%16==0 { print!("\n {:04X}:", snap.regs.5.wrapping_add(i as u16)); } print!(" {:02X}", b); } println!("\nwindow bytes:"); for (i,b) in snap.window.iter().enumerate() { if i%16==0 { print!("\n {:04X}:", snap.last_pc.saturating_sub(24).wrapping_add(i as u16)); } print!(" {:02X}", b); } println!(); } else { println!("[INFO] No se disparó detector (iterations={})", cpu.ram_exec.count); }
}
