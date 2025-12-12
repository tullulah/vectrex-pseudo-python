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
    FDB $0000
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
    ; VPy_LINE:10
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 10
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
    ; DEBUG: Processing 19 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(1)
    ; VPy_LINE:16
    LDD #0
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(1)
    ; VPy_LINE:17
    LDD #0
    STD RESULT
    ; DEBUG: Statement 2 - Discriminant(6)
    ; VPy_LINE:20
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 20
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 3 - Discriminant(6)
    ; VPy_LINE:21
; NATIVE_CALL: VECTREX_SET_ORIGIN at line 21
    JSR VECTREX_SET_ORIGIN
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 4 - Discriminant(6)
    ; VPy_LINE:24
    LDD #255
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 24
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 5 - Discriminant(6)
    ; VPy_LINE:25
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_MOVE_TO at line 25
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 6 - Discriminant(6)
    ; VPy_LINE:26
; DRAW_VECTOR("player") - Malban Draw_Sync_List format
    JSR Reset0Ref       ; Reset integrator to center
    LDA #$7F
    STA VIA_t1_cnt_lo   ; Set scale factor
    LDA #$D0
    TFR A,DP            ; Set DP for hardware
    LDX #_PLAYER_VECTORS  ; X = sync list pointer
DSL_LOOP_1:
    LDA ,X+             ; A = intensity/marker
    CMPA #2
    BEQ DSL_DONE_1            ; End if marker=2
    CMPA #1
    BEQ DSL_LOOP_1            ; Next segment if marker=1
    JSR Intensity_a     ; Set intensity
    LDD ,X++            ; D = y,x position
    JSR Moveto_d_7F     ; Move to position
DSL_LOOP_1_INNER:
    LDA ,X+             ; A = draw marker
    BPL DSL_CHECK       ; Branch if >= 0 (move or break)
    LDD ,X++            ; D = dy,dx
    JSR Draw_Line_d     ; Draw with intensity
    BRA DSL_LOOP_1_INNER            ; Continue inner loop
DSL_CHECK:
    BNE DSL_NEXT_SEG    ; If A!=0, next segment
    LDD ,X++            ; D = dy,dx
    JSR Moveto_d_7F     ; Move beam
    BRA DSL_LOOP_1_INNER            ; Continue inner loop
DSL_NEXT_SEG:
    BRA DSL_LOOP_1            ; Back to outer loop
DSL_DONE_1:
    LDD #0
    STD RESULT
    ; DEBUG: Statement 7 - Discriminant(6)
    ; VPy_LINE:29
    LDD #200
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 29
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 8 - Discriminant(6)
    ; VPy_LINE:30
    LDD #65476
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_MOVE_TO at line 30
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 9 - Discriminant(6)
    ; VPy_LINE:31
; DRAW_VECTOR("enemy") - Malban Draw_Sync_List format
    JSR Reset0Ref       ; Reset integrator to center
    LDA #$7F
    STA VIA_t1_cnt_lo   ; Set scale factor
    LDA #$D0
    TFR A,DP            ; Set DP for hardware
    LDX #_ENEMY_VECTORS  ; X = sync list pointer
DSL_LOOP_2:
    LDA ,X+             ; A = intensity/marker
    CMPA #2
    BEQ DSL_DONE_2            ; End if marker=2
    CMPA #1
    BEQ DSL_LOOP_2            ; Next segment if marker=1
    JSR Intensity_a     ; Set intensity
    LDD ,X++            ; D = y,x position
    JSR Moveto_d_7F     ; Move to position
DSL_LOOP_2_INNER:
    LDA ,X+             ; A = draw marker
    BPL DSL_CHECK       ; Branch if >= 0 (move or break)
    LDD ,X++            ; D = dy,dx
    JSR Draw_Line_d     ; Draw with intensity
    BRA DSL_LOOP_2_INNER            ; Continue inner loop
DSL_CHECK:
    BNE DSL_NEXT_SEG    ; If A!=0, next segment
    LDD ,X++            ; D = dy,dx
    JSR Moveto_d_7F     ; Move beam
    BRA DSL_LOOP_2_INNER            ; Continue inner loop
DSL_NEXT_SEG:
    BRA DSL_LOOP_2            ; Back to outer loop
DSL_DONE_2:
    LDD #0
    STD RESULT
    ; DEBUG: Statement 10 - Discriminant(6)
    ; VPy_LINE:33
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_MOVE_TO at line 33
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 11 - Discriminant(6)
    ; VPy_LINE:34
; DRAW_VECTOR("enemy") - Malban Draw_Sync_List format
    JSR Reset0Ref       ; Reset integrator to center
    LDA #$7F
    STA VIA_t1_cnt_lo   ; Set scale factor
    LDA #$D0
    TFR A,DP            ; Set DP for hardware
    LDX #_ENEMY_VECTORS  ; X = sync list pointer
DSL_LOOP_3:
    LDA ,X+             ; A = intensity/marker
    CMPA #2
    BEQ DSL_DONE_3            ; End if marker=2
    CMPA #1
    BEQ DSL_LOOP_3            ; Next segment if marker=1
    JSR Intensity_a     ; Set intensity
    LDD ,X++            ; D = y,x position
    JSR Moveto_d_7F     ; Move to position
