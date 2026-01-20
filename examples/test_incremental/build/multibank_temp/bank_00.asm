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
VAR_TITLE_INTENSITY  EQU $C880+$3B   ; User variable: TITLE_INTENSITY (2 bytes)
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
Moveto_d_7F EQU $F2FC
DRAW_VECTOR_BANKED EQU $400C
Vec_Default_Stk EQU $CBEA
VEC_SND_SHADOW EQU $C800
Rise_Run_Angle EQU $F593
Get_Rise_Idx EQU $F5D9
DELAY_2 EQU $F571
PMr_start_new EQU $4204
DSWM_NEXT_NO_NEGATE_X EQU $4196
ROT_VL_DFT EQU $F637
music3 EQU $FD81
Vec_SWI3_Vector EQU $CBF2
Obj_Will_Hit_u EQU $F8E5
Draw_Grid_VL EQU $FF9F
Clear_x_d EQU $F548
PSG_WRITE_LOOP EQU $4234
Draw_VLp EQU $F410
VEC_RANDOM_SEED EQU $C87D
WARM_START EQU $F06C
Vec_Counter_6 EQU $C833
Vec_Music_Wk_5 EQU $C847
ASSET_ADDR_TABLE EQU $4008
Reset0Ref EQU $F354
Vec_Music_Wk_6 EQU $C846
RANDOM_3 EQU $F511
VEC_RISERUN_LEN EQU $C83B
VEC_BRIGHTNESS EQU $C827
MOV_DRAW_VL_B EQU $F3B1
VEC_JOY_MUX EQU $C81F
CHECK0REF EQU $F34F
Clear_C8_RAM EQU $F542
Random EQU $F517
Vec_Joy_1_Y EQU $C81C
Rise_Run_Y EQU $F601
ASSET_BANK_TABLE EQU $4006
VEC_JOY_RESLTN EQU $C81A
Rot_VL_Mode_a EQU $F61F
DOT_IX EQU $F2C1
ADD_SCORE_A EQU $F85E
SFX_ENABLETONE EQU $438D
PRINT_TEXT_STR_3232159404 EQU $43D6
Vec_Joy_Mux_1_X EQU $C81F
DEC_6_COUNTERS EQU $F55E
INTENSITY_A EQU $F2AB
DRAW_SYNC_LIST_AT_WITH_MIRRORS EQU $40AB
Vec_ADSR_Table EQU $C84F
VEC_MUSIC_WORK EQU $C83F
PSG_frame_done EQU $4265
MOV_DRAW_VL_AB EQU $F3B7
ADD_SCORE_D EQU $F87C
Print_Str EQU $F495
Vec_Music_Twang EQU $C858
Vec_IRQ_Vector EQU $CBF8
Obj_Will_Hit EQU $F8F3
Vec_Expl_ChanB EQU $C85D
SOUND_BYTE_RAW EQU $F25B
SOUND_BYTE_X EQU $F259
PRINT_TEXT_STR_2223292 EQU $43CC
Print_Str_yx EQU $F378
MOVETO_IX EQU $F310
Vec_ADSR_Timers EQU $C85E
VEC_COLD_FLAG EQU $CBFE
SOUND_BYTES EQU $F27D
DEC_3_COUNTERS EQU $F55A
Vec_Prev_Btns EQU $C810
VEC_MUSIC_WK_1 EQU $C84B
Vec_Button_2_1 EQU $C816
DRAW_VLP_SCALE EQU $F40C
Reset0Int EQU $F36B
VEC_JOY_MUX_2_X EQU $C821
SFX_ENABLENOISE EQU $43AC
Vec_Pattern EQU $C829
AU_MUSIC_LOOP EQU $4302
Abs_b EQU $F58B
Dot_d EQU $F2C3
VEC_ADSR_TABLE EQU $C84F
VEC_RUN_INDEX EQU $C837
Sound_Bytes EQU $F27D
Rot_VL EQU $F616
VEC_FREQ_TABLE EQU $C84D
Vec_Expl_1 EQU $C858
VEC_STR_PTR EQU $C82C
Abs_a_b EQU $F584
VEC_RISE_INDEX EQU $C839
VEC_EXPL_CHANB EQU $C85D
VEC_HIGH_SCORE EQU $CBEB
SFX_DOFRAME EQU $4331
SFX_CHECKTONEFREQ EQU $4344
VEC_JOY_1_Y EQU $C81C
Vec_Max_Games EQU $C850
VEC_MAX_GAMES EQU $C850
music1 EQU $FD0D
PRINT_SHIPS EQU $F393
Get_Rise_Run EQU $F5EF
DSWM_NEXT_PATH EQU $416B
Vec_Music_Chan EQU $C855
VEC_BUTTON_1_1 EQU $C812
INIT_VIA EQU $F14C
Draw_VLp_7F EQU $F408
music4 EQU $FDD3
VEC_EXPL_CHANA EQU $C853
DRAW_VL_MODE EQU $F46E
PLAY_SFX_RUNTIME EQU $431D
ABS_A_B EQU $F584
Move_Mem_a_1 EQU $F67F
UPDATE_MUSIC_PSG EQU $4213
Reset_Pen EQU $F35B
Mov_Draw_VL_ab EQU $F3B7
Rise_Run_Len EQU $F603
AU_MUSIC_HAS_DELAY EQU $42D3
MOVETO_IX_FF EQU $F308
DELAY_B EQU $F57A
DSWM_SET_INTENSITY EQU $40B9
VEC_EXPL_1 EQU $C858
Draw_VLp_b EQU $F40E
Intensity_7F EQU $F2A9
DELAY_0 EQU $F579
COLD_START EQU $F000
VEC_BUTTONS EQU $C811
Vec_Text_Width EQU $C82B
Vec_Duration EQU $C857
GET_RISE_IDX EQU $F5D9
Vec_Random_Seed EQU $C87D
MUSICA EQU $FF44
AU_SKIP_MUSIC EQU $430D
RESET_PEN EQU $F35B
AUDIO_UPDATE EQU $4287
Vec_Joy_Mux_2_Y EQU $C822
ROT_VL_AB EQU $F610
Explosion_Snd EQU $F92E
sfx_checknoisedisable EQU $4397
PRINT_LIST_HW EQU $F385
DRAW_VL EQU $F3DD
sfx_nextframe EQU $43B6
MUSIC8 EQU $FEF8
MUSIC_ADDR_TABLE EQU $4004
Vec_Buttons EQU $C811
Vec_Joy_Resltn EQU $C81A
music5 EQU $FE38
_LOGO_PATH2 EQU $0063
AU_DONE EQU $431A
VEC_BUTTON_2_1 EQU $C816
VEC_JOY_MUX_2_Y EQU $C822
PSG_MUSIC_LOOP EQU $4271
RISE_RUN_Y EQU $F601
DP_to_D0 EQU $F1AA
Sound_Byte_x EQU $F259
VEC_EXPL_2 EQU $C859
Intensity_5F EQU $F2A5
DELAY_1 EQU $F575
CLEAR_X_D EQU $F548
Moveto_ix_a EQU $F30E
VEC_FIRQ_VECTOR EQU $CBF5
VEC_EXPL_TIMER EQU $C877
Rot_VL_dft EQU $F637
DOT_LIST EQU $F2D5
Draw_Sync_List_At_With_Mirrors EQU $40AB
VECTOR_BANK_TABLE EQU $4000
VEC_NUM_PLAYERS EQU $C879
VEC_TEXT_HW EQU $C82A
PRINT_SHIPS_X EQU $F391
Vec_Cold_Flag EQU $CBFE
VEC_DOT_DWELL EQU $C828
Vec_Dot_Dwell EQU $C828
Vec_Music_Wk_7 EQU $C845
Draw_Pat_VL EQU $F437
MOVETO_IX_A EQU $F30E
Delay_1 EQU $F575
musicb EQU $FF62
VEC_MUSIC_WK_5 EQU $C847
VEC_EXPL_4 EQU $C85B
INIT_OS EQU $F18B
JOY_DIGITAL EQU $F1F8
CLEAR_SCORE EQU $F84F
VEC_MUSIC_WK_A EQU $C842
Vec_FIRQ_Vector EQU $CBF5
Vec_Counters EQU $C82E
Vec_Expl_Timer EQU $C877
RISE_RUN_ANGLE EQU $F593
MUSICC EQU $FF7A
sfx_checkvolume EQU $436F
Draw_Pat_VL_a EQU $F434
VEC_COUNTER_5 EQU $C832
Draw_VL_a EQU $F3DA
Dot_ix EQU $F2C1
DRAW_VLP_B EQU $F40E
PSG_music_loop EQU $4271
PMr_done EQU $4212
Draw_VL_ab EQU $F3D8
Mov_Draw_VL_a EQU $F3B9
VEC_BUTTON_1_3 EQU $C814
READ_BTNS_MASK EQU $F1B4
Vec_Music_Work EQU $C83F
Init_Music EQU $F68D
sfx_doframe EQU $4331
VEC_COUNTER_2 EQU $C82F
Print_Ships_x EQU $F391
MUSIC2 EQU $FD1D
VEC_0REF_ENABLE EQU $C824
Vec_Counter_1 EQU $C82E
DRAW_GRID_VL EQU $FF9F
VEC_MUSIC_WK_7 EQU $C845
AU_MUSIC_READ_COUNT EQU $42C4
DSWM_NEXT_USE_OVERRIDE EQU $417B
Moveto_ix EQU $F310
music7 EQU $FEC6
SFX_CHECKVOLUME EQU $436F
Vec_Text_Height EQU $C82A
ROT_VL EQU $F616
Intensity_3F EQU $F2A1
VEC_TEXT_WIDTH EQU $C82B
VEC_MUSIC_WK_6 EQU $C846
SET_REFRESH EQU $F1A2
PLAY_MUSIC_BANKED EQU $4046
Vec_Snd_Shadow EQU $C800
Draw_Line_d EQU $F3DF
DEC_COUNTERS EQU $F563
musicc EQU $FF7A
Draw_VL EQU $F3DD
PRINT_STR_HWYX EQU $F373
Sound_Byte EQU $F256
ROT_VL_MODE_A EQU $F61F
sfx_checktonefreq EQU $4344
Draw_VLc EQU $F3CE
INTENSITY_3F EQU $F2A1
VEC_MAX_PLAYERS EQU $C84F
INIT_MUSIC_BUF EQU $F533
VEC_RFRSH_LO EQU $C83D
Cold_Start EQU $F000
music6 EQU $FE76
Vec_Twang_Table EQU $C851
VEC_COUNTER_1 EQU $C82E
SFX_DISABLETONE EQU $4380
Rot_VL_ab EQU $F610
SFX_CHECKNOISEFREQ EQU $435E
VEC_DEFAULT_STK EQU $CBEA
Vec_Joy_Mux_2_X EQU $C821
INIT_MUSIC EQU $F68D
SFX_NEXTFRAME EQU $43B6
VEC_MUSIC_PTR EQU $C853
_LOGO_PATH4 EQU $0087
VECTOR_ADDR_TABLE EQU $4001
CLEAR_X_256 EQU $F545
Vec_SWI_Vector EQU $CBFB
XFORM_RISE EQU $F663
CLEAR_SOUND EQU $F272
Print_Str_d EQU $F37A
PRINT_LIST_CHK EQU $F38C
VEC_IRQ_VECTOR EQU $CBF8
ABS_B EQU $F58B
Draw_VLp_FF EQU $F404
MOVETO_IX_7F EQU $F30C
Print_List EQU $F38A
MOD16.MOD16_LOOP EQU $408E
MOVE_MEM_A EQU $F683
Mov_Draw_VL_d EQU $F3BE
DSWM_USE_OVERRIDE EQU $40B7
Vec_Expl_Chan EQU $C85C
INIT_MUSIC_X EQU $F692
Print_List_hw EQU $F385
Dec_6_Counters EQU $F55E
Bitmask_a EQU $F57E
VEC_JOY_MUX_1_X EQU $C81F
sfx_disablenoise EQU $439F
PSG_FRAME_DONE EQU $4265
RISE_RUN_X EQU $F5FF
VEC_PREV_BTNS EQU $C810
Vec_Rfrsh EQU $C83D
Xform_Run EQU $F65D
Vec_Expl_Chans EQU $C854
AU_MUSIC_ENDED EQU $42FC
Move_Mem_a EQU $F683
Vec_Counter_4 EQU $C831
VEC_EXPL_CHAN EQU $C85C
MOV_DRAW_VLCS EQU $F3B5
VEC_LOOP_COUNT EQU $C825
Vec_Counter_3 EQU $C830
Vec_Music_Wk_1 EQU $C84B
Xform_Rise_a EQU $F661
musica EQU $FF44
MOV_DRAW_VL_A EQU $F3B9
Moveto_d EQU $F312
PSG_update_done EQU $4279
GET_RISE_RUN EQU $F5EF
Init_OS_RAM EQU $F164
Select_Game EQU $F7A9
XFORM_RUN_A EQU $F65B
Draw_VLcs EQU $F3D6
PRINT_STR_D EQU $F37A
Init_OS EQU $F18B
NOAY EQU $4330
Dot_here EQU $F2C5
Print_List_chk EQU $F38C
Vec_High_Score EQU $CBEB
VEC_BUTTON_2_2 EQU $C817
Vec_Brightness EQU $C827
Vec_Run_Index EQU $C837
noay EQU $4330
DSWM_LOOP EQU $4120
SFX_ENDOFEFFECT EQU $43BB
Vec_NMI_Vector EQU $CBFB
DRAW_PAT_VL_D EQU $F439
VEC_EXPL_3 EQU $C85A
Reset0Ref_D0 EQU $F34A
VEC_MISC_COUNT EQU $C823
Set_Refresh EQU $F1A2
sfx_enablenoise EQU $43AC
_MUSIC1_MUSIC EQU $00E7
DO_SOUND_X EQU $F28C
Vec_Button_1_1 EQU $C812
Moveto_ix_FF EQU $F308
SFX_CHECKNOISEDISABLE EQU $4397
VEC_NMI_VECTOR EQU $CBFB
Joy_Analog EQU $F1F5
Vec_Expl_2 EQU $C859
Mov_Draw_VLc_a EQU $F3AD
Vec_Button_2_4 EQU $C819
PSG_MUSIC_ENDED EQU $426B
Init_VIA EQU $F14C
Vec_Max_Players EQU $C84F
Delay_0 EQU $F579
Add_Score_a EQU $F85E
Vec_0Ref_Enable EQU $C824
VEC_MUSIC_TWANG EQU $C858
_LOGO_PATH3 EQU $0075
VEC_SWI2_VECTOR EQU $CBF2
VEC_ANGLE EQU $C836
BITMASK_A EQU $F57E
RANDOM EQU $F517
VEC_BUTTON_1_4 EQU $C815
PRINT_LIST EQU $F38A
VEC_TWANG_TABLE EQU $C851
musicd EQU $FF8F
Vec_Num_Players EQU $C879
_LOGO_PATH6 EQU $00D5
VEC_EXPL_CHANS EQU $C854
DSWM_NO_NEGATE_Y EQU $40C6
RESET0INT EQU $F36B
Vec_Joy_Mux EQU $C81F
Delay_RTS EQU $F57D
SELECT_GAME EQU $F7A9
PSG_UPDATE_DONE EQU $4279
Clear_Score EQU $F84F
EXPLOSION_SND EQU $F92E
Recalibrate EQU $F2E6
Vec_Joy_2_Y EQU $C81E
Vec_Misc_Count EQU $C823
WAIT_RECAL EQU $F192
OBJ_WILL_HIT_U EQU $F8E5
Wait_Recal EQU $F192
Init_Music_chk EQU $F687
SOUND_BYTES_X EQU $F284
SOUND_BYTE EQU $F256
Sound_Byte_raw EQU $F25B
VEC_SWI_VECTOR EQU $CBFB
Xform_Rise EQU $F663
Rot_VL_Mode EQU $F62B
PSG_write_loop EQU $4234
CLEAR_X_B_A EQU $F552
PLAY_MUSIC_RUNTIME EQU $41F6
sfx_checknoisefreq EQU $435E
DRAW_VLP EQU $F410
SFX_CHECKTONEDISABLE EQU $4378
Vec_Text_HW EQU $C82A
READ_BTNS EQU $F1BA
VEC_NUM_GAME EQU $C87A
Add_Score_d EQU $F87C
Vec_Loop_Count EQU $C825
Vec_Expl_ChanA EQU $C853
VEC_RFRSH_HI EQU $C83E
Dot_List EQU $F2D5
Mov_Draw_VL EQU $F3BC
Clear_x_b EQU $F53F
VEC_DURATION EQU $C857
Vec_Joy_1_X EQU $C81B
DRAW_VLP_FF EQU $F404
MUSIC7 EQU $FEC6
DELAY_3 EQU $F56D
DOT_D EQU $F2C3
Mov_Draw_VL_b EQU $F3B1
Draw_Pat_VL_d EQU $F439
XFORM_RISE_A EQU $F661
Vec_Angle EQU $C836
AU_MUSIC_DONE EQU $42F6
_LOGO_PATH5 EQU $00AE
Vec_Button_1_4 EQU $C815
MUSIC9 EQU $FF26
Vec_Button_2_2 EQU $C817
OBJ_WILL_HIT EQU $F8F3
sfx_enabletone EQU $438D
VEC_RISERUN_TMP EQU $C834
AU_MUSIC_WRITE_LOOP EQU $42DF
DSWM_NEXT_SET_INTENSITY EQU $417D
Do_Sound EQU $F289
MUSIC3 EQU $FD81
MOVETO_D_7F EQU $F2FC
Clear_x_b_80 EQU $F550
PMR_START_NEW EQU $4204
VEC_BUTTON_2_3 EQU $C818
Warm_Start EQU $F06C
CLEAR_X_B EQU $F53F
Delay_b EQU $F57A
INTENSITY_7F EQU $F2A9
Check0Ref EQU $F34F
VEC_SWI3_VECTOR EQU $CBF2
DRAW_VLP_7F EQU $F408
RESET0REF_D0 EQU $F34A
_LOGO_PATH1 EQU $003C
COMPARE_SCORE EQU $F8C7
SFX_UPDATE EQU $4326
MOV_DRAW_VL_D EQU $F3BE
XFORM_RUN EQU $F65D
Dot_ix_b EQU $F2BE
Dec_Counters EQU $F563
VECTREX_PRINT_TEXT EQU $4071
New_High_Score EQU $F8D8
DRAW_VL_B EQU $F3D2
VEC_SEED_PTR EQU $C87B
Clear_Sound EQU $F272
Vec_Button_2_3 EQU $C818
Vec_Rfrsh_lo EQU $C83D
VEC_JOY_MUX_1_Y EQU $C820
Get_Run_Idx EQU $F5DB
Dec_3_Counters EQU $F55A
Vec_Rise_Index EQU $C839
Vec_Music_Ptr EQU $C853
STOP_MUSIC_RUNTIME EQU $427D
DSWM_W2 EQU $415C
RECALIBRATE EQU $F2E6
MOVE_MEM_A_1 EQU $F67F
Vec_Str_Ptr EQU $C82C
Draw_VLp_scale EQU $F40C
DELAY_RTS EQU $F57D
RISE_RUN_LEN EQU $F603
INTENSITY_5F EQU $F2A5
Vec_Joy_2_X EQU $C81D
Moveto_ix_7F EQU $F30C
MUSIC_BANK_TABLE EQU $4003
DRAW_PAT_VL_A EQU $F434
DSWM_NO_NEGATE_DY EQU $4138
MOV_DRAW_VL EQU $F3BC
Vec_RiseRun_Tmp EQU $C834
sfx_endofeffect EQU $43BB
DO_SOUND EQU $F289
MOD16 EQU $408C
music2 EQU $FD1D
Random_3 EQU $F511
Vec_Button_1_2 EQU $C813
_LOGO_VECTORS EQU $0000
Vec_Music_Wk_A EQU $C842
DRAW_VLC EQU $F3CE
VEC_EXPL_FLAG EQU $C867
DRAW_VLCS EQU $F3D6
DP_TO_C8 EQU $F1AF
INIT_MUSIC_CHK EQU $F687
MUSIC4 EQU $FDD3
Do_Sound_x EQU $F28C
_LOGO_PATH0 EQU $000F
Vec_Num_Game EQU $C87A
VEC_BTN_STATE EQU $C80F
GET_RUN_IDX EQU $F5DB
AU_MUSIC_PROCESS_WRITES EQU $42DD
AU_UPDATE_SFX EQU $4310
Vec_Counter_5 EQU $C832
sfx_disabletone EQU $4380
STRIP_ZEROS EQU $F8B7
Vec_Expl_3 EQU $C85A
Intensity_1F EQU $F29D
MUSIC6 EQU $FE76
music9 EQU $FF26
DSWM_NO_NEGATE_X EQU $40D3
Vec_Btn_State EQU $C80F
Intensity_a EQU $F2AB
Vec_Expl_4 EQU $C85B
CLEAR_C8_RAM EQU $F542
DRAW_VL_A EQU $F3DA
Vec_Counter_2 EQU $C82F
Mov_Draw_VLcs EQU $F3B5
VEC_TEXT_HEIGHT EQU $C82A
VEC_PATTERN EQU $C829
SFX_DISABLENOISE EQU $439F
JOY_ANALOG EQU $F1F5
sfx_checktonedisable EQU $4378
Vec_SWI2_Vector EQU $CBF2
MOVETO_X_7F EQU $F2F2
Clear_x_b_a EQU $F552
Vec_Music_Flag EQU $C856
MUSIC5 EQU $FE38
Moveto_x_7F EQU $F2F2
DSWM_W1 EQU $4117
DSWM_NEXT_NO_NEGATE_Y EQU $4189
Print_Ships EQU $F393
Obj_Hit EQU $F8FF
RESET0REF EQU $F354
Joy_Digital EQU $F1F8
MUSICB EQU $FF62
INTENSITY_1F EQU $F29D
VEC_COUNTERS EQU $C82E
CLEAR_X_B_80 EQU $F550
VEC_RFRSH EQU $C83D
ROT_VL_MODE EQU $F62B
MUSICD EQU $FF8F
Compare_Score EQU $F8C7
DRAW_VL_AB EQU $F3D8
Vec_Music_Freq EQU $C861
OBJ_HIT EQU $F8FF
DP_to_C8 EQU $F1AF
Clear_x_256 EQU $F545
Xform_Run_a EQU $F65B
VEC_MUSIC_FREQ EQU $C861
VEC_MUSIC_CHAN EQU $C855
Read_Btns EQU $F1BA
VEC_JOY_2_X EQU $C81D
MOD16.MOD16_END EQU $40A6
VEC_BUTTON_2_4 EQU $C819
Init_Music_Buf EQU $F533
DP_TO_D0 EQU $F1AA
Draw_VL_b EQU $F3D2
DOT_IX_B EQU $F2BE
Vec_Joy_Mux_1_Y EQU $C820
Vec_Expl_Flag EQU $C867
DOT_HERE EQU $F2C5
Print_Str_hwyx EQU $F373
VEC_MUSIC_FLAG EQU $C856
AU_MUSIC_NO_DELAY EQU $42C4
Vec_RiseRun_Len EQU $C83B
Vec_Freq_Table EQU $C84D
Draw_VL_mode EQU $F46E
music8 EQU $FEF8
NEW_HIGH_SCORE EQU $F8D8
DOT_LIST_RESET EQU $F2DE
Vec_Seed_Ptr EQU $C87B
Init_Music_x EQU $F692
AU_MUSIC_READ EQU $42AF
DRAW_PAT_VL EQU $F437
VEC_ADSR_TIMERS EQU $C85E
Rise_Run_X EQU $F5FF
PMR_DONE EQU $4212
PSG_music_ended EQU $426B
VEC_JOY_1_X EQU $C81B
DRAW_LINE_D EQU $F3DF
Vec_Button_1_3 EQU $C814
DSWM_W3 EQU $41E6
MOV_DRAW_VLC_A EQU $F3AD
Strip_Zeros EQU $F8B7
VEC_COUNTER_6 EQU $C833
Sound_Bytes_x EQU $F284
Delay_3 EQU $F56D
Delay_2 EQU $F571
INIT_OS_RAM EQU $F164
PRINT_STR EQU $F495
Vec_Rfrsh_hi EQU $C83E
PRINT_TEXT_STR_3327403 EQU $43D1
MOVETO_D EQU $F312
VEC_COUNTER_3 EQU $C830
DSWM_DONE EQU $41F5
Dot_List_Reset EQU $F2DE
VEC_COUNTER_4 EQU $C831
Read_Btns_Mask EQU $F1B4
VEC_BUTTON_1_2 EQU $C813
PRINT_STR_YX EQU $F378
DSWM_NO_NEGATE_DX EQU $4142
VEC_JOY_2_Y EQU $C81E
MUSIC1 EQU $FD0D


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
; Bank 0 ($0000) is active; fixed bank 31 ($4000-$7FFF) always visible
    JMP MAIN

;***************************************************************************
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
VAR_TITLE_INTENSITY  EQU $C880+$3B   ; User variable: TITLE_INTENSITY (2 bytes)
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
    ; PLAY_MUSIC("music1") - play music asset (index=0)
    LDX #0        ; Music asset index for lookup
    JSR PLAY_MUSIC_BANKED  ; Play with automatic bank switching
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
    ; DRAW_VECTOR: Draw vector asset at position
    ; Asset: logo (index=0, 7 paths)
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD #0
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
    LDX #0        ; Asset index for lookup
    JSR DRAW_VECTOR_BANKED  ; Draw with automatic bank switching
    LDD #0
    STD RESULT
    ; PRINT_TEXT: Print text at position
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_2223292      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    JSR AUDIO_UPDATE  ; Auto-injected: update music + SFX (after all game logic)
    RTS


; ================================================
