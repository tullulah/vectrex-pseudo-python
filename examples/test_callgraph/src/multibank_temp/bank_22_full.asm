    INCLUDE "VECTREX.I"
; External symbols (helpers and shared data)
Read_Btns EQU $F1BA
_ENEMY_PATH0 EQU $4407
IF_END_1 EQU $018D
DSWM_NEXT_NO_NEGATE_Y EQU $4385
DLW_DONE EQU $417E
DLW_SEG1_DY_READY EQU $40EE
_PLAYER_VECTORS EQU $43F2
IF_NEXT_2 EQU $018D
Moveto_d_7F EQU $F1DF
DSWM_NEXT_SET_INTENSITY EQU $4379
VECTREX_SET_INTENSITY EQU $4183
DSL_W1 EQU $41EF
UPDATE_ENEMIES EQU $0135
DRAW_ENEMIES EQU $02C7
DP_to_D0 EQU $F1AA
IF_END_5 EQU $01E3
DSWM_NO_NEGATE_DX EQU $433E
DLW_SEG2_DX_CHECK_NEG EQU $415E
RESET0REF EQU $F192
IF_NEXT_6 EQU $01E3
DSWM_LOOP EQU $431C
DSWM_USE_OVERRIDE EQU $42B3
DLW_SEG2_DY_POS EQU $4147
DSWM_NO_NEGATE_Y EQU $42C2
VECTREX_PRINT_TEXT EQU $4079
MOVE_PLAYER EQU $0131
DSWM_DONE EQU $43F1
CT_15 EQU $0279
INTENSITY_A EQU $F2AB
CLR $C823    ; CRITICAL EQU $005E
DLW_SEG1_DY_LO EQU $40DE
Intensity_a EQU $F2AB
__Reset0Ref EQU $4198
LDA #$01     ; CRITICAL EQU $0060
DLW_SEG1_DX_LO EQU $4101
Wait_Recal EQU $F1A4
DLW_SEG2_DY_DONE EQU $414A
CE_12 EQU $024B
DRAW_ALL EQU $0291
DSWM_NEXT_NO_NEGATE_X EQU $4392
Print_Str_d EQU $F373
UPDATE_PLAYER EQU $0125
START EQU $0022
IF_NEXT_10 EQU $025D
Draw_Sync_List_At_With_Mirrors EQU $42A7
JSR $F1AA  ; DP_to_D0 EQU $007E
DSL_NEXT_PATH EQU $4236
DEBUG_LABEL_A EQU $00D0
DSWM_W3 EQU $43E2
LOOP_BODY EQU $007A
DSWM_W2 EQU $4358
Draw_Line_d EQU $F1F5
DSL_LOOP EQU $41F8
CE_4 EQU $017B
DEBUG_SKIP_DATA_0 EQU $00D3
__Intensity_a EQU $4193
CHECK_INPUT EQU $012D
__Draw_Line_d EQU $41A0
IF_NEXT_14 EQU $028F
MAIN EQU $005C
J1B3_BUILTIN.J1B3_OFF EQU $4064
J1B1_BUILTIN.J1B1_OFF EQU $4042
DSWM_NO_NEGATE_DY EQU $4334
CT_3 EQU $0177
DSWM_SET_INTENSITY EQU $42B5
Reset0Ref EQU $F192
DLW_SEG1_DY_NO_CLAMP EQU $40EB
J1B2_BUILTIN.J1B2_OFF EQU $4053
DLW_NEED_SEG2 EQU $4136
Draw_Sync_List EQU $41A5
J1B4_BUILTIN.J1B4_OFF EQU $4075
_ENEMY_VECTORS EQU $4404
DSWM_W1 EQU $4313
CE_16 EQU $027D
J1X_BUILTIN EQU $400B
J1B1_BUILTIN EQU $4035
Moveto_d EQU $F1EB
JSR $F1AF  ; DP_to_C8 EQU $0082
CT_11 EQU $0247
_PLAYER_PATH0 EQU $43F5
DSWM_NO_NEGATE_X EQU $42CF
IF_END_9 EQU $025D
DSL_DONE EQU $42A6
CE_8 EQU $01D1
J1B2_BUILTIN EQU $4046
DSWM_NEXT_USE_OVERRIDE EQU $4377
DLW_SEG1_DX_READY EQU $4111
CT_7 EQU $01CD
DRAW_PLAYER EQU $0299
DLW_SEG2_DX_NO_REMAIN EQU $416C
J1Y_BUILTIN EQU $4020
DSL_W3 EQU $4297
J1B3_BUILTIN EQU $4057
DSL_W2 EQU $4227
JSR Wait_Recal  ; CRITICAL EQU $007C
IF_END_13 EQU $028F
DSWM_NEXT_PATH EQU $4367
CUSTOM_RESET EQU $4000
DLW_SEG2_DX_DONE EQU $416F
DLW_SEG1_DX_NO_CLAMP EQU $410E
__Moveto_d EQU $419B
DRAW_LINE_WRAPPER EQU $40A4
Draw_VLc EQU $F1CF
J1B4_BUILTIN EQU $4068
JSR $F1BA  ; Read_Btns EQU $0080
Draw_VL EQU $F1D5
DP_to_C8 EQU $F1AF
VECTREX_DEBUG_PRINT EQU $4092


; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 48 bytes
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
VAR_ENEMY1_X         EQU $C880+$1A   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$1C   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$1E   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$20   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$22   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$24   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$26   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$28   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$2A   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$2C   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$2E   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================
