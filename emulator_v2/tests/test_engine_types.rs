// Tests for EngineTypes module  
// Following 1:1 structure from Vectrexy

use vectrex_emulator_v2::core::engine_types::{Input, RenderContext, AudioContext, EmuEvent, EmuEventType};
use vectrex_emulator_v2::types::{Line, Vector2};
use std::path::PathBuf;

#[test]
fn test_input_button_operations() {
    let mut input = Input::new();
    
    // Initially all buttons should be "not pressed" (bits high)
    assert_eq!(input.button_state_mask(), 0xFF);
    assert!(!input.is_button_down(0, 0)); // Joy 0, Button 0
    assert!(!input.is_button_down(1, 3)); // Joy 1, Button 3
    
    // Press button 0 on joystick 0
    input.set_button(0, 0, true);
    assert!(input.is_button_down(0, 0));
    assert!(!input.is_button_down(0, 1)); // Other buttons still not pressed
    
    // Release button
    input.set_button(0, 0, false);
    assert!(!input.is_button_down(0, 0));
}

#[test]
fn test_input_analog_operations() {
    let mut input = Input::new();
    
    // Initially all analog values should be 0
    assert_eq!(input.analog_state_mask(0), 0); // X1
    assert_eq!(input.analog_state_mask(1), 0); // Y1
    assert_eq!(input.analog_state_mask(2), 0); // X2
    assert_eq!(input.analog_state_mask(3), 0); // Y2
    
    // Set analog values
    input.set_analog_axis_x(0, 127);  // Joystick 0, X axis
    input.set_analog_axis_y(0, -128); // Joystick 0, Y axis
    input.set_analog_axis_x(1, 64);   // Joystick 1, X axis
    input.set_analog_axis_y(1, -64);  // Joystick 1, Y axis
    
    assert_eq!(input.analog_state_mask(0), 127);  // X1
    assert_eq!(input.analog_state_mask(1), -128); // Y1
    assert_eq!(input.analog_state_mask(2), 64);   // X2
    assert_eq!(input.analog_state_mask(3), -64);  // Y2
}

#[test]
fn test_line_creation() {
    let line = Line::new(Vector2::new(0.0, 0.0), Vector2::new(100.0, 100.0), 255.0);
    assert_eq!(line.p0.x, 0.0);
    assert_eq!(line.p0.y, 0.0);
    assert_eq!(line.p1.x, 100.0);
    assert_eq!(line.p1.y, 100.0);
    assert_eq!(line.brightness, 255.0);
}

#[test]
fn test_render_context_operations() {
    let mut context = RenderContext::new();
    assert_eq!(context.lines.len(), 0);
    
    let line1 = Line::new(Vector2::new(0.0, 0.0), Vector2::new(10.0, 10.0), 128.0);
    let line2 = Line::new(Vector2::new(10.0, 10.0), Vector2::new(20.0, 20.0), 64.0);
    
    context.add_line(line1);
    context.add_line(line2);
    assert_eq!(context.lines.len(), 2);
    
    context.clear();
    assert_eq!(context.lines.len(), 0);
}

#[test]
fn test_audio_context_operations() {
    let mut context = AudioContext::new(44.1);
    assert_eq!(context.cpu_cycles_per_audio_sample, 44.1);
    assert_eq!(context.samples.len(), 0);
    
    context.add_sample(0.5);
    context.add_sample(-0.3);
    assert_eq!(context.samples.len(), 2);
    assert_eq!(context.samples[0], 0.5);
    assert_eq!(context.samples[1], -0.3);
    
    context.clear();
    assert_eq!(context.samples.len(), 0);
}

#[test]
fn test_emu_event_creation() {
    let event1 = EmuEvent::break_into_debugger();
    match event1.event_type {
        EmuEventType::BreakIntoDebugger => {},
        _ => panic!("Wrong event type"),
    }
    
    let event2 = EmuEvent::reset();
    match event2.event_type {
        EmuEventType::Reset => {},
        _ => panic!("Wrong event type"),
    }
    
    let path = Some(PathBuf::from("test.rom"));
    let event3 = EmuEvent::open_rom_file(path.clone());
    match event3.event_type {
        EmuEventType::OpenRomFile { path: p } => {
            assert_eq!(p, path);
        },
        _ => panic!("Wrong event type"),
    }
    
    let bios_path = Some(PathBuf::from("bios.bin"));
    let event4 = EmuEvent::open_bios_rom_file(bios_path.clone());
    match event4.event_type {
        EmuEventType::OpenBiosRomFile { path: p } => {
            assert_eq!(p, bios_path);
        },
        _ => panic!("Wrong event type"),
    }
}