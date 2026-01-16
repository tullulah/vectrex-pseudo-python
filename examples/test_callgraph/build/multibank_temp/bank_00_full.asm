ORG $0000
    FCC "g GCE 1982"
    FCB $80
    FDB music1
    FCB $F8
    FCB $50
    FCB $20
    FCB $BB
    FCC "CALL GRAPH TEST"
    FCB $80
    FCB 0
; CODE SECTION
;***************************************************************************


    INCLUDE "VECTREX.I"
; External symbols (helpers and shared data)
LDA #$01     ; CRITICAL EQU $0060
DLW_NEED_SEG2 EQU $412B
Intensity_a EQU $F2AB
J1B4_BUILTIN.J1B4_OFF EQU $406A
DLW_SEG2_DX_CHECK_NEG EQU $4153
__Draw_Line_d EQU $4195
DLW_SEG1_DX_NO_CLAMP EQU $4103
JSR $F1AA  ; DP_to_D0 EQU $007E
Moveto_d_7F EQU $F1DF
DLW_SEG1_DY_LO EQU $40D3
_PLAYER_VECTORS EQU $43E7
DSL_NEXT_PATH EQU $422B
Reset0Ref EQU $F192
MOVE_PLAYER EQU $0131
IF_NEXT_2 EQU $018D
CE_8 EQU $01D1
Moveto_d EQU $F1EB
UPDATE_PLAYER EQU $0125
CT_7 EQU $01CD
_ENEMY_VECTORS EQU $43F9
J1Y_BUILTIN EQU $4015
J1B3_BUILTIN.J1B3_OFF EQU $4059
DSWM_W3 EQU $43D7
DLW_DONE EQU $4173
DEBUG_LABEL_A EQU $00D0
IF_END_1 EQU $018D
J1X_BUILTIN EQU $4000
DSWM_NEXT_SET_INTENSITY EQU $436E
DP_to_C8 EQU $F1AF
RESET0REF EQU $F192
DSWM_LOOP EQU $4311
J1B4_BUILTIN EQU $405D
DRAW_LINE_WRAPPER EQU $4099
CT_15 EQU $0279
IF_END_5 EQU $01E3
Wait_Recal EQU $F1A4
Draw_VLc EQU $F1CF
DLW_SEG1_DY_READY EQU $40E3
CLR $C823    ; CRITICAL EQU $005E
IF_NEXT_10 EQU $025D
J1B1_BUILTIN EQU $402A
DLW_SEG2_DX_NO_REMAIN EQU $4161
DP_to_D0 EQU $F1AA
DSWM_W2 EQU $434D
DSWM_USE_OVERRIDE EQU $42A8
CE_4 EQU $017B
DSWM_NO_NEGATE_Y EQU $42B7
LOOP_BODY EQU $007A
DLW_SEG1_DY_NO_CLAMP EQU $40E0
IF_END_9 EQU $025D
INTENSITY_A EQU $F2AB
DSL_DONE EQU $429B
DSL_LOOP EQU $41ED
DSWM_SET_INTENSITY EQU $42AA
JSR Wait_Recal  ; CRITICAL EQU $007C
__Reset0Ref EQU $418D
DEBUG_SKIP_DATA_0 EQU $00D3
JSR $F1BA  ; Read_Btns EQU $0080
DSWM_NEXT_USE_OVERRIDE EQU $436C
J1B1_BUILTIN.J1B1_OFF EQU $4037
Draw_Line_d EQU $F1F5
IF_END_13 EQU $028F
_ENEMY_PATH0 EQU $43FC
DSL_W3 EQU $428C
CHECK_INPUT EQU $012D
CT_3 EQU $0177
DSWM_NEXT_NO_NEGATE_Y EQU $437A
DSWM_NEXT_PATH EQU $435C
DSWM_NEXT_NO_NEGATE_X EQU $4387
Draw_Sync_List_At_With_Mirrors EQU $429C
CE_16 EQU $027D
Print_Str_d EQU $F373
J1B2_BUILTIN EQU $403B
IF_NEXT_14 EQU $028F
DRAW_PLAYER EQU $0299
DSL_W2 EQU $421C
CE_12 EQU $024B
DSL_W1 EQU $41E4
VECTREX_DEBUG_PRINT EQU $4087
__Intensity_a EQU $4188
J1B2_BUILTIN.J1B2_OFF EQU $4048
DLW_SEG2_DX_DONE EQU $4164
START EQU $0022
Draw_VL EQU $F1D5
DSWM_NO_NEGATE_X EQU $42C4
DLW_SEG1_DX_READY EQU $4106
IF_NEXT_6 EQU $01E3
Draw_Sync_List EQU $419A
DLW_SEG2_DY_POS EQU $413C
JSR $F1AF  ; DP_to_C8 EQU $0082
DRAW_ENEMIES EQU $02C7
_PLAYER_PATH0 EQU $43EA
DSWM_NO_NEGATE_DY EQU $4329
DLW_SEG2_DY_DONE EQU $413F
VECTREX_SET_INTENSITY EQU $4178
Read_Btns EQU $F1BA
MAIN EQU $005C
UPDATE_ENEMIES EQU $0135
__Moveto_d EQU $4190
DSWM_NO_NEGATE_DX EQU $4333
DRAW_ALL EQU $0291
J1B3_BUILTIN EQU $404C
DSWM_W1 EQU $4308
VECTREX_PRINT_TEXT EQU $406E
DSWM_DONE EQU $43E6
CT_11 EQU $0247
DLW_SEG1_DX_LO EQU $40F6


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



