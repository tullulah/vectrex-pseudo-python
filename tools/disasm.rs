// M6809 Disassembler for Vectrex Multibank ROMs
// Converts .bin back to readable ASM with real addresses

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const BANK_SIZE: usize = 0x4000; // 16KB per bank

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: disasm <input.bin> [output.asm]");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = if args.len() >= 3 {
        args[2].clone()
    } else {
        format!("{}.disasm.asm", input_path)
    };

    let mut file = File::open(input_path).expect("Failed to open input");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read input");

    let mut output = File::create(&output_path).expect("Failed to create output");
    
    println!("Disassembling {} ({} bytes)...", input_path, data.len());
    
    // Check if multibank (>32KB)
    if data.len() > 32768 {
        let num_banks = data.len() / BANK_SIZE;
        println!("Multibank ROM: {} banks ({} KB)", num_banks, data.len() / 1024);
        
        for bank_id in 0..num_banks {
            let bank_start = bank_id * BANK_SIZE;
            let bank_end = bank_start + BANK_SIZE;
            let bank_data = &data[bank_start..bank_end];
            
            writeln!(&mut output, "; ========================================").unwrap();
            writeln!(&mut output, "; BANK #{} (offset ${:06X}-${:06X})", bank_id, bank_start, bank_end - 1).unwrap();
            writeln!(&mut output, "; ========================================").unwrap();
            
            if is_bank_empty(bank_data) {
                writeln!(&mut output, "; [EMPTY BANK - all 0xFF]\n").unwrap();
                continue;
            }
            
            // Bank #0-30: Maps to $0000-$3FFF (switchable window)
            // Bank #31: Maps to $4000-$7FFF (fixed)
            let base_addr = if bank_id == 31 { 0x4000 } else { 0x0000 };
            
            disassemble_bank(&mut output, bank_data, base_addr, bank_id);
            writeln!(&mut output).unwrap();
        }
        
        // Show RESET vector location
        if num_banks == 32 {
            let reset_offset = (31 * BANK_SIZE) + 0x3FFE; // Bank #31 at $7FFE
            if reset_offset + 1 < data.len() {
                let reset_vec = u16::from_be_bytes([data[reset_offset], data[reset_offset + 1]]);
                writeln!(&mut output, "; ========================================").unwrap();
                writeln!(&mut output, "; RESET VECTOR (Bank #31 offset $3FFE)").unwrap();
                writeln!(&mut output, "; Points to: ${:04X}", reset_vec).unwrap();
                writeln!(&mut output, "; ========================================").unwrap();
            }
        }
    } else {
        println!("Monobank ROM: 32 KB");
        disassemble_bank(&mut output, &data, 0x0000, 0);
    }
    
    println!("✓ Disassembly written to {}", output_path);
}

fn is_bank_empty(data: &[u8]) -> bool {
    data.iter().all(|&b| b == 0xFF)
}

fn disassemble_bank(output: &mut File, data: &[u8], base_addr: u16, bank_id: usize) {
    let mut pc = 0;
    
    // Show header if Bank #0
    if bank_id == 0 && data.len() >= 0x20 {
        writeln!(output, "; --- ROM HEADER ---").unwrap();
        writeln!(output, "${:04X}:  ; Signature: {:?}", base_addr, String::from_utf8_lossy(&data[0..10])).unwrap();
        writeln!(output, "${:04X}:  ; Music pointer: ${:02X}{:02X}", base_addr + 10, data[10], data[11]).unwrap();
        writeln!(output, "${:04X}:  ; Title: {:?}", base_addr + 16, String::from_utf8_lossy(&data[16..31])).unwrap();
        writeln!(output, "").unwrap();
        pc = 0x20; // Skip header
    }
    
    while pc < data.len() {
        let addr = base_addr.wrapping_add(pc as u16);
        let opcode = data[pc];
        
        let (instr, size) = decode_opcode(&data[pc..]);
        
        // Show bytes
        let mut bytes_str = String::new();
        for i in 0..size.min(data.len() - pc) {
            bytes_str.push_str(&format!("{:02X} ", data[pc + i]));
        }
        
        writeln!(output, "${:04X}:  {:12} {}", addr, bytes_str, instr).unwrap();
        
        pc += size;
        
        // Stop if we hit padding
        if opcode == 0xFF && pc < data.len() && data[pc..].iter().all(|&b| b == 0xFF) {
            writeln!(output, "; [Rest of bank is 0xFF padding]").unwrap();
            break;
        }
    }
}

