; --- Motorola 6809 backend (Vectrex) title='PANG' origin=$0000 ---
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
    FCC "PANG"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 37 bytes
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$02   ; Left operand temp (2 bytes)
TMPRIGHT             EQU $C880+$04   ; Right operand temp (2 bytes)
TMPPTR               EQU $C880+$06   ; Pointer temp (2 bytes)
DIV_A                EQU $C880+$08   ; Dividend (2 bytes)
DIV_B                EQU $C880+$0A   ; Divisor (2 bytes)
DIV_Q                EQU $C880+$0C   ; Quotient (2 bytes)
DIV_R                EQU $C880+$0E   ; Remainder (2 bytes)
TEMP_YX              EQU $C880+$10   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$12   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$13   ; Temporary y storage (1 bytes)
PSG_MUSIC_PTR        EQU $C880+$14   ; Current music position pointer (2 bytes)
PSG_MUSIC_START      EQU $C880+$16   ; Music start pointer (for loops) (2 bytes)
PSG_IS_PLAYING       EQU $C880+$18   ; Playing flag ($00=stopped, $01=playing) (1 bytes)
PSG_MUSIC_ACTIVE     EQU $C880+$19   ; Set during UPDATE_MUSIC_PSG (1 bytes)
PSG_FRAME_COUNT      EQU $C880+$1A   ; Frame register write count (1 bytes)
PSG_DELAY_FRAMES     EQU $C880+$1B   ; Frames to wait before next read (1 bytes)
SFX_PTR              EQU $C880+$1C   ; Current SFX data pointer (2 bytes)
SFX_TICK             EQU $C880+$1E   ; Current frame counter (2 bytes)
SFX_ACTIVE           EQU $C880+$20   ; Playback state ($00=stopped, $01=playing) (1 bytes)
SFX_PHASE            EQU $C880+$21   ; Envelope phase (0=A,1=D,2=S,3=R) (1 bytes)
SFX_VOL              EQU $C880+$22   ; Current volume level (0-15) (1 bytes)
NUM_STR              EQU $C880+$23   ; String buffer for PRINT_NUMBER (2 bytes)
PSG_MUSIC_PTR_DP   EQU $14  ; DP-relative
PSG_MUSIC_START_DP EQU $16  ; DP-relative
PSG_IS_PLAYING_DP  EQU $18  ; DP-relative
PSG_MUSIC_ACTIVE_DP EQU $19  ; DP-relative
PSG_FRAME_COUNT_DP EQU $1A  ; DP-relative
PSG_DELAY_FRAMES_DP EQU $1B  ; DP-relative

    JMP START

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
    ; DO NOT RESTORE DP - Keep it at $D0 for subsequent vector drawing
    ; BIOS calls after this will handle DP correctly
    RTS
; DRAW_LINE unified wrapper - handles 16-bit signed coordinates correctly
; Args: (x0,y0,x1,y1,intensity) as 16-bit words, treating x/y as signed bytes.
; ALWAYS sets intensity. Does NOT reset origin (allows connected lines).
DRAW_LINE_WRAPPER:
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode for vector drawing)
    STA >$D00C     ; VIA_cntl
    ; Set DP to hardware registers
    LDA #$D0
    TFR A,DP
    ; ALWAYS set intensity (no optimization)
    LDA VAR_ARG4+1
    JSR Intensity_a
    ; Move to start (y in A, x in B) - use signed byte values
    LDA VAR_ARG1+1  ; Y start (signed byte)
    LDB VAR_ARG0+1  ; X start (signed byte)
    JSR Moveto_d
    ; Compute deltas using 16-bit arithmetic, then clamp to signed bytes
    ; dx = x1 - x0 (treating as signed)
    LDD VAR_ARG2    ; x1 (16-bit)
    SUBD VAR_ARG0   ; subtract x0 (16-bit)
    ; Clamp D to signed byte range (-128 to +127)
    CMPD #127
    BLE DLW_DX_CLAMP_HI_OK
    LDD #127
DLW_DX_CLAMP_HI_OK:
    CMPD #-128
    BGE DLW_DX_CLAMP_LO_OK
    LDD #-128
DLW_DX_CLAMP_LO_OK:
    STB VLINE_DX    ; Store dx as signed byte
    ; dy = y1 - y0 (treating as signed)
    LDD VAR_ARG3    ; y1 (16-bit)
    SUBD VAR_ARG1   ; subtract y0 (16-bit)
    ; Clamp D to signed byte range (-128 to +127)
    CMPD #127
    BLE DLW_DY_CLAMP_HI_OK
    LDD #127
DLW_DY_CLAMP_HI_OK:
    CMPD #-128
    BGE DLW_DY_CLAMP_LO_OK
    LDD #-128
DLW_DY_CLAMP_LO_OK:
    STB VLINE_DY    ; Store dy as signed byte
    ; dx and dy are already clamped to ±127 - no further clamping needed
    ; Vectrex hardware supports full ±127 delta range
    LDA VLINE_DX
    STA VLINE_DX    ; Keep full range
    LDA VLINE_DY
    STA VLINE_DY    ; Keep full range
    ; Clear Vec_Misc_Count for proper timing
    CLR Vec_Misc_Count
    ; Draw line (A=dy, B=dx)
    LDA VLINE_DY
    LDB VLINE_DX
    JSR Draw_Line_d
    LDA #$C8       ; CRITICAL: Restore DP to $C8 for our code
    TFR A,DP
    RTS

; DRAW_LINE_FAST - optimized version that skips redundant setup
; Use this for multiple consecutive draws with same intensity
; Args: (x0,y0,x1,y1) only - intensity must be set beforehand
DRAW_LINE_FAST:
    ; Move to start (y in A, x in B) - use signed byte values
    LDA VAR_ARG1+1  ; Y start (signed byte)
    LDB VAR_ARG0+1  ; X start (signed byte)
    JSR Moveto_d
    ; Compute deltas using 16-bit arithmetic, then clamp to signed bytes
    ; dx = x1 - x0 (treating as signed)
    LDD VAR_ARG2    ; x1 (16-bit)
    SUBD VAR_ARG0   ; subtract x0 (16-bit)
    ; Clamp D to signed byte range (-128 to +127)
    CMPD #127
    BLE DLF_DX_CLAMP_HI_OK
    LDD #127
DLF_DX_CLAMP_HI_OK:
    CMPD #-128
    BGE DLF_DX_CLAMP_LO_OK
    LDD #-128
DLF_DX_CLAMP_LO_OK:
    STB VLINE_DX    ; Store dx as signed byte
    ; dy = y1 - y0 (treating as signed)
    LDD VAR_ARG3    ; y1 (16-bit)
    SUBD VAR_ARG1   ; subtract y0 (16-bit)
    ; Clamp D to signed byte range (-128 to +127)
    CMPD #127
    BLE DLF_DY_CLAMP_HI_OK
    LDD #127
DLF_DY_CLAMP_HI_OK:
    CMPD #-128
    BGE DLF_DY_CLAMP_LO_OK
    LDD #-128
DLF_DY_CLAMP_LO_OK:
    STB VLINE_DY    ; Store dy as signed byte
    ; dx and dy are already clamped to ±127 - no further clamping needed
    ; Vectrex hardware supports full ±127 delta range
    LDA VLINE_DX
    STA VLINE_DX    ; Keep full range
    LDA VLINE_DY
    STA VLINE_DY    ; Keep full range
    ; Clear Vec_Misc_Count for proper timing
    CLR Vec_Misc_Count
    ; Draw line (A=dy, B=dx)
    LDA VLINE_DY
    LDB VLINE_DX
    JSR Draw_Line_d
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
START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S
    JSR $F533       ; Init_Music_Buf - Initialize BIOS music system to silence

    ; *** DEBUG *** main() function code inline (initialization)
    LDD #0
    STD RESULT
    STD VAR_SCREEN
    LDD #30
    STD VAR_TITLE_INTENSITY
    LDD #0
    STD VAR_TITLE_STATE
    LDD #65535
    STD VAR_CURRENT_MUSIC
    LDD #0
    STD VAR_DELAY
    LDD #0
    STD VAR_STAGE
    LDD #0
    STD VAR_COUNTDOWN_TIMER
    LDD #0
    STD VAR_COUNTDOWN_ACTIVE
    LDD #17
    STD VAR_NUM_LOCATIONS
    LDD #0
    STD VAR_CURRENT_LOCATION
    LDD #0
    STD VAR_MAX_LEVEL_UNLOCKED
    LDD #60
    STD VAR_LOCATION_GLOW_INTENSITY
    LDD #0
    STD VAR_LOCATION_GLOW_DIRECTION
    LDD #0
    STD VAR_PREV_JOY_X
    LDD #0
    STD VAR_PREV_JOY_Y
    LDD #0
    STD VAR_PREV_BTN_1
    LDD #0
    STD VAR_INTENSITYVAL
    LDD #0
    STD VAR_PLAYER_X
    LDD #65436
    STD VAR_PLAYER_Y
    LDD #2
    STD VAR_PLAYER_SPEED
    ; VPy_LINE:62
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    ; VPy_LINE:63
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_MAX_LEVEL_UNLOCKED
    STU TMPPTR
    STX ,U
    ; VPy_LINE:64
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_PREV_JOY_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:65
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_PREV_JOY_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:66
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_PREV_BTN_1
    STU TMPPTR
    STX ,U
    ; VPy_LINE:67
    LDD #60
    STD RESULT
    LDX RESULT
    LDU #VAR_LOCATION_GLOW_INTENSITY
    STU TMPPTR
    STX ,U
    ; VPy_LINE:68
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_LOCATION_GLOW_DIRECTION
    STU TMPPTR
    STX ,U
    ; VPy_LINE:69
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_SCREEN
    STU TMPPTR
    STX ,U
    ; VPy_LINE:70
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTDOWN_TIMER
    STU TMPPTR
    STX ,U
    ; VPy_LINE:71
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTDOWN_ACTIVE
    STU TMPPTR
    STX ,U
    ; VPy_LINE:74
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:75
    LDD #65436
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_Y
    STU TMPPTR
    STX ,U

MAIN:
    JSR Wait_Recal
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    BRA MAIN

STATE_TITLE EQU 0
STATE_MAP EQU 1
STATE_GAME EQU 2
STATE_LEVEL_COMPLETE EQU 3
STATE_GAME_OVER EQU 4
LOOP_BODY:
    LEAS -12,S ; allocate locals
    ; DEBUG: Processing 2 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(8)
    ; VPy_LINE:78
; NATIVE_CALL: VECTREX_WAIT_RECAL at line 78
    JSR VECTREX_WAIT_RECAL
    CLRA
    CLRB
    STD RESULT
    ; DEBUG: Statement 1 - Discriminant(9)
    ; VPy_LINE:81
    LDD VAR_SCREEN
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
    ; VPy_LINE:82
    LDD VAR_CURRENT_MUSIC
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_6
    LDD #0
    STD RESULT
    BRA CE_7
CT_6:
    LDD #1
    STD RESULT
CE_7:
    LDD RESULT
    LBEQ IF_NEXT_5
    ; VPy_LINE:83
; PLAY_MUSIC("pang_theme") - play music asset
    LDX #_PANG_THEME_MUSIC
    JSR PLAY_MUSIC_RUNTIME
    LDD #0
    STD RESULT
    ; VPy_LINE:84
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_MUSIC
    STU TMPPTR
    STX ,U
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    ; VPy_LINE:86
    JSR DRAW_TITLE_SCREEN
    ; VPy_LINE:88
; NATIVE_CALL: J1_BUTTON_1 at line 88
; J1_BUTTON_1() - Read Joystick 1 button 1 (BIOS)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$01
    BEQ .j1b1_not_pressed
    LDD #1
    BRA .j1b1_done
.j1b1_not_pressed:
    LDD #0
.j1b1_done:
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:89
; NATIVE_CALL: J1_BUTTON_2 at line 89
; J1_BUTTON_2() - Read Joystick 1 button 2 (BIOS)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$02
    BEQ .j1b2_not_pressed
    LDD #1
    BRA .j1b2_done
.j1b2_not_pressed:
    LDD #0
.j1b2_done:
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:90
; NATIVE_CALL: J1_BUTTON_3 at line 90
; J1_BUTTON_3() - Read Joystick 1 button 3 (BIOS)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$04
    BEQ .j1b3_not_pressed
    LDD #1
    BRA .j1b3_done
