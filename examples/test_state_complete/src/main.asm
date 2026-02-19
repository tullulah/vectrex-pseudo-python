; --- Motorola 6809 backend (Vectrex) title='State Machine Complete Test' origin=$0000 ---
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
    FCC "STATE MACHINE COMPLETE T"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 228 bytes
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
ANIM_POOL            EQU $C880+$12   ; Animation instance pool (16 instances × 12 bytes) (192 bytes)
NUM_STR              EQU $C880+$D2   ; String buffer for PRINT_NUMBER (2 bytes)
VAR_ANIM_ID          EQU $C880+$D4   ; User variable (2 bytes)
VAR_CURRENT_STATE    EQU $C880+$D6   ; User variable (2 bytes)
VAR_FRAME_COUNTER    EQU $C880+$D8   ; User variable (2 bytes)
VAR_JOY_X            EQU $C880+$DA   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$DC   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$DE   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$E0   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$E2   ; Function argument 3 (2 bytes)

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
; CREATE_ANIM_RUNTIME - Find free instance slot and initialize
; Input: X = animation data pointer (ROM)
; Output: D = instance ID (0-15), or -1 if pool full
CREATE_ANIM_RUNTIME:
    ; Search for inactive slot
    PSHS X              ; Save animation pointer
    LDX #ANIM_POOL      ; Start of instance pool
    CLRA                ; Instance ID counter = 0
CAR_SEARCH:
    CMPA #16            ; Checked all 16 instances?
    BEQ CAR_POOL_FULL   ; Yes, pool full
    LDB 10,X            ; Load active flag (8-bit offset mode)
    BEQ CAR_FOUND_SLOT  ; Found inactive slot (B=0)
    LEAX 12,X           ; Move to next instance (12 bytes each)
    INCA                ; Increment ID counter
    BRA CAR_SEARCH
CAR_FOUND_SLOT:
    ; Initialize instance
    PSHS A              ; CRITICAL FIX: Save instance ID (A) before PULS D overwrites it
    PULS D              ; Get animation pointer to D
    STD 0,X             ; Store anim_ptr (offset +0)
    CLR 2,X             ; frame_idx = 0 (offset +2)
    CLR 3,X             ; counter = 0 (offset +3)
    CLR 4,X             ; state_idx = 0 (offset +4)
    CLR 5,X             ; mirror = 0 (offset +5)
    CLR 6,X             ; x = 0 (offset +6, high byte)
    CLR 7,X             ; x = 0 (offset +7, low byte)
    CLR 8,X             ; y = 0 (offset +8, high byte)
    CLR 9,X             ; y = 0 (offset +9, low byte)
    LDB #1
    STB 10,X            ; active = 1 (offset +10)
    ; Return instance ID in D (restore A from stack)
    PULS A              ; FIXED: Restore instance ID
    TFR A,B             ; B = A (make 16-bit: D = A:A)
    CLRA                ; D = 0:ID (16-bit result)
    STD RESULT          ; Store in RESULT
    RTS
CAR_POOL_FULL:
    ; No free slots - return -1
    LDD #$FFFF
    STD RESULT
    PULS X              ; Clean up stack
    RTS

