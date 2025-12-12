; --- Motorola 6809 backend (Vectrex) title='Single Line DSL Test' origin=$0000 ---
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
    FCC "SINGLE LINE DSL TEST"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************
    JMP START

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
; Draw_Sync_List - EXACT port of Malban's draw_synced_list_c
; Data: FCB intensity, y_start, x_start, next_y, next_x, [flag, dy, dx]*, 2
; ============================================================================
Draw_Sync_List:
PSHS U,Y,B
LDA #$D0
TFR A,DP
; Read intensity, y_start, x_start
LDA ,X+                 ; intensity
PSHS A                  ; Save intensity
LDB ,X+                 ; y_start
LDA ,X+                 ; x_start
PSHS D                  ; Save y,x
; Reset to zero (Malban resync) - MUST BE BEFORE intensity
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Set intensity AFTER reset
PULS A                  ; Restore intensity
JSR $F2AB               ; BIOS Intensity_a (expects value in A)
; Move to start position
PULS D                  ; Restore y(B), x(A)
STB VIA_port_a          ; y to DAC
PSHS A                  ; Save x
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore x
STA VIA_port_a          ; x to DAC
LDA #$7F
STA VIA_t1_cnt_lo       ; Scale factor (CRITICAL for timing)
CLR VIA_t1_cnt_hi
; Skip next_y, next_x (read and discard)
LEAX 2,X
; Wait for move
DSL_w1:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_w1

; Main draw loop
DSL_loop:
LDA ,X+
CMPA #2
BEQ DSL_done
CMPA #1
BEQ DSL_next_path
TSTA
BMI DSL_draw
; MoveTo (flag=0) - skip for now
LEAX 2,X
BRA DSL_loop
DSL_next_path:
; Next path marker (flag=1) - read new intensity, position, skip next_y/next_x
LDA ,X+                 ; intensity
JSR $F2AB               ; BIOS Intensity_a (expects value in A)
LDB ,X+                 ; y_start
LDA ,X+                 ; x_start
PSHS D                  ; Save y,x
; Full reset sequence (like initial move)
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl            ; Zero integrators
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move to new position
PULS D                  ; Restore y(B), x(A)
STB VIA_port_a
PSHS A
LDA #$CE
STA VIA_cntl            ; Disable zero
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A
STA VIA_port_a
LDA #$7F
STA VIA_t1_cnt_lo       ; Scale factor (CRITICAL for timing)
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
DSL_w3:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_w3
BRA DSL_loop
DSL_draw:
; Draw line (flag<0)
LDB ,X+                 ; dy
LDA ,X+                 ; dx
PSHS A                  ; Save dx
STB VIA_port_a          ; dy to DAC
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore dx
STA VIA_port_a          ; dx to DAC
CLR VIA_t1_cnt_hi
LDA #$FF
STA VIA_shift_reg
DSL_w2:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_w2
CLR VIA_shift_reg
BRA DSL_loop
DSL_done:
PULS B,Y,U,PC
START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
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
    ; DEBUG: Processing 1 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    ; VPy_LINE:7
; DRAW_VECTOR("single") - Using Malban Draw_Sync_List format
; Draw_Sync_List does its own Reset0Ref internally (VIA_cntl=$CC)
    LDA #$7F
    STA VIA_t1_cnt_lo   ; Set scale factor (CRITICAL - inline uses this)
    LDX #_SINGLE_VECTORS  ; X = vector data pointer
    JSR Draw_Sync_List  ; Draw vector list (Malban format)
    LDD #0
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
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "SINGLE"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26

; ========================================
; ASSET DATA SECTION
; Embedded 1 of 6 assets (unused assets excluded)
; ========================================

; Vector asset: single
; Generated from single.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 2

_SINGLE_VECTORS:
    FCB 100              ; path0: intensity
    FCB $00,$00,0,0        ; path0: header (y=0, x=0, next_y=0, next_x=0)
    FCB $FF,$14,$14          ; line 0: flag=-1, dy=20, dx=20
    FCB 2                ; End marker


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