.j1b3_not_pressed:
    LDD #0
.j1b3_done:
    STD RESULT
    LDX RESULT
    STX 4 ,S
    ; VPy_LINE:91
; NATIVE_CALL: J1_BUTTON_4 at line 91
; J1_BUTTON_4() - Read Joystick 1 button 4 (BIOS)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$08
    BEQ .j1b4_not_pressed
    LDD #1
    BRA .j1b4_done
.j1b4_not_pressed:
    LDD #0
.j1b4_done:
    STD RESULT
    LDX RESULT
    STX 6 ,S
    ; VPy_LINE:93
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_16
    LDD #0
    STD RESULT
    BRA CE_17
CT_16:
    LDD #1
    STD RESULT
CE_17:
    LDD RESULT
    BNE OR_TRUE_14
    LDD 2 ,S
    STD RESULT
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
    BNE OR_TRUE_14
    LDD #0
    STD RESULT
    BRA OR_END_15
OR_TRUE_14:
    LDD #1
    STD RESULT
OR_END_15:
    LDD RESULT
    BNE OR_TRUE_12
    LDD 4 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_20
    LDD #0
    STD RESULT
    BRA CE_21
CT_20:
    LDD #1
    STD RESULT
CE_21:
    LDD RESULT
    BNE OR_TRUE_12
    LDD #0
    STD RESULT
    BRA OR_END_13
OR_TRUE_12:
    LDD #1
    STD RESULT
OR_END_13:
    LDD RESULT
    BNE OR_TRUE_10
    LDD 6 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_22
    LDD #0
    STD RESULT
    BRA CE_23
CT_22:
    LDD #1
    STD RESULT
CE_23:
    LDD RESULT
    BNE OR_TRUE_10
    LDD #0
    STD RESULT
    BRA OR_END_11
OR_TRUE_10:
    LDD #1
    STD RESULT
OR_END_11:
    LDD RESULT
    LBEQ IF_NEXT_9
    ; VPy_LINE:94
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_SCREEN
    STU TMPPTR
    STX ,U
    ; VPy_LINE:95
    LDD #65535
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_MUSIC
    STU TMPPTR
    STX ,U
    ; VPy_LINE:96
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_DELAY
    STU TMPPTR
    STX ,U
    LBRA IF_END_8
IF_NEXT_9:
IF_END_8:
    LBRA IF_END_0
IF_NEXT_1:
    LDD VAR_SCREEN
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_25
    LDD #0
    STD RESULT
    BRA CE_26
CT_25:
    LDD #1
    STD RESULT
CE_26:
    LDD RESULT
    LBEQ IF_NEXT_24
    ; VPy_LINE:100
    LDD VAR_DELAY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_29
    LDD #0
    STD RESULT
    BRA CE_30
CT_29:
    LDD #1
    STD RESULT
CE_30:
    LDD RESULT
    LBEQ IF_NEXT_28
    ; VPy_LINE:101
; NATIVE_CALL: STOP_MUSIC at line 101
; STOP_MUSIC() - stop background music
    JSR STOP_MUSIC_RUNTIME
    LDD #0
    STD RESULT
    LBRA IF_END_27
IF_NEXT_28:
IF_END_27:
    ; VPy_LINE:103
    LDD VAR_CURRENT_MUSIC
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BNE CT_33
    LDD #0
    STD RESULT
    BRA CE_34
CT_33:
    LDD #1
    STD RESULT
CE_34:
    LDD RESULT
    BEQ AND_FALSE_35
    LDD VAR_DELAY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_37
    LDD #0
    STD RESULT
    BRA CE_38
CT_37:
    LDD #1
    STD RESULT
CE_38:
    LDD RESULT
    BEQ AND_FALSE_35
    LDD #1
    STD RESULT
    BRA AND_END_36
AND_FALSE_35:
    LDD #0
    STD RESULT
AND_END_36:
    LDD RESULT
    LBEQ IF_NEXT_32
    ; VPy_LINE:104
; PLAY_MUSIC("map_theme") - play music asset
    LDX #_MAP_THEME_MUSIC
    JSR PLAY_MUSIC_RUNTIME
    LDD #0
    STD RESULT
    ; VPy_LINE:105
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_MUSIC
    STU TMPPTR
    STX ,U
    LBRA IF_END_31
IF_NEXT_32:
IF_END_31:
    ; VPy_LINE:107
    LDD VAR_DELAY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_41
    LDD #0
    STD RESULT
    BRA CE_42
CT_41:
    LDD #1
    STD RESULT
CE_42:
    LDD RESULT
    LBEQ IF_NEXT_40
    ; VPy_LINE:108
    LDD VAR_DELAY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_DELAY
    STU TMPPTR
    STX ,U
    LBRA IF_END_39
IF_NEXT_40:
IF_END_39:
    ; VPy_LINE:110
; NATIVE_CALL: J1_X at line 110
; J1_X() - Read Joystick 1 X axis (Digital from RAM)
; Frontend writes unsigned 0-255 to $CF00 (128=center)
    LDB $CF00    ; Vec_Joy_1_X (0=left, 128=center, 255=right)
; Convert unsigned to digital: <108=left(-1), 108-148=center(0), >148=right(+1)
    CMPB #108    ; Check if < 108 (left)
    BLO J1X_LEFT ; Branch if lower (unsigned)
    CMPB #148    ; Check if > 148 (right)
    BHI J1X_RIGHT ; Branch if higher (unsigned)
    LDD #0       ; Center
    BRA J1X_END
J1X_LEFT:
    LDD #$FFFF   ; Left (-1)
    BRA J1X_END
J1X_RIGHT:
    LDD #1       ; Right (+1)
J1X_END:
    STD RESULT
    LDX RESULT
    STX 8 ,S
    ; VPy_LINE:111
; NATIVE_CALL: J1_Y at line 111
; J1_Y() - Read Joystick 1 Y axis (Digital from RAM)
; Frontend writes unsigned 0-255 to $CF01 (128=center)
    LDB $CF01    ; Vec_Joy_1_Y (0=down, 128=center, 255=up)
; Convert unsigned to digital: <108=down(-1), 108-148=center(0), >148=up(+1)
    CMPB #108    ; Check if < 108 (down)
    BLO J1Y_DOWN ; Branch if lower (unsigned)
    CMPB #148    ; Check if > 148 (up)
    BHI J1Y_UP   ; Branch if higher (unsigned)
    LDD #0       ; Center
    BRA J1Y_END
J1Y_DOWN:
    LDD #$FFFF   ; Down (-1)
    BRA J1Y_END
J1Y_UP:
    LDD #1       ; Up (+1)
J1Y_END:
    STD RESULT
    LDX RESULT
    STX 10 ,S
    ; VPy_LINE:113
    LDD 8 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_45
    LDD #0
    STD RESULT
    BRA CE_46
CT_45:
    LDD #1
    STD RESULT
CE_46:
    LDD RESULT
    BEQ AND_FALSE_47
    LDD VAR_PREV_JOY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BNE CT_49
    LDD #0
    STD RESULT
    BRA CE_50
CT_49:
    LDD #1
    STD RESULT
CE_50:
    LDD RESULT
    BEQ AND_FALSE_47
    LDD #1
    STD RESULT
    BRA AND_END_48
AND_FALSE_47:
    LDD #0
    STD RESULT
AND_END_48:
    LDD RESULT
    LBEQ IF_NEXT_44
    ; VPy_LINE:114
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_MAX_LEVEL_UNLOCKED
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_53
    LDD #0
    STD RESULT
    BRA CE_54
CT_53:
    LDD #1
    STD RESULT
CE_54:
    LDD RESULT
    LBEQ IF_NEXT_52
    ; VPy_LINE:115
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    LBRA IF_END_51
IF_NEXT_52:
IF_END_51:
    LBRA IF_END_43
IF_NEXT_44:
    LDD 8 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_56
    LDD #0
    STD RESULT
    BRA CE_57
CT_56:
    LDD #1
    STD RESULT
CE_57:
    LDD RESULT
    BEQ AND_FALSE_58
    LDD VAR_PREV_JOY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BNE CT_60
    LDD #0
    STD RESULT
    BRA CE_61
CT_60:
    LDD #1
    STD RESULT
CE_61:
    LDD RESULT
    BEQ AND_FALSE_58
    LDD #1
    STD RESULT
    BRA AND_END_59
AND_FALSE_58:
    LDD #0
    STD RESULT
AND_END_59:
    LDD RESULT
    LBEQ IF_NEXT_55
    ; VPy_LINE:117
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_64
    LDD #0
    STD RESULT
    BRA CE_65
CT_64:
    LDD #1
    STD RESULT
CE_65:
    LDD RESULT
    LBEQ IF_NEXT_63
    ; VPy_LINE:118
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    LBRA IF_END_62
IF_NEXT_63:
IF_END_62:
    LBRA IF_END_43
IF_NEXT_55:
    LDD 10 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_67
    LDD #0
    STD RESULT
    BRA CE_68
CT_67:
    LDD #1
    STD RESULT
CE_68:
    LDD RESULT
    BEQ AND_FALSE_69
    LDD VAR_PREV_JOY_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BNE CT_71
    LDD #0
    STD RESULT
    BRA CE_72
CT_71:
    LDD #1
    STD RESULT
CE_72:
    LDD RESULT
    BEQ AND_FALSE_69
    LDD #1
    STD RESULT
    BRA AND_END_70
AND_FALSE_69:
    LDD #0
    STD RESULT
AND_END_70:
    LDD RESULT
    LBEQ IF_NEXT_66
    ; VPy_LINE:120
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_MAX_LEVEL_UNLOCKED
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_75
    LDD #0
    STD RESULT
    BRA CE_76
CT_75:
    LDD #1
    STD RESULT
CE_76:
    LDD RESULT
    LBEQ IF_NEXT_74
    ; VPy_LINE:121
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    LBRA IF_END_73
IF_NEXT_74:
IF_END_73:
    LBRA IF_END_43
IF_NEXT_66:
    LDD 10 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_77
    LDD #0
    STD RESULT
    BRA CE_78
CT_77:
    LDD #1
    STD RESULT
CE_78:
    LDD RESULT
    BEQ AND_FALSE_79
    LDD VAR_PREV_JOY_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BNE CT_81
    LDD #0
    STD RESULT
    BRA CE_82
CT_81:
    LDD #1
    STD RESULT
CE_82:
    LDD RESULT
    BEQ AND_FALSE_79
    LDD #1
    STD RESULT
    BRA AND_END_80
AND_FALSE_79:
    LDD #0
    STD RESULT
AND_END_80:
    LDD RESULT
    LBEQ IF_END_43
    ; VPy_LINE:123
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_85
    LDD #0
    STD RESULT
    BRA CE_86
CT_85:
    LDD #1
    STD RESULT
CE_86:
    LDD RESULT
    LBEQ IF_NEXT_84
    ; VPy_LINE:124
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    LBRA IF_END_83
IF_NEXT_84:
IF_END_83:
    LBRA IF_END_43
IF_END_43:
    ; VPy_LINE:126
    LDD 8 ,S
    STD RESULT
    LDX RESULT
    LDU #VAR_PREV_JOY_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:127
    LDD 10 ,S
    STD RESULT
    LDX RESULT
    LDU #VAR_PREV_JOY_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:129
; NATIVE_CALL: J1_BUTTON_1 at line 129
; J1_BUTTON_1() - Read Joystick 1 button 1 (BIOS)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$01
    BEQ .j1b1_not_pressed
    LDD #1
    BRA .j1b1_done
.j1b1_not_pressed:
    LDD #0
.j1b1_done:
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:130
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_89
    LDD #0
    STD RESULT
    BRA CE_90
CT_89:
    LDD #1
    STD RESULT
CE_90:
    LDD RESULT
    BEQ AND_FALSE_91
    LDD VAR_PREV_BTN_1
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_93
    LDD #0
    STD RESULT
    BRA CE_94
CT_93:
    LDD #1
    STD RESULT
CE_94:
    LDD RESULT
    BEQ AND_FALSE_91
    LDD #1
    STD RESULT
    BRA AND_END_92
AND_FALSE_91:
    LDD #0
    STD RESULT
