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
; ITERACIÓN 7: Setup only (intensity + reset + move + timing + skip header)
; El loop de dibujo debe estar INLINE en el caller para funcionar correctamente
LDA ,X+                 ; intensity
JSR $F2AB               ; BIOS Intensity_a (expects value in A)
LDB ,X+                 ; y_start
LDA ,X+                 ; x_start
STD TEMP_YX             ; Guardar en variable temporal (evita stack)
; Reset completo
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
; Move sequence
LDD TEMP_YX             ; Recuperar y,x
STB VIA_port_a          ; y to DAC
PSHS A                  ; Save x
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore x
STA VIA_port_a          ; x to DAC
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
    ; DEBUG: Statement 2 - Discriminant(6)
    ; VPy_LINE:16
; PLAY_MUSIC("theme") - play music asset
    LDX #_THEME_MUSIC
    JSR PLAY_MUSIC_RUNTIME
    LDD #0
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
; Embedded 2 of 6 assets (unused assets excluded)
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

; Generated from theme.vmus (internal name: theme)
; Tempo: 120 BPM, Total events: 3

_THEME_MUSIC:
    FDB 120              ; tempo (BPM)
    FDB 24              ; ticks per beat
    FDB $0000,$0180  ; total ticks (32-bit)
    FDB 3              ; num events

    ; Event data
    FCB $01             ; NOTE event
    FDB $0000,$0000  ; start tick
    FDB $0000,$0030  ; duration
    FCB 0              ; channel
    FDB $00B3         ; PSG period
    FCB 12              ; velocity

    FCB $01             ; NOTE event
    FDB $0000,$0030  ; start tick
    FDB $0000,$0030  ; duration
    FCB 0              ; channel
    FDB $008E         ; PSG period
    FCB 12              ; velocity

    FCB $01             ; NOTE event
    FDB $0000,$0060  ; start tick
    FDB $0000,$0030  ; duration
    FCB 0              ; channel
    FDB $0077         ; PSG period
    FCB 12              ; velocity

    FDB $0000,$0000  ; loop start
    FDB $0000,$0180  ; loop end


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

