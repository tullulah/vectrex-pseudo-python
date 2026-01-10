; --- Motorola 6809 backend (Vectrex) title='Call Graph Test' origin=$0000 ---
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
    FCC "CALL GRAPH TEST"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 24 bytes
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
VAR_ARG0             EQU $C880+$14   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$16   ; Function argument 1 (2 bytes)

    JMP START

;**** CONST DECLARATIONS (NUMBER-ONLY) ****

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

VECTREX_DEBUG_PRINT:
    ; Debug print to console - writes to gap area (C000-C7FF)
    ; Write both high and low bytes for proper 16-bit signed interpretation
    LDA VAR_ARG0     ; Load high byte (for signed interpretation)
    STA $C002        ; Debug output high byte in gap
    LDA VAR_ARG0+1   ; Load low byte
    STA $C000        ; Debug output low byte in unmapped gap
    LDA #$42         ; Debug marker
    STA $C001        ; Debug marker to indicate new output
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
START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
    CLR $C80E        ; Initialize Vec_Prev_Btns to 0 for Read_Btns debounce
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:7
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
    ; VPy_LINE:9
    JSR init_game_BANK_WRAPPER
    ; VPy_LINE:10
    JSR game_loop_BANK_WRAPPER

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

    ; VPy_LINE:12
INIT_GAME: ; function
; --- function init_game ---
    ; VPy_LINE:13
    JSR LOAD_ASSETS
    ; VPy_LINE:14
    JSR SETUP_PLAYER
    RTS

    ; VPy_LINE:16
LOAD_ASSETS: ; function
; --- function load_assets ---
    ; VPy_LINE:17
    LDD #1
    STD RESULT
; NATIVE_CALL: DEBUG_PRINT at line 17
    LDD RESULT
    STA $C002
    STB $C000
    LDA #$42
    STA $C001
    CLR $C003
    CLR $C005
    LDD #0
    STD RESULT
    RTS

    ; VPy_LINE:19
SETUP_PLAYER: ; function
; --- function setup_player ---
    ; VPy_LINE:20
    LDD #2
    STD RESULT
; NATIVE_CALL: DEBUG_PRINT at line 20
    LDD RESULT
    STA $C002
    STB $C000
    LDA #$42
    STA $C001
    CLR $C003
    CLR $C005
    LDD #0
    STD RESULT
    RTS

    ; VPy_LINE:22
GAME_LOOP: ; function
; --- function game_loop ---
    ; VPy_LINE:23
    JSR UPDATE_PLAYER
    ; VPy_LINE:24
    JSR UPDATE_ENEMIES
    ; VPy_LINE:25
    JSR DRAW_ALL
    RTS

    ; VPy_LINE:27
UPDATE_PLAYER: ; function
; --- function update_player ---
    ; VPy_LINE:28
    JSR CHECK_INPUT
    ; VPy_LINE:29
    JSR MOVE_PLAYER
    RTS

    ; VPy_LINE:31
CHECK_INPUT: ; function
; --- function check_input ---
    ; VPy_LINE:32
    LDD #3
    STD RESULT
; NATIVE_CALL: DEBUG_PRINT at line 32
    LDD RESULT
    STA $C002
    STB $C000
    LDA #$42
    STA $C001
    CLR $C003
    CLR $C005
    LDD #0
    STD RESULT
    RTS

    ; VPy_LINE:34
MOVE_PLAYER: ; function
; --- function move_player ---
    ; VPy_LINE:35
    LDD #4
    STD RESULT
; NATIVE_CALL: DEBUG_PRINT at line 35
    LDD RESULT
    STA $C002
    STB $C000
    LDA #$42
    STA $C001
    CLR $C003
    CLR $C005
    LDD #0
    STD RESULT
    RTS

    ; VPy_LINE:37
UPDATE_ENEMIES: ; function
; --- function update_enemies ---
    LEAS -2,S ; allocate locals
    ; VPy_LINE:38
    LDD #0
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:39
WH_0: ; while start
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #10
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
    LBEQ WH_END_1
    ; VPy_LINE:40
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR MOVE_ENEMY
    ; VPy_LINE:41
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
    LBRA WH_0
