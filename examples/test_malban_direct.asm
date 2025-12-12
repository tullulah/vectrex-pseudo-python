; --- Motorola 6809 backend (Vectrex) title='MALBAN STYLE' origin=$0000 ---
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
    FCC "MALBAN STYLE"
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
    ; VPy_LINE:4
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 4
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
    ; DEBUG: Processing 5 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    ; VPy_LINE:7
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 7
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(6)
    ; VPy_LINE:11
    ; Inline VIA (Malban)
    LDA #0
    STA $D000           ; VIA_port_a = dy
    CLR $D002           ; VIA_port_b = 0 (mux enable)
    LDA #1
    STA $D002           ; VIA_port_b = 1 (mux disable)
    LDA #80
    STA $D000           ; VIA_port_a = dx
    CLR $D005           ; VIA_t1_cnt_hi = 0 (start)
    LDA #$FF
    STA $D05A           ; VIA_shift_reg = 0xff (beam ON)
DRAW_WAIT_FA10:
    LDA $D00D           ; VIA_int_flags
    ANDA #$40           ; Test timer1
    BEQ DRAW_WAIT_FA10
    CLR $D05A           ; VIA_shift_reg = 0
    ; DEBUG: Statement 2 - Discriminant(6)
    ; VPy_LINE:14
    ; Inline VIA (Malban)
    LDA #80
    STA $D000           ; VIA_port_a = dy
    CLR $D002           ; VIA_port_b = 0 (mux enable)
    LDA #1
    STA $D002           ; VIA_port_b = 1 (mux disable)
    LDA #0
    STA $D000           ; VIA_port_a = dx
    CLR $D005           ; VIA_t1_cnt_hi = 0 (start)
    LDA #$FF
    STA $D05A           ; VIA_shift_reg = 0xff (beam ON)
DRAW_WAIT_05A0:
    LDA $D00D           ; VIA_int_flags
    ANDA #$40           ; Test timer1
    BEQ DRAW_WAIT_05A0
    CLR $D05A           ; VIA_shift_reg = 0
    ; DEBUG: Statement 3 - Discriminant(6)
    ; VPy_LINE:17
    ; Inline VIA (Malban)
    LDA #0
    STA $D000           ; VIA_port_a = dy
    CLR $D002           ; VIA_port_b = 0 (mux enable)
    LDA #1
    STA $D002           ; VIA_port_b = 1 (mux disable)
    LDA #-80
    STA $D000           ; VIA_port_a = dx
    CLR $D005           ; VIA_t1_cnt_hi = 0 (start)
    LDA #$FF
    STA $D05A           ; VIA_shift_reg = 0xff (beam ON)
DRAW_WAIT_05F0:
    LDA $D00D           ; VIA_int_flags
    ANDA #$40           ; Test timer1
    BEQ DRAW_WAIT_05F0
    CLR $D05A           ; VIA_shift_reg = 0
    ; DEBUG: Statement 4 - Discriminant(6)
    ; VPy_LINE:20
    ; Inline VIA (Malban)
    LDA #-80
    STA $D000           ; VIA_port_a = dy
    CLR $D002           ; VIA_port_b = 0 (mux enable)
    LDA #1
    STA $D002           ; VIA_port_b = 1 (mux disable)
    LDA #0
    STA $D000           ; VIA_port_a = dx
    CLR $D005           ; VIA_t1_cnt_hi = 0 (start)
    LDA #$FF
    STA $D05A           ; VIA_shift_reg = 0xff (beam ON)
DRAW_WAIT_FA60:
    LDA $D00D           ; VIA_int_flags
    ANDA #$40           ; Test timer1
    BEQ DRAW_WAIT_FA60
    CLR $D05A           ; VIA_shift_reg = 0
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
MUSIC_PTR     EQU RESULT+26
MUSIC_TICK    EQU RESULT+28   ; 32-bit tick counter
MUSIC_EVENT   EQU RESULT+32   ; Current event pointer
MUSIC_ACTIVE  EQU RESULT+34   ; Playback state (1 byte)
; Call argument scratch space
VAR_ARG0 EQU RESULT+26

; ========================================
; NO ASSETS EMBEDDED
; All 5 discovered assets are unused in code
; ========================================

