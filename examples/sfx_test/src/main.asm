; --- Motorola 6809 backend (Vectrex) title='SFX Test' origin=$0000 ---
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
    FCC "SFX TEST"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; Must be defined BEFORE builtin helpers that reference them
RESULT         EQU $C880   ; Main result temporary
SFX_PTR        EQU $C8A8   ; Current SFX pointer (RESULT+$28, 2 bytes)
SFX_TICK       EQU $C8AA   ; Current frame counter (RESULT+$2A, 2 bytes)
SFX_ACTIVE     EQU $C8AC   ; Playback state (RESULT+$2C, 1 byte)
SFX_PHASE      EQU $C8AD   ; Envelope phase: 0=A,1=D,2=S,3=R (RESULT+$2D, 1 byte)
SFX_VOL        EQU $C8AE   ; Current volume 0-15 (RESULT+$2E, 1 byte)

    JMP START

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
; PSG SOUND EFFECTS PLAYER RUNTIME (.vsfx format)
; ============================================================================
; SFX data structure (.vsfx compiled format):
;   +0: FCB flags (bit0=pitch, bit1=noise, bit2=arp, bit3=vib)
;   +1: FCB duration (frames)
;   +2: FCB channel (0=A, 1=B, 2=C)
;   +3: FDB base_period (PSG period, 12-bit)
;   +5: FCB attack, decay, sustain, release (frames/level)
;   +9: FCB peak_volume (0-15)
;   +10: [optional] FDB pitch_start_mult, pitch_end_mult, FCB curve (if flag bit0)
;   +15: [optional] FCB noise_period, noise_volume, noise_decay (if flag bit1)
;
; RAM variables (defined in RAM section):
;   SFX_PTR     - Pointer to current SFX data
;   SFX_TICK    - Current frame counter (16-bit enough for SFX)
;   SFX_ACTIVE  - 0=stopped, 1=playing
;   SFX_PHASE   - 0=attack, 1=decay, 2=sustain, 3=release
;   SFX_VOL     - Current volume (0-15)
; ============================================================================

; PLAY_SFX_RUNTIME - Initialize and start SFX playback
; Input: X = pointer to SFX data structure
PLAY_SFX_RUNTIME:
STX SFX_PTR           ; Store SFX pointer

; Reset playback state
CLRA
CLRB
STD SFX_TICK          ; Reset frame counter
STA SFX_PHASE         ; Start in attack phase
STA SFX_VOL           ; Start at 0 volume

; Mark as active
LDA #1
STA SFX_ACTIVE

; Set initial frequency from base period
LDD 3,X               ; Load base period
JSR SFX_SET_FREQ      ; Set PSG frequency

RTS

; ============================================================================
; SFX_UPDATE - Process one frame of SFX (call from loop, AFTER MUSIC_UPDATE)
; ============================================================================
SFX_UPDATE:
PSHS A,B,X,Y,U

; Check if SFX is active
TST SFX_ACTIVE
BEQ SFX_UPDATE_done   ; Not playing, skip

LDX SFX_PTR           ; Load SFX data pointer

; Increment frame counter
LDD SFX_TICK
ADDD #1
STD SFX_TICK

; Check if duration exceeded
LDB 1,X               ; Duration in frames
CLRA
CMPD SFX_TICK         ; Compare duration with current tick
BLS SFX_STOP          ; Stop if tick >= duration

; Process envelope (ADSR)
JSR SFX_ENVELOPE

; Process pitch sweep (if enabled)
LDA ,X                ; Load flags
BITA #$01             ; Check pitch flag
BEQ SFX_NO_PITCH
JSR SFX_PITCH_SWEEP
SFX_NO_PITCH:

; Process noise (if enabled)
LDA ,X
BITA #$02             ; Check noise flag
BEQ SFX_UPDATE_done
JSR SFX_NOISE

SFX_UPDATE_done:
PULS A,B,X,Y,U
RTS

