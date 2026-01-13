; --- Motorola 6809 backend (Vectrex) title='TestController' origin=$0000 ---
        ORG $0000
;***************************************************************************
; DEFINE SECTION
;***************************************************************************
    INCLUDE "VECTREX.I"

;***************************************************************************
; HEADER SECTION
;***************************************************************************
    FCC "g GCE 1982"
    FCB $80
    FDB music1
    FCB $F8
    FCB $50
    FCB $20
    FCB $BB
    FCC "TESTCONTROLLER"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 49 bytes
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$02   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$04   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$06   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$08   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0A   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0C   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0E   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$10   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$11   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$12   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$14   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$15   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$16   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$17   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$18   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VAR_PLAYER_X         EQU $C880+$19   ; User variable (2 bytes)
VAR_PLAYER_Y         EQU $C880+$1B   ; User variable (2 bytes)
VAR_BTN_DEBOUNCE     EQU $C880+$1D   ; User variable (2 bytes)
VAR_STARTUP_DELAY    EQU $C880+$1F   ; User variable (2 bytes)
VAR_BTN1             EQU $C880+$21   ; User variable (2 bytes)
VAR_BTN2             EQU $C880+$23   ; User variable (2 bytes)
VAR_BTN3             EQU $C880+$25   ; User variable (2 bytes)
VAR_BTN4             EQU $C880+$27   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$29   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$2B   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$2D   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$2F   ; Function argument 3 (2 bytes)


;**** CONST DECLARATIONS (NUMBER-ONLY) ****
; VPy_LINE:12
; _CONST_DECL_0:  ; const BTN_DEBOUNCE_FRAMES

;
; ┌─────────────────────────────────────────────────────────────────┐
; │ PROGRAM CODE SECTION - User VPy Code                            │
; │ This section contains the compiled user program logic.          │
; └─────────────────────────────────────────────────────────────────┘
;

CUSTOM_RESET:
    ; RESET vector handler - entry point from $FFFE
START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
    CLR $C80E        ; Initialize Vec_Prev_Btns to 0 for Read_Btns debounce
    LDA #$80
    STA VIA_t1_cnt_lo
    LDS #$CBFF       ; Initialize stack at top of RAM (safer than Vec_Default_Stk)

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:21
    ; VPy_LINE:7
    LDD #0
    STD VAR_PLAYER_X
    ; VPy_LINE:8
    LDD #0
    STD VAR_PLAYER_Y
    ; VPy_LINE:11
    LDD #0
    STD VAR_BTN_DEBOUNCE
    ; VPy_LINE:13
    LDD #0
    STD VAR_STARTUP_DELAY
    ; VPy_LINE:16
    LDD #0
    STD VAR_BTN1
    ; VPy_LINE:17
    LDD #0
    STD VAR_BTN2
    ; VPy_LINE:18
    LDD #0
    STD VAR_BTN3
    ; VPy_LINE:19
    LDD #0
    STD VAR_BTN4
    ; VPy_LINE:22
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 22
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:23
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN_DEBOUNCE
    STU TMPPTR
    STX ,U
    ; VPy_LINE:24
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_STARTUP_DELAY
    STU TMPPTR
    STX ,U

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
    BRA MAIN

    ; VPy_LINE:26
LOOP_BODY:
    LEAS -4,S ; allocate locals
    JSR Wait_Recal  ; CRITICAL: Sync with CRT refresh (50Hz frame timing)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; DEBUG: Statement 0 - Discriminant(9)
    ; VPy_LINE:29
    LDD VAR_BTN_DEBOUNCE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_2
    LDD #0
    STD RESULT
    BRA CE_3
CT_2:
    LDD #1
    STD RESULT
CE_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    ; VPy_LINE:30
    LDD VAR_BTN_DEBOUNCE
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
    LDU #VAR_BTN_DEBOUNCE
    STU TMPPTR
    STX ,U
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    ; DEBUG: Statement 1 - Discriminant(9)
    ; VPy_LINE:33
    LDD VAR_STARTUP_DELAY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #60
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_6
    LDD #0
    STD RESULT
    BRA CE_7
CT_6:
    LDD #1
    STD RESULT
