use vectrex_emulator::CPU;

// Helper to build a CPU with convenience
fn cpu() -> CPU { CPU::default() }

#[test]
fn timer1_underflow_sets_ifr6_and_irq() {
    let mut c = cpu();
    // Enable IFR6 in IER (bit6)
    c.bus.via.write(0x0E, 0x80 | 0x40); // set bit6
    // Load T1 with small value 0x0003 -> write low then high
    c.bus.via.write(0x04, 0x03);
    c.bus.via.write(0x05, 0x00);
    // Tick cycles until underflow
    c.bus.tick(3);
    let ifr = c.bus.via.read(0x0D);
    assert!(ifr & 0x40 != 0, "IFR6 not set after underflow, IFR={:02X}", ifr);
    assert!(c.bus.via.irq_asserted(), "IRQ line not asserted after T1 underflow");
}

#[test]
fn timer1_continuous_reloads() {
    let mut c = cpu();
    c.bus.via.write(0x0E, 0x80 | 0x40); // enable T1 interrupt
    c.bus.via.write(0x0B, 0x40); // ACR bit6 continuous
    c.bus.via.write(0x04, 0x04); c.bus.via.write(0x05, 0x00); // load 0x0004
    c.bus.tick(4); // first underflow
    let after_first = c.bus.via.read(0x04); // also clears IFR6
    assert!(after_first <= 4, "Counter unexpected after first read");
    // Should have reloaded; tick again to cause another underflow
    c.bus.tick(4);
    let ifr = c.bus.via.read(0x0D);
    assert!(ifr & 0x40 != 0, "Continuous mode failed to set IFR6 again");
}

#[test]
fn pb7_toggles_on_t1_underflow_when_enabled() {
    let mut c = cpu();
    c.bus.via.write(0x0E, 0x80 | 0x40); // enable T1 interrupt
    c.bus.via.write(0x0B, 0xC0); // ACR bit7 PB7 enable, bit6 continuous
    c.bus.via.write(0x04, 0x02); c.bus.via.write(0x05, 0x00); // 0x0002
    let initial = c.bus.via.pb7();
    c.bus.tick(2); // first underflow
    let first = c.bus.via.pb7();
    assert_ne!(initial, first, "PB7 did not toggle on first underflow");
    c.bus.tick(2); // second underflow
    let second = c.bus.via.pb7();
    assert_ne!(first, second, "PB7 did not toggle on second underflow");
}

#[test]
fn timer2_underflow_sets_ifr5() {
    let mut c = cpu();
    c.bus.via.write(0x0E, 0x80 | 0x20); // enable bit5
    c.bus.via.write(0x08, 0x03); c.bus.via.write(0x09, 0x00); // load 0x0003
    c.bus.tick(3);
    let ifr = c.bus.via.read(0x0D);
    assert!(ifr & 0x20 != 0, "IFR5 not set after Timer2 underflow");
}

#[test]
fn timer2_ifr5_clears_on_low_read_and_irq_drops() {
    let mut c = cpu();
    c.bus.via.write(0x0E, 0x80 | 0x20); // enable T2 interrupt bit5
    c.bus.via.write(0x08, 0x02); c.bus.via.write(0x09, 0x00); // load 0x0002
    c.bus.tick(2); // underflow -> set IFR5
    let ifr_before = c.bus.via.read(0x0D);
    assert!(ifr_before & 0x20 != 0, "Precondition failed: IFR5 not set (IFR={:02X})", ifr_before);
    assert!(c.bus.via.irq_asserted(), "IRQ line not asserted before clearing IFR5");
    // Read high first: should NOT clear IFR5 now
    let _hi = c.bus.via.read(0x09);
    let ifr_mid = c.bus.via.read(0x0D);
    assert!(ifr_mid & 0x20 != 0, "IFR5 cleared unexpectedly by high read (IFR={:02X})", ifr_mid);
    assert!(c.bus.via.irq_asserted(), "IRQ line dropped unexpectedly after high read");
    // Read low: should clear IFR5 and deassert IRQ
    let _lo = c.bus.via.read(0x08);
    let ifr_after = c.bus.via.read(0x0D);
    assert!(ifr_after & 0x20 == 0, "IFR5 not cleared after reading T2 low byte (IFR={:02X})", ifr_after);
    assert!(!c.bus.via.irq_asserted(), "IRQ line still asserted after clearing IFR5 via low read");
}

#[test]
fn timer2_high_read_does_not_clear_ifr5() {
    let mut c = cpu();
    c.bus.via.write(0x0E, 0x80 | 0x20);
    c.bus.via.write(0x08, 0x03); c.bus.via.write(0x09, 0x00);
    c.bus.tick(3); // underflow
    // Read high; should not clear
    let _hi = c.bus.via.read(0x09);
    let ifr = c.bus.via.read(0x0D);
    assert!(ifr & 0x20 != 0, "IFR5 unexpectedly cleared by T2C-H read (IFR={:02X})", ifr);
}

#[test]
fn timer1_high_read_does_not_clear_ifr6() {
    let mut c = cpu();
    c.bus.via.write(0x0E, 0x80 | 0x40); // enable T1 bit6
    c.bus.via.write(0x04, 0x02); c.bus.via.write(0x05, 0x00); // load 0x0002
    c.bus.tick(2); // underflow set IFR6
    // Read high first: should NOT clear now
    let _hi = c.bus.via.read(0x05);
    let ifr_mid = c.bus.via.read(0x0D);
    assert!(ifr_mid & 0x40 != 0, "IFR6 unexpectedly cleared by high read (IFR={:02X})", ifr_mid);
    // Read low: clears IFR6
    let _lo = c.bus.via.read(0x04);
    let ifr_after = c.bus.via.read(0x0D);
    assert!(ifr_after & 0x40 == 0, "IFR6 not cleared after low read (IFR={:02X})", ifr_after);
}

#[test]
fn shift_register_free_run_sets_ifr4() {
    let mut c = cpu();
    c.bus.via.write(0x0E, 0x80 | 0x10); // enable bit4
    // Set ACR mode to internal free-run (using our simplified decode (acr >>2)&7 == 0b100)
    // We write a value where bits 4-2 are 0b100 => 0b100 <<2 = 0b10000 = 0x10
    c.bus.via.write(0x0B, 0x10);
    c.bus.via.write(0x0A, 0x55); // load SR & start
    // Enough cycles for at least 8 bits (8*8=64 cycles with placeholder rate)
    c.bus.tick(80);
    let ifr = c.bus.via.read(0x0D);
    assert!(ifr & 0x10 != 0, "IFR4 not set after shift completion (IFR={:02X})", ifr);
}
