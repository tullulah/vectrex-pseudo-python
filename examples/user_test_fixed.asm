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

VECTREX_PRINT_TEXT:
    ; CRITICAL: Print_Str_d requires DP=$D0 and signature is (Y, X, string)
    ; VPy signature: PRINT_TEXT(string, Y, X) -> args (ARG0=string, ARG1=Y, ARG2=X)
    ; BIOS signature: Print_Str_d(A=Y, B=X, U=string)
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDU VAR_ARG0   ; string pointer (ARG0 = first param)
    LDA VAR_ARG1+1 ; Y (ARG1 = second param)
    LDB VAR_ARG2+1 ; X (ARG2 = third param)
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
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDA VAR_ARG0+1
    JSR __Intensity_a
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
; ============================================================================
; Draw_Sync_List - EXACT port of Malban's draw_synced_list_c
; Data: FCB intensity, y_start, x_start, next_y, next_x, [flag, dy, dx]*, 2
; ============================================================================
Draw_Sync_List:
; VERSIÓN SIN MOVE: Respeta posición establecida por MOVE() previo
; El loop de dibujo debe estar INLINE en el caller para funcionar correctamente
LDA ,X+                 ; intensity
JSR $F2AB               ; BIOS Intensity_a (expects value in A)
LEAX 2,X                ; Skip y_start, x_start (posición ya establecida por MOVE)
; Reset completo VIA (sin move)
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
; NO hacemos move - usamos la posición actual del beam (MOVE previo)
; Timing setup
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
RTS                     ; Return con X apuntando al primer flag
; CÓDIGO MUERTO (ya migrado arriba):
LDB ,X+                 ; y_start (DEAD)
LDA ,X+                 ; x_start (DEAD)
PSHS D                  ; Save y,x (DEAD)
; Reset to zero (Malban resync) - AFTER intensity
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
; Read flag and draw (EXACT copy of working inline)
LDA ,X+
TSTA
BPL DSL_done
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
DSL_done:
RTS
START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
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
; DRAW_VECTOR("player") - Using Malban format with inline loop
    LDX #_PLAYER_VECTORS  ; X = vector data pointer
    JSR Draw_Sync_List  ; Setup: intensity + reset + move + timing
; Draw_Sync_List returns with X pointing to first flag
DSL_LOOP_1:
    LDA VIA_int_flags
    ANDA #$40
    BEQ DSL_LOOP_1   ; Wait for timer
    LDA ,X+
    CMPA #2      ; Check end marker
    BEQ DSL_DONE_1   ; Done if end marker
    LDB ,X+      ; dy
    LDA ,X+      ; dx
    PSHS A       ; Save x
    STB VIA_port_a  ; Set y DAC
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b  ; Latch y
    PULS A       ; Restore x
    STA VIA_port_a  ; Set x DAC
    CLR VIA_t1_cnt_hi
    LDA #$FF
    STA VIA_shift_reg  ; Enable drawing
DSL_SKIP_1:
    LDA VIA_int_flags
    ANDA #$40
    BEQ DSL_SKIP_1   ; Wait for draw complete
    CLR VIA_shift_reg  ; Disable drawing
    BRA DSL_LOOP_1   ; Loop for next line
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
; DRAW_VECTOR("enemy") - Using Malban format with inline loop
    LDX #_ENEMY_VECTORS  ; X = vector data pointer
    JSR Draw_Sync_List  ; Setup: intensity + reset + move + timing
; Draw_Sync_List returns with X pointing to first flag
DSL_LOOP_2:
    LDA VIA_int_flags
    ANDA #$40
    BEQ DSL_LOOP_2   ; Wait for timer
    LDA ,X+
    CMPA #2      ; Check end marker
    BEQ DSL_DONE_2   ; Done if end marker
    LDB ,X+      ; dy
    LDA ,X+      ; dx
    PSHS A       ; Save x
    STB VIA_port_a  ; Set y DAC
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b  ; Latch y
    PULS A       ; Restore x
    STA VIA_port_a  ; Set x DAC
    CLR VIA_t1_cnt_hi
    LDA #$FF
    STA VIA_shift_reg  ; Enable drawing
DSL_SKIP_2:
    LDA VIA_int_flags
    ANDA #$40
    BEQ DSL_SKIP_2   ; Wait for draw complete
    CLR VIA_shift_reg  ; Disable drawing
    BRA DSL_LOOP_2   ; Loop for next line
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
; DRAW_VECTOR("enemy") - Using Malban format with inline loop
    LDX #_ENEMY_VECTORS  ; X = vector data pointer
    JSR Draw_Sync_List  ; Setup: intensity + reset + move + timing
