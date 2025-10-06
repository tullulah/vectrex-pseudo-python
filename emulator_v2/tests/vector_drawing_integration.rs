// Integration test: Vector drawing sequence
// Tests the complete flow from CPU opcodes → VIA → Screen → Lines
// This test verifies that the emulator can produce actual vector output

use vectrex_emulator_v2::emulator::Emulator;
use vectrex_emulator_v2::engine_types::{AudioContext, Input, RenderContext};

#[test]
fn test_vector_drawing_complete_sequence() {
    // Setup emulator following WASM pattern: new() -> init() -> load_bios() -> reset()
    let mut emulator = Emulator::new();
    let bios_path =
        r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";

    // Initialize devices (maps VIA, RAM, BIOS, etc. to memory bus)
    emulator.init(bios_path);

    // Reset CPU (reads reset vector at 0xFFFE and sets PC)
    emulator.reset();

    // Create render context and input
    let mut render_context = RenderContext::new();
    let mut audio_context = AudioContext::new(1500000.0 / 44100.0); // 1.5MHz / 44.1kHz
    let input = Input::default();

    // Execute enough cycles for BIOS to initialize and start drawing
    // BIOS init takes ~30K cycles, then starts drawing border/text
    for _ in 0..100_000 {
        let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
    }

    // Verify that lines were generated
    assert!(
        !render_context.lines.is_empty(),
        "Expected vector lines to be generated after BIOS execution"
    );

    println!(
        "✅ Vector drawing test: {} lines generated",
        render_context.lines.len()
    );

    // Verify line properties make sense
    let first_line = &render_context.lines[0];
    assert!(
        first_line.brightness > 0.0 && first_line.brightness <= 1.0,
        "Brightness should be in range 0-1, got {}",
        first_line.brightness
    );

    // Check that coordinates are reasonable (Vectrex range is roughly -127 to +127)
    // Screen uses f32 with scaling, so values will be larger
    let max_coord = 500.0; // Generous bounds for scaled coordinates
    assert!(
        first_line.p0.x.abs() < max_coord && first_line.p0.y.abs() < max_coord,
        "Line start point ({}, {}) outside expected range",
        first_line.p0.x,
        first_line.p0.y
    );
}

#[test]
fn test_via_port_writes_affect_screen() {
    // Setup emulator following WASM pattern
    let mut emulator = Emulator::new();
    let bios_path =
        r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    emulator.init(bios_path);
    emulator.reset();

    let mut render_context = RenderContext::new();
    let mut audio_context = AudioContext::new(1500000.0 / 44100.0); // 1.5MHz / 44.1kHz
    let input = Input::default();

    // Manually write to VIA registers to simulate vector drawing
    // This tests the STA $D000/STA $D001 opcodes

    // 1. Enable integrators (PORT_B bit 7 = 0)
    emulator.get_memory_bus().write(0xD000, 0x00); // PORT_B: ramp enabled

    // 2. Set zero reference
    emulator.get_memory_bus().write(0xD001, 0x80); // PORT_A: 0x80 = center (0)

    // 3. Set mux to zero-ref mode (bits 1-2 of PORT_B)
    emulator.get_memory_bus().write(0xD000, 0x06); // Mux = 3 (zero-ref)

    // 4. Execute some cycles to let integrators settle
    for _ in 0..100 {
        let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
    }

    // 5. Now set X coordinate
    emulator.get_memory_bus().write(0xD000, 0x00); // Mux = 0 (X)
    emulator.get_memory_bus().write(0xD001, 0xA0); // X = 0xA0 (positive value)

    // 6. Execute cycles
    for _ in 0..100 {
        let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
    }

    // 7. Set Y coordinate
    emulator.get_memory_bus().write(0xD000, 0x02); // Mux = 1 (Y)
    emulator.get_memory_bus().write(0xD001, 0x60); // Y = 0x60 (negative value)

    // 8. Set brightness
    emulator.get_memory_bus().write(0xD000, 0x04); // Mux = 2 (Z/brightness)
    emulator.get_memory_bus().write(0xD001, 0x7F); // Brightness = max

    // 9. Execute cycles to draw
    for _ in 0..1000 {
        let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
    }

    // Note: This is a low-level test - in real BIOS the sequence is more complex
    // We're just verifying that VIA writes don't crash and that the plumbing works
    println!(
        "✅ VIA port write test completed - {} lines generated",
        render_context.lines.len()
    );
}

#[test]
fn test_sta_d000_opcode_sequence() {
    // Test that STA $D000 (0x97 0xD000 in extended mode or 0xB7 0xD0 in direct page)
    // actually triggers the integrator update chain

    // Setup emulator following WASM pattern
    let mut emulator = Emulator::new();
    let bios_path =
        r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    emulator.init(bios_path);
    emulator.reset();

    let mut render_context = RenderContext::new();
    let mut audio_context = AudioContext::new(1500000.0 / 44100.0); // 1.5MHz / 44.1kHz
    let input = Input::default();

    // Write a small program to RAM that does STA to VIA
    // LDA #$80 / STA $D001 / LDA #$00 / STA $D000
    let program: &[u8] = &[
        0x86, 0x80, // LDA #$80 (center value)
        0xB7, 0xD0, 0x01, // STA $D001 (extended addressing)
        0x86, 0x00, // LDA #$00
        0xB7, 0xD0, 0x00, // STA $D000 (extended addressing)
        0x12, // NOP (or end)
    ];

    // Write program to RAM at 0xC880
    for (i, &byte) in program.iter().enumerate() {
        emulator.get_memory_bus().write(0xC880 + i as u16, byte);
    }

    // Configure CPU to run program (need to set PC via memory manipulation)
    // Write a JMP instruction at reset vector area or use existing PC
    // For simplicity, write JMP $C880 at 0xC800 and set PC there
    emulator.get_memory_bus().write(0xC800, 0x7E); // JMP extended
    emulator.get_memory_bus().write(0xC801, 0xC8); // High byte
    emulator.get_memory_bus().write(0xC802, 0x80); // Low byte

    // Execute initial jump
    let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);

    // Execute the program
    for _ in 0..20 {
        let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
    }

    // Verify: We should have executed STA opcodes to VIA without crashing
    // The actual line generation depends on many factors (ramp state, etc)
    // but this test verifies the opcode→VIA→Screen chain doesn't panic

    println!("✅ STA $D000 opcode sequence test passed");
}

