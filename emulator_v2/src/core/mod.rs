//! Core emulation logic
//! Port of vectrexy/libs/emulator

pub mod memory_bus;
pub mod cpu6809;
pub mod cpu_helpers;
pub mod cpu_op_codes;
pub mod via6522;
pub mod memory_map;
pub mod ram;
pub mod bios_rom;
pub mod cartridge;
pub mod unmapped_memory_device;
pub mod illegal_memory_device;
pub mod dev_memory_device;
pub mod emulator;
pub mod engine_types;
pub mod delayed_value_store;
pub mod timers;
pub mod psg;
pub mod screen;
pub mod shift_register;

#[cfg(test)]
pub mod emulator_test;

pub use memory_bus::{MemoryBus, MemoryBusDevice, MemoryRange, EnableSync};
pub use cpu6809::{Cpu6809, CpuRegisters, ConditionCode};
pub use cpu_op_codes::AddressingMode;
pub use via6522::{Via6522, Timer1, Timer2, TimerMode};
pub use shift_register::ShiftRegisterMode;
pub use memory_map::*;
pub use ram::Ram;
pub use bios_rom::BiosRom;
pub use cartridge::Cartridge;
pub use unmapped_memory_device::UnmappedMemoryDevice;
pub use illegal_memory_device::IllegalMemoryDevice;
pub use dev_memory_device::DevMemoryDevice;
pub use emulator::Emulator;