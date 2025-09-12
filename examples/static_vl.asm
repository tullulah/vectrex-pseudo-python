; --- Motorola 6809 backend (Vectrex) title='STAT' origin=$0000 ---
        ORG $0000
;***************************************************************************
; DEFINE SECTION
;***************************************************************************
; --- BIOS equates (verified against VECTREXR.I) ---
WAIT_RECAL     EQU $F192
INTENSITY_5F   EQU $F2A5
INTENSITY_A    EQU $F2AB ; variable intensity (A)
PRINT_STR_D    EQU $F37A
MOVETO_D       EQU $F312 ; move to absolute coordinate in D (A=Y,B=X)
RESET0REF      EQU $F354 ; reset zero reference
DP_TO_C8       EQU $F1AF ; set DP=$C8 (vectors)
DRAW_VL        EQU $F3DD ; Draw_VL (y x y x ... )
MUSIC1         EQU $FD0D

;***************************************************************************
; HEADER SECTION
;***************************************************************************
    FCC "g GCE 2025"
    FCB $80
    FDB MUSIC1
    FCB $F8 ; height
    FCB $50 ; width
    FCB $20 ; rel y
    FCB $D0 ; rel x (-$30)
    FCC "STAT"
    FCB $80
    FCB 0
    RMB $0030-* ; pad header to $30

;***************************************************************************
; CODE SECTION
;***************************************************************************
; Init then implicit frame loop (auto_loop enabled)
INIT_START: JSR WAIT_RECAL
    JSR INTENSITY_5F
    JSR RESET0REF
    BRA ENTRY_LOOP
ENTRY_LOOP: JSR WAIT_RECAL
    ; lean loop: FRAME_BEGIN wrapper (called by user) sets intensity + RESET0REF.
    JSR MAIN
    BRA ENTRY_LOOP

MAIN: ; function
; --- function main ---
    LDD VAR_COUNTER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
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
    LDD #95
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_FRAME_BEGIN
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_0
IF_NEXT_1:
    LDD #48
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR VECTREX_FRAME_BEGIN
    CLRA
    CLRB
    STD RESULT
IF_END_0:
    JSR VECTREX_DBG_STATIC_VL
    LDD #0
    STD RESULT
    RTS

;***************************************************************************
; RUNTIME SECTION
;***************************************************************************
; --- Vectrex built-in wrappers ---
VECTREX_VECTOR_PHASE_BEGIN:
    ; Cambia a DP=$C8 para rutinas de lista de vectores y recentra.
    JSR DP_TO_C8
    JSR RESET0REF
    RTS
VECTREX_DBG_STATIC_VL:
    ; Draw static debug vector list (one horizontal line).
    JSR DP_TO_C8
    LDU #DBG_STATIC_LIST
    LDA #$5F
    JSR INTENSITY_A
    JSR DRAW_VL
    RTS
DBG_STATIC_LIST:
    FCB $80,$20 ; end bit set, dy=0, dx=32
VECTREX_FRAME_BEGIN:
    LDA VAR_ARG0+1
    JSR INTENSITY_A
    JSR RESET0REF
    RTS
;***************************************************************************
; DATA SECTION
;***************************************************************************
    ORG $C880 ; begin runtime variables in RAM
; Variables (in RAM)
RESULT:   FDB 0
TMPLEFT:  FDB 0
TMPRIGHT: FDB 0
VAR_COUNTER: FDB 0
; Call argument scratch space
VAR_ARG0: FDB 0
