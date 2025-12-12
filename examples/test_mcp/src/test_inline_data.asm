; --- Motorola 6809 backend (Vectrex) title='UNTITLED' origin=$0000 ---
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
    FCC "UNTITLED"
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
; C code does u+=3 after reading intensity,y,x to skip to after next_y,next_x
; We already advanced X by 3 (LDA ,X+ three times), so skip 2 more for next_y,next_x
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
    ; VPy_LINE:2
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 2
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
    ; DEBUG: Processing 63 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    ; VPy_LINE:6
    LDX #VECTOR_DATA
    ; DEBUG: Statement 1 - Discriminant(6)
    ; VPy_LINE:8
    LDA ,X+
    ; DEBUG: Statement 2 - Discriminant(6)
    ; VPy_LINE:9
    JSR $F2AB
    ; DEBUG: Statement 3 - Discriminant(6)
    ; VPy_LINE:11
    LDB ,X+
    ; DEBUG: Statement 4 - Discriminant(6)
    ; VPy_LINE:13
    LDA ,X+
    ; DEBUG: Statement 5 - Discriminant(6)
    ; VPy_LINE:14
    PSHS D
    ; DEBUG: Statement 6 - Discriminant(6)
    ; VPy_LINE:16
    CLR VIA_shift_reg
    ; DEBUG: Statement 7 - Discriminant(6)
    ; VPy_LINE:17
    LDA #$CC
    ; DEBUG: Statement 8 - Discriminant(6)
    ; VPy_LINE:18
    STA VIA_cntl
    ; DEBUG: Statement 9 - Discriminant(6)
    ; VPy_LINE:19
    CLR VIA_port_a
    ; DEBUG: Statement 10 - Discriminant(6)
    ; VPy_LINE:20
    LDA #$82
    ; DEBUG: Statement 11 - Discriminant(6)
    ; VPy_LINE:21
    STA VIA_port_b
    ; DEBUG: Statement 12 - Discriminant(6)
    ; VPy_LINE:22
    NOP
    ; DEBUG: Statement 13 - Discriminant(6)
    ; VPy_LINE:23
    NOP
    ; DEBUG: Statement 14 - Discriminant(6)
    ; VPy_LINE:24
    NOP
    ; DEBUG: Statement 15 - Discriminant(6)
    ; VPy_LINE:25
    NOP
    ; DEBUG: Statement 16 - Discriminant(6)
    ; VPy_LINE:26
    NOP
    ; DEBUG: Statement 17 - Discriminant(6)
    ; VPy_LINE:27
    LDA #$83
    ; DEBUG: Statement 18 - Discriminant(6)
    ; VPy_LINE:28
    STA VIA_port_b
    ; DEBUG: Statement 19 - Discriminant(6)
    ; VPy_LINE:30
    PULS D
    ; DEBUG: Statement 20 - Discriminant(6)
    ; VPy_LINE:31
    STB VIA_port_a
    ; DEBUG: Statement 21 - Discriminant(6)
    ; VPy_LINE:32
    PSHS A
    ; DEBUG: Statement 22 - Discriminant(6)
    ; VPy_LINE:33
    LDA #$CE
    ; DEBUG: Statement 23 - Discriminant(6)
    ; VPy_LINE:34
    STA VIA_cntl
    ; DEBUG: Statement 24 - Discriminant(6)
    ; VPy_LINE:35
    CLR VIA_port_b
    ; DEBUG: Statement 25 - Discriminant(6)
    ; VPy_LINE:36
    LDA #1
    ; DEBUG: Statement 26 - Discriminant(6)
    ; VPy_LINE:37
    STA VIA_port_b
    ; DEBUG: Statement 27 - Discriminant(6)
    ; VPy_LINE:38
    PULS A
    ; DEBUG: Statement 28 - Discriminant(6)
    ; VPy_LINE:39
    STA VIA_port_a
    ; DEBUG: Statement 29 - Discriminant(6)
    ; VPy_LINE:40
    LDA #$7F
    ; DEBUG: Statement 30 - Discriminant(6)
    ; VPy_LINE:41
    STA VIA_t1_cnt_lo
    ; DEBUG: Statement 31 - Discriminant(6)
    ; VPy_LINE:42
    CLR VIA_t1_cnt_hi
    ; DEBUG: Statement 32 - Discriminant(6)
    ; VPy_LINE:44
    LEAX 2,X
    ; DEBUG: Statement 33 - Discriminant(6)
    ; VPy_LINE:46
    W1:
    ; DEBUG: Statement 34 - Discriminant(6)
    ; VPy_LINE:47
    LDA VIA_int_flags
    ; DEBUG: Statement 35 - Discriminant(6)
    ; VPy_LINE:48
    ANDA #$40
    ; DEBUG: Statement 36 - Discriminant(6)
    ; VPy_LINE:49
    BEQ W1
    ; DEBUG: Statement 37 - Discriminant(6)
    ; VPy_LINE:51
    LDA ,X+
    ; DEBUG: Statement 38 - Discriminant(6)
    ; VPy_LINE:53
    TSTA
    ; DEBUG: Statement 39 - Discriminant(6)
    ; VPy_LINE:54
    BPL DONE
    ; DEBUG: Statement 40 - Discriminant(6)
    ; VPy_LINE:56
    LDB ,X+
    ; DEBUG: Statement 41 - Discriminant(6)
    ; VPy_LINE:58
    LDA ,X+
    ; DEBUG: Statement 42 - Discriminant(6)
    ; VPy_LINE:59
    PSHS A
    ; DEBUG: Statement 43 - Discriminant(6)
    ; VPy_LINE:61
    STB VIA_port_a
    ; DEBUG: Statement 44 - Discriminant(6)
    ; VPy_LINE:62
    CLR VIA_port_b
    ; DEBUG: Statement 45 - Discriminant(6)
    ; VPy_LINE:63
    LDA #1
    ; DEBUG: Statement 46 - Discriminant(6)
    ; VPy_LINE:64
    STA VIA_port_b
    ; DEBUG: Statement 47 - Discriminant(6)
    ; VPy_LINE:65
    PULS A
    ; DEBUG: Statement 48 - Discriminant(6)
    ; VPy_LINE:66
    STA VIA_port_a
    ; DEBUG: Statement 49 - Discriminant(6)
    ; VPy_LINE:67
    CLR VIA_t1_cnt_hi
    ; DEBUG: Statement 50 - Discriminant(6)
    ; VPy_LINE:68
    LDA #$FF
    ; DEBUG: Statement 51 - Discriminant(6)
    ; VPy_LINE:69
    STA VIA_shift_reg
    ; DEBUG: Statement 52 - Discriminant(6)
    ; VPy_LINE:70
    W2:
    ; DEBUG: Statement 53 - Discriminant(6)
    ; VPy_LINE:71
    LDA VIA_int_flags
    ; DEBUG: Statement 54 - Discriminant(6)
    ; VPy_LINE:72
    ANDA #$40
    ; DEBUG: Statement 55 - Discriminant(6)
    ; VPy_LINE:73
    BEQ W2
    ; DEBUG: Statement 56 - Discriminant(6)
    ; VPy_LINE:74
    CLR VIA_shift_reg
    ; DEBUG: Statement 57 - Discriminant(6)
    ; VPy_LINE:75
    DONE:
    ; DEBUG: Statement 58 - Discriminant(6)
    ; VPy_LINE:77
    VECTOR_DATA:
    ; DEBUG: Statement 59 - Discriminant(6)
    ; VPy_LINE:78
    FCB 100
    ; DEBUG: Statement 60 - Discriminant(6)
    ; VPy_LINE:79
    FCB 0,0,0,0
    ; DEBUG: Statement 61 - Discriminant(6)
    ; VPy_LINE:80
    FCB $FF,20,20
    ; DEBUG: Statement 62 - Discriminant(6)
    ; VPy_LINE:81
    FCB 2
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
    FCC "ANDA #$40"
    FCB $80
