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
Vec_Max_Games EQU $C850
MOV_DRAW_VL_AB EQU $F3B7
Vec_Joy_1_Y EQU $C81C
Vec_Str_Ptr EQU $C82C
MUSIC9 EQU $FF26
DRAW_VLP_7F EQU $F408
Add_Score_d EQU $F87C
Rot_VL_dft EQU $F637
VEC_TEXT_HW EQU $C82A
Print_List EQU $F38A
Vec_Dot_Dwell EQU $C828
Abs_b EQU $F58B
MUSICA EQU $FF44
Dot_ix_b EQU $F2BE
VEC_ADSR_TABLE EQU $C84F
Vec_SWI_Vector EQU $CBFB
VEC_FIRQ_VECTOR EQU $CBF5
Select_Game EQU $F7A9
GET_RUN_IDX EQU $F5DB
READ_BTNS_MASK EQU $F1B4
Reset0Int EQU $F36B
Vec_Misc_Count EQU $C823
Init_OS EQU $F18B
Intensity_5F EQU $F2A5
Moveto_x_7F EQU $F2F2
VEC_EXPL_4 EQU $C85B
RESET0REF EQU $F354
VEC_MUSIC_WK_A EQU $C842
INTENSITY_A EQU $F2AB
STRIP_ZEROS EQU $F8B7
Vec_Snd_Shadow EQU $C800
MOV_DRAW_VL_A EQU $F3B9
VEC_JOY_RESLTN EQU $C81A
Dot_d EQU $F2C3
VEC_BUTTON_1_2 EQU $C813
Vec_Rise_Index EQU $C839
VEC_TEXT_HEIGHT EQU $C82A
Clear_x_b_80 EQU $F550
MUSIC5 EQU $FE38
VEC_RUN_INDEX EQU $C837
Draw_VLp_FF EQU $F404
VEC_JOY_1_X EQU $C81B
VEC_MUSIC_WK_1 EQU $C84B
VEC_MISC_COUNT EQU $C823
VEC_IRQ_VECTOR EQU $CBF8
music1 EQU $FD0D
MUSIC8 EQU $FEF8
Check0Ref EQU $F34F
INIT_MUSIC_X EQU $F692
musicd EQU $FF8F
PRINT_LIST EQU $F38A
XFORM_RUN_A EQU $F65B
WAIT_RECAL EQU $F192
Delay_2 EQU $F571
SOUND_BYTES_X EQU $F284
Vec_Button_2_1 EQU $C816
MOVETO_IX EQU $F310
MUSICD EQU $FF8F
Reset0Ref_D0 EQU $F34A
PRINT_STR_D EQU $F37A
Vec_NMI_Vector EQU $CBFB
Vec_ADSR_Table EQU $C84F
Draw_VL_mode EQU $F46E
Mov_Draw_VLcs EQU $F3B5
Mov_Draw_VL_ab EQU $F3B7
Print_Str_hwyx EQU $F373
SOUND_BYTE EQU $F256
Draw_Grid_VL EQU $FF9F
VEC_FREQ_TABLE EQU $C84D
DRAW_GRID_VL EQU $FF9F
Dot_ix EQU $F2C1
Init_OS_RAM EQU $F164
DRAW_PAT_VL EQU $F437
Sound_Byte_x EQU $F259
CLEAR_X_B EQU $F53F
Delay_0 EQU $F579
Vec_Num_Players EQU $C879
Vec_Expl_ChanB EQU $C85D
MOVETO_X_7F EQU $F2F2
music5 EQU $FE38
Vec_Music_Wk_A EQU $C842
RISE_RUN_LEN EQU $F603
Vec_ADSR_Timers EQU $C85E
Read_Btns EQU $F1BA
VEC_JOY_2_Y EQU $C81E
Vec_Seed_Ptr EQU $C87B
VEC_BTN_STATE EQU $C80F
VEC_BUTTON_2_3 EQU $C818
VEC_MUSIC_CHAN EQU $C855
Vec_Text_Height EQU $C82A
Vec_Num_Game EQU $C87A
Vec_Joy_1_X EQU $C81B
Clear_C8_RAM EQU $F542
DOT_D EQU $F2C3
PRINT_SHIPS EQU $F393
Draw_Pat_VL_d EQU $F439
Sound_Byte_raw EQU $F25B
VEC_TWANG_TABLE EQU $C851
Rot_VL_ab EQU $F610
INIT_OS_RAM EQU $F164
PRINT_STR EQU $F495
Print_Str_d EQU $F37A
DOT_LIST EQU $F2D5
INIT_VIA EQU $F14C
OBJ_WILL_HIT_U EQU $F8E5
Vec_IRQ_Vector EQU $CBF8
ROT_VL_MODE EQU $F62B
Vec_Max_Players EQU $C84F
Sound_Bytes EQU $F27D
Explosion_Snd EQU $F92E
VEC_MUSIC_TWANG EQU $C858
Warm_Start EQU $F06C
WARM_START EQU $F06C
VEC_BUTTON_2_4 EQU $C819
Rise_Run_X EQU $F5FF
CLEAR_X_B_A EQU $F552
Vec_Buttons EQU $C811
Rise_Run_Len EQU $F603
DO_SOUND_X EQU $F28C
Clear_x_b EQU $F53F
Vec_Button_1_3 EQU $C814
VEC_SWI_VECTOR EQU $CBFB
MOVETO_IX_A EQU $F30E
Vec_Joy_Mux_2_Y EQU $C822
INTENSITY_1F EQU $F29D
VEC_BUTTON_1_4 EQU $C815
MUSIC3 EQU $FD81
Draw_VL_b EQU $F3D2
Obj_Hit EQU $F8FF
Init_Music_Buf EQU $F533
Rot_VL_Mode_a EQU $F61F
DRAW_VL EQU $F3DD
MOVETO_D_7F EQU $F2FC
COLD_START EQU $F000
Draw_VLp_7F EQU $F408
Vec_Joy_2_X EQU $C81D
Reset_Pen EQU $F35B
Draw_VL EQU $F3DD
Intensity_1F EQU $F29D
Vec_Twang_Table EQU $C851
MUSICB EQU $FF62
Delay_b EQU $F57A
Vec_Button_1_2 EQU $C813
Vec_Joy_2_Y EQU $C81E
VEC_MUSIC_WK_6 EQU $C846
Print_Ships_x EQU $F391
Mov_Draw_VL_d EQU $F3BE
XFORM_RISE EQU $F663
Moveto_d_7F EQU $F2FC
ADD_SCORE_A EQU $F85E
MUSIC4 EQU $FDD3
Vec_Expl_Chans EQU $C854
ABS_A_B EQU $F584
VEC_LOOP_COUNT EQU $C825
Vec_Joy_Mux_2_X EQU $C821
Vec_Expl_ChanA EQU $C853
DRAW_VL_MODE EQU $F46E
Init_Music_chk EQU $F687
Vec_Duration EQU $C857
Joy_Analog EQU $F1F5
Do_Sound_x EQU $F28C
Clear_Score EQU $F84F
Vec_Expl_Flag EQU $C867
DELAY_2 EQU $F571
Draw_VLp_b EQU $F40E
Vec_Button_2_2 EQU $C817
MOD16.MOD16_END EQU $4035
Mov_Draw_VL_b EQU $F3B1
Vec_Music_Chan EQU $C855
VEC_MUSIC_PTR EQU $C853
Vec_Brightness EQU $C827
VEC_EXPL_2 EQU $C859
DRAW_VLP_SCALE EQU $F40C
Rot_VL EQU $F616
CLEAR_SOUND EQU $F272
MUSIC1 EQU $FD0D
Clear_x_256 EQU $F545
music7 EQU $FEC6
XFORM_RISE_A EQU $F661
Sound_Bytes_x EQU $F284
Draw_VLp_scale EQU $F40C
VEC_MUSIC_FLAG EQU $C856
Print_List_chk EQU $F38C
Vec_Angle EQU $C836
DRAW_VLC EQU $F3CE
VEC_EXPL_CHANB EQU $C85D
INIT_OS EQU $F18B
VEC_COUNTER_3 EQU $C830
Vec_RiseRun_Len EQU $C83B
GET_RISE_IDX EQU $F5D9
INTENSITY_7F EQU $F2A9
Moveto_ix_a EQU $F30E
MUSIC2 EQU $FD1D
DRAW_VL_AB EQU $F3D8
VEC_PATTERN EQU $C829
Bitmask_a EQU $F57E
MUSICC EQU $FF7A
MOV_DRAW_VL EQU $F3BC
Move_Mem_a EQU $F683
NEW_HIGH_SCORE EQU $F8D8
RISE_RUN_Y EQU $F601
DO_SOUND EQU $F289
PRINT_LIST_HW EQU $F385
PRINT_TEXT_STR_68624562 EQU $403A
MOV_DRAW_VL_B EQU $F3B1
INTENSITY_3F EQU $F2A1
Dec_3_Counters EQU $F55A
Vec_Random_Seed EQU $C87D
Dec_6_Counters EQU $F55E
Vec_Counter_3 EQU $C830
Draw_Pat_VL EQU $F437
VEC_COUNTER_5 EQU $C832
DOT_IX EQU $F2C1
Vec_Joy_Resltn EQU $C81A
MOV_DRAW_VLCS EQU $F3B5
VEC_EXPL_CHANA EQU $C853
PRINT_STR_YX EQU $F378
MOVE_MEM_A EQU $F683
VEC_JOY_MUX_2_Y EQU $C822
Dec_Counters EQU $F563
BITMASK_A EQU $F57E
MOD16 EQU $401B
VEC_JOY_2_X EQU $C81D
Get_Rise_Idx EQU $F5D9
VEC_EXPL_1 EQU $C858
Vec_Rfrsh_hi EQU $C83E
JOY_ANALOG EQU $F1F5
Vec_Run_Index EQU $C837
EXPLOSION_SND EQU $F92E
Dot_here EQU $F2C5
Read_Btns_Mask EQU $F1B4
Draw_VL_a EQU $F3DA
music2 EQU $FD1D
musicb EQU $FF62
Vec_Counter_6 EQU $C833
Add_Score_a EQU $F85E
ROT_VL_DFT EQU $F637
VEC_JOY_MUX_1_X EQU $C81F
Xform_Run EQU $F65D
INIT_MUSIC_BUF EQU $F533
Clear_x_b_a EQU $F552
Xform_Rise EQU $F663
INIT_MUSIC_CHK EQU $F687
VEC_SWI3_VECTOR EQU $CBF2
Obj_Will_Hit_u EQU $F8E5
Init_VIA EQU $F14C
Vec_Expl_Timer EQU $C877
Vec_Default_Stk EQU $CBEA
Cold_Start EQU $F000
Print_List_hw EQU $F385
VEC_EXPL_3 EQU $C85A
PRINT_STR_HWYX EQU $F373
Init_Music_x EQU $F692
DRAW_VLP EQU $F410
Moveto_ix_FF EQU $F308
DRAW_VL_B EQU $F3D2
DRAW_VLCS EQU $F3D6
DELAY_B EQU $F57A
music4 EQU $FDD3
VEC_BUTTON_2_1 EQU $C816
CLEAR_X_256 EQU $F545
MOVETO_IX_FF EQU $F308
VEC_RISERUN_LEN EQU $C83B
New_High_Score EQU $F8D8
Vec_SWI2_Vector EQU $CBF2
Vec_Joy_Mux_1_Y EQU $C820
VEC_MUSIC_WK_5 EQU $C847
Set_Refresh EQU $F1A2
ADD_SCORE_D EQU $F87C
Vec_Button_1_4 EQU $C815
VEC_RISE_INDEX EQU $C839
Vec_Loop_Count EQU $C825
VEC_MAX_PLAYERS EQU $C84F
DRAW_LINE_D EQU $F3DF
MOVE_MEM_A_1 EQU $F67F
Vec_Counters EQU $C82E
MOVETO_D EQU $F312
Recalibrate EQU $F2E6
Vec_Music_Twang EQU $C858
Vec_Music_Wk_7 EQU $C845
Vec_Rfrsh_lo EQU $C83D
VEC_DURATION EQU $C857
PRINT_LIST_CHK EQU $F38C
VEC_COUNTER_2 EQU $C82F
Vec_Expl_1 EQU $C858
VEC_BUTTON_1_1 EQU $C812
VEC_RFRSH EQU $C83D
DRAW_PAT_VL_A EQU $F434
VEC_NMI_VECTOR EQU $CBFB
Intensity_3F EQU $F2A1
VEC_NUM_PLAYERS EQU $C879
ROT_VL EQU $F616
musica EQU $FF44
VEC_HIGH_SCORE EQU $CBEB
CLEAR_X_D EQU $F548
Vec_Expl_2 EQU $C859
Vec_Button_1_1 EQU $C812
RESET0REF_D0 EQU $F34A
DP_to_D0 EQU $F1AA
VEC_DEFAULT_STK EQU $CBEA
DELAY_0 EQU $F579
ROT_VL_AB EQU $F610
Wait_Recal EQU $F192
Rot_VL_Mode EQU $F62B
Moveto_ix_7F EQU $F30C
VEC_EXPL_TIMER EQU $C877
VEC_RISERUN_TMP EQU $C834
JOY_DIGITAL EQU $F1F8
SOUND_BYTE_X EQU $F259
CLEAR_C8_RAM EQU $F542
VEC_COUNTER_1 EQU $C82E
Draw_VLc EQU $F3CE
VEC_COUNTER_6 EQU $C833
Vec_High_Score EQU $CBEB
XFORM_RUN EQU $F65D
Vec_Rfrsh EQU $C83D
DP_TO_D0 EQU $F1AA
VEC_RFRSH_LO EQU $C83D
Vec_Text_Width EQU $C82B
Get_Run_Idx EQU $F5DB
Vec_Counter_4 EQU $C831
Sound_Byte EQU $F256
VEC_EXPL_CHANS EQU $C854
VEC_JOY_1_Y EQU $C81C
PRINT_TEXT_STR_82781042 EQU $4040
RESET0INT EQU $F36B
Moveto_ix EQU $F310
Vec_Music_Freq EQU $C861
Moveto_d EQU $F312
Clear_Sound EQU $F272
ROT_VL_MODE_A EQU $F61F
SET_REFRESH EQU $F1A2
CHECK0REF EQU $F34F
Xform_Rise_a EQU $F661
MOV_DRAW_VL_D EQU $F3BE
CLEAR_SCORE EQU $F84F
Vec_Counter_2 EQU $C82F
SOUND_BYTE_RAW EQU $F25B
GET_RISE_RUN EQU $F5EF
VEC_RFRSH_HI EQU $C83E
CLEAR_X_B_80 EQU $F550
INTENSITY_5F EQU $F2A5
COMPARE_SCORE EQU $F8C7
VEC_SND_SHADOW EQU $C800
Strip_Zeros EQU $F8B7
RECALIBRATE EQU $F2E6
Vec_SWI3_Vector EQU $CBF2
Vec_Prev_Btns EQU $C810
Obj_Will_Hit EQU $F8F3
RISE_RUN_ANGLE EQU $F593
SOUND_BYTES EQU $F27D
VEC_MUSIC_FREQ EQU $C861
Print_Str_yx EQU $F378
Reset0Ref EQU $F354
VEC_BUTTON_2_2 EQU $C817
DEC_COUNTERS EQU $F563
Draw_Pat_VL_a EQU $F434
VEC_COLD_FLAG EQU $CBFE
Dot_List_Reset EQU $F2DE
SELECT_GAME EQU $F7A9
Vec_Expl_3 EQU $C85A
Get_Rise_Run EQU $F5EF
Vec_Counter_5 EQU $C832
VEC_MUSIC_WORK EQU $C83F
RANDOM EQU $F517
Vec_Counter_1 EQU $C82E
Joy_Digital EQU $F1F8
VEC_SEED_PTR EQU $C87B
VEC_BUTTON_1_3 EQU $C814
VEC_JOY_MUX EQU $C81F
RESET_PEN EQU $F35B
VEC_ANGLE EQU $C836
VEC_MAX_GAMES EQU $C850
DP_to_C8 EQU $F1AF
DP_TO_C8 EQU $F1AF
Mov_Draw_VLc_a EQU $F3AD
music8 EQU $FEF8
Mov_Draw_VL_a EQU $F3B9
Vec_FIRQ_Vector EQU $CBF5
DRAW_VL_A EQU $F3DA
Intensity_a EQU $F2AB
VEC_PREV_BTNS EQU $C810
VEC_COUNTERS EQU $C82E
DELAY_3 EQU $F56D
Random_3 EQU $F511
RISE_RUN_X EQU $F5FF
MUSIC6 EQU $FE76
Rise_Run_Angle EQU $F593
ABS_B EQU $F58B
VEC_DOT_DWELL EQU $C828
DRAW_PAT_VL_D EQU $F439
Draw_Line_d EQU $F3DF
DEC_3_COUNTERS EQU $F55A
OBJ_HIT EQU $F8FF
musicc EQU $FF7A
Vec_Pattern EQU $C829
Delay_RTS EQU $F57D
Random EQU $F517
VEC_ADSR_TIMERS EQU $C85E
DRAW_VLP_B EQU $F40E
VEC_NUM_GAME EQU $C87A
Vec_Music_Ptr EQU $C853
RANDOM_3 EQU $F511
VEC_JOY_MUX_2_X EQU $C821
DELAY_RTS EQU $F57D
VEC_STR_PTR EQU $C82C
Vec_Joy_Mux_1_X EQU $C81F
Dot_List EQU $F2D5
music9 EQU $FF26
Vec_Joy_Mux EQU $C81F
DRAW_VLP_FF EQU $F404
VEC_0REF_ENABLE EQU $C824
Intensity_7F EQU $F2A9
Init_Music EQU $F68D
Draw_VL_ab EQU $F3D8
MOD16.MOD16_LOOP EQU $401D
VEC_EXPL_FLAG EQU $C867
VEC_COUNTER_4 EQU $C831
VEC_EXPL_CHAN EQU $C85C
INIT_MUSIC EQU $F68D
VEC_TEXT_WIDTH EQU $C82B
Draw_VLcs EQU $F3D6
DEC_6_COUNTERS EQU $F55E
VEC_JOY_MUX_1_Y EQU $C820
Do_Sound EQU $F289
Xform_Run_a EQU $F65B
DOT_LIST_RESET EQU $F2DE
OBJ_WILL_HIT EQU $F8F3
Clear_x_d EQU $F548
READ_BTNS EQU $F1BA
Vec_Expl_Chan EQU $C85C
DOT_IX_B EQU $F2BE
Rise_Run_Y EQU $F601
VEC_BRIGHTNESS EQU $C827
DELAY_1 EQU $F575
Vec_Freq_Table EQU $C84D
Abs_a_b EQU $F584
Vec_Expl_4 EQU $C85B
Vec_Music_Work EQU $C83F
Vec_Cold_Flag EQU $CBFE
MOVETO_IX_7F EQU $F30C
Mov_Draw_VL EQU $F3BC
Vec_0Ref_Enable EQU $C824
Vec_Music_Wk_5 EQU $C847
VEC_RANDOM_SEED EQU $C87D
PRINT_SHIPS_X EQU $F391
Delay_3 EQU $F56D
Vec_RiseRun_Tmp EQU $C834
Draw_VLp EQU $F410
MOV_DRAW_VLC_A EQU $F3AD
Vec_Music_Wk_1 EQU $C84B
Vec_Text_HW EQU $C82A
VECTREX_PRINT_TEXT EQU $4000
VEC_SWI2_VECTOR EQU $CBF2
Print_Str EQU $F495
Compare_Score EQU $F8C7
DOT_HERE EQU $F2C5
music3 EQU $FD81
Vec_Music_Flag EQU $C856
Vec_Button_2_3 EQU $C818
Delay_1 EQU $F575
Vec_Btn_State EQU $C80F
MUSIC7 EQU $FEC6
Move_Mem_a_1 EQU $F67F
Print_Ships EQU $F393
Vec_Button_2_4 EQU $C819
VEC_BUTTONS EQU $C811
music6 EQU $FE76
Vec_Music_Wk_6 EQU $C846
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
