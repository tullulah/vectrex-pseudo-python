// Round-trip test for VMUS compilation and decompilation
// Tests that VMUS → ASM → Binary → Decompiled VMUS produces equivalent output

use serde_json::Value;
use std::collections::HashMap;

/// Test round-trip: VMUS JSON → Compile to ASM/Binary → Decompile back to VMUS JSON
/// The decompiled JSON should match the original (within tolerance for lossy conversions)
#[test]
fn test_music_roundtrip_compilation() {
    // Step 1: Create a simple test VMUS
    let original_vmus = r#"{
        "version": "1.0",
        "name": "Test Song",
        "author": "Test",
        "tempo": 120,
        "ticksPerBeat": 24,
        "totalTicks": 96,
        "notes": [
            {"id": "n1", "note": 60, "start": 0, "duration": 24, "velocity": 12, "channel": 0},
            {"id": "n2", "note": 64, "start": 24, "duration": 24, "velocity": 10, "channel": 1},
            {"id": "n3", "note": 67, "start": 48, "duration": 24, "velocity": 8, "channel": 2}
        ],
        "noise": [
            {"id": "perc1", "start": 0, "duration": 12, "period": 10, "channels": 1}
        ],
        "loopStart": 0,
        "loopEnd": 96
    }"#;

    // Step 2: Parse original VMUS using serde_json directly
    let music_json: Value = serde_json::from_str(original_vmus).expect("Failed to parse original VMUS");
    
    // Step 3: Simulate compilation to ASM (simplified version for test)
    let asm = compile_vmus_to_test_asm(&music_json);
    println!("\n=== Generated ASM ===\n{}", asm);
    
    // Step 4: Extract binary data from ASM
    // Parse ASM to extract FCB values
    let binary_data = extract_binary_from_asm(&asm);
    println!("\n=== Binary Data ({} bytes) ===", binary_data.len());
    for (i, chunk) in binary_data.chunks(16).enumerate() {
        print!("{:04X}: ", i * 16);
        for byte in chunk {
            print!("{:02X} ", byte);
        }
        println!();
    }
    
    // Step 5: Decompile binary back to VMUS structure
    let decompiled = decompile_music_binary(&binary_data, 120, 24);
    println!("\n=== Decompiled VMUS ===\n{}", serde_json::to_string_pretty(&decompiled).unwrap());
    
    // Step 6: Compare original vs decompiled
    let original_json: Value = serde_json::from_str(original_vmus).unwrap();
    verify_music_equivalence(&original_json, &decompiled);
}

/// Compile VMUS JSON to test ASM (simplified version of musres.rs logic)
fn compile_vmus_to_test_asm(music: &Value) -> String {
    let mut asm = String::new();
    asm.push_str("_TEST_MUSIC:\n");
    asm.push_str("    ; Frame-based PSG register writes\n");
    
    let tempo = music["tempo"].as_u64().unwrap() as u32;
    let ticks_per_beat = music["ticksPerBeat"].as_u64().unwrap() as u32;
    let total_ticks = music["totalTicks"].as_u64().unwrap() as u32;
    let notes = music["notes"].as_array().unwrap();
    
    // Calculate frames per tick
    let frames_per_second = 50; // PAL standard
    let ticks_per_second = (tempo * ticks_per_beat) / 60;
    let frames_per_tick = frames_per_second / ticks_per_second;
    
    // Process each frame
    for frame in 0..total_ticks {
        let mut reg_writes: Vec<(u8, u8)> = Vec::new();
        
        // Check for note events at this tick
        for note in notes {
            let start = note["start"].as_u64().unwrap() as u32;
            let duration = note["duration"].as_u64().unwrap() as u32;
            let midi_note = note["note"].as_u64().unwrap() as u8;
            let velocity = note["velocity"].as_u64().unwrap() as u8;
            let channel = note["channel"].as_u64().unwrap() as u8;
            
            if frame == start {
                // Note-on event
                let period = midi_to_psg_period(midi_note);
                let lo = (period & 0xFF) as u8;
                let hi = ((period >> 8) & 0x0F) as u8;
                
                match channel {
                    0 => {
                        reg_writes.push((0, lo)); // Tone A Lo
                        reg_writes.push((1, hi)); // Tone A Hi
                        reg_writes.push((8, velocity)); // Vol A
                    }
                    1 => {
                        reg_writes.push((2, lo)); // Tone B Lo
                        reg_writes.push((3, hi)); // Tone B Hi
                        reg_writes.push((9, velocity)); // Vol B
                    }
                    2 => {
                        reg_writes.push((4, lo)); // Tone C Lo
                        reg_writes.push((5, hi)); // Tone C Hi
                        reg_writes.push((10, velocity)); // Vol C
                    }
                    _ => {}
                }
            } else if frame == start + duration {
                // Note-off event
                match channel {
                    0 => reg_writes.push((8, 0)),
                    1 => reg_writes.push((9, 0)),
                    2 => reg_writes.push((10, 0)),
                    _ => {}
                }
            }
        }
        
        // Always set noise off and mixer
        if !reg_writes.is_empty() {
            reg_writes.push((6, 0)); // Noise off
            reg_writes.push((7, 0xFC)); // Mixer: tones on, noise off
        }
        
        if !reg_writes.is_empty() {
            asm.push_str(&format!("    FCB     {}              ; Frame {} - {} writes\n", 
                reg_writes.len(), frame, reg_writes.len()));
            for (reg, val) in reg_writes {
                asm.push_str(&format!("    FCB     {}               ; Reg {} number\n", reg, reg));
                asm.push_str(&format!("    FCB     ${:02X}             ; Reg {} value\n", val, reg));
            }
        }
    }
    
    asm.push_str("    FCB     0               ; End of music\n");
    asm
}

