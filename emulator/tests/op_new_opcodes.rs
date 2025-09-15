use vectrex_emulator::CPU;

fn step_single(mut cpu: CPU) -> CPU { cpu.step(); cpu }

#[test]
fn ror_direct() {
    let mut cpu = CPU::default();
    cpu.dp = 0x20; // direct page 0x2000
    cpu.pc = 0x0100;
    cpu.mem[0x0100] = 0x06; // ROR direct
    cpu.mem[0x0101] = 0x10; // offset -> 0x2010
    cpu.mem[0x2010] = 0b0000_0011; // C will become 1, result 0000_0001
    cpu.cc_c = false; // carry in -> MSB
    cpu = step_single(cpu);
    assert_eq!(cpu.mem[0x2010], 0b0000_0001);
    assert_eq!(cpu.cc_c, true); // original bit0
    assert_eq!(cpu.cc_n, false);
    assert_eq!(cpu.cc_z, false);
}

#[test]
fn rol_direct() {
    let mut cpu = CPU::default();
    cpu.dp = 0x21;
    cpu.pc = 0x0200;
    cpu.mem[0x0200] = 0x09; // ROL direct
    cpu.mem[0x0201] = 0x05; // -> 0x2105
    cpu.mem[0x2105] = 0b1000_0000; // will shift out to carry
    cpu.cc_c = true; // carry in sets bit0
    cpu.step();
    assert_eq!(cpu.mem[0x2105], 0b0000_0001);
    assert_eq!(cpu.cc_c, true); // old msb
    assert_eq!(cpu.cc_n, false);
}

#[test]
fn inc_direct_overflow() {
    let mut cpu = CPU::default();
    cpu.dp = 0x30;
    cpu.pc = 0x0300;
    cpu.mem[0x0300] = 0x0C; // INC direct
    cpu.mem[0x0301] = 0x40; // -> 0x3040
    cpu.mem[0x3040] = 0x7F; // 0x7F -> 0x80 sets V and N
    cpu.step();
    assert_eq!(cpu.mem[0x3040], 0x80);
    assert!(cpu.cc_v && cpu.cc_n);
}

#[test]
fn clr_direct_flags() {
    let mut cpu = CPU::default();
    cpu.dp = 0x22;
    cpu.pc = 0x0400;
    cpu.mem[0x0400] = 0x0F; // CLR direct
    cpu.mem[0x0401] = 0x02; // -> 0x2202
    cpu.mem[0x2202] = 0xAA;
    cpu.cc_n = true; cpu.cc_v = true; cpu.cc_c = true; cpu.cc_z = false;
    cpu.step();
    assert_eq!(cpu.mem[0x2202], 0x00);
    assert!(cpu.cc_z && !cpu.cc_n && !cpu.cc_v && !cpu.cc_c);
}

#[test]
fn ora_indexed() {
    let mut cpu = CPU::default();
    cpu.pc = 0x0500;
    cpu.x = 0x4000;
    cpu.a = 0x55;
    // LDA indexed form: our indexed ORA uses opcode 0xAA then postbyte.
    // Use simple ,X (postbyte 0x84 pattern via decode supports direct base) -> choose 0x84 meaning base + A? We want plain base, so use 0x84 with A=0 yields base.
    // Simpler: use postbyte 0x80 group with 0x04 low bits mapping to base (see decode_indexed_basic). We'll craft 0x84.
    cpu.mem[0x0500] = 0xAA; // ORA indexed
    cpu.mem[0x0501] = 0x84; // base X
    cpu.mem[0x4000] = 0x0F;
    cpu.step();
    assert_eq!(cpu.a, 0x5F);
    assert!(!cpu.cc_v && cpu.cc_n == (cpu.a & 0x80 != 0));
}

#[test]
fn addd_extended() {
    let mut cpu = CPU::default();
    cpu.pc = 0x0600;
    cpu.a = 0x12; cpu.b = 0x34; // D = 0x1234
    cpu.mem[0x0600] = 0xF3; // ADDD extended
    cpu.mem[0x0601] = 0x90; // address 0x9000
    cpu.mem[0x0602] = 0x00;
    cpu.mem[0x9000] = 0x00; cpu.mem[0x9001] = 0x02; // +0x0002 -> 0x1236
    cpu.step();
    assert_eq!(cpu.a, 0x12);
    assert_eq!(cpu.b, 0x36);
    assert!(!cpu.cc_v);
}

#[test]
fn eorb_extended() {
    let mut cpu = CPU::default();
    cpu.pc = 0x0700;
    cpu.b = 0xF0;
    cpu.mem[0x0700] = 0xF8; // EORB extended
    cpu.mem[0x0701] = 0x88; // 0x8800
    cpu.mem[0x0702] = 0x00;
    cpu.mem[0x8800] = 0x0F; // result 0xFF sets N
    cpu.step();
    assert_eq!(cpu.b, 0xFF);
    assert!(cpu.cc_n && !cpu.cc_z);
}