; UPDATE_ANIM_RUNTIME - Update animation position and advance frame
; Input: D = instance_id, X = x_position, Y = y_position
UPDATE_ANIM_RUNTIME:
    ; Validate instance ID (must be 0-15)
    CMPB #16            ; Check if ID >= 16
    BHS UAR_INVALID     ; Invalid ID, return
    
    ; Calculate instance address: ANIM_POOL + (ID * 12)
    PSHS D              ; Save ID
    LDB 1,S             ; Get ID low byte
    LDA #12             ; Instance size
    MUL                 ; D = ID * 12
    ADDD #ANIM_POOL     ; D = instance address
    TFR D,U             ; U = instance pointer
    
    ; Check if instance is active
    TST 10,U            ; Check active flag (offset +10)
    BEQ UAR_INACTIVE    ; Inactive, skip
    
    ; Store new position
    STX 6,U             ; Store x (offset +6, 16-bit)
    STY 8,U             ; Store y (offset +8, 16-bit)
    
    ; Advance frame counter
    INC 3,U             ; counter++ (offset +3)
    
    ; Get current frame data
    LDX 0,U             ; Load anim_ptr (offset +0)
    LDY 3,X             ; Load frame_table_ptr (offset +3 in anim data)
    
    ; Calculate current frame address: frame_table + (frame_idx * 11)
    LDA 2,U             ; Load frame_idx (offset +2)
    LDB #11             ; Frame size
    MUL                 ; D = frame_idx * 11
    LEAY D,Y            ; Y = frame_table + offset
    
    ; Check if counter >= duration
    LDD 2,Y             ; Load duration from frame data (offset +2)
    CMPB 3,U            ; Compare with counter (offset +3)
    BHI UAR_DONE        ; counter < duration, done
    
    ; Advance to next frame
    CLR 3,U             ; Reset counter (offset +3)
    INC 2,U             ; frame_idx++ (offset +2)
    
    ; Check if we reached end of state
    ; If state_num_frames is set (non-zero), use it for bounds checking
    LDA 11,U            ; Load state_num_frames (offset +11)
    BEQ UAR_NO_STATE    ; If 0, no state active - use total frames
    
    ; State-based looping
    CMPA 2,U            ; Compare state_num_frames with frame_idx
    BHI UAR_DONE        ; frame_idx < state_num_frames, continue
    
    ; Reached end of state - loop back to first frame of state
    ; Get state table and read first frame index
    LDX 0,U             ; Load anim_ptr
    LDD 5,X             ; Load state_table_ptr (offset +5 in anim data)
    BEQ UAR_NO_STATE    ; No state table, fall back to simple loop
    
    ; Calculate state entry address
    TFR D,Y             ; Y = state_table_ptr
    LDB 4,U             ; Load current state_idx (offset +4)
    BEQ UAR_STATE_FOUND ; If state 0, already there
    
UAR_STATE_SEEK:
    ; Skip to target state
    LDA 0,Y             ; Load num_frames_in_state
    ADDA #2             ; Add header size
    LEAY A,Y            ; Advance to next state
    DECB
    BNE UAR_STATE_SEEK
    
UAR_STATE_FOUND:
    ; Y points to current state entry
    ; Read first frame index
    LDA 2,Y             ; Load first frame index
    STA 2,U             ; Reset frame_idx to state start
    BRA UAR_DONE
    
UAR_NO_STATE:
    ; No state system - simple loop through all frames
    LDX 0,U             ; Reload anim_ptr
    LDA 0,X             ; Load num_frames (offset +0 in anim data)
    CMPA 2,U            ; Compare with frame_idx
    BHI UAR_DONE        ; frame_idx < num_frames, done
    
    ; Loop back to frame 0
    CLR 2,U             ; frame_idx = 0
    
UAR_DONE:
    PULS D              ; Clean up stack
    RTS
UAR_INVALID:
    RTS                 ; Invalid ID, do nothing
UAR_INACTIVE:
    PULS D              ; Clean up stack
    RTS

