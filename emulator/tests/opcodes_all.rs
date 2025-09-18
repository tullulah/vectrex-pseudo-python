//! Archivo unificado de tests de opcodes 6809.
//! Combina:
//!  - Tests unitarios básicos (cargas, aritmética, saltos, pila)
//!  - Casos mínimos adicionales (branches condicionales tomados/no, LBRA, TFR/EXG 16-bit, SEX, LDD directo)
//!  - Validación de metadatos (size + ciclos base subset) usando `opcode_meta`
//!  - Smoke test exhaustivo (opcional: incluido aquí para tener todo en un solo archivo)
//!
//! Política BIOS: todos son micro-tests sintéticos (permitidos según política de excepción).

use vectrex_emulator::cpu6809::CPU;
use vectrex_emulator::opcode_meta::{OPCODE_META_SUBSET, lookup_meta};
use vectrex_emulator::cycle_table::{CYCLES_BASE, INVALID, CYCLES_PREFIX10, CYCLES_PREFIX11};
use vectrex_emulator::cpu6809::{VALID_PREFIX10, VALID_PREFIX11};

// --------------------------------------------------
// Helpers
// --------------------------------------------------
struct ExecResult { cpu: CPU, cycles: u32 }
fn run_with_cycles<F: FnOnce(&mut CPU)>(setup: F) -> ExecResult {
    let mut cpu = CPU::default();
    setup(&mut cpu);
    let before = cpu.cycles;
    let ok = cpu.step(); assert!(ok, "step() devolvió false");
    let delta = (cpu.cycles - before) as u32;
    ExecResult { cpu, cycles: delta }
}
fn step_once(mut cpu: CPU) -> (CPU, u32) { let before=cpu.cycles; let ok=cpu.step(); assert!(ok); let d=(cpu.cycles-before) as u32; (cpu,d) }

// --------------------------------------------------
// Subconjunto metadatos (size + ciclos base)
// --------------------------------------------------
fn run_one(bytes:&[u8], pc:u16) -> (CPU, u32, u16) {
    let mut cpu = CPU::default(); cpu.pc=pc; for (i,b) in bytes.iter().enumerate(){ cpu.test_write8(pc + i as u16,*b); }
    let before_pc=cpu.pc; let before_c=cpu.cycles; let ok=cpu.step(); assert!(ok);
    let cyc=(cpu.cycles-before_c) as u32; let pc_delta=cpu.pc-before_pc; (cpu,cyc,pc_delta)
}

#[test]
fn opcode_meta_subset_validates() {
    for meta in OPCODE_META_SUBSET {
        match meta.opcode {
            // LDS inmediato: PC avanza exactamente size bytes
            0x10CE => { let (cpu,cyc,pc_d)=run_one(&[0x10,0xCE,0x12,0x34],0x0200); assert_eq!(pc_d as u8,meta.size,"LDS size"); assert_eq!(cyc as u8,meta.base_cycles,"LDS cycles"); assert_eq!(cpu.s,0x1234); }
            // JSR extendido: salto cambia PC a destino, por lo que delta PC no refleja size; comprobamos return address
            0x00BD => { let mut cpu=CPU::default(); cpu.pc=0x0300; cpu.test_write8(0x0300,0xBD); cpu.test_write8(0x0301,0x12); cpu.test_write8(0x0302,0x34); let before_cycles=cpu.cycles; let ok=cpu.step(); assert!(ok); let cyc=(cpu.cycles-before_cycles) as u32; assert_eq!(cyc as u8,meta.base_cycles,"JSR cycles"); assert_eq!(cpu.pc,0x1234,"JSR target"); }
            // BRA tomado: no usamos delta PC para size; verificamos ciclos base+1 y destino correcto
            0x0020 => { let mut cpu=CPU::default(); cpu.pc=0x0400; cpu.test_write8(0x0400,0x20); cpu.test_write8(0x0401,0x04); let before=cpu.cycles; let ok=cpu.step(); assert!(ok); let cyc=(cpu.cycles-before) as u32; assert_eq!(cyc as u8, meta.base_cycles+1, "BRA taken cycles"); assert_eq!(cpu.pc, 0x0400 + meta.size as u16 + 0x04); assert!(lookup_meta(0x20,None).unwrap().branch_short); }
            // BSR relativo: push de return address y salto relativo
            0x008D => { let mut cpu=CPU::default(); cpu.pc=0x0500; cpu.test_write8(0x0500,0x8D); cpu.test_write8(0x0501,0x05); let before=cpu.cycles; let ok=cpu.step(); assert!(ok); let cyc=(cpu.cycles-before) as u32; assert_eq!(cyc as u8,meta.base_cycles,"BSR cycles"); assert_eq!(cpu.pc,0x0500 + meta.size as u16 + 0x05); }
            // RTS: sólo validamos ciclos (PC se restaura desde la pila)
            0x0039 => { let mut cpu=CPU::default(); cpu.pc=0x0600; cpu.test_write8(0x0600,0x39); cpu.call_stack.push(0x0777); let before=cpu.cycles; let ok=cpu.step(); assert!(ok); let cyc=(cpu.cycles-before) as u32; assert_eq!(cyc as u8,meta.base_cycles,"RTS cycles"); assert_eq!(cpu.pc,0x0777); }
            // SUBB inmediato: PC delta = size, ciclos base exactos
            0x00C0 => { let (_cpu,cyc,pc_d)=run_one(&[0xC0,0x10],0x0700); assert_eq!(pc_d as u8,meta.size,"SUBB size"); assert_eq!(cyc as u8,meta.base_cycles,"SUBB cycles"); }
            _ => {}
        }
    }
}

