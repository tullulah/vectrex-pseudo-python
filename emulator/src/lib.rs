pub mod cpu6809;
pub mod bus;
pub mod via6522;
pub mod wasm_api;
pub mod integrator;
pub mod memory_map;
pub mod cycle_table; // added
pub mod opcode_meta; // export metadata subset for tests

pub use cpu6809::CPU;
pub use cpu6809::ILLEGAL_BASE_OPCODES; // re-export para tests de cobertura
pub use bus::Bus;
pub use via6522::Via6522;
pub use integrator::{Integrator, BeamSegment, BeamState};
pub use memory_map::*;

/// Ejecuta exactamente una instrucción si es posible; devuelve true si avanzó.
/// Wrapper ligero para tests largos que necesitan una interfaz estable.
pub fn maybe_exec_one(cpu: &mut CPU) -> bool { cpu.step() }
