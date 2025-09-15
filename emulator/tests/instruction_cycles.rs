use vectrex_emulator::CPU;

fn run_one(cpu: &mut CPU, op: u8) -> u64 {
    cpu.pc = 0x0100;
    cpu.mem[0x0100] = op; cpu.bus.mem[0x0100] = op;
    let before = cpu.cycles; cpu.step(); cpu.cycles - before
}

#[test]
fn immediate_loads_cycle_counts() {
    let mut cpu = CPU::default();
    let c = run_one(&mut cpu, 0x86); assert_eq!(c, 2, "LDA immediate expected 2 cycles (got {c})");
    let c = run_one(&mut cpu, 0xC6); assert_eq!(c, 2, "LDB immediate expected 2 cycles (got {c})");
}

#[test]
fn jsr_extended_cycle_count() {
    let mut cpu = CPU::default();
    cpu.pc = 0x0100; cpu.mem[0x0100]=0xBD; cpu.bus.mem[0x0100]=0xBD; // JSR extended
    cpu.mem[0x0101]=0x02; cpu.bus.mem[0x0101]=0x02; cpu.mem[0x0102]=0x00; cpu.bus.mem[0x0102]=0x00;
    let before=cpu.cycles; cpu.step(); let delta=cpu.cycles-before; assert_eq!(delta,7,"JSR extended should seed 7 cycles");
}
