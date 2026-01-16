; --- Motorola 6809 backend (Vectrex) title='BUTTON TEST' origin=$0000 ---
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
    FCC "BUTTON TEST"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 51 bytes
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
PSG_MUSIC_PTR        EQU $C880+$12   ; Current music position pointer (2 bytes)
PSG_MUSIC_START      EQU $C880+$14   ; Music start pointer (for loops) (2 bytes)
PSG_IS_PLAYING       EQU $C880+$16   ; Playing flag ($00=stopped, $01=playing) (1 bytes)
PSG_MUSIC_ACTIVE     EQU $C880+$17   ; Set during UPDATE_MUSIC_PSG (1 bytes)
PSG_FRAME_COUNT      EQU $C880+$18   ; Frame register write count (1 bytes)
PSG_DELAY_FRAMES     EQU $C880+$19   ; Frames to wait before next read (1 bytes)
SFX_PTR              EQU $C880+$1A   ; Current SFX data pointer (2 bytes)
SFX_TICK             EQU $C880+$1C   ; Current frame counter (2 bytes)
SFX_ACTIVE           EQU $C880+$1E   ; Playback state ($00=stopped, $01=playing) (1 bytes)
SFX_PHASE            EQU $C880+$1F   ; Envelope phase (0=A,1=D,2=S,3=R) (1 bytes)
SFX_VOL              EQU $C880+$20   ; Current volume level (0-15) (1 bytes)
NUM_STR              EQU $C880+$21   ; String buffer for PRINT_NUMBER (2 bytes)
VAR_BTN1_COUNTER     EQU $C880+$23   ; User variable (2 bytes)
VAR_BTN2_COUNTER     EQU $C880+$25   ; User variable (2 bytes)
VAR_BTN3_COUNTER     EQU $C880+$27   ; User variable (2 bytes)
VAR_BTN4_COUNTER     EQU $C880+$29   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$2B   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$2D   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$2F   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$31   ; Function argument 3 (2 bytes)
PSG_MUSIC_PTR_DP   EQU $12  ; DP-relative
PSG_MUSIC_START_DP EQU $14  ; DP-relative
PSG_IS_PLAYING_DP  EQU $16  ; DP-relative
PSG_MUSIC_ACTIVE_DP EQU $17  ; DP-relative
PSG_FRAME_COUNT_DP EQU $18  ; DP-relative
PSG_DELAY_FRAMES_DP EQU $19  ; DP-relative
SFX_PTR_DP         EQU $1A  ; DP-relative
SFX_TICK_DP        EQU $1C  ; DP-relative
SFX_ACTIVE_DP      EQU $1E  ; DP-relative
SFX_PHASE_DP       EQU $1F  ; DP-relative
SFX_VOL_DP         EQU $20  ; DP-relative


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
    ; VPy_LINE:9
    ; VPy_LINE:4
    LDD #0
    STD VAR_BTN1_COUNTER
    ; VPy_LINE:5
    LDD #0
    STD VAR_BTN2_COUNTER
    ; VPy_LINE:6
    LDD #0
    STD VAR_BTN3_COUNTER
    ; VPy_LINE:7
    LDD #0
    STD VAR_BTN4_COUNTER
    ; VPy_LINE:10
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 10
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
    BRA MAIN

    ; VPy_LINE:12
LOOP_BODY:
    LEAS -8,S ; allocate locals
    JSR Wait_Recal  ; CRITICAL: Sync with CRT refresh (50Hz frame timing)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; DEBUG: Statement 0 - Discriminant(0)
    ; VPy_LINE:14
; NATIVE_CALL: J1_BUTTON_1 at line 14
    JSR J1B1_BUILTIN
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; DEBUG: Statement 1 - Discriminant(0)
    ; VPy_LINE:15
; NATIVE_CALL: J1_BUTTON_2 at line 15
    JSR J1B2_BUILTIN
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; DEBUG: Statement 2 - Discriminant(0)
    ; VPy_LINE:16
; NATIVE_CALL: J1_BUTTON_3 at line 16
    JSR J1B3_BUILTIN
    STD RESULT
    LDX RESULT
    STX 4 ,S
    ; DEBUG: Statement 3 - Discriminant(0)
    ; VPy_LINE:17
; NATIVE_CALL: J1_BUTTON_4 at line 17
    JSR J1B4_BUILTIN
    STD RESULT
    LDX RESULT
    STX 6 ,S
    ; DEBUG: Statement 4 - Discriminant(9)
    ; VPy_LINE:20
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
    BEQ CT_2
    LDD #0
    STD RESULT
    BRA CE_3
CT_2:
    LDD #1
    STD RESULT
CE_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    ; VPy_LINE:21
    LDD #30
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN1_COUNTER
    STU TMPPTR
    STX ,U
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    ; DEBUG: Statement 5 - Discriminant(9)
    ; VPy_LINE:22
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
    BEQ CT_6
    LDD #0
    STD RESULT
    BRA CE_7
CT_6:
    LDD #1
    STD RESULT
CE_7:
    LDD RESULT
    LBEQ IF_NEXT_5
    ; VPy_LINE:23
    LDD #30
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN2_COUNTER
    STU TMPPTR
    STX ,U
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    ; DEBUG: Statement 6 - Discriminant(9)
    ; VPy_LINE:24
    LDD 4 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_10
    LDD #0
    STD RESULT
    BRA CE_11
CT_10:
    LDD #1
    STD RESULT