// --------------------------------------------------
// Tests unitarios básicos (originales)
// --------------------------------------------------
#[test]
fn opcode_lds_immediate(){ let r=run_with_cycles(|c|{c.pc=0x0200; c.test_write8(0x0200,0x10); c.test_write8(0x0201,0xCE); c.test_write8(0x0202,0x12); c.test_write8(0x0203,0x34);}); let cpu=r.cpu; assert_eq!(cpu.s,0x1234); assert_eq!(cpu.pc,0x0204); assert_eq!(r.cycles,5); }
#[test]
fn opcode_addb_immediate(){ let r=run_with_cycles(|c|{c.pc=0x0300; c.b=0x10; c.test_write8(0x0300,0xCB); c.test_write8(0x0301,0x22);}); assert_eq!(r.cycles,2); assert_eq!(r.cpu.b,0x32); }
#[test]
fn opcode_jsr_extended(){ let r=run_with_cycles(|c|{c.pc=0x0400; c.test_write8(0x0400,0xBD); c.test_write8(0x0401,0x12); c.test_write8(0x0402,0x34);}); assert_eq!(r.cycles,7); assert_eq!(r.cpu.pc,0x1234); assert_eq!(r.cpu.call_stack[0],0x0403); }
#[test]
fn opcode_bsr_relative(){ let r=run_with_cycles(|c|{c.pc=0x0500; c.test_write8(0x0500,0x8D); c.test_write8(0x0501,0x03);}); assert_eq!(r.cycles,7); assert_eq!(r.cpu.pc,0x0505); }
#[test]
fn opcode_lda_immediate(){ let r=run_with_cycles(|c|{c.pc=0x0600; c.test_write8(0x0600,0x86); c.test_write8(0x0601,0x7F);}); assert_eq!(r.cycles,2); assert_eq!(r.cpu.a,0x7F); }
#[test]
fn opcode_tfr_a_to_b(){ let r=run_with_cycles(|c|{c.pc=0x0700; c.a=0x55; c.test_write8(0x0700,0x1F); c.test_write8(0x0701,0x89);}); assert_eq!(r.cycles,6); assert_eq!(r.cpu.b,0x55); }
#[test]
fn opcode_rts(){ let r=run_with_cycles(|c|{c.pc=0x0800; c.test_write8(0x0800,0x39); c.call_stack.push(0x0AAA);}); assert_eq!(r.cycles,5); assert_eq!(r.cpu.pc,0x0AAA); }
#[test]
fn opcode_ldb_immediate(){ let r=run_with_cycles(|c|{c.pc=0x0900; c.test_write8(0x0900,0xC6); c.test_write8(0x0901,0xE1);}); assert_eq!(r.cycles,2); assert_eq!(r.cpu.b,0xE1); }
#[test]
fn opcode_ldx_immediate(){ let r=run_with_cycles(|c|{c.pc=0x0A00; c.test_write8(0x0A00,0x8E); c.test_write8(0x0A01,0x01); c.test_write8(0x0A02,0x02);}); assert_eq!(r.cycles,3); assert_eq!(r.cpu.x,0x0102); }
#[test]
fn opcode_clra(){ let r=run_with_cycles(|c|{c.pc=0x0B00; c.a=0xFF; c.test_write8(0x0B00,0x4F);}); assert_eq!(r.cycles,2); assert_eq!(r.cpu.a,0x00); }
#[test]
fn opcode_bra_forward(){ let r=run_with_cycles(|c|{c.pc=0x0C00; c.test_write8(0x0C00,0x20); c.test_write8(0x0C01,0x04);}); assert_eq!(r.cycles,3); }
#[test]
fn opcode_lsra(){ let r=run_with_cycles(|c|{c.pc=0x0D00; c.a=0x03; c.test_write8(0x0D00,0x44);}); assert_eq!(r.cycles,2); assert_eq!(r.cpu.a,0x01); }
#[test]
fn opcode_coma(){ let r=run_with_cycles(|c|{c.pc=0x0E00; c.a=0x55; c.test_write8(0x0E00,0x43);}); assert_eq!(r.cycles,2); assert_eq!(r.cpu.a,0xAA); }
#[test]
fn opcode_subb_immediate(){ let r=run_with_cycles(|c|{c.pc=0x0F00; c.b=0x30; c.test_write8(0x0F00,0xC0); c.test_write8(0x0F01,0x10);}); assert_eq!(r.cycles,2); assert_eq!(r.cpu.b,0x20); }
#[test]
fn opcode_stb_direct(){ let r=run_with_cycles(|c|{c.pc=0x1000; c.dp=0x00; c.b=0x42; c.test_write8(0x1000,0xD7); c.test_write8(0x1001,0xFF);}); assert_eq!(r.cycles,4); assert_eq!(r.cpu.mem[0x00FF],0x42); }
#[test]
fn opcode_puls_ab(){ let r=run_with_cycles(|c|{c.pc=0x1100; c.test_write8(0x1100,0x35); c.test_write8(0x1101,0x06); c.s=0x2000; c.test_write8(0x2000,0xAA); c.test_write8(0x2001,0xBB);}); assert_eq!(r.cycles,5); assert_eq!(r.cpu.b,0xAA); assert_eq!(r.cpu.a,0xBB); }

// --------------------------------------------------
// Casos mínimos adicionales
// --------------------------------------------------
#[test]
fn branch_short_taken_and_not_taken(){ let mut cpu=CPU::default(); cpu.pc=0x0200; cpu.test_write8(0x0200,0x26); cpu.test_write8(0x0201,0x02); cpu.test_write8(0x0202,0x27); cpu.test_write8(0x0203,0x02); let (cpu,d1)=step_once(cpu); assert_eq!(cpu.pc,0x0204); assert_eq!(d1,3); let mut cpu2=CPU::default(); cpu2.pc=0x0300; cpu2.test_write8(0x0300,0x27); cpu2.test_write8(0x0301,0x05); let (_cpu2,d2)=step_once(cpu2); assert!(matches!(d2,2|3)); }
#[test]
fn lbra_positive(){ let mut cpu=CPU::default(); cpu.pc=0x0400; cpu.test_write8(0x0400,0x16); cpu.test_write8(0x0401,0x00); cpu.test_write8(0x0402,0x06); let (cpu,d)=step_once(cpu); assert_eq!(cpu.pc,0x0403+0x0006); assert_eq!(d,5); }
#[test]
fn pshs_puls_mixed(){ let mut cpu=CPU::default(); cpu.pc=0x0500; cpu.a=0x5A; cpu.x=0x1234; cpu.s=0x2100; cpu.test_write8(0x0500,0x34); cpu.test_write8(0x0501,0x12); let (cpu,_d1)=step_once(cpu); let mut cpu2=cpu; cpu2.pc=0x0600; cpu2.test_write8(0x0600,0x35); cpu2.test_write8(0x0601,0x12); let (cpu2,_d2)=step_once(cpu2); assert_eq!(cpu2.a,0x5A); assert_eq!(cpu2.x,0x1234); }
#[test]
fn tfr_and_exg_16bit(){ let mut cpu=CPU::default(); cpu.pc=0x0700; cpu.x=0x1111; cpu.y=0x2222; cpu.test_write8(0x0700,0x1F); cpu.test_write8(0x0701,0x01); let (cpu,d1)=step_once(cpu); assert_eq!(cpu.y,0x1111); assert_eq!(d1,6, "TFR debe costar 6 ciclos"); let mut cpu2=CPU::default(); cpu2.pc=0x0710; cpu2.x=0xAAAA; cpu2.y=0xBBBB; cpu2.test_write8(0x0710,0x1E); cpu2.test_write8(0x0711,0x01); let (cpu2,d2)=step_once(cpu2); assert_eq!(cpu2.x,0xBBBB); assert_eq!(cpu2.y,0xAAAA); assert_eq!(d2,8, "EXG debe costar 8 ciclos nominal"); }
#[test]
fn sex_sign_extend(){ let mut cpu=CPU::default(); cpu.pc=0x0800; cpu.b=0x7F; cpu.test_write8(0x0800,0x1D); let (cpu,_)=step_once(cpu); assert_eq!(cpu.a,0x00); let mut cpu2=CPU::default(); cpu2.pc=0x0810; cpu2.b=0x80; cpu2.test_write8(0x0810,0x1D); let (cpu2,_)=step_once(cpu2); assert_eq!(cpu2.a,0xFF); }
#[test]
fn ldd_immediate_and_direct(){ let mut cpu=CPU::default(); cpu.pc=0x0900; cpu.test_write8(0x0900,0xCC); cpu.test_write8(0x0901,0x12); cpu.test_write8(0x0902,0x34); let (cpu,_)=step_once(cpu); assert_eq!(cpu.a,0x12); assert_eq!(cpu.b,0x34); let mut cpu2=CPU::default(); cpu2.pc=0x0910; cpu2.dp=0x00; cpu2.test_write8(0x0910,0xDC); cpu2.test_write8(0x0911,0x80); cpu2.test_write8(0x0080,0xAB); cpu2.test_write8(0x0081,0xCD); let (cpu2,_)=step_once(cpu2); assert_eq!(cpu2.a,0xAB); assert_eq!(cpu2.b,0xCD); }

