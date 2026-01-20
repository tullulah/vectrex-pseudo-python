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
VAR_MY_VAR           EQU $C880+$1C   ; User variable: my_var (2 bytes)
VAR_VAL              EQU $C880+$1E   ; User variable: val (2 bytes)
VAR_MY_ARRAY         EQU $C880+$20   ; User variable: my_array (2 bytes)
VAR_MY_ARRAY_DATA    EQU $C880+$22   ; Mutable array 'my_array' data (4 elements x 2 bytes) (8 bytes)
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
DEC_3_COUNTERS EQU $F55A
MUSIC7 EQU $FEC6
VEC_EXPL_TIMER EQU $C877
VEC_PREV_BTNS EQU $C810
Vec_Joy_1_X EQU $C81B
Vec_Music_Chan EQU $C855
OBJ_HIT EQU $F8FF
Sound_Bytes_x EQU $F284
music2 EQU $FD1D
Reset0Ref EQU $F354
ADD_SCORE_A EQU $F85E
Vec_Joy_Mux_1_Y EQU $C820
Draw_Pat_VL_a EQU $F434
Dot_ix EQU $F2C1
PRINT_SHIPS EQU $F393
MOD16.MOD16_END EQU $401A
Vec_Cold_Flag EQU $CBFE
VEC_JOY_MUX_1_X EQU $C81F
Print_Str_hwyx EQU $F373
DRAW_GRID_VL EQU $FF9F
Vec_Music_Wk_1 EQU $C84B
ROT_VL EQU $F616
VEC_EXPL_CHAN EQU $C85C
Select_Game EQU $F7A9
Sound_Byte_x EQU $F259
VEC_JOY_MUX_2_X EQU $C821
VEC_RFRSH_HI EQU $C83E
VEC_JOY_MUX_1_Y EQU $C820
RANDOM EQU $F517
VEC_EXPL_CHANA EQU $C853
Mov_Draw_VLcs EQU $F3B5
SOUND_BYTE_X EQU $F259
DOT_D EQU $F2C3
Joy_Analog EQU $F1F5
Explosion_Snd EQU $F92E
Vec_Counter_6 EQU $C833
DELAY_B EQU $F57A
musicd EQU $FF8F
Xform_Rise_a EQU $F661
Print_List_chk EQU $F38C
RESET0REF EQU $F354
VEC_PATTERN EQU $C829
CLEAR_X_256 EQU $F545
Vec_Music_Wk_6 EQU $C846
VEC_EXPL_2 EQU $C859
Move_Mem_a EQU $F683
RESET0REF_D0 EQU $F34A
Clear_x_b_80 EQU $F550
INTENSITY_5F EQU $F2A5
Vec_Angle EQU $C836
MUSICB EQU $FF62
Vec_Music_Wk_7 EQU $C845
MOVETO_IX_FF EQU $F308
MUSIC3 EQU $FD81
Rise_Run_Angle EQU $F593
VEC_FREQ_TABLE EQU $C84D
DRAW_VLP_FF EQU $F404
VEC_0REF_ENABLE EQU $C824
Vec_Rfrsh_hi EQU $C83E
Vec_Joy_2_Y EQU $C81E
Dot_here EQU $F2C5
Init_Music EQU $F68D
DRAW_PAT_VL_A EQU $F434
Print_Str EQU $F495
DRAW_VLC EQU $F3CE
Reset0Ref_D0 EQU $F34A
DRAW_VL EQU $F3DD
CHECK0REF EQU $F34F
PRINT_LIST_HW EQU $F385
MOD16.MOD16_LOOP EQU $4002
OBJ_WILL_HIT_U EQU $F8E5
SET_REFRESH EQU $F1A2
Clear_x_b EQU $F53F
Reset0Int EQU $F36B
Vec_Button_1_2 EQU $C813
PRINT_STR EQU $F495
Draw_VLcs EQU $F3D6
Vec_High_Score EQU $CBEB
Vec_Button_2_3 EQU $C818
VEC_BUTTON_2_4 EQU $C819
Vec_FIRQ_Vector EQU $CBF5
DRAW_VL_MODE EQU $F46E
Vec_ADSR_Timers EQU $C85E
Intensity_5F EQU $F2A5
Intensity_a EQU $F2AB
Do_Sound EQU $F289
DEC_6_COUNTERS EQU $F55E
Check0Ref EQU $F34F
VEC_EXPL_CHANS EQU $C854
VEC_ADSR_TIMERS EQU $C85E
VEC_STR_PTR EQU $C82C
MUSICC EQU $FF7A
Xform_Run_a EQU $F65B
Vec_Text_Width EQU $C82B
VEC_MUSIC_PTR EQU $C853
Delay_2 EQU $F571
MOV_DRAW_VL_A EQU $F3B9
VEC_FIRQ_VECTOR EQU $CBF5
Mov_Draw_VL_b EQU $F3B1
Vec_Snd_Shadow EQU $C800
MUSICA EQU $FF44
Moveto_ix EQU $F310
Xform_Run EQU $F65D
DOT_LIST_RESET EQU $F2DE
Vec_Counter_1 EQU $C82E
Intensity_1F EQU $F29D
Wait_Recal EQU $F192
READ_BTNS EQU $F1BA
Clear_C8_RAM EQU $F542
DOT_LIST EQU $F2D5
Draw_VLp_7F EQU $F408
VEC_JOY_1_X EQU $C81B
RISE_RUN_LEN EQU $F603
VEC_DOT_DWELL EQU $C828
Vec_Pattern EQU $C829
Vec_Music_Freq EQU $C861
VEC_NUM_GAME EQU $C87A
Delay_0 EQU $F579
ROT_VL_DFT EQU $F637
VEC_RFRSH EQU $C83D
VEC_MUSIC_WK_A EQU $C842
MOV_DRAW_VL_D EQU $F3BE
Vec_Duration EQU $C857
music6 EQU $FE76
VEC_MUSIC_WORK EQU $C83F
Draw_Pat_VL_d EQU $F439
MOV_DRAW_VL_B EQU $F3B1
Vec_Music_Wk_5 EQU $C847
JOY_DIGITAL EQU $F1F8
Draw_VL_b EQU $F3D2
Compare_Score EQU $F8C7
Vec_Text_HW EQU $C82A
Mov_Draw_VLc_a EQU $F3AD
music4 EQU $FDD3
Vec_Button_2_4 EQU $C819
Draw_VL_mode EQU $F46E
New_High_Score EQU $F8D8
Dec_Counters EQU $F563
Move_Mem_a_1 EQU $F67F
Clear_Sound EQU $F272
MOVE_MEM_A_1 EQU $F67F
Add_Score_a EQU $F85E
VEC_COLD_FLAG EQU $CBFE
VEC_SND_SHADOW EQU $C800
ROT_VL_MODE EQU $F62B
Dot_d EQU $F2C3
Vec_Text_Height EQU $C82A
VEC_MUSIC_WK_7 EQU $C845
Vec_Expl_4 EQU $C85B
Vec_Counter_3 EQU $C830
Bitmask_a EQU $F57E
DRAW_VLCS EQU $F3D6
MOV_DRAW_VL EQU $F3BC
Xform_Rise EQU $F663
MUSIC6 EQU $FE76
DRAW_VL_A EQU $F3DA
DP_TO_D0 EQU $F1AA
VEC_HIGH_SCORE EQU $CBEB
DRAW_VL_AB EQU $F3D8
Init_OS_RAM EQU $F164
MUSIC2 EQU $FD1D
Rot_VL_ab EQU $F610
Draw_VLp_scale EQU $F40C
Vec_Twang_Table EQU $C851
RECALIBRATE EQU $F2E6
Vec_Joy_Mux_1_X EQU $C81F
VEC_BUTTON_1_2 EQU $C813
Dec_6_Counters EQU $F55E
VEC_RISERUN_TMP EQU $C834
Vec_Expl_ChanB EQU $C85D
VEC_MAX_GAMES EQU $C850
Delay_1 EQU $F575
RISE_RUN_Y EQU $F601
Vec_Expl_Chan EQU $C85C
CLEAR_C8_RAM EQU $F542
Get_Run_Idx EQU $F5DB
GET_RUN_IDX EQU $F5DB
VEC_SWI2_VECTOR EQU $CBF2
VEC_JOY_2_X EQU $C81D
Vec_Counter_4 EQU $C831
XFORM_RISE_A EQU $F661
Sound_Byte EQU $F256
INIT_VIA EQU $F14C
Moveto_x_7F EQU $F2F2
Draw_Pat_VL EQU $F437
Vec_Rfrsh EQU $C83D
Vec_Num_Game EQU $C87A
Mov_Draw_VL_a EQU $F3B9
Warm_Start EQU $F06C
BITMASK_A EQU $F57E
Print_Ships_x EQU $F391
VEC_BRIGHTNESS EQU $C827
VEC_MUSIC_WK_1 EQU $C84B
DRAW_PAT_VL_D EQU $F439
Mov_Draw_VL EQU $F3BC
Get_Rise_Run EQU $F5EF
JOY_ANALOG EQU $F1F5
VEC_COUNTER_6 EQU $C833
PRINT_STR_HWYX EQU $F373
Vec_Joy_Mux_2_X EQU $C821
Vec_Num_Players EQU $C879
COMPARE_SCORE EQU $F8C7
CLEAR_SCORE EQU $F84F
Intensity_7F EQU $F2A9
Delay_RTS EQU $F57D
RANDOM_3 EQU $F511
VEC_BUTTON_2_3 EQU $C818
SOUND_BYTE_RAW EQU $F25B
DEC_COUNTERS EQU $F563
VEC_MUSIC_CHAN EQU $C855
VEC_BUTTONS EQU $C811
VEC_JOY_MUX_2_Y EQU $C822
Dec_3_Counters EQU $F55A
Vec_Rfrsh_lo EQU $C83D
Vec_Brightness EQU $C827
VEC_COUNTER_2 EQU $C82F
DRAW_LINE_D EQU $F3DF
WAIT_RECAL EQU $F192
RISE_RUN_X EQU $F5FF
VEC_TWANG_TABLE EQU $C851
ROT_VL_AB EQU $F610
STRIP_ZEROS EQU $F8B7
Vec_SWI2_Vector EQU $CBF2
Delay_3 EQU $F56D
music5 EQU $FE38
musica EQU $FF44
DELAY_RTS EQU $F57D
DP_to_C8 EQU $F1AF
DRAW_VLP_7F EQU $F408
Rise_Run_X EQU $F5FF
Sound_Byte_raw EQU $F25B
Vec_Button_2_1 EQU $C816
INIT_MUSIC_CHK EQU $F687
MUSICD EQU $FF8F
Print_Ships EQU $F393
VEC_EXPL_4 EQU $C85B
Init_VIA EQU $F14C
VEC_COUNTER_1 EQU $C82E
Vec_RiseRun_Tmp EQU $C834
musicb EQU $FF62
VEC_BUTTON_1_4 EQU $C815
VEC_JOY_1_Y EQU $C81C
Obj_Will_Hit EQU $F8F3
music8 EQU $FEF8
MOV_DRAW_VL_AB EQU $F3B7
Joy_Digital EQU $F1F8
Vec_Max_Players EQU $C84F
SOUND_BYTE EQU $F256
Rise_Run_Len EQU $F603
Random EQU $F517
INIT_OS EQU $F18B
MOVE_MEM_A EQU $F683
MOVETO_X_7F EQU $F2F2
Moveto_d EQU $F312
Vec_SWI_Vector EQU $CBFB
Vec_Music_Ptr EQU $C853
VEC_MUSIC_WK_5 EQU $C847
Recalibrate EQU $F2E6
Init_Music_chk EQU $F687
VEC_RANDOM_SEED EQU $C87D
Rise_Run_Y EQU $F601
Moveto_ix_FF EQU $F308
Dot_List EQU $F2D5
MOVETO_IX_A EQU $F30E
Vec_RiseRun_Len EQU $C83B
DRAW_VLP_B EQU $F40E
Dot_ix_b EQU $F2BE
Vec_Joy_Mux EQU $C81F
PRINT_LIST EQU $F38A
VEC_SEED_PTR EQU $C87B
VEC_MUSIC_FREQ EQU $C861
Moveto_ix_a EQU $F30E
NEW_HIGH_SCORE EQU $F8D8
VEC_JOY_RESLTN EQU $C81A
VEC_RISE_INDEX EQU $C839
MOVETO_IX_7F EQU $F30C
Rot_VL_Mode_a EQU $F61F
Abs_b EQU $F58B
Clear_x_256 EQU $F545
Strip_Zeros EQU $F8B7
Draw_Grid_VL EQU $FF9F
Draw_VLp_FF EQU $F404
DOT_IX EQU $F2C1
Vec_Seed_Ptr EQU $C87B
Clear_x_d EQU $F548
VEC_TEXT_HEIGHT EQU $C82A
INIT_MUSIC_BUF EQU $F533
Vec_Str_Ptr EQU $C82C
MUSIC8 EQU $FEF8
Moveto_d_7F EQU $F2FC
Rot_VL EQU $F616
Init_OS EQU $F18B
Vec_Misc_Count EQU $C823
MUSIC9 EQU $FF26
INTENSITY_7F EQU $F2A9
VEC_COUNTER_4 EQU $C831
music7 EQU $FEC6
READ_BTNS_MASK EQU $F1B4
ROT_VL_MODE_A EQU $F61F
Init_Music_Buf EQU $F533
Random_3 EQU $F511
DRAW_VL_B EQU $F3D2
Reset_Pen EQU $F35B
DELAY_2 EQU $F571
XFORM_RUN_A EQU $F65B
VEC_MUSIC_TWANG EQU $C858
Vec_Default_Stk EQU $CBEA
Delay_b EQU $F57A
DELAY_3 EQU $F56D
SOUND_BYTES EQU $F27D
CLEAR_SOUND EQU $F272
DRAW_VLP EQU $F410
Clear_x_b_a EQU $F552
VEC_DEFAULT_STK EQU $CBEA
Print_List EQU $F38A
Vec_Button_1_4 EQU $C815
RESET0INT EQU $F36B
DELAY_0 EQU $F579
DOT_HERE EQU $F2C5
MOD16 EQU $4000
Draw_VLp_b EQU $F40E
VEC_JOY_2_Y EQU $C81E
CLEAR_X_D EQU $F548
Vec_Expl_2 EQU $C859
Rot_VL_dft EQU $F637
VEC_RFRSH_LO EQU $C83D
Draw_VLc EQU $F3CE
INIT_OS_RAM EQU $F164
VEC_EXPL_FLAG EQU $C867
VEC_DURATION EQU $C857
Vec_Joy_Mux_2_Y EQU $C822
VEC_MISC_COUNT EQU $C823
VEC_BUTTON_1_3 EQU $C814
VEC_MAX_PLAYERS EQU $C84F
DP_TO_C8 EQU $F1AF
Draw_VL_ab EQU $F3D8
Abs_a_b EQU $F584
Vec_Counters EQU $C82E
Vec_0Ref_Enable EQU $C824
GET_RISE_IDX EQU $F5D9
VEC_EXPL_1 EQU $C858
MOV_DRAW_VLCS EQU $F3B5
INTENSITY_1F EQU $F29D
Vec_ADSR_Table EQU $C84F
MUSIC5 EQU $FE38
Vec_Music_Flag EQU $C856
INIT_MUSIC_X EQU $F692
RESET_PEN EQU $F35B
Dot_List_Reset EQU $F2DE
DRAW_VLP_SCALE EQU $F40C
CLEAR_X_B EQU $F53F
VEC_TEXT_HW EQU $C82A
MOV_DRAW_VLC_A EQU $F3AD
Print_List_hw EQU $F385
Draw_VLp EQU $F410
Draw_VL_a EQU $F3DA
VEC_BUTTON_2_1 EQU $C816
Mov_Draw_VL_d EQU $F3BE
Vec_Button_1_3 EQU $C814
Vec_Counter_2 EQU $C82F
Print_Str_d EQU $F37A
CLEAR_X_B_A EQU $F552
GET_RISE_RUN EQU $F5EF
Vec_Run_Index EQU $C837
Obj_Hit EQU $F8FF
PRINT_STR_YX EQU $F378
INTENSITY_A EQU $F2AB
Vec_Prev_Btns EQU $C810
VEC_TEXT_WIDTH EQU $C82B
PRINT_LIST_CHK EQU $F38C
PRINT_SHIPS_X EQU $F391
Add_Score_d EQU $F87C
RISE_RUN_ANGLE EQU $F593
VEC_RUN_INDEX EQU $C837
VEC_IRQ_VECTOR EQU $CBF8
Moveto_ix_7F EQU $F30C
Init_Music_x EQU $F692
VEC_BUTTON_2_2 EQU $C817
Intensity_3F EQU $F2A1
Vec_Btn_State EQU $C80F
music1 EQU $FD0D
Vec_Expl_1 EQU $C858
VEC_NUM_PLAYERS EQU $C879
Cold_Start EQU $F000
Read_Btns EQU $F1BA
Rot_VL_Mode EQU $F62B
ABS_A_B EQU $F584
Vec_Random_Seed EQU $C87D
Read_Btns_Mask EQU $F1B4
MUSIC1 EQU $FD0D
Vec_Expl_Chans EQU $C854
DOT_IX_B EQU $F2BE
Print_Str_yx EQU $F378
DO_SOUND EQU $F289
ADD_SCORE_D EQU $F87C
DRAW_PAT_VL EQU $F437
EXPLOSION_SND EQU $F92E
Vec_Joy_1_Y EQU $C81C
PRINT_STR_D EQU $F37A
XFORM_RUN EQU $F65D
Vec_SWI3_Vector EQU $CBF2
VEC_BUTTON_1_1 EQU $C812
Vec_Music_Twang EQU $C858
SELECT_GAME EQU $F7A9
VEC_EXPL_3 EQU $C85A
Vec_Freq_Table EQU $C84D
Draw_VL EQU $F3DD
Vec_Rise_Index EQU $C839
VEC_COUNTERS EQU $C82E
Vec_Expl_ChanA EQU $C853
VEC_SWI_VECTOR EQU $CBFB
CLEAR_X_B_80 EQU $F550
VEC_EXPL_CHANB EQU $C85D
Vec_NMI_Vector EQU $CBFB
INIT_MUSIC EQU $F68D
musicc EQU $FF7A
VEC_MUSIC_WK_6 EQU $C846
WARM_START EQU $F06C
Do_Sound_x EQU $F28C
COLD_START EQU $F000
VEC_MUSIC_FLAG EQU $C856
DO_SOUND_X EQU $F28C
Vec_IRQ_Vector EQU $CBF8
Vec_Dot_Dwell EQU $C828
Vec_Counter_5 EQU $C832
MUSIC4 EQU $FDD3
music9 EQU $FF26
SOUND_BYTES_X EQU $F284
XFORM_RISE EQU $F663
Mov_Draw_VL_ab EQU $F3B7
VEC_JOY_MUX EQU $C81F
Draw_Line_d EQU $F3DF
Set_Refresh EQU $F1A2
VEC_COUNTER_5 EQU $C832
DELAY_1 EQU $F575
Get_Rise_Idx EQU $F5D9
Vec_Max_Games EQU $C850
DP_to_D0 EQU $F1AA
OBJ_WILL_HIT EQU $F8F3
MOVETO_D_7F EQU $F2FC
Vec_Button_2_2 EQU $C817
VEC_BTN_STATE EQU $C80F
Vec_Music_Work EQU $C83F
MOVETO_IX EQU $F310
Vec_Joy_2_X EQU $C81D
INTENSITY_3F EQU $F2A1
music3 EQU $FD81
Vec_Music_Wk_A EQU $C842
MOVETO_D EQU $F312
VEC_COUNTER_3 EQU $C830
Clear_Score EQU $F84F
Vec_Joy_Resltn EQU $C81A
Sound_Bytes EQU $F27D
Vec_Button_1_1 EQU $C812
Obj_Will_Hit_u EQU $F8E5
Vec_Loop_Count EQU $C825
Vec_Expl_Flag EQU $C867
VEC_RISERUN_LEN EQU $C83B
VEC_SWI3_VECTOR EQU $CBF2
VEC_LOOP_COUNT EQU $C825
Vec_Buttons EQU $C811
VEC_ANGLE EQU $C836
VEC_NMI_VECTOR EQU $CBFB
VEC_ADSR_TABLE EQU $C84F
Vec_Expl_Timer EQU $C877
ABS_B EQU $F58B
Vec_Expl_3 EQU $C85A


