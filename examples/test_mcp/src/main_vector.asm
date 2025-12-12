; --- Motorola 6809 backend (Vectrex) title='ASSETS DEMO' origin=$0000 ---
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
    FDB 1
    FCB $F8
    FCB $50
    FCB $20
    FCB $BB
    FCC "ASSETS DEMO"
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
    ; VPy_LINE:11
    LDD #0
    STD RESULT

MAIN:
    JSR Wait_Recal
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    BRA MAIN

LOOP_BODY:
    ; DEBUG: Processing 2 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    ; VPy_LINE:15
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 15
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(6)
    ; VPy_LINE:18
; DRAW_VECTOR("test") - Using BIOS Draw_VLc directly
    JSR Reset0Ref       ; Reset integrator to center
    LDA #$7F
    STA VIA_t1_cnt_lo   ; Set scale factor
    LDX #_TEST_VECTORS  ; X = vector data pointer
    LDA ,X+             ; A = intensity
    JSR Intensity_a     ; Set intensity
    LDD ,X++            ; D = y,x start position
    JSR Moveto_d_7F     ; Move to start
    LDB ,X+             ; B = line count
    JSR Draw_VLc        ; Draw lines (X points to deltas)
    LDD #0
    STD RESULT
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
    FCC "TEST"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26

; ========================================
; ASSET DATA SECTION
; Embedded 1 of 16 assets (unused assets excluded)
; ========================================

; Vector asset: test
; Generated from test.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 2

_TEST_VECTORS:
    FCB 127              ; seg0: intensity
    FCB $E2,$E2          ; seg0: position (y=-30, x=-30)
    FCB 1              ; seg0: 1 lines
    FCB $3C,$3C          ; line 0 delta (dy=60, dx=60)

