; --- Motorola 6809 backend (Vectrex) title='Malban Exact' origin=$0000 ---
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
    FCC "MALBAN EXACT"
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
    ; DEBUG: Processing 33 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    ; VPy_LINE:7
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 7
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(6)
    ; VPy_LINE:10
    CLR $D05A
    ; DEBUG: Statement 2 - Discriminant(6)
    ; VPy_LINE:11
    LDA #$CC
    ; DEBUG: Statement 3 - Discriminant(6)
    ; VPy_LINE:12
    STA $D00B
    ; DEBUG: Statement 4 - Discriminant(6)
    ; VPy_LINE:13
    CLR $D000
    ; DEBUG: Statement 5 - Discriminant(6)
    ; VPy_LINE:14
    LDA #$82
    ; DEBUG: Statement 6 - Discriminant(6)
    ; VPy_LINE:15
    STA $D002
    ; DEBUG: Statement 7 - Discriminant(6)
    ; VPy_LINE:16
    LDA #$7F
    ; DEBUG: Statement 8 - Discriminant(6)
    ; VPy_LINE:17
    STA $D004
    ; DEBUG: Statement 9 - Discriminant(6)
    ; VPy_LINE:20
    LDX #$05
    ; DEBUG: Statement 10 - Discriminant(6)
    ; VPy_LINE:21
    DELAY_LOOP:
    ; DEBUG: Statement 11 - Discriminant(6)
    ; VPy_LINE:22
    LEAX -1,X
    ; DEBUG: Statement 12 - Discriminant(6)
    ; VPy_LINE:23
    BNE DELAY_LOOP
    ; DEBUG: Statement 13 - Discriminant(6)
    ; VPy_LINE:25
    LDA #$83
    ; DEBUG: Statement 14 - Discriminant(6)
    ; VPy_LINE:26
    STA $D002
    ; DEBUG: Statement 15 - Discriminant(6)
    ; VPy_LINE:27
    CLR $D000
    ; DEBUG: Statement 16 - Discriminant(6)
    ; VPy_LINE:28
    LDA #$CE
    ; DEBUG: Statement 17 - Discriminant(6)
    ; VPy_LINE:29
    STA $D00B
    ; DEBUG: Statement 18 - Discriminant(6)
    ; VPy_LINE:30
    CLR $D002
    ; DEBUG: Statement 19 - Discriminant(6)
    ; VPy_LINE:31
    LDA #$01
    ; DEBUG: Statement 20 - Discriminant(6)
    ; VPy_LINE:32
    STA $D002
    ; DEBUG: Statement 21 - Discriminant(6)
    ; VPy_LINE:33
    CLR $D000
    ; DEBUG: Statement 22 - Discriminant(6)
    ; VPy_LINE:34
    CLR $D005
    ; DEBUG: Statement 23 - Discriminant(6)
    ; VPy_LINE:35
    LDA #$7F
    ; DEBUG: Statement 24 - Discriminant(6)
    ; VPy_LINE:36
    STA $D004
    ; DEBUG: Statement 25 - Discriminant(6)
    ; VPy_LINE:39
    MOVE_WAIT:
    ; DEBUG: Statement 26 - Discriminant(6)
    ; VPy_LINE:40
    LDA $D00D
    ; DEBUG: Statement 27 - Discriminant(6)
    ; VPy_LINE:41
    ANDA #$40
    ; DEBUG: Statement 28 - Discriminant(6)
    ; VPy_LINE:42
    BEQ MOVE_WAIT
    ; DEBUG: Statement 29 - Discriminant(6)
    ; VPy_LINE:45
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
    ; DEBUG: Statement 30 - Discriminant(6)
    ; VPy_LINE:46
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
    ; DEBUG: Statement 31 - Discriminant(6)
    ; VPy_LINE:47
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
    ; DEBUG: Statement 32 - Discriminant(6)
    ; VPy_LINE:48
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
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "ANDA #$40"
    FCB $80
STR_1:
    FCC "BEQ MOVE_WAIT"
    FCB $80
STR_2:
    FCC "BNE DELAY_LOOP"
    FCB $80
STR_3:
    FCC "CLR $D000"
    FCB $80
STR_4:
    FCC "CLR $D002"
    FCB $80
STR_5:
    FCC "CLR $D005"
    FCB $80
STR_6:
    FCC "CLR $D05A"
    FCB $80
STR_7:
    FCC "DELAY_LOOP:"
    FCB $80
STR_8:
    FCC "LDA #$01"
    FCB $80
STR_9:
    FCC "LDA #$7F"
    FCB $80
STR_10:
    FCC "LDA #$82"
    FCB $80
STR_11:
    FCC "LDA #$83"
    FCB $80
STR_12:
    FCC "LDA #$CC"
    FCB $80
STR_13:
    FCC "LDA #$CE"
    FCB $80
STR_14:
    FCC "LDA $D00D"
    FCB $80
STR_15:
    FCC "LDX #$05"
    FCB $80
STR_16:
    FCC "LEAX -1,X"
    FCB $80
STR_17:
    FCC "MOVE_WAIT:"
    FCB $80
STR_18:
    FCC "STA $D002"
    FCB $80
STR_19:
    FCC "STA $D004"
    FCB $80
STR_20:
    FCC "STA $D00B"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26

; ========================================
; NO ASSETS EMBEDDED
; All 5 discovered assets are unused in code
; ========================================

