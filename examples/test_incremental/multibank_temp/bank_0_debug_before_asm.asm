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
DRAW_VEC_X           EQU $C880+$08   ; Vector draw X offset (1 bytes)
DRAW_VEC_Y           EQU $C880+$09   ; Vector draw Y offset (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$0A   ; Vector intensity override (0=use vector data) (1 bytes)
MIRROR_PAD           EQU $C880+$0B   ; Safety padding to prevent MIRROR flag corruption (16 bytes)
MIRROR_X             EQU $C880+$1B   ; X mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$1C   ; Y mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_LINE_ARGS       EQU $C880+$1D   ; DRAW_LINE argument buffer (x0,y0,x1,y1,intensity) (10 bytes)
VLINE_DX_16          EQU $C880+$27   ; DRAW_LINE dx (16-bit) (2 bytes)
VLINE_DY_16          EQU $C880+$29   ; DRAW_LINE dy (16-bit) (2 bytes)
VLINE_DX             EQU $C880+$2B   ; DRAW_LINE dx clamped (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$2C   ; DRAW_LINE dy clamped (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$2D   ; DRAW_LINE remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$2F   ; DRAW_LINE remaining dx for segment 2 (16-bit) (2 bytes)
PSG_MUSIC_PTR        EQU $C880+$31   ; PSG music data pointer (2 bytes)
PSG_MUSIC_START      EQU $C880+$33   ; PSG music start pointer (for loops) (2 bytes)
PSG_MUSIC_ACTIVE     EQU $C880+$35   ; PSG music active flag (1 bytes)
PSG_IS_PLAYING       EQU $C880+$36   ; PSG playing flag (1 bytes)
PSG_DELAY_FRAMES     EQU $C880+$37   ; PSG frame delay counter (1 bytes)
SFX_PTR              EQU $C880+$38   ; SFX data pointer (2 bytes)
SFX_ACTIVE           EQU $C880+$3A   ; SFX active flag (1 bytes)
VAR_PLAYING          EQU $C880+$3B   ; User variable: playing (2 bytes)
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
music9 EQU $FF26
VEC_ADSR_TABLE EQU $C84F
Vec_Button_1_1 EQU $C812
Obj_Will_Hit_u EQU $F8E5
Moveto_ix_a EQU $F30E
Vec_Expl_ChanA EQU $C853
Vec_Expl_2 EQU $C859
Vec_Joy_Mux_2_Y EQU $C822
MOV_DRAW_VLCS EQU $F3B5
Do_Sound_x EQU $F28C
music5 EQU $FE38
MOVETO_IX_A EQU $F30E
DLW_SEG1_DX_READY EQU $40A7
Vec_Default_Stk EQU $CBEA
DRAW_VL_AB EQU $F3D8
VEC_SEED_PTR EQU $C87B
Moveto_d EQU $F312
Draw_VLcs EQU $F3D6
Vec_High_Score EQU $CBEB
Get_Rise_Run EQU $F5EF
musicb EQU $FF62
sfx_disablenoise EQU $440D
WARM_START EQU $F06C
CLEAR_X_256 EQU $F545
VEC_HIGH_SCORE EQU $CBEB
music1 EQU $FD0D
Vec_Music_Twang EQU $C858
Strip_Zeros EQU $F8B7
SOUND_BYTE_X EQU $F259
INTENSITY_7F EQU $F2A9
Mov_Draw_VL EQU $F3BC
GET_RUN_IDX EQU $F5DB
Draw_Line_d EQU $F3DF
VEC_JOY_1_X EQU $C81B
Bitmask_a EQU $F57E
Vec_Music_Ptr EQU $C853
VEC_MUSIC_TWANG EQU $C858
JOY_ANALOG EQU $F1F5
ADD_SCORE_D EQU $F87C
DELAY_0 EQU $F579
MOV_DRAW_VL_D EQU $F3BE
DOT_D EQU $F2C3
STOP_MUSIC_RUNTIME EQU $42EB
Dec_Counters EQU $F563
music7 EQU $FEC6
VEC_BUTTON_2_4 EQU $C819
VEC_0REF_ENABLE EQU $C824
VEC_EXPL_FLAG EQU $C867
VEC_LOOP_COUNT EQU $C825
_LOGO_PATH4 EQU $44D8
Vec_Button_1_3 EQU $C814
DRAW_PAT_VL EQU $F437
_JUMP_SFX EQU $459E
VEC_IRQ_VECTOR EQU $CBF8
MUSICD EQU $FF8F
Draw_Grid_VL EQU $FF9F
DP_TO_D0 EQU $F1AA
AU_DONE EQU $4388
VEC_MUSIC_PTR EQU $C853
Vec_Music_Wk_5 EQU $C847
INIT_MUSIC_BUF EQU $F533
Rot_VL_Mode EQU $F62B
CLEAR_C8_RAM EQU $F542
VEC_DURATION EQU $C857
VEC_EXPL_2 EQU $C859
VEC_EXPL_3 EQU $C85A
Vec_Expl_Chans EQU $C854
VEC_DOT_DWELL EQU $C828
Vec_Misc_Count EQU $C823
_LOGO_PATH6 EQU $4526
MUSIC1 EQU $FD0D
MUSICB EQU $FF62
DOT_IX EQU $F2C1
ROT_VL_DFT EQU $F637
Draw_Sync_List_At_With_Mirrors EQU $4119
PSG_frame_done EQU $42D3
INIT_MUSIC EQU $F68D
Xform_Rise_a EQU $F661
Draw_Pat_VL_a EQU $F434
Clear_x_b_a EQU $F552
Vec_Joy_Mux_2_X EQU $C821
Vec_Button_2_4 EQU $C819
Vec_SWI3_Vector EQU $CBF2
Vec_Brightness EQU $C827
PRINT_TEXT_STR_2282136835750346 EQU $4446
AU_MUSIC_WRITE_LOOP EQU $434D
Vec_Max_Players EQU $C84F
Dec_6_Counters EQU $F55E
Draw_VL_mode EQU $F46E
Init_OS_RAM EQU $F164
Warm_Start EQU $F06C
VEC_EXPL_1 EQU $C858
AUDIO_UPDATE EQU $42F5
VEC_RISERUN_LEN EQU $C83B
VEC_RISE_INDEX EQU $C839
Dot_d EQU $F2C3
Rise_Run_X EQU $F5FF
VEC_MUSIC_FREQ EQU $C861
DSWM_SET_INTENSITY EQU $4127
MUSICA EQU $FF44
Reset0Ref_D0 EQU $F34A
Clear_Sound EQU $F272
Moveto_ix EQU $F310
_LOGO_VECTORS EQU $4451
Draw_VLp_scale EQU $F40C
NEW_HIGH_SCORE EQU $F8D8
VEC_JOY_MUX_2_Y EQU $C822
DELAY_2 EQU $F571
SOUND_BYTES_X EQU $F284
Print_Str_d EQU $F37A
PRINT_TEXT_STR_3327403 EQU $443A
VEC_TEXT_WIDTH EQU $C82B
Print_List_hw EQU $F385
Vec_Rfrsh EQU $C83D
Vec_Num_Players EQU $C879
Init_VIA EQU $F14C
Moveto_ix_FF EQU $F308
CLEAR_SOUND EQU $F272
AU_MUSIC_LOOP EQU $4370
Vec_NMI_Vector EQU $CBFB
sfx_checktonedisable EQU $43E6
INTENSITY_5F EQU $F2A5
Do_Sound EQU $F289
Vec_Counters EQU $C82E
OBJ_WILL_HIT EQU $F8F3
VEC_FIRQ_VECTOR EQU $CBF5
INTENSITY_A EQU $F2AB
Sound_Byte_x EQU $F259
INIT_OS EQU $F18B
Obj_Hit EQU $F8FF
DRAW_VLP_B EQU $F40E
Mov_Draw_VL_ab EQU $F3B7
DSWM_NEXT_SET_INTENSITY EQU $41EB
Xform_Run EQU $F65D
SET_REFRESH EQU $F1A2
Vec_Dot_Dwell EQU $C828
Explosion_Snd EQU $F92E
Set_Refresh EQU $F1A2
Vec_Music_Flag EQU $C856
Sound_Byte_raw EQU $F25B
DELAY_1 EQU $F575
MUSICC EQU $FF7A
Vec_Rfrsh_hi EQU $C83E
Vec_Joy_Mux_1_X EQU $C81F
VEC_BUTTON_1_2 EQU $C813
VEC_COUNTER_5 EQU $C832
DOT_IX_B EQU $F2BE
DLW_SEG1_DY_READY EQU $4084
MOVETO_IX_7F EQU $F30C
Print_Str_yx EQU $F378
Vec_Text_Width EQU $C82B
Vec_Duration EQU $C857
Vec_Random_Seed EQU $C87D
DLW_SEG1_DY_NO_CLAMP EQU $4081
Dot_List EQU $F2D5
VEC_RFRSH EQU $C83D
Print_List EQU $F38A
XFORM_RISE EQU $F663
Draw_Pat_VL_d EQU $F439
SOUND_BYTE EQU $F256
MOVETO_D_7F EQU $F2FC
PRINT_STR_HWYX EQU $F373
Print_Str EQU $F495
COLD_START EQU $F000
Vec_Joy_1_Y EQU $C81C
_LASER_SFX EQU $46F2
MUSIC8 EQU $FEF8
SOUND_BYTES EQU $F27D
PRINT_TEXT_STR_3232159404 EQU $443F
VEC_MUSIC_FLAG EQU $C856
Moveto_ix_7F EQU $F30C
Vec_Expl_1 EQU $C858
Vec_Button_1_2 EQU $C813
Vec_Rise_Index EQU $C839
MUSIC6 EQU $FE76
RECALIBRATE EQU $F2E6
Vec_IRQ_Vector EQU $CBF8
_LOGO_PATH1 EQU $448D
Vec_Str_Ptr EQU $C82C
INIT_OS_RAM EQU $F164
JOY_DIGITAL EQU $F1F8
_LOGO_PATH0 EQU $4460
VEC_JOY_MUX_1_Y EQU $C820
MOVETO_IX_FF EQU $F308
Vec_Expl_Timer EQU $C877
VEC_FREQ_TABLE EQU $C84D
VEC_TEXT_HW EQU $C82A
VEC_BUTTON_1_3 EQU $C814
MOV_DRAW_VL_A EQU $F3B9
Vec_Joy_Resltn EQU $C81A
DSWM_NO_NEGATE_Y EQU $4134
AU_MUSIC_PROCESS_WRITES EQU $434B
DLW_SEG2_DY_POS EQU $40DD
VEC_JOY_2_Y EQU $C81E
VEC_RFRSH_LO EQU $C83D
DLW_SEG1_DX_LO EQU $4097
Clear_Score EQU $F84F
VEC_EXPL_TIMER EQU $C877
VEC_MUSIC_WORK EQU $C83F
DRAW_GRID_VL EQU $FF9F
MOV_DRAW_VL_AB EQU $F3B7
_LOGO_PATH5 EQU $44FF
VEC_RUN_INDEX EQU $C837
Vec_Joy_2_Y EQU $C81E
VEC_JOY_1_Y EQU $C81C
DSWM_NEXT_PATH EQU $41D9
CLEAR_X_B_A EQU $F552
PSG_music_ended EQU $42D9
Abs_a_b EQU $F584
Vec_Music_Wk_7 EQU $C845
Init_Music EQU $F68D
GET_RISE_RUN EQU $F5EF
DLW_SEG1_DX_NO_CLAMP EQU $40A4
music6 EQU $FE76
Vec_Run_Index EQU $C837
VEC_COUNTER_4 EQU $C831
Intensity_1F EQU $F29D
DRAW_VL_A EQU $F3DA
Delay_1 EQU $F575
Random_3 EQU $F511
Delay_b EQU $F57A
Random EQU $F517
DLW_SEG2_DX_NO_REMAIN EQU $4102
VEC_TEXT_HEIGHT EQU $C82A
ABS_B EQU $F58B
Intensity_3F EQU $F2A1
Vec_Music_Wk_6 EQU $C846
VEC_COUNTER_3 EQU $C830
_BONUS_COLLECTED_SFX EQU $4615
Print_Ships_x EQU $F391
Sound_Bytes EQU $F27D
DLW_NEED_SEG2 EQU $40CC
DOT_LIST EQU $F2D5
Vec_Seed_Ptr EQU $C87B
Abs_b EQU $F58B
Clear_x_d EQU $F548
VEC_BUTTON_1_1 EQU $C812
PSG_write_loop EQU $42A2
VEC_NMI_VECTOR EQU $CBFB
MUSIC7 EQU $FEC6
Dot_ix_b EQU $F2BE
Move_Mem_a EQU $F683
_COIN_SFX EQU $465B
GET_RISE_IDX EQU $F5D9
Rise_Run_Y EQU $F601
Moveto_d_7F EQU $F2FC
Vec_Text_Height EQU $C82A
Vec_FIRQ_Vector EQU $CBF5
Dot_ix EQU $F2C1
Draw_VLp_7F EQU $F408
VEC_SWI_VECTOR EQU $CBFB
DLW_SEG1_DY_LO EQU $4074
DRAW_VL_B EQU $F3D2
PRINT_STR EQU $F495
Delay_3 EQU $F56D
VEC_RISERUN_TMP EQU $C834
DO_SOUND EQU $F289
MUSIC4 EQU $FDD3
DRAW_VLP EQU $F410
Vec_Button_2_2 EQU $C817
VEC_COLD_FLAG EQU $CBFE
Delay_2 EQU $F571
Vec_Prev_Btns EQU $C810
Vec_ADSR_Timers EQU $C85E
Vec_Music_Chan EQU $C855
RANDOM_3 EQU $F511
PMr_start_new EQU $4272
DP_TO_C8 EQU $F1AF
CLEAR_X_B EQU $F53F
INIT_MUSIC_X EQU $F692
MUSIC2 EQU $FD1D
Vec_Button_2_1 EQU $C816
Sound_Bytes_x EQU $F284
Vec_Counter_1 EQU $C82E
Vec_Music_Wk_1 EQU $C84B
Xform_Rise EQU $F663
Get_Run_Idx EQU $F5DB
RANDOM EQU $F517
Vec_Joy_Mux EQU $C81F
Obj_Will_Hit EQU $F8F3
VEC_TWANG_TABLE EQU $C851
UPDATE_MUSIC_PSG EQU $4281
_LOGO_PATH3 EQU $44C6
Vec_Joy_Mux_1_Y EQU $C820
Vec_Button_1_4 EQU $C815
DSWM_DONE EQU $4263
VEC_NUM_GAME EQU $C87A
VEC_EXPL_CHANS EQU $C854
BITMASK_A EQU $F57E
Vec_Music_Work EQU $C83F
Draw_VLc EQU $F3CE
Get_Rise_Idx EQU $F5D9
Read_Btns_Mask EQU $F1B4
SFX_UPDATE EQU $4394
Vec_Joy_2_X EQU $C81D
VEC_JOY_MUX_2_X EQU $C821
VECTREX_PRINT_TEXT EQU $4000
MOV_DRAW_VLC_A EQU $F3AD
VEC_ANGLE EQU $C836
DRAW_VLC EQU $F3CE
PSG_music_loop EQU $42DF
Vec_Expl_3 EQU $C85A
_LOGO_PATH2 EQU $44B4
Vec_Freq_Table EQU $C84D
PMr_done EQU $4280
MOVETO_D EQU $F312
Init_Music_Buf EQU $F533
Joy_Analog EQU $F1F5
Intensity_7F EQU $F2A9
DSWM_LOOP EQU $418E
DSWM_W2 EQU $41CA
Clear_x_b EQU $F53F
Print_List_chk EQU $F38C
Rot_VL EQU $F616
Delay_RTS EQU $F57D
DSWM_NO_NEGATE_DY EQU $41A6
noay EQU $439E
AU_MUSIC_ENDED EQU $436A
Rot_VL_ab EQU $F610
Draw_Pat_VL EQU $F437
MUSIC3 EQU $FD81
sfx_doframe EQU $439F
sfx_nextframe EQU $4424
DLW_DONE EQU $4114
Vec_Counter_2 EQU $C82F
VEC_JOY_MUX EQU $C81F
Clear_x_b_80 EQU $F550
VEC_EXPL_CHAN EQU $C85C
SOUND_BYTE_RAW EQU $F25B
DELAY_B EQU $F57A
Vec_Rfrsh_lo EQU $C83D
RESET_PEN EQU $F35B
DLW_SEG2_DX_CHECK_NEG EQU $40F4
Vec_Expl_ChanB EQU $C85D
EXPLOSION_SND EQU $F92E
Vec_Angle EQU $C836
Delay_0 EQU $F579
Draw_VL_ab EQU $F3D8
READ_BTNS_MASK EQU $F1B4
VEC_MUSIC_WK_5 EQU $C847
VEC_BUTTON_1_4 EQU $C815
SELECT_GAME EQU $F7A9
AU_UPDATE_SFX EQU $437E
MUSIC5 EQU $FE38
music3 EQU $FD81
ABS_A_B EQU $F584
PRINT_STR_YX EQU $F378
DSWM_NO_NEGATE_X EQU $4141
ROT_VL_MODE_A EQU $F61F
Vec_Btn_State EQU $C80F
DOT_HERE EQU $F2C5
Mov_Draw_VL_d EQU $F3BE
XFORM_RUN EQU $F65D
Vec_Cold_Flag EQU $CBFE
PRINT_SHIPS EQU $F393
Reset0Int EQU $F36B
AU_MUSIC_DONE EQU $4364
ROT_VL EQU $F616
VEC_MISC_COUNT EQU $C823
VEC_EXPL_CHANA EQU $C853
Vec_Joy_1_X EQU $C81B
Draw_VL_b EQU $F3D2
Vec_Pattern EQU $C829
RISE_RUN_Y EQU $F601
Vec_SWI_Vector EQU $CBFB
DOT_LIST_RESET EQU $F2DE
_MUSIC1_MUSIC EQU $4538
Reset0Ref EQU $F354
Vec_Max_Games EQU $C850
MOD16.MOD16_END EQU $4035
PLAY_SFX_RUNTIME EQU $438B
VEC_PATTERN EQU $C829
Mov_Draw_VL_b EQU $F3B1
VEC_SWI2_VECTOR EQU $CBF2
PRINT_SHIPS_X EQU $F391
VEC_BUTTONS EQU $C811
STRIP_ZEROS EQU $F8B7
VEC_BUTTON_2_2 EQU $C817
PRINT_LIST_HW EQU $F385
DSWM_NEXT_NO_NEGATE_X EQU $4204
Clear_x_256 EQU $F545
DSWM_USE_OVERRIDE EQU $4125
Vec_SWI2_Vector EQU $CBF2
CLEAR_X_B_80 EQU $F550
VEC_PREV_BTNS EQU $C810
Vec_RiseRun_Len EQU $C83B
DSWM_W1 EQU $4185
sfx_disabletone EQU $43EE
PLAY_MUSIC_RUNTIME EQU $4264
sfx_checknoisedisable EQU $4405
DRAW_VLP_SCALE EQU $F40C
OBJ_WILL_HIT_U EQU $F8E5
Vec_Snd_Shadow EQU $C800
WAIT_RECAL EQU $F192
ROT_VL_MODE EQU $F62B
DSWM_NEXT_NO_NEGATE_Y EQU $41F7
Vec_Loop_Count EQU $C825
sfx_checktonefreq EQU $43B2
Draw_VLp_FF EQU $F404
VEC_MUSIC_CHAN EQU $C855
Vec_Button_2_3 EQU $C818
Check0Ref EQU $F34F
MOD16 EQU $401B
CLEAR_SCORE EQU $F84F
VEC_COUNTERS EQU $C82E
INIT_MUSIC_CHK EQU $F687
Sound_Byte EQU $F256
Select_Game EQU $F7A9
RESET0INT EQU $F36B
VEC_SWI3_VECTOR EQU $CBF2
Vec_Music_Freq EQU $C861
AU_MUSIC_HAS_DELAY EQU $4341
DRAW_PAT_VL_D EQU $F439
VEC_SND_SHADOW EQU $C800
XFORM_RUN_A EQU $F65B
MOV_DRAW_VL_B EQU $F3B1
DSWM_W3 EQU $4254
DEC_3_COUNTERS EQU $F55A
Dot_here EQU $F2C5
_STAR_VRELEASE_SFX EQU $45BB
DELAY_3 EQU $F56D
VEC_MUSIC_WK_6 EQU $C846
Clear_C8_RAM EQU $F542
COMPARE_SCORE EQU $F8C7
VEC_JOY_RESLTN EQU $C81A
DEC_COUNTERS EQU $F563
AU_MUSIC_NO_DELAY EQU $4332
INIT_VIA EQU $F14C
VEC_ADSR_TIMERS EQU $C85E
CHECK0REF EQU $F34F
PSG_update_done EQU $42E7
music4 EQU $FDD3
RISE_RUN_ANGLE EQU $F593
musicc EQU $FF7A
DRAW_LINE_D EQU $F3DF
CLEAR_X_D EQU $F548
AU_SKIP_MUSIC EQU $437B
Vec_Expl_Flag EQU $C867
_EXPLOSION1_SFX EQU $4631
Reset_Pen EQU $F35B
Cold_Start EQU $F000
sfx_enabletone EQU $43FB
VEC_COUNTER_2 EQU $C82F
music8 EQU $FEF8
MUSIC9 EQU $FF26
VEC_COUNTER_1 EQU $C82E
RISE_RUN_LEN EQU $F603
Vec_Text_HW EQU $C82A
VEC_BRIGHTNESS EQU $C827
PRINT_STR_D EQU $F37A
Rot_VL_dft EQU $F637
Init_OS EQU $F18B
Rot_VL_Mode_a EQU $F61F
Compare_Score EQU $F8C7
XFORM_RISE_A EQU $F661
PRINT_LIST_CHK EQU $F38C
Joy_Digital EQU $F1F8
Vec_Counter_5 EQU $C832
ADD_SCORE_A EQU $F85E
Vec_Counter_3 EQU $C830
VEC_MAX_PLAYERS EQU $C84F
Draw_VLp EQU $F410
MOD16.MOD16_LOOP EQU $401D
Draw_VLp_b EQU $F40E
Add_Score_d EQU $F87C
Recalibrate EQU $F2E6
Add_Score_a EQU $F85E
DRAW_PAT_VL_A EQU $F434
Read_Btns EQU $F1BA
Mov_Draw_VL_a EQU $F3B9
DO_SOUND_X EQU $F28C
musica EQU $FF44
RISE_RUN_X EQU $F5FF
VEC_COUNTER_6 EQU $C833
Move_Mem_a_1 EQU $F67F
VEC_STR_PTR EQU $C82C
RESET0REF EQU $F354
Intensity_a EQU $F2AB
DP_to_D0 EQU $F1AA
Moveto_x_7F EQU $F2F2
MOVE_MEM_A EQU $F683
AU_MUSIC_READ EQU $431D
PRINT_LIST EQU $F38A
VEC_MUSIC_WK_1 EQU $C84B
New_High_Score EQU $F8D8
Intensity_5F EQU $F2A5
Wait_Recal EQU $F192
AU_MUSIC_READ_COUNT EQU $4332
INTENSITY_3F EQU $F2A1
DP_to_C8 EQU $F1AF
sfx_checknoisefreq EQU $43CC
MOVETO_X_7F EQU $F2F2
DELAY_RTS EQU $F57D
Init_Music_x EQU $F692
VEC_EXPL_CHANB EQU $C85D
VEC_BUTTON_2_3 EQU $C818
DRAW_VLCS EQU $F3D6
DRAW_VLP_7F EQU $F408
Vec_Twang_Table EQU $C851
Rise_Run_Angle EQU $F593
MOVE_MEM_A_1 EQU $F67F
_BOMBER_SHOT_SFX EQU $473F
VEC_MAX_GAMES EQU $C850
DLW_SEG2_DY_DONE EQU $40E0
Print_Str_hwyx EQU $F373
VEC_BTN_STATE EQU $C80F
INTENSITY_1F EQU $F29D
DRAW_VLP_FF EQU $F404
MOV_DRAW_VL EQU $F3BC
DLW_SEG2_DX_DONE EQU $4105
RESET0REF_D0 EQU $F34A
Vec_Expl_Chan EQU $C85C
MOVETO_IX EQU $F310
Vec_Counter_4 EQU $C831
Vec_RiseRun_Tmp EQU $C834
Vec_Expl_4 EQU $C85B
Vec_ADSR_Table EQU $C84F
sfx_checkvolume EQU $43DD
Print_Ships EQU $F393
Draw_VL_a EQU $F3DA
Vec_Counter_6 EQU $C833
musicd EQU $FF8F
VEC_DEFAULT_STK EQU $CBEA
DRAW_LINE_WRAPPER EQU $403A
ROT_VL_AB EQU $F610
music2 EQU $FD1D
DRAW_VL EQU $F3DD
Xform_Run_a EQU $F65B
_HIT_SFX EQU $46B4
Draw_VL EQU $F3DD
DEC_6_COUNTERS EQU $F55E
Vec_0Ref_Enable EQU $C824
READ_BTNS EQU $F1BA
OBJ_HIT EQU $F8FF
VEC_MUSIC_WK_A EQU $C842
VEC_RANDOM_SEED EQU $C87D
VEC_RFRSH_HI EQU $C83E
DSWM_NEXT_USE_OVERRIDE EQU $41E9
VEC_NUM_PLAYERS EQU $C879
Init_Music_chk EQU $F687
Mov_Draw_VLcs EQU $F3B5
Dot_List_Reset EQU $F2DE
VEC_EXPL_4 EQU $C85B
VEC_JOY_MUX_1_X EQU $C81F
DRAW_VL_MODE EQU $F46E
VEC_MUSIC_WK_7 EQU $C845
sfx_enablenoise EQU $441A
Vec_Music_Wk_A EQU $C842
VEC_JOY_2_X EQU $C81D
Vec_Buttons EQU $C811
Dec_3_Counters EQU $F55A
sfx_endofeffect EQU $4429
DSWM_NO_NEGATE_DX EQU $41B0
Rise_Run_Len EQU $F603
Mov_Draw_VLc_a EQU $F3AD
VEC_BUTTON_2_1 EQU $C816
Vec_Num_Game EQU $C87A


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
    ; Initialize SFX variables to prevent random noise on startup
    CLR >SFX_ACTIVE         ; Mark SFX as inactive (0=off)
    LDD #$0000
    STD >SFX_PTR            ; Clear SFX pointer
; Bank 0 ($0000) is active; fixed bank 31 ($4000-$7FFF) always visible
    JMP MAIN

;***************************************************************************

;***************************************************************************
; MAIN PROGRAM
;***************************************************************************

MAIN:
    ; Initialize global variables
    LDD #0
    STD VAR_PLAYING
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
    LDD VAR_PLAYING
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #0
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
    LBEQ .IF_0_ELSE
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_PLAYING
    ; PLAY_MUSIC("music1") - play music asset
    LDX #_MUSIC1_MUSIC  ; Load music data pointer
    JSR PLAY_MUSIC_RUNTIME
    LDD #0
    STD RESULT
    LBRA .IF_0_END
.IF_0_ELSE:
.IF_0_END:
    ; PRINT_TEXT: Print text at position
    LDD #-60
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #120
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_2282136835750346      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
    LDD #0
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+0    ; x0
    LDD #30
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+2    ; y0
    ; ABS: Absolute value
    LDD #-40
    STD RESULT
    LDD RESULT
    TSTA           ; Test sign bit
    LBPL .ABS_0_POS   ; Branch if positive
    COMA           ; Complement A
    COMB           ; Complement B
    ADDD #1        ; Add 1 for two's complement
.ABS_0_POS:
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+4    ; x1
    LDD #70
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+6    ; y1
    LDD #80
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+8    ; intensity
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
    LDD #0
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+0    ; x0
    LDD #30
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+2    ; y0
    ; MIN: Return minimum of two values
    LDD #-20
    STD RESULT
    LDD RESULT
    STD TMPPTR     ; Save first value
    LDD #50
    STD RESULT
    LDD TMPPTR     ; Load first value
    CMPD RESULT    ; Compare with second
    LBLE .MIN_1_FIRST ; Branch if first <= second
    LBRA .MIN_1_END
.MIN_1_FIRST:
    STD RESULT     ; First is smaller
.MIN_1_END:
    LDD RESULT
    STD DRAW_LINE_ARGS+4    ; x1
    ; MAX: Return maximum of two values
    LDD #70
    STD RESULT
    LDD RESULT
    STD TMPPTR     ; Save first value
    LDD #-50
    STD RESULT
    LDD TMPPTR     ; Load first value
    CMPD RESULT    ; Compare with second
    LBGE .MAX_2_FIRST ; Branch if first >= second
    LBRA .MAX_2_END
.MAX_2_FIRST:
    STD RESULT     ; First is larger
.MAX_2_END:
    LDD RESULT
    STD DRAW_LINE_ARGS+6    ; y1
    LDD #80
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+8    ; intensity
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LDA #$7F
    JSR Intensity_a
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$5A
    LDB #$B7
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$07
    LDB #$FF
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$05
    LDB #$FC
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$04
    LDB #$FB
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$02
    LDB #$F9
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FE
    LDB #$F9
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FC
    LDB #$FB
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FB
    LDB #$FC
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$F9
    LDB #$FE
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$F9
    LDB #$02
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FB
    LDB #$04
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FC
    LDB #$05
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FF
    LDB #$07
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$01
    LDB #$07
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$04
    LDB #$05
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$05
    LDB #$04
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$07
    LDB #$01
    JSR Draw_Line_d
    LDD #0
    STD RESULT
    ; DRAW_VECTOR: Draw vector asset at position
    ; Asset: logo (7 paths)
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD #-20
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA TMPPTR+1  ; Save Y to temporary storage
    LDA TMPPTR    ; X position
    STA DRAW_VEC_X
    LDA TMPPTR+1  ; Y position
    STA DRAW_VEC_Y
    CLR MIRROR_X
    CLR MIRROR_Y
    CLR DRAW_VEC_INTENSITY  ; Use intensity from vector data
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_LOGO_PATH0  ; Load path 0
    JSR Draw_Sync_List_At_With_Mirrors
    LDX #_LOGO_PATH1  ; Load path 1
    JSR Draw_Sync_List_At_With_Mirrors
    LDX #_LOGO_PATH2  ; Load path 2
    JSR Draw_Sync_List_At_With_Mirrors
    LDX #_LOGO_PATH3  ; Load path 3
    JSR Draw_Sync_List_At_With_Mirrors
    LDX #_LOGO_PATH4  ; Load path 4
    JSR Draw_Sync_List_At_With_Mirrors
    LDX #_LOGO_PATH5  ; Load path 5
    JSR Draw_Sync_List_At_With_Mirrors
    LDX #_LOGO_PATH6  ; Load path 6
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    ; DRAW_VECTOR_EX: Draw vector asset with transformations
    ; Asset: logo (7 paths) with mirror + intensity
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #-110
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDD #1
    STD RESULT
    LDB RESULT+1  ; Mirror mode (0=normal, 1=X, 2=Y, 3=both)
    ; Decode mirror mode into separate flags:
    CLR MIRROR_X  ; Clear X flag
    CLR MIRROR_Y  ; Clear Y flag
    CMPB #1       ; Check if X-mirror (mode 1)
    LBNE .DSVEX_0_CHK_Y
    LDA #1
    STA MIRROR_X
.DSVEX_0_CHK_Y:
    CMPB #2       ; Check if Y-mirror (mode 2)
    LBNE .DSVEX_0_CHK_XY
    LDA #1
    STA MIRROR_Y
.DSVEX_0_CHK_XY:
    CMPB #3       ; Check if both-mirror (mode 3)
    LBNE .DSVEX_0_CALL
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
.DSVEX_0_CALL:
    ; Set intensity override for drawing
    LDD #127
    STD RESULT
    LDA RESULT+1  ; Intensity (0-127)
    STA DRAW_VEC_INTENSITY  ; Store intensity override
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_LOGO_PATH0  ; Load path 0
    JSR Draw_Sync_List_At_With_Mirrors
    LDX #_LOGO_PATH1  ; Load path 1
    JSR Draw_Sync_List_At_With_Mirrors
    LDX #_LOGO_PATH2  ; Load path 2
    JSR Draw_Sync_List_At_With_Mirrors
    LDX #_LOGO_PATH3  ; Load path 3
    JSR Draw_Sync_List_At_With_Mirrors
    LDX #_LOGO_PATH4  ; Load path 4
    JSR Draw_Sync_List_At_With_Mirrors
    LDX #_LOGO_PATH5  ; Load path 5
    JSR Draw_Sync_List_At_With_Mirrors
    LDX #_LOGO_PATH6  ; Load path 6
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    CLR DRAW_VEC_INTENSITY  ; Clear intensity override for next draw
    LDD #0
    STD RESULT
    JSR AUDIO_UPDATE  ; Auto-injected: update music + SFX (after all game logic)
    RTS


; ================================================