;**** CONST DECLARATIONS (NUMBER-ONLY) ****

;
; ┌─────────────────────────────────────────────────────────────────┐
; │ PROGRAM CODE SECTION - User VPy Code                            │
; │ This section contains the compiled user program logic.          │
; └─────────────────────────────────────────────────────────────────┘
;

START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
    CLR $C80E        ; Initialize Vec_Prev_Btns to 0 for Read_Btns debounce
    LDA #$80
    STA VIA_t1_cnt_lo
    LDS #$CBFF       ; Initialize stack at top of RAM (safer than Vec_Default_Stk)

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:19
    ; VPy_LINE:10
    LDD #-50
    STD VAR_ENEMY1_X
    ; VPy_LINE:11
    LDD #60
    STD VAR_ENEMY1_Y
    ; VPy_LINE:12
    LDD #0
    STD VAR_ENEMY2_X
    ; VPy_LINE:13
    LDD #0
    STD VAR_ENEMY2_Y
    ; VPy_LINE:14
    LDD #50
    STD VAR_ENEMY3_X
    ; VPy_LINE:15
    LDD #-60
    STD VAR_ENEMY3_Y
    ; VPy_LINE:16
    LDD #0
    STD VAR_FRAME_COUNT
    ; VPy_LINE:20
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 20
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:21
    ; pass (no-op)

MAIN:
    JSR $F1AF    ; DP_to_C8 (required for RAM access)
    ; === Initialize Joystick (one-time setup) ===
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

    ; JSR Wait_Recal is now called at start of LOOP_BODY (see auto-inject)
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    LBRA MAIN


; ================================================
; ================================================
    ; VPy_LINE:25
LOOP_BODY:
    LEAS -2,S ; allocate locals
    JSR Wait_Recal  ; CRITICAL: Sync with CRT refresh (50Hz frame timing)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; VPy_LINE:26
    LDD #12
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:27
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:29
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 29
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:31
    LDD 0 ,S
    STD RESULT
; NATIVE_CALL: DEBUG_PRINT(a) at line 31
    LDD RESULT
    STA $C002
    STB $C000
    LDA #$FE
    STA $C001
    LDX #DEBUG_LABEL_A
    STX $C004
    LBRA DEBUG_SKIP_DATA_0
DEBUG_LABEL_A:
    FCC "a"
    FCB $00
DEBUG_SKIP_DATA_0:
    LDD #0
    STD RESULT
    ; VPy_LINE:33
    JSR UPDATE_PLAYER
    ; VPy_LINE:34
    JSR UPDATE_ENEMIES
    ; VPy_LINE:35
    JSR DRAW_ALL
    ; VPy_LINE:36
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:37
    LDD VAR_FRAME_COUNT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_FRAME_COUNT
    STU TMPPTR
    STX ,U
    LEAS 2,S ; free locals
    RTS

    ; VPy_LINE:41