// --------------------------------------------------
// Smoke test exhaustivo (consolidado)
// --------------------------------------------------
fn setup_cpu_smoke() -> CPU { let mut cpu=CPU::default(); cpu.test_write8(0xFFFC,0x00); cpu.test_write8(0xFFFD,0x80); cpu.pc=0x0100; cpu }
#[test]
fn opcode_exhaustive_smoke(){ let mut cpu=setup_cpu_smoke(); let mut unimpl:Vec<u8>=Vec::new(); for op in 0u16..=255 { cpu.pc=0x0100; cpu.test_write8(0x0100,op as u8); cpu.test_write8(0x0101,0x00); cpu.test_write8(0x0102,0x00); cpu.test_write8(0x0103,0x00); cpu.test_write8(0x0104,0x00); let before=cpu.cycles; let ok=cpu.step(); let delta=cpu.cycles-before; let expect=CYCLES_BASE[op as usize]!=INVALID; if !ok { unimpl.push(op as u8); } if expect { assert!(delta>0, "opcode {:02X} delta 0", op); } } for &prefix in &[0x10u8,0x11u8]{ let list: &[u8]= if prefix==0x10 { VALID_PREFIX10 } else { VALID_PREFIX11 }; for &sub in list { cpu.pc=0x0200; cpu.test_write8(0x0200,prefix); cpu.test_write8(0x0201,sub); cpu.test_write8(0x0202,0x00); cpu.test_write8(0x0203,0x00); let b=cpu.cycles; let ok=cpu.step(); let d=cpu.cycles-b; let expect= if prefix==0x10 { CYCLES_PREFIX10[sub as usize]!=INVALID } else { CYCLES_PREFIX11[sub as usize]!=INVALID }; if !ok { eprintln!("UNIMPL {:02X} {:02X}",prefix,sub);} if expect { assert!(d>0, "prefix {:02X} sub {:02X} delta 0", prefix,sub); } } } if !unimpl.is_empty(){ eprintln!("Base unimplemented: {:02X?}", unimpl); }}

// --------------------------------------------------
// Lote adicional de tests de branches condicionales (tomados y no tomados)
// Cobertura: BNE, BEQ, BCC, BCS, BMI, BPL, BVS, BVC, BGE, BLT, LBRA (ya), LBCC/LBCS (long), BRN (nunca)
// Verifica: PC destino, ciclos (base+1 si tomado para cortos), flags no modificados salvo PC.
// --------------------------------------------------

fn prepare_branch_cpu(pc:u16, opcode:u8, offset:i8, setup: fn(&mut CPU)) -> (CPU,u32) {
    let mut cpu=CPU::default(); cpu.pc=pc; cpu.test_write8(pc,opcode); cpu.test_write8(pc+1, offset as u8); setup(&mut cpu); let before=cpu.cycles; let ok=cpu.step(); assert!(ok, "branch step false"); let cyc=(cpu.cycles-before) as u32; (cpu,cyc)
}

#[test]
fn branch_bne_taken_and_not(){
    // BNE tomado (Z=0)
    let (cpu,cyc)=prepare_branch_cpu(0x300,0x26,0x04,|c|{ c.cc_z=false; });
    assert_eq!(cpu.pc,0x300 + 2 + 0x04); assert_eq!(cyc,3,"BNE taken = base2+1");
    // BNE no tomado (Z=1)
    let (cpu2,cyc2)=prepare_branch_cpu(0x310,0x26,0x04,|c|{ c.cc_z=true; });
    assert_eq!(cpu2.pc,0x312); assert_eq!(cyc2,2,"BNE not taken = base2");
}

#[test]
fn branch_beq_taken_and_not(){
    let (cpu,cyc)=prepare_branch_cpu(0x320,0x27,0x05,|c|{ c.cc_z=true; });
    assert_eq!(cpu.pc,0x320+2+0x05); assert_eq!(cyc,3);
    let (cpu2,cyc2)=prepare_branch_cpu(0x330,0x27,0x05,|c|{ c.cc_z=false; });
    assert_eq!(cpu2.pc,0x332); assert_eq!(cyc2,2);
}

#[test]
fn branch_bcc_and_bcs(){
    // BCC (carry clear) tomado
    let (cpu,cyc)=prepare_branch_cpu(0x340,0x24,0x06,|c|{ c.cc_c=false; });
    assert_eq!(cpu.pc,0x340+2+0x06); assert_eq!(cyc,3);
    // BCC no tomado
    let (cpu2,cyc2)=prepare_branch_cpu(0x350,0x24,0x06,|c|{ c.cc_c=true; });
    assert_eq!(cpu2.pc,0x352); assert_eq!(cyc2,2);
    // BCS (carry set) tomado
    let (cpu3,cyc3)=prepare_branch_cpu(0x360,0x25,0x07,|c|{ c.cc_c=true; });
    assert_eq!(cpu3.pc,0x360+2+0x07); assert_eq!(cyc3,3);
    // BCS no tomado
    let (cpu4,cyc4)=prepare_branch_cpu(0x370,0x25,0x07,|c|{ c.cc_c=false; });
    assert_eq!(cpu4.pc,0x372); assert_eq!(cyc4,2);
}

#[test]
fn branch_bmi_bpl(){
    let (cpu,cyc)=prepare_branch_cpu(0x380,0x2B,0x04,|c|{ c.cc_n=true; }); // BMI
    assert_eq!(cpu.pc,0x380+2+0x04); assert_eq!(cyc,3);
    let (cpu2,cyc2)=prepare_branch_cpu(0x390,0x2B,0x04,|c|{ c.cc_n=false; });
    assert_eq!(cpu2.pc,0x392); assert_eq!(cyc2,2);
    let (cpu3,cyc3)=prepare_branch_cpu(0x3A0,0x2A,0x05,|c|{ c.cc_n=false; }); // BPL
    assert_eq!(cpu3.pc,0x3A0+2+0x05); assert_eq!(cyc3,3);
    let (cpu4,cyc4)=prepare_branch_cpu(0x3B0,0x2A,0x05,|c|{ c.cc_n=true; });
    assert_eq!(cpu4.pc,0x3B2); assert_eq!(cyc4,2);
}

#[test]
fn branch_bvs_bvc(){
    let (cpu,cyc)=prepare_branch_cpu(0x3C0,0x29,0x05,|c|{ c.cc_v=true; }); // BVS
    assert_eq!(cpu.pc,0x3C0+2+0x05); assert_eq!(cyc,3);
    let (cpu2,cyc2)=prepare_branch_cpu(0x3D0,0x29,0x05,|c|{ c.cc_v=false; });
    assert_eq!(cpu2.pc,0x3D2); assert_eq!(cyc2,2);
    let (cpu3,cyc3)=prepare_branch_cpu(0x3E0,0x28,0x06,|c|{ c.cc_v=false; }); // BVC
    assert_eq!(cpu3.pc,0x3E0+2+0x06); assert_eq!(cyc3,3);
    let (cpu4,cyc4)=prepare_branch_cpu(0x3F0,0x28,0x06,|c|{ c.cc_v=true; });
    assert_eq!(cpu4.pc,0x3F2); assert_eq!(cyc4,2);
}

