use vectrex_emulator::CPU;

fn run(mut cpu: CPU) -> CPU { cpu.step(); cpu }

#[test]
fn tst_direct() {
    let mut cpu = CPU::default();
    cpu.dp = 0x24; cpu.pc = 0x0100;
    cpu.mem[0x0100] = 0x0D; // TST direct
    cpu.mem[0x0101] = 0x10; // -> 0x2410
    cpu.mem[0x2410] = 0x80;
    cpu = run(cpu);
    assert!(cpu.cc_n);
    assert!(!cpu.cc_z);
    assert!(!cpu.cc_v && !cpu.cc_c);
}

#[test]
fn jmp_direct() {
    let mut cpu = CPU::default();
    cpu.dp = 0x25; cpu.pc = 0x0200;
    cpu.mem[0x0200] = 0x0E; // JMP direct
    cpu.mem[0x0201] = 0x40; // -> 0x2540
    cpu.mem[0x2540] = 0x12; // next opcode placeholder
    cpu.step();
    assert_eq!(cpu.pc, 0x2540);
}

#[test]
fn jmp_indexed() {
    let mut cpu = CPU::default();
    cpu.pc = 0x0300; cpu.x = 0x6000;
    cpu.mem[0x0300] = 0x6E; // JMP indexed
    cpu.mem[0x0301] = 0x84; // base X
    cpu.mem[0x6000] = 0xFF; // target opcode placeholder
    cpu.step();
    assert_eq!(cpu.pc, 0x6000);
}

#[test]
fn asl_indexed_flags() {
    let mut cpu = CPU::default();
    cpu.pc = 0x0400; cpu.x = 0x7000;
    cpu.mem[0x0400] = 0x68; // ASL indexed
    cpu.mem[0x0401] = 0x84; // base X
    cpu.mem[0x7000] = 0xC0; // 1100_0000 -> shift left 1000_0000 carry=1 N=1
    cpu.step();
    assert_eq!(cpu.mem[0x7000], 0x80);
    assert!(cpu.cc_n);
    assert!(!cpu.cc_z);
    assert!(cpu.cc_c);
}

#[test]
fn dec_indexed_overflow_v() {
    let mut cpu = CPU::default();
    cpu.pc = 0x0500; cpu.x = 0x7100;
    cpu.mem[0x0500] = 0x6A; // DEC indexed
    cpu.mem[0x0501] = 0x84; // base X
    cpu.mem[0x7100] = 0x80; // -> 0x7F sets V
    cpu.step();
    assert_eq!(cpu.mem[0x7100], 0x7F);
    assert!(cpu.cc_v);
}

#[test]
fn ror_indexed_carry_rotate_in() {
    let mut cpu = CPU::default();
    cpu.pc = 0x0600; cpu.x = 0x7200; cpu.cc_c = true; // carry in will set MSB
    cpu.mem[0x0600] = 0x66; // ROR indexed
    cpu.mem[0x0601] = 0x84; // base X
    cpu.mem[0x7200] = 0x01; // bit0 -> carry, result becomes 0x80 with carry set from bit0
    cpu.step();
    assert_eq!(cpu.mem[0x7200], 0x80);
    assert!(cpu.cc_c); // old bit0
    assert!(cpu.cc_n);
}
