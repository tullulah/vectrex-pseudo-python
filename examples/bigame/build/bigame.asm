; --- Motorola 6809 backend (Vectrex) title='BigGame - Multi-Bank Test' origin=$0000 ---
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
    FDB $0000
    FCB $F8
    FCB $50
    FCB $20
    FCB $BB
    FCC "BIGGAME   MULTI BANK TES"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 53 bytes
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$02   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$04   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$06   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$08   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0A   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0C   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0E   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$10   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$11   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$12   ; String buffer for PRINT_NUMBER (2 bytes)
VLINE_DX_16          EQU $C880+$14   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$16   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$18   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$19   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$1A   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$1C   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$1E   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$1F   ; 2-byte vector list (Y|endbit, X) (2 bytes)
VAR_CURRENT_LEVEL    EQU $C880+$21   ; User variable (2 bytes)
VAR_PLAYER_SCORE     EQU $C880+$23   ; User variable (2 bytes)
VAR_PLAYER_LIVES     EQU $C880+$25   ; User variable (2 bytes)
VAR_GAME_STATE       EQU $C880+$27   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$29   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$2B   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$2D   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$2F   ; Function argument 3 (2 bytes)
VAR_ARG4             EQU $C880+$31   ; Function argument 4 (2 bytes)
VAR_ARG5             EQU $C880+$33   ; Function argument 5 (2 bytes)

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
    ; VPy_LINE:147
    ; VPy_LINE:138
    LDD #0
    STD VAR_CURRENT_LEVEL
    ; VPy_LINE:139
    LDD #0
    STD VAR_PLAYER_SCORE
    ; VPy_LINE:140
    LDD #3
    STD VAR_PLAYER_LIVES
    ; VPy_LINE:141
    LDD #0
    STD VAR_GAME_STATE
    ; VPy_LINE:148
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 148
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


; ================================================
; BANK #31 - 2 function(s)
; ================================================
    ORG $4000  ; Fixed bank (always visible)

    ; VPy_LINE:154
LOOP_BODY:
    JSR Wait_Recal  ; CRITICAL: Sync with CRT refresh (50Hz frame timing)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; VPy_LINE:157
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 157
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:158
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-60
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_0
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 158
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:159
    LDD #65456
    STD TMPPTR+0
    LDD #50
    STD TMPPTR+2
    LDD #80
    STD TMPPTR+4
    LDD #50
    STD TMPPTR+6
    LDD #127
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
    ; VPy_LINE:162
    JSR level1_init_BANK_WRAPPER
    ; VPy_LINE:163
    JSR level1_render_BANK_WRAPPER
    ; VPy_LINE:164
    JSR level2_init_BANK_WRAPPER
    ; VPy_LINE:165
    JSR level2_render_BANK_WRAPPER
    RTS


; ================================================
; BANK #0 - 24 function(s)
; ================================================
    ORG $0000  ; Banked window (switchable)

    ; VPy_LINE:14
LEVEL1_INIT: ; function
; --- function level1_init ---
    ; VPy_LINE:15
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 15
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:16
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #50
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
    RTS

    ; VPy_LINE:18
LEVEL1_UPDATE: ; function
; --- function level1_update ---
    LEAS -4,S ; allocate locals
    ; VPy_LINE:19
    LDD #10
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:20
    LDD #20
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:21
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
    LDD #20
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
    LDD #20
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    STD TMPPTR+6
    LDD #80
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
    LEAS 4,S ; free locals
    RTS

    ; VPy_LINE:23
LEVEL1_RENDER: ; function
; --- function level1_render ---
    ; VPy_LINE:24
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 24
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:25
    LDA #$D0
    TFR A,DP
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$3C
    LDB #$3C
    JSR Draw_Line_d
    LDA #$C8
    TFR A,DP
    LDD #0
    STD RESULT
    RTS

    ; VPy_LINE:27
LEVEL1_ENEMY1: ; function
; --- function level1_enemy1 ---
    LEAS -4,S ; allocate locals
    ; VPy_LINE:28
    LDD #0
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:29
    LDD #0
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:30
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    STX 0 ,S
    LEAS 4,S ; free locals
    RTS

    ; VPy_LINE:32
LEVEL1_ENEMY2: ; function
; --- function level1_enemy2 ---
    LEAS -4,S ; allocate locals
    ; VPy_LINE:33
    LDD #10
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:34
    LDD #10
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:35
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    STX 0 ,S
    LEAS 4,S ; free locals
    RTS

    ; VPy_LINE:37
