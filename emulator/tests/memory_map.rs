use vectrex_emulator::CPU;

#[test]
fn unmapped_read_defaults() {
    let mut cpu = CPU::default();
    // Ensure we have some cart length so 0x0000 space is considered mapped for OOB logic separate from unmapped region.
    cpu.bus.set_cart_len(0x100); // small cart
    // Choose address in 0xD010..0xEFFF (open in simplified model); pick 0xD050.
    let val = cpu.test_read8(0xD050);
    assert_eq!(val, 0xFF, "Unmapped read should return 0xFF (got {val:02X})");
}

#[test]
fn cart_oob_returns_0x01() {
    let mut cpu = CPU::default();
    // Simulate cart loaded length 1 so any address >=1 in window is OOB
    cpu.bus.set_cart_len(1);
    let v = cpu.test_read8(0x1234); // > cart_len so OOB
    assert_eq!(v, 0x01, "Cart OOB should return 0x01 (got {v:02X})");
}

#[test]
fn bios_write_ignored() {
    let mut cpu = CPU::default();
    cpu.bus.set_bios_read_only(true);
    // Load BIOS byte to known value then attempt to overwrite.
    let bios_addr = 0xF000u16; // default BIOS start
    cpu.bus.load_block(bios_addr, &[0xAA], true); // allow initial write
    let before = cpu.test_read8(bios_addr);
    cpu.test_write8(bios_addr, before.wrapping_add(1));
    let after = cpu.test_read8(bios_addr);
    assert_eq!(before, after, "BIOS write should be ignored (before {before:02X} after {after:02X})");
}