; DRAW_ANIM_RUNTIME - Render current frame of animation
; Input: D = instance_id
DRAW_ANIM_RUNTIME:
    ; Validate instance ID
    CMPB #16
    BHS DAR_INVALID
    
    ; Calculate instance address: ANIM_POOL + (ID * 12)
    PSHS D              ; Save ID
    LDB 1,S             ; Get ID low byte
    LDA #12
    MUL                 ; D = ID * 12
    ADDD #ANIM_POOL
    TFR D,U             ; U = instance pointer
    
    ; Check if active
    TST 10,U
    BEQ DAR_INACTIVE
    
    ; Get animation data pointer
    LDX 0,U             ; Load anim_ptr (offset +0)
    LDY 3,X             ; Load frame_table_ptr (offset +3 in anim data)
    
    ; Calculate current frame address: frame_table + (frame_idx * 11)
    LDA 2,U             ; Load frame_idx (offset +2)
    LDB #11
    MUL
    LEAY D,Y            ; Y = current frame data
    
    ; Read frame data for DRAW_VECTOR_EX call
    LDX 0,Y             ; Load vector_ptr (offset +0 in frame)
    PSHS X              ; Save vector_ptr for Draw_VLc call
    
    ; Calculate final position: instance.x + frame.offset_x
    LDD 6,U             ; Load instance x (offset +6)
    ADDD 5,Y            ; Add frame offset_x (offset +5 in frame)
    PSHS D              ; Save final_x
    
    ; Calculate final position: instance.y + frame.offset_y
    LDD 8,U             ; Load instance y (offset +8)
    ADDD 7,Y            ; Add frame offset_y (offset +7 in frame)
    PSHS D              ; Save final_y
    
    ; Get mirror mode from frame (override if instance has custom mirror)
    LDA 9,Y             ; Load frame mirror (offset +9 in frame)
    LDB 5,U             ; Load instance mirror (offset +5)
    TSTB
    BEQ DAR_USE_FRAME_MIRROR
    TFR B,A             ; Use instance mirror if non-zero
