#[cfg(test)]
mod vector_debug_test {
    use vectrex_emulator::CPU;

    #[test]
    fn debug_vector_reading() {
        let mut cpu = CPU::default();
        
        // Load BIOS
        let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
        let bios_data = std::fs::read(bios_path).expect("Failed to load BIOS");
        cpu.load_bios(&bios_data);
        
        // Reset CPU to trigger vector reading
        cpu.reset();
        
        // Read vectors manually using bus
        let reset_hi = cpu.bus.read8(0xFFFE);
        let reset_lo = cpu.bus.read8(0xFFFF);
        let reset_vec = ((reset_hi as u16) << 8) | (reset_lo as u16);
        
        let irq_hi = cpu.bus.read8(0xFFF8);
        let irq_lo = cpu.bus.read8(0xFFF9);
        let irq_vec = ((irq_hi as u16) << 8) | (irq_lo as u16);
        
        let firq_hi = cpu.bus.read8(0xFFF6);
        let firq_lo = cpu.bus.read8(0xFFF7);
        let firq_vec = ((firq_hi as u16) << 8) | (firq_lo as u16);
        
        let nmi_hi = cpu.bus.read8(0xFFFC);
        let nmi_lo = cpu.bus.read8(0xFFFD);
        let nmi_vec = ((nmi_hi as u16) << 8) | (nmi_lo as u16);
        
        println!("RESET vector (FFFE/FFFF): ${:04X}", reset_vec);
        println!("IRQ vector (FFF8/FFF9):   ${:04X}", irq_vec);
        println!("FIRQ vector (FFF6/FFF7):  ${:04X}", firq_vec);
        println!("NMI vector (FFFC/FFFD):   ${:04X}", nmi_vec);
        
        println!("Raw bytes - RESET: {:02X} {:02X}", reset_hi, reset_lo);
        println!("Raw bytes - IRQ:   {:02X} {:02X}", irq_hi, irq_lo);
        
        // Check PC after reset
        println!("PC after reset: ${:04X}", cpu.pc);
        
        // These vectors should point to BIOS ROM, not RAM
        assert!(reset_vec >= 0xE000, "RESET vector should point to BIOS (>= E000), got {:04X}", reset_vec);
        
        // If vectors are correct, this should be the real issue with the BIOS
        if reset_vec >= 0xE000 {
            println!("✓ Vectors are correctly pointing to BIOS ROM region");
            
            // Check if there's code at the reset vector
            let code_at_reset = cpu.bus.read8(reset_vec);
            println!("Code at RESET vector ${:04X}: ${:02X}", reset_vec, code_at_reset);
        }
    }
}