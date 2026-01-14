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
Moveto_d EQU $F1EB
CT_11 EQU $024B
Draw_Sync_List_At_With_Mirrors EQU $484E
DCR_DELTA_TABLE EQU $4A91
DRAW_CIRCLE_RUNTIME_BANK_WRAPPER EQU $4074
IF_END_9 EQU $0261
MAIN EQU $0060
DSWM_LOOP EQU $48C3
JSR $F1AF  ; DP_to_C8 EQU $0086
DEBUG_LABEL_A EQU $00D4
DLW_NEED_SEG2 EQU $46DD
DCR_dy_half EQU $4A1F
LDA #$01     ; CRITICAL EQU $0064
DSL_LOOP EQU $479F
IF_NEXT_2 EQU $0191
JSR $F1BA  ; Read_Btns EQU $0084
DCR_dy_neg_half EQU $4A27
DSWM_W1 EQU $48BA
DLW_SEG1_DY_READY EQU $4695
DRAW_LINE_WRAPPER_BANK_WRAPPER EQU $4057
DSWM_NEXT_NO_NEGATE_X EQU $4939
Wait_Recal EQU $F1A4
IF_END_5 EQU $01E7
IF_END_1 EQU $0191
DSL_W3 EQU $483E
DLW_SEG2_DX_CHECK_NEG EQU $4705
J1B3_BUILTIN.J1B3_OFF EQU $460B
DLW_SEG2_DY_POS EQU $46EE
J1B1_BUILTIN.J1B1_OFF EQU $45E9
CE_16 EQU $0281
DSWM_NEXT_PATH EQU $490E
DCR_dy_done EQU $4A36
VECTREX_PRINT_TEXT EQU $4620
IF_NEXT_6 EQU $01E7
DEBUG_SKIP_DATA_0 EQU $00D7
_PLAYER_PATH0 EQU $4583
Draw_VL EQU $F1D5
IF_NEXT_14 EQU $0293
LOOP_BODY EQU $007E
DCR_dx_done EQU $4A7A
_PLAYER_VECTORS EQU $4580
DSWM_NEXT_NO_NEGATE_Y EQU $492C
Intensity_a EQU $F2AB
J1B4_BUILTIN EQU $460F
__Moveto_d EQU $4742
CT_15 EQU $027D
_ENEMY_PATH0 EQU $4595
DCR_dx_half EQU $4A63
VECTREX_SET_INTENSITY EQU $472A
DSL_NEXT_PATH EQU $47DD
DP_to_C8 EQU $F1AF
DSWM_NEXT_USE_OVERRIDE EQU $491E
DLW_SEG1_DX_READY EQU $46B8
J1Y_BUILTIN EQU $45C7
MOVE_PLAYER EQU $0135
J1B2_BUILTIN.J1B2_OFF EQU $45FA
DLW_SEG2_DX_DONE EQU $4716
J1B1_BUILTIN EQU $45DC
DCR_LOOP EQU $49E8
CT_3 EQU $017B
Reset0Ref EQU $F192
Draw_VLc EQU $F1CF
DSWM_NO_NEGATE_Y EQU $4869
DRAW_LINE_WRAPPER EQU $464B
DRAW_PLAYER EQU $029D
DRAW_ENEMIES EQU $02CB
JSR Wait_Recal  ; CRITICAL EQU $0080
VECTREX_PRINT_TEXT_BANK_WRAPPER EQU $401D
Draw_Line_d EQU $F1F5
_ENEMY_VECTORS EQU $4592
Print_Str_d EQU $F373
DLW_DONE EQU $4725
UPDATE_ENEMIES EQU $0139
INTENSITY_A EQU $F2AB
DRAW_CIRCLE_RUNTIME EQU $4999
Read_Btns EQU $F1BA
CE_8 EQU $01D5
DSWM_DONE EQU $4998
DCR_intensity_5F EQU $49CA
DSL_W1 EQU $4796
IF_END_13 EQU $0293
DLW_SEG1_DY_NO_CLAMP EQU $4692
__Draw_Line_d EQU $4747
DSL_DONE EQU $484D
DLW_SEG1_DY_LO EQU $4685
DLW_SEG2_DX_NO_REMAIN EQU $4713
J1B2_BUILTIN EQU $45ED
DSWM_NO_NEGATE_DX EQU $48E5
__Intensity_a EQU $473A
DCR_dx_zero EQU $4A5D
J1B3_BUILTIN EQU $45FE
DLW_SEG2_DY_DONE EQU $46F1
Draw_Sync_List_At_With_Mirrors_BANK_WRAPPER EQU $4000
J1X_BUILTIN EQU $45B2
CHECK_INPUT EQU $0131
CE_4 EQU $017F
START EQU $0022
DCR_dy_pos EQU $4A33
DP_to_D0 EQU $F1AA
CT_7 EQU $01D1
DSL_W2 EQU $47CE
UPDATE_PLAYER EQU $0129
DLW_SEG1_DX_NO_CLAMP EQU $46B5
DSWM_SET_INTENSITY EQU $485C
Draw_Sync_List EQU $474C
DRAW_ALL EQU $0295
CE_12 EQU $024F
VECTREX_DEBUG_PRINT EQU $4639
J1B4_BUILTIN.J1B4_OFF EQU $461C
DSWM_NO_NEGATE_DY EQU $48DB
DSWM_NO_NEGATE_X EQU $4876
DCR_dy_zero EQU $4A19
CLR $C823    ; CRITICAL EQU $0062
DLW_SEG1_DX_LO EQU $46A8
__Reset0Ref EQU $473F
DSWM_W2 EQU $48FF
DCR_after_intensity EQU $49CD
DSWM_W3 EQU $4989
CUSTOM_RESET EQU $45A7
RESET0REF EQU $F192
DCR_dx_neg_half EQU $4A6B
DSWM_USE_OVERRIDE EQU $485A
Moveto_d_7F EQU $F1DF
DCR_dx_pos EQU $4A77
VECTREX_SET_INTENSITY_BANK_WRAPPER EQU $403A
DSWM_NEXT_SET_INTENSITY EQU $4920
JSR $F1AA  ; DP_to_D0 EQU $0082
IF_NEXT_10 EQU $0261


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
    LDA #0
    STA >CURRENT_ROM_BANK ; Initialize to bank 0 (RAM tracker for debugging)
    ; Note: NOT writing to hardware bank register - already in Bank #0

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
    ; VPy_LINE:24
