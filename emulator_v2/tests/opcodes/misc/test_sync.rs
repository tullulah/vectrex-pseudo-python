use std::cell::UnsafeCell;
use std::rc::Rc;
use vectrex_emulator_v2::core::{Cpu6809, EnableSync, MemoryBus, MemoryBusDevice, Ram};

const RAM_START: u16 = 0xC800;
const STACK_START: u16 = 0xCFFF;

fn setup_cpu_with_ram() -> (Cpu6809, Rc<UnsafeCell<Ram>>) {
    let mut memory_bus = MemoryBus::new();
    let ram = Rc::new(UnsafeCell::new(Ram::new()));
    memory_bus.connect_device(ram.clone(), (RAM_START, 0xFFFF), EnableSync::False);
    let mut cpu = Cpu6809::new(memory_bus);
    cpu.registers_mut().s = STACK_START;
    (cpu, ram)
}

#[test]
fn test_sync_basic_0x13() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    // Setup inicial - guardar estado de registros
    let original_a = 0x42;
    let original_b = 0x69;
    let original_x = 0x1234;
    let original_s = STACK_START;
    let original_dp = 0x12;

    cpu.registers_mut().a = original_a;
    cpu.registers_mut().b = original_b;
    cpu.registers_mut().x = original_x;
    cpu.registers_mut().s = original_s;
    cpu.registers_mut().dp = original_dp;

    // Configurar condition codes iniciales
    cpu.registers_mut().cc.c = true;
    cpu.registers_mut().cc.z = false;
    cpu.registers_mut().cc.n = true;
    cpu.registers_mut().cc.v = false;

    let original_cc = cpu.registers().cc;

    // Escribir instrucción SYNC: 0x13
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x13); // SYNC
    unsafe { &mut *memory.get() }.write(RAM_START + 0x101, 0x12); // NOP después de SYNC

    // Configurar PC
    let original_pc = RAM_START + 0x100;
    cpu.registers_mut().pc = original_pc;

    // Ejecutar SYNC
    let cycles_before = cpu.get_cycles();
    cpu.execute_instruction(false, false);
    let cycles_after = cpu.get_cycles();

    // Verificar que NO se modificó ningún registro (SYNC solo espera)
    assert_eq!(
        cpu.registers().a,
        original_a,
        "A register should not change"
    );
    assert_eq!(
        cpu.registers().b,
        original_b,
        "B register should not change"
    );
    assert_eq!(
        cpu.registers().x,
        original_x,
        "X register should not change"
    );
    assert_eq!(
        cpu.registers().s,
        original_s,
        "S register should not change (no stack push)"
    );
    assert_eq!(
        cpu.registers().dp,
        original_dp,
        "DP register should not change"
    );

    // Verificar que los condition codes NO cambiaron (SYNC no afecta CC)
    assert_eq!(
        cpu.registers().cc.c,
        original_cc.c,
        "Carry flag should not change"
    );
    assert_eq!(
        cpu.registers().cc.z,
        original_cc.z,
        "Zero flag should not change"
    );
    assert_eq!(
        cpu.registers().cc.n,
        original_cc.n,
        "Negative flag should not change"
    );
    assert_eq!(
        cpu.registers().cc.v,
        original_cc.v,
        "Overflow flag should not change"
    );

    // Verificar que PC avanzó (SYNC es 1 byte, pero debe avanzar a siguiente instrucción)
    assert_eq!(
        cpu.registers().pc,
        original_pc + 1,
        "PC should advance by 1 byte after SYNC"
    );

    // Verificar timing: SYNC consume 2 cycles (según Vectrexy)
    // C++ Original: { 0x13, "SYNC", AddressingMode::Inherent, 2, 1, "Sync. to interrupt" }
    let cycles_used = cycles_after - cycles_before;
    assert_eq!(
        cycles_used, 2,
        "SYNC should consume exactly 2 cycles (Vectrexy), got {}",
        cycles_used
    );
}

