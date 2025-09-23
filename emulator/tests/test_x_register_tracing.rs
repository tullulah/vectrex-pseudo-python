use vectrex_emulator::cpu6809::CPU;

#[test]
fn test_comprehensive_x_register_tracing() {
    println!("🔍 Testing comprehensive X register modification tracing");

    // Test setup with CPU default
    let mut cpu = CPU::default();

    // Preparar emulador con BIOS real
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin";
    let bios_data = std::fs::read(bios_path).expect("Failed to load BIOS");
    
    // Load BIOS into CPU's internal bus
    for (i, &byte) in bios_data.iter().enumerate() {
        if i < bios_data.len() {
            cpu.test_write8(0xE000 + i as u16, byte);
        }
    }
    
    cpu.reset();
    cpu.pc = 0xF000; // Start in BIOS

    println!("🔍 Starting X register tracing test from reset state");
    println!("Initial X: 0x{:04X}", cpu.x);

    // Execute several instructions to capture X register changes
    for step in 0..100 {
        let pc = cpu.pc;
        let opcode = cpu.test_read8(pc);
        
        if pc >= 0xF000 {  // BIOS region
            cpu.step();
            // X register changes will be traced by the trace_x_change function
        } else {
            break;
        }
    }

    println!("🔍 X register tracing test completed");
}

#[test]
fn test_specific_x_register_instructions() {
    println!("🔍 Testing specific X register modification instructions");

    let mut cpu = CPU::default();

    // Load specific test instructions into RAM (0xC800+ range)
    let test_addr = 0xC800;
    
    // Test 1: LDX immediate (0x8E)
    cpu.test_write8(test_addr, 0x8E);       // LDX immediate
    cpu.test_write8(test_addr + 1, 0x12);   // High byte
    cpu.test_write8(test_addr + 2, 0x34);   // Low byte
    
    // Test 2: ABX (0x3A)
    cpu.test_write8(test_addr + 3, 0x3A);   // ABX
    
    // Test 3: TFR D,X (0x1F)
    cpu.test_write8(test_addr + 4, 0x1F);   // TFR
    cpu.test_write8(test_addr + 5, 0x70);   // D→X (7 = D, 0 = X)

    cpu.pc = test_addr;
    cpu.x = 0x1000;  // Initial X value
    cpu.a = 0x05;    // For ABX test
    cpu.b = 0x06;    // For TFR D,X test (D = 0x0506)

    println!("Initial state: X=0x{:04X}, A=0x{:02X}, B=0x{:02X}", cpu.x, cpu.a, cpu.b);

    // Execute LDX immediate
    cpu.step();
    println!("After LDX #$1234: X=0x{:04X}", cpu.x);

    // Execute ABX  
    cpu.step();
    println!("After ABX: X=0x{:04X}", cpu.x);

    // Execute TFR D,X
    cpu.step();
    println!("After TFR D,X: X=0x{:04X}", cpu.x);

    println!("🔍 Specific X register instruction test completed");
}