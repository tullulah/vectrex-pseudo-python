pub mod cpu6809;
pub mod bus;
pub mod via6522;
pub mod wasm_api; // Unified WASM API
pub mod integrator;
pub mod memory_map;
pub mod cycle_table; // added
pub mod opcode_meta; // export metadata subset for tests
pub mod psg_ay; // AY-3-8912 PSG (fase inicial)
pub mod emulator; // Proper emulator architecture

#[cfg(test)]
pub mod extended_test;

pub use cpu6809::CPU;
pub use cpu6809::ILLEGAL_BASE_OPCODES; // re-export para tests de cobertura
pub use bus::Bus;
pub use via6522::Via6522;
pub use integrator::{Integrator, BeamSegment, BeamState};
pub use psg_ay::AyPsg;
pub use memory_map::*;
pub use emulator::{Emulator, EmulatorStats, EmulatorConfig, EmulatorDebugState}; // NEW

// Unified WASM API
#[cfg(feature="wasm")]
pub use wasm_api::{WasmEmu as WasmEmulator};

/// Ejecuta exactamente una instrucción si es posible; devuelve true si avanzó.
/// Wrapper ligero para tests largos que necesitan una interfaz estable.
/// DEPRECATED: Use Emulator::step() instead
pub fn maybe_exec_one(cpu: &mut CPU) -> bool { cpu.step() }