SFX_STOP:
; Turn off sound on this channel
LDX SFX_PTR
LDB 2,X               ; Channel number
CLRA                  ; Volume = 0
JSR SFX_SET_VOL
CLR SFX_ACTIVE
PULS A,B,X,Y,U
RTS

; ============================================================================
; SFX_ENVELOPE - Process ADSR envelope
; ============================================================================
SFX_ENVELOPE:
LDX SFX_PTR
LDA SFX_PHASE         ; Current phase

CMPA #0               ; Attack?
BNE SFX_ENV_DECAY
; Attack phase: ramp up to peak
LDB 9,X               ; Peak volume
LDA SFX_VOL
INCA                  ; Increase volume
CMPA 9,X              ; Reached peak?
BLO SFX_ENV_SET
LDA 9,X               ; Clamp to peak
LDB #1
STB SFX_PHASE         ; Move to decay phase
BRA SFX_ENV_SET

SFX_ENV_DECAY:
CMPA #1               ; Decay?
BNE SFX_ENV_SUSTAIN
; Decay phase: ramp down to sustain
LDA SFX_VOL
DECA                  ; Decrease volume
BMI SFX_ENV_TO_SUSTAIN
CMPA 7,X              ; Reached sustain level?
BHI SFX_ENV_SET
SFX_ENV_TO_SUSTAIN:
LDA 7,X               ; Sustain level
LDB #2
STB SFX_PHASE         ; Move to sustain
BRA SFX_ENV_SET

SFX_ENV_SUSTAIN:
CMPA #2               ; Sustain?
BNE SFX_ENV_RELEASE
; Sustain: hold at sustain level
LDA 7,X               ; Sustain level
; Check if we should transition to release
LDD SFX_TICK          ; Current tick
LDB 1,X               ; Total duration
SUBB 8,X              ; Minus release time
BCS SFX_ENV_SET       ; If release > duration, stay in sustain
CLRA
CMPD SFX_TICK
BHI SFX_ENV_SET       ; Still in sustain period
; Time for release
LDA 7,X               ; Sustain level
LDB #3
STB SFX_PHASE
BRA SFX_ENV_SET

SFX_ENV_RELEASE:
; Release: ramp down to 0
LDA SFX_VOL
BEQ SFX_ENV_SET       ; Already at 0
DECA

SFX_ENV_SET:
STA SFX_VOL
LDB 2,X               ; Channel
JSR SFX_SET_VOL       ; Set PSG volume
RTS

; ============================================================================
; SFX_PITCH_SWEEP - Linear pitch interpolation
; ============================================================================
SFX_PITCH_SWEEP:
; Simple linear interpolation between start and end frequency
; Using base period * multiplier (8.8 fixed point)
LDX SFX_PTR
LDD 3,X               ; Base period
; For now, just use base period (full pitch sweep requires more math)
; TODO: Implement proper interpolation with 8.8 fixed point
JSR SFX_SET_FREQ
RTS

; ============================================================================
; SFX_NOISE - Process noise channel
; ============================================================================
SFX_NOISE:
; Get noise parameters from SFX data
; Offset depends on whether pitch sweep is enabled
LDX SFX_PTR
LDA ,X                ; Flags
BITA #$01             ; Pitch enabled?
BEQ SFX_NOISE_NO_PITCH
LDY #15               ; Offset with pitch data
BRA SFX_NOISE_READ
SFX_NOISE_NO_PITCH:
LDY #10               ; Offset without pitch data
SFX_NOISE_READ:
; Read noise period and set PSG noise register
LDA A,X               ; Noise period (Y offset in A is wrong, use indexed)
; TODO: Proper indexed access
; For now, use fixed noise
LDA #8                ; Default noise period
STA >$D006            ; PSG noise period register
RTS

; ============================================================================
; SFX_SET_FREQ - Set PSG frequency for SFX channel
; Input: D = period (12-bit), X = SFX_PTR
; ============================================================================
SFX_SET_FREQ:
PSHS D
LDX SFX_PTR
LDB 2,X               ; Channel (0/1/2)

