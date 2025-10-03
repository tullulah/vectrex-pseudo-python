// Tests for Timers module
// Following 1:1 structure from Vectrexy

use vectrex_emulator_v2::core::timers::{TimerMode, Timer1, Timer2};

#[test]
fn test_timer_mode_to_string() {
    assert_eq!(TimerMode::FreeRunning.to_string(), "FreeRunning");
    assert_eq!(TimerMode::OneShot.to_string(), "OneShot");
    assert_eq!(TimerMode::PulseCounting.to_string(), "PulseCounting");
}

#[test]
fn test_timer1_latch_and_counter() {
    let mut timer1 = Timer1::new();
    
    // Write latches
    timer1.write_counter_low(0x34);
    timer1.write_counter_high(0x12);
    
    // Should transfer to counter
    assert_eq!(timer1.read_counter_low(), 0x34);
    assert_eq!(timer1.read_counter_high(), 0x12);
}

#[test]
fn test_timer2_counter() {
    let mut timer2 = Timer2::new();
    
    // Write counter
    timer2.write_counter_low(0x34);
    timer2.write_counter_high(0x12);
    
    // Should set counter value
    assert_eq!(timer2.read_counter_low(), 0x34);
    assert_eq!(timer2.read_counter_high(), 0x12);
}

#[test]
fn test_timer1_interrupt_flag_on_update() {
    let mut timer1 = Timer1::new();
    timer1.write_counter_low(5);
    timer1.write_counter_high(0);
    
    // Should not expire
    timer1.update(3);
    assert!(!timer1.interrupt_flag());
    
    // Should expire
    timer1.update(5);
    assert!(timer1.interrupt_flag());
}

#[test]
fn test_timer2_interrupt_flag_on_update() {
    let mut timer2 = Timer2::new();
    timer2.write_counter_low(5);
    timer2.write_counter_high(0);
    
    // Should not expire
    timer2.update(3);
    assert!(!timer2.interrupt_flag());
    
    // Should expire
    timer2.update(5);
    assert!(timer2.interrupt_flag());
}

#[test]
fn test_timer1_pb7_signal() {
    let mut timer1 = Timer1::new();
    timer1.set_pb7_flag(true);
    
    // Initially pb7_signal_low should be false
    assert!(!timer1.pb7_signal_low());
    
    // Write counter_high=0, counter_low=2 -> counter = 2
    timer1.write_counter_low(2);
    timer1.write_counter_high(0);
    assert!(timer1.pb7_signal_low());
    
    // Update with enough cycles to expire should set pb7_signal_low=false
    timer1.update(3); // Should expire (3 >= 2) and set pb7_signal_low to false
    assert!(!timer1.pb7_signal_low());
    assert!(timer1.interrupt_flag()); // Should also set interrupt flag
}