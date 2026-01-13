    INCLUDE "VECTREX.I"
; External symbols (helpers and shared data)
CONST_ARRAY_LOCATION_Y EQU $0000
CONST_ARRAY_LEVEL_SPEED EQU $0000
STR_1 EQU $0000
J1X_BUILTIN EQU $0000
VECTREX_SET_INTENSITY EQU $0000
CONST_ARRAY_LEVEL_COUNT EQU $0000
MAIN EQU $0000
STR_0 EQU $0000
VECTREX_WAIT_RECAL EQU $0000
LOOP_BODY EQU $0000
__Moveto_d EQU $0000
J1B1_BUILTIN EQU $0000
START EQU $0000
CONST_ARRAY_LOCATION_NAMES_STR_0 EQU $0000
J1B2_BUILTIN EQU $0000
CONST_ARRAY_LOCATION_NAMES_STR_1 EQU $0000
CONST_ARRAY_LOCATION_NAMES EQU $0000
J1B4_BUILTIN EQU $0000
__Intensity_a EQU $0000
COPY_LOOP_JOYSTICK1_STATE EQU $0000
ARRAY_JOYSTICK1_STATE EQU $0000
CONST_ARRAY_LOCATION_X EQU $0000
__Reset0Ref EQU $0000
J1Y_BUILTIN EQU $0000
J1B3_BUILTIN EQU $0000
__Draw_Line_d EQU $0000


; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 41 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
VAR_JOYSTICK1_STATE_DATA EQU $C880+$15   ; Array data (6 elements) (12 bytes)
VAR_CURRENT_LOCATION EQU $C880+$21   ; User variable (2 bytes)
VAR_LOCATION_GLOW_INTENSITY EQU $C880+$23   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$25   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$27   ; Function argument 1 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)



; ================================================
; --- Motorola 6809 backend (Vectrex) title='Pattern Test' origin=$0000 ---
        ORG $0000
;***************************************************************************
; DEFINE SECTION
;***************************************************************************

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
    FCC "PATTERN TEST"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************


;**** CONST DECLARATIONS (NUMBER-ONLY) ****
; VPy_LINE:15
; _CONST_DECL_0:  ; const num_locations

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
    LDS #$CBFF       ; Initialize stack at top of RAM (safer than Vec_Default_Stk)
    LDA #0
    STA >CURRENT_ROM_BANK ; Initialize to bank 0 (RAM tracker for debugging)
    ; Note: NOT writing to hardware bank register - already in Bank #0

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:19
    ; VPy_LINE:13
    ; Copy array 'joystick1_state' from ROM to RAM (6 elements)
    LDX #ARRAY_JOYSTICK1_STATE       ; Source: ROM array data
    LDU #VAR_JOYSTICK1_STATE_DATA ; Dest: RAM array space
    LDD #6        ; Number of elements
COPY_LOOP_JOYSTICK1_STATE:
    LDY ,X++        ; Load word from ROM, increment source
    STY ,U++        ; Store word to RAM, increment dest
    SUBD #1         ; Decrement counter
    LBNE COPY_LOOP_JOYSTICK1_STATE ; Loop until done
    ; VPy_LINE:16
    LDD #0
    STD VAR_CURRENT_LOCATION
    ; VPy_LINE:17
    LDD #60
    STD VAR_LOCATION_GLOW_INTENSITY
    ; VPy_LINE:20
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 20
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
    LBRA MAIN


; ================================================

    ORG $0000  ; Sequential bank model

    ; VPy_LINE:22
LOOP_BODY:
    LEAS -2,S ; allocate locals
    JSR Wait_Recal  ; CRITICAL: Sync with CRT refresh (50Hz frame timing)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; VPy_LINE:23
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 23
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:24
    LDD #VAR_JOYSTICK1_STATE_DATA
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
    ; VPy_LINE:25
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
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 25
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    LEAS 2,S ; free locals
    RTS

NUM_LOCATIONS EQU 17
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
    LBEQ .J1B1_OFF
    LDD #1         ; Return pressed (rising edge)
    RTS
.J1B1_OFF:
    LDD #0         ; Return not pressed
    RTS

J1B2_BUILTIN:
    LDA $C811
    ANDA #$02      ; Test bit 1 (Button 2)
    LBEQ .J1B2_OFF
    LDD #1
    RTS
.J1B2_OFF:
    LDD #0
    RTS

J1B3_BUILTIN:
    LDA $C811
    ANDA #$04      ; Test bit 2 (Button 3)
    LBEQ .J1B3_OFF
    LDD #1
    RTS
.J1B3_OFF:
    LDD #0
    RTS

J1B4_BUILTIN:
    LDA $C811
    ANDA #$08      ; Test bit 3 (Button 4)
    LBEQ .J1B4_OFF
    LDD #1
    RTS
.J1B4_OFF:
    LDD #0
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
;***************************************************************************
; DATA SECTION
;***************************************************************************
; Array literal for variable 'joystick1_state' (6 elements)
ARRAY_JOYSTICK1_STATE:
    FDB 0   ; Element 0
    FDB 0   ; Element 1
    FDB 0   ; Element 2
    FDB 0   ; Element 3
    FDB 0   ; Element 4
    FDB 0   ; Element 5

; === INLINE ARRAY LITERALS (from function bodies) ===
; VPy_LINE:6
; Const array literal for 'location_x' (2 elements)
CONST_ARRAY_LOCATION_X:
    FDB 40   ; Element 0
    FDB 40   ; Element 1

; VPy_LINE:7
; Const array literal for 'location_y' (2 elements)
CONST_ARRAY_LOCATION_Y:
    FDB 110   ; Element 0
    FDB 79   ; Element 1

; VPy_LINE:8
; Const string array for 'location_names' (2 strings)
CONST_ARRAY_LOCATION_NAMES_STR_0:
    FCC "LOC1"
    FCB $80   ; String terminator
CONST_ARRAY_LOCATION_NAMES_STR_1:
    FCC "LOC2"
    FCB $80   ; String terminator
CONST_ARRAY_LOCATION_NAMES:  ; Pointer table for location_names
    FDB CONST_ARRAY_LOCATION_NAMES_STR_0  ; Pointer to string
    FDB CONST_ARRAY_LOCATION_NAMES_STR_1  ; Pointer to string

; VPy_LINE:10
; Const array literal for 'level_count' (2 elements)
CONST_ARRAY_LEVEL_COUNT:
    FDB 1   ; Element 0
    FDB 2   ; Element 1

; VPy_LINE:11
; Const array literal for 'level_speed' (2 elements)
CONST_ARRAY_LEVEL_SPEED:
    FDB 1   ; Element 0
    FDB 2   ; Element 1

; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "LOC1"
    FCB $80
STR_1:
    FCC "LOC2"
    FCB $80

; === Multibank Mode: Interrupt Vectors in Bank #31 (Linker) ===
; All vectors handled by multi_bank_linker
; Bank #0-#30: Local 0xFFF0-0xFFFF addresses are unreachable
; Bank #31: Contains complete interrupt vector table (fixed at 0x4000-0x7FFF window)
