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
    FCC "Test Math Basic"
    FCB $80                 ; String terminator
    FCB 0                   ; End of header

;***************************************************************************
; SYSTEM RAM VARIABLES
;***************************************************************************
CURRENT_ROM_BANK EQU $C880
RESULT EQU $CF00
TMPPTR EQU $CF02
TMPPTR2 EQU $CF04

;***************************************************************************
; USER VARIABLES
;***************************************************************************
VAR_RESULT_MAX EQU $CF10+0
VAR_RESULT_MIN EQU $CF10+2
VAR_RESULT_CLAMP EQU $CF10+4
VAR_RESULT_ABS EQU $CF10+6

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

.MAIN_LOOP:
    JSR LOOP_BODY
    BRA .MAIN_LOOP

LOOP_BODY:
    JSR Wait_Recal   ; Synchronize with screen refresh (mandatory)
    RTS

; Function: MAIN
MAIN:
    ; SET_INTENSITY: Set drawing intensity
    LDD #127
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    ; ABS: Absolute value
    LDD #-42
    STD RESULT
    LDD RESULT
    TSTA           ; Test sign bit
    BPL .ABS_POS   ; Branch if positive
    COMA           ; Complement A
    COMB           ; Complement B
    ADDD #1        ; Add 1 for two's complement
.ABS_POS:
    STD RESULT
    LDD RESULT
    STD VAR_RESULT_ABS
    ; MIN: Return minimum of two values
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPPTR     ; Save first value
    LDD #25
    STD RESULT
    LDD TMPPTR     ; Load first value
    CMPD RESULT    ; Compare with second
    BLE .MIN_FIRST ; Branch if first <= second
    BRA .MIN_END
.MIN_FIRST:
    STD RESULT     ; First is smaller
.MIN_END:
    LDD RESULT
    STD VAR_RESULT_MIN
    ; MAX: Return maximum of two values
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPPTR     ; Save first value
    LDD #25
    STD RESULT
    LDD TMPPTR     ; Load first value
    CMPD RESULT    ; Compare with second
    BGE .MAX_FIRST ; Branch if first >= second
    BRA .MAX_END
.MAX_FIRST:
    STD RESULT     ; First is larger
.MAX_END:
    LDD RESULT
    STD VAR_RESULT_MAX
    ; CLAMP: Clamp value to range [min, max]
    LDD #50
    STD RESULT
    LDD RESULT
    STD TMPPTR     ; Save value
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPPTR+2   ; Save min
    LDD #100
    STD RESULT
    LDD RESULT
    STD TMPPTR+4   ; Save max
    LDD TMPPTR     ; Load value
    CMPD TMPPTR+2  ; Compare with min
    BGE .CLAMP_CHK_MAX ; Branch if value >= min
    LDD TMPPTR+2
    STD RESULT
    BRA .CLAMP_END
.CLAMP_CHK_MAX:
    LDD TMPPTR     ; Load value again
    CMPD TMPPTR+4  ; Compare with max
    BLE .CLAMP_OK  ; Branch if value <= max
    LDD TMPPTR+4
    STD RESULT
    BRA .CLAMP_END
.CLAMP_OK:
    LDD TMPPTR
    STD RESULT
.CLAMP_END:
    LDD RESULT
    STD VAR_RESULT_CLAMP
    RTS

; Function: LOOP
LOOP:
    ; WAIT_RECAL: Wait for screen refresh
    JSR Wait_Recal
    LDD #0
    STD RESULT
    ; PRINT_TEXT: Print text at position
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_57328601093510      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    ; PRINT_TEXT: Print text at position
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_65109141424791851      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    ; PRINT_TEXT: Print text at position
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_64906153357880893      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    ; PRINT_TEXT: Print text at position
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_12337822332324131586      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    LDD #-15
    STD RESULT
    LDD RESULT
    STD VAR_VAL1
    LDD #30
    STD RESULT
    LDD RESULT
    STD VAR_VAL2
    ; ABS: Absolute value
    LDD VAR_VAL1
    STD RESULT
    LDD RESULT
    TSTA           ; Test sign bit
    BPL .ABS_POS   ; Branch if positive
    COMA           ; Complement A
    COMB           ; Complement B
    ADDD #1        ; Add 1 for two's complement
.ABS_POS:
    STD RESULT
    LDD RESULT
    STD VAR_ABS_VAL
    ; MIN: Return minimum of two values
    LDD VAR_VAL1
    STD RESULT
    LDD RESULT
    STD TMPPTR     ; Save first value
    LDD VAR_VAL2
    STD RESULT
    LDD TMPPTR     ; Load first value
    CMPD RESULT    ; Compare with second
    BLE .MIN_FIRST ; Branch if first <= second
    BRA .MIN_END
