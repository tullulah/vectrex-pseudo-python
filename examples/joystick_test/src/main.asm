; --- Motorola 6809 backend (Vectrex) title='JOY TEST' origin=$0000 ---
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
    FCC "JOY TEST"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 36 bytes
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$02   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$04   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$06   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$08   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0A   ; Pointer temp (2 bytes)
TMPPTR2              EQU $C880+$0C   ; Pointer temp 2 (for nested array operations) (2 bytes)
DIV_A                EQU $C880+$0E   ; Dividend (2 bytes)
DIV_B                EQU $C880+$10   ; Divisor (2 bytes)
DIV_Q                EQU $C880+$12   ; Quotient (2 bytes)
DIV_R                EQU $C880+$14   ; Remainder (2 bytes)
TEMP_YX              EQU $C880+$16   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$18   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$19   ; Temporary y storage (1 bytes)
PSG_MUSIC_PTR        EQU $C880+$1A   ; Current music position pointer (2 bytes)
PSG_MUSIC_START      EQU $C880+$1C   ; Music start pointer (for loops) (2 bytes)
PSG_IS_PLAYING       EQU $C880+$1E   ; Playing flag ($00=stopped, $01=playing) (1 bytes)
PSG_MUSIC_ACTIVE     EQU $C880+$1F   ; Set during UPDATE_MUSIC_PSG (1 bytes)
PSG_FRAME_COUNT      EQU $C880+$20   ; Frame register write count (1 bytes)
PSG_DELAY_FRAMES     EQU $C880+$21   ; Frames to wait before next read (1 bytes)
NUM_STR              EQU $C880+$22   ; String buffer for PRINT_NUMBER (2 bytes)
PSG_MUSIC_PTR_DP   EQU $1A  ; DP-relative
PSG_MUSIC_START_DP EQU $1C  ; DP-relative
PSG_IS_PLAYING_DP  EQU $1E  ; DP-relative
PSG_MUSIC_ACTIVE_DP EQU $1F  ; DP-relative
PSG_FRAME_COUNT_DP EQU $20  ; DP-relative
PSG_DELAY_FRAMES_DP EQU $21  ; DP-relative

    JMP START

;**** CONST DECLARATIONS (NUMBER-ONLY) ****
; VPy_LINE:9
; _CONST_DECL_0:  ; const BTN_DEBOUNCE_FRAMES

; === JOYSTICK BUILTIN SUBROUTINES ===
; J1_X() - Read Joystick 1 X axis (INCREMENTAL - with state preservation)
; Returns: D = raw value from $C81B after Joy_Analog call
J1X_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81B from hardware)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81B)
    LDB $C81B    ; Vec_Joy_1_X (now updated by Joy_Analog)
    SEX          ; Sign-extend B to D
    PULS X       ; Restore X
    RTS

; J1_Y() - Read Joystick 1 Y axis (INCREMENTAL - with state preservation)
; Returns: D = raw value from $C81C after Joy_Analog call
J1Y_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81C from hardware)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81C)
    LDB $C81C    ; Vec_Joy_1_Y (now updated by Joy_Analog)
    SEX          ; Sign-extend B to D
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
VECTREX_SET_INTENSITY:
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode)
    STA >$D00C     ; VIA_cntl
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDA VAR_ARG0+1
    JSR __Intensity_a
    RTS
; ============================================================================
; PSG DIRECT MUSIC PLAYER (inspired by Christman2024/malbanGit)
; ============================================================================
; Writes directly to PSG chip using WRITE_PSG sequence
;
; Music data format (frame-based):
;   FCB count           ; Number of register writes this frame
;   FCB reg, val        ; PSG register/value pairs
;   ...                 ; Repeat for each register
;   FCB $FF             ; End marker
;
; PSG Registers:
;   0-1: Channel A frequency (12-bit)
;   2-3: Channel B frequency
;   4-5: Channel C frequency
;   6:   Noise period
;   7:   Mixer control (enable/disable channels)
;   8-10: Channel A/B/C volume
;   11-12: Envelope period
;   13:  Envelope shape
; ============================================================================

; RAM variables (defined in RAM section above)
; PSG_MUSIC_PTR    EQU RESULT+26  (2 bytes)
; PSG_MUSIC_START  EQU RESULT+28  (2 bytes)
; PSG_IS_PLAYING   EQU RESULT+30  (1 byte)
; PSG_MUSIC_ACTIVE EQU RESULT+31  (1 byte) - Set=1 during UPDATE_MUSIC_PSG
; PSG_DELAY_FRAMES EQU RESULT+32  (1 byte) - Frames to wait before reading next data