AND_END_92:
    LDD RESULT
    LBEQ IF_NEXT_88
    ; VPy_LINE:131
    LDD #2
    STD RESULT
    LDX RESULT
    LDU #VAR_SCREEN
    STU TMPPTR
    STX ,U
    ; VPy_LINE:132
    LDD #180
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTDOWN_TIMER
    STU TMPPTR
    STX ,U
    ; VPy_LINE:133
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTDOWN_ACTIVE
    STU TMPPTR
    STX ,U
    ; VPy_LINE:134
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:135
    LDD #65535
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_MUSIC
    STU TMPPTR
    STX ,U
    LBRA IF_END_87
IF_NEXT_88:
IF_END_87:
    ; VPy_LINE:137
    LDD 0 ,S
    STD RESULT
    LDX RESULT
    LDU #VAR_PREV_BTN_1
    STU TMPPTR
    STX ,U
    ; VPy_LINE:139
    JSR DRAW_MAP_SCREEN
    LBRA IF_END_0
IF_NEXT_24:
    LDD VAR_SCREEN
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_95
    LDD #0
    STD RESULT
    BRA CE_96
CT_95:
    LDD #1
    STD RESULT
CE_96:
    LDD RESULT
    LBEQ IF_END_0
    ; VPy_LINE:143
    LDD VAR_COUNTDOWN_ACTIVE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_99
    LDD #0
    STD RESULT
    BRA CE_100
CT_99:
    LDD #1
    STD RESULT
CE_100:
    LDD RESULT
    LBEQ IF_NEXT_98
    ; VPy_LINE:144
    JSR DRAW_COUNTDOWN_SCREEN
    ; VPy_LINE:145
    LDD VAR_COUNTDOWN_TIMER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTDOWN_TIMER
    STU TMPPTR
    STX ,U
    ; VPy_LINE:147
    LDD VAR_COUNTDOWN_TIMER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLE CT_103
    LDD #0
    STD RESULT
    BRA CE_104
CT_103:
    LDD #1
    STD RESULT
CE_104:
    LDD RESULT
    LBEQ IF_NEXT_102
    ; VPy_LINE:148
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTDOWN_ACTIVE
    STU TMPPTR
    STX ,U
    ; VPy_LINE:149
; PLAY_MUSIC("map_theme") - play music asset
    LDX #_MAP_THEME_MUSIC
    JSR PLAY_MUSIC_RUNTIME
    LDD #0
    STD RESULT
    ; VPy_LINE:150
    LDD #2
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_MUSIC
    STU TMPPTR
    STX ,U
    LBRA IF_END_101
IF_NEXT_102:
IF_END_101:
    LBRA IF_END_97
IF_NEXT_98:
    ; VPy_LINE:152
    JSR HANDLE_GAMEPLAY
IF_END_97:
    LBRA IF_END_0
IF_END_0:
    JSR AUDIO_UPDATE  ; Auto-injected: update music + SFX (at end)
    LEAS 12,S ; free locals
    RTS

HANDLE_GAMEPLAY: ; function
; --- function handle_gameplay ---
    LEAS -2,S ; allocate locals
    ; VPy_LINE:155
; NATIVE_CALL: J1_X at line 155
; J1_X() - Read Joystick 1 X axis (Digital from RAM)
; Frontend writes unsigned 0-255 to $CF00 (128=center)
    LDB $CF00    ; Vec_Joy_1_X (0=left, 128=center, 255=right)
; Convert unsigned to digital: <108=left(-1), 108-148=center(0), >148=right(+1)
    CMPB #108    ; Check if < 108 (left)
    BLO J1X_LEFT ; Branch if lower (unsigned)
    CMPB #148    ; Check if > 148 (right)
    BHI J1X_RIGHT ; Branch if higher (unsigned)
    LDD #0       ; Center
    BRA J1X_END
J1X_LEFT:
    LDD #$FFFF   ; Left (-1)
    BRA J1X_END
J1X_RIGHT:
    LDD #1       ; Right (+1)
J1X_END:
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:156
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_107
    LDD #0
    STD RESULT
    BRA CE_108
CT_107:
    LDD #1
    STD RESULT
CE_108:
    LDD RESULT
    LBEQ IF_NEXT_106
    ; VPy_LINE:157
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_PLAYER_SPEED
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:158
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65416
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_111
    LDD #0
    STD RESULT
    BRA CE_112
CT_111:
    LDD #1
    STD RESULT
CE_112:
    LDD RESULT
    LBEQ IF_NEXT_110
    ; VPy_LINE:159
    LDD #65416
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_109
IF_NEXT_110:
IF_END_109:
    LBRA IF_END_105
IF_NEXT_106:
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_113
    LDD #0
    STD RESULT
    BRA CE_114
CT_113:
    LDD #1
    STD RESULT
CE_114:
    LDD RESULT
    LBEQ IF_END_105
    ; VPy_LINE:161
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_PLAYER_SPEED
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:162
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #120
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_117
    LDD #0
    STD RESULT
    BRA CE_118
CT_117:
    LDD #1
    STD RESULT
CE_118:
    LDD RESULT
    LBEQ IF_NEXT_116
    ; VPy_LINE:163
    LDD #120
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_115
IF_NEXT_116:
IF_END_115:
    LBRA IF_END_105
IF_END_105:
    ; VPy_LINE:165
    JSR DRAW_GAME_SCENE
    LEAS 2,S ; free locals
    RTS

HANDLE_TITLE_SCREEN: ; function
; --- function handle_title_screen ---
    LEAS -8,S ; allocate locals
    ; VPy_LINE:169
    LDD VAR_CURRENT_MUSIC
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_121
    LDD #0
    STD RESULT
    BRA CE_122
CT_121:
    LDD #1
    STD RESULT
CE_122:
    LDD RESULT
    LBEQ IF_NEXT_120
    ; VPy_LINE:170
; PLAY_MUSIC("pang_theme") - play music asset
    LDX #_PANG_THEME_MUSIC
    JSR PLAY_MUSIC_RUNTIME
    LDD #0
    STD RESULT
    ; VPy_LINE:171
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_MUSIC
    STU TMPPTR
    STX ,U
    LBRA IF_END_119
IF_NEXT_120:
IF_END_119:
    ; VPy_LINE:173
    JSR DRAW_TITLE_SCREEN
    ; VPy_LINE:176
; NATIVE_CALL: J1_BUTTON_1 at line 176
; J1_BUTTON_1() - Read Joystick 1 button 1 (BIOS)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$01
    BEQ .j1b1_not_pressed
    LDD #1
    BRA .j1b1_done
.j1b1_not_pressed:
    LDD #0
.j1b1_done:
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:177
; NATIVE_CALL: J1_BUTTON_2 at line 177
; J1_BUTTON_2() - Read Joystick 1 button 2 (BIOS)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$02
    BEQ .j1b2_not_pressed
    LDD #1
    BRA .j1b2_done
.j1b2_not_pressed:
    LDD #0
.j1b2_done:
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:178
; NATIVE_CALL: J1_BUTTON_3 at line 178
; J1_BUTTON_3() - Read Joystick 1 button 3 (BIOS)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$04
    BEQ .j1b3_not_pressed
    LDD #1
    BRA .j1b3_done
.j1b3_not_pressed:
    LDD #0
.j1b3_done:
    STD RESULT
    LDX RESULT
    STX 4 ,S
    ; VPy_LINE:179
; NATIVE_CALL: J1_BUTTON_4 at line 179
; J1_BUTTON_4() - Read Joystick 1 button 4 (BIOS)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$08
    BEQ .j1b4_not_pressed
    LDD #1
    BRA .j1b4_done
.j1b4_not_pressed:
    LDD #0
.j1b4_done:
    STD RESULT
    LDX RESULT
    STX 6 ,S
    ; VPy_LINE:181
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_131
    LDD #0
    STD RESULT
    BRA CE_132
CT_131:
    LDD #1
    STD RESULT
CE_132:
    LDD RESULT
    BNE OR_TRUE_129
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_133
    LDD #0
    STD RESULT
    BRA CE_134
CT_133:
    LDD #1
    STD RESULT
CE_134:
    LDD RESULT
    BNE OR_TRUE_129
    LDD #0
    STD RESULT
    BRA OR_END_130
OR_TRUE_129:
    LDD #1
    STD RESULT
OR_END_130:
    LDD RESULT
    BNE OR_TRUE_127
    LDD 4 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_135
    LDD #0
    STD RESULT
    BRA CE_136
CT_135:
    LDD #1
    STD RESULT
CE_136:
    LDD RESULT
    BNE OR_TRUE_127
    LDD #0
    STD RESULT
    BRA OR_END_128
OR_TRUE_127:
    LDD #1
    STD RESULT
OR_END_128:
    LDD RESULT
    BNE OR_TRUE_125
    LDD 6 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_137
    LDD #0
    STD RESULT
    BRA CE_138
CT_137:
    LDD #1
    STD RESULT
CE_138:
    LDD RESULT
    BNE OR_TRUE_125
    LDD #0
    STD RESULT
    BRA OR_END_126
OR_TRUE_125:
    LDD #1
    STD RESULT
OR_END_126:
    LDD RESULT
    LBEQ IF_NEXT_124
    ; VPy_LINE:182
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_SCREEN
    STU TMPPTR
    STX ,U
    ; VPy_LINE:183
    LDD #65535
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_MUSIC
    STU TMPPTR
    STX ,U
    ; VPy_LINE:184
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_DELAY
    STU TMPPTR
    STX ,U
    LBRA IF_END_123
IF_NEXT_124:
IF_END_123:
    LEAS 8,S ; free locals
    RTS

HANDLE_MAP_SCREEN: ; function
; --- function handle_map_screen ---
    LEAS -6,S ; allocate locals
    ; VPy_LINE:188
    LDD VAR_DELAY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_141
    LDD #0
    STD RESULT
    BRA CE_142
CT_141:
    LDD #1
    STD RESULT
CE_142:
    LDD RESULT
    LBEQ IF_NEXT_140
    ; VPy_LINE:189
; NATIVE_CALL: STOP_MUSIC at line 189
; STOP_MUSIC() - stop background music
    JSR STOP_MUSIC_RUNTIME
    LDD #0
    STD RESULT
    LBRA IF_END_139
IF_NEXT_140:
IF_END_139:
    ; VPy_LINE:191
    LDD VAR_CURRENT_MUSIC
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BNE CT_145
    LDD #0
    STD RESULT
    BRA CE_146
CT_145:
    LDD #1
    STD RESULT
CE_146:
    LDD RESULT
    BEQ AND_FALSE_147
    LDD VAR_DELAY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_149
    LDD #0
    STD RESULT
    BRA CE_150
CT_149:
    LDD #1
    STD RESULT
CE_150:
    LDD RESULT
    BEQ AND_FALSE_147
    LDD #1
    STD RESULT
    BRA AND_END_148
AND_FALSE_147:
    LDD #0
    STD RESULT
AND_END_148:
    LDD RESULT
    LBEQ IF_NEXT_144
    ; VPy_LINE:192
; PLAY_MUSIC("map_theme") - play music asset
    LDX #_MAP_THEME_MUSIC
    JSR PLAY_MUSIC_RUNTIME
    LDD #0
    STD RESULT
    ; VPy_LINE:193
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_MUSIC
    STU TMPPTR
    STX ,U
    LBRA IF_END_143
IF_NEXT_144:
IF_END_143:
    ; VPy_LINE:195
    LDD VAR_DELAY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_153
    LDD #0
    STD RESULT
    BRA CE_154
CT_153:
    LDD #1
    STD RESULT
CE_154:
    LDD RESULT
    LBEQ IF_NEXT_152
    ; VPy_LINE:196
    LDD VAR_DELAY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_DELAY
    STU TMPPTR
    STX ,U
    LBRA IF_END_151
IF_NEXT_152:
IF_END_151:
    ; VPy_LINE:199
; NATIVE_CALL: J1_X at line 199
; J1_X() - Read Joystick 1 X axis (Digital from RAM)
; Frontend writes unsigned 0-255 to $CF00 (128=center)
    LDB $CF00    ; Vec_Joy_1_X (0=left, 128=center, 255=right)
