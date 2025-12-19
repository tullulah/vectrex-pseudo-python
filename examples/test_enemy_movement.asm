; --- Motorola 6809 backend (Vectrex) title='Enemy Movement Test' origin=$0000 ---
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
    FCC "ENEMY MOVEMENT TEST"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; Must be defined BEFORE builtin helpers that reference them
RESULT         EQU $C880   ; Main result temporary

    JMP START

; DRAW_LINE unified wrapper - handles 16-bit signed coordinates correctly
; Args: (x0,y0,x1,y1,intensity) as 16-bit words, treating x/y as signed bytes.
; ALWAYS sets intensity. Does NOT reset origin (allows connected lines).
DRAW_LINE_WRAPPER:
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode for vector drawing)
    STA >$D00C     ; VIA_cntl
    ; Set DP to hardware registers
    LDA #$D0
    TFR A,DP
    ; ALWAYS set intensity (no optimization)
    LDA VAR_ARG4+1
    JSR Intensity_a
    ; Move to start (y in A, x in B) - use signed byte values
    LDA VAR_ARG1+1  ; Y start (signed byte)
    LDB VAR_ARG0+1  ; X start (signed byte)
    JSR Moveto_d
    ; Compute deltas using 16-bit arithmetic, then clamp to signed bytes
    ; dx = x1 - x0 (treating as signed)
    LDD VAR_ARG2    ; x1 (16-bit)
    SUBD VAR_ARG0   ; subtract x0 (16-bit)
    ; Clamp D to signed byte range (-128 to +127)
    CMPD #127
    BLE DLW_DX_CLAMP_HI_OK
    LDD #127
DLW_DX_CLAMP_HI_OK:
    CMPD #-128
    BGE DLW_DX_CLAMP_LO_OK
    LDD #-128
DLW_DX_CLAMP_LO_OK:
    STB VLINE_DX    ; Store dx as signed byte
    ; dy = y1 - y0 (treating as signed)
    LDD VAR_ARG3    ; y1 (16-bit)
    SUBD VAR_ARG1   ; subtract y0 (16-bit)
    ; Clamp D to signed byte range (-128 to +127)
    CMPD #127
    BLE DLW_DY_CLAMP_HI_OK
    LDD #127
DLW_DY_CLAMP_HI_OK:
    CMPD #-128
    BGE DLW_DY_CLAMP_LO_OK
    LDD #-128
DLW_DY_CLAMP_LO_OK:
    STB VLINE_DY    ; Store dy as signed byte
    ; dx and dy are already clamped to ±127 - no further clamping needed
    ; Vectrex hardware supports full ±127 delta range
    LDA VLINE_DX
    STA VLINE_DX    ; Keep full range
    LDA VLINE_DY
    STA VLINE_DY    ; Keep full range
    ; Clear Vec_Misc_Count for proper timing
    CLR Vec_Misc_Count
    ; Draw line (A=dy, B=dx)
    LDA VLINE_DY
    LDB VLINE_DX
    JSR Draw_Line_d
    LDA #$C8       ; CRITICAL: Restore DP to $C8 for our code
    TFR A,DP
    RTS

; DRAW_LINE_FAST - optimized version that skips redundant setup
; Use this for multiple consecutive draws with same intensity
; Args: (x0,y0,x1,y1) only - intensity must be set beforehand
DRAW_LINE_FAST:
    ; Move to start (y in A, x in B) - use signed byte values
    LDA VAR_ARG1+1  ; Y start (signed byte)
    LDB VAR_ARG0+1  ; X start (signed byte)
    JSR Moveto_d
    ; Compute deltas using 16-bit arithmetic, then clamp to signed bytes
    ; dx = x1 - x0 (treating as signed)
    LDD VAR_ARG2    ; x1 (16-bit)
    SUBD VAR_ARG0   ; subtract x0 (16-bit)
    ; Clamp D to signed byte range (-128 to +127)
    CMPD #127
    BLE DLF_DX_CLAMP_HI_OK
    LDD #127
DLF_DX_CLAMP_HI_OK:
    CMPD #-128
    BGE DLF_DX_CLAMP_LO_OK
    LDD #-128
DLF_DX_CLAMP_LO_OK:
    STB VLINE_DX    ; Store dx as signed byte
    ; dy = y1 - y0 (treating as signed)
    LDD VAR_ARG3    ; y1 (16-bit)
    SUBD VAR_ARG1   ; subtract y0 (16-bit)
    ; Clamp D to signed byte range (-128 to +127)
    CMPD #127
    BLE DLF_DY_CLAMP_HI_OK
    LDD #127
DLF_DY_CLAMP_HI_OK:
    CMPD #-128
    BGE DLF_DY_CLAMP_LO_OK
    LDD #-128
