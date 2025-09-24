// C++ Original: Test suite for Branch opcodes - 1:1 Vectrexy port

use vectrex_emulator_v2::core::cpu6809::Cpu6809;
use vectrex_emulator_v2::core::memory_bus::MemoryBus;
use vectrex_emulator_v2::core::ram::Ram;
use std::cell::RefCell;
use std::rc::Rc;

fn create_test_cpu() -> Cpu6809 {
    let memory_bus = Rc::new(RefCell::new(MemoryBus::new()));
    
    // Add RAM for test memory using the configured memory map
    let ram = Rc::new(RefCell::new(Ram::new()));
    Ram::init_memory_bus(ram.clone(), &mut memory_bus.borrow_mut());
    
    Cpu6809::new(memory_bus)
}

fn set_flags(cpu: &mut Cpu6809, z: bool, c: bool, n: bool, v: bool) {
    // Helper to set condition code flags for testing
    cpu.registers_mut().cc.z = z;
    cpu.registers_mut().cc.c = c;
    cpu.registers_mut().cc.n = n;
    cpu.registers_mut().cc.v = v;
}

#[test]
fn test_bra_always_branches() {
    // C++ Original: case 0x20: OpBranch([] { return true; });
    let mut cpu = create_test_cpu();
    
    // Setup: BRA +10 (0x20 0x0A)
    cpu.registers_mut().pc = 0xC800;
    
    // Place BRA instruction with +10 offset
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x20); // BRA opcode
    memory_bus.borrow_mut().write(0xC801, 0x0A); // offset +10
    
    let cycles = cpu.execute_instruction(false, false);
    
    // PC should be C802 (after reading instruction) + 10 = C80C
    assert_eq!(cpu.registers().pc, 0xC80C);
    assert_eq!(cycles, 3); // BRA takes 3 cycles
}

#[test]
fn test_brn_never_branches() {
    // C++ Original: case 0x21: OpBranch([] { return false; });
    let mut cpu = create_test_cpu();
    
    // Setup: BRN +10 (0x21 0x0A)
    cpu.registers_mut().pc = 0xC900;
    
    // Place BRN instruction with +10 offset  
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC900, 0x21); // BRN opcode
    memory_bus.borrow_mut().write(0xC901, 0x0A); // offset +10
    
    let cycles = cpu.execute_instruction(false, false);
    
    // PC should be C902 (after reading instruction), no branch
    assert_eq!(cpu.registers().pc, 0xC902);
    assert_eq!(cycles, 3); // BRN takes 3 cycles
}

#[test]
fn test_beq_branches_when_zero() {
    // C++ Original: case 0x27: OpBranch([this] { return CC.Zero; });
    let mut cpu = create_test_cpu();
    
    // Setup: BEQ +5 with Z flag set
    cpu.registers_mut().pc = 0xCA00;
    set_flags(&mut cpu, true, false, false, false); // Z=1
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xCA00, 0x27); // BEQ opcode
    memory_bus.borrow_mut().write(0xCA01, 0x05); // offset +5
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Should branch: PC = CA02 + 5 = CA07
    assert_eq!(cpu.registers().pc, 0xCA07);
    assert_eq!(cycles, 3);
}

#[test]
fn test_beq_no_branch_when_not_zero() {
    // C++ Original: case 0x27: OpBranch([this] { return CC.Zero; });
    let mut cpu = create_test_cpu();
    
    // Setup: BEQ +5 with Z flag clear
    cpu.registers_mut().pc = 0xCB00;
    set_flags(&mut cpu, false, false, false, false); // Z=0
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xCB00, 0x27); // BEQ opcode
    memory_bus.borrow_mut().write(0xCB01, 0x05); // offset +5
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Should not branch: PC = CB02
    assert_eq!(cpu.registers().pc, 0xCB02);
    assert_eq!(cycles, 3);
}

#[test]
fn test_bne_branches_when_not_zero() {
    // C++ Original: case 0x26: OpBranch([this] { return !CC.Zero; });
    let mut cpu = create_test_cpu();
    
    // Setup: BNE +8 with Z flag clear
    cpu.registers_mut().pc = 0xCC00;
    set_flags(&mut cpu, false, false, false, false); // Z=0
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xCC00, 0x26); // BNE opcode
    memory_bus.borrow_mut().write(0xCC01, 0x08); // offset +8
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Should branch: PC = CC02 + 8 = CC0A
    assert_eq!(cpu.registers().pc, 0xCC0A);
    assert_eq!(cycles, 3);
}