; Convert unsigned to digital: <108=left(-1), 108-148=center(0), >148=right(+1)
    CMPB #108    ; Check if < 108 (left)
    BLO J1X_LEFT ; Branch if lower (unsigned)
    CMPB #148    ; Check if > 148 (right)
    BHI J1X_RIGHT ; Branch if higher (unsigned)
    LDD #0       ; Center
    BRA J1X_END
J1X_LEFT:
    LDD #$FFFF   ; Left (-1)
    BRA J1X_END
J1X_RIGHT:
    LDD #1       ; Right (+1)
J1X_END:
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:200
; NATIVE_CALL: J1_Y at line 200
; J1_Y() - Read Joystick 1 Y axis (Digital from RAM)
; Frontend writes unsigned 0-255 to $CF01 (128=center)
    LDB $CF01    ; Vec_Joy_1_Y (0=down, 128=center, 255=up)
; Convert unsigned to digital: <108=down(-1), 108-148=center(0), >148=up(+1)
    CMPB #108    ; Check if < 108 (down)
    BLO J1Y_DOWN ; Branch if lower (unsigned)
    CMPB #148    ; Check if > 148 (up)
    BHI J1Y_UP   ; Branch if higher (unsigned)
    LDD #0       ; Center
    BRA J1Y_END
J1Y_DOWN:
    LDD #$FFFF   ; Down (-1)
    BRA J1Y_END
J1Y_UP:
    LDD #1       ; Up (+1)
J1Y_END:
    STD RESULT
    LDX RESULT
    STX 4 ,S
    ; VPy_LINE:203
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_157
    LDD #0
    STD RESULT
    BRA CE_158
CT_157:
    LDD #1
    STD RESULT
CE_158:
    LDD RESULT
    BEQ AND_FALSE_159
    LDD VAR_PREV_JOY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BNE CT_161
    LDD #0
    STD RESULT
    BRA CE_162
CT_161:
    LDD #1
    STD RESULT
CE_162:
    LDD RESULT
    BEQ AND_FALSE_159
    LDD #1
    STD RESULT
    BRA AND_END_160
AND_FALSE_159:
    LDD #0
    STD RESULT
AND_END_160:
    LDD RESULT
    LBEQ IF_NEXT_156
    ; VPy_LINE:204
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_MAX_LEVEL_UNLOCKED
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_165
    LDD #0
    STD RESULT
    BRA CE_166
CT_165:
    LDD #1
    STD RESULT
CE_166:
    LDD RESULT
    LBEQ IF_NEXT_164
    ; VPy_LINE:205
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    LBRA IF_END_163
IF_NEXT_164:
IF_END_163:
    LBRA IF_END_155
IF_NEXT_156:
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_168
    LDD #0
    STD RESULT
    BRA CE_169
CT_168:
    LDD #1
    STD RESULT
CE_169:
    LDD RESULT
    BEQ AND_FALSE_170
    LDD VAR_PREV_JOY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BNE CT_172
    LDD #0
    STD RESULT
    BRA CE_173
CT_172:
    LDD #1
    STD RESULT
CE_173:
    LDD RESULT
    BEQ AND_FALSE_170
    LDD #1
    STD RESULT
    BRA AND_END_171
AND_FALSE_170:
    LDD #0
    STD RESULT
AND_END_171:
    LDD RESULT
    LBEQ IF_NEXT_167
    ; VPy_LINE:207
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_176
    LDD #0
    STD RESULT
    BRA CE_177
CT_176:
    LDD #1
    STD RESULT
CE_177:
    LDD RESULT
    LBEQ IF_NEXT_175
    ; VPy_LINE:208
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    LBRA IF_END_174
IF_NEXT_175:
IF_END_174:
    LBRA IF_END_155
IF_NEXT_167:
    LDD 4 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_179
    LDD #0
    STD RESULT
    BRA CE_180
CT_179:
    LDD #1
    STD RESULT
CE_180:
    LDD RESULT
    BEQ AND_FALSE_181
    LDD VAR_PREV_JOY_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BNE CT_183
    LDD #0
    STD RESULT
    BRA CE_184
CT_183:
    LDD #1
    STD RESULT
CE_184:
    LDD RESULT
    BEQ AND_FALSE_181
    LDD #1
    STD RESULT
    BRA AND_END_182
AND_FALSE_181:
    LDD #0
    STD RESULT
AND_END_182:
    LDD RESULT
    LBEQ IF_NEXT_178
    ; VPy_LINE:210
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_MAX_LEVEL_UNLOCKED
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_187
    LDD #0
    STD RESULT
    BRA CE_188
CT_187:
    LDD #1
    STD RESULT
CE_188:
    LDD RESULT
    LBEQ IF_NEXT_186
    ; VPy_LINE:211
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    LBRA IF_END_185
IF_NEXT_186:
IF_END_185:
    LBRA IF_END_155
IF_NEXT_178:
    LDD 4 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_189
    LDD #0
    STD RESULT
    BRA CE_190
CT_189:
    LDD #1
    STD RESULT
CE_190:
    LDD RESULT
    BEQ AND_FALSE_191
    LDD VAR_PREV_JOY_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65535
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BNE CT_193
    LDD #0
    STD RESULT
    BRA CE_194
CT_193:
    LDD #1
    STD RESULT
CE_194:
    LDD RESULT
    BEQ AND_FALSE_191
    LDD #1
    STD RESULT
    BRA AND_END_192
AND_FALSE_191:
    LDD #0
    STD RESULT
AND_END_192:
    LDD RESULT
    LBEQ IF_END_155
    ; VPy_LINE:213
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_197
    LDD #0
    STD RESULT
    BRA CE_198
CT_197:
    LDD #1
    STD RESULT
CE_198:
    LDD RESULT
    LBEQ IF_NEXT_196
    ; VPy_LINE:214
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    LBRA IF_END_195
IF_NEXT_196:
IF_END_195:
    LBRA IF_END_155
IF_END_155:
    ; VPy_LINE:216
    LDD 2 ,S
    STD RESULT
    LDX RESULT
    LDU #VAR_PREV_JOY_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:217
    LDD 4 ,S
    STD RESULT
    LDX RESULT
    LDU #VAR_PREV_JOY_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:220
; NATIVE_CALL: J1_BUTTON_1 at line 220
; J1_BUTTON_1() - Read Joystick 1 button 1 (BIOS)
    JSR $F1BA    ; Read_Btns
    LDA $C80F    ; Vec_Btn_State
    ANDA #$01
    BEQ .j1b1_not_pressed
    LDD #1
    BRA .j1b1_done
.j1b1_not_pressed:
    LDD #0
.j1b1_done:
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:221
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_201
    LDD #0
    STD RESULT
    BRA CE_202
CT_201:
    LDD #1
    STD RESULT
CE_202:
    LDD RESULT
    BEQ AND_FALSE_203
    LDD VAR_PREV_BTN_1
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_205
    LDD #0
    STD RESULT
    BRA CE_206
CT_205:
    LDD #1
    STD RESULT
CE_206:
    LDD RESULT
    BEQ AND_FALSE_203
    LDD #1
    STD RESULT
    BRA AND_END_204
AND_FALSE_203:
    LDD #0
    STD RESULT
AND_END_204:
    LDD RESULT
    LBEQ IF_NEXT_200
    ; VPy_LINE:223
    LDD #2
    STD RESULT
    LDX RESULT
    LDU #VAR_SCREEN
    STU TMPPTR
    STX ,U
    ; VPy_LINE:224
    LDD #180
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTDOWN_TIMER
    STU TMPPTR
    STX ,U
    ; VPy_LINE:225
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTDOWN_ACTIVE
    STU TMPPTR
    STX ,U
    ; VPy_LINE:226
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:227
    LDD #65535
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_MUSIC
    STU TMPPTR
    STX ,U
    LBRA IF_END_199
IF_NEXT_200:
IF_END_199:
    ; VPy_LINE:229
    LDD 0 ,S
    STD RESULT
    LDX RESULT
    LDU #VAR_PREV_BTN_1
    STU TMPPTR
    STX ,U
    ; VPy_LINE:231
    JSR DRAW_MAP_SCREEN
    LEAS 6,S ; free locals
    RTS

HANDLE_GAME_SCREEN: ; function
; --- function handle_game_screen ---
    ; VPy_LINE:235
    LDD VAR_COUNTDOWN_ACTIVE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_209
    LDD #0
    STD RESULT
    BRA CE_210
CT_209:
    LDD #1
    STD RESULT
CE_210:
    LDD RESULT
    LBEQ IF_NEXT_208
    ; VPy_LINE:236
    JSR DRAW_COUNTDOWN_SCREEN
    ; VPy_LINE:237
    LDD VAR_COUNTDOWN_TIMER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTDOWN_TIMER
    STU TMPPTR
    STX ,U
    ; VPy_LINE:239
    LDD VAR_COUNTDOWN_TIMER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLE CT_213
    LDD #0
    STD RESULT
    BRA CE_214
CT_213:
    LDD #1
    STD RESULT
CE_214:
    LDD RESULT
    LBEQ IF_NEXT_212
    ; VPy_LINE:240
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTDOWN_ACTIVE
    STU TMPPTR
    STX ,U
    ; VPy_LINE:242
; PLAY_MUSIC("map_theme") - play music asset
    LDX #_MAP_THEME_MUSIC
    JSR PLAY_MUSIC_RUNTIME
    LDD #0
    STD RESULT
    ; VPy_LINE:243
    LDD #2
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_MUSIC
    STU TMPPTR
    STX ,U
    LBRA IF_END_211
IF_NEXT_212:
IF_END_211:
    ; VPy_LINE:245
    RTS
    LBRA IF_END_207
IF_NEXT_208:
IF_END_207:
    RTS

DRAW_COUNTDOWN_SCREEN: ; function
; --- function draw_countdown_screen ---
    LEAS -2,S ; allocate locals
    ; VPy_LINE:251
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 251
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:254
; PRINT_TEXT(x, y, text, height, width) - custom size
    LDD #65532
    STD RESULT
    LDA RESULT+1  ; Height (negative value)
    STA $C82A     ; Vec_Text_Height
    LDD #32
    STD RESULT
    LDA RESULT+1  ; Width (positive value)
    STA $C82B     ; Vec_Text_Width
    LDD #65409
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    ; ===== Const array indexing: location_names =====
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDX #CONST_ARRAY_2
    LDD TMPPTR
    LEAX D,X
    ; String array - load pointer from table
    LDD ,X
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 254
    JSR VECTREX_PRINT_TEXT
    LDA #$F8      ; Default height (-8 in two's complement)
    STA $C82A     ; Restore Vec_Text_Height
    LDA #72       ; Default width (72)
    STA $C82B     ; Restore Vec_Text_Width
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:257
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 257
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:258
; PRINT_TEXT(x, y, text, height, width) - custom size
    LDD #65528
    STD RESULT
    LDA RESULT+1  ; Height (negative value)
    STA $C82A     ; Vec_Text_Height
    LDD #72
    STD RESULT
    LDA RESULT+1  ; Width (positive value)
    STA $C82B     ; Vec_Text_Width
    LDD #65486
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_10
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 258
    JSR VECTREX_PRINT_TEXT
    LDA #$F8      ; Default height (-8 in two's complement)
    STA $C82A     ; Restore Vec_Text_Height
    LDA #72       ; Default width (72)
    STA $C82B     ; Restore Vec_Text_Width
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:261
    LDD VAR_COUNTDOWN_TIMER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #60
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:262
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_217
    LDD #0
    STD RESULT
    BRA CE_218
CT_217:
    LDD #1
    STD RESULT
CE_218:
    LDD RESULT
    LBEQ IF_NEXT_216
    ; VPy_LINE:263
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #65526
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_2
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 263
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_215
IF_NEXT_216:
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_220
    LDD #0
    STD RESULT
    BRA CE_221
CT_220:
    LDD #1
    STD RESULT
CE_221:
    LDD RESULT
    LBEQ IF_NEXT_219
    ; VPy_LINE:265
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #65526
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_1
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 265
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_215
IF_NEXT_219:
    ; VPy_LINE:267
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #65526
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_0
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 267
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
IF_END_215:
    LEAS 2,S ; free locals
    RTS