CE_7:
    LDD RESULT
    LBEQ IF_NEXT_5
    ; VPy_LINE:34
    LDD VAR_STARTUP_DELAY
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
    LDU #VAR_STARTUP_DELAY
    STU TMPPTR
    STX ,U
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    ; DEBUG: Statement 2 - Discriminant(0)
    ; VPy_LINE:37
; NATIVE_CALL: J1_X at line 37
    JSR J1X_BUILTIN
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; DEBUG: Statement 3 - Discriminant(0)
    ; VPy_LINE:38
; NATIVE_CALL: J1_Y at line 38
    JSR J1Y_BUILTIN
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; DEBUG: Statement 4 - Discriminant(0)
    ; VPy_LINE:41
; NATIVE_CALL: J1_BUTTON_1 at line 41
    JSR J1B1_BUILTIN
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN1
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 5 - Discriminant(0)
    ; VPy_LINE:42
; NATIVE_CALL: J1_BUTTON_2 at line 42
    JSR J1B2_BUILTIN
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN2
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 6 - Discriminant(0)
    ; VPy_LINE:43
; NATIVE_CALL: J1_BUTTON_3 at line 43
    JSR J1B3_BUILTIN
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN3
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 7 - Discriminant(0)
    ; VPy_LINE:44
; NATIVE_CALL: J1_BUTTON_4 at line 44
    JSR J1B4_BUILTIN
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN4
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 8 - Discriminant(9)
    ; VPy_LINE:47
    LDD VAR_STARTUP_DELAY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #60
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_10
    LDD #0
    STD RESULT
    BRA CE_11
CT_10:
    LDD #1
    STD RESULT
CE_11:
    LDD RESULT
    BEQ AND_FALSE_12
    LDD VAR_BTN_DEBOUNCE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_14
    LDD #0
    STD RESULT
    BRA CE_15
CT_14:
    LDD #1
    STD RESULT
CE_15:
    LDD RESULT
    BEQ AND_FALSE_12
    LDD #1
    STD RESULT
    BRA AND_END_13
AND_FALSE_12:
    LDD #0
    STD RESULT
AND_END_13:
    LDD RESULT
    LBEQ IF_NEXT_9
    ; VPy_LINE:48
    LDD VAR_BTN1
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_24
    LDD #0
    STD RESULT
    BRA CE_25
CT_24:
    LDD #1
    STD RESULT
CE_25:
    LDD RESULT
    BNE OR_TRUE_22
    LDD VAR_BTN2
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_26
    LDD #0
    STD RESULT
    BRA CE_27
CT_26:
    LDD #1
    STD RESULT
CE_27:
    LDD RESULT
    BNE OR_TRUE_22
    LDD #0
    STD RESULT
    BRA OR_END_23
OR_TRUE_22:
    LDD #1
    STD RESULT
OR_END_23:
    LDD RESULT
    BNE OR_TRUE_20
    LDD VAR_BTN3
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_28
    LDD #0
    STD RESULT
    BRA CE_29
CT_28:
    LDD #1
    STD RESULT
CE_29:
    LDD RESULT
    BNE OR_TRUE_20
    LDD #0
    STD RESULT
    BRA OR_END_21
OR_TRUE_20:
    LDD #1
    STD RESULT
OR_END_21:
    LDD RESULT
    BNE OR_TRUE_18
    LDD VAR_BTN4
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_30
    LDD #0
    STD RESULT
    BRA CE_31
CT_30:
    LDD #1
    STD RESULT
CE_31:
    LDD RESULT
    BNE OR_TRUE_18
    LDD #0
    STD RESULT
    BRA OR_END_19
OR_TRUE_18:
    LDD #1
    STD RESULT
OR_END_19:
    LDD RESULT
    LBEQ IF_NEXT_17
    ; VPy_LINE:49
    LDD #15
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN_DEBOUNCE
    STU TMPPTR
    STX ,U
    LBRA IF_END_16
IF_NEXT_17:
IF_END_16:
    LBRA IF_END_8
IF_NEXT_9:
IF_END_8:
    ; DEBUG: Statement 9 - Discriminant(9)
    ; VPy_LINE:52
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_34
    LDD #0
    STD RESULT
    BRA CE_35
