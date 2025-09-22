use vectrex_emulator::{CPU, memory_map};

#[test]
fn test_debug_classification() {
    println!("🔍 Debug clasificación memory_map");
    
    // Test classification
    let addr = 0xF001;
    let region = memory_map::classify(addr);
    println!("classify(0x{:04X}) = {:?}", addr, region);
    
    // Test BIOS constants
    println!("BIOS_START = 0x{:04X}", memory_map::BIOS_START);
    println!("BIOS_END = 0x{:04X}", memory_map::BIOS_END);
    
    // Test load path
    let mut cpu = CPU::default();
    
    // Load BIOS
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    let bios_data = std::fs::read(bios_path).unwrap();
    println!("BIOS size: {} bytes", bios_data.len());
    
    let base = memory_map::bios_load_base(bios_data.len());
    println!("bios_load_base({}) = 0x{:04X}", bios_data.len(), base);
    
    cpu.bus.load_bios_image(&bios_data);
    cpu.bios_present = true;
    
    // Test bus bios_base
    let bus_bios_base = cpu.bus.test_bios_base();
    println!("bus.bios_base = 0x{:04X}", bus_bios_base);
    
    // Test classification specifically for our problematic addresses
    for addr in [0xF000, 0xF001, 0xF002, 0xF003] {
        let region = memory_map::classify(addr);
        println!("classify(0x{:04X}) = {:?}", addr, region);
    }
    
    // Test actual memory contents at these locations
    println!("\nMemoria directa:");
    for addr in [0xF000, 0xF001, 0xF002, 0xF003] {
        let val = cpu.bus.mem[addr as usize];
        println!("mem[0x{:04X}] = 0x{:02X}", addr, val);
    }
    
    // Test bus.read8() manually
    println!("\nBus read8:");
    for addr in [0xF000, 0xF001, 0xF002, 0xF003] {
        let val = cpu.bus.read8(addr);
        println!("bus.read8(0x{:04X}) = 0x{:02X}", addr, val);
    }
    
    panic!("Debug stop");
}