; --- Motorola 6809 backend (Vectrex) title='UNTITLED' origin=$0000 ---
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
    FCC "UNTITLED"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 65 bytes
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$02   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$04   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$06   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$08   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0A   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0C   ; Pointer temp 2 (for nested array operations) (2 bytes)
MUL_A                EQU $C880+$0E   ; Multiplicand A (2 bytes)
MUL_B                EQU $C880+$10   ; Multiplicand B (2 bytes)
MUL_RES              EQU $C880+$12   ; Multiply result (2 bytes)
MUL_TMP              EQU $C880+$14   ; Multiply temporary (2 bytes)
MUL_CNT              EQU $C880+$16   ; Multiply counter (2 bytes)
TEMP_YX              EQU $C880+$18   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$1A   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$1B   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$1C   ; String buffer for PRINT_NUMBER (2 bytes)
VLINE_DX_16          EQU $C880+$1E   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$20   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$22   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$23   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$24   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$26   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$28   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$29   ; 2-byte vector list (Y|endbit, X) (2 bytes)
VAR_PLAYER_X         EQU $C880+$2B   ; User variable (2 bytes)
VAR_PLAYER_Y         EQU $C880+$2D   ; User variable (2 bytes)
VAR_PLAYER_SIZE      EQU $C880+$2F   ; User variable (2 bytes)
VAR_INPUT_INPUT_RESULT_DATA EQU $C880+$31   ; Array data (2 elements) (4 bytes)
VAR_ARG0             EQU $C880+$35   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$37   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$39   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$3B   ; Function argument 3 (2 bytes)
VAR_ARG4             EQU $C880+$3D   ; Function argument 4 (2 bytes)
VAR_ARG5             EQU $C880+$3F   ; Function argument 5 (2 bytes)

    JMP START

;**** CONST DECLARATIONS (NUMBER-ONLY) ****

;
; ┌─────────────────────────────────────────────────────────────────┐
; │ RUNTIME SECTION - VPy Builtin Helpers & System Functions       │
; │ This section contains reusable code shared across all VPy       │
; │ programs. These helpers are emitted once per compilation unit.  │
; └─────────────────────────────────────────────────────────────────┘
;

; === JOYSTICK BUILTIN SUBROUTINES ===
; J1_X() - Read Joystick 1 X axis (INCREMENTAL - with state preservation)
; Returns: D = raw value from $C81B after Joy_Analog call
J1X_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81B from hardware)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81B)
    LDB $C81B    ; Vec_Joy_1_X (BIOS writes ~$FE at center)
    SEX          ; Sign-extend B to D
    ADDD #2      ; Calibrate center offset
    PULS X       ; Restore X
    RTS

; J1_Y() - Read Joystick 1 Y axis (INCREMENTAL - with state preservation)
; Returns: D = raw value from $C81C after Joy_Analog call
J1Y_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81C from hardware)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81C)
    LDB $C81C    ; Vec_Joy_1_Y (BIOS writes ~$FE at center)
    SEX          ; Sign-extend B to D
    ADDD #2      ; Calibrate center offset
    PULS X       ; Restore X
    RTS

; === BUTTON SYSTEM - BIOS TRANSITIONS ===
; J1_BUTTON_1-4() - Read transition bits from $C811
; Read_Btns (auto-injected) calculates: ~(new) OR Vec_Prev_Btns
; Result: bit=1 ONLY on rising edge (0→1 transition)
; Returns: D = 1 (just pressed), 0 (not pressed or still held)

J1B1_BUILTIN:
    LDA $C811      ; Read transition bits (Vec_Button_1_1)
    ANDA #$01      ; Test bit 0 (Button 1)
    BEQ .J1B1_OFF
    LDD #1         ; Return pressed (rising edge)
    RTS
.J1B1_OFF:
    LDD #0         ; Return not pressed
    RTS

J1B2_BUILTIN:
    LDA $C811
    ANDA #$02      ; Test bit 1 (Button 2)
    BEQ .J1B2_OFF
    LDD #1
    RTS
.J1B2_OFF:
    LDD #0
    RTS

J1B3_BUILTIN:
    LDA $C811
    ANDA #$04      ; Test bit 2 (Button 3)
    BEQ .J1B3_OFF
    LDD #1
    RTS
.J1B3_OFF:
    LDD #0
    RTS

