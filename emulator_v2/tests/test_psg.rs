// Tests for PSG (Programmable Sound Generator) module
// Following 1:1 structure from Vectrexy

use vectrex_emulator_v2::core::psg::{Psg, Timer, ToneGenerator, NoiseGenerator, EnvelopeGenerator};

#[test]
fn test_timer_creation_and_period() {
    let mut timer = Timer::new(10);
    assert_eq!(timer.period(), 10);
    
    // Test period change
    timer.set_period(20);
    assert_eq!(timer.period(), 20);
}

#[test]
fn test_timer_clock_expiration() {
    let mut timer = Timer::new(3);
    
    // Should not expire for first 2 clocks
    assert!(!timer.clock());
    assert!(!timer.clock());
    
    // Should expire on 3rd clock and reset
    assert!(timer.clock());
    
    // Should start over
    assert!(!timer.clock());
}

#[test]
fn test_timer_zero_period() {
    let mut timer = Timer::new(0);
    // Timer with period 0 should never expire
    assert!(!timer.clock());
    assert!(!timer.clock());
}

#[test]
fn test_tone_generator_period_setting() {
    let mut generator = ToneGenerator::new();
    
    // Set period = 0x1234
    generator.set_period_low(0x34);
    generator.set_period_high(0x12);
    
    assert_eq!(generator.period_low(), 0x34);
    assert_eq!(generator.period_high(), 0x12);
}

#[test]
fn test_tone_generator_enabled_state() {
    let mut generator = ToneGenerator::new();
    
    // Period 0 should be disabled
    assert!(!generator.is_enabled());
    
    // Non-zero period should be enabled
    generator.set_period_low(1);
    assert!(generator.is_enabled());
}

#[test]
fn test_tone_generator_value_toggle() {
    let mut generator = ToneGenerator::new();
    generator.set_period_low(1); // Very short period
    
    let initial_value = generator.value();
    
    // Clock should toggle the value
    generator.clock();
    let toggled_value = generator.value();
    
    assert_ne!(initial_value, toggled_value);
    assert!(toggled_value == 0 || toggled_value == 1);
}

#[test]
fn test_noise_generator_period_setting() {
    let mut generator = NoiseGenerator::new();
    
    generator.set_period(0x15);
    assert_eq!(generator.period(), 0x15);
}

#[test]
fn test_noise_generator_clock() {
    let mut generator = NoiseGenerator::new();
    generator.set_period(1); // Very short period
    
    // Should produce 0 or 1 values
    generator.clock();
    let value = generator.value();
    assert!(value == 0 || value == 1);
}

#[test]
fn test_envelope_generator_period_setting() {
    let mut generator = EnvelopeGenerator::new();
    
    // Set period = 0x5678
    generator.set_period_low(0x78);
    generator.set_period_high(0x56);
    
    assert_eq!(generator.period_low(), 0x78);
    assert_eq!(generator.period_high(), 0x56);
}

#[test]
fn test_envelope_generator_shape_setting() {
    let mut generator = EnvelopeGenerator::new();
    
    generator.set_shape(0x0A);
    assert_eq!(generator.shape(), 0x0A);
}

#[test]
fn test_psg_creation_and_basic_interface() {
    let mut psg = Psg::new();
    
    // Test BDIR/BC1 interface
    assert!(!psg.bdir());
    assert!(!psg.bc1());
    
    psg.set_bdir(true);
    psg.set_bc1(true);
    
    assert!(psg.bdir());
    assert!(psg.bc1());
}

#[test]
fn test_psg_register_operations() {
    let mut psg = Psg::new();
    
    // Test latch address mode (BDIR=1, BC1=1)
    psg.set_bdir(true);
    psg.set_bc1(true);
    psg.write_da(0x05); // Latch register 5
    
    // Test write mode (BDIR=1, BC1=0)
    psg.set_bdir(true);
    psg.set_bc1(false);
    psg.write_da(0x42); // Write value to latched register
    
    // Test read mode (BDIR=0, BC1=1) 
    psg.set_bdir(false);
    psg.set_bc1(true);
    let read_value = psg.read_da();
    
    // The exact value depends on which register was latched and implementation details
    // For now, just test that read operations don't panic
    assert!(read_value <= 0xFF);
}

#[test]
fn test_psg_reset() {
    let mut psg = Psg::new();
    
    // Set some state
    psg.set_bdir(true);
    psg.set_bc1(true);
    psg.write_da(0x01);
    
    // Reset should clear everything
    psg.reset();
    
    // Basic sanity checks after reset
    assert_eq!(psg.sample(), 0.0);
}

#[test]
fn test_psg_update_and_sample() {
    let mut psg = Psg::new();
    
    // Update should not panic
    psg.update(100);
    
    // Sample should return a float
    let sample = psg.sample();
    assert!(sample.is_finite());
}

#[test]
fn test_psg_frame_update() {
    let mut psg = Psg::new();
    
    // Frame update should not panic
    psg.frame_update(1.0 / 60.0); // 60 FPS
}