CT_34:
    LDD #1
    STD RESULT
CE_35:
    LDD RESULT
    LBEQ IF_NEXT_33
    ; VPy_LINE:53
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_32
IF_NEXT_33:
IF_END_32:
    ; DEBUG: Statement 10 - Discriminant(9)
    ; VPy_LINE:54
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_38
    LDD #0
    STD RESULT
    BRA CE_39
CT_38:
    LDD #1
    STD RESULT
CE_39:
    LDD RESULT
    LBEQ IF_NEXT_37
    ; VPy_LINE:55
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_36
IF_NEXT_37:
IF_END_36:
    ; DEBUG: Statement 11 - Discriminant(9)
    ; VPy_LINE:56
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_42
    LDD #0
    STD RESULT
    BRA CE_43
CT_42:
    LDD #1
    STD RESULT
CE_43:
    LDD RESULT
    LBEQ IF_NEXT_41
    ; VPy_LINE:57
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_40
IF_NEXT_41:
IF_END_40:
    ; DEBUG: Statement 12 - Discriminant(9)
    ; VPy_LINE:58
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_46
    LDD #0
    STD RESULT
    BRA CE_47
CT_46:
    LDD #1
    STD RESULT
CE_47:
    LDD RESULT
    LBEQ IF_NEXT_45
    ; VPy_LINE:59
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_44
IF_NEXT_45:
IF_END_44:
    ; DEBUG: Statement 13 - Discriminant(9)
    ; VPy_LINE:62
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-110
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_50
    LDD #0
    STD RESULT
    BRA CE_51
CT_50:
    LDD #1
    STD RESULT
CE_51:
    LDD RESULT
    LBEQ IF_NEXT_49
    ; VPy_LINE:63
    LDD #-110
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_48
IF_NEXT_49:
IF_END_48:
    ; DEBUG: Statement 14 - Discriminant(9)
    ; VPy_LINE:64
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #110
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_54
    LDD #0
    STD RESULT
    BRA CE_55
CT_54:
    LDD #1
    STD RESULT
CE_55:
    LDD RESULT
    LBEQ IF_NEXT_53
    ; VPy_LINE:65
    LDD #110
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_52
IF_NEXT_53:
IF_END_52:
    ; DEBUG: Statement 15 - Discriminant(9)
    ; VPy_LINE:66
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-90
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_58
    LDD #0
    STD RESULT
    BRA CE_59
CT_58:
    LDD #1
    STD RESULT
CE_59:
    LDD RESULT
    LBEQ IF_NEXT_57
    ; VPy_LINE:67
    LDD #-90
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_56
IF_NEXT_57:
IF_END_56:
    ; DEBUG: Statement 16 - Discriminant(9)
    ; VPy_LINE:68
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #90
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_62
    LDD #0
    STD RESULT
    BRA CE_63
CT_62:
    LDD #1
    STD RESULT
CE_63:
    LDD RESULT
    LBEQ IF_NEXT_61
    ; VPy_LINE:69
    LDD #90
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_60
IF_NEXT_61:
IF_END_60:
    ; DEBUG: Statement 17 - Discriminant(8)
    ; VPy_LINE:72
; DRAW_VECTOR("test", x, y) - 1 path(s) at position
    LDD VAR_PLAYER_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD VAR_PLAYER_Y
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
    LDX #_TEST_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses unified mirror function
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    ; DEBUG: Statement 18 - Discriminant(8)
    ; VPy_LINE:75
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 75
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 19 - Discriminant(9)
    ; VPy_LINE:78
    LDD VAR_BTN1
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_66
    LDD #0
    STD RESULT
    BRA CE_67
CT_66:
    LDD #1
    STD RESULT
CE_67:
    LDD RESULT
    LBEQ IF_NEXT_65
    ; VPy_LINE:79
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_1
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 79
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_64
IF_NEXT_65:
    ; VPy_LINE:81
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_0
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 81
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
IF_END_64:
    ; DEBUG: Statement 20 - Discriminant(9)
    ; VPy_LINE:84
    LDD VAR_BTN2
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_70
    LDD #0
    STD RESULT
    BRA CE_71
