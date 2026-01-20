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
PSG_MUSIC_PTR        EQU $C880+$1C   ; PSG music data pointer (2 bytes)
PSG_MUSIC_START      EQU $C880+$1E   ; PSG music start pointer (for loops) (2 bytes)
PSG_MUSIC_ACTIVE     EQU $C880+$20   ; PSG music active flag (1 bytes)
PSG_IS_PLAYING       EQU $C880+$21   ; PSG playing flag (1 bytes)
PSG_DELAY_FRAMES     EQU $C880+$22   ; PSG frame delay counter (1 bytes)
PSG_MUSIC_BANK       EQU $C880+$23   ; PSG music bank ID (for multibank) (1 bytes)
SFX_PTR              EQU $C880+$24   ; SFX data pointer (2 bytes)
SFX_ACTIVE           EQU $C880+$26   ; SFX active flag (1 bytes)
VAR_PLAYING          EQU $C880+$27   ; User variable: PLAYING (2 bytes)
VAR_TITLE_INTENSITY  EQU $C880+$29   ; User variable: TITLE_INTENSITY (2 bytes)
VAR_ARG0             EQU $CFE0   ; Function argument 0 (16-bit) (2 bytes)
VAR_ARG1             EQU $CFE2   ; Function argument 1 (16-bit) (2 bytes)
VAR_ARG2             EQU $CFE4   ; Function argument 2 (16-bit) (2 bytes)
VAR_ARG3             EQU $CFE6   ; Function argument 3 (16-bit) (2 bytes)
VAR_ARG4             EQU $CFE8   ; Function argument 4 (16-bit) (2 bytes)
CURRENT_ROM_BANK     EQU $CFEA   ; Current ROM bank ID (multibank tracking) (1 bytes)


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
DRAW_VL EQU $F3DD
Vec_Rfrsh_lo EQU $C83D
Reset0Int EQU $F36B
Delay_0 EQU $F579
Vec_Music_Wk_5 EQU $C847
VEC_MAX_PLAYERS EQU $C84F
MUSIC8 EQU $FEF8
AU_SKIP_MUSIC EQU $4188
Sound_Byte EQU $F256
VEC_EXPL_CHANB EQU $C85D
Print_Ships EQU $F393
Mov_Draw_VLcs EQU $F3B5
INTENSITY_5F EQU $F2A5
Get_Run_Idx EQU $F5DB
Draw_VLcs EQU $F3D6
DOT_D EQU $F2C3
SFX_NEXTFRAME EQU $4239
Vec_Btn_State EQU $C80F
Draw_VLc EQU $F3CE
Sound_Byte_raw EQU $F25B
VEC_EXPL_3 EQU $C85A
musicd EQU $FF8F
MUSIC2 EQU $FD1D
PSG_music_loop EQU $40D8
INIT_MUSIC EQU $F68D
Get_Rise_Run EQU $F5EF
Vec_Duration EQU $C857
DELAY_0 EQU $F579
DOT_LIST EQU $F2D5
VEC_RISERUN_TMP EQU $C834
OBJ_WILL_HIT_U EQU $F8E5
VEC_SEED_PTR EQU $C87B
Dot_ix_b EQU $F2BE
Print_Str_hwyx EQU $F373
ASSET_BANK_TABLE EQU $4003
Vec_Button_2_1 EQU $C816
DEC_6_COUNTERS EQU $F55E
VEC_JOY_2_Y EQU $C81E
Rise_Run_Y EQU $F601
Vec_SWI2_Vector EQU $CBF2
VEC_BUTTON_1_3 EQU $C814
Mov_Draw_VL_d EQU $F3BE
Moveto_d EQU $F312
Reset0Ref_D0 EQU $F34A
Clear_Score EQU $F84F
Vec_Expl_Chan EQU $C85C
DRAW_VLC EQU $F3CE
VEC_SWI3_VECTOR EQU $CBF2
Vec_Joy_1_Y EQU $C81C
Obj_Hit EQU $F8FF
Vec_Joy_Resltn EQU $C81A
MUSIC_ADDR_TABLE EQU $4001
DRAW_VLCS EQU $F3D6
sfx_enabletone EQU $4210
Read_Btns_Mask EQU $F1B4
Print_List_hw EQU $F385
VEC_JOY_MUX_1_X EQU $C81F
Random EQU $F517
SFX_UPDATE EQU $41A9
Vec_Expl_2 EQU $C859
music7 EQU $FEC6
MOVETO_X_7F EQU $F2F2
Rise_Run_Angle EQU $F593
COLD_START EQU $F000
COMPARE_SCORE EQU $F8C7
Sound_Bytes EQU $F27D
MUSICD EQU $FF8F
VEC_EXPL_FLAG EQU $C867
AU_MUSIC_NO_DELAY EQU $413F
DRAW_VL_A EQU $F3DA
Draw_VL_mode EQU $F46E
VEC_MUSIC_CHAN EQU $C855
music9 EQU $FF26
Vec_Button_1_4 EQU $C815
DELAY_RTS EQU $F57D
DELAY_3 EQU $F56D
Abs_a_b EQU $F584
UPDATE_MUSIC_PSG EQU $407A
PRINT_STR_HWYX EQU $F373
Vec_Snd_Shadow EQU $C800
Vec_RiseRun_Len EQU $C83B
Vec_Joy_2_Y EQU $C81E
DOT_IX_B EQU $F2BE
Bitmask_a EQU $F57E
RISE_RUN_X EQU $F5FF
VEC_DOT_DWELL EQU $C828
Vec_Button_1_2 EQU $C813
Vec_NMI_Vector EQU $CBFB
VEC_MUSIC_TWANG EQU $C858
Vec_Counter_3 EQU $C830
VEC_MUSIC_PTR EQU $C853
VEC_PATTERN EQU $C829
VEC_MUSIC_WK_A EQU $C842
MUSIC_BANK_TABLE EQU $4000
VEC_COUNTER_2 EQU $C82F
musicb EQU $FF62
PRINT_STR_D EQU $F37A
MOVE_MEM_A_1 EQU $F67F
MOV_DRAW_VL_B EQU $F3B1
AUDIO_UPDATE EQU $40EE
music6 EQU $FE76
VEC_RISERUN_LEN EQU $C83B
VEC_BUTTON_1_2 EQU $C813
STRIP_ZEROS EQU $F8B7
RISE_RUN_LEN EQU $F603
DO_SOUND EQU $F289
VEC_BUTTONS EQU $C811
sfx_disabletone EQU $4203
Dot_here EQU $F2C5
Print_Ships_x EQU $F391
Vec_Counter_1 EQU $C82E
NOAY EQU $41B3
VEC_MUSIC_FREQ EQU $C861
Vec_SWI3_Vector EQU $CBF2
VEC_BUTTON_2_1 EQU $C816
PMR_START_NEW EQU $406B
VEC_COLD_FLAG EQU $CBFE
Vec_Expl_1 EQU $C858
DRAW_PAT_VL EQU $F437
RECALIBRATE EQU $F2E6
VEC_TEXT_WIDTH EQU $C82B
Draw_Pat_VL_a EQU $F434
Dot_List_Reset EQU $F2DE
Vec_Button_2_3 EQU $C818
INIT_OS_RAM EQU $F164
RISE_RUN_ANGLE EQU $F593
VEC_MUSIC_WK_5 EQU $C847
DRAW_VLP_FF EQU $F404
Vec_Counters EQU $C82E
Clear_x_d EQU $F548
VEC_LOOP_COUNT EQU $C825
Moveto_ix EQU $F310
Select_Game EQU $F7A9
MUSIC6 EQU $FE76
JOY_DIGITAL EQU $F1F8
MOV_DRAW_VL_A EQU $F3B9
Vec_Text_HW EQU $C82A
MOVE_MEM_A EQU $F683
PSG_WRITE_LOOP EQU $409B
New_High_Score EQU $F8D8
AU_MUSIC_ENDED EQU $4177
Xform_Rise_a EQU $F661
Vec_Counter_4 EQU $C831
DRAW_PAT_VL_A EQU $F434
Draw_VL_ab EQU $F3D8
VEC_BUTTON_2_3 EQU $C818
VEC_NUM_PLAYERS EQU $C879
VEC_EXPL_1 EQU $C858
MOV_DRAW_VL_D EQU $F3BE
VEC_SND_SHADOW EQU $C800
VEC_JOY_2_X EQU $C81D
SFX_CHECKTONEDISABLE EQU $41FB
Vec_Freq_Table EQU $C84D
DELAY_1 EQU $F575
VEC_SWI2_VECTOR EQU $CBF2
Vec_0Ref_Enable EQU $C824
WARM_START EQU $F06C
VEC_BUTTON_1_1 EQU $C812
Vec_Joy_Mux_1_Y EQU $C820
Print_List_chk EQU $F38C
MOV_DRAW_VL EQU $F3BC
RESET_PEN EQU $F35B
Vec_Counter_6 EQU $C833
ASSET_ADDR_TABLE EQU $4004
Dec_Counters EQU $F563
Vec_Twang_Table EQU $C851
VEC_NUM_GAME EQU $C87A
Vec_Expl_4 EQU $C85B
Xform_Run EQU $F65D
MOVETO_IX_7F EQU $F30C
DEC_COUNTERS EQU $F563
Xform_Run_a EQU $F65B
VEC_PREV_BTNS EQU $C810
Clear_x_b_80 EQU $F550
VEC_COUNTER_1 EQU $C82E
VEC_SWI_VECTOR EQU $CBFB
Vec_Angle EQU $C836
AU_MUSIC_HAS_DELAY EQU $414E
MUSICB EQU $FF62
INIT_MUSIC_CHK EQU $F687
Vec_Music_Wk_7 EQU $C845
ADD_SCORE_D EQU $F87C
Draw_VLp_b EQU $F40E
DO_SOUND_X EQU $F28C
AU_BANK_OK EQU $4108
Move_Mem_a EQU $F683
Check0Ref EQU $F34F
Abs_b EQU $F58B
DRAW_PAT_VL_D EQU $F439
DOT_HERE EQU $F2C5
musicc EQU $FF7A
CLEAR_X_B_A EQU $F552
MOVETO_D EQU $F312
Vec_Dot_Dwell EQU $C828
Vec_Loop_Count EQU $C825
Delay_2 EQU $F571
VEC_IRQ_VECTOR EQU $CBF8
Add_Score_d EQU $F87C
Draw_Grid_VL EQU $FF9F
Draw_VLp_7F EQU $F408
AU_MUSIC_DONE EQU $4171
VEC_ANGLE EQU $C836
AU_MUSIC_PROCESS_WRITES EQU $4158
VEC_FREQ_TABLE EQU $C84D
PSG_UPDATE_DONE EQU $40E0
Vec_Music_Ptr EQU $C853
Draw_VLp_FF EQU $F404
Intensity_3F EQU $F2A1
Moveto_x_7F EQU $F2F2
CLEAR_SOUND EQU $F272
VEC_EXPL_4 EQU $C85B
VEC_BTN_STATE EQU $C80F
ROT_VL EQU $F616
PSG_FRAME_DONE EQU $40CC
Vec_Expl_Timer EQU $C877
VEC_JOY_RESLTN EQU $C81A
MUSIC1 EQU $FD0D
VEC_MUSIC_WK_6 EQU $C846
Rise_Run_X EQU $F5FF
Rot_VL_Mode_a EQU $F61F
Add_Score_a EQU $F85E
Vec_ADSR_Table EQU $C84F
Vec_Random_Seed EQU $C87D
VEC_BRIGHTNESS EQU $C827
JOY_ANALOG EQU $F1F5
CLEAR_X_D EQU $F548
Vec_ADSR_Timers EQU $C85E
MUSICC EQU $FF7A
Draw_VLp EQU $F410
READ_BTNS_MASK EQU $F1B4
VEC_BUTTON_2_4 EQU $C819
SFX_ENDOFEFFECT EQU $423E
Warm_Start EQU $F06C
VEC_JOY_MUX EQU $C81F
Joy_Analog EQU $F1F5
VEC_TWANG_TABLE EQU $C851
Obj_Will_Hit EQU $F8F3
VEC_RISE_INDEX EQU $C839
Vec_Joy_2_X EQU $C81D
INIT_OS EQU $F18B
ROT_VL_DFT EQU $F637
Dot_d EQU $F2C3
INIT_VIA EQU $F14C
Vec_Joy_Mux_2_X EQU $C821
VEC_RFRSH_HI EQU $C83E
VEC_MISC_COUNT EQU $C823
Print_Str_yx EQU $F378
PLAY_MUSIC_BANKED EQU $4006
VEC_COUNTER_6 EQU $C833
DRAW_VL_MODE EQU $F46E
MUSIC5 EQU $FE38
SFX_ENABLETONE EQU $4210
Intensity_5F EQU $F2A5
Get_Rise_Idx EQU $F5D9
Dec_3_Counters EQU $F55A
GET_RISE_IDX EQU $F5D9
MOV_DRAW_VLC_A EQU $F3AD
SFX_ENABLENOISE EQU $422F
Do_Sound EQU $F289
MOV_DRAW_VLCS EQU $F3B5
VEC_JOY_MUX_2_Y EQU $C822
MOVETO_IX_A EQU $F30E
VEC_HIGH_SCORE EQU $CBEB
DRAW_VL_AB EQU $F3D8
PSG_MUSIC_ENDED EQU $40D2
Vec_Expl_ChanB EQU $C85D
CHECK0REF EQU $F34F
VEC_RFRSH EQU $C83D
Vec_Seed_Ptr EQU $C87B
SOUND_BYTES EQU $F27D
DP_to_D0 EQU $F1AA
Dot_ix EQU $F2C1
DRAW_GRID_VL EQU $FF9F
music5 EQU $FE38
XFORM_RISE_A EQU $F661
Rot_VL_Mode EQU $F62B
Random_3 EQU $F511
music8 EQU $FEF8
Init_VIA EQU $F14C
Vec_Button_1_1 EQU $C812
INIT_MUSIC_X EQU $F692
Sound_Byte_x EQU $F259
Draw_VL_a EQU $F3DA
PRINT_TEXT_STR_3232159404 EQU $424F
Init_OS EQU $F18B
VEC_RUN_INDEX EQU $C837
Vec_Expl_3 EQU $C85A
Vec_FIRQ_Vector EQU $CBF5
DELAY_B EQU $F57A
PRINT_STR_YX EQU $F378
sfx_checktonefreq EQU $41C7
Vec_Button_1_3 EQU $C814
RESET0REF EQU $F354
Set_Refresh EQU $F1A2
Rot_VL_ab EQU $F610
VEC_JOY_MUX_2_X EQU $C821
Vec_Max_Players EQU $C84F
DP_TO_D0 EQU $F1AA
DP_to_C8 EQU $F1AF
Vec_Run_Index EQU $C837
Vec_SWI_Vector EQU $CBFB
MUSIC9 EQU $FF26
Vec_Music_Wk_6 EQU $C846
Rot_VL EQU $F616
VEC_ADSR_TIMERS EQU $C85E
PLAY_SFX_RUNTIME EQU $41A0
Moveto_ix_a EQU $F30E
ROT_VL_AB EQU $F610
Vec_High_Score EQU $CBEB
READ_BTNS EQU $F1BA
AU_MUSIC_LOOP EQU $417D
Draw_Pat_VL_d EQU $F439
VEC_BUTTON_2_2 EQU $C817
Vec_Counter_5 EQU $C832
MUSICA EQU $FF44
Clear_Sound EQU $F272
Delay_b EQU $F57A
PSG_music_ended EQU $40D2
Clear_x_b EQU $F53F
Vec_Num_Game EQU $C87A
STOP_MUSIC_RUNTIME EQU $40E4
ROT_VL_MODE EQU $F62B
VEC_COUNTERS EQU $C82E
INTENSITY_1F EQU $F29D
ABS_B EQU $F58B
CLEAR_SCORE EQU $F84F
VEC_DEFAULT_STK EQU $CBEA
DP_TO_C8 EQU $F1AF
MUSIC3 EQU $FD81
VEC_EXPL_CHANS EQU $C854
SFX_CHECKTONEFREQ EQU $41C7
Compare_Score EQU $F8C7
VEC_MUSIC_WK_7 EQU $C845
Print_Str EQU $F495
PSG_write_loop EQU $409B
DRAW_LINE_D EQU $F3DF
Vec_Rise_Index EQU $C839
GET_RUN_IDX EQU $F5DB
VEC_COUNTER_4 EQU $C831
Vec_Expl_Flag EQU $C867
DRAW_VLP_SCALE EQU $F40C
Rot_VL_dft EQU $F637
Clear_x_256 EQU $F545
sfx_checknoisedisable EQU $421A
ROT_VL_MODE_A EQU $F61F
VEC_ADSR_TABLE EQU $C84F
Moveto_ix_FF EQU $F308
Intensity_7F EQU $F2A9
music3 EQU $FD81
Vec_Music_Wk_1 EQU $C84B
XFORM_RISE EQU $F663
Intensity_1F EQU $F29D
DRAW_VLP_7F EQU $F408
WAIT_RECAL EQU $F192
SOUND_BYTE_X EQU $F259
Vec_Brightness EQU $C827
INTENSITY_7F EQU $F2A9
VEC_EXPL_TIMER EQU $C877
Vec_Joy_Mux_1_X EQU $C81F
VEC_MUSIC_WK_1 EQU $C84B
noay EQU $41B3
PSG_MUSIC_LOOP EQU $40D8
DOT_IX EQU $F2C1
Delay_RTS EQU $F57D
DOT_LIST_RESET EQU $F2DE
OBJ_WILL_HIT EQU $F8F3
Mov_Draw_VL_a EQU $F3B9
SET_REFRESH EQU $F1A2
PSG_update_done EQU $40E0
SOUND_BYTE_RAW EQU $F25B
VEC_STR_PTR EQU $C82C
RISE_RUN_Y EQU $F601
Intensity_a EQU $F2AB
Sound_Bytes_x EQU $F284
Draw_Line_d EQU $F3DF
DRAW_VLP EQU $F410
AU_UPDATE_SFX EQU $418B
sfx_checktonedisable EQU $41FB
MOD16.MOD16_END EQU $4058
Dec_6_Counters EQU $F55E
VEC_JOY_1_X EQU $C81B
Vec_RiseRun_Tmp EQU $C834
BITMASK_A EQU $F57E
PRINT_LIST_CHK EQU $F38C
Strip_Zeros EQU $F8B7
VEC_FIRQ_VECTOR EQU $CBF5
Vec_Music_Chan EQU $C855
Vec_Music_Flag EQU $C856
SFX_CHECKNOISEFREQ EQU $41E1
sfx_disablenoise EQU $4222
VEC_EXPL_CHAN EQU $C85C
GET_RISE_RUN EQU $F5EF
CLEAR_X_B EQU $F53F
music1 EQU $FD0D
SFX_CHECKVOLUME EQU $41F2
VEC_0REF_ENABLE EQU $C824
MOV_DRAW_VL_AB EQU $F3B7
Xform_Rise EQU $F663
Do_Sound_x EQU $F28C
Mov_Draw_VL EQU $F3BC
Draw_Pat_VL EQU $F437
Rise_Run_Len EQU $F603
PMr_done EQU $4079
Read_Btns EQU $F1BA
Vec_Num_Players EQU $C879
AU_MUSIC_READ EQU $412A
INTENSITY_3F EQU $F2A1
Moveto_ix_7F EQU $F30C
Vec_Joy_1_X EQU $C81B
EXPLOSION_SND EQU $F92E
SFX_CHECKNOISEDISABLE EQU $421A
PRINT_LIST EQU $F38A
CLEAR_X_B_80 EQU $F550
Mov_Draw_VL_ab EQU $F3B7
Init_Music_chk EQU $F687
Vec_Music_Twang EQU $C858
Vec_Pattern EQU $C829
VEC_EXPL_CHANA EQU $C853
XFORM_RUN EQU $F65D
VEC_RANDOM_SEED EQU $C87D
PRINT_SHIPS_X EQU $F391
sfx_checknoisefreq EQU $41E1
AU_MUSIC_WRITE_LOOP EQU $415A
VEC_TEXT_HEIGHT EQU $C82A
ABS_A_B EQU $F584
VEC_NMI_VECTOR EQU $CBFB
Delay_1 EQU $F575
MOVETO_D_7F EQU $F2FC
Print_List EQU $F38A
Joy_Digital EQU $F1F8
SFX_DOFRAME EQU $41B4
Vec_Prev_Btns EQU $C810
VEC_DURATION EQU $C857
NEW_HIGH_SCORE EQU $F8D8
Vec_Rfrsh_hi EQU $C83E
Cold_Start EQU $F000
VEC_RFRSH_LO EQU $C83D
sfx_checkvolume EQU $41F2
sfx_endofeffect EQU $423E
Vec_Str_Ptr EQU $C82C
Init_OS_RAM EQU $F164
RESET0REF_D0 EQU $F34A
Vec_Music_Freq EQU $C861
AU_MUSIC_READ_COUNT EQU $413F
PRINT_SHIPS EQU $F393
Mov_Draw_VL_b EQU $F3B1
Reset_Pen EQU $F35B
Vec_Default_Stk EQU $CBEA
musica EQU $FF44
VEC_BUTTON_1_4 EQU $C815
VEC_COUNTER_5 EQU $C832
Moveto_d_7F EQU $F2FC
VEC_MUSIC_WORK EQU $C83F
XFORM_RUN_A EQU $F65B
DEC_3_COUNTERS EQU $F55A
Draw_VLp_scale EQU $F40C
PMr_start_new EQU $406B
Vec_Music_Work EQU $C83F
RANDOM EQU $F517
Draw_VL_b EQU $F3D2
Mov_Draw_VLc_a EQU $F3AD
PRINT_LIST_HW EQU $F385
Vec_Cold_Flag EQU $CBFE
Vec_Joy_Mux EQU $C81F
VEC_COUNTER_3 EQU $C830
DELAY_2 EQU $F571
Delay_3 EQU $F56D
Vec_Max_Games EQU $C850
VEC_MUSIC_FLAG EQU $C856
INIT_MUSIC_BUF EQU $F533
Vec_IRQ_Vector EQU $CBF8
sfx_doframe EQU $41B4
Recalibrate EQU $F2E6
MOVETO_IX_FF EQU $F308
PMR_DONE EQU $4079
Init_Music EQU $F68D
MUSIC4 EQU $FDD3
Vec_Counter_2 EQU $C82F
PLAY_MUSIC_RUNTIME EQU $405D
ADD_SCORE_A EQU $F85E
Vec_Text_Height EQU $C82A
Reset0Ref EQU $F354
VEC_MAX_GAMES EQU $C850
Explosion_Snd EQU $F92E
Wait_Recal EQU $F192
_MUSIC1_MUSIC EQU $0000
MOVETO_IX EQU $F310
Dot_List EQU $F2D5
AU_DONE EQU $4195
music4 EQU $FDD3
SELECT_GAME EQU $F7A9
RESET0INT EQU $F36B
MOD16 EQU $403E
Init_Music_Buf EQU $F533
Vec_Joy_Mux_2_Y EQU $C822
VEC_JOY_MUX_1_Y EQU $C820
sfx_nextframe EQU $4239
Vec_Buttons EQU $C811
Init_Music_x EQU $F692
Clear_C8_RAM EQU $F542
CLEAR_C8_RAM EQU $F542
RANDOM_3 EQU $F511
Vec_Text_Width EQU $C82B
MOD16.MOD16_LOOP EQU $4040
VEC_TEXT_HW EQU $C82A
Vec_Expl_ChanA EQU $C853
PSG_frame_done EQU $40CC
Obj_Will_Hit_u EQU $F8E5
Vec_Button_2_2 EQU $C817
VEC_JOY_1_Y EQU $C81C
CLEAR_X_256 EQU $F545
VEC_EXPL_2 EQU $C859
Print_Str_d EQU $F37A
SOUND_BYTE EQU $F256
Clear_x_b_a EQU $F552
OBJ_HIT EQU $F8FF
Vec_Rfrsh EQU $C83D
DRAW_VLP_B EQU $F40E
Move_Mem_a_1 EQU $F67F
Vec_Music_Wk_A EQU $C842
SFX_DISABLENOISE EQU $4222
Draw_VL EQU $F3DD
INTENSITY_A EQU $F2AB
music2 EQU $FD1D
MUSIC7 EQU $FEC6
sfx_enablenoise EQU $422F
DRAW_VL_B EQU $F3D2
SFX_DISABLETONE EQU $4203
Vec_Misc_Count EQU $C823
PRINT_STR EQU $F495
Vec_Expl_Chans EQU $C854
SOUND_BYTES_X EQU $F284
Vec_Button_2_4 EQU $C819


