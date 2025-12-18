; --- Motorola 6809 backend (Vectrex) title='Joystick Test' origin=$0000 ---
        ORG $0000
;***************************************************************************
; DEFINE SECTION
;***************************************************************************
    INCLUDE "VECTREX.I"

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
    FCC "JOYSTICK TEST"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; Must be defined BEFORE builtin helpers that reference them
RESULT         EQU $C880   ; Main result temporary

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
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode)
    STA >$D00C     ; VIA_cntl
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDA VAR_ARG0+1
    JSR __Intensity_a
    LDA #$C8       ; Restore DP to $C8 for our code
    TFR A,DP
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
; ITERACIÓN 11: Loop completo dentro (bug assembler arreglado, datos embebidos OK)
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
; Wait for move to complete
DSL_W1:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_W1
; Loop de dibujo
DSL_LOOP:
LDA ,X+                 ; Read flag
CMPA #2                 ; Check end marker
LBEQ DSL_DONE           ; Exit if end (long branch)
CMPA #1                 ; Check next path marker
LBEQ DSL_NEXT_PATH      ; Process next path (long branch)
; Draw line
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
; Wait for line draw
DSL_W2:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_W2
CLR VIA_shift_reg
BRA DSL_LOOP
; Next path: read new intensity and header, then continue drawing
DSL_NEXT_PATH:
; Save current X position before reading anything
TFR X,D                 ; D = X (current position)
PSHS D                  ; Save X address
LDA ,X+                 ; Read intensity (X now points to y_start)
PSHS A                  ; Save intensity
LDB ,X+                 ; y_start
LDA ,X+                 ; x_start (X now points to next_y)
STD TEMP_YX             ; Save y,x
PULS A                  ; Get intensity back
JSR $F2AB               ; BIOS Intensity_a (may corrupt X!)
; Restore X to point to next_y,next_x (after the 3 bytes we read)
PULS D                  ; Get original X
ADDD #3                 ; Skip intensity, y_start, x_start
TFR D,X                 ; X now points to next_y
; Reset to zero (same as Draw_Sync_List start)
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
; Move to new start position
LDD TEMP_YX
STB VIA_port_a          ; y to DAC
PSHS A
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A
STA VIA_port_a          ; x to DAC
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
; Wait for move
DSL_W3:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_W3
CLR VIA_shift_reg       ; Clear before continuing
BRA DSL_LOOP            ; Continue drawing
DSL_DONE:
RTS

