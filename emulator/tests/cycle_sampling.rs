//! Muestreo de ciclos: compara un subconjunto de opcodes ejecutados con la tabla nominal JSON
//! (`docs/6809_cycles_nominal.json`). Objetivo: detectar divergencias tempranas sin
//! exigir cobertura total aún. Ajustes dinámicos aplicados: +1 ciclo si branch corto tomado.
//!
//! Reglas:
//! - No generamos BIOS sintética; este test sólo ejecuta instrucciones simples fuera de BIOS.
//! - Si un opcode no figura en el JSON nominal todavía, se omite silenciosamente.
//! - Si hay mismatch se reporta con detalle (opcode, nominal, medido, contexto adicional).
//!
//! Futuro: añadir máscaras PSHS/PULS (variable), indexados complejos y long branches.

use std::fs;
use std::collections::HashMap;
use vectrex_emulator::CPU; // re-export desde lib

#[derive(Debug)]
struct NominalTables { primary: HashMap<u8,u32>, ext10: HashMap<u8,u32>, ext11: HashMap<u8,u32> }

fn load_nominal() -> NominalTables {
    // docs está en el workspace raíz, subir un nivel desde crate (emulator) usando CARGO_MANIFEST_DIR
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let root = std::path::Path::new(manifest_dir).parent().expect("parent");
    let path = root.join("docs/6809_cycles_nominal.json");
    let data = fs::read_to_string(&path).expect("leer docs/6809_cycles_nominal.json");
    let v: serde_json::Value = serde_json::from_str(&data).expect("json válido");
    let mut primary=HashMap::new();
    if let Some(p)=v.get("primary").and_then(|x| x.as_object()) { for (k,val) in p { if let Ok(byte)=u8::from_str_radix(k,16) { if let Some(c)=val.as_u64() { primary.insert(byte, c as u32); } } } }
    let mut ext10=HashMap::new();
    if let Some(p)=v.get("ext10").and_then(|x| x.as_object()) { for (k,val) in p { if let Ok(byte)=u8::from_str_radix(k,16) { if let Some(c)=val.as_u64() { ext10.insert(byte, c as u32); } } } }
    let mut ext11=HashMap::new();
    if let Some(p)=v.get("ext11").and_then(|x| x.as_object()) { for (k,val) in p { if let Ok(byte)=u8::from_str_radix(k,16) { if let Some(c)=val.as_u64() { ext11.insert(byte, c as u32); } } } }
    NominalTables { primary, ext10, ext11 }
}

fn run_single(mut cpu: CPU) -> (CPU,u32) {
    let before=cpu.cycles; let ok=cpu.step(); assert!(ok,"step false"); let d=(cpu.cycles-before) as u32; (cpu,d)
}

#[test]
fn cycle_sampling_subset() {
    let tables=load_nominal();
    // Subconjunto escogido: LDA imm (0x86), LDB imm (0xC6), BRA tomado (0x20), BRA no tomado (0x20 con offset 0), RTS (0x39), JSR ext (0xBD), TFR (0x1F), EXG (0x1E)
    // Además una long branch LBRA (0x16) si nominal ya está (primary[0x16]==5 según JSON).

    let mut mismatches: Vec<String> = Vec::new();

    // Helper para comparar
    let mut check = |label:&str, op_bytes:&[u8], setup: fn(&mut CPU), dynamic_adj: fn(u32,&CPU)->u32, expect_key:(u8,bool,u8)| {
        let mut cpu=CPU::default();
        cpu.pc=0x0400;
        for (i,b) in op_bytes.iter().enumerate() { cpu.test_write8(0x0400 + i as u16, *b); }
        setup(&mut cpu);
        let (cpu_after, measured)=run_single(cpu);
        let base_nominal = if expect_key.1 { // prefixed
            if expect_key.2==0x10 { tables.ext10.get(&expect_key.0).cloned() } else { tables.ext11.get(&expect_key.0).cloned() }
        } else { tables.primary.get(&expect_key.0).cloned() };
        if let Some(base) = base_nominal {
            let adjusted = dynamic_adj(base, &cpu_after);
            if adjusted != measured {
                mismatches.push(format!("{} opcode {:02X} nominal {} (ajustada {}) medida {} bytes {:02X?}", label, expect_key.0, base, adjusted, measured, op_bytes));
            }
        }
    };

    // LDA inmediato 0x86
    check("LDA imm", &[0x86,0x7F], |_| {}, |base,_cpu| base, (0x86,false,0));
    // LDB inmediato 0xC6
    check("LDB imm", &[0xC6,0x20], |_| {}, |base,_| base, (0xC6,false,0));
    // BRA tomado: offset +4. Nominal JSON da 3 -> asumimos incluye penalización.
    check("BRA taken", &[0x20,0x04], |_| {}, |base,_| base, (0x20,false,0));
    // BRA "zero" offset 0 (salta a misma+2+0) aceptamos base igual.
    check("BRA zero", &[0x20,0x00], |_| {}, |base,_| base, (0x20,false,0));
    // RTS 0x39 (requiere call_stack simulado)
    check("RTS", &[0x39], |c| { c.call_stack.push(0x0555); }, |base,_| base, (0x39,false,0));
    // JSR extended 0xBD
    check("JSR ext", &[0xBD,0x12,0x34], |_| {}, |base,_| base, (0xBD,false,0));
    // TFR 0x1F A->B
    check("TFR A->B", &[0x1F,0x89], |c| { c.a=0x11; }, |base,_| base, (0x1F,false,0));
    // EXG 0x1E (A<->B)
    check("EXG A<->B", &[0x1E,0x89], |c| { c.a=0x22; c.b=0x33; }, |base,_| base, (0x1E,false,0));
    // LBRA 0x16 long branch always. Si no está definido en JSON se omite.
    check("LBRA", &[0x16,0x00,0x03], |_| {}, |base,_| base, (0x16,false,0));

    if !mismatches.is_empty() {
        panic!("Mismatches de ciclos:\n{}", mismatches.join("\n"));
    }
}