LEVEL1_COLLISION: ; function
; --- function level1_collision ---
    LEAS -2,S ; allocate locals
    ; VPy_LINE:38
    LDD #0
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:39
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_2
    LDD #0
    STD RESULT
    BRA CE_3
CT_2:
    LDD #1
    STD RESULT
CE_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    ; VPy_LINE:40
    LDD #1
    STD RESULT
    LDX RESULT
    STX 0 ,S
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    LEAS 2,S ; free locals
    RTS

    ; VPy_LINE:46
LEVEL2_INIT: ; function
; --- function level2_init ---
    ; VPy_LINE:47
    LDD #110
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 47
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:48
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-50
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
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 48
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    RTS

    ; VPy_LINE:50
LEVEL2_UPDATE: ; function
; --- function level2_update ---
    LEAS -4,S ; allocate locals
    ; VPy_LINE:51
    LDD #15
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:52
    LDD #25
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:53
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
    LDD #25
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
    LDD #25
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    STD TMPPTR+6
    LDD #90
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
    LEAS 4,S ; free locals
    RTS

    ; VPy_LINE:55
LEVEL2_RENDER: ; function
; --- function level2_render ---
    ; VPy_LINE:56
    LDD #120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 56
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:57
    LDA #$D0
    TFR A,DP
    LDA #$6E
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$50
    LDB #$50
    JSR Draw_Line_d
    LDA #$C8
    TFR A,DP
    LDD #0
    STD RESULT
    RTS

    ; VPy_LINE:59
LEVEL2_ENEMY1: ; function
; --- function level2_enemy1 ---
    LEAS -4,S ; allocate locals
    ; VPy_LINE:60
    LDD #5
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:61
    LDD #5
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:62
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    STX 0 ,S
    LEAS 4,S ; free locals
    RTS

    ; VPy_LINE:64
LEVEL2_ENEMY2: ; function
; --- function level2_enemy2 ---
    LEAS -4,S ; allocate locals
    ; VPy_LINE:65
    LDD #15
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:66
    LDD #15
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:67
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    STX 0 ,S
    LEAS 4,S ; free locals
    RTS

    ; VPy_LINE:69
LEVEL2_BOSS: ; function
; --- function level2_boss ---
    LEAS -6,S ; allocate locals
    ; VPy_LINE:70
    LDD #0
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:71
    LDD #0
    STD RESULT
    LDX RESULT
    STX 4 ,S
    ; VPy_LINE:72
    LDD #100
    STD RESULT
    LDX RESULT
    STX 0 ,S
    LEAS 6,S ; free locals
    RTS

    ; VPy_LINE:78
LEVEL3_INIT: ; function
; --- function level3_init ---
    ; VPy_LINE:79
    LDD #120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 79
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:80
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #30
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_4
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 80
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    RTS

    ; VPy_LINE:82
LEVEL3_UPDATE: ; function
; --- function level3_update ---
    LEAS -4,S ; allocate locals
    ; VPy_LINE:83
    LDD #20
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:84
    LDD #30
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:85
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
    LDD #30
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
    LDD #30
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    STD TMPPTR+6
    LDD #100
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
    LEAS 4,S ; free locals
    RTS

    ; VPy_LINE:87
LEVEL3_RENDER: ; function
; --- function level3_render ---
    ; VPy_LINE:88
    LDD #110
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 88
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:89
    LDA #$D0
    TFR A,DP
    LDA #$78
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$64
    LDB #$64
    JSR Draw_Line_d
    LDA #$C8
    TFR A,DP
    LDD #0
    STD RESULT
    RTS

    ; VPy_LINE:91
LEVEL3_ENEMY1: ; function
; --- function level3_enemy1 ---
    LEAS -4,S ; allocate locals
    ; VPy_LINE:92
    LDD #10
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:93
    LDD #10
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:94
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    STX 0 ,S
    LEAS 4,S ; free locals
    RTS

    ; VPy_LINE:96
LEVEL3_ENEMY2: ; function
; --- function level3_enemy2 ---
    LEAS -4,S ; allocate locals
    ; VPy_LINE:97
    LDD #20
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:98
    LDD #20
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:99
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    STX 0 ,S
    LEAS 4,S ; free locals
    RTS

    ; VPy_LINE:101
LEVEL3_BOSS: ; function
; --- function level3_boss ---
    LEAS -6,S ; allocate locals
    ; VPy_LINE:102
    LDD #0
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:103
    LDD #0
    STD RESULT
    LDX RESULT
    STX 4 ,S
    ; VPy_LINE:104
    LDD #150
    STD RESULT
    LDX RESULT
    STX 0 ,S
    LEAS 6,S ; free locals
    RTS

    ; VPy_LINE:110