; ============================================================================
; Draw_Sync_List_At - Draw vector at offset position (DRAW_VEC_X, DRAW_VEC_Y)
; Same as Draw_Sync_List but adds offset to y_start, x_start coordinates
; Uses: DRAW_VEC_X, DRAW_VEC_Y (set by DRAW_VECTOR before calling this)
; ============================================================================
Draw_Sync_List_At:
LDA ,X+                 ; intensity
JSR $F2AB               ; BIOS Intensity_a
LDB ,X+                 ; y_start from .vec
ADDB DRAW_VEC_Y         ; Add Y offset
LDA ,X+                 ; x_start from .vec
ADDA DRAW_VEC_X         ; Add X offset
STD TEMP_YX             ; Save adjusted position
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
LDD TEMP_YX             ; Recuperar y,x ajustado
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
; Wait for move to complete
DSLA_W1:
LDA VIA_int_flags
ANDA #$40
BEQ DSLA_W1
; Loop de dibujo (same as Draw_Sync_List)
DSLA_LOOP:
LDA ,X+                 ; Read flag
CMPA #2                 ; Check end marker
LBEQ DSLA_DONE
CMPA #1                 ; Check next path marker
LBEQ DSLA_NEXT_PATH
; Draw line
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
; Wait for line draw
DSLA_W2:
LDA VIA_int_flags
ANDA #$40
BEQ DSLA_W2
CLR VIA_shift_reg
BRA DSLA_LOOP
; Next path: add offset to new coordinates too
DSLA_NEXT_PATH:
TFR X,D
PSHS D
LDA ,X+                 ; Read intensity
PSHS A
LDB ,X+                 ; y_start
ADDB DRAW_VEC_Y         ; Add Y offset to new path
LDA ,X+                 ; x_start
ADDA DRAW_VEC_X         ; Add X offset to new path
STD TEMP_YX
PULS A                  ; Get intensity back
JSR $F2AB
PULS D
ADDD #3
TFR D,X
; Reset to zero
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
; Move to new start position (already offset-adjusted)
LDD TEMP_YX
STB VIA_port_a
PSHS A
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A
STA VIA_port_a
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X
; Wait for move
DSLA_W3:
LDA VIA_int_flags
ANDA #$40
BEQ DSLA_W3
CLR VIA_shift_reg
BRA DSLA_LOOP
DSLA_DONE:
RTS
READ_J1_X:
; Read Joystick 1 X axis from analog channel 1
; Hardware: alg_jch1 mapped at $C880 (emulator sets this)
; Returns: D = signed value (-127 to +127, 0 = center)
LDB $C880       ; Read analog channel 1 (J1 X)
SUBB #128       ; Convert 0-255 to -128 to +127
SEX             ; Sign-extend B to D
RTS
READ_J1_Y:
; Read Joystick 1 Y axis from analog channel 0
; Hardware: alg_jch0 mapped at $C881 (emulator sets this)
; Returns: D = signed value (-127 to +127, 0 = center)
LDB $C881       ; Read analog channel 0 (J1 Y)
SUBB #128       ; Convert 0-255 to -128 to +127
SEX             ; Sign-extend B to D
RTS
READ_J2_X:
; Read Joystick 2 X axis from analog channel 3
; Hardware: alg_jch3 mapped at $C882 (emulator sets this)
; Returns: D = signed value (-127 to +127, 0 = center)
LDB $C882       ; Read analog channel 3 (J2 X)
SUBB #128       ; Convert 0-255 to -128 to +127
SEX             ; Sign-extend B to D
RTS
READ_J2_Y:
; Read Joystick 2 Y axis from analog channel 2
; Hardware: alg_jch2 mapped at $C883 (emulator sets this)
; Returns: D = signed value (-127 to +127, 0 = center)
LDB $C883       ; Read analog channel 2 (J2 Y)
SUBB #128       ; Convert 0-255 to -128 to +127
SEX             ; Sign-extend B to D
RTS
READ_J1_BUTTON_1:
; Read Joystick 1 Button 1 (PSG reg 14, bit 0)
; Returns: D = 0 (released) or 1 (pressed)
LDA $C884       ; Read PSG register 14 (button states)
ANDA #$01       ; Mask bit 0 (J1 Button 1)
BEQ RJ1B1_ZERO
LDD #1          ; Button pressed
RTS
RJ1B1_ZERO:
LDD #0          ; Button released
RTS
READ_J1_BUTTON_2:
LDA $C884       ; Read PSG register 14
ANDA #$02       ; Mask bit 1 (J1 Button 2)
BEQ RJ1B2_ZERO
LDD #1
RTS
RJ1B2_ZERO:
LDD #0
RTS
READ_J1_BUTTON_3:
LDA $C884       ; Read PSG register 14
ANDA #$04       ; Mask bit 2 (J1 Button 3)
BEQ RJ1B3_ZERO
LDD #1
RTS
RJ1B3_ZERO:
LDD #0
RTS
READ_J1_BUTTON_4:
LDA $C884       ; Read PSG register 14
ANDA #$08       ; Mask bit 3 (J1 Button 4)
BEQ RJ1B4_ZERO
LDD #1
RTS
RJ1B4_ZERO:
LDD #0
RTS
READ_J2_BUTTON_1:
; Read Joystick 2 Button 1 (PSG reg 14, bit 4)
; Returns: D = 0 (released) or 1 (pressed)
LDA $C884       ; Read PSG register 14
ANDA #$10       ; Mask bit 4 (J2 Button 1)
BEQ RJ2B1_ZERO
LDD #1
RTS
RJ2B1_ZERO:
LDD #0
RTS
READ_J2_BUTTON_2:
LDA $C884       ; Read PSG register 14
ANDA #$20       ; Mask bit 5 (J2 Button 2)
BEQ RJ2B2_ZERO
LDD #1
RTS
RJ2B2_ZERO:
LDD #0
RTS
READ_J2_BUTTON_3:
LDA $C884       ; Read PSG register 14
ANDA #$40       ; Mask bit 6 (J2 Button 3)
BEQ RJ2B3_ZERO
LDD #1
RTS
RJ2B3_ZERO:
LDD #0
RTS
READ_J2_BUTTON_4:
LDA $C884       ; Read PSG register 14
ANDA #$80       ; Mask bit 7 (J2 Button 4)
BEQ RJ2B4_ZERO
LDD #1
RTS
RJ2B4_ZERO:
LDD #0
RTS
START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
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
    ; DEBUG: Processing 19 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    ; VPy_LINE:8
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 8
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(1)
    ; VPy_LINE:11
; NATIVE_CALL: J1_X at line 11
; J1_X() - Read Joystick 1 X axis
    JSR READ_J1_X
    STD RESULT
    ; DEBUG: Statement 2 - Discriminant(1)
    ; VPy_LINE:12