CT_70:
    LDD #1
    STD RESULT
CE_71:
    LDD RESULT
    LBEQ IF_NEXT_69
    ; VPy_LINE:85
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_3
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 85
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_68
IF_NEXT_69:
    ; VPy_LINE:87
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_2
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 87
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
IF_END_68:
    ; DEBUG: Statement 21 - Discriminant(9)
    ; VPy_LINE:90
    LDD VAR_BTN3
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_74
    LDD #0
    STD RESULT
    BRA CE_75
CT_74:
    LDD #1
    STD RESULT
CE_75:
    LDD RESULT
    LBEQ IF_NEXT_73
    ; VPy_LINE:91
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_5
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 91
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_72
IF_NEXT_73:
    ; VPy_LINE:93
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_4
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 93
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
IF_END_72:
    ; DEBUG: Statement 22 - Discriminant(9)
    ; VPy_LINE:96
    LDD VAR_BTN4
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_78
    LDD #0
    STD RESULT
    BRA CE_79
CT_78:
    LDD #1
    STD RESULT
CE_79:
    LDD RESULT
    LBEQ IF_NEXT_77
    ; VPy_LINE:97
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_7
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 97
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_76
IF_NEXT_77:
    ; VPy_LINE:99
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_6
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 99
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
IF_END_76:
    ; DEBUG: Statement 23 - Discriminant(9)
    ; VPy_LINE:102
    LDD VAR_BTN_DEBOUNCE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_82
    LDD #0
    STD RESULT
    BRA CE_83
CT_82:
    LDD #1
    STD RESULT
CE_83:
    LDD RESULT
    LBEQ IF_NEXT_81
    ; VPy_LINE:103
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-20
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_8
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 103
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_80
IF_NEXT_81:
IF_END_80:
    ; DEBUG: Statement 24 - Discriminant(9)
    ; VPy_LINE:106
    LDD VAR_STARTUP_DELAY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #60
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_86
    LDD #0
    STD RESULT
    BRA CE_87
CT_86:
    LDD #1
    STD RESULT
CE_87:
    LDD RESULT
    LBEQ IF_NEXT_85
    ; VPy_LINE:107
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_9
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 107
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_84
IF_NEXT_85:
IF_END_84:
    LEAS 4,S ; free locals
    RTS

BTN_DEBOUNCE_FRAMES EQU 15
;
; ┌─────────────────────────────────────────────────────────────────┐
; │ RUNTIME SECTION - VPy Builtin Helpers & System Functions       │
; │ This section contains reusable code shared across all VPy       │
; │ programs. These helpers are emitted once per compilation unit.  │
; └─────────────────────────────────────────────────────────────────┘
;

; === JOYSTICK BUILTIN SUBROUTINES ===
; J1_X() - Read Joystick 1 X axis (INCREMENTAL - with state preservation)
; Returns: D = raw value from $C81B after Joy_Analog call
J1X_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81B from hardware)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81B)
    LDB $C81B    ; Vec_Joy_1_X (BIOS writes ~$FE at center)
    SEX          ; Sign-extend B to D
    ADDD #2      ; Calibrate center offset
    PULS X       ; Restore X
    RTS

; J1_Y() - Read Joystick 1 Y axis (INCREMENTAL - with state preservation)
; Returns: D = raw value from $C81C after Joy_Analog call
J1Y_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81C from hardware)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81C)
    LDB $C81C    ; Vec_Joy_1_Y (BIOS writes ~$FE at center)
    SEX          ; Sign-extend B to D
    ADDD #2      ; Calibrate center offset
    PULS X       ; Restore X
    RTS

; === BUTTON SYSTEM - BIOS TRANSITIONS ===
; J1_BUTTON_1-4() - Read transition bits from $C811
; Read_Btns (auto-injected) calculates: ~(new) OR Vec_Prev_Btns
; Result: bit=1 ONLY on rising edge (0→1 transition)
; Returns: D = 1 (just pressed), 0 (not pressed or still held)