;***************************************************************************
; CARTRIDGE HEADER
;***************************************************************************
    FCC "g GCE 2025"
    FCB $80                 ; String terminator
    FDB music1              ; Music pointer
    FCB $F8,$50,$20,$BB     ; Height, Width, Rel Y, Rel X
    FCC "TEST INCREMENTAL"
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
    ; Initialize CURRENT_ROM_BANK to Bank 0 (current switchable window on boot)
    LDA #0
    STA >CURRENT_ROM_BANK   ; Initialize bank tracker (Bank 0 is visible at boot)
    ; Initialize SFX variables to prevent random noise on startup
    CLR >SFX_ACTIVE         ; Mark SFX as inactive (0=off)
    LDD #$0000
    STD >SFX_PTR            ; Clear SFX pointer
    CLR >PSG_MUSIC_BANK     ; Initialize to 0 (prevents garbage bank switches)
; Bank 0 ($0000) is active; fixed bank 31 ($4000-$7FFF) always visible
    JMP MAIN

;***************************************************************************
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
PSG_MUSIC_PTR        EQU $C880+$1C   ; PSG music data pointer (2 bytes)
PSG_MUSIC_START      EQU $C880+$1E   ; PSG music start pointer (for loops) (2 bytes)
PSG_MUSIC_ACTIVE     EQU $C880+$20   ; PSG music active flag (1 bytes)
PSG_IS_PLAYING       EQU $C880+$21   ; PSG playing flag (1 bytes)
PSG_DELAY_FRAMES     EQU $C880+$22   ; PSG frame delay counter (1 bytes)
PSG_MUSIC_BANK       EQU $C880+$23   ; PSG music bank ID (for multibank) (1 bytes)
SFX_PTR              EQU $C880+$24   ; SFX data pointer (2 bytes)
SFX_ACTIVE           EQU $C880+$26   ; SFX active flag (1 bytes)
VAR_PLAYING          EQU $C880+$27   ; User variable: PLAYING (2 bytes)
VAR_TITLE_INTENSITY  EQU $C880+$29   ; User variable: TITLE_INTENSITY (2 bytes)
VAR_ARG0             EQU $CFE0   ; Function argument 0 (16-bit) (2 bytes)
VAR_ARG1             EQU $CFE2   ; Function argument 1 (16-bit) (2 bytes)
VAR_ARG2             EQU $CFE4   ; Function argument 2 (16-bit) (2 bytes)
VAR_ARG3             EQU $CFE6   ; Function argument 3 (16-bit) (2 bytes)
VAR_ARG4             EQU $CFE8   ; Function argument 4 (16-bit) (2 bytes)
CURRENT_ROM_BANK     EQU $CFEA   ; Current ROM bank ID (multibank tracking) (1 bytes)


;***************************************************************************
; MAIN PROGRAM (Bank #0)
;***************************************************************************

MAIN:
    ; Initialize global variables
    LDD #127
    STD VAR_TITLE_INTENSITY
    LDD #0
    STD VAR_PLAYING
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
    LBRA .MAIN_LOOP   ; Use long branch for multibank support

LOOP_BODY:
    JSR Wait_Recal   ; Synchronize with screen refresh (mandatory)
    JSR Reset0Ref    ; Reset beam to center (0,0)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; SET_INTENSITY: Set drawing intensity
    LDD VAR_TITLE_INTENSITY
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    LDD #0
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_PLAYING
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_0_TRUE
    LDD #0
    LBRA .CMP_0_END
.CMP_0_TRUE:
    LDD #1
.CMP_0_END:
    STD RESULT
    LDD RESULT
    LBEQ IF_NEXT_1
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_PLAYING
    ; PLAY_MUSIC("music1") - play music asset (index=0)
    LDX #0        ; Music asset index for lookup
    JSR PLAY_MUSIC_BANKED  ; Play with automatic bank switching
    LDD #0
    STD RESULT
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    JSR AUDIO_UPDATE  ; Auto-injected: update music + SFX (after all game logic)
    RTS


; ================================================