MENU_INIT: ; function
; --- function menu_init ---
    ; VPy_LINE:111
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 111
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:112
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-60
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_6
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 112
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    RTS

    ; VPy_LINE:114
MENU_UPDATE: ; function
; --- function menu_update ---
    LEAS -4,S ; allocate locals
    ; VPy_LINE:115
; NATIVE_CALL: J1_X at line 115
    JSR J1X_BUILTIN
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:116
; NATIVE_CALL: J1_Y at line 116
    JSR J1Y_BUILTIN
    STD RESULT
    LDX RESULT
    STX 2 ,S
    LEAS 4,S ; free locals
    RTS

    ; VPy_LINE:118
MENU_RENDER: ; function
; --- function menu_render ---
    ; VPy_LINE:119
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 119
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:120
    LDA #$D0
    TFR A,DP
    LDA #$7F
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$78
    JSR Draw_Line_d
    LDA #$C8
    TFR A,DP
    LDD #0
    STD RESULT
    RTS

    ; VPy_LINE:126
HUD_DRAW_SCORE: ; function
; --- function hud_draw_score ---
    ; VPy_LINE:127
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_7
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 127
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    RTS

    ; VPy_LINE:129
HUD_DRAW_LIVES: ; function
; --- function hud_draw_lives ---
    ; VPy_LINE:130
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #90
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_5
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 130
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    RTS

    ; VPy_LINE:132
HUD_DRAW_LEVEL: ; function
; --- function hud_draw_level ---
    ; VPy_LINE:133
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_1
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 133
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    RTS


; ===== CROSS-BANK CALL WRAPPERS =====
; Auto-generated wrappers for bank switching


; Cross-bank wrapper for level2_init (Bank #0)
level2_init_BANK_WRAPPER:
    PSHS A              ; Save A register
    LDA $4000         ; Read current bank register
    PSHS A              ; Save current bank on stack
    LDA #0             ; Load target bank ID
    STA $4000         ; Switch to target bank
    JSR LEVEL2_INIT              ; Call real function
    PULS A              ; Restore original bank from stack
    STA $4000         ; Switch back to original bank
    PULS A              ; Restore A register
    RTS

; Cross-bank wrapper for level1_render (Bank #0)
level1_render_BANK_WRAPPER:
    PSHS A              ; Save A register
    LDA $4000         ; Read current bank register
    PSHS A              ; Save current bank on stack
    LDA #0             ; Load target bank ID
    STA $4000         ; Switch to target bank
    JSR LEVEL1_RENDER              ; Call real function
    PULS A              ; Restore original bank from stack
    STA $4000         ; Switch back to original bank
    PULS A              ; Restore A register
    RTS

; Cross-bank wrapper for level1_init (Bank #0)
level1_init_BANK_WRAPPER:
    PSHS A              ; Save A register
    LDA $4000         ; Read current bank register
    PSHS A              ; Save current bank on stack
    LDA #0             ; Load target bank ID
    STA $4000         ; Switch to target bank
    JSR LEVEL1_INIT              ; Call real function
    PULS A              ; Restore original bank from stack
    STA $4000         ; Switch back to original bank
    PULS A              ; Restore A register
    RTS

; Cross-bank wrapper for level2_render (Bank #0)
level2_render_BANK_WRAPPER:
    PSHS A              ; Save A register
    LDA $4000         ; Read current bank register
    PSHS A              ; Save current bank on stack
    LDA #0             ; Load target bank ID
    STA $4000         ; Switch to target bank
    JSR LEVEL2_RENDER              ; Call real function
    PULS A              ; Restore original bank from stack
    STA $4000         ; Switch back to original bank
    PULS A              ; Restore A register
    RTS
; ===== END CROSS-BANK WRAPPERS =====

;***************************************************************************
; DATA SECTION
;***************************************************************************
; === INLINE ARRAY LITERALS (from function bodies) ===
; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "BIG GAME TEST"
    FCB $80
STR_1:
    FCC "LEVEL"
    FCB $80
STR_2:
    FCC "LEVEL 1"
    FCB $80
STR_3:
    FCC "LEVEL 2"
    FCB $80
STR_4:
    FCC "LEVEL 3"
    FCB $80
STR_5:
    FCC "LIVES"
    FCB $80
STR_6:
    FCC "MAIN MENU"
    FCB $80
STR_7:
    FCC "SCORE"
    FCB $80