J1B4_BUILTIN:
    LDA $C811
    ANDA #$08      ; Test bit 3 (Button 4)
    BEQ .J1B4_OFF
    LDD #1
    RTS
.J1B4_OFF:
    LDD #0
    RTS

; DRAW_LINE unified wrapper - handles 16-bit signed coordinates
; Args: (x0,y0,x1,y1,intensity) as 16-bit words
; ALWAYS sets intensity. Does NOT reset origin (allows connected lines).
DRAW_LINE_WRAPPER:
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode for vector drawing)
    STA >$D00C     ; VIA_cntl
    ; Set DP to hardware registers
    LDA #$D0
    TFR A,DP
    ; ALWAYS set intensity (no optimization)
    LDA RESULT+8+1  ; intensity (low byte of 16-bit value)
    JSR Intensity_a
    ; Move to start ONCE (y in A, x in B) - use low bytes (8-bit signed -127..+127)
    LDA RESULT+2+1  ; Y start (low byte of 16-bit value)
    LDB RESULT+0+1  ; X start (low byte of 16-bit value)
    JSR Moveto_d
    ; Compute deltas using 16-bit arithmetic
    ; dx = x1 - x0 (treating as signed 16-bit)
    LDD RESULT+4    ; x1 (RESULT+4, 16-bit)
    SUBD RESULT+0   ; subtract x0 (RESULT+0, 16-bit)
    STD VLINE_DX_16 ; Store full 16-bit dx
    ; dy = y1 - y0 (treating as signed 16-bit)
    LDD RESULT+6    ; y1 (RESULT+6, 16-bit)
    SUBD RESULT+2   ; subtract y0 (RESULT+2, 16-bit)
    STD VLINE_DY_16 ; Store full 16-bit dy
    ; SEGMENT 1: Clamp dy to ±127 and draw
    LDD VLINE_DY_16 ; Load full dy
    CMPD #127
    BLE DLW_SEG1_DY_LO
    LDA #127        ; dy > 127: use 127
    BRA DLW_SEG1_DY_READY
DLW_SEG1_DY_LO:
    CMPD #-128
    BGE DLW_SEG1_DY_NO_CLAMP  ; -128 <= dy <= 127: use original (sign-extended)
    LDA #$80        ; dy < -128: use -128
    BRA DLW_SEG1_DY_READY
DLW_SEG1_DY_NO_CLAMP:
    LDA VLINE_DY_16+1  ; Use original low byte (already in valid range)
DLW_SEG1_DY_READY:
    STA VLINE_DY    ; Save clamped dy for segment 1
    ; Clamp dx to ±127
    LDD VLINE_DX_16
    CMPD #127
    BLE DLW_SEG1_DX_LO
    LDB #127        ; dx > 127: use 127
    BRA DLW_SEG1_DX_READY
DLW_SEG1_DX_LO:
    CMPD #-128
    BGE DLW_SEG1_DX_NO_CLAMP  ; -128 <= dx <= 127: use original (sign-extended)
    LDB #$80        ; dx < -128: use -128
    BRA DLW_SEG1_DX_READY
DLW_SEG1_DX_NO_CLAMP:
    LDB VLINE_DX_16+1  ; Use original low byte (already in valid range)
DLW_SEG1_DX_READY:
    STB VLINE_DX    ; Save clamped dx for segment 1
    ; Draw segment 1
    CLR Vec_Misc_Count
    LDA VLINE_DY
    LDB VLINE_DX
    JSR Draw_Line_d ; Beam moves automatically
    ; Check if we need SEGMENT 2 (dy outside ±127 range)
    LDD VLINE_DY_16 ; Reload original dy
    CMPD #127
    BGT DLW_NEED_SEG2  ; dy > 127: needs segment 2
    CMPD #-128
    BLT DLW_NEED_SEG2  ; dy < -128: needs segment 2
    BRA DLW_DONE       ; dy in range ±127: no segment 2
DLW_NEED_SEG2:
    ; SEGMENT 2: Draw remaining dy and dx
    ; Calculate remaining dy
    LDD VLINE_DY_16 ; Load original full dy
    CMPD #127
    BGT DLW_SEG2_DY_POS  ; dy > 127
    ; dy < -128, so we drew -128 in segment 1
    ; remaining = dy - (-128) = dy + 128
    ADDD #128       ; Add back the -128 we already drew
    BRA DLW_SEG2_DY_DONE