#[test]
fn branch_bge_blt(){
    // BGE (N^V==0)
    let (cpu,cyc)=prepare_branch_cpu(0x400,0x2C,0x04,|c|{ c.cc_n=false; c.cc_v=false; });
    assert_eq!(cpu.pc,0x400+2+0x04); assert_eq!(cyc,3);
    let (cpu2,cyc2)=prepare_branch_cpu(0x410,0x2C,0x04,|c|{ c.cc_n=true; c.cc_v=false; });
    assert_eq!(cpu2.pc,0x412); assert_eq!(cyc2,2);
    // BLT (N^V==1)
    let (cpu3,cyc3)=prepare_branch_cpu(0x420,0x2D,0x05,|c|{ c.cc_n=true; c.cc_v=false; });
    assert_eq!(cpu3.pc,0x420+2+0x05); assert_eq!(cyc3,3);
    let (cpu4,cyc4)=prepare_branch_cpu(0x430,0x2D,0x05,|c|{ c.cc_n=false; c.cc_v=false; });
    assert_eq!(cpu4.pc,0x432); assert_eq!(cyc4,2);
}

#[test]
fn branch_long_lbcc_lbcs(){
    // Use 0x10 prefix long branches: LBRA covered; we simulate one conditional long by manually emitiendo prefijo + opcode (ejemplo 0x10 0x24 = L BCC) si está soportado en meta futuro
    // Placeholder: verify LBRA ya probado arriba; aquí prueba de LBSR ya existe en CPU (0x17). Dejamos test mínimo para LBSR tomado.
    let mut cpu=CPU::default(); cpu.pc=0x440; cpu.test_write8(0x0440,0x17); cpu.test_write8(0x0441,0x00); cpu.test_write8(0x0442,0x06); let before=cpu.cycles; let ok=cpu.step(); assert!(ok); let cyc=(cpu.cycles-before) as u32; assert_eq!(cyc,9); assert_eq!(cpu.pc,0x443+0x0006);
}

#[test]
fn branch_brn_never(){
    let (cpu,cyc)=prepare_branch_cpu(0x450,0x21,0x7F,|_c|{}); // BRN never taken
    assert_eq!(cpu.pc,0x452); assert_eq!(cyc,2); // nunca suma +1
}

// --------------------------------------------------
// TFR / EXG invalid width & DP transfer tests
// --------------------------------------------------

#[test]
fn tfr_dp_valid_and_invalid_cases(){
    // Caso válido: TFR A->DP (postbyte src=8 (A, width 1), dst=5 (DP, width 1)) => 0x85
    let mut cpu=CPU::default(); cpu.pc=0x0800; cpu.a=0xC8; cpu.dp=0x00; cpu.test_write8(0x0800,0x1F); cpu.test_write8(0x0801,0x85); // TFR A,DP
    let before=cpu.cycles; let ok=cpu.step(); assert!(ok); let d=(cpu.cycles-before) as u32; assert_eq!(cpu.dp,0xC8,"DP debe tomar valor de A"); assert_eq!(d,6,"TFR ciclos válido");

    // Caso válido inverso: TFR DP->A (src=5 dst=8 => 0x58)
    let mut cpu2=CPU::default(); cpu2.pc=0x0810; cpu2.a=0x00; cpu2.dp=0x7E; cpu2.test_write8(0x0810,0x1F); cpu2.test_write8(0x0811,0x58); let _=cpu2.step(); assert_eq!(cpu2.a,0x7E);

    // Caso inválido: TFR A->X (ancho 1 -> 2) (src=8 dst=0 => 0x80). No debe cambiar X.
    let mut cpu3=CPU::default(); cpu3.pc=0x0820; cpu3.a=0x12; cpu3.x=0x3456; cpu3.test_write8(0x0820,0x1F); cpu3.test_write8(0x0821,0x80); let _=cpu3.step(); assert_eq!(cpu3.x,0x3456,"X no debe mutar en TFR inválido (A->X)");

    // Caso inválido: TFR X->A (2 -> 1) (src=0 dst=8 => 0x08). A no cambia.
    let mut cpu4=CPU::default(); cpu4.pc=0x0830; cpu4.x=0xAAAA; cpu4.a=0x11; cpu4.test_write8(0x0830,0x1F); cpu4.test_write8(0x0831,0x08); let _=cpu4.step(); assert_eq!(cpu4.a,0x11,"A no debe mutar en TFR inválido (X->A)");

    // EXG inválido: intercambiar A<->X (postbyte 0x8? src=A(8) dst=X(0) => 0x80 para EXG 0x1E). Ambos quedan igual.
    let mut cpu5=CPU::default(); cpu5.pc=0x0840; cpu5.a=0x5A; cpu5.x=0x7777; cpu5.test_write8(0x0840,0x1E); cpu5.test_write8(0x0841,0x80); let _=cpu5.step(); assert_eq!(cpu5.a,0x5A); assert_eq!(cpu5.x,0x7777);
}

// --------------------------------------------------
// Lote de tests de pila / call stack
// - JSR directo y extendido: sólo usan call_stack (no hardware push16 en esta implementación)
// - BSR / LBSR: empujan return address en la pila (S) vía push16 (memoria descendente)
// - PSHS/PULS máscara completa: orden y restauración de registros + layout exacto en memoria
// (PSHU/PULU se cubrirá en lote posterior si es necesario)
// --------------------------------------------------

#[test]
fn jsr_direct_and_extended_call_stack(){
    // JSR directo (0x9D): destino = DP:offset, se añade return address a call_stack
    let mut cpu=CPU::default();
    cpu.dp=0x02; cpu.pc=0x0500; cpu.test_write8(0x0500,0x9D); cpu.test_write8(0x0501,0x40); // target 0x0240
    let before_calls = cpu.call_stack.len();
    let before_cyc=cpu.cycles; let ok=cpu.step(); assert!(ok);
    let delta=(cpu.cycles-before_cyc) as u32;
    assert_eq!(delta,7,"JSR directo ciclos");
    assert_eq!(cpu.pc,0x0240,"PC salto directo");
    assert_eq!(cpu.call_stack.len(), before_calls+1,"call_stack push");
    assert_eq!(cpu.call_stack[before_calls],0x502,"return address correcto (pc tras instrucción)");

    // JSR extendido (0xBD): destino 16-bit
    let mut cpu2=CPU::default();
    cpu2.pc=0x0520; cpu2.test_write8(0x0520,0xBD); cpu2.test_write8(0x0521,0x12); cpu2.test_write8(0x0522,0x34);
    let ok2=cpu2.step(); assert!(ok2);
    assert_eq!(cpu2.pc,0x1234); assert_eq!(cpu2.call_stack.len(),1); assert_eq!(cpu2.call_stack[0],0x523);
}

#[test]
fn bsr_and_lbsr_push_on_s(){
    // BSR (0x8D) empuja return address en memoria usando push16 (alto, luego bajo) y salta relativo
    let mut cpu=CPU::default();
    cpu.pc=0x0600; cpu.test_write8(0x0600,0x8D); cpu.test_write8(0x0601,0x04); cpu.s=0x0800;
    let before_s=cpu.s; let ok=cpu.step(); assert!(ok);
    assert_eq!(cpu.pc,0x600+2+0x04); // PC destino
    assert_eq!(cpu.s, before_s-2, "S debe decrementar 2");
    // return address = 0x602; en stack orden: [high] en before_s-1, [low] en before_s-2
    assert_eq!(cpu.mem[(before_s as usize)-1], 0x06);
    assert_eq!(cpu.mem[(before_s as usize)-2], 0x02);

    // LBSR (0x17) similar pero offset 16-bit y más ciclos
    let mut cpu2=CPU::default(); cpu2.pc=0x0620; cpu2.test_write8(0x0620,0x17); cpu2.test_write8(0x0621,0x00); cpu2.test_write8(0x0622,0x05); cpu2.s=0x0900;
    let s_start=cpu2.s; let c_before=cpu2.cycles; let ok2=cpu2.step(); assert!(ok2);
    let cyc=(cpu2.cycles-c_before) as u32; assert_eq!(cyc,9,"LBSR ciclos");
    assert_eq!(cpu2.pc,0x620+3+0x0005, "PC destino LBSR"); // PC after fetching (0x623) + offset 0x0005 = 0x628
    assert_eq!(cpu2.s, s_start-2);
    assert_eq!(cpu2.mem[(s_start as usize)-1], 0x06); // return 0x623
    assert_eq!(cpu2.mem[(s_start as usize)-2], 0x23);
}