; Draw_Sync_List returns with X pointing to first flag
DSL_LOOP_3:
    LDA VIA_int_flags
    ANDA #$40
    BEQ DSL_LOOP_3   ; Wait for timer
    LDA ,X+
    CMPA #2      ; Check end marker
    BEQ DSL_DONE_3   ; Done if end marker
    LDB ,X+      ; dy
    LDA ,X+      ; dx
    PSHS A       ; Save x
    STB VIA_port_a  ; Set y DAC
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b  ; Latch y
    PULS A       ; Restore x
    STA VIA_port_a  ; Set x DAC
    CLR VIA_t1_cnt_hi
    LDA #$FF
    STA VIA_shift_reg  ; Enable drawing
DSL_SKIP_3:
    LDA VIA_int_flags
    ANDA #$40
    BEQ DSL_SKIP_3   ; Wait for draw complete
    CLR VIA_shift_reg  ; Disable drawing
    BRA DSL_LOOP_3   ; Loop for next line
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
; DRAW_VECTOR("bullet") - Using Malban format with inline loop
    LDX #_BULLET_VECTORS  ; X = vector data pointer
    JSR Draw_Sync_List  ; Setup: intensity + reset + move + timing
; Draw_Sync_List returns with X pointing to first flag
DSL_LOOP_4:
    LDA VIA_int_flags
    ANDA #$40
    BEQ DSL_LOOP_4   ; Wait for timer
    LDA ,X+
    CMPA #2      ; Check end marker
    BEQ DSL_DONE_4   ; Done if end marker
    LDB ,X+      ; dy
    LDA ,X+      ; dx
    PSHS A       ; Save x
    STB VIA_port_a  ; Set y DAC
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b  ; Latch y
    PULS A       ; Restore x
    STA VIA_port_a  ; Set x DAC
    CLR VIA_t1_cnt_hi
    LDA #$FF
    STA VIA_shift_reg  ; Enable drawing
DSL_SKIP_4:
    LDA VIA_int_flags
    ANDA #$40
    BEQ DSL_SKIP_4   ; Wait for draw complete
    CLR VIA_shift_reg  ; Disable drawing
    BRA DSL_LOOP_4   ; Loop for next line
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

;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
TEMP_YX   EQU RESULT+26   ; Temporary y,x storage (2 bytes)
MUSIC_PTR     EQU RESULT+28
MUSIC_TICK    EQU RESULT+30   ; 32-bit tick counter
MUSIC_EVENT   EQU RESULT+34   ; Current event pointer
MUSIC_ACTIVE  EQU RESULT+36   ; Playback state (1 byte)
VL_PTR     EQU $CF80      ; Current position in vector list
VL_Y       EQU $CF82      ; Y position (1 byte)
VL_X       EQU $CF83      ; X position (1 byte)
VL_SCALE   EQU $CF84      ; Scale factor (1 byte)
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
; Embedded 3 of 6 assets (unused assets excluded)
; ========================================

; Vector asset: player
; Generated from player.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 3

_PLAYER_VECTORS:
    FCB 127              ; path0: intensity
    FCB $14,$00,0,0        ; path0: header (y=20, x=0, next_y=0, next_x=0)
    FCB $FF,$E2,$F1          ; line 0: flag=-1, dy=-30, dx=-15
    FCB $FF,$00,$1E          ; line 1: flag=-1, dy=0, dx=30
    FCB $FF,$1E,$F1          ; closing line: flag=-1, dy=30, dx=-15
    FCB 2                ; End marker

; Vector asset: bullet
; Generated from bullet.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 4

_BULLET_VECTORS:
    FCB 127              ; path0: intensity
    FCB $02,$FE,0,0        ; path0: header (y=2, x=-2, next_y=0, next_x=0)
    FCB $FF,$00,$04          ; line 0: flag=-1, dy=0, dx=4
    FCB $FF,$FC,$00          ; line 1: flag=-1, dy=-4, dx=0
    FCB $FF,$00,$FC          ; line 2: flag=-1, dy=0, dx=-4
    FCB $FF,$04,$00          ; closing line: flag=-1, dy=4, dx=0
    FCB 2                ; End marker

; Vector asset: enemy
; Generated from enemy.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 4

_ENEMY_VECTORS:
    FCB 127              ; path0: intensity
    FCB $0A,$F6,0,0        ; path0: header (y=10, x=-10, next_y=0, next_x=0)
    FCB $FF,$00,$14          ; line 0: flag=-1, dy=0, dx=20
    FCB $FF,$EC,$00          ; line 1: flag=-1, dy=-20, dx=0
    FCB $FF,$00,$EC          ; line 2: flag=-1, dy=0, dx=-20
    FCB $FF,$14,$00          ; closing line: flag=-1, dy=20, dx=0
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