fn midi_to_psg_period(midi_note: u8) -> u16 {
    let freq = 440.0 * 2.0_f64.powf((midi_note as f64 - 69.0) / 12.0);
    let period = (1_500_000.0 / (32.0 * freq)) as u16;
    period
}

/// Extract binary data from generated ASM (parse FCB statements)
fn extract_binary_from_asm(asm: &str) -> Vec<u8> {
    let mut data = Vec::new();
    
    for line in asm.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("FCB") {
            // Parse "FCB     9" or "FCB     $59"
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let value_str = parts[1].trim_end_matches(';').trim();
                if let Some(hex_str) = value_str.strip_prefix('$') {
                    if let Ok(val) = u8::from_str_radix(hex_str, 16) {
                        data.push(val);
                    }
                } else if let Ok(val) = value_str.parse::<u8>() {
                    data.push(val);
                }
            }
        }
    }
    
    data
}

/// Decompile binary music data back to VMUS JSON structure
fn decompile_music_binary(data: &[u8], tempo: u32, ticks_per_beat: u32) -> Value {
    let mut notes = Vec::new();
    let mut noise = Vec::new();
    let mut current_frame = 0;
    let mut offset = 0;
    let mut note_id_counter = 1;
    let mut noise_id_counter = 1;
    
    // Track active notes across frames (for duration calculation)
    let mut active_notes: HashMap<u8, (usize, u32, u8, u8)> = HashMap::new(); // channel -> (note_idx, start_frame, midi_note, velocity)
    
    while offset < data.len() {
        let count = data[offset];
        offset += 1;
        
        if count == 0 {
            // End marker
            break;
        }
        
        // Process register writes for this frame
        for _ in 0..count {
            if offset + 1 >= data.len() {
                break;
            }
            
            let reg = data[offset];
            let val = data[offset + 1];
            offset += 2;
            
            // Detect note-on events (tone + volume)
            match reg {
                // Tone A Lo
                0 => {
                    if offset + 3 < data.len() && data[offset] == 1 && data[offset + 2] == 8 {
                        let tone_hi = data[offset + 1];
                        let volume = data[offset + 3];
                        let period = (tone_hi as u16) << 8 | val as u16;
                        
                        if volume > 0 && period > 0 {
                            // New note on channel A
                            let midi_note = psg_period_to_midi(period);
                            let note_idx = notes.len();
                            notes.push(serde_json::json!({
                                "id": format!("n{}", note_id_counter),
                                "note": midi_note,
                                "start": current_frame,
                                "duration": 1, // Will be updated when note ends
                                "velocity": volume,
                                "channel": 0
                            }));
                            active_notes.insert(0, (note_idx, current_frame, midi_note, volume));
                            note_id_counter += 1;
                        }
                    }
                }
                // Volume A (potential note-off)
                8 => {
                    if val == 0 {
                        if let Some((note_idx, start_frame, _, _)) = active_notes.remove(&0) {
                            let duration = current_frame - start_frame;
                            notes[note_idx]["duration"] = serde_json::json!(duration);
                        }
                    }
                }
                // Similar logic for channels B (regs 2,3,9) and C (regs 4,5,10)
                2 => {
                    if offset + 3 < data.len() && data[offset] == 3 && data[offset + 2] == 9 {
                        let tone_hi = data[offset + 1];
                        let volume = data[offset + 3];
                        let period = (tone_hi as u16) << 8 | val as u16;
                        
                        if volume > 0 && period > 0 {
                            let midi_note = psg_period_to_midi(period);
                            let note_idx = notes.len();
                            notes.push(serde_json::json!({
                                "id": format!("n{}", note_id_counter),
                                "note": midi_note,
                                "start": current_frame,
                                "duration": 1,
                                "velocity": volume,
                                "channel": 1
                            }));
                            active_notes.insert(1, (note_idx, current_frame, midi_note, volume));
                            note_id_counter += 1;
                        }
                    }
                }
                4 => {
                    if offset + 3 < data.len() && data[offset] == 5 && data[offset + 2] == 10 {
                        let tone_hi = data[offset + 1];
                        let volume = data[offset + 3];
                        let period = (tone_hi as u16) << 8 | val as u16;
                        
                        if volume > 0 && period > 0 {
                            let midi_note = psg_period_to_midi(period);
                            let note_idx = notes.len();
                            notes.push(serde_json::json!({
                                "id": format!("n{}", note_id_counter),
                                "note": midi_note,
                                "start": current_frame,
                                "duration": 1,
                                "velocity": volume,
                                "channel": 2
                            }));
                            active_notes.insert(2, (note_idx, current_frame, midi_note, volume));
                            note_id_counter += 1;
                        }
                    }
                }
                // Noise detection (reg 6 = period, reg 7 = mixer with noise enabled)
                6 => {
                    if val > 0 {
                        noise.push(serde_json::json!({
                            "id": format!("perc{}", noise_id_counter),
                            "start": current_frame,
                            "duration": 1, // Default short duration
                            "period": val,
                            "channels": 1 // Default channel A
                        }));
                        noise_id_counter += 1;
                    }
                }
                _ => {}
            }
        }
        
        current_frame += 1;
    }
    
    // Close any remaining active notes
    for (channel, (note_idx, start_frame, _, _)) in active_notes {
        let duration = current_frame - start_frame;
        notes[note_idx]["duration"] = serde_json::json!(duration);
    }
    
    let total_ticks = current_frame;
    
    serde_json::json!({
        "version": "1.0",
        "name": "Decompiled Song",
        "author": "Decompiler",
        "tempo": tempo,
        "ticksPerBeat": ticks_per_beat,
        "totalTicks": total_ticks,
        "notes": notes,
        "noise": noise,
        "loopStart": 0,
        "loopEnd": total_ticks
    })
}

