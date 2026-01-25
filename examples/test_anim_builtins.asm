; --- Motorola 6809 backend (Vectrex) title='Animation Test' origin=$0000 ---
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
    FCC "ANIMATION TEST"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 20 bytes
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPPTR               EQU $C880+$02   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$04   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$06   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$08   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$09   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$0A   ; String buffer for PRINT_NUMBER (2 bytes)
VAR_ARG0             EQU $C880+$0C   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$0E   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$10   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$12   ; Function argument 3 (2 bytes)

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
    ; VPy_LINE:3
    ; VPy_LINE:4
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 4
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

    ; VPy_LINE:6
LOOP_BODY:
    LEAS -2,S ; allocate locals
    JSR Wait_Recal  ; CRITICAL: Sync with CRT refresh (50Hz frame timing)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; DEBUG: Statement 0 - Discriminant(8)
    ; VPy_LINE:7
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 7
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(0)
    ; VPy_LINE:9
; CREATE_ANIM("player_walk") - allocate instance from pool
    LDX #_PLAYER_WALK_ANIM        ; Load animation data pointer
    JSR CREATE_ANIM_RUNTIME  ; Returns instance ID in D
    STD RESULT               ; Store instance ID (0-15 or -1)
    LDX RESULT
    STX 0 ,S
    ; DEBUG: Statement 2 - Discriminant(8)
    ; VPy_LINE:10
; UPDATE_ANIM(instance_id, x, y) - update position + advance frame
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPPTR               ; Save instance_id
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPPTR+2             ; Save X position
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPPTR+4             ; Save Y position
    LDD TMPPTR               ; Instance ID
    LDX TMPPTR+2             ; X position
    LDY TMPPTR+4             ; Y position
    JSR UPDATE_ANIM_RUNTIME  ; Update instance
    LDD #0
    STD RESULT
    ; DEBUG: Statement 3 - Discriminant(8)
    ; VPy_LINE:11
; DRAW_ANIM(instance_id) - render current frame
    LDD 0 ,S
    STD RESULT
    LDD RESULT               ; Instance ID
    JSR DRAW_ANIM_RUNTIME    ; Draw at stored position
    LDD #0
    STD RESULT
    ; DEBUG: Statement 4 - Discriminant(8)
    ; VPy_LINE:12
; SET_ANIM_MIRROR(instance_id, mirror) - set mirror flags
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPPTR               ; Save instance_id
    LDD #0
    STD RESULT
    LDD RESULT               ; Mirror value (0-3)
    LDX TMPPTR               ; Instance ID
    JSR SET_ANIM_MIRROR_RUNTIME
    LDD #0
    STD RESULT
    LEAS 2,S ; free locals
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************

; ========================================
; ASSET DATA SECTION
; Embedded 1 of 1 assets (unused assets excluded)
; ========================================

; Animation Asset: player_walk (from /Users/daniel/projects/vectrex-pseudo-python/examples/assets/animations/player_walk.vanim)
; 5 frame(s), 2 state(s)
; ===== ANIMATION: player_walk =====
; Version: 1.0
; Frames: 5
; States: 2

_PLAYER_WALK_ANIM:
    FCB 5          ; num_frames
    FCB 2          ; num_states
    FCB $01        ; controller_flags (bit 0: mirror_on_left)
    FDB _PLAYER_WALK_FRAMES    ; Pointer to frame table
    FDB _PLAYER_WALK_STATES    ; Pointer to state table

; Frame table for player_walk
_PLAYER_WALK_FRAMES:
    ; Frame 0: walk_1 (duration=5)
_PLAYER_WALK_WALK_1_FRAME:
    FDB _PLAYER_WALK_1_VECTORS     ; Pointer to vector asset
    FDB 5          ; Duration (ticks)
    FCB 80          ; Intensity
    FDB 0          ; Offset X
    FDB 0          ; Offset Y
    FCB 0          ; Mirror mode
    ; Frame 1: walk_2 (duration=5)
_PLAYER_WALK_WALK_2_FRAME:
    FDB _PLAYER_WALK_2_VECTORS     ; Pointer to vector asset
    FDB 5          ; Duration (ticks)
    FCB 80          ; Intensity
    FDB 0          ; Offset X
    FDB 0          ; Offset Y
    FCB 0          ; Mirror mode
    ; Frame 2: walk_3 (duration=5)
_PLAYER_WALK_WALK_3_FRAME:
    FDB _PLAYER_WALK_3_VECTORS     ; Pointer to vector asset
    FDB 5          ; Duration (ticks)
    FCB 80          ; Intensity
    FDB 0          ; Offset X
    FDB 0          ; Offset Y
    FCB 0          ; Mirror mode
    ; Frame 3: walk_4 (duration=5)
_PLAYER_WALK_WALK_4_FRAME:
    FDB _PLAYER_WALK_4_VECTORS     ; Pointer to vector asset
    FDB 5          ; Duration (ticks)
    FCB 80          ; Intensity
    FDB 0          ; Offset X
    FDB 0          ; Offset Y
    FCB 0          ; Mirror mode
    ; Frame 4: walk_5 (duration=5)
_PLAYER_WALK_WALK_5_FRAME:
    FDB _PLAYER_WALK_5_VECTORS     ; Pointer to vector asset
    FDB 5          ; Duration (ticks)
    FCB 80          ; Intensity
    FDB 0          ; Offset X
    FDB 0          ; Offset Y
    FCB 0          ; Mirror mode

; State table for player_walk
_PLAYER_WALK_STATES:
    ; State: walking
_PLAYER_WALK_WALKING_STATE:
    FCB 5          ; num_frames in state
    FCB 1          ; loop_state (0=no, 1=yes)
    FCB 0          ; Frame index: walk_1
    FCB 1          ; Frame index: walk_2
    FCB 2          ; Frame index: walk_3
    FCB 3          ; Frame index: walk_4
    FCB 4          ; Frame index: walk_5
    ; State: idle
_PLAYER_WALK_IDLE_STATE:
    FCB 1          ; num_frames in state
    FCB 1          ; loop_state (0=no, 1=yes)
    FCB 0          ; Frame index: walk_1


