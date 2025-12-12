; --- Motorola 6809 backend (Vectrex) title='INLINE MALBAN TEST' origin=$0000 ---
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
    FCC "INLINE MALBAN TEST"
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
PSHS U,Y,B              ; Save
LDA #$D0
TFR A,DP
LDA ,X+
JSR __Intensity_a
LDB ,X+
LDA ,X+
STB 1,S
STA ,S
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
LDB 1,S
STB VIA_port_a
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
LDA ,S
STA VIA_port_a
CLR VIA_t1_cnt_hi
LDB ,X+
LDA ,X+
TSTB
BNE DSL_next
TSTA
BEQ DSL_w1
DSL_next:
STB 1,S
STA ,S
DSL_w1:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_w1
LDB 1,S
BEQ DSL_loop
STB VIA_port_a
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
LDA ,S
STA VIA_port_a
CLR VIA_t1_cnt_hi
DSL_w2:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_w2

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
    ; DEBUG: Processing 88 statements in loop() body
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
    LDA #127
    ; DEBUG: Statement 2 - Discriminant(6)
    ; VPy_LINE:10
    JSR Intensity_a
    ; DEBUG: Statement 3 - Discriminant(6)
    ; VPy_LINE:11
    CLR VIA_shift_reg
    ; DEBUG: Statement 4 - Discriminant(6)
    ; VPy_LINE:12
    LDA #$CC
    ; DEBUG: Statement 5 - Discriminant(6)
    ; VPy_LINE:13
    STA VIA_cntl
    ; DEBUG: Statement 6 - Discriminant(6)
    ; VPy_LINE:14
    CLR VIA_port_a
    ; DEBUG: Statement 7 - Discriminant(6)
    ; VPy_LINE:15
    LDA #$82
    ; DEBUG: Statement 8 - Discriminant(6)
    ; VPy_LINE:16
    STA VIA_port_b
    ; DEBUG: Statement 9 - Discriminant(6)
    ; VPy_LINE:17
    NOP
    ; DEBUG: Statement 10 - Discriminant(6)
    ; VPy_LINE:18
    NOP
    ; DEBUG: Statement 11 - Discriminant(6)
    ; VPy_LINE:19
    NOP
    ; DEBUG: Statement 12 - Discriminant(6)
    ; VPy_LINE:20
    NOP
    ; DEBUG: Statement 13 - Discriminant(6)
    ; VPy_LINE:21
    NOP
    ; DEBUG: Statement 14 - Discriminant(6)
    ; VPy_LINE:22
    LDA #$83
    ; DEBUG: Statement 15 - Discriminant(6)
    ; VPy_LINE:23
    STA VIA_port_b
    ; DEBUG: Statement 16 - Discriminant(6)
    ; VPy_LINE:24
    LDA #$EC
    ; DEBUG: Statement 17 - Discriminant(6)
    ; VPy_LINE:25
    STA VIA_port_a
    ; DEBUG: Statement 18 - Discriminant(6)
    ; VPy_LINE:26
    LDA #$CE
    ; DEBUG: Statement 19 - Discriminant(6)
    ; VPy_LINE:27
    STA VIA_cntl
    ; DEBUG: Statement 20 - Discriminant(6)
    ; VPy_LINE:28
    CLR VIA_port_b
    ; DEBUG: Statement 21 - Discriminant(6)
    ; VPy_LINE:29
    LDA #1
    ; DEBUG: Statement 22 - Discriminant(6)
    ; VPy_LINE:30
    STA VIA_port_b
    ; DEBUG: Statement 23 - Discriminant(6)
    ; VPy_LINE:31
    LDA #$EC
    ; DEBUG: Statement 24 - Discriminant(6)
    ; VPy_LINE:32
    STA VIA_port_a
    ; DEBUG: Statement 25 - Discriminant(6)
    ; VPy_LINE:33
    LDA #$7F
    ; DEBUG: Statement 26 - Discriminant(6)
    ; VPy_LINE:34
    STA VIA_t1_cnt_lo
    ; DEBUG: Statement 27 - Discriminant(6)
    ; VPy_LINE:35
    CLR VIA_t1_cnt_hi
    ; DEBUG: Statement 28 - Discriminant(6)
    ; VPy_LINE:36
    WAIT1:
    ; DEBUG: Statement 29 - Discriminant(6)
    ; VPy_LINE:37
    LDA VIA_int_flags
    ; DEBUG: Statement 30 - Discriminant(6)
    ; VPy_LINE:38
    ANDA #$40
    ; DEBUG: Statement 31 - Discriminant(6)
    ; VPy_LINE:39
    BEQ WAIT1
    ; DEBUG: Statement 32 - Discriminant(6)
    ; VPy_LINE:40
    LDA #40
    ; DEBUG: Statement 33 - Discriminant(6)
    ; VPy_LINE:41
    STA VIA_port_a
    ; DEBUG: Statement 34 - Discriminant(6)
    ; VPy_LINE:42
    CLR VIA_port_b
    ; DEBUG: Statement 35 - Discriminant(6)
    ; VPy_LINE:43
    LDA #1
    ; DEBUG: Statement 36 - Discriminant(6)
    ; VPy_LINE:44
    STA VIA_port_b
    ; DEBUG: Statement 37 - Discriminant(6)
    ; VPy_LINE:45
    CLR VIA_port_a
    ; DEBUG: Statement 38 - Discriminant(6)
    ; VPy_LINE:46
    CLR VIA_t1_cnt_hi
    ; DEBUG: Statement 39 - Discriminant(6)
    ; VPy_LINE:47
    LDA #$FF
    ; DEBUG: Statement 40 - Discriminant(6)
    ; VPy_LINE:48
    STA VIA_shift_reg
    ; DEBUG: Statement 41 - Discriminant(6)
    ; VPy_LINE:49
    WAIT2:
    ; DEBUG: Statement 42 - Discriminant(6)
    ; VPy_LINE:50
    LDA VIA_int_flags
    ; DEBUG: Statement 43 - Discriminant(6)
    ; VPy_LINE:51
    ANDA #$40
    ; DEBUG: Statement 44 - Discriminant(6)
    ; VPy_LINE:52
    BEQ WAIT2
    ; DEBUG: Statement 45 - Discriminant(6)
    ; VPy_LINE:53
    CLR VIA_shift_reg
    ; DEBUG: Statement 46 - Discriminant(6)
    ; VPy_LINE:54
    CLR VIA_port_a
    ; DEBUG: Statement 47 - Discriminant(6)
    ; VPy_LINE:55
    CLR VIA_port_b
    ; DEBUG: Statement 48 - Discriminant(6)
    ; VPy_LINE:56
    LDA #1
    ; DEBUG: Statement 49 - Discriminant(6)
    ; VPy_LINE:57
    STA VIA_port_b
    ; DEBUG: Statement 50 - Discriminant(6)
    ; VPy_LINE:58
    LDA #40
    ; DEBUG: Statement 51 - Discriminant(6)
    ; VPy_LINE:59
    STA VIA_port_a
    ; DEBUG: Statement 52 - Discriminant(6)
    ; VPy_LINE:60
    CLR VIA_t1_cnt_hi
    ; DEBUG: Statement 53 - Discriminant(6)
    ; VPy_LINE:61
    LDA #$FF
    ; DEBUG: Statement 54 - Discriminant(6)
    ; VPy_LINE:62
    STA VIA_shift_reg
    ; DEBUG: Statement 55 - Discriminant(6)
    ; VPy_LINE:63
    WAIT3:
    ; DEBUG: Statement 56 - Discriminant(6)
    ; VPy_LINE:64
    LDA VIA_int_flags
    ; DEBUG: Statement 57 - Discriminant(6)
    ; VPy_LINE:65
    ANDA #$40
    ; DEBUG: Statement 58 - Discriminant(6)
    ; VPy_LINE:66
    BEQ WAIT3
    ; DEBUG: Statement 59 - Discriminant(6)
    ; VPy_LINE:67
    CLR VIA_shift_reg
    ; DEBUG: Statement 60 - Discriminant(6)
    ; VPy_LINE:68
    LDA #$D8
    ; DEBUG: Statement 61 - Discriminant(6)
    ; VPy_LINE:69
    STA VIA_port_a
    ; DEBUG: Statement 62 - Discriminant(6)
    ; VPy_LINE:70
    CLR VIA_port_b
    ; DEBUG: Statement 63 - Discriminant(6)
    ; VPy_LINE:71
    LDA #1
    ; DEBUG: Statement 64 - Discriminant(6)
    ; VPy_LINE:72
    STA VIA_port_b
    ; DEBUG: Statement 65 - Discriminant(6)
    ; VPy_LINE:73
    CLR VIA_port_a
    ; DEBUG: Statement 66 - Discriminant(6)
    ; VPy_LINE:74
    CLR VIA_t1_cnt_hi
    ; DEBUG: Statement 67 - Discriminant(6)
    ; VPy_LINE:75
    LDA #$FF
    ; DEBUG: Statement 68 - Discriminant(6)
    ; VPy_LINE:76
    STA VIA_shift_reg
    ; DEBUG: Statement 69 - Discriminant(6)
    ; VPy_LINE:77
    WAIT4:
    ; DEBUG: Statement 70 - Discriminant(6)
    ; VPy_LINE:78
    LDA VIA_int_flags
    ; DEBUG: Statement 71 - Discriminant(6)
    ; VPy_LINE:79
    ANDA #$40
    ; DEBUG: Statement 72 - Discriminant(6)
    ; VPy_LINE:80
    BEQ WAIT4
    ; DEBUG: Statement 73 - Discriminant(6)
    ; VPy_LINE:81
    CLR VIA_shift_reg
    ; DEBUG: Statement 74 - Discriminant(6)
    ; VPy_LINE:82
    CLR VIA_port_a
    ; DEBUG: Statement 75 - Discriminant(6)
    ; VPy_LINE:83
    CLR VIA_port_b
    ; DEBUG: Statement 76 - Discriminant(6)
    ; VPy_LINE:84
    LDA #1
    ; DEBUG: Statement 77 - Discriminant(6)
    ; VPy_LINE:85
    STA VIA_port_b
    ; DEBUG: Statement 78 - Discriminant(6)
    ; VPy_LINE:86
    LDA #$D8
    ; DEBUG: Statement 79 - Discriminant(6)
    ; VPy_LINE:87
    STA VIA_port_a
    ; DEBUG: Statement 80 - Discriminant(6)
    ; VPy_LINE:88
    CLR VIA_t1_cnt_hi
    ; DEBUG: Statement 81 - Discriminant(6)
    ; VPy_LINE:89
    LDA #$FF
    ; DEBUG: Statement 82 - Discriminant(6)
    ; VPy_LINE:90
    STA VIA_shift_reg
    ; DEBUG: Statement 83 - Discriminant(6)
    ; VPy_LINE:91
    WAIT5:
    ; DEBUG: Statement 84 - Discriminant(6)
    ; VPy_LINE:92
    LDA VIA_int_flags
    ; DEBUG: Statement 85 - Discriminant(6)
    ; VPy_LINE:93
    ANDA #$40
    ; DEBUG: Statement 86 - Discriminant(6)
    ; VPy_LINE:94
    BEQ WAIT5
    ; DEBUG: Statement 87 - Discriminant(6)
    ; VPy_LINE:95
    CLR VIA_shift_reg
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
    FCC "BEQ WAIT1"
    FCB $80
