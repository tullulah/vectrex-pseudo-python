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
    FCC "Test Joystick 2"
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
VAR_PLAYER2_X EQU $CF10+0
VAR_PLAYER2_Y EQU $CF10+2

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
    RTS

; Function: LOOP
LOOP:
    ; WAIT_RECAL: Wait for screen refresh
    JSR Wait_Recal
    LDD #0
    STD RESULT
    JSR J2X_BUILTIN
    STD RESULT
    LDD RESULT
    STD VAR_DX
    JSR J2Y_BUILTIN
    STD RESULT
    LDD RESULT
    STD VAR_DY
    LDD VAR_PLAYER2_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_DX
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_PLAYER2_X
    LDD VAR_PLAYER2_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_DY
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_PLAYER2_Y
    LDA $C812      ; Vec_Button_1_2 (Player 2 transition bits)
    ANDA #$01      ; Test bit 0
    BEQ .J2B1_OFF
    LDD #1
    BRA .J2B1_END
.J2B1_OFF:
    LDD #0
.J2B1_END:
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    BEQ .CMP_TRUE
    LDD #0
    BRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    BEQ .IF_ELSE
    ; PRINT_TEXT: Print text at position
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_67138332013      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    BRA .IF_END
.IF_ELSE:
.IF_END:
    LDA $C812      ; Vec_Button_1_2 (Player 2 transition bits)
    ANDA #$02      ; Test bit 1
    BEQ .J2B2_OFF
    LDD #1
    BRA .J2B2_END
.J2B2_OFF:
    LDD #0
.J2B2_END:
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    BEQ .CMP_TRUE
    LDD #0
    BRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    BEQ .IF_ELSE
    ; PRINT_TEXT: Print text at position
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_67138332014      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    BRA .IF_END
.IF_ELSE:
.IF_END:
    LDA $C812      ; Vec_Button_1_2 (Player 2 transition bits)
    ANDA #$04      ; Test bit 2
    BEQ .J2B3_OFF
    LDD #1
    BRA .J2B3_END
.J2B3_OFF:
    LDD #0
.J2B3_END:
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    BEQ .CMP_TRUE
    LDD #0
    BRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    BEQ .IF_ELSE
    ; PRINT_TEXT: Print text at position
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_67138332015      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    BRA .IF_END
.IF_ELSE:
.IF_END:
    LDA $C812      ; Vec_Button_1_2 (Player 2 transition bits)
    ANDA #$08      ; Test bit 3
    BEQ .J2B4_OFF
    LDD #1
    BRA .J2B4_END
.J2B4_OFF:
    LDD #0
.J2B4_END:
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    BEQ .CMP_TRUE
    LDD #0
    BRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    BEQ .IF_ELSE
    ; PRINT_TEXT: Print text at position
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #30
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_67138332016      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    BRA .IF_END
.IF_ELSE:
.IF_END:
    ; J2_BUTTON_UP: Player 2 D-pad UP
    LDB $CF03      ; Joy_2_Y
    CMPB #149      ; Threshold for UP (>148)
    BHI .J2UP_ON
    LDD #0
    BRA .J2UP_END
.J2UP_ON:
    LDD #1
.J2UP_END:
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    BEQ .CMP_TRUE
    LDD #0
    BRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    BEQ .IF_ELSE
    ; PRINT_TEXT: Print text at position
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_69863571      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    BRA .IF_END
.IF_ELSE:
.IF_END:
    ; J2_BUTTON_DOWN: Player 2 D-pad DOWN
    LDB $CF03      ; Joy_2_Y
    CMPB #108      ; Threshold for DOWN (<108)
    BLO .J2DN_ON
    LDD #0
    BRA .J2DN_END
.J2DN_ON:
    LDD #1
.J2DN_END:
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    BEQ .CMP_TRUE
    LDD #0
    BRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    BEQ .IF_ELSE
    ; PRINT_TEXT: Print text at position
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #10
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_67138387098      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    BRA .IF_END
.IF_ELSE:
.IF_END:
    ; J2_BUTTON_LEFT: Player 2 D-pad LEFT
    LDB $CF02      ; Joy_2_X
    CMPB #108      ; Threshold for LEFT (<108)
    BLO .J2LFT_ON
    LDD #0
    BRA .J2LFT_END
.J2LFT_ON:
    LDD #1
.J2LFT_END:
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    BEQ .CMP_TRUE
    LDD #0
    BRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    BEQ .IF_ELSE
    ; PRINT_TEXT: Print text at position
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_67138615295      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    BRA .IF_END
.IF_ELSE:
.IF_END:
    ; J2_BUTTON_RIGHT: Player 2 D-pad RIGHT
    LDB $CF02      ; Joy_2_X
    CMPB #149      ; Threshold for RIGHT (>148)
    BHI .J2RGT_ON
    LDD #0
    BRA .J2RGT_END
.J2RGT_ON:
    LDD #1
.J2RGT_END:
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    BEQ .CMP_TRUE
    LDD #0
    BRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    BEQ .IF_ELSE
    ; PRINT_TEXT: Print text at position
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-10
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_2081302735108      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    BRA .IF_END
.IF_ELSE:
.IF_END:
    ; J2_ANALOG_X: Read raw Player 2 X axis (0-255)
    LDB $CF02      ; Joy_2_X (unsigned byte)
    CLRA           ; Zero extend to 16-bit
    STD RESULT
    LDD RESULT
    STD VAR_ANALOG_X
    ; J2_ANALOG_Y: Read raw Player 2 Y axis (0-255)
    LDB $CF03      ; Joy_2_Y (unsigned byte)
    CLRA           ; Zero extend to 16-bit
    STD RESULT
    LDD RESULT
    STD VAR_ANALOG_Y
    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
    LDD VAR_PLAYER2_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #5
    STD RESULT
    LDD RESULT
    SUBD ,S++
    STD RESULT
    LDD RESULT
    STD TMPPTR+0    ; x0
    LDD VAR_PLAYER2_Y
    STD RESULT
    LDD RESULT
    STD TMPPTR+2    ; y0
    LDD VAR_PLAYER2_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #5
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD TMPPTR+4    ; x1
    LDD VAR_PLAYER2_Y
    STD RESULT
    LDD RESULT
    STD TMPPTR+6    ; y1
    LDD #127
    STD RESULT
    LDD RESULT
    STD TMPPTR+8    ; intensity
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
    LDD VAR_PLAYER2_X
    STD RESULT
    LDD RESULT
    STD TMPPTR+0    ; x0
    LDD VAR_PLAYER2_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #5
    STD RESULT
    LDD RESULT
    SUBD ,S++
    STD RESULT
    LDD RESULT
    STD TMPPTR+2    ; y0
    LDD VAR_PLAYER2_X
    STD RESULT
    LDD RESULT
    STD TMPPTR+4    ; x1
    LDD VAR_PLAYER2_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #5
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD TMPPTR+6    ; y1
    LDD #127
    STD RESULT
    LDD RESULT
    STD TMPPTR+8    ; intensity
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    RTS

;**** PRINT_TEXT String Data ****
PRINT_TEXT_STR_69863571:
    FCC "J2 UP"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_67138332013:
    FCC "J2 BTN1"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_67138332014:
    FCC "J2 BTN2"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_67138332015:
    FCC "J2 BTN3"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_67138332016:
    FCC "J2 BTN4"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_67138387098:
    FCC "J2 DOWN"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_67138615295:
    FCC "J2 LEFT"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_2081302735108:
    FCC "J2 RIGHT"
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