J1B1_BUILTIN:
    LDA $C811      ; Read transition bits (Vec_Button_1_1)
    ANDA #$01      ; Test bit 0 (Button 1)
    BEQ .J1B1_OFF
    LDD #1         ; Return pressed (rising edge)
    RTS
.J1B1_OFF:
    LDD #0         ; Return not pressed
    RTS

J1B2_BUILTIN:
    LDA $C811
    ANDA #$02      ; Test bit 1 (Button 2)
    BEQ .J1B2_OFF
    LDD #1
    RTS
.J1B2_OFF:
    LDD #0
    RTS

J1B3_BUILTIN:
    LDA $C811
    ANDA #$04      ; Test bit 2 (Button 3)
    BEQ .J1B3_OFF
    LDD #1
    RTS
.J1B3_OFF:
    LDD #0
    RTS

J1B4_BUILTIN:
    LDA $C811
    ANDA #$08      ; Test bit 3 (Button 4)
    BEQ .J1B4_OFF
    LDD #1
    RTS
.J1B4_OFF:
    LDD #0
    RTS

VECTREX_PRINT_TEXT:
    ; CRITICAL: Print_Str_d requires DP=$D0 and signature is (Y, X, string)
    ; VPy signature: PRINT_TEXT(x, y, string) -> args (ARG0=x, ARG1=y, ARG2=string)
    ; BIOS signature: Print_Str_d(A=Y, B=X, U=string)
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode for text rendering)
    STA >$D00C     ; VIA_cntl
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDU VAR_ARG2   ; string pointer (ARG2 = third param)
    LDA VAR_ARG1+1 ; Y (ARG1 = second param)
    LDB VAR_ARG0+1 ; X (ARG0 = first param)
    JSR Print_Str_d
    JSR $F1AF      ; DP_to_C8 (restore before return - CRITICAL for TMPPTR access)
    RTS
VECTREX_SET_INTENSITY:
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode)
    STA >$D00C     ; VIA_cntl
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDA VAR_ARG0+1
    JSR __Intensity_a
    RTS