DRAW_GAME_SCENE: ; function
; --- function draw_game_scene ---
    ; VPy_LINE:271
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 271
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:272
    LDA #$D0
    TFR A,DP
    LDA #$50
    JSR Intensity_a
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$F0
    JSR Draw_Line_d
    LDA #$C8
    TFR A,DP
    LDD #0
    STD RESULT
    ; VPy_LINE:275
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 275
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:276
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65436
    STD VAR_ARG1
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #65456
    STD VAR_ARG3
    LDD #127
    STD VAR_ARG4
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; VPy_LINE:281
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 281
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:282
; PRINT_TEXT(x, y, text, height, width) - custom size
    LDD #65530
    STD RESULT
    LDA RESULT+1  ; Height (negative value)
    STA $C82A     ; Vec_Text_Height
    LDD #48
    STD RESULT
    LDA RESULT+1  ; Width (positive value)
    STA $C82B     ; Vec_Text_Width
    LDD #65409
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #120
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_12
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 282
    JSR VECTREX_PRINT_TEXT
    LDA #$F8      ; Default height (-8 in two's complement)
    STA $C82A     ; Restore Vec_Text_Height
    LDA #72       ; Default width (72)
    STA $C82B     ; Restore Vec_Text_Width
    CLRA
    CLRB
    STD RESULT
    RTS

DRAW_MAP_SCREEN: ; function
; --- function draw_map_screen ---
    LEAS -4,S ; allocate locals
    ; VPy_LINE:287
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 287
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:288
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #65409
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65436
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_22
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 288
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:289
; PRINT_TEXT(x, y, text, height, width) - custom size
    LDD #65530
    STD RESULT
    LDA RESULT+1  ; Height (negative value)
    STA $C82A     ; Vec_Text_Height
    LDD #60
    STD RESULT
    LDA RESULT+1  ; Width (positive value)
    STA $C82B     ; Vec_Text_Width
    LDD #65409
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65416
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    ; ===== Const array indexing: location_names =====
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDX #CONST_ARRAY_2
    LDD TMPPTR
    LEAX D,X
    ; String array - load pointer from table
    LDD ,X
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 289
    JSR VECTREX_PRINT_TEXT
    LDA #$F8      ; Default height (-8 in two's complement)
    STA $C82A     ; Restore Vec_Text_Height
    LDA #72       ; Default width (72)
    STA $C82B     ; Restore Vec_Text_Width
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:291
; DRAW_VECTOR_EX("map", x, y, mirror) - 15 path(s), width=242, center_x=-6
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #20
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDD #0
    STD RESULT
    LDB RESULT+1  ; Mirror mode (0=normal, 1=X, 2=Y, 3=both)
    ; Decode mirror mode into separate flags:
    CLR MIRROR_X  ; Clear X flag
    CLR MIRROR_Y  ; Clear Y flag
    CMPB #1       ; Check if X-mirror (mode 1)
    BNE DSVEX_CHK_Y_222
    LDA #1
    STA MIRROR_X
DSVEX_CHK_Y_222:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE DSVEX_CHK_XY_223
    LDA #1
    STA MIRROR_Y
DSVEX_CHK_XY_223:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE DSVEX_CALL_224
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
DSVEX_CALL_224:
    ; Set intensity override for drawing
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Intensity (0-127)
    STA DRAW_VEC_INTENSITY  ; Store intensity override (function will use this)
    LDX #_MAP_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_MAP_PATH1  ; Path 1
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_MAP_PATH2  ; Path 2
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_MAP_PATH3  ; Path 3
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_MAP_PATH4  ; Path 4
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_MAP_PATH5  ; Path 5
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_MAP_PATH6  ; Path 6
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_MAP_PATH7  ; Path 7
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_MAP_PATH8  ; Path 8
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_MAP_PATH9  ; Path 9
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_MAP_PATH10  ; Path 10
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_MAP_PATH11  ; Path 11
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_MAP_PATH12  ; Path 12
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_MAP_PATH13  ; Path 13
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_MAP_PATH14  ; Path 14
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    CLR DRAW_VEC_INTENSITY  ; Clear intensity override for next draw
    LDD #0
    STD RESULT
    ; VPy_LINE:294
    LDD VAR_LOCATION_GLOW_DIRECTION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_227
    LDD #0
    STD RESULT
    BRA CE_228
CT_227:
    LDD #1
    STD RESULT
CE_228:
    LDD RESULT
    LBEQ IF_NEXT_226
    ; VPy_LINE:295
    LDD VAR_LOCATION_GLOW_INTENSITY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_LOCATION_GLOW_INTENSITY
    STU TMPPTR
    STX ,U
    ; VPy_LINE:296
    LDD VAR_LOCATION_GLOW_INTENSITY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #127
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_231
    LDD #0
    STD RESULT
    BRA CE_232
CT_231:
    LDD #1
    STD RESULT
CE_232:
    LDD RESULT
    LBEQ IF_NEXT_230
    ; VPy_LINE:297
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_LOCATION_GLOW_DIRECTION
    STU TMPPTR
    STX ,U
    LBRA IF_END_229
IF_NEXT_230:
IF_END_229:
    LBRA IF_END_225
IF_NEXT_226:
    ; VPy_LINE:299
    LDD VAR_LOCATION_GLOW_INTENSITY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_LOCATION_GLOW_INTENSITY
    STU TMPPTR
    STX ,U
    ; VPy_LINE:300
    LDD VAR_LOCATION_GLOW_INTENSITY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #30
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLE CT_235
    LDD #0
    STD RESULT
    BRA CE_236
CT_235:
    LDD #1
    STD RESULT
CE_236:
    LDD RESULT
    LBEQ IF_NEXT_234
    ; VPy_LINE:301
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_LOCATION_GLOW_DIRECTION
    STU TMPPTR
    STX ,U
    LBRA IF_END_233
IF_NEXT_234:
IF_END_233:
IF_END_225:
    ; VPy_LINE:304
    ; ===== Const array indexing: location_x_coords =====
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDX #CONST_ARRAY_0
    LDD TMPPTR
    LEAX D,X
    LDD ,X
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:305
    ; ===== Const array indexing: location_y_coords =====
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDX #CONST_ARRAY_1
    LDD TMPPTR
    LEAX D,X
    LDD ,X
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:307
; DRAW_VECTOR_EX("location_marker", x, y, mirror) - 1 path(s), width=22, center_x=0
    LDD 2 ,S
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD 0 ,S
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDD #0
    STD RESULT
    LDB RESULT+1  ; Mirror mode (0=normal, 1=X, 2=Y, 3=both)
    ; Decode mirror mode into separate flags:
    CLR MIRROR_X  ; Clear X flag
    CLR MIRROR_Y  ; Clear Y flag
    CMPB #1       ; Check if X-mirror (mode 1)
    BNE DSVEX_CHK_Y_237
    LDA #1
    STA MIRROR_X
DSVEX_CHK_Y_237:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE DSVEX_CHK_XY_238
    LDA #1
    STA MIRROR_Y
DSVEX_CHK_XY_238:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE DSVEX_CALL_239
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
DSVEX_CALL_239:
    ; Set intensity override for drawing
    LDD VAR_LOCATION_GLOW_INTENSITY
    STD RESULT
    LDA RESULT+1  ; Intensity (0-127)
    STA DRAW_VEC_INTENSITY  ; Store intensity override (function will use this)
    LDX #_LOCATION_MARKER_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    CLR DRAW_VEC_INTENSITY  ; Clear intensity override for next draw
    LDD #0
    STD RESULT
    LEAS 4,S ; free locals
    RTS

DRAW_TITLE_SCREEN: ; function
; --- function draw_title_screen ---
    ; VPy_LINE:312
; DRAW_VECTOR("logo", x, y) - 16 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #20
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
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
    LDX #_LOGO_PATH7  ; Path 7
    JSR Draw_Sync_List_At
    LDX #_LOGO_PATH8  ; Path 8
    JSR Draw_Sync_List_At
    LDX #_LOGO_PATH9  ; Path 9
    JSR Draw_Sync_List_At
    LDX #_LOGO_PATH10  ; Path 10
    JSR Draw_Sync_List_At
    LDX #_LOGO_PATH11  ; Path 11
    JSR Draw_Sync_List_At
    LDX #_LOGO_PATH12  ; Path 12
    JSR Draw_Sync_List_At
    LDX #_LOGO_PATH13  ; Path 13
    JSR Draw_Sync_List_At
    LDX #_LOGO_PATH14  ; Path 14
    JSR Draw_Sync_List_At
    LDX #_LOGO_PATH15  ; Path 15
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    ; VPy_LINE:314
    LDD VAR_TITLE_INTENSITY
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 314
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:315
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #65446
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_20
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 315
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:316
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #65486
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65476
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_24
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 316
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:318
    LDD VAR_TITLE_STATE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_242
    LDD #0
    STD RESULT
    BRA CE_243
CT_242:
    LDD #1
    STD RESULT
CE_243:
    LDD RESULT
    LBEQ IF_NEXT_241
    ; VPy_LINE:319
    LDD VAR_TITLE_INTENSITY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_TITLE_INTENSITY
    STU TMPPTR
    STX ,U
    LBRA IF_END_240
IF_NEXT_241:
IF_END_240:
    ; VPy_LINE:321
    LDD VAR_TITLE_STATE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_246
    LDD #0
    STD RESULT
    BRA CE_247
CT_246:
    LDD #1
    STD RESULT
CE_247:
    LDD RESULT
    LBEQ IF_NEXT_245
    ; VPy_LINE:322
    LDD VAR_TITLE_INTENSITY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_TITLE_INTENSITY
    STU TMPPTR
    STX ,U
    LBRA IF_END_244
IF_NEXT_245:
IF_END_244:
    ; VPy_LINE:324
    LDD VAR_TITLE_INTENSITY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #127
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_250
    LDD #0
    STD RESULT
    BRA CE_251
CT_250:
    LDD #1
    STD RESULT
CE_251:
    LDD RESULT
    LBEQ IF_NEXT_249
    ; VPy_LINE:325
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_TITLE_STATE
    STU TMPPTR
    STX ,U
    LBRA IF_END_248
IF_NEXT_249:
IF_END_248:
    ; VPy_LINE:327
    LDD VAR_TITLE_INTENSITY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #30
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_254
    LDD #0
    STD RESULT
    BRA CE_255
CT_254:
    LDD #1
    STD RESULT
CE_255:
    LDD RESULT
    LBEQ IF_NEXT_253
    ; VPy_LINE:328
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_TITLE_STATE
    STU TMPPTR
    STX ,U
    LBRA IF_END_252
IF_NEXT_253:
IF_END_252:
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
VAR_SCREEN EQU $CF10+0
VAR_TITLE_INTENSITY EQU $CF10+2
VAR_TITLE_STATE EQU $CF10+4
VAR_CURRENT_MUSIC EQU $CF10+6
VAR_DELAY EQU $CF10+8
VAR_STAGE EQU $CF10+10
VAR_COUNTDOWN_TIMER EQU $CF10+12
VAR_COUNTDOWN_ACTIVE EQU $CF10+14
VAR_NUM_LOCATIONS EQU $CF10+16
VAR_CURRENT_LOCATION EQU $CF10+18
VAR_MAX_LEVEL_UNLOCKED EQU $CF10+20
VAR_LOCATION_GLOW_INTENSITY EQU $CF10+22
VAR_LOCATION_GLOW_DIRECTION EQU $CF10+24
VAR_PREV_JOY_X EQU $CF10+26
VAR_PREV_JOY_Y EQU $CF10+28
VAR_PREV_BTN_1 EQU $CF10+30
VAR_INTENSITYVAL EQU $CF10+32
VAR_PLAYER_X EQU $CF10+34
VAR_PLAYER_Y EQU $CF10+36
VAR_PLAYER_SPEED EQU $CF10+38
; Call argument scratch space
VAR_ARG0 EQU $C8B2
VAR_ARG1 EQU $C8B4
VAR_ARG2 EQU $C8B6
VAR_ARG3 EQU $C8B8
VAR_ARG4 EQU $C8BA
VAR_ARG5 EQU $C8BC

; ========================================
; ASSET DATA SECTION
; Embedded 5 of 9 assets (unused assets excluded)
; ========================================

; Vector asset: logo
; Generated from logo.vec (Malban Draw_Sync_List format)
; Total paths: 16, points: 77
; X bounds: min=-82, max=104, width=186
; Center: (11, -3)

_LOGO_WIDTH EQU 186
_LOGO_CENTER_X EQU 11
_LOGO_CENTER_Y EQU -3

