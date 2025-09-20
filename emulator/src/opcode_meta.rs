//! Opcode metadata scaffold: tamaños (bytes) y ciclos base documentados para un subconjunto inicial.
//!
//! Objetivo: centralizar longitud de instrucción y ciclos teóricos (sin ajustes dinámicos:
//!  - ramas cortas tomadas +1 ciclo (se marca flag `branch_short`)
//!  - ramas largas (prefijo 0x10) también pueden añadir
//!  - modos indexados complejos pueden añadir (pendiente modelar por postbyte)
//!
//! Política actual: este módulo es PASIVO. No se usa aún para conducir `CPU::step`.
//! Se añade para permitir tests que verifiquen que la longitud (PC delta) y ciclos
//! coinciden con la referencia mientras migramos desde seeds embebidos.
//!
//! Añadir nuevas entradas incrementalmente; mantener comentarios bilingües cuando sea relevante.

#![allow(dead_code)]

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OpcodeMeta {
    pub opcode: u16,      // opcode completo (incluye prefijo en high byte si aplica: 0x10CE -> LDS imm)
    pub size: u8,         // bytes totales consumidos por la instrucción (incluyendo prefijo(s))
    pub base_cycles: u8,  // ciclos base (sin sumas dinámicas por branch taken / indexing extra)
    pub branch_short: bool,
    pub branch_long: bool,
}

