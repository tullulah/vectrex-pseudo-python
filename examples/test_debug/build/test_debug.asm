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
    FCC "Debug Tools Test"
    FCB $80                 ; String terminator
    FCB 0                   ; End of header

;***************************************************************************
; SYSTEM RAM VARIABLES
;***************************************************************************
CURRENT_ROM_BANK EQU $C880
RESULT EQU $CF00
TMPPTR EQU $CF02
TMPPTR2 EQU $CF04
NUM_STR EQU $CF06   ; 2-byte buffer for PRINT_NUMBER hex output

;***************************************************************************
; USER VARIABLES
;***************************************************************************
VAR_TEST_VALUE EQU $CF10+0
VAR_COUNTER EQU $CF10+2
VAR_MY_STRING EQU $CF10+4

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
    ; TODO: Statement Pass { source_line: 8 }
    RTS

; Function: LOOP
LOOP:
    LDD VAR_COUNTER
    STD RESULT
    ; DEBUG_PRINT(COUNTER)
    LDD RESULT
    STA $C002
    STB $C000
    LDA #$FE
    STA $C001
    LDX #DEBUG_LABEL_COUNTER
    STX $C004
    BRA .DEBUG_0
DEBUG_LABEL_COUNTER:
    FCC "COUNTER"
    FCB $00
.DEBUG_0:
    LDD #0
    STD RESULT
    LDD VAR_TEST_VALUE
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #2
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR MUL16   ; D = X * D
    STD RESULT
    ; DEBUG_PRINT(expression)
    LDD RESULT
    STA $C002
    STB $C000
    LDA #$42
    STA $C001
    CLR $C003
    CLR $C005
    LDD #0
    STD RESULT
    LDD VAR_MY_STRING
    STD RESULT
    ; DEBUG_PRINT_STR(MY_STRING)
    LDD RESULT
    STD $C002
    LDA #$FD
    STA $C001
    LDX #DEBUG_LABEL_MY_STRING
    STX $C004
    BRA .DEBUG_1
DEBUG_LABEL_MY_STRING:
    FCC "MY_STRING"
    FCB $00
.DEBUG_1:
    LDD #0
    STD RESULT
    ; PRINT_NUMBER(x, y, num)
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0    ; X position
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG1    ; Y position
    LDD VAR_COUNTER
    STD RESULT
    LDD RESULT
    STD VAR_ARG2    ; Number value
    JSR VECTREX_PRINT_NUMBER
    LDD #0
    STD RESULT
    ; PRINT_NUMBER(x, y, num)
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0    ; X position
    LDD #30
    STD RESULT
    LDD RESULT
    STD VAR_ARG1    ; Y position
    LDD VAR_TEST_VALUE
    STD RESULT
    LDD RESULT
    STD VAR_ARG2    ; Number value
    JSR VECTREX_PRINT_NUMBER
    LDD #0
    STD RESULT
    ; PRINT_NUMBER(x, y, num)
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0    ; X position
    LDD #10
    STD RESULT
    LDD RESULT
    STD VAR_ARG1    ; Y position
    LDD VAR_COUNTER
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_TEST_VALUE
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_ARG2    ; Number value
    JSR VECTREX_PRINT_NUMBER
    LDD #0
    STD RESULT
    LDD VAR_COUNTER
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_COUNTER
    RTS

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

VECTREX_PRINT_NUMBER:
    ; VPy signature: PRINT_NUMBER(x, y, num)
    ; Convert number to hex string and print
    JSR $F1AA      ; DP_to_D0 - set Direct Page for BIOS/VIA access
    LDA VAR_ARG1+1   ; Y position
    LDB VAR_ARG0+1   ; X position
    JSR Moveto_d     ; Move to position
    
    ; Convert number to string (show low byte as hex)
    LDA VAR_ARG2+1   ; Load number value
    
    ; Convert high nibble to ASCII
    LSRA
    LSRA
    LSRA
    LSRA
    ANDA #$0F
    CMPA #10
    BLO PN_DIGIT1
    ADDA #7          ; A-F
PN_DIGIT1:
    ADDA #'0'
    STA NUM_STR      ; Store first digit
    
    ; Convert low nibble to ASCII  
    LDA VAR_ARG2+1
    ANDA #$0F
    CMPA #10
    BLO PN_DIGIT2
    ADDA #7          ; A-F
PN_DIGIT2:
    ADDA #'0'
    ORA #$80         ; Set high bit for string termination
    STA NUM_STR+1    ; Store second digit with high bit
    
    ; Print the string
    LDU #NUM_STR     ; Point to our number string
    JSR Print_Str_d  ; Print using BIOS
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

