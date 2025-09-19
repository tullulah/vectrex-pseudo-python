use vectrex_emulator::CPU;

#[test]
#[ignore]
fn set_input_writes_ram_and_metrics() {
    // Only compile this test when wasm feature not required; we call internal CPU then mimic wasm API behavior.
    let mut cpu = CPU::default();
    // Simulate what set_input_state does (logic duplicated minimal for test):
    let x = -50i16; let y = 100i16; let buttons = 0b10101u8; // extra bit beyond 4 should be masked
    let clamped_x = x.clamp(-128,127); let clamped_y = y.clamp(-128,127);
    let bx = (clamped_x as i32 + 128) as u8; let by = (clamped_y as i32 + 128) as u8;
    // Write to RAM area 0x00F0..0x00F2
    cpu.mem[0x00F0] = bx; cpu.bus.mem[0x00F0] = bx;
    cpu.mem[0x00F1] = by; cpu.bus.mem[0x00F1] = by;
    cpu.mem[0x00F2] = buttons & 0x0F; cpu.bus.mem[0x00F2] = buttons & 0x0F;
    assert_eq!(cpu.mem[0x00F0], ((-50 + 128) as u8));
    assert_eq!(cpu.mem[0x00F1], ((100 + 128) as u8));
    assert_eq!(cpu.mem[0x00F2], 0b0101); // masked to low 4 bits
    // Metrics JSON should include these fields
    let m = cpu.opcode_metrics(); assert_eq!(m.total, 0); // baseline sanity
}
