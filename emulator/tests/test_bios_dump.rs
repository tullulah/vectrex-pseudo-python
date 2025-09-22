//! Test para examinar los bytes exactos en la BIOS

use std::fs;

#[test]
fn test_bios_memory_dump() {
    // Cargar BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = fs::read(bios_path).expect("Failed to load BIOS");
    
    println!("🔍 BIOS Memory Dump");
    println!("BIOS file size: {} bytes", bios_data.len());
    
    // Los primeros bytes en 0xF000 (0x2000 en el archivo)
    let offset_f000 = 0x2000; // F000 - E000 = 2000
    
    println!("\nBytes en 0xF000-0xF00F:");
    for i in 0..16 {
        if offset_f000 + i < bios_data.len() {
            let addr = 0xF000 + i;
            let byte = bios_data[offset_f000 + i];
            println!("  0x{:04X}: 0x{:02X}", addr, byte);
        }
    }
    
    // Reset vector
    let reset_high = bios_data[0x1FFE]; // FFFE - E000 = 1FFE
    let reset_low = bios_data[0x1FFF];  // FFFF - E000 = 1FFF
    let reset_vector = ((reset_high as u16) << 8) | (reset_low as u16);
    
    println!("\nReset vector:");
    println!("  0xFFFE: 0x{:02X}", reset_high);
    println!("  0xFFFF: 0x{:02X}", reset_low);
    println!("  Vector: 0x{:04X}", reset_vector);
    
    // Primeros bytes en el reset vector
    if reset_vector >= 0xE000 {
        let vec_offset = (reset_vector - 0xE000) as usize;
        println!("\nBytes en reset vector 0x{:04X}:", reset_vector);
        for i in 0..10 {
            if vec_offset + i < bios_data.len() {
                let addr = reset_vector + i as u16;
                let byte = bios_data[vec_offset + i];
                println!("  0x{:04X}: 0x{:02X}", addr, byte);
            }
        }
    }
}