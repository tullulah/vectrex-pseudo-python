; --- Motorola 6809 backend (Vectrex) title='ENEMY DEMO' origin=$0000 ---
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
    FCC "ENEMY DEMO"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************
    JMP START

START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:6
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
    ; DEBUG: Processing 4 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    ; VPy_LINE:9
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 9
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(6)
    ; VPy_LINE:12
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 12
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 2 - Discriminant(6)
    ; VPy_LINE:13
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_MOVE_TO at line 13
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 3 - Discriminant(6)
    ; VPy_LINE:14
; DRAW_VECTOR("enemigo") - Using BIOS Draw_VLc directly
    JSR Reset0Ref       ; Reset integrator to center
    LDA #$7F
    STA VIA_t1_cnt_lo   ; Set scale factor
    LDX #_ENEMIGO_VECTORS  ; X = vector data pointer
    LDA ,X+             ; A = intensity
    JSR Intensity_a     ; Set intensity
    LDD ,X++            ; D = y,x start position
    JSR Moveto_d_7F     ; Move to start
    LDB ,X+             ; B = line count
    JSR Draw_VLc        ; Draw lines (X points to deltas)
    LDD #0
    STD RESULT
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
VECTREX_SET_INTENSITY:
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDA VAR_ARG0+1
    JSR Intensity_a
    RTS
VECTREX_WAIT_RECAL:
    JSR Wait_Recal
    RTS
; BIOS Wrappers - VIDE compatible (ensure DP=$D0 per call)
__Intensity_a:
TFR B,A         ; Move B to A (BIOS expects intensity in A)
JMP Intensity_a ; JMP (not JSR) - BIOS returns to original caller
__Reset0Ref:
JMP Reset0Ref   ; JMP (not JSR) - BIOS returns to original caller
__Moveto_d:
LDA 2,S         ; Get Y from stack (after return address)
JMP Moveto_d    ; JMP (not JSR) - BIOS returns to original caller
__Draw_Line_d:
LDA 2,S         ; Get dy from stack (after return address)
JMP Draw_Line_d ; JMP (not JSR) - BIOS returns to original caller
;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
MUSIC_PTR     EQU RESULT+26
MUSIC_TICK    EQU RESULT+28   ; 32-bit tick counter
MUSIC_EVENT   EQU RESULT+32   ; Current event pointer
MUSIC_ACTIVE  EQU RESULT+34   ; Playback state (1 byte)
VL_PTR     EQU $CF80      ; Current position in vector list
VL_Y       EQU $CF82      ; Y position (1 byte)
VL_X       EQU $CF83      ; X position (1 byte)
VL_SCALE   EQU $CF84      ; Scale factor (1 byte)
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "ENEMIGO"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VCUR_X EQU RESULT+0
VCUR_Y EQU RESULT+1

; ========================================
; ASSET DATA SECTION
; Embedded 1 of 16 assets (unused assets excluded)
; ========================================

; Vector asset: enemigo
; Generated from enemigo.vec (Malban Draw_Sync_List format)
; Total paths: 9, points: 34

_ENEMIGO_VECTORS:
    FCB 100              ; seg0: intensity
    FCB $FD,$F8          ; seg0: position (y=-3, x=-8)
    FCB 7              ; seg0: 7 lines
    FCB $FB,$02          ; line 0 delta (dy=-5, dx=2)
    FCB $00,$0C          ; line 1 delta (dy=0, dx=12)
    FCB $05,$02          ; line 2 delta (dy=5, dx=2)
    FCB $08,$00          ; line 3 delta (dy=8, dx=0)
    FCB $03,$FE          ; line 4 delta (dy=3, dx=-2)
    FCB $00,$F4          ; line 5 delta (dy=0, dx=-12)
    FCB $FD,$FE          ; line 6 delta (dy=-3, dx=-2)


; ========================================
; VECTOR LIST DATA (Malban format)
; ========================================
_SQUARE:
    FCB 0, 0, 0          ; Header (y=0, x=0, next_y=0)
    FCB $FF, $D8, $D8    ; Line 1: flag=-1, dy=-40, dx=-40
    FCB $FF, 0, 80       ; Line 2: flag=-1, dy=0, dx=80
    FCB $FF, 80, 0       ; Line 3: flag=-1, dy=80, dx=0
    FCB $FF, 0, $B0      ; Line 4: flag=-1, dy=0, dx=-80
    FCB 2                ; End marker

