; VPy M6809 Assembly (Vectrex)
; ROM: 524288 bytes
; Multibank cartridge: 32 banks (16KB each)
; Helpers bank: 31 (fixed bank at $4000-$7FFF)

; ================================================


; === RAM VARIABLE DEFINITIONS ===
;***************************************************************************
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPPTR               EQU $C880+$02   ; Temporary pointer (2 bytes)
TMPPTR2              EQU $C880+$04   ; Temporary pointer 2 (2 bytes)
TEMP_YX              EQU $C880+$06   ; Temporary Y/X coordinate storage (2 bytes)
VLINE_DX_16          EQU $C880+$08   ; DRAW_LINE dx (16-bit) (2 bytes)
VLINE_DY_16          EQU $C880+$0A   ; DRAW_LINE dy (16-bit) (2 bytes)
VLINE_DX             EQU $C880+$0C   ; DRAW_LINE dx clamped (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$0D   ; DRAW_LINE dy clamped (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$0E   ; DRAW_LINE remaining dy for segment 2 (1 bytes)
VAR_ARG0             EQU $CFE0   ; Function argument 0 (16-bit) (2 bytes)
VAR_ARG1             EQU $CFE2   ; Function argument 1 (16-bit) (2 bytes)
VAR_ARG2             EQU $CFE4   ; Function argument 2 (16-bit) (2 bytes)
VAR_ARG3             EQU $CFE6   ; Function argument 3 (16-bit) (2 bytes)
VAR_ARG4             EQU $CFE8   ; Function argument 4 (16-bit) (2 bytes)


; ================================================

; VPy M6809 Assembly (Vectrex)
; ROM: 524288 bytes
; Multibank cartridge: 32 banks (16KB each)
; Helpers bank: 31 (fixed bank at $4000-$7FFF)

; ================================================

    ORG $0000

;***************************************************************************
; DEFINE SECTION
;***************************************************************************
    INCLUDE "VECTREX.I"
; External symbols (helpers and shared data)
DRAW_PAT_VL EQU $F437
Vec_Duration EQU $C857
RESET_PEN EQU $F35B
DOT_IX_B EQU $F2BE
musicc EQU $FF7A
MOV_DRAW_VL_A EQU $F3B9
Vec_Expl_ChanB EQU $C85D
JOY_DIGITAL EQU $F1F8
GET_RUN_IDX EQU $F5DB
MUSIC3 EQU $FD81
Vec_Expl_Chans EQU $C854
CLEAR_X_B_A EQU $F552
Check0Ref EQU $F34F
Vec_Joy_Mux_1_Y EQU $C820
DEC_6_COUNTERS EQU $F55E
Vec_Music_Wk_6 EQU $C846
VEC_COUNTER_5 EQU $C832
VEC_MAX_PLAYERS EQU $C84F
music4 EQU $FDD3
Vec_Button_2_1 EQU $C816
Vec_Pattern EQU $C829
VEC_0REF_ENABLE EQU $C824
WAIT_RECAL EQU $F192
Vec_Music_Wk_5 EQU $C847
DELAY_B EQU $F57A
VEC_RISE_INDEX EQU $C839
Mov_Draw_VL EQU $F3BC
SOUND_BYTE EQU $F256
VEC_EXPL_4 EQU $C85B
Vec_Joy_2_Y EQU $C81E
DRAW_VLC EQU $F3CE
Moveto_ix_FF EQU $F308
MOV_DRAW_VL_B EQU $F3B1
Mov_Draw_VL_b EQU $F3B1
DRAW_VLP_FF EQU $F404
Xform_Rise_a EQU $F661
RISE_RUN_LEN EQU $F603
Dot_List_Reset EQU $F2DE
Vec_Expl_Timer EQU $C877
ABS_A_B EQU $F584
XFORM_RUN_A EQU $F65B
XFORM_RUN EQU $F65D
VEC_MISC_COUNT EQU $C823
Xform_Rise EQU $F663
RECALIBRATE EQU $F2E6
Abs_b EQU $F58B
INIT_OS_RAM EQU $F164
VEC_BRIGHTNESS EQU $C827
Vec_Music_Chan EQU $C855
Rise_Run_X EQU $F5FF
Dec_6_Counters EQU $F55E
Init_VIA EQU $F14C
MUSIC2 EQU $FD1D
Vec_Button_2_3 EQU $C818
ABS_B EQU $F58B
Vec_Counter_5 EQU $C832
VEC_TEXT_WIDTH EQU $C82B
XFORM_RISE_A EQU $F661
Select_Game EQU $F7A9
MUSICA EQU $FF44
Vec_Expl_1 EQU $C858
VEC_EXPL_3 EQU $C85A
Random_3 EQU $F511
Print_Str_d EQU $F37A
Get_Run_Idx EQU $F5DB
Vec_Misc_Count EQU $C823
ROT_VL_AB EQU $F610
DRAW_LINE_D EQU $F3DF
Vec_Default_Stk EQU $CBEA
Vec_Music_Work EQU $C83F
VEC_EXPL_2 EQU $C859
VEC_BUTTON_2_2 EQU $C817
VEC_COUNTER_1 EQU $C82E
SOUND_BYTE_X EQU $F259
Dot_d EQU $F2C3
DRAW_PAT_VL_D EQU $F439
DRAW_VLP_SCALE EQU $F40C
MUSIC6 EQU $FE76
VEC_BUTTONS EQU $C811
Intensity_3F EQU $F2A1
VEC_TWANG_TABLE EQU $C851
Set_Refresh EQU $F1A2
Init_Music EQU $F68D
VEC_EXPL_CHANS EQU $C854
READ_BTNS_MASK EQU $F1B4
PRINT_SHIPS EQU $F393
Vec_Joy_Resltn EQU $C81A
Vec_Joy_Mux_2_Y EQU $C822
Reset0Ref_D0 EQU $F34A
Vec_Counter_6 EQU $C833
PRINT_STR_HWYX EQU $F373
Init_Music_Buf EQU $F533
Vec_Expl_3 EQU $C85A
INIT_OS EQU $F18B
Vec_Angle EQU $C836
PRINT_LIST_HW EQU $F385
Vec_Music_Wk_A EQU $C842
VEC_IRQ_VECTOR EQU $CBF8
VEC_DOT_DWELL EQU $C828
Draw_VLp EQU $F410
DP_TO_D0 EQU $F1AA
Explosion_Snd EQU $F92E
DRAW_VLP_B EQU $F40E
VEC_SEED_PTR EQU $C87B
RISE_RUN_X EQU $F5FF
SOUND_BYTE_RAW EQU $F25B
INIT_MUSIC_BUF EQU $F533
PRINT_STR EQU $F495
Rot_VL_dft EQU $F637
Vec_Str_Ptr EQU $C82C
Clear_Sound EQU $F272
MOV_DRAW_VL_AB EQU $F3B7
Rot_VL_Mode EQU $F62B
MOD16 EQU $4013
STRIP_ZEROS EQU $F8B7
Draw_VLcs EQU $F3D6
Sound_Byte_x EQU $F259
Draw_VL_a EQU $F3DA
MUSIC1 EQU $FD0D
Vec_Prev_Btns EQU $C810
MOV_DRAW_VL_D EQU $F3BE
VEC_RISERUN_TMP EQU $C834
Draw_Pat_VL EQU $F437
VEC_COUNTER_2 EQU $C82F
DP_TO_C8 EQU $F1AF
INIT_MUSIC_X EQU $F692
Vec_Joy_Mux_2_X EQU $C821
VEC_COUNTER_6 EQU $C833
music5 EQU $FE38
Get_Rise_Idx EQU $F5D9
DRAW_VL EQU $F3DD
MOVETO_X_7F EQU $F2F2
Vec_SWI2_Vector EQU $CBF2
Draw_VLp_b EQU $F40E
Vec_Seed_Ptr EQU $C87B
Intensity_a EQU $F2AB
VEC_JOY_1_Y EQU $C81C
Do_Sound EQU $F289
Vec_Dot_Dwell EQU $C828
Vec_Button_1_3 EQU $C814
Clear_x_b_80 EQU $F550
Clear_C8_RAM EQU $F542
MOVE_MEM_A_1 EQU $F67F
Vec_Expl_4 EQU $C85B
VEC_JOY_1_X EQU $C81B
Vec_Rfrsh EQU $C83D
Init_Music_chk EQU $F687
Read_Btns EQU $F1BA
VEC_RUN_INDEX EQU $C837
Delay_1 EQU $F575
MUSIC9 EQU $FF26
DOT_D EQU $F2C3
VEC_SWI3_VECTOR EQU $CBF2
RANDOM EQU $F517
VECTREX_PRINT_TEXT EQU $4000
Delay_RTS EQU $F57D
DEC_3_COUNTERS EQU $F55A
Dot_List EQU $F2D5
VEC_BUTTON_2_3 EQU $C818
VEC_JOY_MUX_2_Y EQU $C822
Sound_Bytes EQU $F27D
Vec_Text_Height EQU $C82A
Moveto_ix EQU $F310
Draw_VLp_7F EQU $F408
Mov_Draw_VL_ab EQU $F3B7
DRAW_VLP EQU $F410
Wait_Recal EQU $F192
PRINT_TEXT_STR_82781042 EQU $4038
Vec_Rise_Index EQU $C839
DELAY_RTS EQU $F57D
MOVETO_IX_FF EQU $F308
Recalibrate EQU $F2E6
VEC_BUTTON_1_2 EQU $C813
EXPLOSION_SND EQU $F92E
Vec_Buttons EQU $C811
VEC_EXPL_TIMER EQU $C877
PRINT_STR_YX EQU $F378
INTENSITY_1F EQU $F29D
Dec_3_Counters EQU $F55A
VEC_FIRQ_VECTOR EQU $CBF5
MUSIC8 EQU $FEF8
Vec_Counter_4 EQU $C831
VEC_SWI2_VECTOR EQU $CBF2
DO_SOUND EQU $F289
VEC_MUSIC_WORK EQU $C83F
music7 EQU $FEC6
RANDOM_3 EQU $F511
MOV_DRAW_VL EQU $F3BC
BITMASK_A EQU $F57E
MUSICB EQU $FF62
Vec_Button_2_4 EQU $C819
VEC_HIGH_SCORE EQU $CBEB
Vec_Music_Wk_7 EQU $C845
Draw_Grid_VL EQU $FF9F
RESET0REF_D0 EQU $F34A
Vec_Counter_2 EQU $C82F
VEC_ANGLE EQU $C836
Obj_Will_Hit_u EQU $F8E5
VEC_COUNTER_4 EQU $C831
Rise_Run_Len EQU $F603
musica EQU $FF44
PRINT_LIST_CHK EQU $F38C
Add_Score_a EQU $F85E
Move_Mem_a_1 EQU $F67F
OBJ_HIT EQU $F8FF
DELAY_2 EQU $F571
Reset_Pen EQU $F35B
Xform_Run EQU $F65D
PRINT_SHIPS_X EQU $F391
Joy_Analog EQU $F1F5
Draw_Pat_VL_d EQU $F439
ADD_SCORE_D EQU $F87C
PRINT_STR_D EQU $F37A
VEC_NMI_VECTOR EQU $CBFB
Vec_Text_Width EQU $C82B
SOUND_BYTES EQU $F27D
VEC_BUTTON_1_4 EQU $C815
DP_to_C8 EQU $F1AF
Vec_Joy_2_X EQU $C81D
Intensity_1F EQU $F29D
Print_Ships_x EQU $F391
Vec_Brightness EQU $C827
VEC_MAX_GAMES EQU $C850
Vec_Freq_Table EQU $C84D
VEC_LOOP_COUNT EQU $C825
MUSIC4 EQU $FDD3
DEC_COUNTERS EQU $F563
ADD_SCORE_A EQU $F85E
Intensity_7F EQU $F2A9
MOVETO_IX_A EQU $F30E
CLEAR_X_B EQU $F53F
VEC_MUSIC_PTR EQU $C853
Draw_VL_b EQU $F3D2
Delay_0 EQU $F579
Clear_x_d EQU $F548
Draw_VL EQU $F3DD
INTENSITY_7F EQU $F2A9
VEC_MUSIC_WK_7 EQU $C845
Rot_VL_ab EQU $F610
VEC_PATTERN EQU $C829
Vec_Cold_Flag EQU $CBFE
ROT_VL_DFT EQU $F637
Dot_ix EQU $F2C1
Vec_Joy_1_X EQU $C81B
VEC_EXPL_CHANB EQU $C85D
Vec_Expl_ChanA EQU $C853
Mov_Draw_VL_d EQU $F3BE
Vec_Music_Freq EQU $C861
Read_Btns_Mask EQU $F1B4
Vec_RiseRun_Len EQU $C83B
MOV_DRAW_VLC_A EQU $F3AD
SET_REFRESH EQU $F1A2
New_High_Score EQU $F8D8
Delay_2 EQU $F571
Vec_Counters EQU $C82E
DRAW_PAT_VL_A EQU $F434
SELECT_GAME EQU $F7A9
VEC_EXPL_FLAG EQU $C867
Vec_Text_HW EQU $C82A
MOVE_MEM_A EQU $F683
Vec_Button_1_2 EQU $C813
VEC_EXPL_CHAN EQU $C85C
Vec_Random_Seed EQU $C87D
Vec_Max_Players EQU $C84F
Vec_SWI3_Vector EQU $CBF2
Vec_Run_Index EQU $C837
READ_BTNS EQU $F1BA
Moveto_ix_a EQU $F30E
Vec_0Ref_Enable EQU $C824
DOT_LIST EQU $F2D5
MOVETO_D_7F EQU $F2FC
CLEAR_SOUND EQU $F272
VEC_JOY_MUX_1_X EQU $C81F
Vec_Button_1_1 EQU $C812
VEC_MUSIC_FLAG EQU $C856
Init_OS EQU $F18B
DRAW_VL_MODE EQU $F46E
Vec_High_Score EQU $CBEB
Draw_Line_d EQU $F3DF
WARM_START EQU $F06C
music9 EQU $FF26
Vec_ADSR_Table EQU $C84F
Intensity_5F EQU $F2A5
RISE_RUN_ANGLE EQU $F593
Vec_Joy_Mux_1_X EQU $C81F
DELAY_1 EQU $F575
VEC_JOY_MUX_1_Y EQU $C820
musicb EQU $FF62
CLEAR_SCORE EQU $F84F
DRAW_VLCS EQU $F3D6
Vec_IRQ_Vector EQU $CBF8
Abs_a_b EQU $F584
Draw_VL_ab EQU $F3D8
OBJ_WILL_HIT EQU $F8F3
DOT_IX EQU $F2C1
Vec_Num_Game EQU $C87A
Vec_Rfrsh_hi EQU $C83E
GET_RISE_RUN EQU $F5EF
MUSICD EQU $FF8F
MUSICC EQU $FF7A
Mov_Draw_VL_a EQU $F3B9
VEC_JOY_MUX EQU $C81F
Clear_Score EQU $F84F
INTENSITY_A EQU $F2AB
DP_to_D0 EQU $F1AA
VEC_JOY_2_Y EQU $C81E
Clear_x_b_a EQU $F552
VEC_SND_SHADOW EQU $C800
Clear_x_256 EQU $F545
COLD_START EQU $F000
VEC_RANDOM_SEED EQU $C87D
Rot_VL_Mode_a EQU $F61F
Vec_Snd_Shadow EQU $C800
CLEAR_C8_RAM EQU $F542
VEC_RFRSH EQU $C83D
Vec_FIRQ_Vector EQU $CBF5
Vec_Music_Twang EQU $C858
Vec_Twang_Table EQU $C851
Cold_Start EQU $F000
PRINT_LIST EQU $F38A
Vec_SWI_Vector EQU $CBFB
Vec_Expl_2 EQU $C859
INIT_MUSIC_CHK EQU $F687
CLEAR_X_D EQU $F548
Draw_VLc EQU $F3CE
VEC_COUNTER_3 EQU $C830
MOD16.MOD16_LOOP EQU $4015
Vec_Music_Flag EQU $C856
VEC_PREV_BTNS EQU $C810
VEC_COUNTERS EQU $C82E
Vec_Rfrsh_lo EQU $C83D
Clear_x_b EQU $F53F
Vec_RiseRun_Tmp EQU $C834
VEC_TEXT_HEIGHT EQU $C82A
Sound_Byte_raw EQU $F25B
NEW_HIGH_SCORE EQU $F8D8
VEC_EXPL_1 EQU $C858
Moveto_ix_7F EQU $F30C
VEC_RFRSH_HI EQU $C83E
GET_RISE_IDX EQU $F5D9
COMPARE_SCORE EQU $F8C7
Vec_Joy_Mux EQU $C81F
Moveto_x_7F EQU $F2F2
DRAW_GRID_VL EQU $FF9F
Vec_Expl_Flag EQU $C867
Init_Music_x EQU $F692
Mov_Draw_VLcs EQU $F3B5
Rise_Run_Y EQU $F601
Print_List_hw EQU $F385
RESET0REF EQU $F354
RISE_RUN_Y EQU $F601
Print_Str_yx EQU $F378
DO_SOUND_X EQU $F28C
VEC_MUSIC_CHAN EQU $C855
Init_OS_RAM EQU $F164
Obj_Hit EQU $F8FF
DELAY_3 EQU $F56D
INTENSITY_3F EQU $F2A1
MUSIC7 EQU $FEC6
Draw_VLp_scale EQU $F40C
VEC_SWI_VECTOR EQU $CBFB
Do_Sound_x EQU $F28C
VEC_MUSIC_WK_A EQU $C842
MUSIC5 EQU $FE38
Random EQU $F517
Draw_Pat_VL_a EQU $F434
VEC_JOY_RESLTN EQU $C81A
Mov_Draw_VLc_a EQU $F3AD
Vec_Loop_Count EQU $C825
MOV_DRAW_VLCS EQU $F3B5
Print_List_chk EQU $F38C
VEC_EXPL_CHANA EQU $C853
CLEAR_X_256 EQU $F545
JOY_ANALOG EQU $F1F5
DRAW_VL_AB EQU $F3D8
Vec_Music_Ptr EQU $C853
DRAW_VL_B EQU $F3D2
VEC_RFRSH_LO EQU $C83D
RESET0INT EQU $F36B
Strip_Zeros EQU $F8B7
VEC_JOY_2_X EQU $C81D
DELAY_0 EQU $F579
VEC_BUTTON_2_1 EQU $C816
DOT_HERE EQU $F2C5
Dot_here EQU $F2C5
Rot_VL EQU $F616
INIT_MUSIC EQU $F68D
music1 EQU $FD0D
Vec_Num_Players EQU $C879
Moveto_d_7F EQU $F2FC
music3 EQU $FD81
Move_Mem_a EQU $F683
VEC_BUTTON_2_4 EQU $C819
SOUND_BYTES_X EQU $F284
Print_Str_hwyx EQU $F373
Vec_Button_2_2 EQU $C817
MOD16.MOD16_END EQU $402D
VEC_NUM_PLAYERS EQU $C879
VEC_MUSIC_WK_5 EQU $C847
Vec_Button_1_4 EQU $C815
CHECK0REF EQU $F34F
PRINT_TEXT_STR_68624562 EQU $4032
DRAW_VLP_7F EQU $F408
MOVETO_IX EQU $F310
Print_Str EQU $F495
Warm_Start EQU $F06C
INIT_VIA EQU $F14C
musicd EQU $FF8F
VEC_MUSIC_WK_6 EQU $C846
Vec_Max_Games EQU $C850
Print_List EQU $F38A
Sound_Bytes_x EQU $F284
Bitmask_a EQU $F57E
VEC_MUSIC_WK_1 EQU $C84B
Dot_ix_b EQU $F2BE
ROT_VL_MODE_A EQU $F61F
Compare_Score EQU $F8C7
Sound_Byte EQU $F256
Reset0Int EQU $F36B
Obj_Will_Hit EQU $F8F3
VEC_JOY_MUX_2_X EQU $C821
Joy_Digital EQU $F1F8
OBJ_WILL_HIT_U EQU $F8E5
Moveto_d EQU $F312
VEC_DURATION EQU $C857
VEC_STR_PTR EQU $C82C
VEC_ADSR_TIMERS EQU $C85E
Draw_VLp_FF EQU $F404
DRAW_VL_A EQU $F3DA
VEC_DEFAULT_STK EQU $CBEA
Get_Rise_Run EQU $F5EF
ROT_VL EQU $F616
Vec_Music_Wk_1 EQU $C84B
Delay_b EQU $F57A
music2 EQU $FD1D
Vec_Btn_State EQU $C80F
VEC_BUTTON_1_3 EQU $C814
Vec_Expl_Chan EQU $C85C
VEC_COLD_FLAG EQU $CBFE
VEC_BUTTON_1_1 EQU $C812
VEC_BTN_STATE EQU $C80F
XFORM_RISE EQU $F663
music6 EQU $FE76
Draw_VL_mode EQU $F46E
Vec_Joy_1_Y EQU $C81C
VEC_TEXT_HW EQU $C82A
MOVETO_IX_7F EQU $F30C
VEC_NUM_GAME EQU $C87A
Dec_Counters EQU $F563
DOT_LIST_RESET EQU $F2DE
VEC_MUSIC_FREQ EQU $C861
Add_Score_d EQU $F87C
Reset0Ref EQU $F354
MOVETO_D EQU $F312
music8 EQU $FEF8
Vec_Counter_3 EQU $C830
VEC_FREQ_TABLE EQU $C84D
Vec_NMI_Vector EQU $CBFB
Xform_Run_a EQU $F65B
VEC_MUSIC_TWANG EQU $C858
INTENSITY_5F EQU $F2A5
Print_Ships EQU $F393
VEC_RISERUN_LEN EQU $C83B
Delay_3 EQU $F56D
ROT_VL_MODE EQU $F62B
VEC_ADSR_TABLE EQU $C84F
Vec_ADSR_Timers EQU $C85E
Rise_Run_Angle EQU $F593
CLEAR_X_B_80 EQU $F550
Vec_Counter_1 EQU $C82E


;***************************************************************************
; CARTRIDGE HEADER
;***************************************************************************
    FCC "g GCE 2025"
    FCB $80                 ; String terminator
    FDB music1              ; Music pointer
    FCB $F8,$50,$20,$BB     ; Height, Width, Rel Y, Rel X
    FCC "MULTIBANK PDB TEST"
    FCB $80                 ; String terminator
    FCB 0                   ; End of header

;***************************************************************************
; CODE SECTION
;***************************************************************************

START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS
    CLR $C80E        ; Initialize Vec_Prev_Btns
    LDA #$80
    STA VIA_t1_cnt_lo
    LDS #$CBFF       ; Initialize stack
; Bank 0 ($0000) is active; fixed bank 31 ($4000-$7FFF) always visible
    JMP MAIN

;***************************************************************************
; Internal builtin variables (aliases to RESULT slots)
DRAW_VEC_X EQU RESULT+0
DRAW_VEC_Y EQU RESULT+2
MIRROR_X EQU RESULT+4
MIRROR_Y EQU RESULT+6
DRAW_VEC_INTENSITY EQU RESULT+8


;***************************************************************************
; MAIN PROGRAM
;***************************************************************************

MAIN:
    ; Initialize global variables
    ; Call main() for initialization
    ; SET_INTENSITY: Set drawing intensity
    LDD #127
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT

.MAIN_LOOP:
    JSR LOOP_BODY
    LBRA .MAIN_LOOP

LOOP_BODY:
    JSR Wait_Recal   ; Synchronize with screen refresh (mandatory)
    JSR Reset0Ref    ; Reset beam to center (0,0)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; PRINT_TEXT: Print text at position
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_68624562      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    ; PRINT_TEXT: Print text at position
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_82781042      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    RTS


; ================================================