;***************************************************************************
; CARTRIDGE HEADER
;***************************************************************************
    FCC "g GCE 2025"
    FCB $80                 ; String terminator
    FDB $0000              ; Music pointer
    FCB $F8,$50,$20,$BB     ; Height, Width, Rel Y, Rel X
    FCC "Test Multi-Bank Variable"
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
; ARRAY DATA (ROM literals)
;***************************************************************************
; Arrays are stored in ROM and accessed via pointers
; At startup, main() initializes VAR_{name} to point to ARRAY_{name}_DATA

; Array literal for variable 'my_array' (4 elements)
ARRAY_MY_ARRAY_DATA:
    FDB 127   ; Element 0
    FDB 100   ; Element 1
    FDB 80   ; Element 2
    FDB 60   ; Element 3


;***************************************************************************
; MAIN PROGRAM (Bank #0)
;***************************************************************************

MAIN:
    ; Initialize global variables
    LDD #10
    STD VAR_MY_VAR
    ; Copy array 'my_array' from ROM to RAM (4 elements)
    LDX #ARRAY_MY_ARRAY_DATA       ; Source: ROM array data
    LDU #VAR_MY_ARRAY_DATA       ; Dest: RAM array space
    LDD #4        ; Number of elements
.COPY_LOOP_0:
    LDY ,X++        ; Load word from ROM, increment source
    STY ,U++        ; Store word to RAM, increment dest
    SUBD #1         ; Decrement counter
    LBNE .COPY_LOOP_0 ; Loop until done (LBNE for long branch)
    LDX #VAR_MY_ARRAY_DATA    ; Array now in RAM
    STX VAR_MY_ARRAY
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
    JSR WAIT_RECAL
    LDX #VAR_MY_ARRAY_DATA  ; Array data
    PSHS X
    LDD #0
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    STD VAR_VAL
    ; SET_INTENSITY: Set drawing intensity
    LDD VAR_VAL
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    RTS

;***************************************************************************
; EMBEDDED ASSETS (vectors, music, levels, SFX)
;***************************************************************************

; Generated from asym.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 3
; X bounds: min=0, max=20, width=20
; Center: (10, 10)

_ASYM_WIDTH EQU 20
_ASYM_CENTER_X EQU 10
_ASYM_CENTER_Y EQU 10

_ASYM_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _ASYM_PATH0        ; pointer to path 0

_ASYM_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $F6,$F6,0,0        ; path0: header (y=-10, x=-10, relative to center)
    FCB $FF,$14,$0A          ; line 0: flag=-1, dy=20, dx=10
    FCB $FF,$EC,$0A          ; line 1: flag=-1, dy=-20, dx=10
    FCB 2                ; End marker (path complete)

; ================================================