WH_END_1: ; while end
    LEAS 2,S ; free locals
    RTS

    ; VPy_LINE:43
MOVE_ENEMY: ; function
; --- function move_enemy ---
    LEAS -2,S ; allocate locals
    LDD VAR_ARG0
    STD 0,S ; param 0
    ; VPy_LINE:44
    LDD 0 ,S
    STD RESULT
; NATIVE_CALL: DEBUG_PRINT(enemy_id) at line 44
    LDD RESULT
    STA $C002
    STB $C000
    LDA #$FE
    STA $C001
    LDX #DEBUG_LABEL_ENEMY_ID
    STX $C004
    BRA DEBUG_SKIP_DATA_4
DEBUG_LABEL_ENEMY_ID:
    FCC "enemy_id"
    FCB $00
DEBUG_SKIP_DATA_4:
    LDD #0
    STD RESULT
    LEAS 2,S ; free locals
    RTS

    ; VPy_LINE:46
DRAW_ALL: ; function
; --- function draw_all ---
    ; VPy_LINE:47
    JSR DRAW_PLAYER
    ; VPy_LINE:48
    JSR DRAW_ENEMIES
    RTS

    ; VPy_LINE:50
DRAW_PLAYER: ; function
; --- function draw_player ---
    ; VPy_LINE:52
    LDA #$D0
    TFR A,DP
    LDA #$64
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$28
    JSR Draw_Line_d
    LDA #$C8
    TFR A,DP
    LDD #0
    STD RESULT
    RTS

    ; VPy_LINE:54
DRAW_ENEMIES: ; function
; --- function draw_enemies ---
    LEAS -2,S ; allocate locals
    ; VPy_LINE:55
    LDD #0
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:56
WH_5: ; while start
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_7
    LDD #0
    STD RESULT
    BRA CE_8
CT_7:
    LDD #1
    STD RESULT
CE_8:
    LDD RESULT
    LBEQ WH_END_6
    ; VPy_LINE:58
    LDA #$D0
    TFR A,DP
    LDA #$50
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$14
    JSR Draw_Line_d
    LDA #$C8
    TFR A,DP
    LDD #0
    STD RESULT
    ; VPy_LINE:59
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
    LBRA WH_5
WH_END_6: ; while end
    LEAS 2,S ; free locals
    RTS

    ; VPy_LINE:61
LOOP_BODY:
    JSR Wait_Recal  ; CRITICAL: Sync with CRT refresh (50Hz frame timing)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; DEBUG: Statement 0 - Discriminant(8)
    ; VPy_LINE:62
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 62
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(8)
    ; VPy_LINE:63
    JSR game_loop_BANK_WRAPPER
    RTS


; ===== CROSS-BANK CALL WRAPPERS =====
; Auto-generated wrappers for bank switching


; Cross-bank wrapper for init_game (Bank #0)
init_game_BANK_WRAPPER:
    PSHS A              ; Save A register
    LDA $4000         ; Read current bank register
    PSHS A              ; Save current bank on stack
    LDA #0             ; Load target bank ID
    STA $4000         ; Switch to target bank
    JSR INIT_GAME              ; Call real function
    PULS A              ; Restore original bank from stack
    STA $4000         ; Switch back to original bank
    PULS A              ; Restore A register
    RTS

; Cross-bank wrapper for game_loop (Bank #0)
game_loop_BANK_WRAPPER:
    PSHS A              ; Save A register
    LDA $4000         ; Read current bank register
    PSHS A              ; Save current bank on stack
    LDA #0             ; Load target bank ID
    STA $4000         ; Switch to target bank
    JSR GAME_LOOP              ; Call real function
    PULS A              ; Restore original bank from stack
    STA $4000         ; Switch back to original bank
    PULS A              ; Restore A register
    RTS
; ===== END CROSS-BANK WRAPPERS =====

;***************************************************************************
; DATA SECTION
;***************************************************************************

; ========================================
; NO ASSETS EMBEDDED
; All 2 discovered assets are unused in code
; ========================================