DSL_LOOP_3_INNER:
    LDA ,X+             ; A = draw marker
    BPL DSL_CHECK       ; Branch if >= 0 (move or break)
    LDD ,X++            ; D = dy,dx
    JSR Draw_Line_d     ; Draw with intensity
    BRA DSL_LOOP_3_INNER            ; Continue inner loop
DSL_CHECK:
    BNE DSL_NEXT_SEG    ; If A!=0, next segment
    LDD ,X++            ; D = dy,dx
    JSR Moveto_d_7F     ; Move beam
    BRA DSL_LOOP_3_INNER            ; Continue inner loop
DSL_NEXT_SEG:
    BRA DSL_LOOP_3            ; Back to outer loop
DSL_DONE_3:
    LDD #0
    STD RESULT
    ; DEBUG: Statement 12 - Discriminant(6)
    ; VPy_LINE:37
    LDD #180
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 37
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 13 - Discriminant(6)
    ; VPy_LINE:38
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65506
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_MOVE_TO at line 38
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 14 - Discriminant(6)
    ; VPy_LINE:39
; DRAW_VECTOR("bullet") - Malban Draw_Sync_List format
    JSR Reset0Ref       ; Reset integrator to center
    LDA #$7F
    STA VIA_t1_cnt_lo   ; Set scale factor
    LDA #$D0
    TFR A,DP            ; Set DP for hardware
    LDX #_BULLET_VECTORS  ; X = sync list pointer
DSL_LOOP_4:
    LDA ,X+             ; A = intensity/marker
    CMPA #2
    BEQ DSL_DONE_4            ; End if marker=2
    CMPA #1
    BEQ DSL_LOOP_4            ; Next segment if marker=1
    JSR Intensity_a     ; Set intensity
    LDD ,X++            ; D = y,x position
    JSR Moveto_d_7F     ; Move to position
DSL_LOOP_4_INNER:
    LDA ,X+             ; A = draw marker
    BPL DSL_CHECK       ; Branch if >= 0 (move or break)
    LDD ,X++            ; D = dy,dx
    JSR Draw_Line_d     ; Draw with intensity
    BRA DSL_LOOP_4_INNER            ; Continue inner loop
DSL_CHECK:
    BNE DSL_NEXT_SEG    ; If A!=0, next segment
    LDD ,X++            ; D = dy,dx
    JSR Moveto_d_7F     ; Move beam
    BRA DSL_LOOP_4_INNER            ; Continue inner loop
DSL_NEXT_SEG:
    BRA DSL_LOOP_4            ; Back to outer loop
DSL_DONE_4:
    LDD #0
    STD RESULT
    ; DEBUG: Statement 15 - Discriminant(6)
    ; VPy_LINE:42
    LDD #150
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 42
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 16 - Discriminant(6)
    ; VPy_LINE:43
    LDD #65476
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #90
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_0
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 43
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 17 - Discriminant(6)
    ; VPy_LINE:44
    LDD #65476
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_2
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 44
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 18 - Discriminant(6)
    ; VPy_LINE:45
    LDD #65476
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65446
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_1
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 45
    JSR VECTREX_PRINT_TEXT
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
VECTREX_SET_ORIGIN:
    JSR Reset0Ref
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
VAR_PLAYER_X EQU $CF00+0
VAR_PLAYER_Y EQU $CF00+2
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "ASSETS DEMO"
    FCB $80
STR_1:
    FCC "MUSIC: THEME.VMUS"
    FCB $80
STR_2:
    FCC "PLAYER - ENEMIES - BULLET"
    FCB $80
STR_3:
    FCC "BULLET"
    FCB $80
STR_4:
    FCC "ENEMY"
    FCB $80
STR_5:
    FCC "PLAYER"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
VCUR_X EQU RESULT+4
VCUR_Y EQU RESULT+5

; ========================================
; ASSET DATA SECTION
; Embedded 3 of 5 assets (unused assets excluded)
; ========================================

; Vector asset: player
; Generated from player.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 3

_PLAYER_VECTORS:
    FCB 127              ; seg0: intensity
    FCB $14,$00          ; seg0: position (y=20, x=0)
    FCB -1              ; draw line 0
    FCB $E2,$F1          ; delta (dy=-30, dx=-15)
    FCB -1              ; draw line 1
    FCB $00,$1E          ; delta (dy=0, dx=30)
    FCB 2               ; end of list

; Vector asset: bullet
; Generated from bullet.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 4

_BULLET_VECTORS:
    FCB 127              ; seg0: intensity
    FCB $02,$FE          ; seg0: position (y=2, x=-2)
    FCB -1              ; draw line 0
    FCB $00,$04          ; delta (dy=0, dx=4)
    FCB -1              ; draw line 1
    FCB $FC,$00          ; delta (dy=-4, dx=0)
    FCB -1              ; draw line 2
    FCB $00,$FC          ; delta (dy=0, dx=-4)
    FCB 2               ; end of list

; Vector asset: enemy
; Generated from enemy.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 4

_ENEMY_VECTORS:
    FCB 127              ; seg0: intensity
    FCB $0A,$F6          ; seg0: position (y=10, x=-10)
    FCB -1              ; draw line 0
    FCB $00,$14          ; delta (dy=0, dx=20)
    FCB -1              ; draw line 1
    FCB $EC,$00          ; delta (dy=-20, dx=0)
    FCB -1              ; draw line 2
    FCB $00,$EC          ; delta (dy=0, dx=-20)
    FCB 2               ; end of list

