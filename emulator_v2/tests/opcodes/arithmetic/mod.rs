// Arithmetic opcode tests
// Auto-generated - one file per opcode

use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::{Cpu6809, EnableSync, MemoryBus, Ram};

const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

// Common test helper
pub fn setup_cpu_with_ram() -> (Cpu6809, Rc<UnsafeCell<Ram>>) {
    let mut memory_bus = MemoryBus::new();
    let ram = Rc::new(UnsafeCell::new(Ram::new()));
    memory_bus.connect_device(ram.clone(), (RAM_START, 0xFFFF), EnableSync::False);
    let mut cpu = Cpu6809::new(memory_bus);
    cpu.registers_mut().s = STACK_START;
    (cpu, ram)
}

pub mod test_add;
pub mod test_adda_variants;
pub mod test_addb;
pub mod test_and;
pub mod test_andb;
pub mod test_cmpa_opcodes;
pub mod test_cmpb_opcodes;
pub mod test_cmpd_opcodes;
pub mod test_cmps_opcodes;
pub mod test_cmpu_opcodes;
pub mod test_cmpx_opcodes;
pub mod test_cmpy_opcodes;
pub mod test_eorb;
pub mod test_extended;
pub mod test_ldaa;
pub mod test_logic;
pub mod test_mul;
pub mod test_oraa;
pub mod test_orab;
pub mod test_orb;
pub mod test_register_control_opcodes;
pub mod test_sex;
pub mod test_staa;
pub mod test_subb;
