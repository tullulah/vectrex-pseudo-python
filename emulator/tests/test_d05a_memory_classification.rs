#[cfg(test)]
mod tests {
    use vectrex_emulator::memory_map;

    #[test]
    fn test_d05a_memory_classification() {
        println!("=== ANÁLISIS MEMORIA 0xD05A ===");
        
        let addr = 0xD05A;
        let region = memory_map::classify(addr);
        
        println!("🎯 Dirección 0xD05A:");
        println!("   Región clasificada: {:?}", region);
        
        // Verificar límites de cada región
        println!("\n📊 Límites de regiones:");
        println!("   CART: 0x{:04X}-0x{:04X}", memory_map::CART_START, memory_map::CART_END);
        println!("   GAP:  0x{:04X}-0x{:04X}", memory_map::GAP_START, memory_map::GAP_END);
        println!("   RAM:  0x{:04X}-0x{:04X}", memory_map::RAM_START, memory_map::RAM_END);
        println!("   VIA:  0x{:04X}-0x{:04X}", memory_map::VIA_START, memory_map::VIA_END);
        println!("   ILL:  0x{:04X}-0x{:04X}", memory_map::ILLEGAL_START, memory_map::ILLEGAL_END);
        println!("   BIOS: 0x{:04X}-0x{:04X}", memory_map::BIOS_START, memory_map::BIOS_END);
        
        // Verificar dónde cae 0xD05A específicamente
        if addr >= memory_map::VIA_START && addr <= memory_map::VIA_END {
            let via_reg = memory_map::via_reg(addr);
            println!("\n🔍 0xD05A en región VIA:");
            println!("   Registro VIA calculado: 0x{:02X}", via_reg);
            println!("   Registro esperado: 0x0A (si fuera espejo de 0xD00A)");
            
            // Verificar el cálculo de via_reg
            let offset_from_via_start = addr - memory_map::VIA_START;
            let reg_calculated = offset_from_via_start % 0x10;
            println!("   Offset desde VIA_START: 0x{:04X}", offset_from_via_start);
            println!("   Reg = offset % 0x10 = 0x{:02X}", reg_calculated);
        }
        
        // Comprobar direcciones cercanas
        println!("\n🔍 Direcciones cercanas:");
        for test_addr in 0xD050..=0xD060 {
            let test_region = memory_map::classify(test_addr);
            if test_addr >= memory_map::VIA_START && test_addr <= memory_map::VIA_END {
                let test_reg = memory_map::via_reg(test_addr);
                println!("   0x{:04X}: {:?} (VIA reg 0x{:02X})", test_addr, test_region, test_reg);
            } else {
                println!("   0x{:04X}: {:?}", test_addr, test_region);
            }
        }
        
        println!("\n💡 ANÁLISIS:");
        match region {
            memory_map::Region::Via => {
                println!("   ✅ 0xD05A está en región VIA");
                println!("   📝 Esto significa que debería mapear a un registro VIA");
                println!("   🔧 El problema puede estar en el registro específico mapeado");
            }
            memory_map::Region::Gap | memory_map::Region::Illegal | memory_map::Region::Unmapped => {
                println!("   ❌ 0xD05A está en región no mapeada: {:?}", region);
                println!("   🔧 Por eso devuelve 0xFF - necesitamos corregir el mapeo");
            }
            _ => {
                println!("   ⚠️  0xD05A está en región inesperada: {:?}", region);
            }
        }
    }
}