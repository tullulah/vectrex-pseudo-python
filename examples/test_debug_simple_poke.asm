; --- Motorola 6809 backend (Vectrex) title='TEST_DEBUG_SIMPLE_POKE' origin=$0000 ---
        ORG $0000
;***************************************************************************
; DEFINE SECTION
;***************************************************************************
    INCLUDE "include/VECTREX.I"

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
    FCC "TEST DEBUG SIMPLE POKE"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************
    JMP START

START:
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S

    ; *** DEBUG *** main() function code inline (initialization)
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT

MAIN:
    JSR Wait_Recal
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    BRA MAIN

LOOP_BODY:
    ; DEBUG: Processing 9 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    LDD #42
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_DEBUG_PRINT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(6)
    LDD #52992
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #99
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_POKE
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 2 - Discriminant(6)
    LDD #52993
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #102
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_POKE
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 3 - Discriminant(1)
    LDD #52992
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_PEEK
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 4 - Discriminant(1)
    LDD #52993
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_PEEK
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 5 - Discriminant(6)
    LDD #65516
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_4
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 6 - Discriminant(6)
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_VALUE
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_NUMBER
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 7 - Discriminant(7)
    LDD VAR_VALUE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #99
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
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65516
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_3
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_0
IF_NEXT_1:
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65516
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_2
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
IF_END_0:
    ; DEBUG: Statement 8 - Discriminant(7)
    LDD VAR_MARKER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #102
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
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_1
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_4
IF_NEXT_5:
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_0
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
IF_END_4:
    RTS

VECTREX_PRINT_TEXT:
    ; Wait_Recal set DP=$D0 and zeroed beam; just load U,Y,X and call BIOS
    LDU VAR_ARG2   ; string pointer (high-bit terminated)
    LDA VAR_ARG1+1 ; Y
    LDB VAR_ARG0+1 ; X
    JSR Print_Str_d
    RTS
VECTREX_DEBUG_PRINT:
    ; Debug print to console - writes to pseudo debug area (illegal memory region)
    LDA VAR_ARG0+1   ; Load value to debug print
    STA $D800        ; Debug output value in pseudo debug area
    LDA #$42         ; Debug marker
    STA $D801        ; Debug marker to indicate new output
    RTS
VECTREX_POKE:
    ; Write byte to memory address
    ; ARG0 = address (16-bit), ARG1 = value (8-bit)
    LDX VAR_ARG0     ; Load address into X
    LDA VAR_ARG1+1   ; Load value (low byte)
    STA ,X           ; Store value to address
    RTS
VECTREX_PEEK:
    ; Read byte from memory address
    ; ARG0 = address (16-bit), returns value in VAR_ARG0+1
    LDX VAR_ARG0     ; Load address into X
    LDA ,X           ; Load value from address
    STA VAR_ARG0+1   ; Store result in low byte of ARG0
    RTS
VECTREX_PRINT_NUMBER:
    ; Print number at position
    ; ARG0 = X position, ARG1 = Y position, ARG2 = number value
    ; Simple implementation: convert number to string and print
    LDA VAR_ARG1+1   ; Y position
    LDB VAR_ARG0+1   ; X position
    JSR Moveto_d     ; Move to position
    
    ; Convert number to string (simple: just show low byte as hex)
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
    RTS

NUM_STR: RMB 2      ; Space for 2-digit hex number
VECTREX_SET_INTENSITY:
    LDA VAR_ARG0+1
    JSR Intensity_a
    RTS
VECTREX_WAIT_RECAL:
    JSR Wait_Recal
    RTS
;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
TMPLEFT   EQU RESULT+2
TMPRIGHT  EQU RESULT+4
VAR_MARKER EQU $D800+0
VAR_VALUE EQU $D800+2
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "MRK BAD"
    FCB $80
STR_1:
    FCC "MRK OK"
    FCB $80
STR_2:
    FCC "VAL BAD"
    FCB $80
STR_3:
    FCC "VAL OK"
    FCB $80
STR_4:
    FCC "VAL="
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
