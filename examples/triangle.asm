; --- Motorola 6809 backend (Vectrex) title='TRIANGLE' origin=$0000 ---
        ORG $0000
;***************************************************************************
; DEFINE SECTION
;***************************************************************************
    INCLUDE "../include/VECTREX.I"

;***************************************************************************
; HEADER SECTION
;***************************************************************************
    FCC "g GCE 1982"
    FCB $80
    FDB music1
    FCB $F8,$50,$20,-$45
    FCC "TRIANGLE"
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

MAIN_LOOP:
    JSR Wait_Recal
    LDA #$D0
    TFR A,DP
    JSR Intensity_5F
    JSR Reset0Ref
    JSR MAIN
    BRA MAIN_LOOP

X0 EQU 0
Y0 EQU 0
X1 EQU 40
Y1 EQU 0
X2 EQU 20
Y2 EQU 40
INT EQU 95
DRAW_FRAME: ; function
; --- function draw_frame ---
    LDD VAR_ARG0
    STD VAR_I
    LDD VAR_I
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #32
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ANDA TMPRIGHT+1
    ANDB TMPRIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    LDD #0
    STD RESULT
    BEQ CT_2
    BRA CE_3
CT_2:
    LDD #1
    STD RESULT
CE_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    LDD #48
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_FRAME_BEGIN
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_0
IF_NEXT_1:
    LDD #95
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_FRAME_BEGIN
    CLRA
    CLRB
    STD RESULT
IF_END_0:
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #50
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
    LDD #65476
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65476
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
    LDA #$D0
    TFR A,DP
    LDA #$00
    LDB #$00
    JSR Moveto_d ; move to (0, 0)
    JSR Intensity_5F
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$0A
    JSR Draw_Line_d ; dy=0, dx=10
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$00
    LDB #$00
    JSR Moveto_d ; move to (0, 0)
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$28
    JSR Draw_Line_d ; dy=0, dx=40
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$00
    LDB #$28
    JSR Moveto_d ; move to (40, 0)
    CLR Vec_Misc_Count
    LDA #$28
    LDB #$EC
    JSR Draw_Line_d ; dy=40, dx=-20
    CLRA
    CLRB
    STD RESULT
    LDA #$D0
    TFR A,DP
    LDA #$28
    LDB #$14
    JSR Moveto_d ; move to (20, 40)
    CLR Vec_Misc_Count
    LDA #$D8
    LDB #$EC
    JSR Draw_Line_d ; dy=-40, dx=-20
    CLRA
    CLRB
    STD RESULT
    LDD #0
    STD RESULT
    RTS

MAIN: ; function
; --- function main ---
    LDD VAR_FRAME_COUNTER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_FRAME_COUNTER
    STU TMPPTR
    STX ,U
    LDD VAR_FRAME_COUNTER
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR DRAW_FRAME
    LDD #0
    STD RESULT
    RTS

    INCLUDE "../runtime/vectorlist_runtime.asm"
VECTREX_PRINT_TEXT:
    ; Wait_Recal set DP=$D0 and zeroed beam; just load U,Y,X and call BIOS
    LDU VAR_ARG2   ; string pointer (high-bit terminated)
    LDA VAR_ARG1+1 ; Y
    LDB VAR_ARG0+1 ; X
    JSR Print_Str_d
    RTS
VECTREX_FRAME_BEGIN:
    LDA VAR_ARG0+1
    JSR Intensity_a
    JSR Reset0Ref
    RTS
;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
TMPLEFT   EQU RESULT+2
TMPRIGHT  EQU RESULT+4
TMPPTR    EQU RESULT+6
VAR_FRAME_COUNTER EQU RESULT+26
VAR_I EQU RESULT+28
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "DEMO"
    FCB $80
STR_1:
    FCC "TRI"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+30
VAR_ARG1 EQU RESULT+32
VAR_ARG2 EQU RESULT+34
VAR_ARG3 EQU RESULT+36
VAR_ARG4 EQU RESULT+38
