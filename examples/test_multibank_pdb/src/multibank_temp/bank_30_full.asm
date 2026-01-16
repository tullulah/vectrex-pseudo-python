    INCLUDE "VECTREX.I"
; External symbols (helpers and shared data)
LDA #$01     ; CRITICAL EQU $0047
Moveto_d EQU $F1EB
J1B1_BUILTIN EQU $402A
DLW_DONE EQU $4161
J1X_BUILTIN EQU $4000
J1Y_BUILTIN EQU $4015
STR_0 EQU $4188
DRAW_LINE_WRAPPER EQU $4087
DLW_SEG1_DX_LO EQU $40E4
JSR $F1BA  ; Read_Btns EQU $0065
DP_to_C8 EQU $F1AF
Wait_Recal EQU $F1A4
JSR Wait_Recal  ; CRITICAL EQU $0061
__Moveto_d EQU $417E
DLW_SEG1_DY_LO EQU $40C1
DLW_SEG1_DY_NO_CLAMP EQU $40CE
LOOP_BODY EQU $0061
J1B3_BUILTIN EQU $404C
DLW_SEG2_DX_NO_REMAIN EQU $414F
DLW_SEG1_DX_NO_CLAMP EQU $40F1
STR_1 EQU $418E
Print_Str_d EQU $F373
VECTREX_SET_INTENSITY EQU $4166
MAIN EQU $0043
INTENSITY_A EQU $F2AB
Intensity_a EQU $F2AB
DLW_SEG2_DX_DONE EQU $4152
Draw_VLc EQU $F1CF
__Draw_Line_d EQU $4183
DLW_SEG1_DY_READY EQU $40D1
JSR $F1AA  ; DP_to_D0 EQU $0063
Reset0Ref EQU $F192
DP_to_D0 EQU $F1AA
J1B4_BUILTIN EQU $405D
DLW_NEED_SEG2 EQU $4119
DLW_SEG2_DY_POS EQU $412A
J1B2_BUILTIN.J1B2_OFF EQU $4048
Read_Btns EQU $F1BA
START EQU $0022
VECTREX_PRINT_TEXT EQU $406E
__Reset0Ref EQU $417B
DLW_SEG1_DX_READY EQU $40F4
Draw_VL EQU $F1D5
Moveto_d_7F EQU $F1DF
RESET0REF EQU $F192
Draw_Sync_List_At_With_Mirrors EQU $F0E8
J1B3_BUILTIN.J1B3_OFF EQU $4059
Draw_Line_d EQU $F1F5
__Intensity_a EQU $4176
J1B2_BUILTIN EQU $403B
J1B4_BUILTIN.J1B4_OFF EQU $406A
JSR $F1AF  ; DP_to_C8 EQU $0067
DLW_SEG2_DX_CHECK_NEG EQU $4141
DLW_SEG2_DY_DONE EQU $412D
J1B1_BUILTIN.J1B1_OFF EQU $4037
CLR $C823    ; CRITICAL EQU $0045


; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 21 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPPTR               EQU $C880+$03   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$05   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$07   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$09   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$0A   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$0B   ; String buffer for PRINT_NUMBER (2 bytes)
VAR_ARG0             EQU $C880+$0D   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$0F   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$11   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$13   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000

