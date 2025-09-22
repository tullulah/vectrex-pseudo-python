use vectrex_emulator::cpu6809::{CPU, VALID_PREFIX10, VALID_PREFIX11}; // bring in valid extended opcode lists
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let bios_path = args.iter().skip(1).find(|a| a.ends_with(".bin"));
    let mut cpu = CPU::default();
    if let Some(path) = bios_path {
        if let Ok(data) = fs::read(path) { cpu.load_bios(&data); } else { eprintln!("Failed to read BIOS {}", path); }
    }
    // Recompute static opcode coverage (synthetic one-step execution for all opcodes)
    let (_impld, _unimpl, unimpl) = cpu.recompute_opcode_coverage();
    println!("Primary-byte unimplemented ({}):", unimpl.len());
    for op in &unimpl { println!("0x{:02X}", op); }
    // Extended valid coverage: iterate only defined extended sub-opcodes.
    let mut ext_missing: Vec<u16> = Vec::new();
    let mut tested = 0usize;
    for (prefix, list) in [(0x10u8, VALID_PREFIX10),(0x11u8, VALID_PREFIX11)] {
        for &sub in list {
            tested += 1;
            let mut clone = CPU::default();
            clone.pc=0x0100; clone.bus.mem[0x0100]=prefix; clone.bus.mem[0x0101]=sub; clone.bus.mem[0xFFFC]=0x00; clone.bus.mem[0xFFFD]=0x02;
            if !clone.step() { ext_missing.push(((prefix as u16)<<8)|sub as u16); }
        }
    }
    ext_missing.sort_unstable();
    println!("Extended valid opcode pairs tested: {}", tested);
    println!("Extended valid unimplemented pairs ({}):", ext_missing.len());
    for v in &ext_missing { println!("  {:04X}", v); }
    // Report count of invalid (unassigned) codes skipped just for transparency
    let skipped_invalid = (2*256) - tested; // two prefix pages * 256 slots each minus tested valid ones
    println!("Extended invalid/unassigned pairs skipped: {skipped_invalid}");

    // Optional: run a limited number of BIOS steps to catch runtime traps
    if cpu.bios_present { cpu.reset(); }
    let mut runtime_unimpl: Vec<u8> = Vec::new();
    let mut steps = 0u64; let max_steps = 50_000u64; // conservative default
    while steps < max_steps {
        if !cpu.step() {
            // last executed opcode flagged as unimplemented; record it
            let pc = cpu.pc.wrapping_sub(1); // step() increments pc after fetch
            let op = cpu.bus.mem[pc as usize];
            if !runtime_unimpl.contains(&op) { runtime_unimpl.push(op); }
            break;
        }
        steps += 1;
    }
    if !runtime_unimpl.is_empty() { println!("Runtime unimplemented encountered: {runtime_unimpl:?}"); }
    else { println!("No runtime unimplemented opcodes in first {steps} steps."); }
}