#[test]
fn test_sync_with_masked_interrupts_0x13() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    // Configurar interrupciones enmascaradas (I y F flags en CC)
    cpu.registers_mut().cc.i = true; // IRQ masked
    cpu.registers_mut().cc.f = true; // FIRQ masked

    // Escribir SYNC seguido de instrucción de test
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x13); // SYNC
    unsafe { &mut *memory.get() }.write(RAM_START + 0x101, 0x86); // LDA immediate
    unsafe { &mut *memory.get() }.write(RAM_START + 0x102, 0xFF); // #$FF

    cpu.registers_mut().pc = RAM_START + 0x100;
    cpu.registers_mut().a = 0x00;

    // Ejecutar SYNC
    cpu.execute_instruction(false, false);

    // SYNC debe haber completado y PC apuntar a siguiente instrucción
    assert_eq!(
        cpu.registers().pc,
        RAM_START + 0x101,
        "PC should point to next instruction"
    );

    // Ejecutar siguiente instrucción (LDA #$FF) para verificar que SYNC terminó
    cpu.execute_instruction(false, false);
    assert_eq!(
        cpu.registers().a,
        0xFF,
        "Following instruction should execute"
    );
}

#[test]
fn test_sync_preserves_all_state_0x13() {
    let (mut cpu, memory) = setup_cpu_with_ram();
    // Configurar todos los registros con valores únicos
    cpu.registers_mut().a = 0x12;
    cpu.registers_mut().b = 0x34;
    cpu.registers_mut().x = 0x5678;
    cpu.registers_mut().y = 0x9ABC;
    cpu.registers_mut().u = 0xDEF0;
    cpu.registers_mut().s = 0xC900;
    cpu.registers_mut().dp = 0xAB;

    // Configurar todos los condition codes
    cpu.registers_mut().cc.c = true;
    cpu.registers_mut().cc.v = false;
    cpu.registers_mut().cc.z = true;
    cpu.registers_mut().cc.n = false;
    cpu.registers_mut().cc.i = true;
    cpu.registers_mut().cc.h = false;
    cpu.registers_mut().cc.f = true;
    cpu.registers_mut().cc.e = false;

    // Guardar estado completo (clonar antes de mutar)
    let snapshot_a = cpu.registers().a;
    let snapshot_b = cpu.registers().b;
    let snapshot_x = cpu.registers().x;
    let snapshot_y = cpu.registers().y;
    let snapshot_u = cpu.registers().u;
    let snapshot_s = cpu.registers().s;
    let snapshot_dp = cpu.registers().dp;
    let snapshot_cc = cpu.registers().cc;

    // Escribir y ejecutar SYNC
    unsafe { &mut *memory.get() }.write(RAM_START + 0x100, 0x13);
    cpu.registers_mut().pc = RAM_START + 0x100;

    cpu.execute_instruction(false, false);

    // Verificar que TODOS los registros se preservaron
    assert_eq!(cpu.registers().a, snapshot_a, "A preserved");
    assert_eq!(cpu.registers().b, snapshot_b, "B preserved");
    assert_eq!(cpu.registers().x, snapshot_x, "X preserved");
    assert_eq!(cpu.registers().y, snapshot_y, "Y preserved");
    assert_eq!(cpu.registers().u, snapshot_u, "U preserved");
    assert_eq!(cpu.registers().s, snapshot_s, "S preserved");
    assert_eq!(cpu.registers().dp, snapshot_dp, "DP preserved");

    // Verificar que TODOS los condition codes se preservaron
    assert_eq!(cpu.registers().cc.c, snapshot_cc.c, "C flag preserved");
    assert_eq!(cpu.registers().cc.v, snapshot_cc.v, "V flag preserved");
    assert_eq!(cpu.registers().cc.z, snapshot_cc.z, "Z flag preserved");
    assert_eq!(cpu.registers().cc.n, snapshot_cc.n, "N flag preserved");
    assert_eq!(cpu.registers().cc.i, snapshot_cc.i, "I flag preserved");
    assert_eq!(cpu.registers().cc.h, snapshot_cc.h, "H flag preserved");
    assert_eq!(cpu.registers().cc.f, snapshot_cc.f, "F flag preserved");
    assert_eq!(cpu.registers().cc.e, snapshot_cc.e, "E flag preserved");

    // PC debe haber avanzado exactamente 1 byte
    assert_eq!(cpu.registers().pc, RAM_START + 0x101, "PC advanced by 1");
}