_LOGO_VECTORS:  ; Main entry
_LOGO_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $16,$A3,0,0        ; path0: header (y=22, x=-93, relative to center)
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

_LOGO_PATH1:
    FCB 127              ; path1: intensity
    FCB $FE,$D8,0,0        ; path1: header (y=-2, x=-40, relative to center)
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

_LOGO_PATH2:
    FCB 127              ; path2: intensity
    FCB $0A,$C3,0,0        ; path2: header (y=10, x=-61, relative to center)
    FCB $FF,$F8,$02          ; line 0: flag=-1, dy=-8, dx=2
    FCB $FF,$07,$08          ; line 1: flag=-1, dy=7, dx=8
    FCB $FF,$01,$F6          ; line 2: flag=-1, dy=1, dx=-10
    FCB $FF,$00,$00          ; line 3: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_LOGO_PATH3:
    FCB 127              ; path3: intensity
    FCB $09,$E9,0,0        ; path3: header (y=9, x=-23, relative to center)
    FCB $FF,$F6,$FD          ; line 0: flag=-1, dy=-10, dx=-3
    FCB $FF,$02,$07          ; line 1: flag=-1, dy=2, dx=7
    FCB $FF,$08,$FC          ; line 2: flag=-1, dy=8, dx=-4
    FCB $FF,$FE,$01          ; line 3: flag=-1, dy=-2, dx=1
    FCB 2                ; End marker (path complete)

_LOGO_PATH4:
    FCB 127              ; path4: intensity
    FCB $F6,$FF,0,0        ; path4: header (y=-10, x=-1, relative to center)
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

_LOGO_PATH5:
    FCB 127              ; path5: intensity
    FCB $09,$3A,0,0        ; path5: header (y=9, x=58, relative to center)
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

_LOGO_PATH6:
    FCB 127              ; path6: intensity
    FCB $09,$3A,0,0        ; path6: header (y=9, x=58, relative to center)
    FCB $FF,$00,$0C          ; line 0: flag=-1, dy=0, dx=12
    FCB $FF,$0C,$F8          ; line 1: flag=-1, dy=12, dx=-8
    FCB $FF,$03,$F0          ; line 2: flag=-1, dy=3, dx=-16
    FCB $FF,$FB,$FC          ; line 3: flag=-1, dy=-5, dx=-4
    FCB 2                ; End marker (path complete)

_LOGO_PATH7:
_LOGO_PATH8:
_LOGO_PATH9:
_LOGO_PATH10:
_LOGO_PATH11:
_LOGO_PATH12:
    FCB 127              ; path12: intensity
    FCB $D8,$5D,0,0        ; path12: header (y=-40, x=93, relative to center)
    FCB $FF,$00,$00          ; line 0: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_LOGO_PATH13:
_LOGO_PATH14:
    FCB 127              ; path14: intensity
    FCB $D8,$BF,0,0        ; path14: header (y=-40, x=-65, relative to center)
    FCB $FF,$00,$00          ; line 0: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_LOGO_PATH15:
    FCB 127              ; path15: intensity
    FCB $DB,$BF,0,0        ; path15: header (y=-37, x=-65, relative to center)
    FCB $FF,$00,$00          ; line 0: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

; Vector asset: map
; Generated from map.vec (Malban Draw_Sync_List format)
; Total paths: 15, points: 165
; X bounds: min=-127, max=115, width=242
; Center: (-6, -3)

_MAP_WIDTH EQU 242
_MAP_CENTER_X EQU -6
_MAP_CENTER_Y EQU -3

_MAP_VECTORS:  ; Main entry
_MAP_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $22,$D7,0,0        ; path0: header (y=34, x=-41, relative to center)
    FCB $FF,$0E,$1A          ; line 0: flag=-1, dy=14, dx=26
    FCB $FF,$07,$0C          ; line 1: flag=-1, dy=7, dx=12
    FCB $FF,$06,$00          ; line 2: flag=-1, dy=6, dx=0
    FCB $FF,$09,$0C          ; line 3: flag=-1, dy=9, dx=12
    FCB $FF,$00,$0E          ; line 4: flag=-1, dy=0, dx=14
    FCB $FF,$08,$0A          ; line 5: flag=-1, dy=8, dx=10
    FCB $FF,$00,$21          ; line 6: flag=-1, dy=0, dx=33
    FCB $FF,$FC,$03          ; line 7: flag=-1, dy=-4, dx=3
    FCB $FF,$FF,$14          ; line 8: flag=-1, dy=-1, dx=20
    FCB $FF,$EE,$20          ; line 9: flag=-1, dy=-18, dx=32
    FCB $FF,$FB,$FC          ; line 10: flag=-1, dy=-5, dx=-4
    FCB $FF,$F9,$FE          ; line 11: flag=-1, dy=-7, dx=-2
    FCB $FF,$06,$FA          ; line 12: flag=-1, dy=6, dx=-6
    FCB $FF,$02,$F0          ; line 13: flag=-1, dy=2, dx=-16
    FCB $FF,$F4,$06          ; line 14: flag=-1, dy=-12, dx=6
    FCB $FF,$E2,$FE          ; line 15: flag=-1, dy=-30, dx=-2
    FCB $FF,$FB,$FB          ; line 16: flag=-1, dy=-5, dx=-5
    FCB $FF,$F8,$FE          ; line 17: flag=-1, dy=-8, dx=-2
    FCB $FF,$FF,$F6          ; line 18: flag=-1, dy=-1, dx=-10
    FCB $FF,$F7,$05          ; line 19: flag=-1, dy=-9, dx=5
    FCB $FF,$FC,$FD          ; line 20: flag=-1, dy=-4, dx=-3
    FCB $FF,$0E,$F6          ; line 21: flag=-1, dy=14, dx=-10
    FCB $FF,$05,$01          ; line 22: flag=-1, dy=5, dx=1
    FCB $FF,$06,$FD          ; line 23: flag=-1, dy=6, dx=-3
    FCB $FF,$EA,$F7          ; line 24: flag=-1, dy=-22, dx=-9
    FCB $FF,$20,$F0          ; line 25: flag=-1, dy=32, dx=-16
    FCB $FF,$05,$F9          ; line 26: flag=-1, dy=5, dx=-7
    FCB $FF,$F9,$03          ; line 27: flag=-1, dy=-7, dx=3
    FCB $FF,$F5,$F9          ; line 28: flag=-1, dy=-11, dx=-7
    FCB $FF,$0E,$F3          ; line 29: flag=-1, dy=14, dx=-13
    FCB $FF,$FD,$FD          ; line 30: flag=-1, dy=-3, dx=-3
    FCB $FF,$F2,$0C          ; line 31: flag=-1, dy=-14, dx=12
    FCB $FF,$00,$03          ; line 32: flag=-1, dy=0, dx=3
    FCB $FF,$F2,$F7          ; line 33: flag=-1, dy=-14, dx=-9
    FCB $FF,$F3,$FE          ; line 34: flag=-1, dy=-13, dx=-2
    FCB $FF,$EC,$ED          ; line 35: flag=-1, dy=-20, dx=-19
    FCB $FF,$0D,$F3          ; line 36: flag=-1, dy=13, dx=-13
    FCB $FF,$0E,$00          ; line 37: flag=-1, dy=14, dx=0
    FCB $FF,$09,$F8          ; line 38: flag=-1, dy=9, dx=-8
    FCB $FF,$00,$F0          ; line 39: flag=-1, dy=0, dx=-16
    FCB $FF,$08,$F8          ; line 40: flag=-1, dy=8, dx=-8
    FCB $FF,$0B,$00          ; line 41: flag=-1, dy=11, dx=0
    FCB $FF,$0B,$0A          ; line 42: flag=-1, dy=11, dx=10
    FCB $FF,$01,$22          ; line 43: flag=-1, dy=1, dx=34
    FCB $FF,$09,$F4          ; line 44: flag=-1, dy=9, dx=-12
    FCB $FF,$FA,$EE          ; line 45: flag=-1, dy=-6, dx=-18
    FCB $FF,$FF,$F3          ; line 46: flag=-1, dy=-1, dx=-13
    FCB $FF,$0A,$00          ; line 47: flag=-1, dy=10, dx=0
    FCB $FF,$00,$00          ; line 48: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_MAP_PATH1:
    FCB 127              ; path1: intensity
    FCB $38,$DE,0,0        ; path1: header (y=56, x=-34, relative to center)
    FCB $FF,$04,$06          ; line 0: flag=-1, dy=4, dx=6
    FCB $FF,$FC,$01          ; line 1: flag=-1, dy=-4, dx=1
    FCB $FF,$FD,$FC          ; line 2: flag=-1, dy=-3, dx=-4
    FCB $FF,$00,$FD          ; line 3: flag=-1, dy=0, dx=-3
    FCB $FF,$03,$00          ; line 4: flag=-1, dy=3, dx=0
    FCB $FF,$00,$00          ; line 5: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_MAP_PATH2:
    FCB 127              ; path2: intensity
    FCB $34,$E5,0,0        ; path2: header (y=52, x=-27, relative to center)
    FCB $FF,$06,$0A          ; line 0: flag=-1, dy=6, dx=10
    FCB $FF,$06,$FE          ; line 1: flag=-1, dy=6, dx=-2
    FCB $FF,$02,$05          ; line 2: flag=-1, dy=2, dx=5
    FCB $FF,$FB,$FE          ; line 3: flag=-1, dy=-5, dx=-2
    FCB $FF,$F6,$02          ; line 4: flag=-1, dy=-10, dx=2
    FCB $FF,$FF,$F4          ; line 5: flag=-1, dy=-1, dx=-12
    FCB $FF,$02,$FF          ; line 6: flag=-1, dy=2, dx=-1
    FCB $FF,$00,$00          ; line 7: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_MAP_PATH3:
    FCB 127              ; path3: intensity
    FCB $BD,$70,0,0        ; path3: header (y=-67, x=112, relative to center)
    FCB $FF,$08,$05          ; line 0: flag=-1, dy=8, dx=5
    FCB $FF,$14,$00          ; line 1: flag=-1, dy=20, dx=0
    FCB $FF,$06,$FB          ; line 2: flag=-1, dy=6, dx=-5
    FCB $FF,$F8,$FE          ; line 3: flag=-1, dy=-8, dx=-2
    FCB $FF,$06,$EE          ; line 4: flag=-1, dy=6, dx=-18
    FCB $FF,$F3,$F1          ; line 5: flag=-1, dy=-13, dx=-15
    FCB $FF,$F5,$07          ; line 6: flag=-1, dy=-11, dx=7
    FCB $FF,$03,$0C          ; line 7: flag=-1, dy=3, dx=12
    FCB $FF,$F4,$10          ; line 8: flag=-1, dy=-12, dx=16
    FCB $FF,$00,$00          ; line 9: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_MAP_PATH4:
    FCB 127              ; path4: intensity
    FCB $ED,$66,0,0        ; path4: header (y=-19, x=102, relative to center)
    FCB $FF,$F1,$00          ; line 0: flag=-1, dy=-15, dx=0
    FCB $FF,$04,$F8          ; line 1: flag=-1, dy=4, dx=-8
    FCB $FF,$05,$00          ; line 2: flag=-1, dy=5, dx=0
    FCB $FF,$06,$09          ; line 3: flag=-1, dy=6, dx=9
    FCB $FF,$00,$00          ; line 4: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_MAP_PATH5:
    FCB 127              ; path5: intensity
    FCB $EE,$57,0,0        ; path5: header (y=-18, x=87, relative to center)
    FCB $FF,$F8,$05          ; line 0: flag=-1, dy=-8, dx=5
    FCB $FF,$F9,$FF          ; line 1: flag=-1, dy=-7, dx=-1
    FCB $FF,$05,$FA          ; line 2: flag=-1, dy=5, dx=-6
    FCB $FF,$0A,$02          ; line 3: flag=-1, dy=10, dx=2
    FCB $FF,$00,$00          ; line 4: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_MAP_PATH6:
    FCB 127              ; path6: intensity
    FCB $E6,$72,0,0        ; path6: header (y=-26, x=114, relative to center)
    FCB $FF,$FD,$FB          ; line 0: flag=-1, dy=-3, dx=-5
    FCB $FF,$FB,$08          ; line 1: flag=-1, dy=-5, dx=8
    FCB $FF,$04,$00          ; line 2: flag=-1, dy=4, dx=0
    FCB $FF,$04,$FD          ; line 3: flag=-1, dy=4, dx=-3
    FCB $FF,$00,$00          ; line 4: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_MAP_PATH7:
    FCB 127              ; path7: intensity
    FCB $DD,$1A,0,0        ; path7: header (y=-35, x=26, relative to center)
    FCB $FF,$09,$08          ; line 0: flag=-1, dy=9, dx=8
    FCB $FF,$01,$FA          ; line 1: flag=-1, dy=1, dx=-6
    FCB $FF,$F7,$FA          ; line 2: flag=-1, dy=-9, dx=-6
    FCB $FF,$FE,$05          ; line 3: flag=-1, dy=-2, dx=5
    FCB $FF,$00,$00          ; line 4: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_MAP_PATH8:
    FCB 127              ; path8: intensity
    FCB $4C,$B0,0,0        ; path8: header (y=76, x=-80, relative to center)
    FCB $FF,$FC,$0D          ; line 0: flag=-1, dy=-4, dx=13
    FCB $FF,$FD,$00          ; line 1: flag=-1, dy=-3, dx=0
    FCB $FF,$FA,$08          ; line 2: flag=-1, dy=-6, dx=8
    FCB $FF,$09,$06          ; line 3: flag=-1, dy=9, dx=6
    FCB $FF,$09,$F2          ; line 4: flag=-1, dy=9, dx=-14
    FCB $FF,$FF,$F6          ; line 5: flag=-1, dy=-1, dx=-10
    FCB $FF,$FC,$FD          ; line 6: flag=-1, dy=-4, dx=-3
    FCB $FF,$00,$00          ; line 7: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_MAP_PATH9:
    FCB 127              ; path9: intensity
    FCB $2D,$87,0,0        ; path9: header (y=45, x=-121, relative to center)
    FCB $FF,$F7,$08          ; line 0: flag=-1, dy=-9, dx=8
    FCB $FF,$F7,$F9          ; line 1: flag=-1, dy=-9, dx=-7
    FCB $FF,$E4,$17          ; line 2: flag=-1, dy=-28, dx=23
    FCB $FF,$FE,$16          ; line 3: flag=-1, dy=-2, dx=22
    FCB $FF,$09,$F6          ; line 4: flag=-1, dy=9, dx=-10
    FCB $FF,$00,$FA          ; line 5: flag=-1, dy=0, dx=-6
    FCB $FF,$0D,$FE          ; line 6: flag=-1, dy=13, dx=-2
    FCB $FF,$09,$0E          ; line 7: flag=-1, dy=9, dx=14
    FCB $FF,$F9,$06          ; line 8: flag=-1, dy=-7, dx=6
    FCB $FF,$18,$13          ; line 9: flag=-1, dy=24, dx=19
    FCB $FF,$10,$F5          ; line 10: flag=-1, dy=16, dx=-11
    FCB $FF,$F4,$FD          ; line 11: flag=-1, dy=-12, dx=-3
    FCB $FF,$04,$F5          ; line 12: flag=-1, dy=4, dx=-11
    FCB $FF,$08,$01          ; line 13: flag=-1, dy=8, dx=1
    FCB $FF,$0A,$EE          ; line 14: flag=-1, dy=10, dx=-18
    FCB $FF,$06,$E7          ; line 15: flag=-1, dy=6, dx=-25
    FCB $FF,$DF,$01          ; line 16: flag=-1, dy=-33, dx=1
    FCB $FF,$00,$00          ; line 17: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_MAP_PATH10:
    FCB 127              ; path10: intensity
    FCB $04,$BE,0,0        ; path10: header (y=4, x=-66, relative to center)
    FCB $FF,$ED,$F8          ; line 0: flag=-1, dy=-19, dx=-8
    FCB $FF,$F9,$06          ; line 1: flag=-1, dy=-7, dx=6
    FCB $FF,$E0,$05          ; line 2: flag=-1, dy=-32, dx=5
    FCB $FF,$19,$14          ; line 3: flag=-1, dy=25, dx=20
    FCB $FF,$FF,$08          ; line 4: flag=-1, dy=-1, dx=8
    FCB $FF,$10,$00          ; line 5: flag=-1, dy=16, dx=0
    FCB $FF,$03,$F7          ; line 6: flag=-1, dy=3, dx=-9
    FCB $FF,$09,$F8          ; line 7: flag=-1, dy=9, dx=-8
    FCB $FF,$06,$F3          ; line 8: flag=-1, dy=6, dx=-13
    FCB $FF,$01,$00          ; line 9: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

