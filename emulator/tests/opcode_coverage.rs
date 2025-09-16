// Basic opcode coverage test ensuring that all valid extended (prefix 0x10/0x11) opcodes
// defined in VALID_PREFIX10/11 execute without triggering the unimplemented path.
// Also prints (to test output) any remaining primary-byte unimplemented opcodes.

use vectrex_emulator::cpu6809::{CPU, VALID_PREFIX10, VALID_PREFIX11};

#[test]
fn extended_opcodes_all_implemented() {
    let mut missing: Vec<(u8,u8)> = Vec::new();
    for (prefix, list) in [(0x10u8, VALID_PREFIX10), (0x11u8, VALID_PREFIX11)] {
        for &sub in list.iter() {
            let mut cpu = CPU::default();
            // Lay down a tiny program: prefixed opcode then a RESET vector so PC starts at 0x0100.
            cpu.pc = 0x0100;
            cpu.mem[0x0100] = prefix;
            cpu.mem[0x0101] = sub;
            cpu.mem[0xFFFC] = 0x00; cpu.mem[0xFFFD] = 0x02; // reset vector -> 0x0200 (unused here)
            if !cpu.step() { missing.push((prefix, sub)); }
        }
    }
    if !missing.is_empty() {
        panic!("Missing extended opcodes: {:?}", missing);
    }
}

#[test]
fn report_primary_unimplemented() {
    let mut cpu = CPU::default();
    let (_done, _missing_count, missing) = cpu.recompute_opcode_coverage();
    // This test doesn't fail; it surfaces information in CI logs for visibility.
    // If you want to enforce zero, change the assert below.
    eprintln!("Primary-byte unimplemented count: {} -> {:?}", missing.len(), missing);
    // assert!(missing.is_empty(), "Primary opcodes remain unimplemented: {:?}", missing);
}
