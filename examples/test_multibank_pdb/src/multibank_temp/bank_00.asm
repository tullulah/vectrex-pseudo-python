ORG $0000
; CODE SECTION
;***************************************************************************
LDA #$01     ; CRITICAL EQU $0047
JSR $F1BA  ; Read_Btns EQU $0065
JSR Wait_Recal  ; CRITICAL EQU $0061
JSR $F1AA  ; DP_to_D0 EQU $0063
JSR $F1AF  ; DP_to_C8 EQU $0067
CLR $C823    ; CRITICAL EQU $0045
; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 21 bytes
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
    ; VPy_LINE:6
    ; VPy_LINE:7
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 7
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
    ; VPy_LINE:10
LOOP_BODY:
    JSR Wait_Recal  ; CRITICAL: Sync with CRT refresh (50Hz frame timing)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; VPy_LINE:11
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_0
    STX VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 11
    JSR VECTREX_PRINT_TEXT  ; Bank #31 (fixed) - no wrapper needed
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:12
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_1
    STX VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 12
    JSR VECTREX_PRINT_TEXT  ; Bank #31 (fixed) - no wrapper needed
    CLRA
    CLRB
    STD RESULT
    RTS
; ================================================
