// Basic tests for Emulator integration

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::emulator::Emulator;

    #[test]
    fn test_emulator_creation() {
        let mut emulator = Emulator::new();

        // Test that we can get references to devices
        let memory_bus = emulator.get_memory_bus();
        assert_eq!(memory_bus.device_count(), 0); // Should have no devices connected yet

        let cpu = emulator.get_cpu();
        // CPU should be initialized (basic smoke test)
        assert_eq!(cpu.registers().pc, 0); // PC starts at 0
    }

    #[test]
    fn test_emulator_init() {
        let mut emulator = Emulator::new();

        // Test that devices are properly connected after init
        // Note: We use a non-existent BIOS file for now (init should handle gracefully)
        emulator.init("non_existent_bios.bin");

        // Memory bus should now have devices connected
        let memory_bus = emulator.get_memory_bus();
        let device_count = memory_bus.device_count();

        // We should have at least RAM, BIOS, Illegal, and Cartridge devices
        // Plus either Dev or Unmapped (depending on DEVELOPER_MODE)
        assert!(
            device_count >= 4,
            "Expected at least 4 devices, got {}",
            device_count
        );
    }

    #[test]
    fn test_emulator_reset() {
        let mut emulator = Emulator::new();
        emulator.init("non_existent_bios.bin");

        // Reset should not panic
        emulator.reset();

        // CPU should be reset
        let cpu = emulator.get_cpu();
        assert_eq!(cpu.registers().pc, 0);
    }

    #[test]
    fn test_memory_devices_connected() {
        let mut emulator = Emulator::new();
        emulator.init("non_existent_bios.bin");

        let memory_bus = emulator.get_memory_bus();
        let device_count = memory_bus.device_count();

        // Verify we have multiple devices
        assert!(device_count > 0, "No devices connected");

        // Verify devices are sorted by address range (C++ behavior)
        for i in 1..device_count {
            let prev = memory_bus.device_info(i - 1).unwrap();
            let curr = memory_bus.device_info(i).unwrap();
            assert!(prev.0 <= curr.0, "Devices not sorted by address range");
        }

        // Test that we can access RAM through memory bus
        let ram = emulator.get_ram();
        unsafe { &mut *ram.get() }.write(0x0000, 0x42);
        let value = unsafe { &*ram.get() }.read(0x0000);
        assert_eq!(value, 0x42, "RAM read/write failed");
    }
}
