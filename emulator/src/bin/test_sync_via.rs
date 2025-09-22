use vectrex_emulator::Emulator;

fn main() {
    println!("🧪 Test: Actualizaciones síncronas VIA (vectrexy/jsvecx pattern)");
    
    // Test con BIOS para verificar que las actualizaciones síncronas previenen interferencia
    let mut emulator = Emulator::new();
    emulator.cpu.trace = false; // Minimizar ruido
    
    // Estado inicial
    println!("\n=== Estado inicial ===");
    println!("MUX enabled: {}, selector: {}", emulator.cpu.mux_enabled, emulator.cpu.mux_selector);
    println!("Position: ({}, {})", emulator.cpu.current_x, emulator.cpu.current_y);
    
    // Test 1: Escribir Port B para habilitar MUX con selector Y-axis
    println!("\n=== Test 1: Habilitar MUX Y-axis ===");
    emulator.cpu.test_write8(0xD000, 0x00); // Port B: MUX enabled, selector=0 (Y-axis)
    
    println!("MUX enabled: {}, selector: {}", emulator.cpu.mux_enabled, emulator.cpu.mux_selector);
    println!("Position: ({}, {})", emulator.cpu.current_x, emulator.cpu.current_y);
    
    // Test 2: Escribir Port A con valor positivo para Y
    println!("\n=== Test 2: Escribir Port A para Y-axis ===");
    emulator.cpu.test_write8(0xD001, 0x40); // Port A: +64 -> should update Y coordinate
    
    println!("Port A value: 0x{:02X} ({})", emulator.cpu.port_a_value, emulator.cpu.port_a_value as i8);
    println!("Position: ({}, {})", emulator.cpu.current_x, emulator.cpu.current_y);
    
    // Test 3: Cambiar MUX a X-axis y escribir otro valor
    println!("\n=== Test 3: Cambiar a MUX disabled (solo X-axis) ===");
    emulator.cpu.test_write8(0xD000, 0x01); // Port B: MUX disabled
    emulator.cpu.test_write8(0xD001, 0x80); // Port A: -128
    
    println!("MUX enabled: {}, selector: {}", emulator.cpu.mux_enabled, emulator.cpu.mux_selector);
    println!("Port A value: 0x{:02X} ({})", emulator.cpu.port_a_value, emulator.cpu.port_a_value as i8);
    println!("Position: ({}, {})", emulator.cpu.current_x, emulator.cpu.current_y);
    
    // Test 4: Verificar comportamiento con BIOS ejecutándose
    println!("\n=== Test 4: Con BIOS ejecutándose (prevenir interferencia) ===");
    emulator.cpu.test_write8(0xD000, 0x00); // Habilitar MUX Y-axis
    emulator.cpu.test_write8(0xD001, 0x20); // Port A: +32
    
    println!("Antes de step() - Position: ({}, {})", emulator.cpu.current_x, emulator.cpu.current_y);
    
    // Ejecutar algunos pasos para ver si BIOS interfiere
    for i in 0..10 {
        let stepped = emulator.step();
        if !stepped { break; }
        
        if i == 4 {
            println!("Después de {} steps - Position: ({}, {})", i+1, emulator.cpu.current_x, emulator.cpu.current_y);
        }
    }
    
    println!("Final - Position: ({}, {})", emulator.cpu.current_x, emulator.cpu.current_y);
    
    println!("\n✅ Test completado - Verificar que las actualizaciones son inmediatas y no se ven afectadas por BIOS");
}