_MAP_PATH11:
    FCB 127              ; path11: intensity
    FCB $B0,$AE,0,0        ; path11: header (y=-80, x=-82, relative to center)
    FCB $FF,$0D,$0C          ; line 0: flag=-1, dy=13, dx=12
    FCB $FF,$FB,$0D          ; line 1: flag=-1, dy=-5, dx=13
    FCB $FF,$F9,$08          ; line 2: flag=-1, dy=-7, dx=8
    FCB $FF,$FE,$DF          ; line 3: flag=-1, dy=-2, dx=-33
    FCB $FF,$00,$00          ; line 4: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_MAP_PATH12:
    FCB 127              ; path12: intensity
    FCB $0E,$69,0,0        ; path12: header (y=14, x=105, relative to center)
    FCB $FF,$08,$FC          ; line 0: flag=-1, dy=8, dx=-4
    FCB $FF,$01,$01          ; line 1: flag=-1, dy=1, dx=1
    FCB $FF,$02,$03          ; line 2: flag=-1, dy=2, dx=3
    FCB $FF,$F5,$00          ; line 3: flag=-1, dy=-11, dx=0
    FCB $FF,$00,$00          ; line 4: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_MAP_PATH13:
    FCB 127              ; path13: intensity
    FCB $24,$69,0,0        ; path13: header (y=36, x=105, relative to center)
    FCB $FF,$04,$07          ; line 0: flag=-1, dy=4, dx=7
    FCB $FF,$04,$F9          ; line 1: flag=-1, dy=4, dx=-7
    FCB $FF,$F8,$00          ; line 2: flag=-1, dy=-8, dx=0
    FCB $FF,$00,$00          ; line 3: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_MAP_PATH14:
    FCB 127              ; path14: intensity
    FCB $21,$6D,0,0        ; path14: header (y=33, x=109, relative to center)
    FCB $FF,$F9,$FD          ; line 0: flag=-1, dy=-7, dx=-3
    FCB $FF,$FB,$02          ; line 1: flag=-1, dy=-5, dx=2
    FCB $FF,$FF,$03          ; line 2: flag=-1, dy=-1, dx=3
    FCB $FF,$05,$04          ; line 3: flag=-1, dy=5, dx=4
    FCB $FF,$08,$FC          ; line 4: flag=-1, dy=8, dx=-4
    FCB $FF,$00,$FE          ; line 5: flag=-1, dy=0, dx=-2
    FCB $FF,$00,$00          ; line 6: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

; Vector asset: location_marker
; Generated from location_marker.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 10
; X bounds: min=-11, max=11, width=22
; Center: (0, 1)

_LOCATION_MARKER_WIDTH EQU 22
_LOCATION_MARKER_CENTER_X EQU 0
_LOCATION_MARKER_CENTER_Y EQU 1

_LOCATION_MARKER_VECTORS:  ; Main entry
_LOCATION_MARKER_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0B,$00,0,0        ; path0: header (y=11, x=0, relative to center)
    FCB $FF,$F8,$04          ; line 0: flag=-1, dy=-8, dx=4
    FCB $FF,$00,$07          ; line 1: flag=-1, dy=0, dx=7
    FCB $FF,$F9,$FC          ; line 2: flag=-1, dy=-7, dx=-4
    FCB $FF,$F9,$00          ; line 3: flag=-1, dy=-7, dx=0
    FCB $FF,$05,$F9          ; line 4: flag=-1, dy=5, dx=-7
    FCB $FF,$FB,$F9          ; line 5: flag=-1, dy=-5, dx=-7
    FCB $FF,$07,$00          ; line 6: flag=-1, dy=7, dx=0
    FCB $FF,$07,$FC          ; line 7: flag=-1, dy=7, dx=-4
    FCB $FF,$00,$07          ; line 8: flag=-1, dy=0, dx=7
    FCB $FF,$08,$04          ; closing line: flag=-1, dy=8, dx=4
    FCB 2                ; End marker (path complete)

; Generated from pang_theme.vmus (internal name: pang_theme)
; Tempo: 120 BPM, Total events: 34 (PSG Direct format)
; Format: FCB count, FCB reg, val, ... (per frame), FCB 0 (end)

_PANG_THEME_MUSIC:
    ; Frame-based PSG register writes
    FCB     0              ; Delay 0 frames (maintain previous state)
    FCB     11              ; Frame 0 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $1C             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $99             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $05             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $0F             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     12              ; Delay 12 frames (maintain previous state)
    FCB     10              ; Frame 12 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $1C             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $99             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $05             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     13              ; Delay 13 frames (maintain previous state)
    FCB     10              ; Frame 25 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $8E             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $1C             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $99             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $05             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     11              ; Frame 50 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $77             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $EF             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $99             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $05             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $0F             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     12              ; Delay 12 frames (maintain previous state)
    FCB     10              ; Frame 62 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $77             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $EF             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $99             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $05             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     13              ; Delay 13 frames (maintain previous state)
    FCB     10              ; Frame 75 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $59             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $EF             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $99             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $05             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     11              ; Frame 100 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $77             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $B3             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $99             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $05             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $0F             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     12              ; Delay 12 frames (maintain previous state)
    FCB     10              ; Frame 112 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $77             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $B3             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $99             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $05             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     12              ; Delay 12 frames (maintain previous state)
    FCB     10              ; Frame 124 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $8E             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $B3             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $99             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $05             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     26              ; Delay 26 frames (maintain previous state)
    FCB     11              ; Frame 150 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $1C             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $99             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $05             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $0F             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     12              ; Delay 12 frames (maintain previous state)
    FCB     10              ; Frame 162 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $1C             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $99             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $05             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     38              ; Delay 38 frames (maintain previous state)
    FCB     11              ; Frame 200 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $9F             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $0C             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $FC             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $04             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $0F             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     12              ; Delay 12 frames (maintain previous state)
    FCB     10              ; Frame 212 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $9F             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $0C             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $FC             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $04             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     12              ; Delay 12 frames (maintain previous state)
    FCB     10              ; Frame 224 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $86             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $0C             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $FC             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $04             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     11              ; Frame 249 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $6A             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $D5             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $FC             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $04             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $0F             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     13              ; Delay 13 frames (maintain previous state)
    FCB     10              ; Frame 262 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $6A             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $D5             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $FC             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $04             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     13              ; Delay 13 frames (maintain previous state)
    FCB     10              ; Frame 275 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $4F             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $D5             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $FC             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $04             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     11              ; Frame 300 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $6A             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $9F             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $FC             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $04             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $0F             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     12              ; Delay 12 frames (maintain previous state)
    FCB     10              ; Frame 312 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $6A             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $9F             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $FC             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $04             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     13              ; Delay 13 frames (maintain previous state)
    FCB     10              ; Frame 325 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $86             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $9F             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $FC             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $04             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     11              ; Frame 350 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $9F             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $0C             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $FC             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $04             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $0F             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     12              ; Delay 12 frames (maintain previous state)
    FCB     10              ; Frame 362 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $9F             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $0C             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $FC             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $04             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $08             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     38              ; Delay 38 frames before loop
    FCB     $FF             ; Loop command ($FF never valid as count)
    FDB     _PANG_THEME_MUSIC       ; Jump to start (absolute address)


; Generated from map_theme.vmus (internal name: Space Groove)
; Tempo: 140 BPM, Total events: 36 (PSG Direct format)
; Format: FCB count, FCB reg, val, ... (per frame), FCB 0 (end)