CE_11:
    LDD RESULT
    LBEQ IF_NEXT_9
    ; VPy_LINE:25
    LDD #30
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN3_COUNTER
    STU TMPPTR
    STX ,U
    LBRA IF_END_8
IF_NEXT_9:
IF_END_8:
    ; DEBUG: Statement 7 - Discriminant(9)
    ; VPy_LINE:26
    LDD 6 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
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
    LBEQ IF_NEXT_13
    ; VPy_LINE:27
    LDD #30
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN4_COUNTER
    STU TMPPTR
    STX ,U
    LBRA IF_END_12
IF_NEXT_13:
IF_END_12:
    ; DEBUG: Statement 8 - Discriminant(8)
    ; VPy_LINE:30
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_4
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 30
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 9 - Discriminant(8)
    ; VPy_LINE:31
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #70
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_6
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 31
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 10 - Discriminant(8)
    ; VPy_LINE:32
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_5
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 32
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 11 - Discriminant(9)
    ; VPy_LINE:35
    LDD VAR_BTN1_COUNTER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_18
    LDD #0
    STD RESULT
    BRA CE_19
CT_18:
    LDD #1
    STD RESULT
CE_19:
    LDD RESULT
    LBEQ IF_NEXT_17
    ; VPy_LINE:36
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_0
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 36
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:37
    LDD VAR_BTN1_COUNTER
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
    LDU #VAR_BTN1_COUNTER
    STU TMPPTR
    STX ,U
    LBRA IF_END_16
IF_NEXT_17:
IF_END_16:
    ; DEBUG: Statement 12 - Discriminant(9)
    ; VPy_LINE:39
    LDD VAR_BTN2_COUNTER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_22
    LDD #0
    STD RESULT
    BRA CE_23
CT_22:
    LDD #1
    STD RESULT
CE_23:
    LDD RESULT
    LBEQ IF_NEXT_21
    ; VPy_LINE:40
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-20
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_1
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 40
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:41
    LDD VAR_BTN2_COUNTER
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
    LDU #VAR_BTN2_COUNTER
    STU TMPPTR
    STX ,U
    LBRA IF_END_20
IF_NEXT_21:
IF_END_20:
    ; DEBUG: Statement 13 - Discriminant(9)
    ; VPy_LINE:43
    LDD VAR_BTN3_COUNTER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_26
    LDD #0
    STD RESULT
    BRA CE_27
CT_26:
    LDD #1
    STD RESULT
CE_27:
    LDD RESULT
    LBEQ IF_NEXT_25
    ; VPy_LINE:44
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_2
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 44
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:45
    LDD VAR_BTN3_COUNTER
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
    LDU #VAR_BTN3_COUNTER
    STU TMPPTR
    STX ,U
    LBRA IF_END_24
IF_NEXT_25:
IF_END_24:
    ; DEBUG: Statement 14 - Discriminant(9)
    ; VPy_LINE:47
    LDD VAR_BTN4_COUNTER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_30
    LDD #0
    STD RESULT
    BRA CE_31
CT_30:
    LDD #1
    STD RESULT
CE_31:
    LDD RESULT
    LBEQ IF_NEXT_29
    ; VPy_LINE:48
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_3
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 48
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:49
    LDD VAR_BTN4_COUNTER
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
    LDU #VAR_BTN4_COUNTER
    STU TMPPTR
    STX ,U
    LBRA IF_END_28
IF_NEXT_29:
IF_END_28:
    LEAS 8,S ; free locals
    RTS

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
    LBEQ .J1B1_OFF ; Long branch (helpers may be >127 bytes away)
    LDD #1         ; Return pressed (rising edge)
    RTS
.J1B1_OFF:
    LDD #0         ; Return not pressed
    RTS

J1B2_BUILTIN:
    LDA $C811
    ANDA #$02      ; Test bit 1 (Button 2)
    LBEQ .J1B2_OFF ; Long branch
    LDD #1
    RTS
.J1B2_OFF:
    LDD #0
    RTS

J1B3_BUILTIN:
    LDA $C811
    ANDA #$04      ; Test bit 2 (Button 3)
    LBEQ .J1B3_OFF ; Long branch
    LDD #1
    RTS
.J1B3_OFF:
    LDD #0
    RTS

J1B4_BUILTIN:
    LDA $C811
    ANDA #$08      ; Test bit 3 (Button 4)
    LBEQ .J1B4_OFF ; Long branch
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
;***************************************************************************
; DATA SECTION
;***************************************************************************

; ========================================
; NO ASSETS EMBEDDED
; All 5 discovered assets are unused in code
; ========================================

; === INLINE ARRAY LITERALS (from function bodies) ===
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC ">>> BTN 1 PRESSED <<<"
    FCB $80
STR_1:
    FCC ">>> BTN 2 PRESSED <<<"
    FCB $80
STR_2:
    FCC ">>> BTN 3 PRESSED <<<"
    FCB $80
STR_3:
    FCC ">>> BTN 4 PRESSED <<<"
    FCB $80
STR_4:
    FCC "BUTTON DEBOUNCE TEST"
    FCB $80
STR_5:
    FCC "ONE PRESS = ONE MSG"
    FCB $80
STR_6:
    FCC "PRESS ANY BUTTON"
    FCB $80

; === RESET Vector (Entry Point) ===
; Other vectors ($FFF0-$FFFC) provided by BIOS ROM
    ORG $FFFE
    FDB START           ; RESET vector (entry point)