fn decode_opcode(data: &[u8]) -> (String, usize) {
    if data.is_empty() {
        return ("???".to_string(), 1);
    }
    
    let op = data[0];
    
    match op {
        0x12 => ("NOP".to_string(), 1),
        0x16 => {
            if data.len() >= 3 {
                (format!("LBRA ${:04X}", i16::from_be_bytes([data[1], data[2]])), 3)
            } else {
                ("LBRA ???".to_string(), 1)
            }
        },
        0x17 => {
            if data.len() >= 3 {
                (format!("LBSR ${:04X}", i16::from_be_bytes([data[1], data[2]])), 3)
            } else {
                ("LBSR ???".to_string(), 1)
            }
        },
        0x1F => {
            if data.len() >= 2 {
                let regs = decode_tfr_regs(data[1]);
                (format!("TFR {}", regs), 2)
            } else {
                ("TFR ???".to_string(), 1)
            }
        },
        0x20 => {
            if data.len() >= 2 {
                (format!("BRA ${:02X}", data[1] as i8), 2)
            } else {
                ("BRA ???".to_string(), 1)
            }
        },
        0x30 => {
            if data.len() >= 2 {
                (format!("LEAX {},X", data[1] as i8), 2)
            } else {
                ("LEAX ???".to_string(), 1)
            }
        },
        0x32 => {
            if data.len() >= 2 {
                (format!("LEAS {},S", data[1]), 2)
            } else {
                ("LEAS ???".to_string(), 1)
            }
        },
        0x34 => {
            if data.len() >= 2 {
                let regs = decode_push_pull_regs(data[1]);
                (format!("PSHS {}", regs), 2)
            } else {
                ("PSHS ???".to_string(), 1)
            }
        },
        0x35 => {
            if data.len() >= 2 {
                let regs = decode_push_pull_regs(data[1]);
                (format!("PULS {}", regs), 2)
            } else {
                ("PULS ???".to_string(), 1)
            }
        },
        0x39 => ("RTS".to_string(), 1),
        0x4F => ("CLRA".to_string(), 1),
        0x5F => ("CLRB".to_string(), 1),
        0x7F => {
            if data.len() >= 3 {
                (format!("CLR ${:04X}", u16::from_be_bytes([data[1], data[2]])), 3)
            } else {
                ("CLR ???".to_string(), 1)
            }
        },
        0x86 => {
            if data.len() >= 2 {
                (format!("LDA #${:02X}", data[1]), 2)
            } else {
                ("LDA ???".to_string(), 1)
            }
        },
        0x8E => {
            if data.len() >= 3 {
                (format!("LDX #${:04X}", u16::from_be_bytes([data[1], data[2]])), 3)
            } else {
                ("LDX ???".to_string(), 1)
            }
        },
        0xB6 => {
            if data.len() >= 3 {
                (format!("LDA ${:04X}", u16::from_be_bytes([data[1], data[2]])), 3)
            } else {
                ("LDA ???".to_string(), 1)
            }
        },
        0xB7 => {
            if data.len() >= 3 {
                (format!("STA ${:04X}", u16::from_be_bytes([data[1], data[2]])), 3)
            } else {
                ("STA ???".to_string(), 1)
            }
        },
        0xBD => {
            if data.len() >= 3 {
                (format!("JSR ${:04X}", u16::from_be_bytes([data[1], data[2]])), 3)
            } else {
                ("JSR ???".to_string(), 1)
            }
        },
        0xCC => {
            if data.len() >= 3 {
                (format!("LDD #${:04X}", u16::from_be_bytes([data[1], data[2]])), 3)
            } else {
                ("LDD ???".to_string(), 1)
            }
        },
        0xFC => {
            if data.len() >= 3 {
                (format!("LDD ${:04X}", u16::from_be_bytes([data[1], data[2]])), 3)
            } else {
                ("LDD ???".to_string(), 1)
            }
        },
        0xFD => {
            if data.len() >= 3 {
                (format!("STD ${:04X}", u16::from_be_bytes([data[1], data[2]])), 3)
            } else {
                ("STD ???".to_string(), 1)
            }
        },
        0x10 => {
            // Page 2 opcodes
            if data.len() >= 2 {
                match data[1] {
                    0x26 => {
                        if data.len() >= 4 {
                            (format!("LBNE ${:04X}", i16::from_be_bytes([data[2], data[3]])), 4)
                        } else {
                            ("LBNE ???".to_string(), 2)
                        }
                    },
                    0x27 => {
                        if data.len() >= 4 {
                            (format!("LBEQ ${:04X}", i16::from_be_bytes([data[2], data[3]])), 4)
                        } else {
                            ("LBEQ ???".to_string(), 2)
                        }
                    },
                    0x2E => {
                        if data.len() >= 4 {
                            (format!("LBGT ${:04X}", i16::from_be_bytes([data[2], data[3]])), 4)
                        } else {
                            ("LBGT ???".to_string(), 2)
                        }
                    },
                    _ => (format!("PAGE2 ${:02X}", data[1]), 2),
                }
            } else {
                ("PAGE2 ???".to_string(), 1)
            }
        },
        _ => (format!("FCB ${:02X}", op), 1),
    }
}

fn decode_tfr_regs(postbyte: u8) -> String {
    let src = (postbyte >> 4) & 0x0F;
    let dst = postbyte & 0x0F;
    
    let src_name = match src {
        0x0 => "D",
        0x1 => "X",
        0x2 => "Y",
        0x3 => "U",
        0x4 => "S",
        0x5 => "PC",
        0x8 => "A",
        0x9 => "B",
        0xA => "CC",
        0xB => "DP",
        _ => "??",
    };
    
    let dst_name = match dst {
        0x0 => "D",
        0x1 => "X",
        0x2 => "Y",
        0x3 => "U",
        0x4 => "S",
        0x5 => "PC",
        0x8 => "A",
        0x9 => "B",
        0xA => "CC",
        0xB => "DP",
        _ => "??",
    };
    
    format!("{},{}", src_name, dst_name)
}

fn decode_push_pull_regs(postbyte: u8) -> String {
    let mut regs = Vec::new();
    
    if postbyte & 0x80 != 0 { regs.push("PC"); }
    if postbyte & 0x40 != 0 { regs.push("U/S"); }
    if postbyte & 0x20 != 0 { regs.push("Y"); }
    if postbyte & 0x10 != 0 { regs.push("X"); }
    if postbyte & 0x08 != 0 { regs.push("DP"); }
    if postbyte & 0x04 != 0 { regs.push("B"); }
    if postbyte & 0x02 != 0 { regs.push("A"); }
    if postbyte & 0x01 != 0 { regs.push("CC"); }
    
    if regs.is_empty() {
        "".to_string()
    } else {
        regs.join(",")
    }
}