_MAP_THEME_MUSIC:
    ; Frame-based PSG register writes
    FCB     0              ; Delay 0 frames (maintain previous state)
    FCB     11              ; Frame 0 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0D             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $CC             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $02             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $66             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $14             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     5              ; Delay 5 frames (maintain previous state)
    FCB     10              ; Frame 5 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0D             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $CC             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $02             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $66             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     5              ; Delay 5 frames (maintain previous state)
    FCB     11              ; Frame 10 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $9F             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0D             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $CC             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $02             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $66             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $03             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     3              ; Delay 3 frames (maintain previous state)
    FCB     10              ; Frame 13 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $9F             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0D             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $CC             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $02             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $66             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     8              ; Delay 8 frames (maintain previous state)
    FCB     9              ; Frame 21 - 9 register writes
    FCB     0               ; Reg 0 number
    FCB     $8E             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0E             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $66             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $03             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F2             ; Reg 7 value
    FCB     3              ; Delay 3 frames (maintain previous state)
    FCB     8              ; Frame 24 - 8 register writes
    FCB     0               ; Reg 0 number
    FCB     $8E             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0E             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $66             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FA             ; Reg 7 value
    FCB     8              ; Delay 8 frames (maintain previous state)
    FCB     9              ; Frame 32 - 9 register writes
    FCB     0               ; Reg 0 number
    FCB     $9F             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $66             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $03             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F2             ; Reg 7 value
    FCB     2              ; Delay 2 frames (maintain previous state)
    FCB     8              ; Frame 34 - 8 register writes
    FCB     0               ; Reg 0 number
    FCB     $9F             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $66             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FA             ; Reg 7 value
    FCB     8              ; Delay 8 frames (maintain previous state)
    FCB     11              ; Frame 42 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0D             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $CC             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $02             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $14             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     6              ; Delay 6 frames (maintain previous state)
    FCB     10              ; Frame 48 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0D             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $CC             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $02             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     5              ; Delay 5 frames (maintain previous state)
    FCB     11              ; Frame 53 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0D             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $CC             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $02             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $03             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     3              ; Delay 3 frames (maintain previous state)
    FCB     10              ; Frame 56 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0D             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $CC             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $02             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     8              ; Delay 8 frames (maintain previous state)
    FCB     9              ; Frame 64 - 9 register writes
    FCB     0               ; Reg 0 number
    FCB     $D5             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $03             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F2             ; Reg 7 value
    FCB     2              ; Delay 2 frames (maintain previous state)
    FCB     8              ; Frame 66 - 8 register writes
    FCB     0               ; Reg 0 number
    FCB     $D5             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FA             ; Reg 7 value
    FCB     9              ; Delay 9 frames (maintain previous state)
    FCB     9              ; Frame 75 - 9 register writes
    FCB     0               ; Reg 0 number
    FCB     $EF             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0B             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $03             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F2             ; Reg 7 value
    FCB     2              ; Delay 2 frames (maintain previous state)
    FCB     8              ; Frame 77 - 8 register writes
    FCB     0               ; Reg 0 number
    FCB     $EF             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0B             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FA             ; Reg 7 value
    FCB     8              ; Delay 8 frames (maintain previous state)
    FCB     11              ; Frame 85 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0D             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $DE             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $EF             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $00             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $14             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     6              ; Delay 6 frames (maintain previous state)
    FCB     10              ; Frame 91 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0D             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $DE             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $EF             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $00             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     5              ; Delay 5 frames (maintain previous state)
    FCB     11              ; Frame 96 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $8E             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0E             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $DE             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $EF             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $00             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $03             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     3              ; Delay 3 frames (maintain previous state)
    FCB     10              ; Frame 99 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $8E             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0E             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $DE             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $EF             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $00             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     8              ; Delay 8 frames (maintain previous state)
    FCB     9              ; Frame 107 - 9 register writes
    FCB     0               ; Reg 0 number
    FCB     $77             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0F             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $EF             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $00             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $03             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F2             ; Reg 7 value
    FCB     2              ; Delay 2 frames (maintain previous state)
    FCB     8              ; Frame 109 - 8 register writes
    FCB     0               ; Reg 0 number
    FCB     $77             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0F             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $EF             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $00             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FA             ; Reg 7 value
    FCB     8              ; Delay 8 frames (maintain previous state)
    FCB     9              ; Frame 117 - 9 register writes
    FCB     0               ; Reg 0 number
    FCB     $77             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0F             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $EF             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $00             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $03             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F2             ; Reg 7 value
    FCB     3              ; Delay 3 frames (maintain previous state)
    FCB     8              ; Frame 120 - 8 register writes
    FCB     0               ; Reg 0 number
    FCB     $77             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0F             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $EF             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $00             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FA             ; Reg 7 value
    FCB     8              ; Delay 8 frames (maintain previous state)
    FCB     11              ; Frame 128 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $8E             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0D             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $DE             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $14             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     5              ; Delay 5 frames (maintain previous state)
    FCB     10              ; Frame 133 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $8E             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0D             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $DE             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     6              ; Delay 6 frames (maintain previous state)
    FCB     11              ; Frame 139 - 11 register writes
    FCB     0               ; Reg 0 number
    FCB     $8E             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0D             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $DE             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $03             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F0             ; Reg 7 value
    FCB     2              ; Delay 2 frames (maintain previous state)
    FCB     10              ; Frame 141 - 10 register writes
    FCB     0               ; Reg 0 number
    FCB     $8E             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0D             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $DE             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0B             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $F8             ; Reg 7 value
    FCB     9              ; Delay 9 frames (maintain previous state)
    FCB     9              ; Frame 150 - 9 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $03             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F2             ; Reg 7 value
    FCB     2              ; Delay 2 frames (maintain previous state)
    FCB     8              ; Frame 152 - 8 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FA             ; Reg 7 value
    FCB     8              ; Delay 8 frames (maintain previous state)
    FCB     9              ; Frame 160 - 9 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     6               ; Reg 6 number
    FCB     $03             ; Reg 6 value
    FCB     7               ; Reg 7 number
    FCB     $F2             ; Reg 7 value
    FCB     3              ; Delay 3 frames (maintain previous state)
    FCB     8              ; Frame 163 - 8 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0C             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     4               ; Reg 4 number
    FCB     $1C             ; Reg 4 value
    FCB     5               ; Reg 5 number
    FCB     $01             ; Reg 5 value
    FCB     10               ; Reg 10 number
    FCB     $09             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FA             ; Reg 7 value
    FCB     8              ; Delay 8 frames before loop
    FCB     $FF             ; Loop command ($FF never valid as count)
    FDB     _MAP_THEME_MUSIC       ; Jump to start (absolute address)


; Const array literal for 'location_x_coords' (17 elements)
CONST_ARRAY_0:
    FDB 40   ; Element 0
    FDB 40   ; Element 1
    FDB 65496   ; Element 2
    FDB 65526   ; Element 3
    FDB 20   ; Element 4
    FDB 50   ; Element 5
    FDB 80   ; Element 6
    FDB 65451   ; Element 7
    FDB 65486   ; Element 8
    FDB 65521   ; Element 9
    FDB 15   ; Element 10
    FDB 50   ; Element 11
    FDB 85   ; Element 12
    FDB 65446   ; Element 13
    FDB 65491   ; Element 14
    FDB 0   ; Element 15
    FDB 45   ; Element 16

; Const array literal for 'location_y_coords' (17 elements)
CONST_ARRAY_1:
    FDB 110   ; Element 0
    FDB 79   ; Element 1
    FDB 65516   ; Element 2
    FDB 10   ; Element 3
    FDB 40   ; Element 4
    FDB 70   ; Element 5
    FDB 100   ; Element 6
    FDB 65496   ; Element 7
    FDB 65526   ; Element 8
    FDB 30   ; Element 9
    FDB 60   ; Element 10
    FDB 90   ; Element 11
    FDB 20   ; Element 12
    FDB 50   ; Element 13
    FDB 0   ; Element 14
    FDB 65476   ; Element 15
    FDB 65506   ; Element 16

; Const string array for 'location_names' (17 strings)
CONST_ARRAY_2_STR_0:
    FCC "MOUNT FUJI (JP)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_1:
    FCC "MOUNT KEIRIN (CN)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_2:
    FCC "EMERALD BUDDHA TEMPLE (TH)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_3:
    FCC "ANGKOR WAT (KH)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_4:
    FCC "AYERS ROCK (AU)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_5:
    FCC "TAJ MAHAL (IN)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_6:
    FCC "LENINGRAD (RU)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_7:
    FCC "PARIS (FR)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_8:
    FCC "LONDON (UK)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_9:
    FCC "BARCELONA (ES)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_10:
    FCC "ATHENS (GR)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_11:
    FCC "PYRAMIDS (EG)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_12:
    FCC "MOUNT KILIMANJARO (TZ)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_13:
    FCC "NEW YORK (US)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_14:
    FCC "MAYAN RUINS (MX)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_15:
    FCC "ANTARCTICA (AQ)"
    FCB $80   ; String terminator
CONST_ARRAY_2_STR_16:
    FCC "EASTER ISLAND (CL)"
    FCB $80   ; String terminator
CONST_ARRAY_2:  ; Pointer table for location_names
    FDB CONST_ARRAY_2_STR_0  ; Pointer to string
    FDB CONST_ARRAY_2_STR_1  ; Pointer to string
    FDB CONST_ARRAY_2_STR_2  ; Pointer to string
    FDB CONST_ARRAY_2_STR_3  ; Pointer to string
    FDB CONST_ARRAY_2_STR_4  ; Pointer to string
    FDB CONST_ARRAY_2_STR_5  ; Pointer to string
    FDB CONST_ARRAY_2_STR_6  ; Pointer to string
    FDB CONST_ARRAY_2_STR_7  ; Pointer to string
    FDB CONST_ARRAY_2_STR_8  ; Pointer to string
    FDB CONST_ARRAY_2_STR_9  ; Pointer to string
    FDB CONST_ARRAY_2_STR_10  ; Pointer to string
    FDB CONST_ARRAY_2_STR_11  ; Pointer to string
    FDB CONST_ARRAY_2_STR_12  ; Pointer to string
    FDB CONST_ARRAY_2_STR_13  ; Pointer to string
    FDB CONST_ARRAY_2_STR_14  ; Pointer to string
    FDB CONST_ARRAY_2_STR_15  ; Pointer to string
    FDB CONST_ARRAY_2_STR_16  ; Pointer to string

; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "1"
    FCB $80
STR_1:
    FCC "2"
    FCB $80
STR_2:
    FCC "3"
    FCB $80
STR_3:
    FCC "ANGKOR WAT (KH)"
    FCB $80
STR_4:
    FCC "ANTARCTICA (AQ)"
    FCB $80
STR_5:
    FCC "ATHENS (GR)"
    FCB $80
STR_6:
    FCC "AYERS ROCK (AU)"
    FCB $80
STR_7:
    FCC "BARCELONA (ES)"
    FCB $80
STR_8:
    FCC "EASTER ISLAND (CL)"
    FCB $80
STR_9:
    FCC "EMERALD BUDDHA TEMPLE (TH)"
    FCB $80
STR_10:
    FCC "GET READY"
    FCB $80
STR_11:
    FCC "LENINGRAD (RU)"
    FCB $80
STR_12:
    FCC "LEVEL:"
    FCB $80
STR_13:
    FCC "LONDON (UK)"
    FCB $80
STR_14:
    FCC "MAYAN RUINS (MX)"
    FCB $80
STR_15:
    FCC "MOUNT FUJI (JP)"
    FCB $80
STR_16:
    FCC "MOUNT KEIRIN (CN)"
    FCB $80
STR_17:
    FCC "MOUNT KILIMANJARO (TZ)"
    FCB $80
STR_18:
    FCC "NEW YORK (US)"
    FCB $80
STR_19:
    FCC "PARIS (FR)"
    FCB $80
STR_20:
    FCC "PRESS A BUTTON"
    FCB $80
STR_21:
    FCC "PYRAMIDS (EG)"
    FCB $80
STR_22:
    FCC "SELECT A STAGE"
    FCB $80
STR_23:
    FCC "TAJ MAHAL (IN)"
    FCB $80
STR_24:
    FCC "TO START"
    FCB $80
VLINE_DX EQU RESULT+40
VLINE_DY EQU RESULT+41
VLINE_STEPS EQU RESULT+42
VLINE_LIST EQU RESULT+43
DRAW_VEC_X EQU RESULT+45
DRAW_VEC_Y EQU RESULT+46
MIRROR_X EQU RESULT+47
MIRROR_Y EQU RESULT+48
DRAW_VEC_INTENSITY EQU RESULT+49