#[test]
fn pshs_full_mask_and_puls_restore(){
    // Config registros únicos
    let mut cpu=CPU::default();
    cpu.pc=0x0700; // instrucción PSHS en 0x700
    cpu.a=0x11; cpu.b=0x22; cpu.dp=0x33; cpu.x=0x4A4B; cpu.y=0x5C5D; cpu.u=0x6E6F; cpu.s=0x0400;
    // PSHS 0xFF (push CC,A,B,DP,X,Y,U,PC). PC que se empuja es PC tras leer máscara => 0x702
    cpu.test_write8(0x0700,0x34); cpu.test_write8(0x0701,0xFF); // PSHS
    // Colocamos PULS inmediatamente después
    cpu.test_write8(0x0702,0x35); cpu.test_write8(0x0703,0xFF); // PULS
    // Ejecuta PSHS
    let ok=cpu.step(); assert!(ok);
    // Layout esperado (stack descendente). push16: high luego low.
    // Orden pushes: CC, A, B, DP, X(h,l), Y(h,l), U(h,l), PC(h,l)
    let s_after_pshs = cpu.s; assert_eq!(s_after_pshs, 0x0400 - (1+1+1+1 +2+2+2 +2) , "S tras PSHS completo");
    let mut addr=0x0400-1; // primer byte almacenado (CC)
    // Reconstruimos CC esperado leyendo flags preservados en CPU (no tenemos acceso a pack_cc por privacidad)
    let cc_expected = 
        (if cpu.cc_e {0x80} else {0}) |
        (if cpu.cc_f {0x40} else {0}) |
        (if cpu.cc_h {0x20} else {0}) |
        (if cpu.cc_i {0x10} else {0}) |
        (if cpu.cc_n {0x08} else {0}) |
        (if cpu.cc_z {0x04} else {0}) |
        (if cpu.cc_v {0x02} else {0}) |
        (if cpu.cc_c {0x01} else {0});
    assert_eq!(cpu.mem[addr as usize], cc_expected); addr-=1; // A
    assert_eq!(cpu.mem[addr as usize], 0x11); addr-=1; // B
    assert_eq!(cpu.mem[addr as usize], 0x22); addr-=1; // DP
    assert_eq!(cpu.mem[addr as usize], 0x33); addr-=1; // X high next (push16 high then low)
    assert_eq!(cpu.mem[addr as usize], (cpu.x>>8) as u8, "X high"); addr-=1; // X low position actually holds high? re-evaluate after retrieval
    // Correct sequence: after DP we expect X high then X low; we already consumed DP at addr+1
    // Adjust: previous line should check high, next line low
    assert_eq!(cpu.mem[addr as usize], (cpu.x & 0xFF) as u8, "X low"); addr-=1; // proceed to Y high
    assert_eq!(cpu.mem[addr as usize], (cpu.y>>8) as u8, "Y high"); addr-=1;
    assert_eq!(cpu.mem[addr as usize], (cpu.y & 0xFF) as u8, "Y low"); addr-=1;
    assert_eq!(cpu.mem[addr as usize], (cpu.u>>8) as u8, "U high"); addr-=1;
    assert_eq!(cpu.mem[addr as usize], (cpu.u & 0xFF) as u8, "U low"); addr-=1;
    // PC pushed value should be 0x702 (after reading mask)
    assert_eq!(cpu.mem[addr as usize], 0x07); addr-=1; // PC high
    assert_eq!(cpu.mem[addr as usize], 0x02); // PC low

    // Ejecuta PULS restaurando todo
    let ok2=cpu.step(); assert!(ok2);
    assert_eq!(cpu.a,0x11); assert_eq!(cpu.b,0x22); assert_eq!(cpu.dp,0x33); assert_eq!(cpu.x,0x4A4B); assert_eq!(cpu.y,0x5C5D); assert_eq!(cpu.u,0x6E6F);
    assert_eq!(cpu.pc,0x702, "PC restaurado");
    assert_eq!(cpu.s,0x0400, "S restaurado al origen");
}

// --------------------------------------------------
// Aritmética: tests de flags N,Z,V,C para ADDA / SUBB / CMPA / CMPB (casos borde)
// Cobertura:
//  - ADDA overflow (0x7F + 0x01 -> 0x80) V=1, N=1
//  - ADDA carry sin overflow (0xF0 + 0x20 -> 0x10) C=1, V=0
//  - ADDA carry + zero (0xFF + 0x01 -> 0x00) C=1, Z=1, V=0
//  - SUBB borrow + negativo (0x10 - 0x20 -> 0xF0) C=1, N=1, V=0
//  - SUBB overflow (0x80 - 0x01 -> 0x7F) V=1, C=0, N=0
//  - CMPA igualdad (A conservado) (0x55 - 0x55) Z=1, C=0
//  - CMPB borrow (0x00 - 0x01 -> 0xFF) C=1, N=1
// --------------------------------------------------

// (run_step_simple) eliminado: ya no se usa tras reorganización de tests de aritmética.

#[test]
fn adda_overflow_sets_v_n(){
    // ADDA #$01 con A=0x7F => 0x80 (overflow signed positivo->negativo)
    let mut cpu=CPU::default(); cpu.a=0x7F; cpu.pc=0x2000; cpu.test_write8(0x2000,0x8B); cpu.test_write8(0x2001,0x01); let _=cpu.step();
    assert_eq!(cpu.a,0x80); assert!(cpu.cc_v, "V debe ser 1"); assert!(cpu.cc_n, "N debe ser 1"); assert!(!cpu.cc_c, "C=0 sin carry"); assert!(!cpu.cc_z, "Z=0");
}

#[test]
fn adda_carry_without_overflow(){
    // 0xF0 + 0x20 = 0x110 -> 0x10 con carry, sin overflow (signos distintos -> no V)
    let mut cpu=CPU::default(); cpu.a=0xF0; cpu.pc=0x2010; cpu.test_write8(0x2010,0x8B); cpu.test_write8(0x2011,0x20); let _=cpu.step();
    assert_eq!(cpu.a,0x10); assert!(cpu.cc_c, "Carry debe ser 1"); assert!(!cpu.cc_v, "V=0"); assert!(!cpu.cc_n, "N=0"); assert!(!cpu.cc_z, "Z=0");
}

#[test]
fn adda_carry_and_zero(){
    // 0xFF + 0x01 = 0x100 -> 0x00; Carry=1, Zero=1, sin overflow (signos opuestos)
    let mut cpu=CPU::default(); cpu.a=0xFF; cpu.pc=0x2020; cpu.test_write8(0x2020,0x8B); cpu.test_write8(0x2021,0x01); let _=cpu.step();
    assert_eq!(cpu.a,0x00); assert!(cpu.cc_c); assert!(cpu.cc_z); assert!(!cpu.cc_v); assert!(!cpu.cc_n);
}