DLF_DY_CLAMP_LO_OK:
    STB VLINE_DY    ; Store dy as signed byte
    ; dx and dy are already clamped to ±127 - no further clamping needed
    ; Vectrex hardware supports full ±127 delta range
    LDA VLINE_DX
    STA VLINE_DX    ; Keep full range
    LDA VLINE_DY
    STA VLINE_DY    ; Keep full range
    ; Clear Vec_Misc_Count for proper timing
    CLR Vec_Misc_Count
    ; Draw line (A=dy, B=dx)
    LDA VLINE_DY
    LDB VLINE_DX
    JSR Draw_Line_d
    RTS
VECTREX_SET_INTENSITY:
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode)
    STA >$D00C     ; VIA_cntl
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
PSHS A                  ; Save intensity again
LDA #$D0
TFR A,DP                ; Set DP=$D0 (BIOS requirement)
PULS A                  ; Restore intensity
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
PSHS A                  ; Save intensity
LDA #$D0
PULS A                  ; Restore intensity
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
START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S

    ; *** DEBUG *** main() function code inline (initialization)
    LDX #ARRAY_0    ; Array literal
    STX VAR_ENEMY_X
    LDX #ARRAY_1    ; Array literal
    STX VAR_ENEMY_Y
    LDX #ARRAY_2    ; Array literal
    STX VAR_ENEMY_DX
    LDX #ARRAY_3    ; Array literal
    STX VAR_ENEMY_ACTIVE
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
    ; DEBUG: Processing 7 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(6)
    ; VPy_LINE:13
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 13
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(7)
    ; VPy_LINE:16
    LDD VAR_ENEMY_ACTIVE
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #0
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_2
    LDD #0
    STD RESULT
    BRA CE_3
CT_2:
    LDD #1
    STD RESULT
CE_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    ; VPy_LINE:17
    LDD VAR_ENEMY_X
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #0
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    ADDD RESULT
    STD RESULT
    ; VPy_LINE:18
    LDD VAR_NEW_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65436
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_6
    LDD #0
    STD RESULT
    BRA CE_7
CT_6:
    LDD #1
    STD RESULT
CE_7:
    LDD RESULT
    LBEQ IF_NEXT_5
    ; VPy_LINE:19
    LDD #0
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD VAR_ENEMY_DX
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR
    LDD #2
    STD RESULT
    LDX TMPPTR
    LDD RESULT
    STD ,X
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    ; VPy_LINE:20
    LDD VAR_NEW_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_10
    LDD #0
    STD RESULT
    BRA CE_11
CT_10:
    LDD #1
    STD RESULT
CE_11:
    LDD RESULT
    LBEQ IF_NEXT_9
    ; VPy_LINE:21
    LDD #0
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD VAR_ENEMY_DX
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR
    LDD #65534
    STD RESULT
    LDX TMPPTR
    LDD RESULT
    STD ,X
    LBRA IF_END_8
IF_NEXT_9:
IF_END_8:
    ; VPy_LINE:22
    LDD #0
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD VAR_ENEMY_X
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR
    LDD VAR_NEW_X
    STD RESULT
    LDX TMPPTR
    LDD RESULT
    STD ,X
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    ; DEBUG: Statement 2 - Discriminant(7)
    ; VPy_LINE:25
    LDD VAR_ENEMY_ACTIVE
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #1
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_14
    LDD #0
    STD RESULT
    BRA CE_15
CT_14:
    LDD #1
    STD RESULT
CE_15:
    LDD RESULT
    LBEQ IF_NEXT_13
    ; VPy_LINE:26
    LDD VAR_ENEMY_X
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #1
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    ADDD RESULT
    STD RESULT
    ; VPy_LINE:27
    LDD VAR_NEW_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65436
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_18
    LDD #0
    STD RESULT
    BRA CE_19
CT_18:
    LDD #1
    STD RESULT
CE_19:
    LDD RESULT
    LBEQ IF_NEXT_17
    ; VPy_LINE:28
    LDD #1
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD VAR_ENEMY_DX
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR
    LDD #2
    STD RESULT
    LDX TMPPTR
    LDD RESULT
    STD ,X
    LBRA IF_END_16
IF_NEXT_17:
IF_END_16:
    ; VPy_LINE:29
    LDD VAR_NEW_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_22
    LDD #0
    STD RESULT
    BRA CE_23
CT_22:
    LDD #1
    STD RESULT
CE_23:
    LDD RESULT
    LBEQ IF_NEXT_21
    ; VPy_LINE:30
    LDD #1
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD VAR_ENEMY_DX
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR
    LDD #65534
    STD RESULT
    LDX TMPPTR
    LDD RESULT
    STD ,X
    LBRA IF_END_20
