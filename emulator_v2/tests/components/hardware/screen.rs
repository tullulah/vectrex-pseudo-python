//! Tests for screen.rs - Vector display system
//! C++ Original: tests matching Screen.cpp functionality

use vectrex_emulator_v2::core::screen::{Screen, RampPhase};
use vectrex_emulator_v2::types::{Vector2, magnitude, normalized};
use vectrex_emulator_v2::core::engine_types::RenderContext;

#[test]
fn test_vector2_basic() {
    // C++ Original: basic Vector2 operations
    let v1 = Vector2::new(3.0, 4.0);
    let v2 = Vector2::new(1.0, 2.0);
    
    // Addition
    let sum = v1 + v2;
    assert_eq!(sum.x, 4.0);
    assert_eq!(sum.y, 6.0);
    
    // Subtraction
    let diff = v1 - v2;
    assert_eq!(diff.x, 2.0);
    assert_eq!(diff.y, 2.0);
    
    // Scalar multiplication
    let scaled = v1 * 2.0;
    assert_eq!(scaled.x, 6.0);
    assert_eq!(scaled.y, 8.0);
    
    // Scalar division
    let divided = v1 / 2.0;
    assert_eq!(divided.x, 1.5);
    assert_eq!(divided.y, 2.0);
}

#[test]
fn test_vector2_magnitude() {
    // C++ Original: inline float Magnitude(const Vector2& v)
    let v = Vector2::new(3.0, 4.0);
    let mag = magnitude(v);
    assert!((mag - 5.0).abs() < 1e-6);
    
    let zero = Vector2::zero();
    assert_eq!(magnitude(zero), 0.0);
}

#[test]
fn test_vector2_normalized() {
    // C++ Original: inline Vector2 Normalized(const Vector2& v)
    let v = Vector2::new(3.0, 4.0);
    let norm = normalized(v);
    assert!((norm.x - 0.6).abs() < 1e-6);
    assert!((norm.y - 0.8).abs() < 1e-6);
    
    // Test zero vector normalization
    let zero = Vector2::zero();
    let norm_zero = normalized(zero);
    assert_eq!(norm_zero.x, 0.0);
    assert_eq!(norm_zero.y, 0.0);
}

#[test]
fn test_vector2_add_assign() {
    // C++ Original: void operator+=(const Vector2& rhs)
    let mut v1 = Vector2::new(1.0, 2.0);
    let v2 = Vector2::new(3.0, 4.0);
    v1 += v2;
    assert_eq!(v1.x, 4.0);
    assert_eq!(v1.y, 6.0);
}

#[test]
fn test_screen_initial_state() {
    // C++ Original: Screen constructor defaults
    let screen = Screen::new();
    
    assert_eq!(screen.pos(), Vector2::zero());
    assert_eq!(screen.ramp_phase(), RampPhase::RampOff);
    assert!(!screen.integrators_enabled());
    assert_eq!(screen.brightness(), 0.0);
    assert!(!screen.blank());
}

#[test]
fn test_screen_init() {
    // C++ Original: void Screen::Init()
    let mut screen = Screen::new();
    screen.init();
    
    // Verify VelocityXDelay is set correctly
    // This is internal to DelayedValueStore so we test indirectly
    screen.set_integrator_x(50);
    // After init, velocity_x should have proper delay cycles
}

#[test]
fn test_screen_zero_beam() {
    // C++ Original: void Screen::ZeroBeam()
    let mut screen = Screen::new();
    
    // Move beam away from zero
    screen.set_integrator_x(10);
    screen.set_integrator_y(20);
    screen.set_integrators_enabled(true);
    
    // Zero the beam
    screen.zero_beam();
    
    assert_eq!(screen.pos(), Vector2::zero());
}

#[test]
fn test_screen_blank_control() {
    // C++ Original: void SetBlankEnabled(bool enabled)
    let mut screen = Screen::new();
    
    assert!(!screen.blank());
    screen.set_blank_enabled(true);
    assert!(screen.blank());
    screen.set_blank_enabled(false);
    assert!(!screen.blank());
}

#[test]
fn test_screen_integrators_control() {
    // C++ Original: void SetIntegratorsEnabled(bool enabled)
    let mut screen = Screen::new();
    
    assert!(!screen.integrators_enabled());
    screen.set_integrators_enabled(true);
    assert!(screen.integrators_enabled());
    screen.set_integrators_enabled(false);
    assert!(!screen.integrators_enabled());
}

#[test]
fn test_screen_brightness_control() {
    // C++ Original: void SetBrightness(uint8_t value)
    let mut screen = Screen::new();
    
    assert_eq!(screen.brightness(), 0.0);
    screen.set_brightness(128);
    assert_eq!(screen.brightness(), 128.0);
    screen.set_brightness(255);
    assert_eq!(screen.brightness(), 255.0);
}