DLW_SEG2_DY_POS:
    ; dy > 127, so we drew 127 in segment 1
    ; remaining = dy - 127
    SUBD #127       ; Subtract 127 we already drew
DLW_SEG2_DY_DONE:
    STD VLINE_DY_REMAINING  ; Store remaining dy (16-bit)
    ; Calculate remaining dx
    LDD VLINE_DX_16 ; Load original full dx
    CMPD #127
    BLE DLW_SEG2_DX_CHECK_NEG
    ; dx > 127, so we drew 127 in segment 1
    ; remaining = dx - 127
    SUBD #127
    BRA DLW_SEG2_DX_DONE
DLW_SEG2_DX_CHECK_NEG:
    CMPD #-128
    BGE DLW_SEG2_DX_NO_REMAIN  ; -128 <= dx <= 127: no remaining dx
    ; dx < -128, so we drew -128 in segment 1
    ; remaining = dx - (-128) = dx + 128
    ADDD #128
    BRA DLW_SEG2_DX_DONE
DLW_SEG2_DX_NO_REMAIN:
    LDD #0          ; No remaining dx
DLW_SEG2_DX_DONE:
    STD VLINE_DX_REMAINING  ; Store remaining dx (16-bit) in VLINE_DX_REMAINING
    ; Setup for Draw_Line_d: A=dy, B=dx (CRITICAL: order matters!)
    ; Load remaining dy from VLINE_DY_REMAINING (already saved)
    LDA VLINE_DY_REMAINING+1  ; Low byte of remaining dy
    LDB VLINE_DX_REMAINING+1  ; Low byte of remaining dx
    CLR Vec_Misc_Count
    JSR Draw_Line_d ; Beam continues from segment 1 endpoint
DLW_DONE:
    LDA #$C8       ; CRITICAL: Restore DP to $C8 for our code
    TFR A,DP
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
;
; ┌─────────────────────────────────────────────────────────────────┐
; │ PROGRAM CODE SECTION - User VPy Code                            │
; │ This section contains the compiled user program logic.          │
; └─────────────────────────────────────────────────────────────────┘
;

START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
    CLR $C80E        ; Initialize Vec_Prev_Btns to 0 for Read_Btns debounce
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:11
    ; VPy_LINE:7
    LDD #0
    STD VAR_PLAYER_X
    ; VPy_LINE:8
    LDD #0
    STD VAR_PLAYER_Y
    ; VPy_LINE:9
    LDD #10
    STD VAR_PLAYER_SIZE
    ; VPy_LINE:4
    ; Copy array 'input_input_result' from ROM to RAM (2 elements)
    LDX #ARRAY_INPUT_INPUT_RESULT       ; Source: ROM array data
    LDU #VAR_INPUT_INPUT_RESULT_DATA ; Dest: RAM array space
    LDD #2        ; Number of elements
COPY_LOOP_INPUT_INPUT_RESULT:
    LDY ,X++        ; Load word from ROM, increment source
    STY ,U++        ; Store word to RAM, increment dest
    SUBD #1         ; Decrement counter
    BNE COPY_LOOP_INPUT_INPUT_RESULT ; Loop until done
    ; VPy_LINE:13
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 13
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT

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

    ; VPy_LINE:3