UPDATE_PLAYER: ; function
; --- function update_player ---
    ; VPy_LINE:43
    JSR CHECK_INPUT
    ; VPy_LINE:44
    JSR MOVE_PLAYER
    RTS

    ; VPy_LINE:46
CHECK_INPUT: ; function
; --- function check_input ---
    ; VPy_LINE:48
    ; pass (no-op)
    RTS

    ; VPy_LINE:50
MOVE_PLAYER: ; function
; --- function move_player ---
    ; VPy_LINE:52
    ; pass (no-op)
    RTS

    ; VPy_LINE:54
UPDATE_ENEMIES: ; function
; --- function update_enemies ---
    ; VPy_LINE:56
    LDD VAR_ENEMY1_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY1_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:57
    LDD VAR_ENEMY1_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    LBGT CT_3
    LDD #0
    STD RESULT
    LBRA CE_4
CT_3:
    LDD #1
    STD RESULT
CE_4:
    LDD RESULT
    LBEQ IF_NEXT_2
    ; VPy_LINE:58
    LDD #-100
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY1_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_1
IF_NEXT_2:
IF_END_1:
    ; VPy_LINE:61
    LDD VAR_ENEMY2_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY2_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:62
    LDD VAR_ENEMY2_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    LBGT CT_7
    LDD #0
    STD RESULT
    LBRA CE_8
CT_7:
    LDD #1
    STD RESULT
CE_8:
    LDD RESULT
    LBEQ IF_NEXT_6
    ; VPy_LINE:63
    LDD #-100
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY2_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_5
IF_NEXT_6:
IF_END_5:
    ; VPy_LINE:66
    LDD VAR_ENEMY3_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY3_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:67
    LDD VAR_ENEMY3_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY3_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:68
    LDD VAR_ENEMY3_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    LBLT CT_11
    LDD #0
    STD RESULT
    LBRA CE_12
CT_11:
    LDD #1
    STD RESULT
CE_12:
    LDD RESULT
    LBEQ IF_NEXT_10
    ; VPy_LINE:69
    LDD #100
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY3_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_9
IF_NEXT_10:
IF_END_9:
    ; VPy_LINE:70
    LDD VAR_ENEMY3_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    LBLT CT_15
    LDD #0
    STD RESULT
    LBRA CE_16
CT_15:
    LDD #1
    STD RESULT
CE_16:
    LDD RESULT
    LBEQ IF_NEXT_14
    ; VPy_LINE:71
    LDD #100
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY3_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_13
IF_NEXT_14:
IF_END_13:
    RTS

    ; VPy_LINE:75
DRAW_ALL: ; function
; --- function draw_all ---
    ; VPy_LINE:77
    JSR DRAW_PLAYER
    ; VPy_LINE:78
    JSR DRAW_ENEMIES
    RTS

    ; VPy_LINE:80
DRAW_PLAYER: ; function
; --- function draw_player ---
    ; VPy_LINE:82
; DRAW_VECTOR("player", x, y) - 1 path(s) at position
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
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_PLAYER_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors  ; Bank #31 (fixed) - no wrapper needed
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    RTS

    ; VPy_LINE:84
DRAW_ENEMIES: ; function
; --- function draw_enemies ---
    ; VPy_LINE:86
; DRAW_VECTOR("enemy", x, y) - 1 path(s) at position
    LDD VAR_ENEMY1_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD VAR_ENEMY1_Y
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
    LDX #_ENEMY_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors  ; Bank #31 (fixed) - no wrapper needed
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    ; VPy_LINE:87
; DRAW_VECTOR("enemy", x, y) - 1 path(s) at position
    LDD VAR_ENEMY2_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD VAR_ENEMY2_Y
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
    LDX #_ENEMY_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors  ; Bank #31 (fixed) - no wrapper needed
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    ; VPy_LINE:88
; DRAW_VECTOR("enemy", x, y) - 1 path(s) at position
    LDD VAR_ENEMY3_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD VAR_ENEMY3_Y
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
    LDX #_ENEMY_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors  ; Bank #31 (fixed) - no wrapper needed
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    RTS


; ================================================