// BIOS label helper mapping.
// Nota: mantener sincronizado con bios.asm. Sólo añadir etiquetas confirmadas del listado original.
pub fn bios_label_for(addr: u16) -> Option<&'static str> {
    match addr {
    // Entry / reset & intro loops
    0xF000 => Some("Start"), // Punto de entrada BIOS (Reset vector apunta aquí)
    0xF01C => Some("Intro_Loop_1"), // Primer bucle power-up (logo VECTREX, música intro)
    0xF0A4 => Some("Intro_Loop_2"), // Segundo bucle (copyright + high score + salto a cartucho)
        // System / init
        0xF06C => Some("Warm_Start"),
        0xF14C => Some("Init_VIA"),
        0xF164 => Some("Init_OS_RAM"),
        0xF18B => Some("Init_OS"),
        0xF192 => Some("Wait_Recal"),
        0xF1A2 => Some("Set_Refresh"),
        0xF1AA => Some("DP_to_D0"),
        0xF1AF => Some("DP_to_C8"),
        // Input
        0xF1B4 => Some("Read_Btns_Mask"),
        0xF1BA => Some("Read_Btns"),
        0xF1F5 => Some("Joy_Analog"),
        0xF1F8 => Some("Joy_Digital"),
        // Sound intensity & audio
        0xF256 => Some("Sound_Byte"),
        0xF259 => Some("Sound_Byte_x"),
        0xF25B => Some("Sound_Byte_raw"),
        0xF272 => Some("Clear_Sound"),
        0xF27D => Some("Sound_Bytes"),
        0xF284 => Some("Sound_Bytes_x"),
        0xF289 => Some("Do_Sound"),
        0xF28C => Some("Do_Sound_x"),
        0xF29D => Some("Intensity_1F"),
        0xF2A1 => Some("Intensity_3F"),
        0xF2A5 => Some("Intensity_5F"),
        0xF2A9 => Some("Intensity_7F"),
        0xF2AB => Some("Intensity_a"),
        // Dots / movement / positioning
        0xF2BE => Some("Dot_ix_b"),
        0xF2C1 => Some("Dot_ix"),
        0xF2C3 => Some("Dot_d"),
        0xF2C5 => Some("Dot_here"),
        0xF2D5 => Some("Dot_List"),
        0xF2DE => Some("Dot_List_Reset"),
        0xF2E6 => Some("Recalibrate"),
        0xF2F2 => Some("Moveto_x_7F"),
        0xF2FC => Some("Moveto_d_7F"),
        0xF308 => Some("Moveto_ix_FF"),
        0xF30C => Some("Moveto_ix_7F"),
        0xF30E => Some("Moveto_ix_b"),
        0xF310 => Some("Moveto_ix"),
        0xF312 => Some("Moveto_d"),
        // Reset / references / pen
        0xF34A => Some("Reset0Ref_D0"),
        0xF34F => Some("Check0Ref"),
        0xF354 => Some("Reset0Ref"),
        0xF35B => Some("Reset_Pen"),
        0xF36B => Some("Reset0Int"),
        // Printing & list display
        0xF373 => Some("Print_Str_hwyx"),
        0xF378 => Some("Print_Str_yx"),
        0xF37A => Some("Print_Str_d"),
        0xF385 => Some("Print_List_hw"),
        0xF38A => Some("Print_List"),
        0xF38C => Some("Print_List_chk"),
        0xF391 => Some("Print_Ships_x"),
        0xF393 => Some("Print_Ships"),
        0xF495 => Some("Print_Str"),
        // Move + draw combined
        0xF3AD => Some("Mov_Draw_VLc_a"),
        0xF3B1 => Some("Mov_Draw_VL_b"),
        0xF3B5 => Some("Mov_Draw_VLcs"),
        0xF3B7 => Some("Mov_Draw_VL_ab"),
        0xF3B9 => Some("Mov_Draw_VL_a"),
        0xF3BC => Some("Mov_Draw_VL"),
        0xF3BE => Some("Mov_Draw_VL_d"),
        // Drawing core & variants
        0xF3CE => Some("Draw_VLc"),
        0xF3D2 => Some("Draw_VL_b"),
        0xF3D6 => Some("Draw_VLcs"),
        0xF3D8 => Some("Draw_VL_ab"),
        0xF3DA => Some("Draw_VL_a"),
        0xF3DD => Some("Draw_VL"),
        0xF3DF => Some("Draw_Line_d"),
        // Pre-move draw variants
        0xF404 => Some("Draw_VLp_FF"),
        0xF408 => Some("Draw_VLp_7F"),
        0xF40C => Some("Draw_VLp_scale"),
        0xF40E => Some("Draw_VLp_b"),
        0xF410 => Some("Draw_VLp"),
        // Patterned vector lists & modes
        0xF434 => Some("Draw_Pat_VL_a"),
        0xF437 => Some("Draw_Pat_VL"),
        0xF439 => Some("Draw_Pat_VL_d"),
        0xF46E => Some("Draw_VL_mode"),
        // Random
        0xF511 => Some("Random_3"),
        0xF517 => Some("Random"),
        // Music buffer + init
        0xF533 => Some("Init_Music_Buf"),
        0xF53F => Some("Clear_x_b"),
        0xF542 => Some("Clear_C8_RAM"),
        0xF545 => Some("Clear_x_256"),
        0xF548 => Some("Clear_x_d"),
        0xF550 => Some("Clear_x_b_80"),
        0xF552 => Some("Clear_x_b_a"),
        // Counters
        0xF55A => Some("Dec_3_Counters"),
        0xF55E => Some("Dec_6_Counters"),
        0xF563 => Some("Dec_Counters"),
        // Delays
        0xF56D => Some("Delay_3"),
        0xF571 => Some("Delay_2"),
        0xF575 => Some("Delay_1"),
        0xF579 => Some("Delay_0"),
        0xF57A => Some("Delay_b"),
        0xF57D => Some("Delay_RTS"),
        0xF57E => Some("Bitmask_a"),
        // Math / geometry helpers
        0xF584 => Some("Abs_a_b"),
        0xF58B => Some("Abs_b"),
        0xF593 => Some("Rise_Run_Angle"),
        0xF5D9 => Some("Get_Rise_Idx"),
        0xF5DB => Some("Get_Run_Idx"),
        0xF5EF => Some("Rise_Run_Idx"),
        0xF5FF => Some("Rise_Run_X"),
        0xF601 => Some("Rise_Run_Y"),
        0xF603 => Some("Rise_Run_Len"),
        0xF610 => Some("Rot_VL_ab"),
        0xF616 => Some("Rot_VL"),
        0xF61F => Some("Rot_VL_Mode"),
        0xF62B => Some("Rot_VL_M_dft"),
        0xF65B => Some("Xform_Run_a"),
        0xF65D => Some("Xform_Run"),
        0xF661 => Some("Xform_Rise_a"),
        0xF663 => Some("Xform_Rise"),
        // Memory move
        0xF67F => Some("Move_Mem_a_1"),
        0xF683 => Some("Move_Mem_a"),
        // Music runtime
        0xF687 => Some("Init_Music_chk"),
        0xF68D => Some("Init_Music"),
        0xF692 => Some("Init_Music_dft"),
        // Game selection / UI
        0xF7A9 => Some("Select_Game"),
        0xF835 => Some("Display_Option"),
        // Score handling
        0xF84F => Some("Clear_Score"),
        0xF85E => Some("Add_Score_a"),
        0xF87C => Some("Add_Score_d"),
        0xF8B7 => Some("Strip_Zeros"),
        0xF8C7 => Some("Compare_Score"),
        0xF8D8 => Some("New_High_Score"),
        // Collision
        0xF8E5 => Some("Obj_Will_Hit_u"),
        0xF8F3 => Some("Obj_Will_Hit"),
        0xF8FF => Some("Obj_Hit"),
        // Effects
        0xF92E => Some("Explosion_Snd"),
        0xFF9F => Some("Draw_Grid_VL"), // Rutina grilla 16x16 (vector list compuesta)
        // Final
        _ => None,
    }
}

