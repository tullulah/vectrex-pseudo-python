use vectrex_emulator::cpu6809::{CPU, VALID_PREFIX10, VALID_PREFIX11};
use std::fs;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)] struct Nominal { primary: HashMap<String,u32>, ext10: HashMap<String,u32>, ext11: HashMap<String,u32> }

fn main(){
    // Load nominal json
    let data = fs::read_to_string("docs/6809_cycles_nominal.json").expect("missing nominal json");
    let nom: Nominal = serde_json::from_str(&data).expect("bad json");
    println!("type,opcode,sub,emu_cycles,nom_cycles,delta");
    // Primary
    for op in 0u16..=255 { let mut cpu=CPU::default(); cpu.pc=0x0100; cpu.mem[0x0100]=op as u8; cpu.mem[0x0101]=0; cpu.mem[0xFFFC]=0; cpu.mem[0xFFFD]=2; let c0=cpu.cycles; let _=cpu.step(); let cyc=cpu.cycles - c0; let key=format!("{:02X}", op); let nomc=nom.primary.get(&key).cloned().unwrap_or(0); let delta=cyc as i32 - nomc as i32; println!("PRIMARY,{:02X},,{cyc},{nomc},{delta}", op); }
    // Extended 0x10
    for &sub in VALID_PREFIX10 { let mut cpu=CPU::default(); cpu.pc=0x0100; cpu.mem[0x0100]=0x10; cpu.mem[0x0101]=sub; cpu.mem[0xFFFC]=0; cpu.mem[0xFFFD]=2; let c0=cpu.cycles; let _=cpu.step(); let cyc=cpu.cycles - c0; let key=format!("{:02X}", sub); let nomc=nom.ext10.get(&key).cloned().unwrap_or(0); let delta=cyc as i32 - nomc as i32; println!("EXT10,10,{:02X},{cyc},{nomc},{delta}", sub); }
    // Extended 0x11
    for &sub in VALID_PREFIX11 { let mut cpu=CPU::default(); cpu.pc=0x0100; cpu.mem[0x0100]=0x11; cpu.mem[0x0101]=sub; cpu.mem[0xFFFC]=0; cpu.mem[0xFFFD]=2; let c0=cpu.cycles; let _=cpu.step(); let cyc=cpu.cycles - c0; let key=format!("{:02X}", sub); let nomc=nom.ext11.get(&key).cloned().unwrap_or(0); let delta=cyc as i32 - nomc as i32; println!("EXT11,11,{:02X},{cyc},{nomc},{delta}", sub); }
}
