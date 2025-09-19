use vectrex_emulator::CPU;
use std::fs;

// NOTE: Real BIOS required (see project guidelines: no synthetic BIOS). Adjust path if helper introduced later.
#[test]
fn trace_captures_instructions_after_enable() {
    let bios_path = "C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/ide/frontend/dist/bios.bin"; // canonical path per guidelines
    let data = fs::read(bios_path).expect("BIOS file missing at canonical path");
    assert!(data.len()==4096 || data.len()==8192, "Unexpected BIOS size");

    let mut cpu = CPU::default();
    cpu.load_bios(&data);
    cpu.reset();
    // Enable trace with a small limit to keep test light
    cpu.trace_enabled = true; cpu.trace_limit = 512;
    // Execute some instructions until we collect entries or reach safety cap
    for _ in 0..2000 { if !cpu.step() { break; } if !cpu.trace_buf.is_empty() { break; } }
    assert!(cpu.trace_buf.len()>0, "Trace buffer remained empty after enabling trace and executing steps");
    // Sanity: first entry's PC should be inside BIOS window (>= E000)
    let first_pc = cpu.trace_buf[0].pc;
    assert!(first_pc >= 0xE000, "First traced PC {:04X} below expected BIOS window", first_pc);
}