; PLAY_MUSIC_RUNTIME - Start PSG music playback
; Input: X = pointer to PSG music data
PLAY_MUSIC_RUNTIME:
STX >PSG_MUSIC_PTR     ; Store current music pointer (force extended)
STX >PSG_MUSIC_START   ; Store start pointer for loops (force extended)
CLR >PSG_DELAY_FRAMES  ; Clear delay counter
LDA #$01
STA >PSG_IS_PLAYING ; Mark as playing (extended - var at 0xC8A0)
RTS

; ============================================================================
; UPDATE_MUSIC_PSG - Update PSG (call every frame)
; ============================================================================
UPDATE_MUSIC_PSG:
; CRITICAL: Set VIA to PSG mode BEFORE accessing PSG (don't assume state)
; DISABLED: Conflicts with SFX which uses Sound_Byte (HANDSHAKE mode)
; LDA #$00       ; VIA_cntl = $00 (PSG mode)
; STA >$D00C     ; VIA_cntl
LDA #$01
STA >PSG_MUSIC_ACTIVE  ; Mark music system active (for PSG logging)
LDA >PSG_IS_PLAYING ; Check if playing (extended - var at 0xC8A0)
BEQ PSG_update_done    ; Not playing, exit

LDX >PSG_MUSIC_PTR     ; Load pointer (force extended - LDX has no DP mode)
BEQ PSG_update_done    ; No music loaded

; Read frame count byte (number of register writes)
LDB ,X+
BEQ PSG_music_ended    ; Count=0 means end (no loop)
CMPB #$FF              ; Check for loop command
BEQ PSG_music_loop     ; $FF means loop (never valid as count)

; Process frame - push counter to stack
PSHS B                 ; Save count on stack

; Write register/value pairs to PSG
PSG_write_loop:
LDA ,X+                ; Load register number
LDB ,X+                ; Load register value
PSHS X                 ; Save pointer (after reads)

; WRITE_PSG sequence
STA VIA_port_a         ; Store register number
LDA #$19               ; BDIR=1, BC1=1 (LATCH)
STA VIA_port_b
LDA #$01               ; BDIR=0, BC1=0 (INACTIVE)
STA VIA_port_b
LDA VIA_port_a         ; Read status
STB VIA_port_a         ; Store data
LDB #$11               ; BDIR=1, BC1=0 (WRITE)
STB VIA_port_b
LDB #$01               ; BDIR=0, BC1=0 (INACTIVE)
STB VIA_port_b

PULS X                 ; Restore pointer
PULS B                 ; Get counter
DECB                   ; Decrement
BEQ PSG_frame_done     ; Done with this frame
PSHS B                 ; Save counter back
BRA PSG_write_loop

PSG_frame_done:

; Frame complete - update pointer and done
STX >PSG_MUSIC_PTR     ; Update pointer (force extended)
BRA PSG_update_done

PSG_music_ended:
CLR >PSG_IS_PLAYING ; Stop playback (extended - var at 0xC8A0)
; NOTE: Do NOT write PSG registers here - corrupts VIA for vector drawing
; Music will fade naturally as frame data stops updating
BRA PSG_update_done

PSG_music_loop:
; Loop command: $FF followed by 2-byte address (FDB)
; X points past $FF, read the target address
LDD ,X                 ; Load 2-byte loop target address
STD >PSG_MUSIC_PTR     ; Update pointer to loop start
; Exit - next frame will start from loop target
BRA PSG_update_done

PSG_update_done:
CLR >PSG_MUSIC_ACTIVE  ; Clear flag (music system done)
RTS

; ============================================================================
; STOP_MUSIC_RUNTIME - Stop music playback
; ============================================================================
STOP_MUSIC_RUNTIME:
CLR >PSG_IS_PLAYING ; Clear playing flag (extended - var at 0xC8A0)
CLR >PSG_MUSIC_PTR     ; Clear pointer high byte (force extended)
CLR >PSG_MUSIC_PTR+1   ; Clear pointer low byte (force extended)
; NOTE: Do NOT write PSG registers here - corrupts VIA for vector drawing
RTS

; ============================================================================
; AUDIO_UPDATE - Unified music + SFX update (auto-injected after WAIT_RECAL)
; ============================================================================
; Processes both music (channel B) and SFX (channel C) in one pass
; Uses Sound_Byte (BIOS) for PSG writes - compatible with both systems
; Sets DP=$D0 once at entry, restores at exit

; RAM variables (always defined, even if SFX not used)
sfx_pointer EQU RESULT+32    ; 2 bytes - Current AYFX frame pointer
sfx_status  EQU RESULT+34    ; 1 byte  - Active flag (0=inactive, 1=active)

AUDIO_UPDATE:
PSHS DP                 ; Save current DP
LDA #$D0                ; Set DP=$D0 (Sound_Byte requirement)
TFR A,DP

; UPDATE MUSIC (channel B: registers 9, 11-14)
LDA >PSG_IS_PLAYING     ; Check if music is playing
BEQ AU_SKIP_MUSIC       ; Skip if not

; Check delay counter first
LDA >PSG_DELAY_FRAMES   ; Load delay counter
BEQ AU_MUSIC_READ       ; If zero, read next frame data
DECA                    ; Decrement delay
STA >PSG_DELAY_FRAMES   ; Store back
CMPA #0                 ; Check if it just reached zero
BNE AU_UPDATE_SFX       ; If not zero yet, skip this frame

; Delay just reached zero, X points to count byte already
LDX >PSG_MUSIC_PTR      ; Load music pointer (points to count)
BEQ AU_SKIP_MUSIC       ; Skip if null
BRA AU_MUSIC_READ_COUNT ; Skip delay read, go straight to count

AU_MUSIC_READ:
LDX >PSG_MUSIC_PTR      ; Load music pointer
BEQ AU_SKIP_MUSIC       ; Skip if null

; Check if we need to read delay or we're ready for count
; PSG_DELAY_FRAMES just reached 0, so we read delay byte first
LDB ,X+                 ; Read delay counter (X now points to count byte)
CMPB #$FF               ; Check for loop marker
BEQ AU_MUSIC_LOOP       ; Handle loop
CMPB #0                 ; Check if delay is 0
BNE AU_MUSIC_HAS_DELAY  ; If not 0, process delay

; Delay is 0, read count immediately
AU_MUSIC_NO_DELAY:
AU_MUSIC_READ_COUNT:
LDB ,X+                 ; Read count (number of register writes)
BEQ AU_MUSIC_ENDED      ; If 0, end of music
CMPB #$FF               ; Check for loop marker (can appear after delay)
BEQ AU_MUSIC_LOOP       ; Handle loop
BRA AU_MUSIC_PROCESS_WRITES

AU_MUSIC_HAS_DELAY:
; B has delay > 0, store it and skip to next frame
DECB                    ; Delay-1 (we consume this frame)
STB >PSG_DELAY_FRAMES   ; Save delay counter
STX >PSG_MUSIC_PTR      ; Save pointer (X points to count byte)
BRA AU_UPDATE_SFX       ; Skip reading data this frame

AU_MUSIC_PROCESS_WRITES:
PSHS B                  ; Save count

; Mark that next time we should read delay, not count
; (This is implicit - after processing, X points to next delay byte)

AU_MUSIC_WRITE_LOOP:
LDA ,X+                 ; Load register number
LDB ,X+                 ; Load register value
PSHS X                  ; Save pointer
JSR Sound_Byte          ; Write to PSG using BIOS (DP=$D0)
PULS X                  ; Restore pointer
PULS B                  ; Get counter
DECB                    ; Decrement
BEQ AU_MUSIC_DONE       ; Done if count=0
PSHS B                  ; Save counter
BRA AU_MUSIC_WRITE_LOOP ; Continue

AU_MUSIC_DONE:
STX >PSG_MUSIC_PTR      ; Update music pointer
BRA AU_UPDATE_SFX       ; Now update SFX

AU_MUSIC_ENDED:
CLR >PSG_IS_PLAYING     ; Stop music
BRA AU_UPDATE_SFX       ; Continue to SFX

AU_MUSIC_LOOP:
LDD ,X                  ; Load loop target
STD >PSG_MUSIC_PTR      ; Set music pointer to loop
CLR >PSG_DELAY_FRAMES   ; Clear delay on loop
BRA AU_UPDATE_SFX       ; Continue to SFX

AU_SKIP_MUSIC:
BRA AU_UPDATE_SFX       ; Skip music, go to SFX

; UPDATE SFX (channel C: registers 4/5=tone, 6=noise, 10=volume, 7=mixer)
AU_UPDATE_SFX:
LDA >sfx_status         ; Check if SFX is active
BEQ AU_DONE             ; Skip if not active

JSR sfx_doframe         ; Process one SFX frame (uses Sound_Byte internally)

AU_DONE:
PULS DP                 ; Restore original DP
RTS

; sfx_doframe stub (SFX not used in this project)
sfx_doframe:
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
    JSR $F533       ; Init_Music_Buf - Initialize BIOS music system to silence

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:4
    LDD #0
    STD VAR_TRIANGLE_X
    ; VPy_LINE:5
    LDD #0
    STD VAR_TRIANGLE_Y
    ; VPy_LINE:8
    LDD #0
    STD VAR_BTN_DEBOUNCE
    ; VPy_LINE:10
    LDD #0
    STD VAR_STARTUP_DELAY
    ; VPy_LINE:13
    LDD #0
    STD VAR_MUSIC_STARTED
    ; VPy_LINE:16
    LDD #0
    STD VAR_BTN1
    ; VPy_LINE:17
    LDD #0
    STD VAR_BTN2
    ; VPy_LINE:18
    LDD #0
    STD VAR_BTN3
    ; VPy_LINE:19
    LDD #0
    STD VAR_BTN4
    ; VPy_LINE:22
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 22
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:23
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN_DEBOUNCE
    STU TMPPTR
    STX ,U
    ; VPy_LINE:24
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_STARTUP_DELAY
    STU TMPPTR
    STX ,U
; VPy_LINE:21

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

BTN_DEBOUNCE_FRAMES EQU 5
    ; VPy_LINE:26
LOOP_BODY:
    LEAS -4,S ; allocate locals
    JSR $F1AF    ; DP_to_C8 (ensure DP for variable access)
    JSR Wait_Recal ; Auto-injected: sync with vector beam
    JSR AUDIO_UPDATE  ; Auto-injected: update music + SFX (consistent timing)
    ; DEBUG: Processing 21 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(9)
    ; VPy_LINE:33
    LDD VAR_BTN_DEBOUNCE
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
    ; VPy_LINE:34
    LDD VAR_BTN_DEBOUNCE
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
    LDU #VAR_BTN_DEBOUNCE
    STU TMPPTR
    STX ,U
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    ; DEBUG: Statement 1 - Discriminant(9)
    ; VPy_LINE:37
    LDD VAR_STARTUP_DELAY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #30
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
    ; VPy_LINE:38
    LDD VAR_STARTUP_DELAY
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
    LDU #VAR_STARTUP_DELAY
    STU TMPPTR
    STX ,U
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    ; DEBUG: Statement 2 - Discriminant(0)
    ; VPy_LINE:41
; NATIVE_CALL: J1_X at line 41
    JSR J1X_BUILTIN
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; DEBUG: Statement 3 - Discriminant(0)
    ; VPy_LINE:42
; NATIVE_CALL: J1_Y at line 42
    JSR J1Y_BUILTIN
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; DEBUG: Statement 4 - Discriminant(0)
    ; VPy_LINE:45
; NATIVE_CALL: J1_BUTTON_1 at line 45
    JSR J1B1_BUILTIN
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN1
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 5 - Discriminant(0)
    ; VPy_LINE:46
; NATIVE_CALL: J1_BUTTON_2 at line 46
    JSR J1B2_BUILTIN
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN2
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 6 - Discriminant(0)
    ; VPy_LINE:47
; NATIVE_CALL: J1_BUTTON_3 at line 47
    JSR J1B3_BUILTIN
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN3
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 7 - Discriminant(0)
    ; VPy_LINE:48
; NATIVE_CALL: J1_BUTTON_4 at line 48
    JSR J1B4_BUILTIN
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN4
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 8 - Discriminant(9)
    ; VPy_LINE:51
    LDD VAR_STARTUP_DELAY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #30
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_10
    LDD #0
    STD RESULT
    BRA CE_11
CT_10:
    LDD #1
    STD RESULT
CE_11:
    LDD RESULT
    BEQ AND_FALSE_12
    LDD VAR_BTN_DEBOUNCE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_14
    LDD #0
    STD RESULT
    BRA CE_15
CT_14:
    LDD #1
    STD RESULT
CE_15:
    LDD RESULT
    BEQ AND_FALSE_12
    LDD #1
    STD RESULT
    BRA AND_END_13
AND_FALSE_12:
    LDD #0
    STD RESULT
AND_END_13:
    LDD RESULT
    LBEQ IF_NEXT_9
    ; VPy_LINE:52
    LDD VAR_BTN1
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_24
    LDD #0
    STD RESULT
    BRA CE_25
CT_24:
    LDD #1
    STD RESULT
CE_25:
    LDD RESULT
    BNE OR_TRUE_22
    LDD VAR_BTN2
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_26
    LDD #0
    STD RESULT
    BRA CE_27
CT_26:
    LDD #1
    STD RESULT
CE_27:
    LDD RESULT
    BNE OR_TRUE_22
    LDD #0
    STD RESULT
    BRA OR_END_23
OR_TRUE_22:
    LDD #1
    STD RESULT
OR_END_23:
    LDD RESULT
    BNE OR_TRUE_20
    LDD VAR_BTN3
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_28
    LDD #0
    STD RESULT
    BRA CE_29
CT_28:
    LDD #1
    STD RESULT
CE_29:
    LDD RESULT
    BNE OR_TRUE_20
    LDD #0
    STD RESULT
    BRA OR_END_21
OR_TRUE_20:
    LDD #1
    STD RESULT
OR_END_21:
    LDD RESULT
    BNE OR_TRUE_18
    LDD VAR_BTN4
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_30
    LDD #0
    STD RESULT
    BRA CE_31
CT_30:
    LDD #1
    STD RESULT
CE_31:
    LDD RESULT
    BNE OR_TRUE_18
    LDD #0
    STD RESULT
    BRA OR_END_19
OR_TRUE_18:
    LDD #1
    STD RESULT
OR_END_19:
    LDD RESULT
    LBEQ IF_NEXT_17
    ; VPy_LINE:53
    LDD #5
    STD RESULT
    LDX RESULT
    LDU #VAR_BTN_DEBOUNCE
    STU TMPPTR
    STX ,U
    LBRA IF_END_16
IF_NEXT_17:
IF_END_16:
    LBRA IF_END_8
IF_NEXT_9:
IF_END_8:
    ; DEBUG: Statement 9 - Discriminant(0)
    ; VPy_LINE:57
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    LSRA
    RORB
    STD RESULT
    LDX RESULT
    LDU #VAR_TRIANGLE_X
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 10 - Discriminant(0)
    ; VPy_LINE:58
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    LSRA
    RORB
    STD RESULT
    LDX RESULT
    LDU #VAR_TRIANGLE_Y
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 11 - Discriminant(8)
    ; VPy_LINE:61
; DRAW_VECTOR("tri", x, y) - 1 path(s) at position
    LDD VAR_TRIANGLE_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD VAR_TRIANGLE_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA TMPPTR+1  ; Save Y to temporary storage
    LDA TMPPTR    ; X position
    STA DRAW_VEC_X
    LDA TMPPTR+1  ; Y position
    STA DRAW_VEC_Y
    LDX #_TRI_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    ; DEBUG: Statement 12 - Discriminant(8)
    ; VPy_LINE:64
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 64
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 13 - Discriminant(8)
    ; VPy_LINE:65
; DRAW_VECTOR("logo", x, y) - 7 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD #20
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA TMPPTR+1  ; Save Y to temporary storage
    LDA TMPPTR    ; X position
    STA DRAW_VEC_X
    LDA TMPPTR+1  ; Y position
    STA DRAW_VEC_Y
    LDX #_LOGO_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_LOGO_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_LOGO_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_LOGO_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDX #_LOGO_PATH4  ; Path 4
    JSR Draw_Sync_List_At
    LDX #_LOGO_PATH5  ; Path 5
    JSR Draw_Sync_List_At
    LDX #_LOGO_PATH6  ; Path 6
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    ; DEBUG: Statement 14 - Discriminant(8)
    ; VPy_LINE:68
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 68
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 15 - Discriminant(9)
    ; VPy_LINE:70
    LDD VAR_BTN1
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_34
    LDD #0
    STD RESULT
    BRA CE_35
CT_34:
    LDD #1
    STD RESULT
CE_35:
    LDD RESULT
    LBEQ IF_NEXT_33
    ; VPy_LINE:71
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
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
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 71
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_32
IF_NEXT_33:
    ; VPy_LINE:73
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_0
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 73
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
IF_END_32:
    ; DEBUG: Statement 16 - Discriminant(9)
    ; VPy_LINE:75
    LDD VAR_BTN2
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_38
    LDD #0
    STD RESULT
    BRA CE_39
CT_38:
    LDD #1
    STD RESULT
CE_39:
    LDD RESULT
    LBEQ IF_NEXT_37
    ; VPy_LINE:76
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_3
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 76
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_36
IF_NEXT_37:
    ; VPy_LINE:78
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_2
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 78
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
IF_END_36:
    ; DEBUG: Statement 17 - Discriminant(9)
    ; VPy_LINE:80
    LDD VAR_BTN3
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_42
    LDD #0
    STD RESULT
    BRA CE_43
CT_42:
    LDD #1
    STD RESULT
CE_43:
    LDD RESULT
    LBEQ IF_NEXT_41
    ; VPy_LINE:81
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_5
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 81
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_40
IF_NEXT_41:
    ; VPy_LINE:83
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_4
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 83
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
IF_END_40:
    ; DEBUG: Statement 18 - Discriminant(9)
    ; VPy_LINE:85
    LDD VAR_BTN4
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_46
    LDD #0
    STD RESULT
    BRA CE_47
CT_46:
    LDD #1
    STD RESULT
CE_47:
    LDD RESULT
    LBEQ IF_NEXT_45
    ; VPy_LINE:86
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_7
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 86
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_44
IF_NEXT_45:
    ; VPy_LINE:88
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_6
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 88
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
IF_END_44:
    ; DEBUG: Statement 19 - Discriminant(9)
    ; VPy_LINE:91
    LDD VAR_BTN_DEBOUNCE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_50
    LDD #0
    STD RESULT
    BRA CE_51
CT_50:
    LDD #1
    STD RESULT
CE_51:
    LDD RESULT
    LBEQ IF_NEXT_49
    ; VPy_LINE:92
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-20
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_8
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 92
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_48
IF_NEXT_49:
IF_END_48:
    ; DEBUG: Statement 20 - Discriminant(9)
    ; VPy_LINE:94
    LDD VAR_STARTUP_DELAY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #30
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_54
    LDD #0
    STD RESULT
    BRA CE_55
CT_54:
    LDD #1
    STD RESULT
CE_55:
    LDD RESULT
    LBEQ IF_NEXT_53
    ; VPy_LINE:95
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_9
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 95
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_52
IF_NEXT_53:
IF_END_52:
    LEAS 4,S ; free locals
    RTS

DIV16:
    LDD #0
    STD DIV_Q
    LDD DIV_A
    STD DIV_R
    LDD DIV_B
    BEQ DIV16_DONE
DIV16_LOOP:
    LDD DIV_R
    SUBD DIV_B
    BLO DIV16_DONE
    STD DIV_R
    LDD DIV_Q
    ADDD #1
    STD DIV_Q
    BRA DIV16_LOOP
DIV16_DONE:
    LDD DIV_Q
    STD RESULT
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************
VL_PTR     EQU $CF80      ; Current position in vector list
VL_Y       EQU $CF82      ; Y position (1 byte)
VL_X       EQU $CF83      ; X position (1 byte)
VL_SCALE   EQU $CF84      ; Scale factor (1 byte)
VAR_TRIANGLE_X EQU $CF10+0
VAR_TRIANGLE_Y EQU $CF10+2
VAR_BTN_DEBOUNCE EQU $CF10+4
VAR_STARTUP_DELAY EQU $CF10+6
VAR_MUSIC_STARTED EQU $CF10+8
VAR_BTN1 EQU $CF10+10
VAR_BTN2 EQU $CF10+12
VAR_BTN3 EQU $CF10+14
VAR_BTN4 EQU $CF10+16
; Call argument scratch space
VAR_ARG0 EQU $C8B2
VAR_ARG1 EQU $C8B4
VAR_ARG2 EQU $C8B6
VAR_ARG3 EQU $C8B8

; ========================================
; ASSET DATA SECTION
; Embedded 2 of 3 assets (unused assets excluded)
; ========================================

; Vector asset: tri
; Generated from tri.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 3
; X bounds: min=-15, max=15, width=30
; Center: (0, 5)

_TRI_WIDTH EQU 30
_TRI_CENTER_X EQU 0
_TRI_CENTER_Y EQU 5

_TRI_VECTORS:  ; Main entry
_TRI_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0F,$00,0,0        ; path0: header (y=15, x=0, relative to center)
    FCB $FF,$E2,$F1          ; line 0: flag=-1, dy=-30, dx=-15
    FCB $FF,$00,$1E          ; line 1: flag=-1, dy=0, dx=30
    FCB $FF,$1E,$F1          ; closing line: flag=-1, dy=30, dx=-15
    FCB 2                ; End marker (path complete)

; Vector asset: logo
; Generated from logo.vec (Malban Draw_Sync_List format)
; Total paths: 7, points: 65
; X bounds: min=-82, max=81, width=163
; Center: (0, 0)

_LOGO_WIDTH EQU 163
_LOGO_CENTER_X EQU 0
_LOGO_CENTER_Y EQU 0

_LOGO_VECTORS:  ; Main entry
_LOGO_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $13,$AE,0,0        ; path0: header (y=19, x=-82, relative to center)
    FCB $FF,$EF,$06          ; line 0: flag=-1, dy=-17, dx=6
    FCB $FF,$02,$07          ; line 1: flag=-1, dy=2, dx=7
    FCB $FF,$D6,$09          ; line 2: flag=-1, dy=-42, dx=9
    FCB $FF,$0B,$11          ; line 3: flag=-1, dy=11, dx=17
    FCB $FF,$0C,$FC          ; line 4: flag=-1, dy=12, dx=-4
    FCB $FF,$0D,$10          ; line 5: flag=-1, dy=13, dx=16
    FCB $FF,$0B,$09          ; line 6: flag=-1, dy=11, dx=9
    FCB $FF,$0C,$01          ; line 7: flag=-1, dy=12, dx=1
    FCB $FF,$08,$F8          ; line 8: flag=-1, dy=8, dx=-8
    FCB $FF,$02,$F0          ; line 9: flag=-1, dy=2, dx=-16
    FCB $FF,$FC,$F1          ; line 10: flag=-1, dy=-4, dx=-15
    FCB $FF,$F8,$EA          ; line 11: flag=-1, dy=-8, dx=-22
    FCB $FF,$00,$00          ; line 12: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_LOGO_PATH1:    ; Path 1
    FCB 127              ; path1: intensity
    FCB $FB,$E3,0,0        ; path1: header (y=-5, x=-29, relative to center)
    FCB $FF,$E7,$F8          ; line 0: flag=-1, dy=-25, dx=-8
    FCB $FF,$04,$10          ; line 1: flag=-1, dy=4, dx=16
    FCB $FF,$0C,$02          ; line 2: flag=-1, dy=12, dx=2
    FCB $FF,$03,$0B          ; line 3: flag=-1, dy=3, dx=11
    FCB $FF,$FA,$00          ; line 4: flag=-1, dy=-6, dx=0
    FCB $FF,$03,$0D          ; line 5: flag=-1, dy=3, dx=13
    FCB $FF,$22,$F7          ; line 6: flag=-1, dy=34, dx=-9
    FCB $FF,$FD,$F1          ; line 7: flag=-1, dy=-3, dx=-15
    FCB $FF,$F5,$FF          ; line 8: flag=-1, dy=-11, dx=-1
    FCB $FF,$F5,$F7          ; line 9: flag=-1, dy=-11, dx=-9
    FCB $FF,$00,$00          ; line 10: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_LOGO_PATH2:    ; Path 2
    FCB 127              ; path2: intensity
    FCB $07,$CE,0,0        ; path2: header (y=7, x=-50, relative to center)
    FCB $FF,$F8,$02          ; line 0: flag=-1, dy=-8, dx=2
    FCB $FF,$07,$08          ; line 1: flag=-1, dy=7, dx=8
    FCB $FF,$01,$F6          ; line 2: flag=-1, dy=1, dx=-10
    FCB $FF,$00,$00          ; line 3: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_LOGO_PATH3:    ; Path 3
    FCB 127              ; path3: intensity
    FCB $06,$F4,0,0        ; path3: header (y=6, x=-12, relative to center)
    FCB $FF,$F6,$FD          ; line 0: flag=-1, dy=-10, dx=-3
    FCB $FF,$02,$07          ; line 1: flag=-1, dy=2, dx=7
    FCB $FF,$08,$FC          ; line 2: flag=-1, dy=8, dx=-4
    FCB $FF,$FE,$01          ; line 3: flag=-1, dy=-2, dx=1
    FCB 2                ; End marker (path complete)

_LOGO_PATH4:    ; Path 4
    FCB 127              ; path4: intensity
    FCB $F3,$0A,0,0        ; path4: header (y=-13, x=10, relative to center)
    FCB $FF,$29,$02          ; line 0: flag=-1, dy=41, dx=2
    FCB $FF,$02,$0D          ; line 1: flag=-1, dy=2, dx=13
    FCB $FF,$EB,$0A          ; line 2: flag=-1, dy=-21, dx=10
    FCB $FF,$1A,$07          ; line 3: flag=-1, dy=26, dx=7
    FCB $FF,$03,$14          ; line 4: flag=-1, dy=3, dx=20
    FCB $FF,$D8,$EF          ; line 5: flag=-1, dy=-40, dx=-17
    FCB $FF,$FE,$F3          ; line 6: flag=-1, dy=-2, dx=-13
    FCB $FF,$0D,$F8          ; line 7: flag=-1, dy=13, dx=-8
    FCB $FF,$EE,$FC          ; line 8: flag=-1, dy=-18, dx=-4
    FCB $FF,$FC,$F6          ; line 9: flag=-1, dy=-4, dx=-10
    FCB $FF,$00,$00          ; line 10: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_LOGO_PATH5:    ; Path 5
    FCB 127              ; path5: intensity
    FCB $06,$45,0,0        ; path5: header (y=6, x=69, relative to center)
    FCB $FF,$08,$F5          ; line 0: flag=-1, dy=8, dx=-11
    FCB $FF,$F4,$F7          ; line 1: flag=-1, dy=-12, dx=-9
    FCB $FF,$F7,$01          ; line 2: flag=-1, dy=-9, dx=1
    FCB $FF,$FE,$0C          ; line 3: flag=-1, dy=-2, dx=12
    FCB $FF,$03,$FA          ; line 4: flag=-1, dy=3, dx=-6
    FCB $FF,$05,$01          ; line 5: flag=-1, dy=5, dx=1
    FCB $FF,$02,$17          ; line 6: flag=-1, dy=2, dx=23
    FCB $FF,$F3,$FD          ; line 7: flag=-1, dy=-13, dx=-3
    FCB $FF,$F9,$EE          ; line 8: flag=-1, dy=-7, dx=-18
    FCB $FF,$04,$F0          ; line 9: flag=-1, dy=4, dx=-16
    FCB $FF,$0B,$F8          ; line 10: flag=-1, dy=11, dx=-8
    FCB 2                ; End marker (path complete)

_LOGO_PATH6:    ; Path 6
    FCB 127              ; path6: intensity
    FCB $06,$45,0,0        ; path6: header (y=6, x=69, relative to center)
    FCB $FF,$00,$0C          ; line 0: flag=-1, dy=0, dx=12
    FCB $FF,$0C,$F8          ; line 1: flag=-1, dy=12, dx=-8
    FCB $FF,$03,$F0          ; line 2: flag=-1, dy=3, dx=-16
    FCB $FF,$FB,$FC          ; line 3: flag=-1, dy=-5, dx=-4
    FCB 2                ; End marker (path complete)

; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "B1:OFF"
    FCB $80
STR_1:
    FCC "B1:ON "
    FCB $80
STR_2:
    FCC "B2:OFF"
    FCB $80
STR_3:
    FCC "B2:ON "
    FCB $80
STR_4:
    FCC "B3:OFF"
    FCB $80
STR_5:
    FCC "B3:ON "
    FCB $80
STR_6:
    FCC "B4:OFF"
    FCB $80
STR_7:
    FCC "B4:ON "
    FCB $80
STR_8:
    FCC "DEBOUNCE"
    FCB $80
STR_9:
    FCC "STARTING..."
    FCB $80
DRAW_VEC_X EQU RESULT+18
DRAW_VEC_Y EQU RESULT+19
MIRROR_X EQU RESULT+20
MIRROR_Y EQU RESULT+21
DRAW_VEC_INTENSITY EQU RESULT+22
DRAW_CIRCLE_XC EQU RESULT+23
DRAW_CIRCLE_YC EQU RESULT+24
DRAW_CIRCLE_DIAM EQU RESULT+25
DRAW_CIRCLE_INTENSITY EQU RESULT+26
DRAW_CIRCLE_TEMP EQU RESULT+27
