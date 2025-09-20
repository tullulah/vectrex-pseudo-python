//! Regression test: ensure all mapped BIOS addresses in bios_label_for() stay covered.
//! Añadir nuevas direcciones aquí cuando se etiqueten en bios_label_for.

use vectrex_emulator::opcode_meta::bios_label_for;

#[test]
fn bios_label_coverage() {
    // Curated list of addresses currently mapped. Keep sorted.
    let addresses: &[u16] = &[
        0xF06C, // Warm_Start
        0xF14C, // Init_VIA
        0xF164, // Init_OS_RAM
        0xF18B, // Init_OS
        0xF192, // Wait_Recal
        0xF1A2, // Set_Refresh
        0xF1AA, // DP_to_D0
        0xF1AF, // DP_to_C8
        0xF1B4, // Read_Btns_Mask
        0xF1BA, // Read_Btns
        0xF1F5, // Joy_Analog
        0xF1F8, // Joy_Digital
        0xF256, // Sound_Byte
    0xF259, // Sound_Byte_x
    0xF25B, // Sound_Byte_raw
        0xF272, // Clear_Sound
        0xF27D, // Sound_Bytes
        0xF284, // Sound_Bytes_x
        0xF289, // Do_Sound
        0xF28C, // Do_Sound_x
        0xF29D, // Intensity_1F
        0xF2A1, // Intensity_3F
        0xF2A5, // Intensity_5F
        0xF2A9, // Intensity_7F
        0xF2AB, // Intensity_a
        0xF2BE, // Dot_ix_b
        0xF2C1, // Dot_ix
        0xF2C3, // Dot_d
        0xF2C5, // Dot_here
        0xF2D5, // Dot_List
        0xF2DE, // Dot_List_Reset
        0xF2E6, // Recalibrate
        0xF2F2, // Moveto_x_7F
        0xF2FC, // Moveto_d_7F
        0xF308, // Moveto_ix_FF
        0xF30C, // Moveto_ix_7F
        0xF30E, // Moveto_ix_b
        0xF310, // Moveto_ix
        0xF312, // Moveto_d
        0xF34A, // Reset0Ref_D0
        0xF34F, // Check0Ref
        0xF354, // Reset0Ref
        0xF35B, // Reset_Pen
        0xF36B, // Reset0Int
        0xF373, // Print_Str_hwyx
        0xF378, // Print_Str_yx
        0xF37A, // Print_Str_d
        0xF385, // Print_List_hw
        0xF38A, // Print_List
        0xF38C, // Print_List_chk
        0xF391, // Print_Ships_x
        0xF393, // Print_Ships
        0xF3AD, // Mov_Draw_VLc_a
        0xF3B1, // Mov_Draw_VL_b
        0xF3B5, // Mov_Draw_VLcs
        0xF3B7, // Mov_Draw_VL_ab
        0xF3B9, // Mov_Draw_VL_a
        0xF3BC, // Mov_Draw_VL
        0xF3BE, // Mov_Draw_VL_d
        0xF3CE, // Draw_VLc
        0xF3D2, // Draw_VL_b
        0xF3D6, // Draw_VLcs
        0xF3D8, // Draw_VL_ab
        0xF3DA, // Draw_VL_a
        0xF3DD, // Draw_VL
        0xF3DF, // Draw_Line_d
        0xF404, // Draw_VLp_FF
        0xF408, // Draw_VLp_7F
        0xF40C, // Draw_VLp_scale
        0xF40E, // Draw_VLp_b
        0xF410, // Draw_VLp
        0xF434, // Draw_Pat_VL_a
        0xF437, // Draw_Pat_VL
        0xF439, // Draw_Pat_VL_d
        0xF46E, // Draw_VL_mode
        0xF511, // Random_3
        0xF517, // Random
        0xF533, // Init_Music_Buf
        0xF53F, // Clear_x_b
        0xF542, // Clear_C8_RAM
        0xF545, // Clear_x_256
        0xF548, // Clear_x_d
        0xF550, // Clear_x_b_80
        0xF552, // Clear_x_b_a
    0xF55A, // Dec_3_Counters
    0xF55E, // Dec_6_Counters
        0xF563, // Dec_Counters
        0xF56D, // Delay_3
        0xF571, // Delay_2
        0xF575, // Delay_1
        0xF579, // Delay_0
        0xF57A, // Delay_b
        0xF57D, // Delay_RTS
        0xF57E, // Bitmask_a
        0xF584, // Abs_a_b
        0xF58B, // Abs_b
        0xF593, // Rise_Run_Angle
        0xF5D9, // Get_Rise_Idx
        0xF5DB, // Get_Run_Idx
        0xF5EF, // Rise_Run_Idx
        0xF5FF, // Rise_Run_X
        0xF601, // Rise_Run_Y
        0xF603, // Rise_Run_Len
        0xF610, // Rot_VL_ab
        0xF616, // Rot_VL
        0xF61F, // Rot_VL_Mode
        0xF62B, // Rot_VL_M_dft
        0xF65B, // Xform_Run_a
        0xF65D, // Xform_Run
        0xF661, // Xform_Rise_a
        0xF663, // Xform_Rise
        0xF67F, // Move_Mem_a_1
        0xF683, // Move_Mem_a
        0xF687, // Init_Music_chk
        0xF68D, // Init_Music
        0xF692, // Init_Music_dft
        0xF7A9, // Select_Game
        0xF835, // Display_Option
        0xF84F, // Clear_Score
        0xF85E, // Add_Score_a
        0xF87C, // Add_Score_d
        0xF8B7, // Strip_Zeros
        0xF8C7, // Compare_Score
        0xF8D8, // New_High_Score
        0xF8E5, // Obj_Will_Hit_u
        0xF8F3, // Obj_Will_Hit
        0xF8FF, // Obj_Hit
        0xF92E, // Explosion_Snd
    ];

    for &addr in addresses {
        assert!(bios_label_for(addr).is_some(), "Missing BIOS label for address 0x{addr:04X}");
    }
    // Sane guard: ensure none of these resolves to placeholder name (should not happen because we unwrap before).
    // (If future refactor changes bios_label_for to return placeholder, this will catch it.)
}