impl OpcodeMeta {
    pub const fn simple(opcode:u16, size:u8, base_cycles:u8) -> Self { Self { opcode, size, base_cycles, branch_short:false, branch_long:false } }
    pub const fn branch_short(opcode:u16, size:u8, base_cycles:u8) -> Self { Self { opcode, size, base_cycles, branch_short:true, branch_long:false } }
}

// Nota sobre codificación de prefijos:
//  - Usamos 0x10xx para opcodes con primer byte 0x10 y segundo = xx
//  - 0x11xx similar para prefijo 0x11
//  Esto evita ambigüedad y permite key único en un solo u16.

pub static OPCODE_META_SUBSET: &[OpcodeMeta] = &[
    // LDS inmediato: 10 CE hi lo -> tamaño 4 bytes, 5 ciclos base
    OpcodeMeta::simple(0x10CE, 4, 5),
    // JSR extendido: BD hi lo -> 3 bytes, 7 ciclos
    OpcodeMeta::simple(0x00BD, 3, 7),
    // BRA relativo corto: 20 off -> 2 bytes, base 2 ciclos (+1 si tomada). Marcamos branch_short
    OpcodeMeta::branch_short(0x0020, 2, 2),
    // BSR relativo: 8D off -> 2 bytes, 7 ciclos (no increment dinámico; push + fetch + calc)
    OpcodeMeta::simple(0x008D, 2, 7),
    // RTS: 39 -> 1 byte, 5 ciclos
    OpcodeMeta::simple(0x0039, 1, 5),
    // SUBB inmediato: C0 imm -> 2 bytes, 2 ciclos
    OpcodeMeta::simple(0x00C0, 2, 2),
];

pub fn lookup_meta(opcode_first: u8, maybe_second: Option<u8>) -> Option<OpcodeMeta> {
    let key = match maybe_second { Some(s) if opcode_first==0x10 || opcode_first==0x11 => ((opcode_first as u16)<<8) | s as u16, _ => (opcode_first as u16)<<8 | 0x00 | maybe_second.unwrap_or(0) as u16 }; // fallback simple
    // Para opcodes sin prefijo, guardamos como 0x00XX para que coincida con pattern 0x00BD etc.
    let key = if opcode_first==0x10 || opcode_first==0x11 { key } else { 0x0000 | opcode_first as u16 }; // ignorar maybe_second si no prefijo
    for m in OPCODE_META_SUBSET { if m.opcode == key { return Some(*m); } }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn meta_lookup_basic() {
        let lds = lookup_meta(0x10, Some(0xCE)).unwrap();
        assert_eq!(lds.size, 4); assert_eq!(lds.base_cycles, 5);
        let jsr = lookup_meta(0xBD, None).unwrap();
        assert_eq!(jsr.size, 3); assert_eq!(jsr.base_cycles, 7);
        let bra = lookup_meta(0x20, None).unwrap();
        assert!(bra.branch_short); assert_eq!(bra.base_cycles, 2);
    }
}