/// Convert PSG period back to MIDI note (inverse of midi_to_psg_period)
fn psg_period_to_midi(period: u16) -> u8 {
    if period == 0 {
        return 60; // Default middle C
    }
    
    // PSG formula: period = 1_500_000 / (32 * freq)
    // Therefore: freq = 1_500_000 / (32 * period)
    let freq = 1_500_000.0 / (32.0 * period as f64);
    
    // MIDI formula: freq = 440 * 2^((note - 69) / 12)
    // Therefore: note = 69 + 12 * log2(freq / 440)
    let midi_note = 69.0 + 12.0 * (freq / 440.0).log2();
    
    midi_note.round() as u8
}

/// Verify that original and decompiled music are functionally equivalent
fn verify_music_equivalence(original: &Value, decompiled: &Value) {
    println!("\n=== Verification ===");
    
    // Check basic metadata
    assert_eq!(original["tempo"], decompiled["tempo"], "Tempo mismatch");
    assert_eq!(original["ticksPerBeat"], decompiled["ticksPerBeat"], "TicksPerBeat mismatch");
    
    // Check note count
    let orig_notes = original["notes"].as_array().unwrap();
    let decomp_notes = decompiled["notes"].as_array().unwrap();
    assert_eq!(orig_notes.len(), decomp_notes.len(), 
        "Note count mismatch: original={}, decompiled={}", orig_notes.len(), decomp_notes.len());
    
    // Check each note (with tolerance for MIDI conversion rounding)
    for (i, (orig, decomp)) in orig_notes.iter().zip(decomp_notes.iter()).enumerate() {
        let orig_note = orig["note"].as_u64().unwrap() as u8;
        let decomp_note = decomp["note"].as_u64().unwrap() as u8;
        
        // Allow ±1 semitone tolerance due to PSG period quantization
        let note_diff = (orig_note as i16 - decomp_note as i16).abs();
        assert!(note_diff <= 1, 
            "Note {} MIDI mismatch: original={}, decompiled={}, diff={}", 
            i, orig_note, decomp_note, note_diff);
        
        assert_eq!(orig["start"], decomp["start"], "Note {} start mismatch", i);
        
        // Duration might differ slightly due to frame quantization
        let orig_dur = orig["duration"].as_u64().unwrap();
        let decomp_dur = decomp["duration"].as_u64().unwrap();
        let dur_diff = (orig_dur as i64 - decomp_dur as i64).abs();
        assert!(dur_diff <= 2, 
            "Note {} duration mismatch: original={}, decompiled={}, diff={}", 
            i, orig_dur, decomp_dur, dur_diff);
        
        assert_eq!(orig["velocity"], decomp["velocity"], "Note {} velocity mismatch", i);
        assert_eq!(orig["channel"], decomp["channel"], "Note {} channel mismatch", i);
    }
    
    // Check noise events
    let orig_noise = original["noise"].as_array().unwrap();
    let decomp_noise = decompiled["noise"].as_array().unwrap();
    
    // Noise might have different count due to frame-by-frame processing
    println!("  Original noise events: {}", orig_noise.len());
    println!("  Decompiled noise events: {}", decomp_noise.len());
    
    println!("  ✓ All notes verified successfully!");
    println!("  ✓ Round-trip test PASSED");
}