; BIOS Wrappers - VIDE compatible (ensure DP=$D0 per call)
__Intensity_a:
TFR B,A         ; Move B to A (BIOS expects intensity in A)
JMP Intensity_a ; JMP (not JSR) - BIOS returns to original caller
__Reset0Ref:
JMP Reset0Ref   ; JMP (not JSR) - BIOS returns to original caller
__Moveto_d:
LDA 2,S         ; Get Y from stack (after return address)
JMP Moveto_d    ; JMP (not JSR) - BIOS returns to original caller
__Draw_Line_d:
LDA 2,S         ; Get dy from stack (after return address)
JMP Draw_Line_d ; JMP (not JSR) - BIOS returns to original caller
; ============================================================================
; Draw_Sync_List - EXACT port of Malban's draw_synced_list_c
; Data: FCB intensity, y_start, x_start, next_y, next_x, [flag, dy, dx]*, 2
; ============================================================================
Draw_Sync_List:
; ITERACIÓN 11: Loop completo dentro (bug assembler arreglado, datos embebidos OK)
LDA ,X+                 ; intensity
JSR $F2AB               ; BIOS Intensity_a (expects value in A)
LDB ,X+                 ; y_start
LDA ,X+                 ; x_start
STD TEMP_YX             ; Guardar en variable temporal (evita stack)
; Reset completo
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move sequence
LDD TEMP_YX             ; Recuperar y,x
STB VIA_port_a          ; y to DAC
PSHS A                  ; Save x
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore x
STA VIA_port_a          ; x to DAC
; Timing setup
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
; Wait for move to complete
DSL_W1:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_W1
; Loop de dibujo
DSL_LOOP:
LDA ,X+                 ; Read flag
CMPA #2                 ; Check end marker
LBEQ DSL_DONE           ; Exit if end (long branch)
CMPA #1                 ; Check next path marker
LBEQ DSL_NEXT_PATH      ; Process next path (long branch)
; Draw line
CLR Vec_Misc_Count      ; Clear for relative line drawing (CRITICAL for continuity)
LDB ,X+                 ; dy
LDA ,X+                 ; dx
PSHS A                  ; Save dx
STB VIA_port_a          ; dy to DAC
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore dx
STA VIA_port_a          ; dx to DAC
CLR VIA_t1_cnt_hi
LDA #$FF
STA VIA_shift_reg
; Wait for line draw
DSL_W2:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_W2
CLR VIA_shift_reg
LBRA DSL_LOOP            ; Long branch back to loop start
; Next path: read new intensity and header, then continue drawing
DSL_NEXT_PATH:
; Save current X position before reading anything
TFR X,D                 ; D = X (current position)
PSHS D                  ; Save X address
LDA ,X+                 ; Read intensity (X now points to y_start)
PSHS A                  ; Save intensity
LDB ,X+                 ; y_start
LDA ,X+                 ; x_start (X now points to next_y)
STD TEMP_YX             ; Save y,x
PULS A                  ; Get intensity back
PSHS A                  ; Save intensity again
LDA #$D0
TFR A,DP                ; Set DP=$D0 (BIOS requirement)
PULS A                  ; Restore intensity
JSR $F2AB               ; BIOS Intensity_a (may corrupt X!)
; Restore X to point to next_y,next_x (after the 3 bytes we read)
PULS D                  ; Get original X
ADDD #3                 ; Skip intensity, y_start, x_start
TFR D,X                 ; X now points to next_y
; Reset to zero (same as Draw_Sync_List start)
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move to new start position
LDD TEMP_YX
STB VIA_port_a          ; y to DAC
PSHS A
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A
STA VIA_port_a          ; x to DAC
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
; Wait for move
DSL_W3:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_W3
CLR VIA_shift_reg       ; Clear before continuing
LBRA DSL_LOOP            ; Continue drawing - LONG BRANCH
DSL_DONE:
RTS
Draw_Sync_List_At_With_Mirrors:
; Unified mirror support using flags: MIRROR_X and MIRROR_Y
; Conditionally negates X and/or Y coordinates and deltas
; NOTE: Caller must ensure DP=$D0 for VIA access
LDA DRAW_VEC_INTENSITY  ; Check if intensity override is set
BNE DSWM_USE_OVERRIDE   ; If non-zero, use override
LDA ,X+                 ; Otherwise, read intensity from vector data
BRA DSWM_SET_INTENSITY
DSWM_USE_OVERRIDE:
LEAX 1,X                ; Skip intensity byte in vector data
DSWM_SET_INTENSITY:
JSR $F2AB               ; BIOS Intensity_a
LDB ,X+                 ; y_start from .vec (already relative to center)
; Check if Y mirroring is enabled
TST MIRROR_Y
BEQ DSWM_NO_NEGATE_Y
NEGB                    ; ← Negate Y if flag set
DSWM_NO_NEGATE_Y:
ADDB DRAW_VEC_Y         ; Add Y offset
LDA ,X+                 ; x_start from .vec (already relative to center)
; Check if X mirroring is enabled
TST MIRROR_X
BEQ DSWM_NO_NEGATE_X
NEGA                    ; ← Negate X if flag set
DSWM_NO_NEGATE_X:
ADDA DRAW_VEC_X         ; Add X offset
STD TEMP_YX             ; Save adjusted position
; Reset completo
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move sequence
LDD TEMP_YX
STB VIA_port_a          ; y to DAC
PSHS A                  ; Save x
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore x
STA VIA_port_a          ; x to DAC
; Timing setup
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
; Wait for move to complete
DSWM_W1:
LDA VIA_int_flags
ANDA #$40
BEQ DSWM_W1
; Loop de dibujo (conditional mirrors)
DSWM_LOOP:
LDA ,X+                 ; Read flag
CMPA #2                 ; Check end marker
LBEQ DSWM_DONE
CMPA #1                 ; Check next path marker
LBEQ DSWM_NEXT_PATH
; Draw line with conditional negations
LDB ,X+                 ; dy
; Check if Y mirroring is enabled
TST MIRROR_Y
BEQ DSWM_NO_NEGATE_DY
NEGB                    ; ← Negate dy if flag set
DSWM_NO_NEGATE_DY:
LDA ,X+                 ; dx
; Check if X mirroring is enabled
TST MIRROR_X
BEQ DSWM_NO_NEGATE_DX
NEGA                    ; ← Negate dx if flag set
DSWM_NO_NEGATE_DX:
PSHS A                  ; Save final dx
STB VIA_port_a          ; dy (possibly negated) to DAC
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore final dx
STA VIA_port_a          ; dx (possibly negated) to DAC
CLR VIA_t1_cnt_hi
LDA #$FF
STA VIA_shift_reg
; Wait for line draw
DSWM_W2:
LDA VIA_int_flags
ANDA #$40
BEQ DSWM_W2
CLR VIA_shift_reg
LBRA DSWM_LOOP          ; Long branch
; Next path: repeat mirror logic for new path header
DSWM_NEXT_PATH:
TFR X,D
PSHS D
; Check intensity override (same logic as start)
LDA DRAW_VEC_INTENSITY  ; Check if intensity override is set
BNE DSWM_NEXT_USE_OVERRIDE   ; If non-zero, use override
LDA ,X+                 ; Otherwise, read intensity from vector data
BRA DSWM_NEXT_SET_INTENSITY
DSWM_NEXT_USE_OVERRIDE:
LEAX 1,X                ; Skip intensity byte in vector data
DSWM_NEXT_SET_INTENSITY:
PSHS A
LDB ,X+                 ; y_start
TST MIRROR_Y
BEQ DSWM_NEXT_NO_NEGATE_Y
NEGB
DSWM_NEXT_NO_NEGATE_Y:
ADDB DRAW_VEC_Y         ; Add Y offset
LDA ,X+                 ; x_start
TST MIRROR_X
BEQ DSWM_NEXT_NO_NEGATE_X
NEGA
DSWM_NEXT_NO_NEGATE_X:
ADDA DRAW_VEC_X         ; Add X offset
STD TEMP_YX
PULS A                  ; Get intensity back
JSR $F2AB
PULS D
ADDD #3
TFR D,X
; Reset to zero
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move to new start position
LDD TEMP_YX
STB VIA_port_a
PSHS A
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A
STA VIA_port_a
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X
; Wait for move
DSWM_W3:
LDA VIA_int_flags
ANDA #$40
BEQ DSWM_W3
CLR VIA_shift_reg
LBRA DSWM_LOOP          ; Long branch
DSWM_DONE:
RTS
;***************************************************************************
; DATA SECTION
;***************************************************************************

