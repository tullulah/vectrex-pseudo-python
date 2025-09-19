//! Watchdog de ciclos: asegura que ciertos opcodes implementados nunca producen delta 0
//! y que si su nominal (>1) no cae accidentalmente a 1. Complementa `opcode_exhaustive_smoke`
//! con una lista curada de instrucciones de memoria / control consideradas críticas.
//!
//! Política: no genera BIOS, sólo test sintético de instrucciones sueltas.

use vectrex_emulator::CPU;

#[derive(Clone,Copy)]
struct Case { bytes: &'static [u8], min_cycles: u32, label: &'static str }

fn run(mut cpu: CPU) -> u32 { let before=cpu.cycles; let ok=cpu.step(); assert!(ok, "step false"); (cpu.cycles-before) as u32 }

#[test]
fn cycle_watchdog_core_set() {
    // Lista: incluye RMW, store, branch, jsr, long branch, tfr/exg y un par de indexados.
    let cases = [
        Case { bytes: &[0xC6,0x10], min_cycles:2, label:"LDB #imm" },
        Case { bytes: &[0xD7,0x80], min_cycles:4, label:"STB direct" },
        Case { bytes: &[0xBD,0x12,0x34], min_cycles:7, label:"JSR extended" },
        Case { bytes: &[0x20,0x00], min_cycles:2, label:"BRA short base" },
        Case { bytes: &[0x20,0x04], min_cycles:3, label:"BRA taken" },
        Case { bytes: &[0x1F,0x89], min_cycles:6, label:"TFR A->B" },
        Case { bytes: &[0x1E,0x89], min_cycles:8, label:"EXG A<->B" },
    // STB ,X (modo indexado simple) actualmente mide 5 ciclos en implementación; nominal puede ser >=5.
    Case { bytes: &[0xE7,0x00], min_cycles:5, label:"STB ,X+ (indexed simple placeholder)" },
        Case { bytes: &[0x8D,0x02], min_cycles:7, label:"BSR" },
        Case { bytes: &[0x16,0x00,0x03], min_cycles:5, label:"LBRA" },
    ];

    for case in cases { 
        let mut cpu=CPU::default();
        cpu.pc=0x0400;
        for (i,b) in case.bytes.iter().enumerate() { cpu.test_write8(0x0400 + i as u16,*b); }
        // Pequeñas precondiciones para algunas (indexado): set X
        if case.label.starts_with("STB ,X") { cpu.x=0x0500; cpu.b=0x22; }
        let delta = run(cpu);
        assert!(delta >= case.min_cycles, "{}: delta {} < mínimo {} (regresión de timing)", case.label, delta, case.min_cycles);
        assert!(delta > 0, "{}: delta 0 imposible para opcode implementado", case.label);
    }
}
