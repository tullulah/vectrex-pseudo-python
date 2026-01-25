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
; Total RAM used: 212 bytes
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPPTR               EQU $C880+$02   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$04   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$06   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$08   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$09   ; Temporary y storage (1 bytes)
ANIM_POOL            EQU $C880+$0A   ; Animation instance pool (16 instances × 12 bytes) (192 bytes)
NUM_STR              EQU $C880+$CA   ; String buffer for PRINT_NUMBER (2 bytes)
VAR_ARG0             EQU $C880+$CC   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$CE   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$D0   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$D2   ; Function argument 3 (2 bytes)

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
    TST 10,X            ; Check active flag (offset +10)
    BEQ CAR_FOUND_SLOT  ; Found inactive slot
    LEAX 12,X           ; Move to next instance (12 bytes each)
    INCA                ; Increment ID counter
    BRA CAR_SEARCH
CAR_FOUND_SLOT:
    ; Initialize instance
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
    ; Return instance ID in D (already in A from counter)
    TFR A,B             ; A = instance ID, B = 0
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
    
    ; Check if we reached end of state (need to loop)
    LDX 0,U             ; Reload anim_ptr
    LDA 0,X             ; Load num_frames (offset +0 in anim data)
    CMPA 2,U            ; Compare with frame_idx
    BHI UAR_DONE        ; frame_idx < num_frames, done
    
    ; Loop back to first frame of current state
    ; TODO (Phase 4): Implement proper state machine with start_frame
    CLR 2,U             ; frame_idx = 0 (simple loop for now)
    
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

; SET_ANIM_MIRROR_RUNTIME - Set mirror flags for animation instance
; Input: X = instance_id (in X per builtins.rs), D = mirror_value
SET_ANIM_MIRROR_RUNTIME:
    ; Validate instance ID (X contains ID as 16-bit)
    CMPX #16
    BHS SAMR_INVALID
    
    ; Calculate instance address: ANIM_POOL + (ID * 12)
    PSHS D              ; Save mirror value
    TFR X,D             ; D = instance ID
    LDA #12
    MUL                 ; D = ID * 12
    ADDD #ANIM_POOL
    TFR D,X             ; X = instance pointer
    
    ; Check if active
    TST 10,X            ; Check active flag
    BEQ SAMR_INACTIVE
    
    ; Store mirror value
    PULS D              ; Get mirror value
    STB 5,X             ; Store mirror (offset +5, low byte only)
    RTS
    
SAMR_INACTIVE:
    PULS D              ; Clean stack
SAMR_INVALID:
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
    ; State: idle
_PLAYER_WALK_IDLE_STATE:
    FCB 1          ; num_frames in state
    FCB 1          ; loop_state (0=no, 1=yes)
    FCB 0          ; Frame index: walk_1
    ; State: walking
_PLAYER_WALK_WALKING_STATE:
    FCB 5          ; num_frames in state
    FCB 1          ; loop_state (0=no, 1=yes)
    FCB 0          ; Frame index: walk_1
    FCB 1          ; Frame index: walk_2
    FCB 2          ; Frame index: walk_3
    FCB 3          ; Frame index: walk_4
    FCB 4          ; Frame index: walk_5