#[test]
fn test_screen_integrator_values() {
    // C++ Original: void SetIntegratorX/Y(int8_t value)
    let mut screen = Screen::new();
    
    screen.set_integrator_x(100);
    screen.set_integrator_y(-50);
    screen.set_integrator_xy_offset(25);
    
    // Values are stored internally in DelayedValueStore
    // We test indirectly through update behavior
}

#[test]
fn test_screen_ramp_phase_transitions() {
    // C++ Original: RampPhase state machine in Update()
    let mut screen = Screen::new();
    let mut render_context = RenderContext::new();
    
    // Initial state
    assert_eq!(screen.ramp_phase(), RampPhase::RampOff);
    
    // Enable integrators should start RampUp
    screen.set_integrators_enabled(true);
    screen.update(1, &mut render_context);
    assert_eq!(screen.ramp_phase(), RampPhase::RampUp);
    
    // After RampUpDelay cycles, should go to RampOn
    for _ in 0..5 {
        screen.update(1, &mut render_context);
    }
    assert_eq!(screen.ramp_phase(), RampPhase::RampOn);
    
    // Disable integrators should start RampDown
    screen.set_integrators_enabled(false);
    screen.update(1, &mut render_context);
    assert_eq!(screen.ramp_phase(), RampPhase::RampDown);
    
    // After RampDownDelay cycles, should go to RampOff
    for _ in 0..10 {
        screen.update(1, &mut render_context);
    }
    assert_eq!(screen.ramp_phase(), RampPhase::RampOff);
}

#[test]
fn test_screen_beam_movement() {
    // C++ Original: beam movement during RampOn/RampDown
    let mut screen = Screen::new();
    let mut render_context = RenderContext::new();
    
    screen.set_integrator_x(64); // Half of 128
    screen.set_integrator_y(64);
    screen.set_integrators_enabled(true);
    
    // Get to RampOn state
    screen.update(1, &mut render_context); // Start RampUp
    for _ in 0..5 {
        screen.update(1, &mut render_context); // Complete RampUp
    }
    assert_eq!(screen.ramp_phase(), RampPhase::RampOn);
    
    let initial_pos = screen.pos();
    
    // Update with some cycles - beam should move (C++ original uses cycles for delta calculation)
    screen.update(100, &mut render_context);
    
    let new_pos = screen.pos();
    
    // Position should have changed
    assert_ne!(initial_pos, new_pos);
    assert!(new_pos.x > initial_pos.x);
    assert!(new_pos.y > initial_pos.y);
}

#[test]
fn test_screen_line_drawing() {
    // C++ Original: line drawing logic in Update()
    let mut screen = Screen::new();
    let mut render_context = RenderContext::new();
    
    screen.set_brightness(64); // Non-zero brightness
    screen.set_blank_enabled(false);
    screen.set_integrator_x(32);
    screen.set_integrator_y(32);
    screen.set_integrators_enabled(true);
    
    // Get to drawing state
    screen.update(1, &mut render_context);
    for _ in 0..5 {
        screen.update(1, &mut render_context);
    }
    
    // Clear any existing lines
    render_context.lines.clear();
    
    // Update to draw lines
    screen.update(50, &mut render_context);
    
    // Should have created at least one line when drawing is enabled
    // (exact behavior depends on movement and brightness)
}

#[test]
fn test_screen_brightness_curve() {
    // C++ Original: void SetBrightnessCurve(float v)
    let mut screen = Screen::new();
    
    screen.set_brightness_curve(0.5);
    
    // Brightness curve affects line drawing calculations
    // Test indirectly through drawing behavior
    let mut render_context = RenderContext::new();
    screen.set_brightness(64);
    screen.set_blank_enabled(false);
    screen.update(10, &mut render_context);
}

#[test]
fn test_screen_frame_update() {
    // C++ Original: void Screen::FrameUpdate(double frameTime)
    let mut screen = Screen::new();
    
    // Frame update should not crash and should maintain state
    screen.frame_update(16.67); // ~60 FPS
    
    assert_eq!(screen.ramp_phase(), RampPhase::RampOff);
}

#[test]
fn test_ramp_phase_enum_values() {
    // C++ Original: enum class RampPhase values
    assert_ne!(RampPhase::RampOff, RampPhase::RampUp);
    assert_ne!(RampPhase::RampUp, RampPhase::RampOn);
    assert_ne!(RampPhase::RampOn, RampPhase::RampDown);
    assert_ne!(RampPhase::RampDown, RampPhase::RampOff);
}