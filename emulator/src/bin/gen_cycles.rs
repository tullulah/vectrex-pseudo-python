use vectrex_emulator::cpu6809::{CPU, VALID_PREFIX10, VALID_PREFIX11};

// Simple cycle profiler: executes each primary opcode once in isolation and reports the cycles consumed
// as observed via cpu.cycles delta. For prefixed opcodes 0x10/0x11, iterates valid sub-opcodes.
// Output: CSV to stdout: type,opcode,sub_opcode,cycles
// type = PRIMARY | EXT10 | EXT11
// sub_opcode empty for PRIMARY rows.
fn main(){
    println!("type,opcode,sub,cycles");
    // Primary opcodes 0x00-0xFF
    for op in 0u16..=255 { let mut cpu = CPU::default(); cpu.pc=0x0100; cpu.bus.mem[0x0100]=op as u8; cpu.bus.mem[0x0101]=0; cpu.bus.mem[0x0102]=0; cpu.bus.mem[0xFFFC]=0x00; cpu.bus.mem[0xFFFD]=0x02; let c0=cpu.cycles; let _=cpu.step(); let cyc=cpu.cycles - c0; println!("PRIMARY,{:02X},,{cyc}", op); }
    for &sub in VALID_PREFIX10 { let mut cpu=CPU::default(); cpu.pc=0x0100; cpu.bus.mem[0x0100]=0x10; cpu.bus.mem[0x0101]=sub; cpu.bus.mem[0x0102]=0; cpu.bus.mem[0xFFFC]=0x00; cpu.bus.mem[0xFFFD]=0x02; let c0=cpu.cycles; let _=cpu.step(); let cyc=cpu.cycles - c0; println!("EXT10,10,{:02X},{cyc}", sub); }
    for &sub in VALID_PREFIX11 { let mut cpu=CPU::default(); cpu.pc=0x0100; cpu.bus.mem[0x0100]=0x11; cpu.bus.mem[0x0101]=sub; cpu.bus.mem[0x0102]=0; cpu.bus.mem[0xFFFC]=0x00; cpu.bus.mem[0xFFFD]=0x02; let c0=cpu.cycles; let _=cpu.step(); let cyc=cpu.cycles - c0; println!("EXT11,11,{:02X},{cyc}", sub); }
}