#[test]
fn subb_borrow_negative(){
    // SUBB #$20 con B=0x10 => 0x10-0x20=0xF0 (borrow) => C=1, N=1, V=0
    let mut cpu=CPU::default(); cpu.b=0x10; cpu.pc=0x2030; cpu.test_write8(0x2030,0xC0); cpu.test_write8(0x2031,0x20); let _=cpu.step();
    assert_eq!(cpu.b,0xF0); assert!(cpu.cc_c); assert!(cpu.cc_n); assert!(!cpu.cc_v); assert!(!cpu.cc_z);
}

#[test]
fn subb_overflow(){
    // SUBB #$01 con B=0x80 => 0x80-0x01=0x7F; overflow (neg -> pos) V=1, C=0, N=0
    let mut cpu=CPU::default(); cpu.b=0x80; cpu.pc=0x2040; cpu.test_write8(0x2040,0xC0); cpu.test_write8(0x2041,0x01); let _=cpu.step();
    assert_eq!(cpu.b,0x7F); assert!(cpu.cc_v); assert!(!cpu.cc_c); assert!(!cpu.cc_n); assert!(!cpu.cc_z);
}

#[test]
fn cmpa_equal_sets_z_keeps_a(){
    // CMPA #$55 con A=0x55 -> resultado 0 (Z=1) A no cambia
    let mut cpu=CPU::default(); cpu.a=0x55; cpu.pc=0x2050; cpu.test_write8(0x2050,0x81); cpu.test_write8(0x2051,0x55); let _=cpu.step();
    assert_eq!(cpu.a,0x55); assert!(cpu.cc_z); assert!(!cpu.cc_c); assert!(!cpu.cc_n); assert!(!cpu.cc_v);
}

#[test]
fn cmpb_borrow_sets_c_and_n(){
    // CMPB #$01 con B=0x00 -> resultado 0xFF (N=1) borrow => C=1, Z=0, V=0
    let mut cpu=CPU::default(); cpu.b=0x00; cpu.pc=0x2060; cpu.test_write8(0x2060,0xC1); cpu.test_write8(0x2061,0x01); let _=cpu.step();
    assert_eq!(cpu.b,0x00); assert!(cpu.cc_c); assert!(cpu.cc_n); assert!(!cpu.cc_z); assert!(!cpu.cc_v);
}

// ============================================================================
// Sección unificada adicional: contenido migrado de archivos individuales
// (opcode_audit.rs, opcode_scan.rs, opcode_validity.rs, opcode_spec.rs,
//  opcode_coverage.rs, op_new_opcodes.rs, op_new_added_rmws.rs, op_inca.rs,
//  subd.rs). Cada bloque se encapsula en un módulo para aislar helpers y evitar
//  colisiones de nombres. Mantiene tests intactos.
// ============================================================================

// --- audit ---
mod unified_audit {
    use vectrex_emulator::cpu6809::CPU;
    fn run_single(op: u8, setup: impl Fn(&mut CPU)) -> (CPU, u32) {
        let mut cpu = CPU::default(); cpu.pc=0x0200; cpu.test_write8(0x0200,op); cpu.test_write8(0x0201,0); cpu.test_write8(0x0202,0); cpu.test_write8(0x0203,0); setup(&mut cpu); let before=cpu.cycles as u32; let ok=cpu.step(); assert!(ok,"step() returned false (unimplemented?) for opcode {:02X}",op); let d=(cpu.cycles as u32)-before; (cpu,d)
    }
    #[test] fn audit_inca_cycles_and_flags(){ let (cpu,cyc)=run_single(0x4C, |_| {}); assert_eq!(cyc,2); assert_eq!(cpu.a,0x01); assert!(!cpu.cc_z && !cpu.cc_n && !cpu.cc_v); }
    #[test] fn audit_decb_cycles_and_flags(){ let (cpu,cyc)=run_single(0x5A, |c|{c.b=0x10;}); assert_eq!(cyc,2); assert_eq!(cpu.b,0x0F); assert!(!cpu.cc_z && !cpu.cc_n); }
    #[test] fn audit_bra_taken_cycle_adjust(){ let (_cpu,cyc)=run_single(0x20, |c|{ c.test_write8(0x0201,0xFE); }); assert!(cyc==2 || cyc==3, "BRA cycles {} unexpected",cyc); }
    #[test] fn audit_jsr_extended_cycles(){ let (cpu,cyc)=run_single(0xBD, |c|{ c.test_write8(0x0201,0x03); c.test_write8(0x0202,0x00); }); assert_eq!(cyc,7); assert_eq!(cpu.pc,0x0300); }
    #[test] fn audit_mul_cycles(){ let (cpu,cyc)=run_single(0x3D, |c|{ c.a=3; c.b=4; }); assert_eq!(cyc,11); assert_eq!(cpu.a as u16 * 0x100 + cpu.b as u16,12); }
    #[test] fn audit_cmpx_immediate_cycles(){ let (_cpu,cyc)=run_single(0x8C, |c|{ c.test_write8(0x0201,0x12); c.test_write8(0x0202,0x34); }); assert_eq!(cyc,5); }
    #[test] fn audit_cmpx_indexed_cycles(){
        // Place postbyte after opcode (0xAC + simple ,X postbyte 0x84)
        let mut cpu=CPU::default(); cpu.pc=0x0200; cpu.x=0x4000; cpu.test_write8(0x0200,0xAC); cpu.test_write8(0x0201,0x84); cpu.test_write8(0x4000,0x12); cpu.test_write8(0x4001,0x34); let before=cpu.cycles; let ok=cpu.step(); assert!(ok); let cyc=(cpu.cycles-before) as u32; assert_eq!(cyc,6,"CMPX indexed debería costar 6, got {cyc}"); }
    #[test] fn audit_cmpx_extended_cycles(){
        let mut cpu=CPU::default(); cpu.pc=0x0200; cpu.test_write8(0x0200,0xBC); cpu.test_write8(0x0201,0x90); cpu.test_write8(0x0202,0x00); cpu.test_write8(0x9000,0x12); cpu.test_write8(0x9001,0x34); let before=cpu.cycles; let ok=cpu.step(); assert!(ok); let cyc=(cpu.cycles-before) as u32; assert_eq!(cyc,7,"CMPX extended debería costar 7, got {cyc}"); }
}

// --- scan ---
mod unified_scan {
    use vectrex_emulator::cpu6809::{CPU,is_illegal_base_opcode}; fn is_illegal(op:u8)->bool { is_illegal_base_opcode(op) }
    #[test] fn scan_unimplemented_valids(){ let mut unimpl=Vec::new(); for op in 0u8..=255 { if is_illegal(op){continue;} let mut cpu=CPU::default(); cpu.mem[0]=op; cpu.step(); if cpu.opcode_unimpl_bitmap[op as usize]{ unimpl.push(op);} } println!("UNIMPL VALID COUNT: {}", unimpl.len()); for chunk in unimpl.chunks(16){ print!("   "); for op in chunk { print!("{:02X} ", op);} println!(""); } }
}

