ORG $0000
; CODE SECTION
;***************************************************************************
CLR $C823    ; CRITICAL EQU $005E
LDA #$01     ; CRITICAL EQU $0060
JSR $F1AA  ; DP_to_D0 EQU $007E
JSR $F1AF  ; DP_to_C8 EQU $0082
JSR Wait_Recal  ; CRITICAL EQU $007C
JSR $F1BA  ; Read_Btns EQU $0080
; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 48 bytes
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
    JSR Draw_Sync_List_At_With_Mirrors  ; Bank #31 (fixed) - no wrapper needed
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
    JSR Draw_Sync_List_At_With_Mirrors  ; Bank #31 (fixed) - no wrapper needed
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
    JSR Draw_Sync_List_At_With_Mirrors  ; Bank #31 (fixed) - no wrapper needed
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
    JSR Draw_Sync_List_At_With_Mirrors  ; Bank #31 (fixed) - no wrapper needed
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    RTS
; ================================================
