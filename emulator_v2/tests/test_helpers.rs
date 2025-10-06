//! Shared test helpers for emulator tests

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::{EnableSync, MemoryBus};
use vectrex_emulator_v2::core::ram::Ram;

/// Setup CPU with RAM connected for testing
/// Returns (CPU, RAM cell) for test manipulation
pub fn setup_cpu_with_ram() -> (Cpu6809, Rc<UnsafeCell<Ram>>) {
    let mut memory_bus = MemoryBus::new();
    let ram = Rc::new(UnsafeCell::new(Ram::new()));

    // Connect RAM to full address space for testing
    memory_bus.connect_device(ram.clone(), (0xC800, 0xFFFF), EnableSync::False);

    let cpu = Cpu6809::new(memory_bus);
    (cpu, ram)
}