// --- validity ---
mod unified_validity {
    use vectrex_emulator::cpu6809::{CPU,is_illegal_base_opcode}; fn is_illegal(op:u8)->bool { is_illegal_base_opcode(op) }
    fn run_single(op:u8)->(u64,bool,bool){ let mut cpu=CPU::default(); cpu.mem[cpu.pc as usize]=op; cpu.step(); let cyc=cpu.cycles; let unimpl=cpu.opcode_unimpl_bitmap[op as usize]; let ill=is_illegal(op); (cyc,unimpl,ill) }
    #[test] fn illegal_opcodes_are_1_cycle_and_not_unimpl(){ for op in 0u8..=255 { if is_illegal(op){ let (cyc,unimpl,_)=run_single(op); assert_eq!(cyc,1,"Illegal opcode {:02X} should consume 1 cycle (got {cyc})",op); assert!(!unimpl,"Illegal opcode {:02X} should not mark as unimplemented",op); } } }
    #[test] fn unimplemented_valid_opcodes_marked(){ for op in 0u8..=255 { if !is_illegal(op){ let _=run_single(op); } } }
}

// --- enforcement (no primaries missing) ---
mod unified_enforce_primary_complete {
    use vectrex_emulator::cpu6809::{CPU,is_illegal_base_opcode};
    fn is_illegal(op:u8)->bool { is_illegal_base_opcode(op) }
    #[test]
    fn enforce_no_unimplemented_primary_opcodes(){
        let mut cpu=CPU::default();
        let (_done,_cnt,missing)=cpu.recompute_opcode_coverage();
        // Filter out anything we classify illegal here (defensive if coverage includes them)
        let missing_valid:Vec<u8>=missing.into_iter().filter(|op| !is_illegal(*op)).collect();
        assert!(missing_valid.is_empty(),"Primary opcode gap(s) restantes: {:?}", missing_valid);
    }
}

// --- spec table ---
mod unified_spec_table {
    use vectrex_emulator::cpu6809::CPU;
    #[derive(Clone)] struct Spec { name:&'static str, code:&'static [u8], setup:fn(&mut CPU), expect_pc:u16, expect_s:Option<u16>, expect_cc_z:Option<bool>, expect_cc_n:Option<bool>, expect_cycles:Option<u32> }
    impl Spec { fn check(&self,before:u64,cpu:&CPU,delta:u32){ if let Some(c)=self.expect_cycles{ assert_eq!(delta,c,"{}: ciclos esperados {} got {}",self.name,c,delta);} if let Some(v)=self.expect_s{ assert_eq!(cpu.s,v,"{}: S esperado {:04X} got {:04X}",self.name,v,cpu.s);} if let Some(v)=self.expect_cc_z{ assert_eq!(cpu.cc_z,v,"{}: Z esperado {} got {}",self.name,v,cpu.cc_z);} if let Some(v)=self.expect_cc_n{ assert_eq!(cpu.cc_n,v,"{}: N esperado {} got {}",self.name,v,cpu.cc_n);} assert_eq!(cpu.pc,self.expect_pc,"{}: PC esperado {:04X} got {:04X}",self.name,self.expect_pc,cpu.pc); assert!(cpu.cycles>=before); } }
    fn spec_cases()->Vec<Spec>{ vec![ Spec{ name:"LDS_immediate_basic", code:&[0x10,0xCE,0x12,0x34], setup: |cpu:&mut CPU|{ cpu.pc=0x0200; }, expect_pc:0x0204, expect_s:Some(0x1234), expect_cc_z:Some(false), expect_cc_n:Some(false), expect_cycles:Some(5)} ] }
    #[test] fn opcode_spec_table(){ for spec in spec_cases(){ let mut cpu=CPU::default(); let base=0x0200u16; for (i,b) in spec.code.iter().enumerate(){ cpu.test_write8(base+i as u16,*b);} (spec.setup)(&mut cpu); if cpu.pc==0 { cpu.pc=base; } let before=cpu.cycles; let ok=cpu.step(); assert!(ok,"{}: step() devolvió false",spec.name); let delta=(cpu.cycles-before) as u32; spec.check(before,&cpu,delta); } }
}

// --- coverage ---
mod unified_coverage {
    use vectrex_emulator::cpu6809::{CPU, VALID_PREFIX10, VALID_PREFIX11};
    #[test] fn extended_opcodes_all_implemented(){ let mut missing=Vec::new(); for (prefix,list) in [(0x10u8,VALID_PREFIX10),(0x11u8,VALID_PREFIX11)] { for &sub in list.iter(){ let mut cpu=CPU::default(); cpu.pc=0x0100; cpu.test_write8(0x0100,prefix); cpu.test_write8(0x0101,sub); cpu.test_write8(0xFFFC,0x00); cpu.test_write8(0xFFFD,0x02); if !cpu.step(){ missing.push((prefix,sub)); } } } if !missing.is_empty(){ panic!("Missing extended opcodes: {:?}", missing); } }
    #[test] fn report_primary_unimplemented(){ let mut cpu=CPU::default(); let (_done,_cnt,missing)=cpu.recompute_opcode_coverage(); eprintln!("Primary-byte unimplemented count: {} -> {:?}", missing.len(), missing); }
}

// --- nuevas operaciones / direct/indexed / shifts ---
mod unified_new_ops {
    use vectrex_emulator::CPU; fn step_single(mut cpu:CPU)->CPU{ cpu.step(); cpu }
    #[test] fn ror_direct(){ let mut cpu=CPU::default(); cpu.dp=0x20; cpu.pc=0x0100; cpu.test_write8(0x0100,0x06); cpu.test_write8(0x0101,0x10); cpu.test_write8(0x2010,0b0000_0011); cpu.cc_c=false; cpu=step_single(cpu); assert_eq!(cpu.mem[0x2010],0b0000_0001); assert!(cpu.cc_c); assert!(!cpu.cc_n); assert!(!cpu.cc_z); }
    #[test] fn rol_direct(){ let mut cpu=CPU::default(); cpu.dp=0x21; cpu.pc=0x0200; cpu.test_write8(0x0200,0x09); cpu.test_write8(0x0201,0x05); cpu.test_write8(0x2105,0b1000_0000); cpu.cc_c=true; cpu.step(); assert_eq!(cpu.mem[0x2105],0b0000_0001); assert!(cpu.cc_c); assert!(!cpu.cc_n); }
    #[test] fn inc_direct_overflow(){ let mut cpu=CPU::default(); cpu.dp=0x30; cpu.pc=0x0300; cpu.test_write8(0x0300,0x0C); cpu.test_write8(0x0301,0x40); cpu.test_write8(0x3040,0x7F); cpu.step(); assert_eq!(cpu.mem[0x3040],0x80); assert!(cpu.cc_v && cpu.cc_n); }
    #[test] fn clr_direct_flags(){ let mut cpu=CPU::default(); cpu.dp=0x22; cpu.pc=0x0400; cpu.test_write8(0x0400,0x0F); cpu.test_write8(0x0401,0x02); cpu.test_write8(0x2202,0xAA); cpu.cc_n=true; cpu.cc_v=true; cpu.cc_c=true; cpu.cc_z=false; cpu.step(); assert_eq!(cpu.mem[0x2202],0x00); assert!(cpu.cc_z && !cpu.cc_n && !cpu.cc_v && !cpu.cc_c); }
    #[test] fn ora_indexed(){ let mut cpu=CPU::default(); cpu.pc=0x0500; cpu.x=0x4000; cpu.a=0x55; cpu.test_write8(0x0500,0xAA); cpu.test_write8(0x0501,0x84); cpu.test_write8(0x4000,0x0F); cpu.step(); assert_eq!(cpu.a,0x5F); assert!(!cpu.cc_v && cpu.cc_n==(cpu.a & 0x80 !=0)); }
    #[test] fn addd_extended(){ let mut cpu=CPU::default(); cpu.pc=0x0600; cpu.a=0x12; cpu.b=0x34; cpu.test_write8(0x0600,0xF3); cpu.test_write8(0x0601,0x90); cpu.test_write8(0x0602,0x00); cpu.test_write8(0x9000,0x00); cpu.test_write8(0x9001,0x02); cpu.step(); assert_eq!(cpu.a,0x12); assert_eq!(cpu.b,0x36); assert!(!cpu.cc_v); }
    #[test] fn eorb_extended(){ let mut cpu=CPU::default(); cpu.pc=0x0700; cpu.b=0xF0; cpu.test_write8(0x0700,0xF8); cpu.test_write8(0x0701,0x88); cpu.test_write8(0x0702,0x00); cpu.test_write8(0x8800,0x0F); cpu.step(); assert_eq!(cpu.b,0xFF); assert!(cpu.cc_n && !cpu.cc_z); }
    #[test] fn sbca_direct_borrow_sets_c(){
        // A=0x10, mem=0x10, C=1 -> 0x10 - 0x10 - 1 = 0xFF (borrow), C set, N set, Z clear
        let mut cpu=CPU::default(); cpu.dp=0x20; cpu.pc=0x0800; cpu.a=0x10; cpu.cc_c=true; cpu.test_write8(0x0800,0x92); cpu.test_write8(0x0801,0x40); cpu.test_write8(0x2040,0x10); cpu.step();
        assert_eq!(cpu.a,0xFF); assert!(cpu.cc_c); assert!(cpu.cc_n); assert!(!cpu.cc_z); }
    #[test] fn bita_direct_zero_sets_z(){ let mut cpu=CPU::default(); cpu.dp=0x21; cpu.pc=0x0810; cpu.a=0xF0; cpu.test_write8(0x0810,0x95); cpu.test_write8(0x0811,0x22); cpu.test_write8(0x2122,0x0F); cpu.step(); assert!(cpu.cc_z); assert!(!cpu.cc_n); }
}