IF_NEXT_21:
IF_END_20:
    ; VPy_LINE:31
    LDD #1
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD VAR_ENEMY_X
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR
    LDD VAR_NEW_X
    STD RESULT
    LDX TMPPTR
    LDD RESULT
    STD ,X
    LBRA IF_END_12
IF_NEXT_13:
IF_END_12:
    ; DEBUG: Statement 3 - Discriminant(7)
    ; VPy_LINE:34
    LDD VAR_ENEMY_ACTIVE
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_26
    LDD #0
    STD RESULT
    BRA CE_27
CT_26:
    LDD #1
    STD RESULT
CE_27:
    LDD RESULT
    LBEQ IF_NEXT_25
    ; VPy_LINE:35
    LDD VAR_ENEMY_X
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    ADDD RESULT
    STD RESULT
    ; VPy_LINE:36
    LDD VAR_NEW_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65436
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_30
    LDD #0
    STD RESULT
    BRA CE_31
CT_30:
    LDD #1
    STD RESULT
CE_31:
    LDD RESULT
    LBEQ IF_NEXT_29
    ; VPy_LINE:37
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD VAR_ENEMY_DX
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR
    LDD #2
    STD RESULT
    LDX TMPPTR
    LDD RESULT
    STD ,X
    LBRA IF_END_28
IF_NEXT_29:
IF_END_28:
    ; VPy_LINE:38
    LDD VAR_NEW_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_34
    LDD #0
    STD RESULT
    BRA CE_35
CT_34:
    LDD #1
    STD RESULT
CE_35:
    LDD RESULT
    LBEQ IF_NEXT_33
    ; VPy_LINE:39
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD VAR_ENEMY_DX
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR
    LDD #65534
    STD RESULT
    LDX TMPPTR
    LDD RESULT
    STD ,X
    LBRA IF_END_32
IF_NEXT_33:
IF_END_32:
    ; VPy_LINE:40
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD VAR_ENEMY_X
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR
    LDD VAR_NEW_X
    STD RESULT
    LDX TMPPTR
    LDD RESULT
    STD ,X
    LBRA IF_END_24
IF_NEXT_25:
IF_END_24:
    ; DEBUG: Statement 4 - Discriminant(6)
    ; VPy_LINE:43
    LDD VAR_ENEMY_X
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #0
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_ENEMY_Y
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #0
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_ENEMY_X
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #0
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_ENEMY_Y
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #0
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #127
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 5 - Discriminant(6)
    ; VPy_LINE:44
    LDD VAR_ENEMY_X
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #1
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_ENEMY_Y
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #1
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_ENEMY_X
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #1
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_ENEMY_Y
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #1
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #127
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DEBUG: Statement 6 - Discriminant(6)
    ; VPy_LINE:45
    LDD VAR_ENEMY_X
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_ENEMY_Y
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_ENEMY_X
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_ENEMY_Y
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #127
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
TMPLEFT   EQU RESULT+2
TMPRIGHT  EQU RESULT+4
TMPPTR    EQU RESULT+6
TEMP_YX   EQU RESULT+26   ; Temporary y,x storage (2 bytes)
VL_PTR     EQU $CF80      ; Current position in vector list
VL_Y       EQU $CF82      ; Y position (1 byte)
VL_X       EQU $CF83      ; X position (1 byte)
VL_SCALE   EQU $CF84      ; Scale factor (1 byte)
VAR_ENEMY_ACTIVE EQU $CF00+0
VAR_ENEMY_DX EQU $CF00+2
VAR_ENEMY_X EQU $CF00+4
VAR_ENEMY_Y EQU $CF00+6
VAR_NEW_X EQU $CF00+8
; Call argument scratch space
VAR_ARG0 EQU $C8B2
VAR_ARG1 EQU $C8B4
VAR_ARG2 EQU $C8B6
VAR_ARG3 EQU $C8B8
VAR_ARG4 EQU $C8BA
; Array literal for variable 'enemy_x' (3 elements)
ARRAY_0:
    FDB 100   ; Element 0
    FDB 65446   ; Element 1
    FDB 50   ; Element 2

; Array literal for variable 'enemy_y' (3 elements)
ARRAY_1:
    FDB 60   ; Element 0
    FDB 0   ; Element 1
    FDB 65496   ; Element 2

; Array literal for variable 'enemy_dx' (3 elements)
ARRAY_2:
    FDB 65534   ; Element 0
    FDB 2   ; Element 1
    FDB 65533   ; Element 2

; Array literal for variable 'enemy_active' (3 elements)
ARRAY_3:
    FDB 1   ; Element 0
    FDB 1   ; Element 1
    FDB 1   ; Element 2

VLINE_DX EQU RESULT+10
VLINE_DY EQU RESULT+11
VLINE_STEPS EQU RESULT+12
VLINE_LIST EQU RESULT+13
DRAW_VEC_X EQU RESULT+15
DRAW_VEC_Y EQU RESULT+16
