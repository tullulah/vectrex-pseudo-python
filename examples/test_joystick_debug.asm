; --- Motorola 6809 backend (Vectrex) title='Joystick Debug' origin=$0000 ---
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
    FCC "JOYSTICK DEBUG"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 54 bytes
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$02   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$04   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$06   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$08   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0A   ; Pointer temp (2 bytes)
TMPPTR2              EQU $C880+$0C   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0E   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$10   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$11   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$12   ; String buffer for PRINT_NUMBER (-FFFF format) (6 bytes)
VL_PTR               EQU $C880+$18   ; Current position in vector list (2 bytes)
VL_Y                 EQU $C880+$1A   ; Y position (1 bytes)
VL_X                 EQU $C880+$1B   ; X position (1 bytes)
VL_SCALE             EQU $C880+$1C   ; Scale factor (1 bytes)
DRAW_VEC_X           EQU $C880+$1D   ; DRAW_VECTOR X offset (1 bytes)
DRAW_VEC_Y           EQU $C880+$1E   ; DRAW_VECTOR Y offset (1 bytes)
MIRROR_X             EQU $C880+$1F   ; Mirror X flag for DRAW_VECTOR_EX (1 bytes)
MIRROR_Y             EQU $C880+$20   ; Mirror Y flag for DRAW_VECTOR_EX (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$21   ; Intensity override (0=use vector's) (1 bytes)
DRAW_CIRCLE_XC       EQU $C880+$22   ; Circle center X (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$23   ; Circle center Y (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$24   ; Circle diameter (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$25   ; Circle intensity (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$26   ; Circle temp (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_JOY_X            EQU $C880+$2E   ; User variable (2 bytes)
VAR_JOY_Y            EQU $C880+$30   ; User variable (2 bytes)
VAR_JOY_X_RAW        EQU $C880+$32   ; User variable (2 bytes)
VAR_JOY_Y_RAW        EQU $C880+$34   ; User variable (2 bytes)

    JMP START

;**** CONST DECLARATIONS (NUMBER-ONLY) ****

; === JOYSTICK BUILTIN SUBROUTINES ===
; J1_X() - Read Joystick 1 X axis from BIOS
; Returns: D = signed value -128 to +127 (0 = center)
J1X_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81B from hardware)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81B)
    LDB $C81B    ; Vec_Joy_1_X (now updated by Joy_Analog)
    SUBB #128    ; Center at 0: 0→-128, 128→0, 255→127
    SEX          ; Sign-extend B to D (preserves negative values)
    PULS X       ; Restore X
    RTS

; J1_Y() - Read Joystick 1 Y axis from BIOS
; Returns: D = signed value -128 to +127 (0 = center)
J1Y_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81C from hardware)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81C)
    LDB $C81C    ; Vec_Joy_1_Y (now updated by Joy_Analog)
    SUBB #128    ; Center at 0: 0→-128, 128→0, 255→127
    SEX          ; Sign-extend B to D (preserves negative values)
    PULS X       ; Restore X
    RTS

; === BUTTON BUILTIN SUBROUTINES ===
; J1_BUTTON_1() - Read Joystick 1 button 1 (BIOS)
; Returns: D = 0 (released), 1 (pressed)
; NOTE: Leaves DP=$D0 after call (BIOS convention)
J1B1_BUILTIN:
    JSR $F1AA    ; DP_to_D0 (BIOS routine)
    CLR $C80F    ; Clear Vec_Btn_State before reading (fix stale buttons on hardware)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$01
    BEQ .J1B1_OFF
    JSR $F1AF    ; DP_to_C8 (restore before return)
    LDD #1
    RTS
.J1B1_OFF:
    JSR $F1AF    ; DP_to_C8 (restore before return)
    LDD #0
    RTS

; J1_BUTTON_2() - Read Joystick 1 button 2 (BIOS)
; NOTE: Leaves DP=$D0 after call (BIOS convention)
J1B2_BUILTIN:
    JSR $F1AA    ; DP_to_D0 (BIOS routine)
    CLR $C80F    ; Clear Vec_Btn_State before reading (fix stale buttons on hardware)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$02
    BEQ .J1B2_OFF
    JSR $F1AF    ; DP_to_C8 (restore before return)
    LDD #1
    RTS
.J1B2_OFF:
    JSR $F1AF    ; DP_to_C8 (restore before return)
    LDD #0
    RTS

; J1_BUTTON_3() - Read Joystick 1 button 3 (BIOS)
; NOTE: Leaves DP=$D0 after call (BIOS convention)
J1B3_BUILTIN:
    JSR $F1AA    ; DP_to_D0 (BIOS routine)
    CLR $C80F    ; Clear Vec_Btn_State before reading (fix stale buttons on hardware)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$04
    BEQ .J1B3_OFF
    JSR $F1AF    ; DP_to_C8 (restore before return)
    LDD #1
    RTS
.J1B3_OFF:
    JSR $F1AF    ; DP_to_C8 (restore before return)
    LDD #0
    RTS

; J1_BUTTON_4() - Read Joystick 1 button 4 (BIOS)
; NOTE: Leaves DP=$D0 after call (BIOS convention)
J1B4_BUILTIN:
    JSR $F1AA    ; DP_to_D0 (BIOS routine)
    CLR $C80F    ; Clear Vec_Btn_State before reading (fix stale buttons on hardware)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$08
    BEQ .J1B4_OFF
    JSR $F1AF    ; DP_to_C8 (restore before return)
    LDD #1
    RTS
.J1B4_OFF:
    JSR $F1AF    ; DP_to_C8 (restore before return)
    LDD #0
    RTS

VECTREX_PRINT_TEXT:
    ; CRITICAL: Print_Str_d requires DP=$D0 and signature is (Y, X, string)
    ; VPy signature: PRINT_TEXT(x, y, string) -> args (ARG0=x, ARG1=y, ARG2=string)
    ; BIOS signature: Print_Str_d(A=Y, B=X, U=string)
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode for text rendering)
    STA >$D00C     ; VIA_cntl
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDU VAR_ARG2   ; string pointer (ARG2 = third param)
    LDA VAR_ARG1+1 ; Y (ARG1 = second param)
    LDB VAR_ARG0+1 ; X (ARG0 = first param)
    JSR Print_Str_d
    JSR $F1AF      ; DP_to_C8 (restore before return - CRITICAL for TMPPTR access)
    RTS
VECTREX_PRINT_NUMBER:
    ; Print signed 16-bit number at position
    ; VPy signature: PRINT_NUMBER(x, y, value) -> args (ARG0=x, ARG1=y, ARG2=value)
    ; ARG0 = X position, ARG1 = Y position, ARG2 = number value (16-bit signed)
    
    ; CRITICAL: Configure VIA mode and DP BEFORE using BIOS
    LDA #$98       ; VIA_cntl = $98 (DAC mode for text rendering)
    STA >$D00C     ; VIA_cntl
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    
    ; Load 16-bit signed number
    LDD VAR_ARG2     ; Load full 16-bit value
    
    ; Check if negative
    TSTA             ; Test high byte
    BPL PN_POS       ; Branch if positive
    
    ; Negative: print '-' and negate
    LDA #'-'
    STA NUM_STR
    LDD VAR_ARG2
    COMA             ; Two's complement
    COMB
    ADDD #1
    STD VAR_ARG2     ; Store positive value temporarily
    LDA #1
    STA NUM_STR+1    ; Mark that we have minus sign
    BRA PN_CONV
    
PN_POS:
    CLR NUM_STR+1    ; No minus sign
    
PN_CONV:
    ; Convert to hex (format: -FFFF or FFFF)
    LDA VAR_ARG2     ; High byte
    
    ; Convert high byte to hex
    LSRA
    LSRA
    LSRA
    LSRA
    ANDA #$0F
    CMPA #10
    BLO PN_HI1
    ADDA #7
PN_HI1:
    ADDA #'0'
    STA NUM_STR+2
    
    LDA VAR_ARG2
    ANDA #$0F
    CMPA #10
    BLO PN_HI2
    ADDA #7
PN_HI2:
    ADDA #'0'
    STA NUM_STR+3
    
    ; Convert low byte to hex
    LDA VAR_ARG2+1
    LSRA
    LSRA
    LSRA
    LSRA
    ANDA #$0F
    CMPA #10
    BLO PN_LO1
    ADDA #7
PN_LO1:
    ADDA #'0'
    STA NUM_STR+4
    
    LDA VAR_ARG2+1
    ANDA #$0F
    CMPA #10
    BLO PN_LO2
    ADDA #7
PN_LO2:
    ADDA #'0'
    ORA #$80         ; Terminator
    STA NUM_STR+5
    
    ; Print the string using Print_Str_d (handles positioning correctly)
    ; Print_Str_d signature: A=Y, B=X, U=string
    TST NUM_STR+1
    BEQ PN_NOMIN
    LDU #NUM_STR     ; Print with minus
    BRA PN_LOAD_POS
PN_NOMIN:
    LDU #NUM_STR+2   ; Skip minus
PN_LOAD_POS:
    LDA VAR_ARG1+1   ; Y position (ARG1 = second param)
    LDB VAR_ARG0+1   ; X position (ARG0 = first param)
    JSR Print_Str_d
    JSR $F1AF        ; DP_to_C8 (restore DP before return)
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
CLR Vec_Misc_Count      ; Clear for relative line drawing (CRITICAL for continuity)
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
CLR Vec_Misc_Count      ; Clear for relative line drawing (CRITICAL for continuity)
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
Draw_Sync_List_At_With_Mirrors:
; Unified mirror support using flags: MIRROR_X and MIRROR_Y
; Conditionally negates X and/or Y coordinates and deltas
LDA DRAW_VEC_INTENSITY  ; Check if intensity override is set
BNE DSWM_USE_OVERRIDE   ; If non-zero, use override
LDA ,X+                 ; Otherwise, read intensity from vector data
BRA DSWM_SET_INTENSITY
DSWM_USE_OVERRIDE:
LEAX 1,X                ; Skip intensity byte in vector data
DSWM_SET_INTENSITY:
PSHS A                  ; Save intensity
LDA #$D0
PULS A                  ; Restore intensity
JSR $F2AB               ; BIOS Intensity_a
LDB ,X+                 ; y_start from .vec (already relative to center)
; Check if Y mirroring is enabled
TST MIRROR_Y
BEQ DSWM_NO_NEGATE_Y
NEGB                    ; ← Negate Y if flag set
DSWM_NO_NEGATE_Y:
ADDB DRAW_VEC_Y         ; Add Y offset
LDA ,X+                 ; x_start from .vec (already relative to center)
; Check if X mirroring is enabled
TST MIRROR_X
BEQ DSWM_NO_NEGATE_X
NEGA                    ; ← Negate X if flag set
DSWM_NO_NEGATE_X:
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
LDD TEMP_YX
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
DSWM_W1:
LDA VIA_int_flags
ANDA #$40
BEQ DSWM_W1
; Loop de dibujo (conditional mirrors)
DSWM_LOOP:
LDA ,X+                 ; Read flag
CMPA #2                 ; Check end marker
LBEQ DSWM_DONE
CMPA #1                 ; Check next path marker
LBEQ DSWM_NEXT_PATH
; Draw line with conditional negations
LDB ,X+                 ; dy
; Check if Y mirroring is enabled
TST MIRROR_Y
BEQ DSWM_NO_NEGATE_DY
NEGB                    ; ← Negate dy if flag set
DSWM_NO_NEGATE_DY:
LDA ,X+                 ; dx
; Check if X mirroring is enabled
TST MIRROR_X
BEQ DSWM_NO_NEGATE_DX
NEGA                    ; ← Negate dx if flag set
DSWM_NO_NEGATE_DX:
PSHS A                  ; Save final dx
STB VIA_port_a          ; dy (possibly negated) to DAC
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore final dx
STA VIA_port_a          ; dx (possibly negated) to DAC
CLR VIA_t1_cnt_hi
LDA #$FF
STA VIA_shift_reg
; Wait for line draw
DSWM_W2:
LDA VIA_int_flags
ANDA #$40
BEQ DSWM_W2
CLR VIA_shift_reg
BRA DSWM_LOOP
; Next path: repeat mirror logic for new path header
DSWM_NEXT_PATH:
TFR X,D
PSHS D
; Check intensity override (same logic as start)
LDA DRAW_VEC_INTENSITY  ; Check if intensity override is set
BNE DSWM_NEXT_USE_OVERRIDE   ; If non-zero, use override
LDA ,X+                 ; Otherwise, read intensity from vector data
BRA DSWM_NEXT_SET_INTENSITY
DSWM_NEXT_USE_OVERRIDE:
LEAX 1,X                ; Skip intensity byte in vector data
DSWM_NEXT_SET_INTENSITY:
PSHS A
LDB ,X+                 ; y_start
TST MIRROR_Y
BEQ DSWM_NEXT_NO_NEGATE_Y
NEGB
DSWM_NEXT_NO_NEGATE_Y:
ADDB DRAW_VEC_Y         ; Add Y offset
LDA ,X+                 ; x_start
TST MIRROR_X
BEQ DSWM_NEXT_NO_NEGATE_X
NEGA
DSWM_NEXT_NO_NEGATE_X:
ADDA DRAW_VEC_X         ; Add X offset
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
; Move to new start position
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
DSWM_W3:
LDA VIA_int_flags
ANDA #$40
BEQ DSWM_W3
CLR VIA_shift_reg
BRA DSWM_LOOP
DSWM_DONE:
RTS
; ============================================================================
; DRAW_CIRCLE_RUNTIME - Draw circle with runtime parameters
; ============================================================================
; Follows Draw_Sync_List_At pattern: read params BEFORE DP change
; Inputs: DRAW_CIRCLE_XC, DRAW_CIRCLE_YC, DRAW_CIRCLE_DIAM, DRAW_CIRCLE_INTENSITY (bytes in RAM)
; Uses 8 segments (octagon) with lookup table for efficiency
DRAW_CIRCLE_RUNTIME:
; Read ALL parameters into registers/stack BEFORE changing DP (critical!)
; (These are byte variables, use LDB not LDD)
LDB DRAW_CIRCLE_INTENSITY
PSHS B                 ; Save intensity on stack

LDB DRAW_CIRCLE_DIAM
SEX                    ; Sign-extend to 16-bit (diameter is unsigned 0..255)
LSRA                   ; Divide by 2 to get radius
RORB
STD DRAW_CIRCLE_TEMP   ; DRAW_CIRCLE_TEMP = radius (16-bit)

LDB DRAW_CIRCLE_XC     ; xc (signed -128..127)
SEX
STD DRAW_CIRCLE_TEMP+2 ; Save xc

LDB DRAW_CIRCLE_YC     ; yc (signed -128..127)
SEX
STD DRAW_CIRCLE_TEMP+4 ; Save yc

; NOW safe to setup BIOS (all params are in DRAW_CIRCLE_TEMP+stack)
LDA #$D0
TFR A,DP
JSR Reset0Ref

; Set intensity (from stack)
PULS A                 ; Get intensity from stack
CMPA #$5F
BEQ DCR_intensity_5F
JSR Intensity_a
BRA DCR_after_intensity
DCR_intensity_5F:
JSR Intensity_5F
DCR_after_intensity:

; Move to start position: (xc + radius, yc)
; radius = DRAW_CIRCLE_TEMP, xc = DRAW_CIRCLE_TEMP+2, yc = DRAW_CIRCLE_TEMP+4
LDD DRAW_CIRCLE_TEMP   ; D = radius
ADDD DRAW_CIRCLE_TEMP+2 ; D = xc + radius
TFR B,B                ; Keep X in B (low byte)
PSHS B                 ; Save X on stack
LDD DRAW_CIRCLE_TEMP+4 ; Load yc
TFR B,A                ; Y to A
PULS B                 ; X to B
JSR Moveto_d

; Loop through 8 segments using lookup table
LDX #DCR_DELTA_TABLE   ; Point to delta table
LDB #8                 ; 8 segments
PSHS B                 ; Save counter on stack

DCR_LOOP:
CLR Vec_Misc_Count     ; Relative drawing

; Load delta multipliers from table
LDA ,X+                ; dx multiplier (-1, 0, 1, or 2 for half)
LDB ,X+                ; dy multiplier
PSHS A,B               ; Save multipliers

; Calculate dy = (dy_mult * radius) / 2 if needed
LDD DRAW_CIRCLE_TEMP   ; Load radius
PULS A,B               ; Get multipliers (A=dx_mult, B=dy_mult)
PSHS A                 ; Save dx_mult

; Process dy_mult
TSTB
BEQ DCR_dy_zero        ; dy = 0
CMPB #2
BEQ DCR_dy_half        ; dy = r/2
CMPB #$FE              ; -2 (half negative)
BEQ DCR_dy_neg_half
CMPB #1
BEQ DCR_dy_pos         ; dy = r
; dy = -r
LDD DRAW_CIRCLE_TEMP
NEGA
NEGB
SBCA #0
BRA DCR_dy_done
DCR_dy_zero:
LDD #0                 ; Clear both A and B
BRA DCR_dy_done
DCR_dy_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
BRA DCR_dy_done
DCR_dy_neg_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
NEGA
NEGB
SBCA #0
BRA DCR_dy_done
DCR_dy_pos:
LDD DRAW_CIRCLE_TEMP
DCR_dy_done:
TFR B,A                ; Move dy result to A (we only need 8-bit for Vectrex coordinates)
PSHS A                 ; Save dy on stack

; Process dx_mult (same logic)
LDB 1,S                ; Get dx_mult from stack
TSTB
BEQ DCR_dx_zero
CMPB #2
BEQ DCR_dx_half
CMPB #$FE
BEQ DCR_dx_neg_half
CMPB #1
BEQ DCR_dx_pos
; dx = -r
LDD DRAW_CIRCLE_TEMP
NEGA
NEGB
SBCA #0
BRA DCR_dx_done
DCR_dx_zero:
LDD #0                 ; Clear both A and B
BRA DCR_dx_done
DCR_dx_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
BRA DCR_dx_done
DCR_dx_neg_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
NEGA
NEGB
SBCA #0
BRA DCR_dx_done
DCR_dx_pos:
LDD DRAW_CIRCLE_TEMP
DCR_dx_done:
TFR B,B                ; dx in B
PULS A                 ; dy in A
LEAS 1,S               ; Drop dx_mult

; Draw line with calculated deltas (preserve X - it points to table)
PSHS X                 ; Save table pointer
JSR Draw_Line_d
PULS X                 ; Restore table pointer

; Loop control
DEC ,S                 ; Decrement counter
BNE DCR_LOOP

LEAS 1,S               ; Clean counter from stack

; DP is ALREADY $D0 from BIOS, no need to restore (Draw_Sync_List_At doesn't restore either)
RTS

RTS

; Delta multiplier table: 8 segments (dx_mult, dy_mult)
; 0=zero, 1=r, -1=$FF=-r, 2=r/2, -2=$FE=-r/2
DCR_DELTA_TABLE:
FCB 2,2      ; Seg 1: dx=r/2, dy=r/2 (right-up)
FCB 0,1      ; Seg 2: dx=0, dy=r (up)
FCB $FE,2    ; Seg 3: dx=-r/2, dy=r/2 (left-up)
FCB $FF,0    ; Seg 4: dx=-r, dy=0 (left)
FCB $FE,$FE  ; Seg 5: dx=-r/2, dy=-r/2 (left-down)
FCB 0,$FF    ; Seg 6: dx=0, dy=-r (down)
FCB 2,$FE    ; Seg 7: dx=r/2, dy=-r/2 (right-down)
FCB 1,0      ; Seg 8: dx=r, dy=0 (right)

START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:3
    LDD #0
    STD VAR_JOY_X
    ; VPy_LINE:4
    LDD #0
    STD VAR_JOY_Y
    ; VPy_LINE:5
    LDD #0
    STD VAR_JOY_X_RAW
    ; VPy_LINE:6
    LDD #0
    STD VAR_JOY_Y_RAW
    ; VPy_LINE:9
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 9
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
; VPy_LINE:8

MAIN:
    JSR $F1AF    ; DP_to_C8 (required for RAM access)
    ; === Initialize Joystick (one-time setup) ===
    CLR $C823    ; CRITICAL: Clear analog mode flag (Joy_Analog does DEC on this)
    LDA #$01     ; CRITICAL: Resolution threshold (power of 2: $40=fast, $01=accurate)
    STA $C81A    ; Vec_Joy_Resltn (loop terminates when B=this value after LSRBs)
    LDA #$01
    STA $C81F    ; Vec_Joy_Mux_1_X (enable X axis reading)
    LDA #$03
    STA $C820    ; Vec_Joy_Mux_1_Y (enable Y axis reading)
    LDA #$00
    STA $C821    ; Vec_Joy_Mux_2_X (disable joystick 2 - CRITICAL!)
    STA $C822    ; Vec_Joy_Mux_2_Y (disable joystick 2 - saves cycles)
    ; Mux configured - J1_X()/J1_Y() can now be called

    ; JSR Wait_Recal is now called at start of LOOP_BODY (see auto-inject)
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    BRA MAIN

    ; VPy_LINE:11
LOOP_BODY:
    ; DEBUG: Statement 0 - Discriminant(8)
    ; VPy_LINE:14
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 14
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(8)
    ; VPy_LINE:15
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_6
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 15
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 2 - Discriminant(8)
    ; VPy_LINE:16
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #85
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_2
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 16
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 3 - Discriminant(8)
    ; VPy_LINE:17
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #85
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_NUMBER at line 17
    JSR VECTREX_PRINT_NUMBER
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 4 - Discriminant(8)
    ; VPy_LINE:18
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #70
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_4
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 18
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 5 - Discriminant(8)
    ; VPy_LINE:19
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #70
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #5
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_NUMBER at line 19
    JSR VECTREX_PRINT_NUMBER
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 6 - Discriminant(8)
    ; VPy_LINE:20
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #55
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_1
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 20
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 7 - Discriminant(8)
    ; VPy_LINE:21
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #55
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #-5
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_NUMBER at line 21
    JSR VECTREX_PRINT_NUMBER
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 8 - Discriminant(8)
    ; VPy_LINE:22
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_3
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 22
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 9 - Discriminant(8)
    ; VPy_LINE:23
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_NUMBER at line 23
    JSR VECTREX_PRINT_NUMBER
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 10 - Discriminant(8)
    ; VPy_LINE:24
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #25
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_0
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 24
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 11 - Discriminant(8)
    ; VPy_LINE:25
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #25
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #-127
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_NUMBER at line 25
    JSR VECTREX_PRINT_NUMBER
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 12 - Discriminant(0)
    ; VPy_LINE:28
; NATIVE_CALL: J1_X at line 28
    JSR J1X_BUILTIN
    STD RESULT
    LDX RESULT
    LDU #VAR_JOY_X
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 13 - Discriminant(0)
    ; VPy_LINE:29
; NATIVE_CALL: J1_Y at line 29
    JSR J1Y_BUILTIN
    STD RESULT
    LDX RESULT
    LDU #VAR_JOY_Y
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 14 - Discriminant(8)
    ; VPy_LINE:31
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_5
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 31
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 15 - Discriminant(8)
    ; VPy_LINE:32
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-15
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_9
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 32
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 16 - Discriminant(8)
    ; VPy_LINE:33
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-15
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_JOY_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_NUMBER at line 33
    JSR VECTREX_PRINT_NUMBER
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 17 - Discriminant(8)
    ; VPy_LINE:34
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-30
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_10
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 34
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 18 - Discriminant(8)
    ; VPy_LINE:35
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-30
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_JOY_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_NUMBER at line 35
    JSR VECTREX_PRINT_NUMBER
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 19 - Discriminant(9)
    ; VPy_LINE:38
    LDD VAR_JOY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-20
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_4
    LDD #0
    STD RESULT
    BRA CE_5
CT_4:
    LDD #1
    STD RESULT
CE_5:
    LDD RESULT
    BNE OR_TRUE_2
    LDD VAR_JOY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #20
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_6
    LDD #0
    STD RESULT
    BRA CE_7
CT_6:
    LDD #1
    STD RESULT
CE_7:
    LDD RESULT
    BNE OR_TRUE_2
    LDD #0
    STD RESULT
    BRA OR_END_3
OR_TRUE_2:
    LDD #1
    STD RESULT
OR_END_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    ; VPy_LINE:39
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_7
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 39
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_0
IF_NEXT_1:
    ; VPy_LINE:41
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_8
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 41
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
IF_END_0:
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************
; Call argument scratch space (using RESULT area)
VAR_ARG0 EQU $C880+$02
VAR_ARG1 EQU $C880+$04
VAR_ARG2 EQU $C880+$06
VAR_ARG3 EQU $C880+$08

; ========================================
; NO ASSETS EMBEDDED
; All 1 discovered assets are unused in code
; ========================================

; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "-127="
    FCB $80
STR_1:
    FCC "-5="
    FCB $80
STR_2:
    FCC "0="
    FCB $80
STR_3:
    FCC "127="
    FCB $80
STR_4:
    FCC "5="
    FCB $80
STR_5:
    FCC "JOYSTICK:"
    FCB $80
STR_6:
    FCC "KNOWN VALUES:"
    FCB $80
STR_7:
    FCC "MOVE"
    FCB $80
STR_8:
    FCC "STOP"
    FCB $80
STR_9:
    FCC "X="
    FCB $80
STR_10:
    FCC "Y="
    FCB $80