GRAPHICS_DRAW_BOX: ; function
; --- function graphics_draw_box ---
    LEAS -10,S ; allocate locals
    LDD VAR_ARG0
    STD 0,S ; param 0
    LDD VAR_ARG1
    STD 2,S ; param 1
    LDD VAR_ARG2
    STD 4,S ; param 2
    LDD VAR_ARG3
    STD 6,S ; param 3
    ; VPy_LINE:6
    LDD 0 ,S
    STD RESULT
    STD TMPPTR+0
    LDD 2 ,S
    STD RESULT
    STD TMPPTR+2
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD 6 ,S
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    STD TMPPTR+4
    LDD 2 ,S
    STD RESULT
    STD TMPPTR+6
    LDD 8 ,S
    STD RESULT
    STD TMPPTR+8
    LDD TMPPTR+0
    STD RESULT+0
    LDD TMPPTR+2
    STD RESULT+2
    LDD TMPPTR+4
    STD RESULT+4
    LDD TMPPTR+6
    STD RESULT+6
    LDD TMPPTR+8
    STD RESULT+8
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; VPy_LINE:8
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD 6 ,S
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    STD TMPPTR+0
    LDD 2 ,S
    STD RESULT
    STD TMPPTR+2
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD 6 ,S
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    STD TMPPTR+4
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD 8 ,S
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    STD TMPPTR+6
    LDD 8 ,S
    STD RESULT
    STD TMPPTR+8
    LDD TMPPTR+0
    STD RESULT+0
    LDD TMPPTR+2
    STD RESULT+2
    LDD TMPPTR+4
    STD RESULT+4
    LDD TMPPTR+6
    STD RESULT+6
    LDD TMPPTR+8
    STD RESULT+8
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; VPy_LINE:10
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD 6 ,S
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    STD TMPPTR+0
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD 8 ,S
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    STD TMPPTR+2
    LDD 0 ,S
    STD RESULT
    STD TMPPTR+4
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD 8 ,S
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    STD TMPPTR+6
    LDD 8 ,S
    STD RESULT
    STD TMPPTR+8
    LDD TMPPTR+0
    STD RESULT+0
    LDD TMPPTR+2
    STD RESULT+2
    LDD TMPPTR+4
    STD RESULT+4
    LDD TMPPTR+6
    STD RESULT+6
    LDD TMPPTR+8
    STD RESULT+8
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; VPy_LINE:12
    LDD 0 ,S
    STD RESULT
    STD TMPPTR+0
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD 8 ,S
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    STD TMPPTR+2
    LDD 0 ,S
    STD RESULT
    STD TMPPTR+4
    LDD 2 ,S
    STD RESULT
    STD TMPPTR+6
    LDD 8 ,S
    STD RESULT
    STD TMPPTR+8
    LDD TMPPTR+0
    STD RESULT+0
    LDD TMPPTR+2
    STD RESULT+2
    LDD TMPPTR+4
    STD RESULT+4
    LDD TMPPTR+6
    STD RESULT+6
    LDD TMPPTR+8
    STD RESULT+8
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LEAS 10,S ; free locals
    RTS

    ; VPy_LINE:14
GRAPHICS_DRAW_CROSS: ; function
; --- function graphics_draw_cross ---
    LEAS -8,S ; allocate locals
    LDD VAR_ARG0
    STD 0,S ; param 0
    LDD VAR_ARG1
    STD 2,S ; param 1
    LDD VAR_ARG2
    STD 4,S ; param 2
    LDD VAR_ARG3
    STD 6,S ; param 3
    ; VPy_LINE:16
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    STD TMPPTR+0
    LDD 2 ,S
    STD RESULT
    STD TMPPTR+2
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    STD TMPPTR+4
    LDD 2 ,S
    STD RESULT
    STD TMPPTR+6
    LDD 6 ,S
    STD RESULT
    STD TMPPTR+8
    LDD TMPPTR+0
    STD RESULT+0
    LDD TMPPTR+2
    STD RESULT+2
    LDD TMPPTR+4
    STD RESULT+4
    LDD TMPPTR+6
    STD RESULT+6
    LDD TMPPTR+8
    STD RESULT+8
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; VPy_LINE:17
    LDD 0 ,S
    STD RESULT
    STD TMPPTR+0
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    STD TMPPTR+2
    LDD 0 ,S
    STD RESULT
    STD TMPPTR+4
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    STD TMPPTR+6
    LDD 6 ,S
    STD RESULT
    STD TMPPTR+8
    LDD TMPPTR+0
    STD RESULT+0
    LDD TMPPTR+2
    STD RESULT+2
    LDD TMPPTR+4
    STD RESULT+4
    LDD TMPPTR+6
    STD RESULT+6
    LDD TMPPTR+8
    STD RESULT+8
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LEAS 8,S ; free locals
    RTS

    ; VPy_LINE:15
