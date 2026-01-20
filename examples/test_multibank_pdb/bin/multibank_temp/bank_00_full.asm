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
DRAW_LINE_ARGS       EQU $C880+$08   ; DRAW_LINE argument buffer (x0,y0,x1,y1,intensity) (10 bytes)
VLINE_DX_16          EQU $C880+$12   ; DRAW_LINE dx (16-bit) (2 bytes)
VLINE_DY_16          EQU $C880+$14   ; DRAW_LINE dy (16-bit) (2 bytes)
VLINE_DX             EQU $C880+$16   ; DRAW_LINE dx clamped (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$17   ; DRAW_LINE dy clamped (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$18   ; DRAW_LINE remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$1A   ; DRAW_LINE remaining dx for segment 2 (16-bit) (2 bytes)
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
Move_Mem_a EQU $F683
DRAW_PAT_VL_D EQU $F439
VEC_LOOP_COUNT EQU $C825
musicc EQU $FF7A
XFORM_RISE EQU $F663
MOVETO_IX_7F EQU $F30C
XFORM_RISE_A EQU $F661
DRAW_VLP_7F EQU $F408
Vec_Button_2_4 EQU $C819
Delay_1 EQU $F575
music6 EQU $FE76
Mov_Draw_VL_a EQU $F3B9
VEC_JOY_1_X EQU $C81B
OBJ_WILL_HIT EQU $F8F3
Vec_SWI2_Vector EQU $CBF2
VEC_RANDOM_SEED EQU $C87D
Vec_Joy_2_Y EQU $C81E
Move_Mem_a_1 EQU $F67F
Clear_Score EQU $F84F
MUSIC3 EQU $FD81
VEC_BUTTON_1_1 EQU $C812
Rise_Run_Angle EQU $F593
RISE_RUN_ANGLE EQU $F593
DRAW_VLP_B EQU $F40E
VEC_RFRSH EQU $C83D
Draw_VL EQU $F3DD
Dot_List EQU $F2D5
Clear_x_b_a EQU $F552
CLEAR_X_D EQU $F548
Vec_Music_Wk_A EQU $C842
music3 EQU $FD81
CLEAR_X_B_A EQU $F552
Joy_Analog EQU $F1F5
DRAW_VLP_FF EQU $F404
VEC_DEFAULT_STK EQU $CBEA
Vec_FIRQ_Vector EQU $CBF5
VEC_SEED_PTR EQU $C87B
VEC_BUTTON_1_2 EQU $C813
PRINT_STR_YX EQU $F378
Sound_Byte EQU $F256
Vec_Music_Ptr EQU $C853
VEC_BUTTON_1_3 EQU $C814
ABS_B EQU $F58B
Reset0Int EQU $F36B
Draw_Pat_VL_d EQU $F439
VEC_NUM_GAME EQU $C87A
VEC_JOY_MUX_2_X EQU $C821
VEC_EXPL_1 EQU $C858
VEC_ADSR_TABLE EQU $C84F
DRAW_PAT_VL_A EQU $F434
MOD16.MOD16_LOOP EQU $401D
Add_Score_a EQU $F85E
Print_Str_d EQU $F37A
EXPLOSION_SND EQU $F92E
VEC_BUTTONS EQU $C811
PRINT_STR_D EQU $F37A
Print_Str_yx EQU $F378
music4 EQU $FDD3
MOVETO_D_7F EQU $F2FC
VEC_MUSIC_WK_5 EQU $C847
VEC_HIGH_SCORE EQU $CBEB
Draw_VLcs EQU $F3D6
ADD_SCORE_A EQU $F85E
musica EQU $FF44
Vec_Counter_1 EQU $C82E
Print_Str EQU $F495
Vec_Max_Games EQU $C850
VEC_PATTERN EQU $C829
Print_List_chk EQU $F38C
DOT_D EQU $F2C3
Vec_High_Score EQU $CBEB
Abs_a_b EQU $F584
Moveto_d EQU $F312
GET_RISE_RUN EQU $F5EF
JOY_DIGITAL EQU $F1F8
Mov_Draw_VL_ab EQU $F3B7
PRINT_STR EQU $F495
Delay_3 EQU $F56D
Obj_Will_Hit_u EQU $F8E5
VEC_MUSIC_WK_1 EQU $C84B
Vec_Duration EQU $C857
Vec_Button_1_3 EQU $C814
Vec_Joy_Mux_1_Y EQU $C820
DRAW_VL_AB EQU $F3D8
VEC_RFRSH_LO EQU $C83D
VEC_SND_SHADOW EQU $C800
DO_SOUND_X EQU $F28C
DRAW_VL_MODE EQU $F46E
ABS_A_B EQU $F584
DP_to_C8 EQU $F1AF
Vec_Joy_Resltn EQU $C81A
Warm_Start EQU $F06C
Vec_Joy_Mux_2_X EQU $C821
RESET_PEN EQU $F35B
Vec_Music_Wk_5 EQU $C847
MOVE_MEM_A EQU $F683
VEC_TWANG_TABLE EQU $C851
Vec_Expl_2 EQU $C859
VEC_DURATION EQU $C857
Draw_VLp EQU $F410
Draw_Pat_VL EQU $F437
Rot_VL_Mode EQU $F62B
Delay_2 EQU $F571
Delay_RTS EQU $F57D
musicd EQU $FF8F
Dec_3_Counters EQU $F55A
VEC_JOY_MUX_1_Y EQU $C820
Vec_Expl_Chans EQU $C854
VEC_MUSIC_TWANG EQU $C858
Vec_Joy_Mux_1_X EQU $C81F
MOV_DRAW_VLCS EQU $F3B5
Vec_Max_Players EQU $C84F
Mov_Draw_VL_d EQU $F3BE
MOV_DRAW_VL_B EQU $F3B1
Clear_C8_RAM EQU $F542
DOT_IX EQU $F2C1
INIT_OS_RAM EQU $F164
MOVETO_X_7F EQU $F2F2
MOV_DRAW_VL_AB EQU $F3B7
Vec_Button_2_1 EQU $C816
VEC_SWI_VECTOR EQU $CBFB
MOV_DRAW_VL_D EQU $F3BE
VEC_BUTTON_2_3 EQU $C818
DELAY_3 EQU $F56D
Do_Sound_x EQU $F28C
VEC_JOY_MUX EQU $C81F
Vec_Num_Game EQU $C87A
Vec_Str_Ptr EQU $C82C
Vec_Counter_5 EQU $C832
MOVETO_D EQU $F312
Vec_Button_1_4 EQU $C815
Print_Ships EQU $F393
BITMASK_A EQU $F57E
DEC_3_COUNTERS EQU $F55A
VEC_MAX_PLAYERS EQU $C84F
Vec_Btn_State EQU $C80F
Clear_x_d EQU $F548
PRINT_SHIPS_X EQU $F391
Rise_Run_Len EQU $F603
DOT_LIST_RESET EQU $F2DE
VEC_COUNTER_5 EQU $C832
Draw_VL_b EQU $F3D2
DELAY_RTS EQU $F57D
Vec_Dot_Dwell EQU $C828
VEC_ADSR_TIMERS EQU $C85E
INTENSITY_7F EQU $F2A9
Vec_Num_Players EQU $C879
Strip_Zeros EQU $F8B7
INTENSITY_A EQU $F2AB
Vec_Expl_Timer EQU $C877
Init_Music_chk EQU $F687
VEC_FREQ_TABLE EQU $C84D
INTENSITY_1F EQU $F29D
VEC_BRIGHTNESS EQU $C827
Vec_Music_Chan EQU $C855
Vec_Rfrsh_hi EQU $C83E
Draw_VL_mode EQU $F46E
Dot_ix EQU $F2C1
Vec_RiseRun_Len EQU $C83B
DRAW_VLCS EQU $F3D6
music5 EQU $FE38
Vec_Joy_1_Y EQU $C81C
VEC_JOY_MUX_1_X EQU $C81F
Clear_x_b_80 EQU $F550
GET_RISE_IDX EQU $F5D9
COMPARE_SCORE EQU $F8C7
Draw_VLp_scale EQU $F40C
CLEAR_SCORE EQU $F84F
Vec_Music_Wk_1 EQU $C84B
Vec_Music_Wk_7 EQU $C845
DOT_IX_B EQU $F2BE
Draw_Line_d EQU $F3DF
COLD_START EQU $F000
CLEAR_C8_RAM EQU $F542
Select_Game EQU $F7A9
Explosion_Snd EQU $F92E
VEC_MUSIC_PTR EQU $C853
ROT_VL_MODE EQU $F62B
WAIT_RECAL EQU $F192
VEC_0REF_ENABLE EQU $C824
DP_to_D0 EQU $F1AA
Vec_Counter_4 EQU $C831
INIT_MUSIC_BUF EQU $F533
MUSICA EQU $FF44
PRINT_TEXT_STR_68624562 EQU $403A
MOD16 EQU $401B
PRINT_SHIPS EQU $F393
music7 EQU $FEC6
Xform_Rise_a EQU $F661
READ_BTNS EQU $F1BA
Dec_Counters EQU $F563
INIT_MUSIC_X EQU $F692
DP_TO_C8 EQU $F1AF
DRAW_LINE_D EQU $F3DF
Vec_Counter_3 EQU $C830
Vec_Counter_6 EQU $C833
INIT_MUSIC EQU $F68D
Vec_Rfrsh EQU $C83D
STRIP_ZEROS EQU $F8B7
MUSIC7 EQU $FEC6
INIT_MUSIC_CHK EQU $F687
MOV_DRAW_VL EQU $F3BC
VEC_JOY_MUX_2_Y EQU $C822
Rot_VL EQU $F616
Vec_IRQ_Vector EQU $CBF8
Reset_Pen EQU $F35B
VEC_JOY_2_X EQU $C81D
DRAW_VLC EQU $F3CE
DRAW_VL EQU $F3DD
Joy_Digital EQU $F1F8
Vec_Prev_Btns EQU $C810
VEC_JOY_RESLTN EQU $C81A
Rot_VL_dft EQU $F637
Vec_Music_Flag EQU $C856
VEC_EXPL_4 EQU $C85B
SOUND_BYTES_X EQU $F284
VEC_IRQ_VECTOR EQU $CBF8
RESET0REF EQU $F354
SOUND_BYTES EQU $F27D
Print_List_hw EQU $F385
VEC_COUNTERS EQU $C82E
XFORM_RUN EQU $F65D
OBJ_WILL_HIT_U EQU $F8E5
Vec_Pattern EQU $C829
Vec_Default_Stk EQU $CBEA
VEC_TEXT_WIDTH EQU $C82B
DO_SOUND EQU $F289
Moveto_ix_FF EQU $F308
Init_OS EQU $F18B
Intensity_1F EQU $F29D
DRAW_VLP EQU $F410
Vec_Joy_Mux_2_Y EQU $C822
Vec_Counters EQU $C82E
DRAW_VLP_SCALE EQU $F40C
Vec_Snd_Shadow EQU $C800
Intensity_5F EQU $F2A5
Delay_b EQU $F57A
DELAY_1 EQU $F575
DRAW_VL_B EQU $F3D2
ROT_VL_MODE_A EQU $F61F
DOT_HERE EQU $F2C5
Vec_Button_1_1 EQU $C812
Draw_Pat_VL_a EQU $F434
MUSICC EQU $FF7A
Vec_Buttons EQU $C811
Add_Score_d EQU $F87C
VEC_JOY_2_Y EQU $C81E
PRINT_LIST_CHK EQU $F38C
VEC_COUNTER_2 EQU $C82F
Wait_Recal EQU $F192
Vec_Text_Height EQU $C82A
Vec_Twang_Table EQU $C851
INIT_VIA EQU $F14C
Vec_SWI_Vector EQU $CBFB
XFORM_RUN_A EQU $F65B
Xform_Run_a EQU $F65B
Vec_Music_Freq EQU $C861
Obj_Hit EQU $F8FF
VEC_MUSIC_CHAN EQU $C855
MUSIC6 EQU $FE76
MOV_DRAW_VL_A EQU $F3B9
Vec_Joy_1_X EQU $C81B
RISE_RUN_X EQU $F5FF
VEC_MAX_GAMES EQU $C850
Reset0Ref EQU $F354
GET_RUN_IDX EQU $F5DB
DRAW_GRID_VL EQU $FF9F
Init_VIA EQU $F14C
Vec_NMI_Vector EQU $CBFB
Sound_Byte_raw EQU $F25B
MOVETO_IX_FF EQU $F308
VEC_NUM_PLAYERS EQU $C879
SELECT_GAME EQU $F7A9
Vec_Text_HW EQU $C82A
Vec_Expl_3 EQU $C85A
Vec_Loop_Count EQU $C825
VEC_PREV_BTNS EQU $C810
MUSIC5 EQU $FE38
VEC_JOY_1_Y EQU $C81C
DELAY_0 EQU $F579
SOUND_BYTE EQU $F256
VEC_FIRQ_VECTOR EQU $CBF5
MUSIC8 EQU $FEF8
music9 EQU $FF26
Clear_x_b EQU $F53F
DP_TO_D0 EQU $F1AA
Intensity_a EQU $F2AB
Rot_VL_Mode_a EQU $F61F
Draw_VLp_b EQU $F40E
Draw_VLp_7F EQU $F408
INTENSITY_3F EQU $F2A1
Get_Run_Idx EQU $F5DB
Dot_d EQU $F2C3
Get_Rise_Idx EQU $F5D9
VEC_RISERUN_TMP EQU $C834
MOV_DRAW_VLC_A EQU $F3AD
Xform_Rise EQU $F663
MOVETO_IX_A EQU $F30E
Get_Rise_Run EQU $F5EF
Set_Refresh EQU $F1A2
VEC_BUTTON_2_4 EQU $C819
MOD16.MOD16_END EQU $4035
Vec_0Ref_Enable EQU $C824
Delay_0 EQU $F579
Recalibrate EQU $F2E6
VEC_COLD_FLAG EQU $CBFE
Vec_SWI3_Vector EQU $CBF2
PRINT_TEXT_STR_82781042 EQU $4040
Dot_here EQU $F2C5
music8 EQU $FEF8
Vec_ADSR_Timers EQU $C85E
DOT_LIST EQU $F2D5
READ_BTNS_MASK EQU $F1B4
Random EQU $F517
Print_Ships_x EQU $F391
VEC_EXPL_CHANB EQU $C85D
VEC_MUSIC_FLAG EQU $C856
JOY_ANALOG EQU $F1F5
CLEAR_X_256 EQU $F545
VEC_COUNTER_1 EQU $C82E
PRINT_LIST EQU $F38A
Abs_b EQU $F58B
New_High_Score EQU $F8D8
SOUND_BYTE_X EQU $F259
VEC_SWI3_VECTOR EQU $CBF2
Mov_Draw_VLcs EQU $F3B5
VEC_MUSIC_WK_A EQU $C842
Mov_Draw_VL EQU $F3BC
MUSICD EQU $FF8F
Vec_Expl_1 EQU $C858
VEC_EXPL_FLAG EQU $C867
Vec_Joy_Mux EQU $C81F
MUSIC9 EQU $FF26
Dot_ix_b EQU $F2BE
DEC_6_COUNTERS EQU $F55E
CLEAR_X_B_80 EQU $F550
Vec_Cold_Flag EQU $CBFE
VEC_BUTTON_2_1 EQU $C816
VEC_TEXT_HEIGHT EQU $C82A
Draw_Grid_VL EQU $FF9F
Moveto_d_7F EQU $F2FC
Vec_Angle EQU $C836
Vec_Rise_Index EQU $C839
VEC_TEXT_HW EQU $C82A
Sound_Bytes EQU $F27D
RANDOM_3 EQU $F511
Vec_Expl_ChanB EQU $C85D
Draw_VLp_FF EQU $F404
PRINT_LIST_HW EQU $F385
VEC_COUNTER_4 EQU $C831
Vec_Text_Width EQU $C82B
Init_OS_RAM EQU $F164
Rot_VL_ab EQU $F610
RESET0INT EQU $F36B
VEC_EXPL_3 EQU $C85A
DEC_COUNTERS EQU $F563
VEC_RUN_INDEX EQU $C837
Read_Btns_Mask EQU $F1B4
MUSIC4 EQU $FDD3
Obj_Will_Hit EQU $F8F3
VEC_RISERUN_LEN EQU $C83B
VEC_SWI2_VECTOR EQU $CBF2
SOUND_BYTE_RAW EQU $F25B
Vec_Run_Index EQU $C837
ADD_SCORE_D EQU $F87C
Init_Music EQU $F68D
Do_Sound EQU $F289
Moveto_ix_7F EQU $F30C
DRAW_VL_A EQU $F3DA
Vec_Joy_2_X EQU $C81D
INTENSITY_5F EQU $F2A5
VEC_DOT_DWELL EQU $C828
Check0Ref EQU $F34F
VEC_EXPL_CHANA EQU $C853
RESET0REF_D0 EQU $F34A
Vec_Brightness EQU $C827
VEC_EXPL_TIMER EQU $C877
CHECK0REF EQU $F34F
VEC_MUSIC_WK_6 EQU $C846
Vec_Button_2_3 EQU $C818
VEC_RISE_INDEX EQU $C839
ROT_VL EQU $F616
Sound_Byte_x EQU $F259
VEC_RFRSH_HI EQU $C83E
CLEAR_X_B EQU $F53F
Rise_Run_Y EQU $F601
Print_Str_hwyx EQU $F373
Xform_Run EQU $F65D
PRINT_STR_HWYX EQU $F373
RECALIBRATE EQU $F2E6
Vec_ADSR_Table EQU $C84F
Vec_Random_Seed EQU $C87D
RISE_RUN_Y EQU $F601
VEC_BTN_STATE EQU $C80F
RISE_RUN_LEN EQU $F603
VECTREX_PRINT_TEXT EQU $4000
Vec_Seed_Ptr EQU $C87B
Vec_RiseRun_Tmp EQU $C834
DELAY_B EQU $F57A
VEC_MUSIC_FREQ EQU $C861
Dot_List_Reset EQU $F2DE
RANDOM EQU $F517
Rise_Run_X EQU $F5FF
Dec_6_Counters EQU $F55E
music2 EQU $FD1D
Vec_Counter_2 EQU $C82F
Moveto_ix_a EQU $F30E
Bitmask_a EQU $F57E
VEC_ANGLE EQU $C836
WARM_START EQU $F06C
VEC_EXPL_2 EQU $C859
Vec_Rfrsh_lo EQU $C83D
Vec_Music_Work EQU $C83F
Vec_Expl_Chan EQU $C85C
Compare_Score EQU $F8C7
VEC_EXPL_CHANS EQU $C854
INIT_OS EQU $F18B
MUSICB EQU $FF62
VEC_BUTTON_2_2 EQU $C817
Init_Music_Buf EQU $F533
Vec_Expl_4 EQU $C85B
Mov_Draw_VLc_a EQU $F3AD
Intensity_3F EQU $F2A1
NEW_HIGH_SCORE EQU $F8D8
Vec_Music_Twang EQU $C858
DRAW_PAT_VL EQU $F437
musicb EQU $FF62
Vec_Freq_Table EQU $C84D
Vec_Button_2_2 EQU $C817
VEC_MUSIC_WORK EQU $C83F
MUSIC1 EQU $FD0D
Read_Btns EQU $F1BA
Vec_Misc_Count EQU $C823
Sound_Bytes_x EQU $F284
VEC_EXPL_CHAN EQU $C85C
Init_Music_x EQU $F692
Clear_Sound EQU $F272
Moveto_x_7F EQU $F2F2
Clear_x_256 EQU $F545
Vec_Music_Wk_6 EQU $C846
VEC_STR_PTR EQU $C82C
Reset0Ref_D0 EQU $F34A
Draw_VL_ab EQU $F3D8
CLEAR_SOUND EQU $F272
SET_REFRESH EQU $F1A2
Mov_Draw_VL_b EQU $F3B1
MOVETO_IX EQU $F310
Cold_Start EQU $F000
OBJ_HIT EQU $F8FF
ROT_VL_AB EQU $F610
Random_3 EQU $F511
ROT_VL_DFT EQU $F637
VEC_NMI_VECTOR EQU $CBFB
Moveto_ix EQU $F310
Draw_VLc EQU $F3CE
Print_List EQU $F38A
Vec_Expl_ChanA EQU $C853
VEC_COUNTER_6 EQU $C833
Vec_Expl_Flag EQU $C867
Intensity_7F EQU $F2A9
VEC_MISC_COUNT EQU $C823
Vec_Button_1_2 EQU $C813
VEC_COUNTER_3 EQU $C830
VEC_BUTTON_1_4 EQU $C815
DELAY_2 EQU $F571
MOVE_MEM_A_1 EQU $F67F
music1 EQU $FD0D
Draw_VL_a EQU $F3DA
MUSIC2 EQU $FD1D
VEC_MUSIC_WK_7 EQU $C845


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

;***************************************************************************
; MAIN PROGRAM
;***************************************************************************

MAIN:
    ; Initialize global variables
    ; === Initialize Joystick (one-time setup) ===
    JSR $F1AF    ; DP_to_C8 (required for RAM access)
    CLR $C823    ; CRITICAL: Clear analog mode flag (Joy_Analog does DEC on this)
    LDA #$01     ; CRITICAL: Resolution threshold (power of 2: $40=fast, $01=accurate)
    STA $C81A    ; Vec_Joy_Resltn (loop terminates when B=this value after LSRBs)
    LDA #$01
    STA $C81F    ; Vec_Joy_Mux_1_X (enable X axis reading)
    LDA #$03
    STA $C820    ; Vec_Joy_Mux_1_Y (enable Y axis reading)
    LDA #$00
    STA $C821    ; Vec_Joy_Mux_2_X (disable joystick 2 - CRITICAL!)
    STA $C822    ; Vec_Joy_Mux_2_Y (disable joystick 2 - saves cycles)
    ; Mux configured - J1_X()/J1_Y() can now be called

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
