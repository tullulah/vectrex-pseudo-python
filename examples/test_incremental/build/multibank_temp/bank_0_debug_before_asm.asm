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
VAR_PLAYING          EQU $C880+$3B   ; User variable: PLAYING (2 bytes)
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
Moveto_d EQU $F312
RESET0REF_D0 EQU $F34A
DLW_SEG2_DX_CHECK_NEG EQU $40F4
Vec_Freq_Table EQU $C84D
VEC_COUNTER_5 EQU $C832
_COIN_SFX EQU $465B
_HIT_SFX EQU $46B4
Reset_Pen EQU $F35B
Vec_Joy_Mux_2_X EQU $C821
Clear_x_b_80 EQU $F550
NEW_HIGH_SCORE EQU $F8D8
Moveto_d_7F EQU $F2FC
VEC_MISC_COUNT EQU $C823
PMr_start_new EQU $4272
PRINT_SHIPS EQU $F393
Delay_RTS EQU $F57D
sfx_disablenoise EQU $440D
VEC_MUSIC_CHAN EQU $C855
DLW_SEG2_DX_NO_REMAIN EQU $4102
Dot_here EQU $F2C5
ADD_SCORE_D EQU $F87C
DRAW_GRID_VL EQU $FF9F
PSG_frame_done EQU $42D3
Vec_FIRQ_Vector EQU $CBF5
MOVETO_IX_FF EQU $F308
MOD16.MOD16_LOOP EQU $401D
VEC_RFRSH EQU $C83D
VEC_MUSIC_WK_7 EQU $C845
Vec_Expl_3 EQU $C85A
RESET_PEN EQU $F35B
VECTREX_PRINT_TEXT EQU $4000
Vec_Joy_2_Y EQU $C81E
UPDATE_MUSIC_PSG EQU $4281
Init_VIA EQU $F14C
DRAW_VL_A EQU $F3DA
MUSICA EQU $FF44
noay EQU $439E
AU_MUSIC_ENDED EQU $436A
Vec_ADSR_Timers EQU $C85E
PRINT_LIST_HW EQU $F385
Intensity_3F EQU $F2A1
Vec_Angle EQU $C836
DSWM_NEXT_SET_INTENSITY EQU $41EB
Print_Str_d EQU $F37A
DSWM_NEXT_NO_NEGATE_X EQU $4204
INTENSITY_A EQU $F2AB
MOV_DRAW_VLC_A EQU $F3AD
DSWM_NEXT_NO_NEGATE_Y EQU $41F7
Vec_Joy_1_Y EQU $C81C
Vec_Button_1_1 EQU $C812
AU_MUSIC_READ EQU $431D
Vec_Counter_3 EQU $C830
Print_List_hw EQU $F385
Rot_VL_dft EQU $F637
Random EQU $F517
musicc EQU $FF7A
Dot_ix EQU $F2C1
Vec_Button_2_1 EQU $C816
PSG_update_done EQU $42E7
sfx_disabletone EQU $43EE
Vec_Joy_Mux EQU $C81F
VEC_MUSIC_WK_5 EQU $C847
VEC_TEXT_HEIGHT EQU $C82A
Vec_Music_Work EQU $C83F
music6 EQU $FE76
Vec_Num_Game EQU $C87A
Vec_Duration EQU $C857
VEC_SWI3_VECTOR EQU $CBF2
VEC_EXPL_CHANA EQU $C853
DLW_SEG1_DY_READY EQU $4084
CHECK0REF EQU $F34F
DRAW_LINE_D EQU $F3DF
RANDOM EQU $F517
CLEAR_X_B_A EQU $F552
VEC_RISERUN_TMP EQU $C834
music2 EQU $FD1D
READ_BTNS_MASK EQU $F1B4
MOV_DRAW_VL_B EQU $F3B1
Print_Str_hwyx EQU $F373
Abs_a_b EQU $F584
AU_DONE EQU $4388
_LOGO_PATH1 EQU $448D
Init_Music_Buf EQU $F533
sfx_enablenoise EQU $441A
AU_MUSIC_WRITE_LOOP EQU $434D
MUSIC4 EQU $FDD3
_BONUS_COLLECTED_SFX EQU $4615
RESET0INT EQU $F36B
PRINT_SHIPS_X EQU $F391
Rot_VL_Mode_a EQU $F61F
Vec_Joy_2_X EQU $C81D
Vec_Default_Stk EQU $CBEA
CLEAR_X_B_80 EQU $F550
_EXPLOSION1_SFX EQU $4631
Vec_Counter_2 EQU $C82F
Vec_0Ref_Enable EQU $C824
INTENSITY_3F EQU $F2A1
VEC_JOY_RESLTN EQU $C81A
VEC_JOY_MUX_1_X EQU $C81F
MOV_DRAW_VLCS EQU $F3B5
VEC_COUNTER_6 EQU $C833
Check0Ref EQU $F34F
VEC_JOY_MUX EQU $C81F
MUSIC7 EQU $FEC6
Vec_Joy_Mux_1_Y EQU $C820
VEC_BUTTON_2_3 EQU $C818
DRAW_VL_AB EQU $F3D8
musicd EQU $FF8F
DSWM_LOOP EQU $418E
Xform_Run EQU $F65D
New_High_Score EQU $F8D8
VEC_MUSIC_WORK EQU $C83F
Vec_Button_1_3 EQU $C814
RESET0REF EQU $F354
VEC_JOY_MUX_2_Y EQU $C822
Clear_x_256 EQU $F545
Get_Rise_Run EQU $F5EF
PSG_music_ended EQU $42D9
DSWM_DONE EQU $4263
DP_TO_C8 EQU $F1AF
Read_Btns_Mask EQU $F1B4
VEC_STR_PTR EQU $C82C
Dot_List_Reset EQU $F2DE
Vec_Buttons EQU $C811
Vec_Music_Wk_5 EQU $C847
Vec_Counter_6 EQU $C833
_LOGO_PATH6 EQU $4526
DLW_SEG1_DX_NO_CLAMP EQU $40A4
Vec_High_Score EQU $CBEB
Print_Str EQU $F495
Vec_Music_Wk_A EQU $C842
VEC_NUM_PLAYERS EQU $C879
DLW_SEG2_DY_DONE EQU $40E0
VEC_EXPL_CHANS EQU $C854
Sound_Bytes_x EQU $F284
Mov_Draw_VL EQU $F3BC
Reset0Ref EQU $F354
AU_UPDATE_SFX EQU $437E
Vec_ADSR_Table EQU $C84F
VEC_ADSR_TIMERS EQU $C85E
RISE_RUN_ANGLE EQU $F593
_LOGO_PATH4 EQU $44D8
Print_Ships EQU $F393
DP_TO_D0 EQU $F1AA
Vec_Counter_1 EQU $C82E
STRIP_ZEROS EQU $F8B7
_LOGO_VECTORS EQU $4451
Mov_Draw_VLcs EQU $F3B5
Vec_Rise_Index EQU $C839
VEC_NUM_GAME EQU $C87A
Vec_IRQ_Vector EQU $CBF8
VEC_RISE_INDEX EQU $C839
DP_to_C8 EQU $F1AF
Init_Music_x EQU $F692
SOUND_BYTES EQU $F27D
ROT_VL_MODE_A EQU $F61F
DLW_NEED_SEG2 EQU $40CC
Draw_VLp_b EQU $F40E
VEC_MUSIC_FREQ EQU $C861
VEC_TWANG_TABLE EQU $C851
_JUMP_SFX EQU $459E
Vec_Text_HW EQU $C82A
Vec_Counters EQU $C82E
Vec_Run_Index EQU $C837
Clear_Sound EQU $F272
PLAY_MUSIC_RUNTIME EQU $4264
Vec_Max_Games EQU $C850
DOT_LIST_RESET EQU $F2DE
XFORM_RISE EQU $F663
SELECT_GAME EQU $F7A9
DRAW_VLP_B EQU $F40E
VEC_MUSIC_TWANG EQU $C858
Vec_Music_Flag EQU $C856
MOVETO_X_7F EQU $F2F2
Rot_VL_Mode EQU $F62B
JOY_ANALOG EQU $F1F5
Clear_x_b EQU $F53F
Random_3 EQU $F511
Wait_Recal EQU $F192
VEC_FIRQ_VECTOR EQU $CBF5
MOVETO_D EQU $F312
Print_List_chk EQU $F38C
VEC_IRQ_VECTOR EQU $CBF8
musica EQU $FF44
VEC_MUSIC_WK_1 EQU $C84B
DLW_SEG2_DX_DONE EQU $4105
Draw_VL EQU $F3DD
WAIT_RECAL EQU $F192
VEC_MUSIC_FLAG EQU $C856
Move_Mem_a EQU $F683
Dot_d EQU $F2C3
VEC_EXPL_2 EQU $C859
Vec_Expl_Timer EQU $C877
sfx_enabletone EQU $43FB
DP_to_D0 EQU $F1AA
DRAW_PAT_VL EQU $F437
Explosion_Snd EQU $F92E
Cold_Start EQU $F000
Vec_Expl_ChanA EQU $C853
MOV_DRAW_VL EQU $F3BC
Print_Ships_x EQU $F391
Intensity_1F EQU $F29D
Vec_Max_Players EQU $C84F
DELAY_RTS EQU $F57D
VEC_EXPL_3 EQU $C85A
VEC_COLD_FLAG EQU $CBFE
VEC_HIGH_SCORE EQU $CBEB
GET_RISE_RUN EQU $F5EF
VEC_SND_SHADOW EQU $C800
VEC_BUTTON_1_2 EQU $C813
Vec_Button_2_4 EQU $C819
VEC_EXPL_1 EQU $C858
Vec_Music_Ptr EQU $C853
VEC_0REF_ENABLE EQU $C824
Vec_Expl_1 EQU $C858
PRINT_STR_D EQU $F37A
DLW_DONE EQU $4114
DEC_6_COUNTERS EQU $F55E
PLAY_SFX_RUNTIME EQU $438B
DRAW_LINE_WRAPPER EQU $403A
Rot_VL_ab EQU $F610
DSWM_NO_NEGATE_X EQU $4141
VEC_JOY_MUX_1_Y EQU $C820
AU_MUSIC_PROCESS_WRITES EQU $434B
Vec_Twang_Table EQU $C851
_STAR_VRELEASE_SFX EQU $45BB
CLEAR_C8_RAM EQU $F542
MOVETO_IX_A EQU $F30E
DSWM_SET_INTENSITY EQU $4127
MUSICC EQU $FF7A
Vec_Expl_ChanB EQU $C85D
COMPARE_SCORE EQU $F8C7
DLW_SEG1_DY_NO_CLAMP EQU $4081
PMr_done EQU $4280
VEC_EXPL_4 EQU $C85B
VEC_EXPL_FLAG EQU $C867
MUSIC9 EQU $FF26
Print_Str_yx EQU $F378
DSWM_NEXT_USE_OVERRIDE EQU $41E9
DSWM_W1 EQU $4185
_BOMBER_SHOT_SFX EQU $473F
PRINT_LIST_CHK EQU $F38C
music8 EQU $FEF8
Obj_Hit EQU $F8FF
MUSIC2 EQU $FD1D
Vec_Button_2_3 EQU $C818
Delay_b EQU $F57A
VEC_RANDOM_SEED EQU $C87D
Vec_Prev_Btns EQU $C810
PRINT_TEXT_STR_3232159404 EQU $443F
Vec_Num_Players EQU $C879
MOD16.MOD16_END EQU $4035
PRINT_STR_YX EQU $F378
Init_OS_RAM EQU $F164
EXPLOSION_SND EQU $F92E
Mov_Draw_VL_a EQU $F3B9
VEC_JOY_1_Y EQU $C81C
Rise_Run_Angle EQU $F593
music1 EQU $FD0D
MUSICB EQU $FF62
Strip_Zeros EQU $F8B7
Vec_Rfrsh EQU $C83D
sfx_checktonefreq EQU $43B2
DSWM_W3 EQU $4254
Vec_Button_1_4 EQU $C815
Delay_2 EQU $F571
VEC_DOT_DWELL EQU $C828
Intensity_a EQU $F2AB
Dec_Counters EQU $F563
Delay_3 EQU $F56D
OBJ_WILL_HIT EQU $F8F3
Vec_Dot_Dwell EQU $C828
VEC_LOOP_COUNT EQU $C825
MOVE_MEM_A EQU $F683
VEC_PATTERN EQU $C829
sfx_nextframe EQU $4424
DELAY_0 EQU $F579
Obj_Will_Hit EQU $F8F3
Sound_Byte_raw EQU $F25B
CLEAR_X_D EQU $F548
MUSIC6 EQU $FE76
Draw_Pat_VL EQU $F437
INTENSITY_7F EQU $F2A9
VEC_BRIGHTNESS EQU $C827
Draw_VLc EQU $F3CE
Init_OS EQU $F18B
JOY_DIGITAL EQU $F1F8
Moveto_ix_FF EQU $F308
OBJ_WILL_HIT_U EQU $F8E5
VEC_MUSIC_WK_6 EQU $C846
Xform_Rise_a EQU $F661
RISE_RUN_Y EQU $F601
INIT_OS EQU $F18B
VEC_ANGLE EQU $C836
DRAW_VLP_7F EQU $F408
INTENSITY_5F EQU $F2A5
AU_MUSIC_READ_COUNT EQU $4332
MUSICD EQU $FF8F
Sound_Byte EQU $F256
sfx_checknoisefreq EQU $43CC
STOP_MUSIC_RUNTIME EQU $42EB
DOT_LIST EQU $F2D5
Intensity_7F EQU $F2A9
VEC_BUTTON_2_2 EQU $C817
ADD_SCORE_A EQU $F85E
BITMASK_A EQU $F57E
Vec_Joy_Mux_1_X EQU $C81F
MOVETO_D_7F EQU $F2FC
Moveto_x_7F EQU $F2F2
Recalibrate EQU $F2E6
DRAW_PAT_VL_A EQU $F434
PSG_music_loop EQU $42DF
VEC_ADSR_TABLE EQU $C84F
Vec_Expl_4 EQU $C85B
Dec_6_Counters EQU $F55E
SOUND_BYTE EQU $F256
DELAY_1 EQU $F575
Moveto_ix EQU $F310
VEC_MAX_GAMES EQU $C850
Bitmask_a EQU $F57E
music9 EQU $FF26
Vec_Rfrsh_lo EQU $C83D
Vec_Pattern EQU $C829
SOUND_BYTE_X EQU $F259
INIT_VIA EQU $F14C
SET_REFRESH EQU $F1A2
PSG_write_loop EQU $42A2
Vec_Cold_Flag EQU $CBFE
VEC_JOY_2_X EQU $C81D
Vec_Expl_Chans EQU $C854
Vec_Counter_4 EQU $C831
ROT_VL EQU $F616
DRAW_VLC EQU $F3CE
Draw_VL_a EQU $F3DA
Get_Run_Idx EQU $F5DB
VEC_BUTTONS EQU $C811
VEC_BTN_STATE EQU $C80F
Draw_Pat_VL_d EQU $F439
Vec_Music_Freq EQU $C861
INIT_OS_RAM EQU $F164
Delay_1 EQU $F575
Vec_Button_1_2 EQU $C813
Vec_Seed_Ptr EQU $C87B
GET_RUN_IDX EQU $F5DB
DRAW_PAT_VL_D EQU $F439
DOT_IX_B EQU $F2BE
Do_Sound_x EQU $F28C
music7 EQU $FEC6
Compare_Score EQU $F8C7
DOT_IX EQU $F2C1
Mov_Draw_VL_b EQU $F3B1
_LOGO_PATH0 EQU $4460
DRAW_VLP_SCALE EQU $F40C
Vec_Text_Width EQU $C82B
INIT_MUSIC EQU $F68D
Rise_Run_Len EQU $F603
Draw_VL_ab EQU $F3D8
DRAW_VLP EQU $F410
VEC_SEED_PTR EQU $C87B
Vec_NMI_Vector EQU $CBFB
_MUSIC1_MUSIC EQU $4538
XFORM_RUN_A EQU $F65B
Joy_Analog EQU $F1F5
VEC_BUTTON_1_1 EQU $C812
VEC_MUSIC_WK_A EQU $C842
DELAY_B EQU $F57A
Vec_Misc_Count EQU $C823
Select_Game EQU $F7A9
Joy_Digital EQU $F1F8
Vec_Button_2_2 EQU $C817
Move_Mem_a_1 EQU $F67F
VEC_BUTTON_1_3 EQU $C814
VEC_COUNTERS EQU $C82E
VEC_JOY_2_Y EQU $C81E
ABS_B EQU $F58B
sfx_checktonedisable EQU $43E6
AU_MUSIC_NO_DELAY EQU $4332
Obj_Will_Hit_u EQU $F8E5
Dot_List EQU $F2D5
VEC_DURATION EQU $C857
Vec_Expl_Chan EQU $C85C
VEC_RFRSH_HI EQU $C83E
Mov_Draw_VLc_a EQU $F3AD
Intensity_5F EQU $F2A5
CLEAR_SCORE EQU $F84F
DELAY_3 EQU $F56D
RISE_RUN_X EQU $F5FF
Vec_Expl_2 EQU $C859
SOUND_BYTE_RAW EQU $F25B
MOVETO_IX_7F EQU $F30C
WARM_START EQU $F06C
Clear_C8_RAM EQU $F542
_LOGO_PATH2 EQU $44B4
RECALIBRATE EQU $F2E6
READ_BTNS EQU $F1BA
Draw_VL_mode EQU $F46E
RANDOM_3 EQU $F511
Vec_Music_Chan EQU $C855
Vec_Snd_Shadow EQU $C800
Vec_Counter_5 EQU $C832
Draw_VLcs EQU $F3D6
Reset0Ref_D0 EQU $F34A
ROT_VL_AB EQU $F610
Draw_Grid_VL EQU $FF9F
DSWM_NEXT_PATH EQU $41D9
AU_SKIP_MUSIC EQU $437B
music3 EQU $FD81
INTENSITY_1F EQU $F29D
Rot_VL EQU $F616
PRINT_STR_HWYX EQU $F373
DEC_COUNTERS EQU $F563
Vec_Joy_1_X EQU $C81B
XFORM_RUN EQU $F65D
DRAW_VLP_FF EQU $F404
Vec_SWI3_Vector EQU $CBF2
Draw_Sync_List_At_With_Mirrors EQU $4119
Dot_ix_b EQU $F2BE
PRINT_LIST EQU $F38A
Init_Music_chk EQU $F687
Vec_Btn_State EQU $C80F
Moveto_ix_a EQU $F30E
ROT_VL_MODE EQU $F62B
INIT_MUSIC_BUF EQU $F533
VEC_MUSIC_PTR EQU $C853
DSWM_USE_OVERRIDE EQU $4125
Vec_Rfrsh_hi EQU $C83E
ROT_VL_DFT EQU $F637
VEC_RUN_INDEX EQU $C837
Draw_Pat_VL_a EQU $F434
AU_MUSIC_DONE EQU $4364
sfx_doframe EQU $439F
DOT_D EQU $F2C3
VEC_BUTTON_2_4 EQU $C819
Sound_Bytes EQU $F27D
sfx_checknoisedisable EQU $4405
VEC_EXPL_CHANB EQU $C85D
Delay_0 EQU $F579
Vec_Brightness EQU $C827
PRINT_TEXT_STR_3327403 EQU $443A
Vec_Music_Wk_1 EQU $C84B
DLW_SEG1_DX_READY EQU $40A7
VEC_RFRSH_LO EQU $C83D
Xform_Rise EQU $F663
Vec_SWI2_Vector EQU $CBF2
VEC_EXPL_TIMER EQU $C877
Init_Music EQU $F68D
PRINT_STR EQU $F495
Clear_Score EQU $F84F
Mov_Draw_VL_ab EQU $F3B7
VEC_SWI2_VECTOR EQU $CBF2
SFX_UPDATE EQU $4394
Draw_VLp_scale EQU $F40C
Mov_Draw_VL_d EQU $F3BE
Draw_Line_d EQU $F3DF
VEC_SWI_VECTOR EQU $CBFB
MUSIC1 EQU $FD0D
VEC_EXPL_CHAN EQU $C85C
Draw_VLp EQU $F410
VEC_RISERUN_LEN EQU $C83B
RISE_RUN_LEN EQU $F603
VEC_PREV_BTNS EQU $C810
Dec_3_Counters EQU $F55A
DEC_3_COUNTERS EQU $F55A
CLEAR_X_B EQU $F53F
DRAW_VLCS EQU $F3D6
VEC_COUNTER_4 EQU $C831
DLW_SEG1_DY_LO EQU $4074
PRINT_TEXT_STR_2282136835750346 EQU $4446
MUSIC3 EQU $FD81
Vec_Expl_Flag EQU $C867
Vec_Joy_Resltn EQU $C81A
Vec_Music_Wk_6 EQU $C846
AU_MUSIC_HAS_DELAY EQU $4341
GET_RISE_IDX EQU $F5D9
Vec_Music_Wk_7 EQU $C845
Add_Score_a EQU $F85E
MOVETO_IX EQU $F310
Get_Rise_Idx EQU $F5D9
DRAW_VL_B EQU $F3D2
Rise_Run_X EQU $F5FF
DRAW_VL_MODE EQU $F46E
Do_Sound EQU $F289
CLEAR_SOUND EQU $F272
DSWM_NO_NEGATE_Y EQU $4134
Set_Refresh EQU $F1A2
Xform_Run_a EQU $F65B
COLD_START EQU $F000
VEC_JOY_MUX_2_X EQU $C821
_LOGO_PATH5 EQU $44FF
VEC_COUNTER_2 EQU $C82F
MOD16 EQU $401B
SOUND_BYTES_X EQU $F284
Draw_VL_b EQU $F3D2
DLW_SEG1_DX_LO EQU $4097
VEC_TEXT_WIDTH EQU $C82B
DO_SOUND_X EQU $F28C
Rise_Run_Y EQU $F601
DSWM_W2 EQU $41CA
VEC_MAX_PLAYERS EQU $C84F
Abs_b EQU $F58B
VEC_COUNTER_3 EQU $C830
sfx_endofeffect EQU $4429
ABS_A_B EQU $F584
Vec_Str_Ptr EQU $C82C
VEC_JOY_1_X EQU $C81B
MUSIC8 EQU $FEF8
VEC_FREQ_TABLE EQU $C84D
VEC_NMI_VECTOR EQU $CBFB
CLEAR_X_256 EQU $F545
OBJ_HIT EQU $F8FF
_LASER_SFX EQU $46F2
VEC_BUTTON_1_4 EQU $C815
Reset0Int EQU $F36B
Draw_VLp_FF EQU $F404
AUDIO_UPDATE EQU $42F5
DELAY_2 EQU $F571
Moveto_ix_7F EQU $F30C
musicb EQU $FF62
AU_MUSIC_LOOP EQU $4370
DRAW_VL EQU $F3DD
Draw_VLp_7F EQU $F408
Warm_Start EQU $F06C
MOVE_MEM_A_1 EQU $F67F
XFORM_RISE_A EQU $F661
VEC_COUNTER_1 EQU $C82E
VEC_TEXT_HW EQU $C82A
MUSIC5 EQU $FE38
Vec_RiseRun_Tmp EQU $C834
Vec_Music_Twang EQU $C858
_LOGO_PATH3 EQU $44C6
DOT_HERE EQU $F2C5
music4 EQU $FDD3
Vec_Random_Seed EQU $C87D
DLW_SEG2_DY_POS EQU $40DD
INIT_MUSIC_CHK EQU $F687
DSWM_NO_NEGATE_DX EQU $41B0
VEC_BUTTON_2_1 EQU $C816
DO_SOUND EQU $F289
Read_Btns EQU $F1BA
Clear_x_b_a EQU $F552
sfx_checkvolume EQU $43DD
Vec_Loop_Count EQU $C825
MOV_DRAW_VL_D EQU $F3BE
Print_List EQU $F38A
Vec_SWI_Vector EQU $CBFB
music5 EQU $FE38
Clear_x_d EQU $F548
Vec_Joy_Mux_2_Y EQU $C822
INIT_MUSIC_X EQU $F692
MOV_DRAW_VL_AB EQU $F3B7
Sound_Byte_x EQU $F259
VEC_DEFAULT_STK EQU $CBEA
DSWM_NO_NEGATE_DY EQU $41A6
Vec_RiseRun_Len EQU $C83B
MOV_DRAW_VL_A EQU $F3B9
Add_Score_d EQU $F87C
Vec_Text_Height EQU $C82A


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
    LBEQ IF_NEXT_1
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_PLAYING
    ; PLAY_MUSIC("music1") - play music asset
    LDX #_MUSIC1_MUSIC  ; Load music data pointer
    JSR PLAY_MUSIC_RUNTIME
    LDD #0
    STD RESULT
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
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