LOOP_BODY:
    LEAS -4,S ; allocate locals
    JSR Wait_Recal  ; CRITICAL: Sync with CRT refresh (50Hz frame timing)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; DEBUG: Statement 0 - Discriminant(8)
    ; VPy_LINE:17
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 17
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(8)
    ; VPy_LINE:20
    JSR INPUT_GET_INPUT
    ; DEBUG: Statement 2 - Discriminant(0)
    ; VPy_LINE:21
    LDD #VAR_INPUT_INPUT_RESULT_DATA
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
    LDX RESULT
    STX 0 ,S
    ; DEBUG: Statement 3 - Discriminant(0)
    ; VPy_LINE:22
    LDD #VAR_INPUT_INPUT_RESULT_DATA
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
    LDX RESULT
    STX 2 ,S
    ; DEBUG: Statement 4 - Discriminant(0)
    ; VPy_LINE:23
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 5 - Discriminant(0)
    ; VPy_LINE:24
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD 4 ,S
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_Y
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 6 - Discriminant(9)
    ; VPy_LINE:27
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_2
    LDD #0
    STD RESULT
    BRA CE_3
CT_2:
    LDD #1
    STD RESULT
CE_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    ; VPy_LINE:28
    LDD #-100
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    ; DEBUG: Statement 7 - Discriminant(9)
    ; VPy_LINE:29
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #100
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
    LBEQ IF_NEXT_5
    ; VPy_LINE:30
    LDD #100
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    ; DEBUG: Statement 8 - Discriminant(9)
    ; VPy_LINE:31
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_10
    LDD #0
    STD RESULT
    BRA CE_11
CT_10:
    LDD #1
    STD RESULT
CE_11:
    LDD RESULT
    LBEQ IF_NEXT_9
    ; VPy_LINE:32
    LDD #-100
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_8
IF_NEXT_9:
IF_END_8:
    ; DEBUG: Statement 9 - Discriminant(9)
    ; VPy_LINE:33
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #100
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
    ; VPy_LINE:34
    LDD #100
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_12
IF_NEXT_13:
IF_END_12:
    ; DEBUG: Statement 10 - Discriminant(8)
    ; VPy_LINE:37
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD VAR_PLAYER_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD VAR_PLAYER_SIZE
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_PLAYER_SIZE
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_PLAYER_SIZE
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR GRAPHICS_DRAW_BOX
    ; DEBUG: Statement 11 - Discriminant(9)
    ; VPy_LINE:40
    JSR INPUT_CHECK_FIRE
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_18
    LDD #0
    STD RESULT
    BRA CE_19
CT_18:
    LDD #1
    STD RESULT
CE_19:
    LDD RESULT
    LBEQ IF_NEXT_17
    ; VPy_LINE:41
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    JSR GRAPHICS_DRAW_CROSS
    LBRA IF_END_16
IF_NEXT_17:
IF_END_16:
    LEAS 4,S ; free locals
    RTS

    ; VPy_LINE:6
INPUT_GET_INPUT: ; function
; --- function input_get_input ---
    LEAS -4,S ; allocate locals
    ; VPy_LINE:8
; NATIVE_CALL: J1_X at line 8
    JSR J1X_BUILTIN
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:9
; NATIVE_CALL: J1_Y at line 9
    JSR J1Y_BUILTIN
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:10
    LDD #0
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_INPUT_INPUT_RESULT_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD 0 ,S
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:11
    LDD #1
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_INPUT_INPUT_RESULT_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD 2 ,S
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:12
    LDD #0
    STD RESULT
    LEAS 4 ,S ; free locals
    RTS

    ; VPy_LINE:14
INPUT_CHECK_FIRE: ; function
; --- function input_check_fire ---
    ; VPy_LINE:16
; NATIVE_CALL: J1_BUTTON_1 at line 16
    JSR J1B1_BUILTIN
    STD RESULT
    RTS

MUL16:
    LDD MUL_A
    STD MUL_RES
    LDD #0
    STD MUL_TMP
    LDD MUL_B
    STD MUL_CNT
MUL16_LOOP:
    LDD MUL_CNT
    BEQ MUL16_DONE
    LDD MUL_CNT
    ANDA #1
    BEQ MUL16_SKIP
    LDD MUL_RES
    ADDD MUL_TMP
    STD MUL_TMP
MUL16_SKIP:
    LDD MUL_RES
    ASLB
    ROLA
    STD MUL_RES
    LDD MUL_CNT
    LSRA
    RORB
    STD MUL_CNT
    BRA MUL16_LOOP
MUL16_DONE:
    LDD MUL_TMP
    STD RESULT
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************
; Array literal for variable 'input_input_result' (2 elements)
ARRAY_INPUT_INPUT_RESULT:
    FDB 0   ; Element 0
    FDB 0   ; Element 1

; === INLINE ARRAY LITERALS (from function bodies) ===
