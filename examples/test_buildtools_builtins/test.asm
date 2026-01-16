; VPy M6809 Assembly (Vectrex)
; ROM: 32768 bytes

    ORG $0000

;***************************************************************************
; DEFINE SECTION
;***************************************************************************
    INCLUDE "VECTREX.I"

;***************************************************************************
; CARTRIDGE HEADER
;***************************************************************************
    FCC "g GCE 2025"
    FCB $80                 ; String terminator
    FDB $0000              ; Music pointer
    FCB $F8,$50,$20,$BB     ; Height, Width, Rel Y, Rel X
    FCC "Buildtools Builtins Test"
    FCB $80                 ; String terminator
    FCB 0                   ; End of header

;***************************************************************************
; SYSTEM RAM VARIABLES
;***************************************************************************
CURRENT_ROM_BANK EQU $C880
RESULT EQU $CF00
TMPPTR EQU $CF02

; Function argument slots
VAR_ARG0 EQU $CFE0+0
VAR_ARG1 EQU $CFE0+2
VAR_ARG2 EQU $CFE0+4
VAR_ARG3 EQU $CFE0+6
VAR_ARG4 EQU $CFE0+8

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
    JMP MAIN

;***************************************************************************
; MAIN PROGRAM
;***************************************************************************

MAIN:
    ; Call main() for initialization
    ; SET_INTENSITY: Set drawing intensity
    LDD #127
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT

.MAIN_LOOP:
    JSR LOOP_BODY
    BRA .MAIN_LOOP

LOOP_BODY:
    JSR Wait_Recal   ; Synchronize with screen refresh (mandatory)
    ; WAIT_RECAL: Wait for screen refresh
    JSR Wait_Recal
    LDD #0
    STD RESULT
    JSR J1X_BUILTIN
    STD RESULT
    LDD RESULT
    STD VAR_X
    JSR J1Y_BUILTIN
    STD RESULT
    LDD RESULT
    STD VAR_Y
    LDA $C811      ; Vec_Button_1_1 (transition bits)
    ANDA #$01      ; Test bit 0
    BEQ .J1B1_OFF
    LDD #1
    BRA .J1B1_END
.J1B1_OFF:
    LDD #0
.J1B1_END:
    STD RESULT
    LDD RESULT
    STD VAR_BTN1
    LDA $C811      ; Vec_Button_1_1 (transition bits)
    ANDA #$02      ; Test bit 1
    BEQ .J1B2_OFF
    LDD #1
    BRA .J1B2_END
.J1B2_OFF:
    LDD #0
.J1B2_END:
    STD RESULT
    LDD RESULT
    STD VAR_BTN2
    ; ABS: Absolute value
    LDD VAR_X
    STD RESULT
    LDD RESULT
    BPL .ABS_POSITIVE
    NEGD
.ABS_POSITIVE:
    STD RESULT
    LDD RESULT
    STD VAR_ABS_X
    ; MIN: Minimum of two values
    LDD VAR_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    CMPD 2,S
    BLT .MIN_USE_ARG2
    LDD 2,S
.MIN_USE_ARG2:
    LEAS 2,S
    STD RESULT
    LDD RESULT
    STD VAR_MIN_VAL
    ; MAX: Maximum of two values
    LDD VAR_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    CMPD 2,S
    BGT .MAX_USE_ARG2
    LDD 2,S
.MAX_USE_ARG2:
    LEAS 2,S
    STD RESULT
    LDD RESULT
    STD VAR_MAX_VAL
    ; AUDIO_UPDATE: Update audio/music
    JSR AUDIO_UPDATE
    LDD #0
    STD RESULT
    ; PRINT_TEXT: Print text at position
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_1501574887082362679      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
    LDD #0
    STD RESULT
    LDA RESULT+1    ; Intensity value
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA VAR_ARG1+1  ; Y0
    LDB VAR_ARG0+1  ; X0
    JSR Moveto_d    ; Move to start position
    LDD VAR_ARG3    ; y1
    SUBD VAR_ARG1   ; dy = y1 - y0
    TFR A,B         ; dy in B register
    LDA VAR_ARG2+1  ; x1 (low byte)
    SUBA VAR_ARG0+1 ; dx = x1 - x0
    JSR Draw_Line_d ; BIOS draw line
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR VECTREX_DRAW_LINE
    RTS

;**** PRINT_TEXT String Data ****
PRINT_TEXT_STR_1501574887082362679:
    FCC "BUILDTOOLS TEST OK"
    FCB $80          ; Vectrex string terminator

;***************************************************************************
; RUNTIME HELPERS
;***************************************************************************

VECTREX_PRINT_TEXT:
    ; VPy signature: PRINT_TEXT(x, y, string)
    ; BIOS signature: Print_Str_d(A=Y, B=X, U=string)
    JSR $F1AA      ; DP_to_D0 - set Direct Page for BIOS/VIA access
    LDU VAR_ARG2   ; string pointer (third parameter)
    LDA VAR_ARG1+1 ; Y coordinate (second parameter, low byte)
    LDB VAR_ARG0+1 ; X coordinate (first parameter, low byte)
    JSR Print_Str_d ; Print string from U register
    JSR $F1AF      ; DP_to_C8 - restore DP before return
    RTS

MUL16:
    ; Multiply 16-bit X * D -> D
    ; Simple implementation (can be optimized)
    PSHS X,B,A
    LDD #0         ; Result accumulator
    LDX 2,S        ; Multiplier
.MUL16_LOOP:
    BEQ .MUL16_END
    ADDD ,S        ; Add multiplicand
    LEAX -1,X
    BRA .MUL16_LOOP
.MUL16_END:
    LEAS 4,S
    RTS

DIV16:
    ; Divide 16-bit X / D -> D
    ; Simple implementation
    PSHS X,D
    LDD #0         ; Quotient
.DIV16_LOOP:
    PSHS D         ; Save quotient
    LDD 4,S        ; Load dividend (after PSHS D)
    CMPD 2,S       ; Compare with divisor (after PSHS D)
    PULS D         ; Restore quotient
    BLT .DIV16_END
    ADDD #1        ; Increment quotient
    LDX 2,S
    PSHS D
    LDD 2,S        ; Divisor
    LEAX D,X       ; Subtract divisor
    STX 4,S
    PULS D
    BRA .DIV16_LOOP
.DIV16_END:
    LEAS 4,S
    RTS

MOD16:
    ; Modulo 16-bit X % D -> D
    PSHS X,D
.MOD16_LOOP:
    PSHS D         ; Save D
    LDD 4,S        ; Load dividend (after PSHS D)
    CMPD 2,S       ; Compare with divisor (after PSHS D)
    PULS D         ; Restore D
    BLT .MOD16_END
    LDX 2,S
    LDD ,S
    LEAX D,X
    STX 2,S
    BRA .MOD16_LOOP
.MOD16_END:
    LDD 2,S        ; Remainder
    LEAS 4,S
    RTS