DAR_USE_FRAME_MIRROR:
    PSHS A              ; Save mirror
    
    ; Get intensity from frame
    LDA 4,Y             ; Load intensity (offset +4 in frame)
    PSHS A              ; Save intensity
    
    ; Call drawing function (simplified - full implementation would use DRAW_VECTOR_EX)
    ; For now, just set intensity and draw vector at instance position
    
    ; Set intensity
    LDA ,S              ; Get intensity from stack (don't pop yet)
    JSR $F2AB           ; BIOS Intensity_a
    
    ; Move to position (y, x)
    LDA 2,S             ; Get y (second word on stack, high byte)
    LDB 3,S             ; Get x (third word on stack, low byte)
    JSR $F312           ; BIOS Moveto_d
    
    ; Draw vector
    LDX 5,S             ; Get vector_ptr from bottom of stack
    JSR $F408           ; BIOS Draw_VLc
    
    ; Clean up stack (5 words: intensity, mirror, final_y, final_x, vector_ptr)
    LEAS 7,S            ; Pop 7 bytes (1+1+2+2+2 = 8 but S points to next)
    
DAR_DONE:
    PULS D              ; Restore ID from stack
    RTS
DAR_INVALID:
    RTS
DAR_INACTIVE:
    PULS D
    RTS

; SET_ANIM_STATE_RUNTIME - Change animation state (Phase 4: State Machines COMPLETE)
; Input: VAR_ARG0 = instance_id, VAR_ARG1 = state_index
; Animation Data Structure (offset from anim_ptr):
;   +0: num_frames (1 byte)
;   +1: num_states (1 byte)
;   +2: controller_flags (1 byte)
;   +3: frame_table_ptr (2 bytes)
;   +5: state_table_ptr (2 bytes) - can be $0000 if no states
; State Table Entry Structure:
;   +0: num_frames_in_state (1 byte)
;   +1: loop_state (1 byte) - 0=hold, 1=loop
;   +2+: frame_indices (1 byte each) - indices into frame table
SET_ANIM_STATE_RUNTIME:
    ; Validate instance ID
    LDD VAR_ARG0
    CMPB #16
    BHS SASR_INVALID
    
    ; Calculate instance address: ANIM_POOL + (ID * 12)
    LDA #12
    MUL                 ; D = ID * 12
    ADDD #ANIM_POOL
    TFR D,X             ; X = instance pointer
    
    ; Check if active
    TST 10,X            ; Check active flag (offset +10)
    BEQ SASR_INVALID
    
    ; Load animation data pointer
    LDU 0,X             ; U = anim_ptr (offset +0 in instance)
    
    ; Check if animation has states
    LDD 5,U             ; Load state_table_ptr (offset +5 in anim data)
    BEQ SASR_NO_STATES  ; If NULL, animation has no states
    
    ; Calculate state entry address: state_table_ptr + state calculations
    ; Each state entry is variable size: 2 + num_frames_in_state bytes
    ; So we need to iterate through states until we reach target state_index
    TFR D,Y             ; Y = state_table_ptr (base of state table)
    LDA VAR_ARG1+1      ; A = target state_index (low byte)
    BEQ SASR_FOUND_STATE ; If state 0, we're already there
    
    ; Iterate through states to find target state
    PSHS X              ; Save instance pointer
    LDB A               ; B = remaining states to skip
    
SASR_STATE_LOOP:
    ; Current state entry is at Y
    ; Size = 2 + num_frames_in_state
    LDA 0,Y             ; Load num_frames_in_state
    ADDA #2             ; Add header size (num_frames + loop_state)
    LEAY A,Y            ; Y += state entry size (advance to next state)
    DECB                ; remaining--
    BNE SASR_STATE_LOOP ; Continue until we reach target state
    
    PULS X              ; Restore instance pointer
    
SASR_FOUND_STATE:
    ; Y now points to target state entry
    ; Read state data
    LDA 0,Y             ; num_frames_in_state
    LDB 1,Y             ; loop_state flag
    
    ; Update instance with state info
    ; Store state index
    LDA VAR_ARG1+1      ; Get state_index back
    STA 4,X             ; Update state_idx (offset +4 in instance)
    
    ; Store state's first frame index as current frame
    LDA 2,Y             ; Load first frame index in state
    STA 2,X             ; Update frame_idx (offset +2 in instance)
    
    ; Store state frame count (for bounds checking in UPDATE_ANIM)
    LDA 0,Y             ; num_frames_in_state
    STA 11,X            ; Store in state_num_frames (offset +11)
    
    ; Reset frame counter to restart timing
    CLR 3,X             ; counter = 0 (offset +3)
    
    RTS
    
SASR_NO_STATES:
    ; Animation has no states - just reset frame counter
    CLR 3,X             ; counter = 0
    RTS
    
SASR_INVALID:
    RTS

START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
    CLR $C80E        ; Initialize Vec_Prev_Btns to 0 for Read_Btns debounce
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:9
    ; VPy_LINE:4
    LDD #-1
    STD VAR_ANIM_ID
    ; VPy_LINE:5
    LDD #0
    STD VAR_CURRENT_STATE
    ; VPy_LINE:6
    LDD #0
    STD VAR_FRAME_COUNTER
    ; VPy_LINE:7
    LDD #0
    STD VAR_JOY_X
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
    ; VPy_LINE:12
; CREATE_ANIM("player") - allocate instance from pool
    LDX #_PLAYER_ANIM        ; Load animation data pointer
    JSR CREATE_ANIM_RUNTIME  ; Returns instance ID in D
    STD RESULT               ; Store instance ID (0-15 or -1)
    LDX RESULT
    STX VAR_ANIM_ID
    ; VPy_LINE:15
; SET_ANIM_STATE(instance_id, state_index) - change state
    LDD VAR_ANIM_ID
    STD RESULT
    LDD RESULT               ; Instance ID
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT               ; State index
    STD VAR_ARG1
    JSR SET_ANIM_STATE_RUNTIME
    LDD #0
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

    ; === Initialize Animation Pool (mark all 16 slots as inactive) ===
    LDX #ANIM_POOL
    LDA #16          ; 16 animation slots
ANIM_POOL_INIT_LOOP:
    CLR 0,X          ; Clear 'active' flag (byte 0 of each 12-byte slot)
    LEAX 12,X        ; Move to next slot (12 bytes per instance)
    DECA
    BNE ANIM_POOL_INIT_LOOP
    ; Animation pool ready for CREATE_ANIM calls

    ; JSR Wait_Recal is now called at start of LOOP_BODY (see auto-inject)
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    BRA MAIN

    ; VPy_LINE:17
LOOP_BODY:
    JSR Wait_Recal  ; CRITICAL: Sync with CRT refresh (50Hz frame timing)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; DEBUG: Statement 0 - Discriminant(8)
    ; VPy_LINE:18
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 18
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(0)
    ; VPy_LINE:21
; NATIVE_CALL: J1_X at line 21
    JSR J1X_BUILTIN
    STD RESULT
    LDX RESULT
    STX VAR_JOY_X
    ; DEBUG: Statement 2 - Discriminant(9)
    ; VPy_LINE:24
    LDD VAR_JOY_X
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
    ; VPy_LINE:26
    LDD VAR_CURRENT_STATE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BNE CT_6
    LDD #0
    STD RESULT
    BRA CE_7
CT_6:
    LDD #1
    STD RESULT
CE_7:
    LDD RESULT
    LBEQ IF_NEXT_5
    ; VPy_LINE:27
    LDD #0
    STD RESULT
    LDX RESULT
    STX VAR_CURRENT_STATE
    ; VPy_LINE:28
; SET_ANIM_STATE(instance_id, state_index) - change state
    LDD VAR_ANIM_ID
    STD RESULT
    LDD RESULT               ; Instance ID
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT               ; State index
    STD VAR_ARG1
    JSR SET_ANIM_STATE_RUNTIME
    LDD #0
    STD RESULT
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    LBRA IF_END_0
IF_NEXT_1:
    ; VPy_LINE:31
    LDD VAR_CURRENT_STATE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BNE CT_10
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
    LDD #1
    STD RESULT
    LDX RESULT
    STX VAR_CURRENT_STATE
    ; VPy_LINE:33
; SET_ANIM_STATE(instance_id, state_index) - change state
    LDD VAR_ANIM_ID
    STD RESULT
    LDD RESULT               ; Instance ID
    STD VAR_ARG0
    LDD #1
    STD RESULT
    LDD RESULT               ; State index
    STD VAR_ARG1
    JSR SET_ANIM_STATE_RUNTIME
    LDD #0
    STD RESULT
    LBRA IF_END_8
IF_NEXT_9:
IF_END_8:
IF_END_0:
    ; DEBUG: Statement 3 - Discriminant(8)
    ; VPy_LINE:36
; UPDATE_ANIM(instance_id, x, y) - update position + advance frame
    LDD VAR_ANIM_ID
    STD RESULT
    LDD RESULT
    STD TMPPTR               ; Save instance_id
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPPTR+2             ; Save X position
    LDD #30
    STD RESULT
    LDD RESULT
    STD TMPPTR+4             ; Save Y position
    LDD TMPPTR               ; Instance ID
    LDX TMPPTR+2             ; X position
    LDY TMPPTR+4             ; Y position
    JSR UPDATE_ANIM_RUNTIME  ; Update instance
    LDD #0
    STD RESULT
    ; DEBUG: Statement 4 - Discriminant(8)
    ; VPy_LINE:37
; DRAW_ANIM(instance_id) - render current frame
    LDD VAR_ANIM_ID
    STD RESULT
    LDD RESULT               ; Instance ID
    JSR DRAW_ANIM_RUNTIME    ; Draw at stored position
    LDD #0
    STD RESULT
    ; DEBUG: Statement 5 - Discriminant(0)
    ; VPy_LINE:39
    LDD VAR_FRAME_COUNTER
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
    STX VAR_FRAME_COUNTER
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************

; ========================================
; ASSET DATA SECTION
; Embedded 6 of 6 assets (unused assets excluded)
; ========================================

; Vector asset: player_walk_1
; Generated from player_walk_1.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 3
; X bounds: min=-10, max=6, width=16
; Center: (-2, 0)

_PLAYER_WALK_1_WIDTH EQU 16
_PLAYER_WALK_1_CENTER_X EQU -2
_PLAYER_WALK_1_CENTER_Y EQU 0

_PLAYER_WALK_1_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _PLAYER_WALK_1_PATH0        ; pointer to path 0

_PLAYER_WALK_1_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0A,$FF,0,0        ; path0: header (y=10, x=-1, relative to center)
    FCB $FF,$EC,$F9          ; line 0: flag=-1, dy=-20, dx=-7
    FCB $FF,$00,$10          ; line 1: flag=-1, dy=0, dx=16
    FCB $FF,$14,$F7          ; closing line: flag=-1, dy=20, dx=-9
    FCB 2                ; End marker (path complete)

; Vector asset: player_walk_2
; Generated from player_walk_2.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 3
; X bounds: min=-8, max=8, width=16
; Center: (0, 0)

_PLAYER_WALK_2_WIDTH EQU 16
_PLAYER_WALK_2_CENTER_X EQU 0
_PLAYER_WALK_2_CENTER_Y EQU 0

_PLAYER_WALK_2_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _PLAYER_WALK_2_PATH0        ; pointer to path 0

_PLAYER_WALK_2_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0A,$00,0,0        ; path0: header (y=10, x=0, relative to center)
    FCB $FF,$EC,$F8          ; line 0: flag=-1, dy=-20, dx=-8
    FCB $FF,$00,$10          ; line 1: flag=-1, dy=0, dx=16
    FCB $FF,$14,$F8          ; closing line: flag=-1, dy=20, dx=-8
    FCB 2                ; End marker (path complete)

; Vector asset: player_walk_3
; Generated from player_walk_3.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 3
; X bounds: min=-6, max=10, width=16
; Center: (2, 0)

_PLAYER_WALK_3_WIDTH EQU 16
_PLAYER_WALK_3_CENTER_X EQU 2
_PLAYER_WALK_3_CENTER_Y EQU 0

_PLAYER_WALK_3_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _PLAYER_WALK_3_PATH0        ; pointer to path 0

_PLAYER_WALK_3_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0A,$01,0,0        ; path0: header (y=10, x=1, relative to center)
    FCB $FF,$EC,$F7          ; line 0: flag=-1, dy=-20, dx=-9
    FCB $FF,$00,$10          ; line 1: flag=-1, dy=0, dx=16
    FCB $FF,$14,$F9          ; closing line: flag=-1, dy=20, dx=-7
    FCB 2                ; End marker (path complete)

; Vector asset: player_idle_1
; Generated from player_idle_1.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 3
; X bounds: min=-8, max=8, width=16
; Center: (0, 0)

_PLAYER_IDLE_1_WIDTH EQU 16
_PLAYER_IDLE_1_CENTER_X EQU 0
_PLAYER_IDLE_1_CENTER_Y EQU 0

_PLAYER_IDLE_1_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _PLAYER_IDLE_1_PATH0        ; pointer to path 0

_PLAYER_IDLE_1_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0A,$00,0,0        ; path0: header (y=10, x=0, relative to center)
    FCB $FF,$EC,$F8          ; line 0: flag=-1, dy=-20, dx=-8
    FCB $FF,$00,$10          ; line 1: flag=-1, dy=0, dx=16
    FCB $FF,$14,$F8          ; closing line: flag=-1, dy=20, dx=-8
    FCB 2                ; End marker (path complete)

; Vector asset: player_idle_2
; Generated from player_idle_2.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 3
; X bounds: min=-8, max=8, width=16
; Center: (0, 1)

_PLAYER_IDLE_2_WIDTH EQU 16
_PLAYER_IDLE_2_CENTER_X EQU 0
_PLAYER_IDLE_2_CENTER_Y EQU 1

_PLAYER_IDLE_2_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _PLAYER_IDLE_2_PATH0        ; pointer to path 0

_PLAYER_IDLE_2_PATH0:    ; Path 0
    FCB 120              ; path0: intensity
    FCB $0B,$00,0,0        ; path0: header (y=11, x=0, relative to center)
    FCB $FF,$EA,$F8          ; line 0: flag=-1, dy=-22, dx=-8
    FCB $FF,$00,$10          ; line 1: flag=-1, dy=0, dx=16
    FCB $FF,$16,$F8          ; closing line: flag=-1, dy=22, dx=-8
    FCB 2                ; End marker (path complete)

; Animation Asset: player (from /Users/daniel/projects/vectrex-pseudo-python/examples/test_state_complete/assets/animations/player.vanim)
; 5 frame(s), 2 state(s)
; ===== ANIMATION: player =====
; Version: 1.0
; Frames: 5
; States: 2

_PLAYER_ANIM:
    FCB 5          ; num_frames
    FCB 2          ; num_states
    FCB $00        ; controller_flags (bit 0: mirror_on_left)
    FDB _PLAYER_FRAMES    ; Pointer to frame table
    FDB _PLAYER_STATES    ; Pointer to state table

; Frame table for player
_PLAYER_FRAMES:
    ; Frame 0: idle1 (duration=30)
_PLAYER_IDLE1_FRAME:
    FDB _PLAYER_IDLE_1_VECTORS     ; Pointer to vector asset
    FDB 30          ; Duration (ticks)
    FCB 127          ; Intensity
    FDB 0          ; Offset X
    FDB 0          ; Offset Y
    FCB 0          ; Mirror mode
    ; Frame 1: idle2 (duration=30)
_PLAYER_IDLE2_FRAME:
    FDB _PLAYER_IDLE_2_VECTORS     ; Pointer to vector asset
    FDB 30          ; Duration (ticks)
    FCB 127          ; Intensity
    FDB 0          ; Offset X
    FDB 0          ; Offset Y
    FCB 0          ; Mirror mode
    ; Frame 2: walk1 (duration=8)
_PLAYER_WALK1_FRAME:
    FDB _PLAYER_WALK_1_VECTORS     ; Pointer to vector asset
    FDB 8          ; Duration (ticks)
    FCB 127          ; Intensity
    FDB 0          ; Offset X
    FDB 0          ; Offset Y
    FCB 0          ; Mirror mode
    ; Frame 3: walk2 (duration=8)
_PLAYER_WALK2_FRAME:
    FDB _PLAYER_WALK_2_VECTORS     ; Pointer to vector asset
    FDB 8          ; Duration (ticks)
    FCB 127          ; Intensity
    FDB 0          ; Offset X
    FDB 0          ; Offset Y
    FCB 0          ; Mirror mode
    ; Frame 4: walk3 (duration=8)
_PLAYER_WALK3_FRAME:
    FDB _PLAYER_WALK_3_VECTORS     ; Pointer to vector asset
    FDB 8          ; Duration (ticks)
    FCB 127          ; Intensity
    FDB 0          ; Offset X
    FDB 0          ; Offset Y
    FCB 0          ; Mirror mode

; State table for player
_PLAYER_STATES:
    ; State: walking
_PLAYER_WALKING_STATE:
    FCB 3          ; num_frames in state
    FCB 1          ; loop_state (0=no, 1=yes)
    FCB 2          ; Frame index: walk1
    FCB 3          ; Frame index: walk2
    FCB 4          ; Frame index: walk3
    ; State: idle
_PLAYER_IDLE_STATE:
    FCB 2          ; num_frames in state
    FCB 1          ; loop_state (0=no, 1=yes)
    FCB 0          ; Frame index: idle1
    FCB 1          ; Frame index: idle2