.MIN_FIRST:
    STD RESULT     ; First is smaller
.MIN_END:
    LDD RESULT
    STD VAR_MIN_VAL
    ; MAX: Return maximum of two values
    LDD VAR_VAL1
    STD RESULT
    LDD RESULT
    STD TMPPTR     ; Save first value
    LDD VAR_VAL2
    STD RESULT
    LDD TMPPTR     ; Load first value
    CMPD RESULT    ; Compare with second
    BGE .MAX_FIRST ; Branch if first >= second
    BRA .MAX_END
.MAX_FIRST:
    STD RESULT     ; First is larger
.MAX_END:
    LDD RESULT
    STD VAR_MAX_VAL
    ; CLAMP: Clamp value to range [min, max]
    LDD VAR_VAL2
    STD RESULT
    LDD RESULT
    STD TMPPTR     ; Save value
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPPTR+2   ; Save min
    LDD #20
    STD RESULT
    LDD RESULT
    STD TMPPTR+4   ; Save max
    LDD TMPPTR     ; Load value
    CMPD TMPPTR+2  ; Compare with min
    BGE .CLAMP_CHK_MAX ; Branch if value >= min
    LDD TMPPTR+2
    STD RESULT
    BRA .CLAMP_END
.CLAMP_CHK_MAX:
    LDD TMPPTR     ; Load value again
    CMPD TMPPTR+4  ; Compare with max
    BLE .CLAMP_OK  ; Branch if value <= max
    LDD TMPPTR+4
    STD RESULT
    BRA .CLAMP_END
.CLAMP_OK:
    LDD TMPPTR
    STD RESULT
.CLAMP_END:
    LDD RESULT
    STD VAR_CLAMPED
    RTS

;**** PRINT_TEXT String Data ****
PRINT_TEXT_STR_57328601093510:
    FCC "ABS(-42):"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_64906153357880893:
    FCC "MAX(10,25):"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_65109141424791851:
    FCC "MIN(10,25):"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_12337822332324131586:
    FCC "CLAMP(50,0,100):"
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

J1X_BUILTIN:
    ; Read J1_X from $CF00 and return -1/0/+1
    LDB $CF00      ; Joy_1_X (unsigned byte 0-255)
    CMPB #108      ; Compare with lower threshold
    BLO .J1X_LEFT  ; Branch if <108 (left)
    CMPB #148      ; Compare with upper threshold
    BHI .J1X_RIGHT ; Branch if >148 (right)
    ; Center (108-148)
    LDD #0
    RTS
.J1X_LEFT:
    LDD #-1
    RTS
.J1X_RIGHT:
    LDD #1
    RTS

J1Y_BUILTIN:
    ; Read J1_Y from $CF01 and return -1/0/+1
    LDB $CF01      ; Joy_1_Y (unsigned byte 0-255)
    CMPB #108      ; Compare with lower threshold
    BLO .J1Y_DOWN  ; Branch if <108 (down)
    CMPB #148      ; Compare with upper threshold
    BHI .J1Y_UP    ; Branch if >148 (up)
    ; Center (108-148)
    LDD #0
    RTS
.J1Y_DOWN:
    LDD #-1
    RTS
.J1Y_UP:
    LDD #1
    RTS

J2X_BUILTIN:
    ; Read J2_X from $CF02 and return -1/0/+1
    LDB $CF02      ; Joy_2_X (unsigned byte 0-255)
    CMPB #108      ; Compare with lower threshold
    BLO .J2X_LEFT  ; Branch if <108 (left)
    CMPB #148      ; Compare with upper threshold
    BHI .J2X_RIGHT ; Branch if >148 (right)
    ; Center (108-148)
    LDD #0
    RTS
.J2X_LEFT:
    LDD #-1
    RTS
.J2X_RIGHT:
    LDD #1
    RTS

J2Y_BUILTIN:
    ; Read J2_Y from $CF03 and return -1/0/+1
    LDB $CF03      ; Joy_2_Y (unsigned byte 0-255)
    CMPB #108      ; Compare with lower threshold
    BLO .J2Y_DOWN  ; Branch if <108 (down)
    CMPB #148      ; Compare with upper threshold
    BHI .J2Y_UP    ; Branch if >148 (up)
    ; Center (108-148)
    LDD #0
    RTS
.J2Y_DOWN:
    LDD #-1
    RTS
.J2Y_UP:
    LDD #1
    RTS

