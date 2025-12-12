; --- Motorola 6809 backend (Vectrex) title='Malban Vector List' origin=$0000 ---
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
    FCC "MALBAN VECTOR LIST"
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
    ; VPy_LINE:5
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 5
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
    ; DEBUG: Processing 2 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    ; VPy_LINE:8
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 8
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(6)
    ; VPy_LINE:12
    LDD #0
    STD RESULT
    LDA RESULT+1
    STA VL_Y
    LDD #0
    STD RESULT
    LDA RESULT+1
    STA VL_X
    LDD #80
    STD RESULT
    LDA RESULT+1
    STA VL_SCALE
; DRAW_VECTOR_LIST(SQUARE, y, x, scale) - Malban algorithm
    LDX #_SQUARE
    STX VL_PTR
VL_LOOP_START:
    CLR $D05A           ; VIA_shift_reg = 0 (blank beam)
    LDA #$CC
    STA $D00B           ; VIA_cntl = 0xCC (zero integrators)
    CLR $D000           ; VIA_port_a = 0 (reset offset)
    LDA #$82
    STA $D002           ; VIA_port_b = 0x82
    LDA VL_SCALE
    STA $D004           ; VIA_t1_cnt_lo = scale
    LDB #5              ; ZERO_DELAY
VL_DELAY:
    DECB
    BNE VL_DELAY
    LDA #$83
    STA $D002           ; VIA_port_b = 0x83
    LDA VL_Y
    STA $D000           ; VIA_port_a = y
    LDA #$CE
    STA $D00B           ; VIA_cntl = 0xCE (integrator mode)
    CLR $D002           ; VIA_port_b = 0 (mux enable)
    LDA #1
    STA $D002           ; VIA_port_b = 1 (mux disable)
    LDA VL_X
    STA $D000           ; VIA_port_a = x
    CLR $D005           ; VIA_t1_cnt_hi = 0 (start timer)
    LDA VL_SCALE
    STA $D004           ; VIA_t1_cnt_lo = scale
    LDX VL_PTR
    LEAX 3,X
    STX VL_PTR
VL_WAIT_MOVE:
    LDA $D00D           ; VIA_int_flags
    ANDA #$40
    BEQ VL_WAIT_MOVE
VL_PROCESS_LOOP:
    LDX VL_PTR
    LDA ,X              ; Load flag byte (*u)
    TSTA
    BPL VL_CHECK_MOVE   ; If >= 0, not a draw
VL_DRAW:
    LDA 1,X             ; dy
    STA $D000           ; VIA_port_a = dy
    CLR $D002           ; VIA_port_b = 0
    LDA #1
    STA $D002           ; VIA_port_b = 1
    LDA 2,X             ; dx
    STA $D000           ; VIA_port_a = dx
    CLR $D005           ; VIA_t1_cnt_hi = 0
    LDA #$FF
    STA $D05A           ; VIA_shift_reg = 0xFF (beam ON)
VL_WAIT_DRAW:
    LDA $D00D
    ANDA #$40
    BEQ VL_WAIT_DRAW
    CLR $D05A           ; VIA_shift_reg = 0 (beam OFF)
    BRA VL_CONTINUE
VL_CHECK_MOVE:
    TSTA
    BNE VL_CHECK_END    ; If != 0, check for end
    ; MoveTo logic (similar to draw but no beam)
    LDA 1,X             ; dy
    BEQ VL_CHECK_DX
VL_DO_MOVE:
    STA $D000           ; VIA_port_a = dy
    LDA #$CE
    STA $D00B           ; VIA_cntl = 0xCE
    CLR $D002
    LDA #1
    STA $D002
    LDA 2,X             ; dx
    STA $D000
    CLR $D005
VL_WAIT_MOVE2:
    LDA $D00D
    ANDA #$40
    BEQ VL_WAIT_MOVE2
    BRA VL_CONTINUE
VL_CHECK_DX:
    LDA 2,X
    BNE VL_DO_MOVE
    BRA VL_CONTINUE
VL_CHECK_END:
    CMPA #2
    BEQ VL_DONE         ; Exit if *u == 2
VL_CONTINUE:
    LDX VL_PTR
    LEAX 3,X
    STX VL_PTR
    BRA VL_PROCESS_LOOP
VL_DONE:
    ; DO-WHILE check: if more lists, loop to VL_LOOP_START
    ; For single list, we're done
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
    FCC "SQUARE"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
VAR_ARG3 EQU RESULT+32

; ========================================
; NO ASSETS EMBEDDED
; All 5 discovered assets are unused in code
; ========================================


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

