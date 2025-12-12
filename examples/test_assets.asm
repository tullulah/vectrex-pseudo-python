; --- Motorola 6809 backend (Vectrex) title='ASSETS TEST' origin=$0000 ---
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
    FCC "ASSETS TEST"
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
    ; VPy_LINE:7
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 7
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
    ; DEBUG: Processing 3 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    ; VPy_LINE:10
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 10
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(6)
    ; VPy_LINE:13
; DRAW_VECTOR("player") - render vector asset
    LDX #_PLAYER_VECTORS
    JSR Draw_VLc
    LDD #0
    STD RESULT
    ; DEBUG: Statement 2 - Discriminant(6)
    ; VPy_LINE:16
; PLAY_MUSIC("theme") - play music asset
    LDX #_THEME_MUSIC
    JSR PLAY_MUSIC_RUNTIME
    LDD #0
    STD RESULT
    RTS

VECTREX_SET_INTENSITY:
    LDA VAR_ARG0+1
    JSR Intensity_a
    RTS
VECTREX_WAIT_RECAL:
    JSR Wait_Recal
    RTS
; PLAY_MUSIC_RUNTIME - Initialize music playback from asset data
; Input: X = pointer to music data structure
; Note: MUSIC_PTR storage defined in RAM variables section
PLAY_MUSIC_RUNTIME:
; Store music pointer for later use
STX MUSIC_PTR
; TODO: Implement PSG initialization and music player
; For now, just store the pointer and return
RTS
;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
MUSIC_PTR  EQU RESULT+26
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "PLAYER"
    FCB $80
STR_1:
    FCC "THEME"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26

; ========================================
; ASSET DATA SECTION
; Embedded .vec and .vmus resources
; ========================================

; Vector asset: player
; Generated from player.vec
; Total paths: 1, points: 3

_PLAYER_SHIP_VECTORS:
    FCB 3              ; num_points
    FCB 127              ; intensity
    FCB 20, 0          ; point 0
    FCB -10, -15          ; point 1
    FCB -10, 15          ; point 2
    FCB $01             ; closed path

_PLAYER_VECTORS:  ; Main alias for DRAW_VECTOR
    ; Redirect to _PLAYER_SHIP_VECTORS
    FCB 3              ; num_points
    FCB 127              ; intensity
    FCB 20, 0          ; point 0
    FCB -10, -15          ; point 1
    FCB -10, 15          ; point 2
    FCB $01             ; closed path


; Music asset: theme (TODO: implement PSG music generation)
_THEME_MUSIC:
    FCB 0 ; Placeholder

