use vectrex_emulator::CPU;
use std::time::Instant;

// Ignored by default: run with `cargo test -p vectrex_emulator -- --ignored perf_smoke`.
#[test]
#[ignore]
fn perf_smoke() {
    let mut cpu = CPU::default();
    // Fill a small block with NOP (assume 0x12 is a NOP-like unimplemented treated minimal) or use a known cheap opcode.
    // If real NOP opcode exists, replace 0x12 accordingly.
    let nop = 0x12u8; // placeholder; adjust if different.
    for i in 0..256 { cpu.mem[0x0100 + i] = nop; cpu.bus.mem[0x0100 + i] = nop; }
    cpu.pc = 0x0100;
    let target_instr = 200_000; // run this many instructions
    let start = Instant::now();
    for _ in 0..target_instr { cpu.step(); }
    let elapsed = start.elapsed().as_secs_f64();
    let ips = target_instr as f64 / elapsed.max(1e-9);
    eprintln!("perf_smoke: {target_instr} instr in {elapsed:.6}s => {ips:.0} ips");
    // Just assert we executed something and didn't take an absurd amount of time; loose floor.
    assert!(ips > 10_000.0, "Instruction throughput unexpectedly low: {ips}");
}