STR_1:
    FCC "BEQ W1"
    FCB $80
STR_2:
    FCC "BEQ W2"
    FCB $80
STR_3:
    FCC "BPL DONE"
    FCB $80
STR_4:
    FCC "CLR VIA_PORT_A"
    FCB $80
STR_5:
    FCC "CLR VIA_PORT_B"
    FCB $80
STR_6:
    FCC "CLR VIA_SHIFT_REG"
    FCB $80
STR_7:
    FCC "CLR VIA_T1_CNT_HI"
    FCB $80
STR_8:
    FCC "DONE:"
    FCB $80
STR_9:
    FCC "FCB $FF,20,20"
    FCB $80
STR_10:
    FCC "FCB 0,0,0,0"
    FCB $80
STR_11:
    FCC "FCB 100"
    FCB $80
STR_12:
    FCC "FCB 2"
    FCB $80
STR_13:
    FCC "JSR $F2AB"
    FCB $80
STR_14:
    FCC "LDA #$7F"
    FCB $80
STR_15:
    FCC "LDA #$82"
    FCB $80
STR_16:
    FCC "LDA #$83"
    FCB $80
STR_17:
    FCC "LDA #$CC"
    FCB $80
STR_18:
    FCC "LDA #$CE"
    FCB $80
STR_19:
    FCC "LDA #$FF"
    FCB $80
STR_20:
    FCC "LDA #1"
    FCB $80
STR_21:
    FCC "LDA ,X+"
    FCB $80
STR_22:
    FCC "LDA VIA_INT_FLAGS"
    FCB $80
STR_23:
    FCC "LDB ,X+"
    FCB $80
STR_24:
    FCC "LDX #VECTOR_DATA"
    FCB $80
STR_25:
    FCC "LEAX 2,X"
    FCB $80
STR_26:
    FCC "NOP"
    FCB $80
STR_27:
    FCC "PSHS A"
    FCB $80
STR_28:
    FCC "PSHS D"
    FCB $80
STR_29:
    FCC "PULS A"
    FCB $80
STR_30:
    FCC "PULS D"
    FCB $80
STR_31:
    FCC "STA VIA_CNTL"
    FCB $80
STR_32:
    FCC "STA VIA_PORT_A"
    FCB $80
STR_33:
    FCC "STA VIA_PORT_B"
    FCB $80
STR_34:
    FCC "STA VIA_SHIFT_REG"
    FCB $80
STR_35:
    FCC "STA VIA_T1_CNT_LO"
    FCB $80
STR_36:
    FCC "STB VIA_PORT_A"
    FCB $80
STR_37:
    FCC "TSTA"
    FCB $80
STR_38:
    FCC "VECTOR_DATA:"
    FCB $80
STR_39:
    FCC "W1:"
    FCB $80
STR_40:
    FCC "W2:"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26

; ========================================
; NO ASSETS EMBEDDED
; All 18 discovered assets are unused in code
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