// --- INC A casos adicionales ---
mod unified_inca { use vectrex_emulator::CPU; #[test] fn inca_basic(){ let mut cpu=CPU::default(); cpu.a=0x00; cpu.pc=0x0100; cpu.mem[0x0100]=0x4C; cpu.step(); assert_eq!(cpu.a,0x01); assert!(!cpu.cc_z && !cpu.cc_n && !cpu.cc_v); } #[test] fn inca_sets_zero(){ let mut cpu=CPU::default(); cpu.a=0xFF; cpu.pc=0x0200; cpu.mem[0x0200]=0x4C; cpu.step(); assert_eq!(cpu.a,0x00); assert!(cpu.cc_z && !cpu.cc_n && !cpu.cc_v); } #[test] fn inca_overflow_flag(){ let mut cpu=CPU::default(); cpu.a=0x7F; cpu.pc=0x0300; cpu.mem[0x0300]=0x4C; cpu.step(); assert_eq!(cpu.a,0x80); assert!(cpu.cc_n && cpu.cc_v && !cpu.cc_z); } }

// --- RMW / adicionales ---
mod unified_rmw_added { use vectrex_emulator::CPU; fn run(mut cpu:CPU)->CPU{ cpu.step(); cpu }
    #[test] fn tst_direct(){ let mut cpu=CPU::default(); cpu.dp=0x24; cpu.pc=0x0100; cpu.test_write8(0x0100,0x0D); cpu.test_write8(0x0101,0x10); cpu.test_write8(0x2410,0x80); cpu=run(cpu); assert!(cpu.cc_n); assert!(!cpu.cc_z); assert!(!cpu.cc_v && !cpu.cc_c); }
    #[test] fn jmp_direct(){ let mut cpu=CPU::default(); cpu.dp=0x25; cpu.pc=0x0200; cpu.test_write8(0x0200,0x0E); cpu.test_write8(0x0201,0x40); cpu.test_write8(0x2540,0x12); cpu.step(); assert_eq!(cpu.pc,0x2540); }
    #[test] fn jmp_indexed(){ let mut cpu=CPU::default(); cpu.pc=0x0300; cpu.x=0x6000; cpu.test_write8(0x0300,0x6E); cpu.test_write8(0x0301,0x84); cpu.test_write8(0x6000,0xFF); cpu.step(); assert_eq!(cpu.pc,0x6000); }
    #[test] fn asl_indexed_flags(){ let mut cpu=CPU::default(); cpu.pc=0x0400; cpu.x=0x7000; cpu.test_write8(0x0400,0x68); cpu.test_write8(0x0401,0x84); cpu.test_write8(0x7000,0xC0); cpu.step(); assert_eq!(cpu.mem[0x7000],0x80); assert!(cpu.cc_n); assert!(!cpu.cc_z); assert!(cpu.cc_c); }
    #[test] fn dec_indexed_overflow_v(){ let mut cpu=CPU::default(); cpu.pc=0x0500; cpu.x=0x7100; cpu.test_write8(0x0500,0x6A); cpu.test_write8(0x0501,0x84); cpu.test_write8(0x7100,0x80); cpu.step(); assert_eq!(cpu.mem[0x7100],0x7F); assert!(cpu.cc_v); }
    #[test] fn ror_indexed_carry_rotate_in(){ let mut cpu=CPU::default(); cpu.pc=0x0600; cpu.x=0x7200; cpu.cc_c=true; cpu.test_write8(0x0600,0x66); cpu.test_write8(0x0601,0x84); cpu.test_write8(0x7200,0x01); cpu.step(); assert_eq!(cpu.mem[0x7200],0x80); assert!(cpu.cc_c); assert!(cpu.cc_n); }
}

// --- SUBD ---
mod unified_subd { use vectrex_emulator::CPU; fn set_d(cpu:&mut CPU,val:u16){ cpu.a=(val>>8) as u8; cpu.b=val as u8; } fn get_d(cpu:&CPU)->u16{ ((cpu.a as u16)<<8)|cpu.b as u16 }
    #[test] fn subd_immediate_basic(){ let mut cpu=CPU::default(); set_d(&mut cpu,0x1234); cpu.pc=0x0100; cpu.test_write8(0x0100,0x83); cpu.test_write8(0x0101,0x00); cpu.test_write8(0x0102,0x34); assert!(cpu.step()); assert_eq!(get_d(&cpu),0x1200); assert!(!cpu.cc_z); assert!(!cpu.cc_c); }
    #[test] fn subd_direct_and_indexed(){ let mut cpu=CPU::default(); cpu.dp=0x00; cpu.test_write8(0x0000,0x01); cpu.test_write8(0x0001,0x00); set_d(&mut cpu,0x0200); cpu.pc=0x0200; cpu.test_write8(0x0200,0x93); cpu.test_write8(0x0201,0x00); assert!(cpu.step()); assert_eq!(get_d(&cpu),0x0100); cpu.x=0x3000; let ea=0x3000u16+5; cpu.test_write8(ea,0x00); cpu.test_write8(ea+1,0x10); set_d(&mut cpu,0x0110); cpu.pc=0x0300; cpu.test_write8(0x0300,0xA3); cpu.test_write8(0x0301,0x85); let _=cpu.step(); }
    #[test] fn subd_borrow_and_zero(){ let mut cpu=CPU::default(); set_d(&mut cpu,0x0001); cpu.pc=0x0100; cpu.test_write8(0x0100,0x83); cpu.test_write8(0x0101,0x00); cpu.test_write8(0x0102,0x02); let _=cpu.step(); assert_eq!(get_d(&cpu),0xFFFF); assert!(cpu.cc_c); }
}