STR_2:
    FCC "BEQ WAIT2"
    FCB $80
STR_3:
    FCC "BEQ WAIT3"
    FCB $80
STR_4:
    FCC "BEQ WAIT4"
    FCB $80
STR_5:
    FCC "BEQ WAIT5"
    FCB $80
STR_6:
    FCC "CLR VIA_PORT_A"
    FCB $80
STR_7:
    FCC "CLR VIA_PORT_B"
    FCB $80
STR_8:
    FCC "CLR VIA_SHIFT_REG"
    FCB $80
STR_9:
    FCC "CLR VIA_T1_CNT_HI"
    FCB $80
STR_10:
    FCC "JSR INTENSITY_A"
    FCB $80
STR_11:
    FCC "LDA #$7F"
    FCB $80
STR_12:
    FCC "LDA #$82"
    FCB $80
STR_13:
    FCC "LDA #$83"
    FCB $80
STR_14:
    FCC "LDA #$CC"
    FCB $80
STR_15:
    FCC "LDA #$CE"
    FCB $80
STR_16:
    FCC "LDA #$D8"
    FCB $80
STR_17:
    FCC "LDA #$EC"
    FCB $80
STR_18:
    FCC "LDA #$FF"
    FCB $80
STR_19:
    FCC "LDA #1"
    FCB $80
STR_20:
    FCC "LDA #127"
    FCB $80
STR_21:
    FCC "LDA #40"
    FCB $80
STR_22:
    FCC "LDA VIA_INT_FLAGS"
    FCB $80
STR_23:
    FCC "NOP"
    FCB $80
STR_24:
    FCC "STA VIA_CNTL"
    FCB $80
STR_25:
    FCC "STA VIA_PORT_A"
    FCB $80
STR_26:
    FCC "STA VIA_PORT_B"
    FCB $80
STR_27:
    FCC "STA VIA_SHIFT_REG"
    FCB $80
STR_28:
    FCC "STA VIA_T1_CNT_LO"
    FCB $80
STR_29:
    FCC "WAIT1:"
    FCB $80
STR_30:
    FCC "WAIT2:"
    FCB $80
STR_31:
    FCC "WAIT3:"
    FCB $80
STR_32:
    FCC "WAIT4:"
    FCB $80
STR_33:
    FCC "WAIT5:"
    FCB $80
; Call argument scratch space
VAR_ARG0 EQU RESULT+26

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

