    INCLUDE "VECTREX.I"
; External symbols (helpers and shared data)
Print_Str_d EQU $F373
DRAW_CIRCLE_RUNTIME EQU $4999
CE_12 EQU $024F
DCR_dy_half EQU $4A1F
DSWM_W2 EQU $48FF
JSR $F1BA  ; Read_Btns EQU $0084
IF_END_13 EQU $0293
DSL_LOOP EQU $479F
DLW_SEG2_DX_NO_REMAIN EQU $4713
J1X_BUILTIN EQU $45B2
DCR_after_intensity EQU $49CD
DSWM_NEXT_USE_OVERRIDE EQU $491E
__Moveto_d EQU $4742
JSR $F1AA  ; DP_to_D0 EQU $0082
Moveto_d_7F EQU $F1DF
J1Y_BUILTIN EQU $45C7
CE_16 EQU $0281
UPDATE_ENEMIES EQU $0139
IF_END_1 EQU $0191
Draw_Sync_List_At_With_Mirrors_BANK_WRAPPER EQU $400B
DLW_SEG2_DX_DONE EQU $4716
DP_to_C8 EQU $F1AF
JSR $F1AF  ; DP_to_C8 EQU $0086
DSL_NEXT_PATH EQU $47DD
START EQU $0022
VECTREX_SET_INTENSITY EQU $472A
DCR_dx_done EQU $4A7A
DSL_DONE EQU $484D
DLW_NEED_SEG2 EQU $46DD
DSWM_NO_NEGATE_DY EQU $48DB
DP_to_D0 EQU $F1AA
Draw_VLc EQU $F1CF
DSWM_NO_NEGATE_X EQU $4876
J1B1_BUILTIN EQU $45DC
VECTREX_DEBUG_PRINT EQU $4639
DRAW_ALL EQU $0295
DCR_dy_neg_half EQU $4A27
J1B2_BUILTIN EQU $45ED
Moveto_d EQU $F1EB
DLW_SEG1_DX_READY EQU $46B8
DCR_dx_half EQU $4A63
DRAW_LINE_WRAPPER_BANK_WRAPPER EQU $4062
DSL_W2 EQU $47CE
__Intensity_a EQU $473A
JSR Wait_Recal  ; CRITICAL EQU $0080
IF_NEXT_6 EQU $01E7
DRAW_PLAYER EQU $029D
CE_4 EQU $017F
DLW_DONE EQU $4725
UPDATE_PLAYER EQU $0129
DSWM_SET_INTENSITY EQU $485C
J1B3_BUILTIN EQU $45FE
DCR_dy_pos EQU $4A33
DSWM_NEXT_NO_NEGATE_Y EQU $492C
_ENEMY_VECTORS EQU $459D
DRAW_ENEMIES EQU $02CB
CLR $C823    ; CRITICAL EQU $0062
DSWM_NEXT_PATH EQU $490E
DSWM_NEXT_NO_NEGATE_X EQU $4939
LOOP_BODY EQU $007E
VECTREX_SET_INTENSITY_BANK_WRAPPER EQU $4045
__Reset0Ref EQU $473F
Wait_Recal EQU $F1A4
IF_NEXT_14 EQU $0293
DLW_SEG1_DX_NO_CLAMP EQU $46B5
J1B4_BUILTIN.J1B4_OFF EQU $461C
CE_8 EQU $01D5
DSWM_NO_NEGATE_Y EQU $4869
DSWM_NO_NEGATE_DX EQU $48E5
DSL_W1 EQU $4796
Intensity_a EQU $F2AB
LDA #$01     ; CRITICAL EQU $0064
IF_NEXT_10 EQU $0261
DLW_SEG2_DY_DONE EQU $46F1
_ENEMY_PATH0 EQU $45A0
DCR_LOOP EQU $49E8
DLW_SEG1_DY_LO EQU $4685
DSWM_LOOP EQU $48C3
CT_3 EQU $017B
IF_NEXT_2 EQU $0191
DEBUG_LABEL_A EQU $00D4
Draw_Sync_List EQU $474C
IF_END_9 EQU $0261
J1B4_BUILTIN EQU $460F
DCR_intensity_5F EQU $49CA
VECTREX_PRINT_TEXT EQU $4620
MAIN EQU $0060
Draw_VL EQU $F1D5
CUSTOM_RESET EQU $4000
DSWM_W1 EQU $48BA
CT_11 EQU $024B
DLW_SEG2_DY_POS EQU $46EE
RESET0REF EQU $F192
DRAW_LINE_WRAPPER EQU $464B
J1B2_BUILTIN.J1B2_OFF EQU $45FA
CHECK_INPUT EQU $0131
J1B1_BUILTIN.J1B1_OFF EQU $45E9
__Draw_Line_d EQU $4747
Draw_Line_d EQU $F1F5
DCR_dy_zero EQU $4A19
DSWM_NEXT_SET_INTENSITY EQU $4920
CT_15 EQU $027D
DLW_SEG1_DX_LO EQU $46A8
DLW_SEG1_DY_NO_CLAMP EQU $4692
CT_7 EQU $01D1
DCR_dx_pos EQU $4A77
MOVE_PLAYER EQU $0135
DSL_W3 EQU $483E
_PLAYER_VECTORS EQU $458B
DSWM_W3 EQU $4989
DCR_dx_neg_half EQU $4A6B
DCR_dx_zero EQU $4A5D
DLW_SEG2_DX_CHECK_NEG EQU $4705
DCR_DELTA_TABLE EQU $4A91
DRAW_CIRCLE_RUNTIME_BANK_WRAPPER EQU $407F
INTENSITY_A EQU $F2AB
DSWM_DONE EQU $4998
_PLAYER_PATH0 EQU $458E
IF_END_5 EQU $01E7
DLW_SEG1_DY_READY EQU $4695
J1B3_BUILTIN.J1B3_OFF EQU $460B
VECTREX_PRINT_TEXT_BANK_WRAPPER EQU $4028
DCR_dy_done EQU $4A36
Reset0Ref EQU $F192
DEBUG_SKIP_DATA_0 EQU $00D7
Read_Btns EQU $F1BA
DSWM_USE_OVERRIDE EQU $485A
Draw_Sync_List_At_With_Mirrors EQU $484E


; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000

