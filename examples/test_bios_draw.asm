; --- Motorola 6809 backend (Vectrex) title='BIOS DRAW TEST' origin=$0000 ---
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
    FCC "BIOS DRAW TEST"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************
    JMP START

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
; Deltas pueden ser ±127 (hardware Vectrex soporta rango completo).
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
    ; No clamping needed - signed byte arithmetic handles ±127 correctly
    ; Mover haz al origen previo (VCUR_Y en A, VCUR_X en B)
    LDA VCUR_Y
    LDB VCUR_X
    JSR Moveto_d
    ; Dibujar línea usando deltas (A=dy, B=dx)
    LDA VLINE_DY
    LDB VLINE_DX
    JSR Draw_Line_d
    ; Actualizar posición actual al destino exacto original
    LDA VAR_ARG0+1
    STA VCUR_X
    LDA VAR_ARG1+1
    STA VCUR_Y
    RTS
VECTREX_SET_INTENSITY:
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDA VAR_ARG0+1
    JSR __Intensity_a
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
; ============================================================================
; Draw_Sync_List - Malban format vector list renderer
; Input: X = pointer to vector data
; Format: FCB intensity, y, x, next_y, [FCB flag, dy, dx]*, FCB 2
; Direct port from Malban's draw_synced_list_c (vectrex.h)
; ============================================================================
Draw_Sync_List:
PSHS U,Y,X,B            ; Save registers (keep A free)

; Ensure DP=$D0 for VIA direct page addressing
LDA #$D0
TFR A,DP

; Load and set intensity
LDA ,X+                 ; A = intensity
JSR __Intensity_a

; Load start position
LDB ,X+                 ; B = y
LDA ,X+                 ; A = x  
LEAX 1,X                ; Skip next_y byte
PSHS D                  ; Save y,x for later (B=y on stack+1, A=x on stack)

; === RESET TO ZERO (Malban's resync/startsync) ===
CLR VIA_shift_reg       ; VIA_shift_reg = 0 (all output BLANK)
LDA #$CC
STA VIA_cntl            ; VIA_cntl = 0xCC (zero the integrators)
CLR VIA_port_a          ; VIA_port_a = 0 (reset integrator offset)
LDA #$82                ; 0b10000010
STA VIA_port_b          ; VIA_port_b = 0x82

; VIA_t1_cnt_lo = scaleMove (already set by caller)

; Delay till beam is at zero (ZERO_DELAY = 5 cycles minimum)
NOP
NOP
NOP
NOP
NOP

LDA #$83                ; 0b10000011
STA VIA_port_b          ; VIA_port_b = 0x83

; === MOVE TO START POSITION ===
; Malban: VIA_port_a = y; VIA_cntl = 0xCE; VIA_port_b = 0; VIA_port_b = 1; VIA_port_a = x;
PULS D                  ; Restore y,x (B=y, A=x)
STB VIA_port_a          ; VIA_port_a = y
PSHS A                  ; Save x temporarily
LDA #$CE
STA VIA_cntl            ; VIA_cntl = 0xCE (disable zero, disable blank)
CLR VIA_port_b          ; VIA_port_b = 0 (mux enable)
LDA #1
STA VIA_port_b          ; VIA_port_b = 1 (mux disable)
PULS A                  ; Restore x
STA VIA_port_a          ; VIA_port_a = x
CLR VIA_t1_cnt_hi       ; VIA_t1_cnt_hi = 0 (start timer)

; VIA_t1_cnt_lo = scaleList (use same scale)
; u += 3 (already done: X points to first flag byte)

; Wait for move timer
DSL_wait_move:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_wait_move

; === MAIN DRAW LOOP ===
DSL_loop:
LDA ,X+                 ; Load flag byte
CMPA #2                 ; if (*u == 2) break
BEQ DSL_done

TSTA                    ; Check if negative (*u < 0)
BMI DSL_draw            ; if (*u < 0) draw line

; else if (*u == 0) MoveTo
TSTA
BNE DSL_loop            ; if (*u > 0) invalid, skip

; flag=0: Internal MoveTo
LDB ,X+                 ; B = dy (*(u+1))
LDA ,X+                 ; A = dx (*(u+2))

; Malban: if ((*(u+1)!=0) || (*(u+2)!=0))
TSTB
BNE DSL_do_move
TSTA
BEQ DSL_loop            ; Both zero, skip

DSL_do_move:
; Internal moveTo sequence
STB VIA_port_a          ; VIA_port_a = dy
PSHS A                  ; Save dx
LDA #$CE
STA VIA_cntl            ; VIA_cntl = 0xCE
CLR VIA_port_b          ; VIA_port_b = 0
LDA #1
STA VIA_port_b          ; VIA_port_b = 1
PULS A                  ; Restore dx
STA VIA_port_a          ; VIA_port_a = dx
CLR VIA_t1_cnt_hi       ; Start timer
DSL_wait_move2:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_wait_move2
BRA DSL_loop

DSL_draw:
; Draw vector (beam ON)
LDB ,X+                 ; B = dy (*(1+u))
LDA ,X+                 ; A = dx (*(2+u))
STB VIA_port_a          ; VIA_port_a = dy
PSHS A                  ; Save dx
CLR VIA_port_b          ; VIA_port_b = 0
LDA #1
STA VIA_port_b          ; VIA_port_b = 1
PULS A                  ; Restore dx
STA VIA_port_a          ; VIA_port_a = dx
CLR VIA_t1_cnt_hi       ; Start timer
LDA #$FF
STA VIA_shift_reg       ; VIA_shift_reg = 0xFF (beam ON)
DSL_wait_draw:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_wait_draw
CLR VIA_shift_reg       ; VIA_shift_reg = 0 (beam OFF)
BRA DSL_loop

DSL_done:
PULS B,X,Y,U,PC         ; Restore and return
START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:5
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
    ; DEBUG: Processing 3 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    ; VPy_LINE:8
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 8
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(6)
    ; VPy_LINE:9
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_MOVE_TO at line 9
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 2 - Discriminant(6)
    ; VPy_LINE:10
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_DRAW_TO at line 10
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
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
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VCUR_X EQU RESULT+0
VCUR_Y EQU RESULT+1
VLINE_DX EQU RESULT+2
VLINE_DY EQU RESULT+3
VLINE_STEPS EQU RESULT+4
VLINE_LIST EQU RESULT+5

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