; Channel A = $D000/$D001, B = $D002/$D003, C = $D004/$D005
ASLB                  ; Channel * 2
TFR B,Y               ; Y = register offset

PULS D                ; Restore period

; Set VIA for PSG access
LDA #$FF
STA >$D003            ; VIA port A direction = output

; Write low byte of period
TFR B,A               ; Low 8 bits
STA >$D001            ; Data to port A
TFR Y,A               ; Register number (0/2/4)
ORA #$80              ; Set write bit
STA >$D000            ; Select register
ANDA #$7F             ; Clear write bit
STA >$D000            ; Latch

; Write high 4 bits of period
PULS D                ; Original D (stored on stack twice? fix)
; TODO: Fix period high bits
RTS

; ============================================================================
; SFX_SET_VOL - Set PSG volume for channel
; Input: A = volume (0-15), B = channel (0/1/2)
; ============================================================================
SFX_SET_VOL:
; Volume registers are 8/9/10 for channels A/B/C
ADDB #8               ; Register = 8 + channel

; Set VIA for PSG access
PSHS A,B
LDA #$FF
STA >$D003            ; VIA port A direction = output
PULS A,B

; Write volume
PSHS B
STA >$D001            ; Volume to port A
PULS A                ; Register number from B
ORA #$80              ; Set write bit
STA >$D000            ; Select register
ANDA #$7F             ; Clear write bit
STA >$D000            ; Latch
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
Draw_Sync_List_At_With_Mirrors:
; Unified mirror support using flags: MIRROR_X and MIRROR_Y
; Conditionally negates X and/or Y coordinates and deltas
LDA ,X+                 ; intensity
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
LDA ,X+                 ; Read intensity
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
START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S
    JSR $F533       ; Init_Music_Buf - Initialize BIOS music system to silence

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:5
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 5
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
    ; DEBUG: Processing 2 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(8)
    ; VPy_LINE:9
; PLAY_SFX("laser") - play sound effect (one-shot)
    LDX #_LASER_SFX
    JSR PLAY_SFX_RUNTIME
    LDD #0
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(8)
    ; VPy_LINE:10
    ; StructInit(SFX_UPDATE)
    LDD #0
    STD RESULT
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
TEMP_YX   EQU RESULT+26   ; Temporary y,x storage (2 bytes)
TEMP_X    EQU RESULT+28   ; Temporary x storage (1 byte)
TEMP_Y    EQU RESULT+29   ; Temporary y storage (1 byte)
VL_PTR     EQU $CF80      ; Current position in vector list
VL_Y       EQU $CF82      ; Y position (1 byte)
VL_X       EQU $CF83      ; X position (1 byte)
VL_SCALE   EQU $CF84      ; Scale factor (1 byte)
; Call argument scratch space
VAR_ARG0 EQU $C8B2
VAR_ARG1 EQU $C8B4

; ========================================
; ASSET DATA SECTION
; Embedded 1 of 1 assets (unused assets excluded)
; ========================================

; ========================================
; SFX Asset: laser (from /Users/daniel/projects/vectrex-pseudo-python/examples/sfx_test/assets/sfx/laser.vsfx)
; ========================================
_LASER_SFX:
    ; SFX: laser (laser)
    FCB $01        ; flags (pitch=1, noise=0, arp=0, vib=0)
    FCB 7         ; duration (frames)
    FCB 0          ; channel
    FDB 53        ; base period (PSG)
    FCB 0, 0, 12, 5 ; A, D, S, R (frames/level)
    FCB 15         ; peak volume
    FDB $0200     ; pitch start mult (8.8 fixed)
    FDB $0080     ; pitch end mult (8.8 fixed)
    FCB -2          ; pitch curve


DRAW_VEC_X EQU RESULT+0
DRAW_VEC_Y EQU RESULT+1
MIRROR_X EQU RESULT+2
MIRROR_Y EQU RESULT+3