#[test]
fn test_bne_no_branch_when_zero() {
    // C++ Original: case 0x26: OpBranch([this] { return !CC.Zero; });
    let mut cpu = create_test_cpu();
    
    // Setup: BNE +8 with Z flag set
    cpu.registers_mut().pc = 0xCD00;
    set_flags(&mut cpu, true, false, false, false); // Z=1
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xCD00, 0x26); // BNE opcode
    memory_bus.borrow_mut().write(0xCD01, 0x08); // offset +8
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Should not branch: PC = CD02
    assert_eq!(cpu.registers().pc, 0xCD02);
    assert_eq!(cycles, 3);
}

#[test]
fn test_bcc_branches_when_carry_clear() {
    // C++ Original: case 0x24: OpBranch([this] { return !CC.Carry; });
    let mut cpu = create_test_cpu();
    
    // Setup: BCC +3 with C flag clear
    cpu.registers_mut().pc = 0xCE00;
    set_flags(&mut cpu, false, false, false, false); // C=0
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xCE00, 0x24); // BCC opcode
    memory_bus.borrow_mut().write(0xCE01, 0x03); // offset +3
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Should branch: PC = 4002 + 3 = 4005
    assert_eq!(cpu.registers().pc, 0xCE05);
    assert_eq!(cycles, 3);
}

#[test]
fn test_bcs_branches_when_carry_set() {
    // C++ Original: case 0x25: OpBranch([this] { return CC.Carry; });
    let mut cpu = create_test_cpu();
    
    // Setup: BCS +6 with C flag set
    cpu.registers_mut().pc = 0xCF00;
    set_flags(&mut cpu, false, true, false, false); // C=1
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xCF00, 0x25); // BCS opcode
    memory_bus.borrow_mut().write(0xCF01, 0x06); // offset +6
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Should branch: PC = 5002 + 6 = 5008
    assert_eq!(cpu.registers().pc, 0xCF08);
    assert_eq!(cycles, 3);
}

#[test]
fn test_bhi_branches_when_unsigned_higher() {
    // C++ Original: case 0x22: OpBranch([this] { return !(CC.Carry || CC.Zero); });
    let mut cpu = create_test_cpu();
    
    // Setup: BHI +4 with both C=0 and Z=0 (unsigned higher condition)
    cpu.registers_mut().pc = 0xCE00;
    set_flags(&mut cpu, false, false, false, false); // Z=0, C=0
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xCE00, 0x22); // BHI opcode
    memory_bus.borrow_mut().write(0xCE01, 0x04); // offset +4
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Should branch: PC = 6002 + 4 = 6006
    assert_eq!(cpu.registers().pc, 0xCE06);
    assert_eq!(cycles, 3);
}

#[test]
fn test_bls_branches_when_unsigned_lower_same() {
    // C++ Original: case 0x23: OpBranch([this] { return CC.Carry || CC.Zero; });
    let mut cpu = create_test_cpu();
    
    // Setup: BLS +7 with C=1 (unsigned lower/same condition)
    cpu.registers_mut().pc = 0xCF00;
    set_flags(&mut cpu, false, true, false, false); // Z=0, C=1
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xCF00, 0x23); // BLS opcode
    memory_bus.borrow_mut().write(0xCF01, 0x07); // offset +7
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Should branch: PC = 7002 + 7 = 7009
    assert_eq!(cpu.registers().pc, 0xCF09);
    assert_eq!(cycles, 3);
}

#[test]
fn test_bge_branches_when_signed_greater_equal() {
    // C++ Original: case 0x2C: OpBranch([this] { return !(CC.Negative ^ CC.Overflow); });
    let mut cpu = create_test_cpu();
    
    // Setup: BGE +2 with N=0, V=0 (signed greater/equal condition)
    cpu.registers_mut().pc = 0xC800;
    set_flags(&mut cpu, false, false, false, false); // N=0, V=0
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC800, 0x2C); // BGE opcode
    memory_bus.borrow_mut().write(0xC801, 0x02); // offset +2
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Should branch: PC = 8002 + 2 = 8004
    assert_eq!(cpu.registers().pc, 0xC804);
    assert_eq!(cycles, 3);
}

#[test]
fn test_blt_branches_when_signed_less() {
    // C++ Original: case 0x2D: OpBranch([this] { return CC.Negative ^ CC.Overflow; });
    let mut cpu = create_test_cpu();
    
    // Setup: BLT +9 with N=1, V=0 (signed less condition)
    cpu.registers_mut().pc = 0xC900;
    set_flags(&mut cpu, false, false, true, false); // N=1, V=0
    
    let memory_bus = cpu.memory_bus().clone();
    memory_bus.borrow_mut().write(0xC900, 0x2D); // BLT opcode
    memory_bus.borrow_mut().write(0xC901, 0x09); // offset +9
    
    let cycles = cpu.execute_instruction(false, false);
    
    // Should branch: PC = 9002 + 9 = 900B
    assert_eq!(cpu.registers().pc, 0xC90B);
    assert_eq!(cycles, 3);
}