; ========================================
; ASSET DATA SECTION
; Embedded 1 of 1 assets (unused assets excluded)
; ========================================

; Vector asset: test
; Generated from test.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 3
; X bounds: min=-15, max=15, width=30
; Center: (0, 5)

_TEST_WIDTH EQU 30
_TEST_CENTER_X EQU 0
_TEST_CENTER_Y EQU 5

_TEST_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _TEST_PATH0        ; pointer to path 0

_TEST_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0F,$00,0,0        ; path0: header (y=15, x=0, relative to center)
    FCB $FF,$E2,$F1          ; line 0: flag=-1, dy=-30, dx=-15
    FCB $FF,$00,$1E          ; line 1: flag=-1, dy=0, dx=30
    FCB $FF,$1E,$F1          ; closing line: flag=-1, dy=30, dx=-15
    FCB 2                ; End marker (path complete)

; === INLINE ARRAY LITERALS (from function bodies) ===
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "BTN1: OFF"
    FCB $80
STR_1:
    FCC "BTN1: ON "
    FCB $80
STR_2:
    FCC "BTN2: OFF"
    FCB $80
STR_3:
    FCC "BTN2: ON "
    FCB $80
STR_4:
    FCC "BTN3: OFF"
    FCB $80
STR_5:
    FCC "BTN3: ON "
    FCB $80
STR_6:
    FCC "BTN4: OFF"
    FCB $80
STR_7:
    FCC "BTN4: ON "
    FCB $80
STR_8:
    FCC "DEBOUNCE"
    FCB $80
STR_9:
    FCC "STARTING..."
    FCB $80

; === RESET Vector (Entry Point) ===
; Other vectors ($FFF0-$FFFC) provided by BIOS ROM
    ORG $FFFE
    FDB CUSTOM_RESET    ; RESET vector (entry point)
