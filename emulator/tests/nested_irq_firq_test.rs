use vectrex_emulator::CPU;

// Escenario: Un IRQ entra en handler que habilita FIRQ (baja F flag) y luego se dispara un FIRQ
// Forzamos ambas interrupciones con los helpers test_force_irq/test_force_firq para aislar frame logic.
// Verificamos:
//  - Tras IRQ: shadow_stack +1 (IRQ)
//  - Tras FIRQ dentro de handler: shadow_stack +2 total (IRQ luego FIRQ en top)
//  - Ejecutar RTI del handler FIRQ devuelve al handler IRQ y shadow_stack vuelve a 1
//  - Ejecutar RTI del handler IRQ devuelve al mainline y shadow_stack vuelve a 0
//  - Los PCs de retorno coinciden exactamente con ret registrados en frames sombra
//  - Frames hardware no se pisan (S restaura al final)
#[test]
fn nested_irq_firq_shadow() {
    let mut cpu = CPU::default();
    cpu.trace = true; // habilitar trazas para ver instrumentación service_firq/RTI
    // Vectores estándar: FIRQ en 0xFFF6 (hi,lo) IRQ en 0xFFF8 (hi,lo)
    // IRQ handler 0x0400
    cpu.bus.mem[0xFFF8] = 0x04; cpu.bus.mem[0xFFF9] = 0x00; // IRQ high, low
    // FIRQ handler 0x0500
    cpu.bus.mem[0xFFF6] = 0x05; cpu.bus.mem[0xFFF7] = 0x00; // FIRQ high, low

    // Mainline PC
    cpu.pc = 0x0100;
    cpu.bus.mem[0x0100] = 0x12; cpu.mem[0x0100] = 0x12; // NOP (placeholder)

    // IRQ handler @0x0400: CLRA ; RTI
    // Simplificamos: disparamos FIRQ tras ejecutar CLRA para observar anidamiento temprano.
    cpu.bus.mem[0x0400] = 0x4F; cpu.mem[0x0400] = 0x4F; // CLRA
    cpu.bus.mem[0x0401] = 0x3B; cpu.mem[0x0401] = 0x3B; // RTI (IRQ)

    // FIRQ handler @0x0500: LDA #$7E ; RTI
    cpu.bus.mem[0x0500] = 0x86; cpu.mem[0x0500] = 0x86; // LDA imm
    cpu.bus.mem[0x0501] = 0x7E; cpu.mem[0x0501] = 0x7E;
    cpu.bus.mem[0x0502] = 0x3B; cpu.mem[0x0502] = 0x3B; // RTI

    let depth0 = cpu.shadow_stack.len();
    let s_initial = cpu.s;

    // Disparar IRQ inicial
    cpu.irq_pending = true;
    cpu.test_force_irq();
    assert_eq!(cpu.pc, 0x0400, "IRQ vector incorrecto");
    assert_eq!(cpu.shadow_stack.len(), depth0 + 1, "Shadow stack no incrementó tras IRQ");
    let irq_frame_ret = cpu.shadow_stack.last().unwrap().ret;
    assert_eq!(irq_frame_ret, 0x0100, "Ret IRQ esperado 0x0100");

    let s_after_irq = cpu.s; // SP tras apilar frame IRQ

    // Ejecutar CLRA
    cpu.step(); // CLRA at 0x0400, PC now 0x0401 (RTI opcode next)
    println!("[TEST] After CLRA pc={:04X}", cpu.pc);
    // Limpiar F (permitir FIRQ) manualmente porque IRQ estableció F=0? (en IRQ sólo se enmascara I). Aseguramos FIRQ permitido limpiando F si estuviera.
    if cpu.cc_f { cpu.cc_f = false; }

    // Disparar FIRQ dentro del handler IRQ
    println!("[TEST] Before FIRQ trigger pc={:04X} S={:04X}", cpu.pc, cpu.s);
    cpu.firq_pending = true;
    cpu.test_force_firq();
    println!("[TEST] After FIRQ service pc={:04X} shadow_depth={} last_ret={:04X}", cpu.pc, cpu.shadow_stack.len(), cpu.shadow_stack.last().unwrap().ret);
    assert_eq!(cpu.pc, 0x0500, "FIRQ vector incorrecto");
    assert_eq!(cpu.shadow_stack.len(), depth0 + 2, "Shadow stack no incrementó tras FIRQ anidado");
    let firq_frame_ret = cpu.shadow_stack.last().unwrap().ret; // prev_pc before vector (expected 0x0401)
    assert_eq!(firq_frame_ret, 0x0401, "Ret FIRQ debería apuntar a RTI del IRQ handler (0x0401)");

    let s_after_firq = cpu.s;
    assert!(s_after_firq < s_after_irq, "FIRQ debería haber empujado frame adicional");

    // Ejecutar LDA #$7E (inmediato). Una sola step() consume opcode + operando inmediato.
    cpu.step(); // LDA #$7E
    assert_eq!(cpu.a, 0x7E, "FIRQ handler no cargó A correctamente");
    assert_eq!(cpu.pc, 0x0502, "Tras LDA inmediato PC debe apuntar al RTI (0x0502)");

    // Dump stack bytes BEFORE ejecutar RTI del FIRQ (PC=0502 apunta a RTI)
    let s_before_rti_firq = cpu.s;
    let mut dump = String::new();
    for off in 0..6 { let addr = s_before_rti_firq.wrapping_add(off); dump.push_str(&format!(" {:02X}", cpu.test_read8(addr))); }
    println!("[TEST] FIRQ pre-RTI (real) S={:04X} bytes:{}", s_before_rti_firq, dump);

    // Ejecutar RTI FIRQ (primer retorno anidado). Esperamos regresar a 0x0401 (RTI del IRQ handler)
    let pc_before_rti_firq = cpu.pc; assert_eq!(pc_before_rti_firq, 0x0502, "PC debería apuntar al RTI del FIRQ (0x0502)");
    cpu.step(); // RTI FIRQ
    println!("[TEST] After RTI FIRQ pc={:04X}", cpu.pc);
    assert_eq!(cpu.pc, 0x0401, "RTI FIRQ no regresó al IRQ handler (RTI opcode en 0x0401)");
    assert_eq!(cpu.shadow_stack.len(), depth0 + 1, "Shadow stack no decrementó tras RTI FIRQ");

    // RTI del IRQ (en 0x0401)
    cpu.step(); // ejecuta RTI IRQ
    assert_eq!(cpu.pc, 0x0100, "RTI IRQ no regresó al mainline");
    assert_eq!(cpu.shadow_stack.len(), depth0, "Shadow stack no quedó vacía tras RTI IRQ final");

    // Sanidad: tras ambos RTI el stack debe volver al valor inicial antes de IRQ
    assert_eq!(cpu.s, s_initial, "SP final tras ambos RTI debería igualar S inicial antes de IRQ");
}