; NATIVE_CALL: J1_Y at line 12
; J1_Y() - Read Joystick 1 Y axis
    JSR READ_J1_Y
    STD RESULT
    ; DEBUG: Statement 3 - Discriminant(1)
    ; VPy_LINE:16
    LDD #0
    STD RESULT
    ; DEBUG: Statement 4 - Discriminant(1)
    ; VPy_LINE:17
    LDD #0
    STD RESULT
    ; DEBUG: Statement 5 - Discriminant(6)
    ; VPy_LINE:20
    LDD VAR_CENTER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #20
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_CENTER_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_MOVE_TO at line 20
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 6 - Discriminant(6)
    ; VPy_LINE:21
    LDD VAR_CENTER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #20
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_CENTER_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_DRAW_TO at line 21
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 7 - Discriminant(6)
    ; VPy_LINE:24
    LDD VAR_CENTER_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_CENTER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #20
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_MOVE_TO at line 24
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 8 - Discriminant(6)
    ; VPy_LINE:25
    LDD VAR_CENTER_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_CENTER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #20
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_DRAW_TO at line 25
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 9 - Discriminant(1)
    ; VPy_LINE:28
    LDD VAR_J1_X
    STD RESULT
    ; DEBUG: Statement 10 - Discriminant(1)
    ; VPy_LINE:29
    LDD VAR_J1_Y
    STD RESULT
    ; DEBUG: Statement 11 - Discriminant(6)
    ; VPy_LINE:32
    LDD VAR_CURSOR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_CURSOR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_MOVE_TO at line 32
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 12 - Discriminant(6)
    ; VPy_LINE:33
    LDD VAR_CURSOR_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_CURSOR_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_DRAW_TO at line 33
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 13 - Discriminant(6)
    ; VPy_LINE:34
    LDD VAR_CURSOR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_CURSOR_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_MOVE_TO at line 34
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 14 - Discriminant(6)
    ; VPy_LINE:35
    LDD VAR_CURSOR_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_CURSOR_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_DRAW_TO at line 35
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 15 - Discriminant(7)
    ; VPy_LINE:38
; NATIVE_CALL: J1_BUTTON_1 at line 38
; J1_BUTTON_1() - Read Joystick 1 button 1
    JSR READ_J1_BUTTON_1
    STD RESULT
    LDD RESULT
    LBEQ IF_NEXT_1
    ; VPy_LINE:40
    LDD #65476
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_MOVE_TO at line 40
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:41
    LDD #65481
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_DRAW_TO at line 41
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    ; DEBUG: Statement 16 - Discriminant(7)
    ; VPy_LINE:43
; NATIVE_CALL: J1_BUTTON_2 at line 43
; J1_BUTTON_2() - Read Joystick 1 button 2
    JSR READ_J1_BUTTON_2
    STD RESULT
    LDD RESULT
    LBEQ IF_NEXT_3
    ; VPy_LINE:45
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_MOVE_TO at line 45
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:46
    LDD #55
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_DRAW_TO at line 46
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_2
IF_NEXT_3:
IF_END_2:
    ; DEBUG: Statement 17 - Discriminant(7)
    ; VPy_LINE:48
; NATIVE_CALL: J1_BUTTON_3 at line 48
; J1_BUTTON_3() - Read Joystick 1 button 3
    JSR READ_J1_BUTTON_3
    STD RESULT
    LDD RESULT
    LBEQ IF_NEXT_5
    ; VPy_LINE:50
    LDD #65476
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65476
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_MOVE_TO at line 50
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:51
    LDD #65481
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65476
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_DRAW_TO at line 51
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    ; DEBUG: Statement 18 - Discriminant(7)
    ; VPy_LINE:53
; NATIVE_CALL: J1_BUTTON_4 at line 53
; J1_BUTTON_4() - Read Joystick 1 button 4
    JSR READ_J1_BUTTON_4
    STD RESULT
    LDD RESULT
    LBEQ IF_NEXT_7
    ; VPy_LINE:55
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65476
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_MOVE_TO at line 55
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:56
    LDD #55
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65476
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_DRAW_TO at line 56
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_6
IF_NEXT_7:
IF_END_6:
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
TMPLEFT   EQU RESULT+2
TMPRIGHT  EQU RESULT+4
TEMP_YX   EQU RESULT+26   ; Temporary y,x storage (2 bytes)
VL_PTR     EQU $CF80      ; Current position in vector list
VL_Y       EQU $CF82      ; Y position (1 byte)
VL_X       EQU $CF83      ; X position (1 byte)
VL_SCALE   EQU $CF84      ; Scale factor (1 byte)
VAR_CENTER_X EQU $CF00+0
VAR_CENTER_Y EQU $CF00+2
VAR_CURSOR_X EQU $CF00+4
VAR_CURSOR_Y EQU $CF00+6
VAR_J1_X EQU $CF00+8
VAR_J1_Y EQU $CF00+10
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VCUR_X EQU RESULT+12
VCUR_Y EQU RESULT+13
VLINE_DX EQU RESULT+14
VLINE_DY EQU RESULT+15
VLINE_STEPS EQU RESULT+16
VLINE_LIST EQU RESULT+17
DRAW_VEC_X EQU RESULT+19
DRAW_VEC_Y EQU RESULT+20
