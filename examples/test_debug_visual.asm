; --- Motorola 6809 backend (Vectrex) title='UNTITLED' origin=$0000 ---
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
    FCC "UNTITLED"
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
    ; DEBUG: Processing 20 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    LDX #STR_2
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(6)
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 2 - Discriminant(6)
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65486
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 3 - Discriminant(6)
    LDX #STR_3
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 4 - Discriminant(6)
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 5 - Discriminant(6)
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65486
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 6 - Discriminant(6)
    LDX #STR_4
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 7 - Discriminant(6)
    LDD #65456
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
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 8 - Discriminant(6)
    LDX #STR_6
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 9 - Discriminant(1)
    LDD #30
    STD RESULT
    ; DEBUG: Statement 10 - Discriminant(1)
    LDD #40
    STD RESULT
    ; DEBUG: Statement 11 - Discriminant(6)
    LDX #STR_7
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 12 - Discriminant(6)
    LDX #STR_8
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 13 - Discriminant(6)
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 14 - Discriminant(6)
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 15 - Discriminant(6)
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 16 - Discriminant(6)
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 17 - Discriminant(6)
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 18 - Discriminant(6)
    LDX #STR_5
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 19 - Discriminant(6)
    LDX #STR_1
    STX RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    JSR VECTREX_DEBUG_PRINT_LABELED
    CLRA
    CLRB
    STD RESULT
    RTS

VECTREX_PRINT_TEXT:
    ; Wait_Recal set DP=$D0 and zeroed beam; just load U,Y,X and call BIOS
    LDU VAR_ARG2   ; string pointer (high-bit terminated)
    LDA VAR_ARG1+1 ; Y
    LDB VAR_ARG0+1 ; X
    JSR Print_Str_d
    RTS
VECTREX_DEBUG_PRINT_LABELED:
    ; Debug print with label - emulator intercepts special addresses
    ; First write label marker (0xFE) to indicate labeled output
    LDA #$FE
    STA $DFFF        ; Label marker
    ; Write label string pointer to special address
    LDA VAR_ARG0     ; Label string pointer high byte
    STA $DFFE        ; Label pointer high
    LDA VAR_ARG0+1   ; Label string pointer low byte  
    STA $DFFD        ; Label pointer low
    ; Write value to debug output
    LDA VAR_ARG1+1   ; Load value to debug print
    STA $DFFF        ; Value to debug output
    RTS
VECTREX_MOVE_TO:
    LDA VAR_ARG1+1 ; Y
    LDB VAR_ARG0+1 ; X
    JSR Moveto_d
    ; store new current position
    LDA VAR_ARG0+1
    STA VCUR_X
    LDA VAR_ARG1+1
    STA VCUR_Y
    RTS
; Draw from current (VCUR_X,VCUR_Y) to new (x,y) provided in low bytes VAR_ARG0/1.
; Semántica: igual a MOVE_TO seguido de línea, pero preserva origen previo como punto inicial.
; Limita dx/dy a rango BIOS (-64..63) antes de invocar Draw_Line_d.
VECTREX_DRAW_TO:
    ; Cargar destino (x,y)
    LDA VAR_ARG0+1  ; Xdest en A temporalmente
    STA VLINE_DX    ; reutilizar buffer temporal (bajo) para Xdest
    LDA VAR_ARG1+1  ; Ydest en A
    STA VLINE_DY    ; reutilizar buffer temporal para Ydest
    ; Calcular dx = Xdest - VCUR_X
    LDA VLINE_DX
    SUBA VCUR_X
    STA VLINE_DX
    ; Calcular dy = Ydest - VCUR_Y
    LDA VLINE_DY
    SUBA VCUR_Y
    STA VLINE_DY
    ; Clamp dx
    LDA VLINE_DX
    CMPA #63
    BLE D2_DX_HI_OK
    LDA #63
D2_DX_HI_OK: CMPA #-64
    BGE D2_DX_LO_OK
    LDA #-64
D2_DX_LO_OK: STA VLINE_DX
    ; Clamp dy
    LDA VLINE_DY
    CMPA #63
    BLE D2_DY_HI_OK
    LDA #63
D2_DY_HI_OK: CMPA #-64
    BGE D2_DY_LO_OK
    LDA #-64
D2_DY_LO_OK: STA VLINE_DY
    ; Mover haz al origen previo (VCUR_Y en A, VCUR_X en B)
    LDA VCUR_Y
    LDB VCUR_X
    JSR Moveto_d
    ; Dibujar línea usando deltas (A=dy, B=dx)
    LDA VLINE_DY
    LDB VLINE_DX
    JSR Draw_Line_d
    ; Actualizar posición actual al destino exacto original (no clamped)
    LDA VAR_ARG0+1
    STA VCUR_X
    LDA VAR_ARG1+1
    STA VCUR_Y
    RTS
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
VAR_X EQU $C900+0
VAR_Y EQU $C900+2
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "DEBUG TEST"
    FCB $80
STR_1:
    FCC "FRAME_END"
    FCB $80
STR_2:
    FCC "FRAME_START"
    FCB $80
STR_3:
    FCC "LINE1_DRAWN"
    FCB $80
STR_4:
    FCC "LINE2_DRAWN"
    FCB $80
STR_5:
    FCC "SQUARE_DRAWN"
    FCB $80
STR_6:
    FCC "TEXT_DRAWN"
    FCB $80
STR_7:
    FCC "X_VALUE"
    FCB $80
STR_8:
    FCC "Y_VALUE"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
VCUR_X EQU RESULT+4
VCUR_Y EQU RESULT+5
VLINE_DX EQU RESULT+6
VLINE_DY EQU RESULT+7
VLINE_STEPS EQU RESULT+8
VLINE_LIST EQU RESULT+9