LOOP_BODY:
    LEAS -2,S ; allocate locals
    JSR Wait_Recal  ; CRITICAL: Sync with CRT refresh (50Hz frame timing)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; VPy_LINE:25
    LDD #12
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:26
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
    ; VPy_LINE:28
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 28
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:30
    LDD 0 ,S
    STD RESULT
; NATIVE_CALL: DEBUG_PRINT(a) at line 30
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
    ; VPy_LINE:32
    JSR UPDATE_PLAYER
    ; VPy_LINE:33
    JSR UPDATE_ENEMIES
    ; VPy_LINE:34
    JSR DRAW_ALL
    ; VPy_LINE:35
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
    ; VPy_LINE:36
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

    ; VPy_LINE:40
UPDATE_PLAYER: ; function
; --- function update_player ---
    ; VPy_LINE:42
    JSR CHECK_INPUT
    ; VPy_LINE:43
    JSR MOVE_PLAYER
    RTS

    ; VPy_LINE:45
CHECK_INPUT: ; function
; --- function check_input ---
    ; VPy_LINE:47
    ; pass (no-op)
    RTS

    ; VPy_LINE:49
MOVE_PLAYER: ; function
; --- function move_player ---
    ; VPy_LINE:51
    ; pass (no-op)
    RTS

    ; VPy_LINE:53
UPDATE_ENEMIES: ; function
; --- function update_enemies ---
    ; VPy_LINE:55
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
    ; VPy_LINE:56
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
    ; VPy_LINE:57
    LDD #-100
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY1_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_1
IF_NEXT_2:
IF_END_1:
    ; VPy_LINE:60
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
    ; VPy_LINE:61
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
    ; VPy_LINE:62
    LDD #-100
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY2_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_5
IF_NEXT_6:
IF_END_5:
    ; VPy_LINE:65
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
    ; VPy_LINE:66
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
    ; VPy_LINE:67
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
    ; VPy_LINE:68
    LDD #100
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY3_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_9
IF_NEXT_10:
IF_END_9:
    ; VPy_LINE:69
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
    ; VPy_LINE:70
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

    ; VPy_LINE:74
DRAW_ALL: ; function
; --- function draw_all ---
    ; VPy_LINE:76
    JSR DRAW_PLAYER
    ; VPy_LINE:77
    JSR DRAW_ENEMIES
    RTS

    ; VPy_LINE:79
DRAW_PLAYER: ; function
; --- function draw_player ---
    ; VPy_LINE:81
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
    JSR Draw_Sync_List_At_With_Mirrors_BANK_WRAPPER  ; Cross-bank call to helper in bank #31
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    RTS

    ; VPy_LINE:83
DRAW_ENEMIES: ; function
; --- function draw_enemies ---
    ; VPy_LINE:85
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
    JSR Draw_Sync_List_At_With_Mirrors_BANK_WRAPPER  ; Cross-bank call to helper in bank #31
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    ; VPy_LINE:86
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
    JSR Draw_Sync_List_At_With_Mirrors_BANK_WRAPPER  ; Cross-bank call to helper in bank #31
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    ; VPy_LINE:87
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
    JSR Draw_Sync_List_At_With_Mirrors_BANK_WRAPPER  ; Cross-bank call to helper in bank #31
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    RTS


; ================================================