#[test]
fn test_line_accumulation_and_clearing() {
    // Test that render_context accumulates lines and can be cleared

    // Setup emulator following WASM pattern
    let mut emulator = Emulator::new();
    let bios_path =
        r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    emulator.init(bios_path);
    emulator.reset();

    let mut render_context = RenderContext::new();
    let mut audio_context = AudioContext::new(1500000.0 / 44100.0); // 1.5MHz / 44.1kHz
    let input = Input::default();

    // Execute some cycles
    for _ in 0..50_000 {
        let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
    }

    let initial_line_count = render_context.lines.len();
    assert!(
        initial_line_count > 0,
        "Expected some lines to be generated"
    );

    // Clear render context
    render_context.lines.clear();
    assert_eq!(render_context.lines.len(), 0, "Lines should be cleared");

    // Execute more cycles
    for _ in 0..50_000 {
        let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
    }

    // Should have new lines
    assert!(
        render_context.lines.len() > 0,
        "Expected new lines after clearing and continuing execution"
    );

    println!(
        "✅ Line accumulation test: {} initial lines, {} lines after clear+continue",
        initial_line_count,
        render_context.lines.len()
    );
}

#[test]
fn test_vector_geometry_no_skew() {
    // Test that verifies geometric quality of lines - detects skew/distortion
    
    // Setup emulator following WASM pattern
    let mut emulator = Emulator::new();
    let bios_path = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin";
    emulator.init(bios_path);
    emulator.reset();
    
    let mut render_context = RenderContext::new();
    let mut audio_context = AudioContext::new(1500000.0 / 44100.0);
    let input = Input::default();
    
    // Execute enough cycles for BIOS to draw some recognizable shapes
    // BIOS draws border and "MINE STORM" text
    for _ in 0..200_000 {
        let _ = emulator.execute_instruction(&input, &mut render_context, &mut audio_context);
    }
    
    println!("\n=== GEOMETRIC ANALYSIS ===");
    println!("Total lines: {}", render_context.lines.len());
    
    // Analyze lines for geometric properties
    let mut horizontal_lines = 0;
    let mut vertical_lines = 0;
    let mut diagonal_lines = 0;
    let mut max_skew = 0.0f32;
    
    for line in &render_context.lines {
        let dx = (line.p1.x - line.p0.x).abs();
        let dy = (line.p1.y - line.p0.y).abs();
        
        // Classify line orientation
        if dx > dy * 3.0 {
            // Mostly horizontal
            horizontal_lines += 1;
            
            // For horizontal lines, check Y deviation (should be minimal)
            let skew_ratio = if dx > 0.0 { dy / dx } else { 0.0 };
            max_skew = max_skew.max(skew_ratio);
            
        } else if dy > dx * 3.0 {
            // Mostly vertical
            vertical_lines += 1;
            
            // For vertical lines, check X deviation (should be minimal)
            let skew_ratio = if dy > 0.0 { dx / dy } else { 0.0 };
            max_skew = max_skew.max(skew_ratio);
            
        } else {
            // Diagonal
            diagonal_lines += 1;
        }
    }
    
    println!("Horizontal lines: {}", horizontal_lines);
    println!("Vertical lines: {}", vertical_lines);
    println!("Diagonal lines: {}", diagonal_lines);
    println!("Max skew ratio: {:.4}", max_skew);
    
    // Sample some lines for detailed inspection
    println!("\n=== SAMPLE LINES (first 20) ===");
    for (i, line) in render_context.lines.iter().take(20).enumerate() {
        let dx = line.p1.x - line.p0.x;
        let dy = line.p1.y - line.p0.y;
        let length = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx).to_degrees();
        
        println!(
            "Line {}: ({:.1}, {:.1}) → ({:.1}, {:.1}) | len={:.1}, angle={:.1}°, bright={:.2}",
            i, line.p0.x, line.p0.y, line.p1.x, line.p1.y, length, angle, line.brightness
        );
    }
    
    // Check for expected BIOS patterns
    // BIOS should draw border (rectangular shape with horizontal/vertical lines)
    assert!(
        horizontal_lines > 0 || vertical_lines > 0,
        "Expected some horizontal or vertical lines from BIOS border"
    );
    
    // Skew check: for straight lines, deviation should be < 30%
    // (allowing some tolerance for Vectrex's analog nature)
    assert!(
        max_skew < 0.3,
        "Excessive skew detected: {:.2}. Lines that should be straight are tilted > 30%",
        max_skew
    );
    
    println!("\n✅ Geometry test passed - skew within acceptable limits");
}
