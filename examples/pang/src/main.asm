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
; Total RAM used: 53 bytes
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$02   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$04   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$06   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$08   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0A   ; Pointer temp (2 bytes)
TMPPTR2              EQU $C880+$0C   ; Pointer temp 2 (for nested array operations) (2 bytes)
MUL_A                EQU $C880+$0E   ; Multiplicand A (2 bytes)
MUL_B                EQU $C880+$10   ; Multiplicand B (2 bytes)
MUL_RES              EQU $C880+$12   ; Multiply result (2 bytes)
MUL_TMP              EQU $C880+$14   ; Multiply temporary (2 bytes)
MUL_CNT              EQU $C880+$16   ; Multiply counter (2 bytes)
DIV_A                EQU $C880+$18   ; Dividend (2 bytes)
DIV_B                EQU $C880+$1A   ; Divisor (2 bytes)
DIV_Q                EQU $C880+$1C   ; Quotient (2 bytes)
DIV_R                EQU $C880+$1E   ; Remainder (2 bytes)
TEMP_YX              EQU $C880+$20   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$22   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$23   ; Temporary y storage (1 bytes)
PSG_MUSIC_PTR        EQU $C880+$24   ; Current music position pointer (2 bytes)
PSG_MUSIC_START      EQU $C880+$26   ; Music start pointer (for loops) (2 bytes)
PSG_IS_PLAYING       EQU $C880+$28   ; Playing flag ($00=stopped, $01=playing) (1 bytes)
PSG_MUSIC_ACTIVE     EQU $C880+$29   ; Set during UPDATE_MUSIC_PSG (1 bytes)
PSG_FRAME_COUNT      EQU $C880+$2A   ; Frame register write count (1 bytes)
PSG_DELAY_FRAMES     EQU $C880+$2B   ; Frames to wait before next read (1 bytes)
SFX_PTR              EQU $C880+$2C   ; Current SFX data pointer (2 bytes)
SFX_TICK             EQU $C880+$2E   ; Current frame counter (2 bytes)
SFX_ACTIVE           EQU $C880+$30   ; Playback state ($00=stopped, $01=playing) (1 bytes)
SFX_PHASE            EQU $C880+$31   ; Envelope phase (0=A,1=D,2=S,3=R) (1 bytes)
SFX_VOL              EQU $C880+$32   ; Current volume level (0-15) (1 bytes)
NUM_STR              EQU $C880+$33   ; String buffer for PRINT_NUMBER (2 bytes)
PSG_MUSIC_PTR_DP   EQU $24  ; DP-relative
PSG_MUSIC_START_DP EQU $26  ; DP-relative
PSG_IS_PLAYING_DP  EQU $28  ; DP-relative
PSG_MUSIC_ACTIVE_DP EQU $29  ; DP-relative
PSG_FRAME_COUNT_DP EQU $2A  ; DP-relative
PSG_DELAY_FRAMES_DP EQU $2B  ; DP-relative

    JMP START

;**** CONST DECLARATIONS (NUMBER-ONLY) ****
; VPy_LINE:7
; _CONST_DECL_0:  ; const STATE_TITLE
; VPy_LINE:8
; _CONST_DECL_1:  ; const STATE_MAP
; VPy_LINE:9
; _CONST_DECL_2:  ; const STATE_GAME
; VPy_LINE:68
; _CONST_DECL_3:  ; const MAX_ENEMIES
; VPy_LINE:77
; _CONST_DECL_4:  ; const GRAVITY
; VPy_LINE:78
; _CONST_DECL_5:  ; const BOUNCE_DAMPING
; VPy_LINE:79
; _CONST_DECL_6:  ; const GROUND_Y

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
    LDA RESULT+8+1  ; intensity (RESULT+8, byte access)
    JSR Intensity_a
    ; Move to start ONCE (y in A, x in B) - use signed byte values
    LDA RESULT+2+1  ; Y start (RESULT+2, byte access)
    LDB RESULT+0+1  ; X start (RESULT+0, byte access)
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
    BGE DLW_SEG1_DY_READY
    LDA #$80        ; dy < -128: use -128
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
    BGE DLW_SEG1_DX_READY
    LDB #$80        ; dx < -128: use -128
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
    ; SEGMENT 2: Draw remaining dy
    ; Check if dy was > 127 or < -128
    LDD VLINE_DY_16 ; Load original full dy
    CMPD #127
    BGT DLW_SEG2_POSITIVE_OVER  ; dy > 127
    ; dy < -128, so we drew -128 in segment 1
    ; remaining = dy - (-128) = dy + 128
    ADDD #128       ; Add back the -128 we already drew
    BRA DLW_SEG2_STORE
DLW_SEG2_POSITIVE_OVER:
    ; dy > 127, so we drew 127 in segment 1
    ; remaining = dy - 127
    SUBD #127       ; Subtract 127 we already drew
DLW_SEG2_STORE:
    ; D now contains remaining dy to draw (16-bit signed)
    ; Need to extract the 8-bit low byte for Draw_Line_d
    ; Note: after ADDD/SUBD, D is already correctly positioned
    ; We just need to get the low byte into B
    EXCH            ; Swap A and B: now B has the low byte (dy)
    CLRA            ; dx = 0 in A
    EXCH            ; Swap back: A has 0, B has dy
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
    ; Initialize SFX variables to prevent random noise on startup
    CLR sfx_status         ; Mark SFX as inactive (0=off)
    LDD #$0000
    STD sfx_pointer        ; Clear SFX pointer

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:11
    LDD #0
    STD RESULT
    STD VAR_SCREEN
    ; VPy_LINE:12
    LDD #30
    STD VAR_TITLE_INTENSITY
    ; VPy_LINE:13
    LDD #0
    STD VAR_TITLE_STATE
    ; VPy_LINE:14
    LDD #-1
    STD VAR_CURRENT_MUSIC
    ; VPy_LINE:15
    LDD #0
    STD VAR_DELAY
    ; VPy_LINE:16
    LDD #0
    STD VAR_STAGE
    ; VPy_LINE:29
    ; Copy array 'joystick1_state' from ROM to RAM (6 elements)
    LDX #ARRAY_0       ; Source: ROM array data
    LDU #VAR_JOYSTICK1_STATE_DATA ; Dest: RAM array space
    LDD #6        ; Number of elements
COPY_LOOP_0:
    LDY ,X++        ; Load word from ROM, increment source
    STY ,U++        ; Store word to RAM, increment dest
    SUBD #1         ; Decrement counter
    BNE COPY_LOOP_0 ; Loop until done
    ; VPy_LINE:31
    LDD #17
    STD VAR_NUM_LOCATIONS
    ; VPy_LINE:32
    LDD #0
    STD VAR_CURRENT_LOCATION
    ; VPy_LINE:33
    LDD #60
    STD VAR_LOCATION_GLOW_INTENSITY
    ; VPy_LINE:34
    LDD #0
    STD VAR_LOCATION_GLOW_DIRECTION
    ; VPy_LINE:35
    LDD #0
    STD VAR_JOY_X
    ; VPy_LINE:36
    LDD #0
    STD VAR_JOY_Y
    ; VPy_LINE:37
    LDD #0
    STD VAR_PREV_JOY_X
    ; VPy_LINE:38
    LDD #0
    STD VAR_PREV_JOY_Y
    ; VPy_LINE:39
    LDD #0
    STD VAR_INTENSITYVAL
    ; VPy_LINE:42
    LDD #0
    STD VAR_COUNTDOWN_TIMER
    ; VPy_LINE:43
    LDD #0
    STD VAR_COUNTDOWN_ACTIVE
    ; VPy_LINE:44
    LDD #0
    STD VAR_MAX_LEVEL_UNLOCKED
    ; VPy_LINE:45
    LDD #0
    STD VAR_JOYSTICK_POLL_COUNTER
    ; VPy_LINE:48
    LDD #0
    STD VAR_HOOK_ACTIVE
    ; VPy_LINE:49
    LDD #0
    STD VAR_HOOK_X
    ; VPy_LINE:50
    LDD #-100
    STD VAR_HOOK_Y
    ; VPy_LINE:51
    LDD #72
    STD VAR_HOOK_MAX_Y
    ; VPy_LINE:52
    LDD #0
    STD VAR_HOOK_GUN_X
    ; VPy_LINE:53
    LDD #0
    STD VAR_HOOK_GUN_Y
    ; VPy_LINE:54
    LDD #0
    STD VAR_HOOK_INIT_Y
    ; VPy_LINE:56
    LDD #0
    STD VAR_PLAYER_X
    ; VPy_LINE:57
    LDD #-100
    STD VAR_PLAYER_Y
    ; VPy_LINE:58
    LDD #2
    STD VAR_PLAYER_SPEED
    ; VPy_LINE:59
    LDD #0
    STD VAR_MOVE_SPEED
    ; VPy_LINE:60
    LDD #0
    STD VAR_ABS_JOY
    ; VPy_LINE:61
    LDD #0
    STD VAR_SPEED_MULTIPLIER
    ; VPy_LINE:62
    LDD #1
    STD VAR_PLAYER_ANIM_FRAME
    ; VPy_LINE:63
    LDD #0
    STD VAR_PLAYER_ANIM_COUNTER
    ; VPy_LINE:64
    LDD #5
    STD VAR_PLAYER_ANIM_SPEED
    ; VPy_LINE:65
    LDD #1
    STD VAR_PLAYER_FACING
    ; VPy_LINE:69
    ; Copy array 'enemy_active' from ROM to RAM (8 elements)
    LDX #ARRAY_1       ; Source: ROM array data
    LDU #VAR_ENEMY_ACTIVE_DATA ; Dest: RAM array space
    LDD #8        ; Number of elements
COPY_LOOP_1:
    LDY ,X++        ; Load word from ROM, increment source
    STY ,U++        ; Store word to RAM, increment dest
    SUBD #1         ; Decrement counter
    BNE COPY_LOOP_1 ; Loop until done
    ; VPy_LINE:70
    ; Copy array 'enemy_x' from ROM to RAM (8 elements)
    LDX #ARRAY_2       ; Source: ROM array data
    LDU #VAR_ENEMY_X_DATA ; Dest: RAM array space
    LDD #8        ; Number of elements
COPY_LOOP_2:
    LDY ,X++        ; Load word from ROM, increment source
    STY ,U++        ; Store word to RAM, increment dest
    SUBD #1         ; Decrement counter
    BNE COPY_LOOP_2 ; Loop until done
    ; VPy_LINE:71
    ; Copy array 'enemy_y' from ROM to RAM (8 elements)
    LDX #ARRAY_3       ; Source: ROM array data
    LDU #VAR_ENEMY_Y_DATA ; Dest: RAM array space
    LDD #8        ; Number of elements
COPY_LOOP_3:
    LDY ,X++        ; Load word from ROM, increment source
    STY ,U++        ; Store word to RAM, increment dest
    SUBD #1         ; Decrement counter
    BNE COPY_LOOP_3 ; Loop until done
    ; VPy_LINE:72
    ; Copy array 'enemy_vx' from ROM to RAM (8 elements)
    LDX #ARRAY_4       ; Source: ROM array data
    LDU #VAR_ENEMY_VX_DATA ; Dest: RAM array space
    LDD #8        ; Number of elements
COPY_LOOP_4:
    LDY ,X++        ; Load word from ROM, increment source
    STY ,U++        ; Store word to RAM, increment dest
    SUBD #1         ; Decrement counter
    BNE COPY_LOOP_4 ; Loop until done
    ; VPy_LINE:73
    ; Copy array 'enemy_vy' from ROM to RAM (8 elements)
    LDX #ARRAY_5       ; Source: ROM array data
    LDU #VAR_ENEMY_VY_DATA ; Dest: RAM array space
    LDD #8        ; Number of elements
COPY_LOOP_5:
    LDY ,X++        ; Load word from ROM, increment source
    STY ,U++        ; Store word to RAM, increment dest
    SUBD #1         ; Decrement counter
    BNE COPY_LOOP_5 ; Loop until done
    ; VPy_LINE:74
    ; Copy array 'enemy_size' from ROM to RAM (8 elements)
    LDX #ARRAY_6       ; Source: ROM array data
    LDU #VAR_ENEMY_SIZE_DATA ; Dest: RAM array space
    LDD #8        ; Number of elements
COPY_LOOP_6:
    LDY ,X++        ; Load word from ROM, increment source
    STY ,U++        ; Store word to RAM, increment dest
    SUBD #1         ; Decrement counter
    BNE COPY_LOOP_6 ; Loop until done
    ; VPy_LINE:86
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    ; VPy_LINE:87
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_PREV_JOY_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:88
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_PREV_JOY_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:89
    LDD #60
    STD RESULT
    LDX RESULT
    LDU #VAR_LOCATION_GLOW_INTENSITY
    STU TMPPTR
    STX ,U
    ; VPy_LINE:90
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_LOCATION_GLOW_DIRECTION
    STU TMPPTR
    STX ,U
    ; VPy_LINE:91
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_SCREEN
    STU TMPPTR
    STX ,U
    ; VPy_LINE:94
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTDOWN_TIMER
    STU TMPPTR
    STX ,U
    ; VPy_LINE:95
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTDOWN_ACTIVE
    STU TMPPTR
    STX ,U
    ; VPy_LINE:96
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_MAX_LEVEL_UNLOCKED
    STU TMPPTR
    STX ,U
    ; VPy_LINE:99
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_HOOK_ACTIVE
    STU TMPPTR
    STX ,U
    ; VPy_LINE:100
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_HOOK_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:101
    LDD #-100
    STD RESULT
    LDX RESULT
    LDU #VAR_HOOK_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:104
    JSR INIT_ENEMIES
; VPy_LINE:82

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

STATE_TITLE EQU 0
STATE_MAP EQU 1
STATE_GAME EQU 2
MAX_ENEMIES EQU 8
GRAVITY EQU 2
BOUNCE_DAMPING EQU 9
GROUND_Y EQU 65456
    ; VPy_LINE:106
LOOP_BODY:
    LEAS -2,S ; allocate locals
    JSR $F1AF    ; DP_to_C8 (ensure DP for variable access)
    JSR Wait_Recal ; Auto-injected: sync with vector beam
    JSR AUDIO_UPDATE  ; Auto-injected: update music + SFX (consistent timing)
    ; DEBUG: Processing 4 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(8)
    ; VPy_LINE:108
    JSR READ_JOYSTICK1_STATE
    ; DEBUG: Statement 1 - Discriminant(9)
    ; VPy_LINE:110
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
    ; VPy_LINE:111
    LDD VAR_CURRENT_MUSIC
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-1
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
    ; VPy_LINE:112
; PLAY_MUSIC("pang_theme") - play music asset
    LDX #_PANG_THEME_MUSIC
    JSR PLAY_MUSIC_RUNTIME
    LDD #0
    STD RESULT
    ; VPy_LINE:113
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_MUSIC
    STU TMPPTR
    STX ,U
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    ; VPy_LINE:115
    JSR DRAW_TITLE_SCREEN
    ; VPy_LINE:117
    LDD #VAR_JOYSTICK1_STATE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
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
    LDD #VAR_JOYSTICK1_STATE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #3
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
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
    LDD #VAR_JOYSTICK1_STATE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #4
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
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
    LDD #VAR_JOYSTICK1_STATE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #5
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
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
    ; VPy_LINE:118
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_SCREEN
    STU TMPPTR
    STX ,U
    ; VPy_LINE:119
    LDD #-1
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_MUSIC
    STU TMPPTR
    STX ,U
    LBRA IF_END_8
IF_NEXT_9:
IF_END_8:
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    ; DEBUG: Statement 2 - Discriminant(9)
    ; VPy_LINE:121
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
    BEQ CT_26
    LDD #0
    STD RESULT
    BRA CE_27
CT_26:
    LDD #1
    STD RESULT
CE_27:
    LDD RESULT
    LBEQ IF_NEXT_25
    ; VPy_LINE:122
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
    BNE CT_30
    LDD #0
    STD RESULT
    BRA CE_31
CT_30:
    LDD #1
    STD RESULT
CE_31:
    LDD RESULT
    LBEQ IF_NEXT_29
    ; VPy_LINE:123
; PLAY_MUSIC("map_theme") - play music asset
    LDX #_MAP_THEME_MUSIC
    JSR PLAY_MUSIC_RUNTIME
    LDD #0
    STD RESULT
    ; VPy_LINE:124
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_MUSIC
    STU TMPPTR
    STX ,U
    LBRA IF_END_28
IF_NEXT_29:
IF_END_28:
    ; VPy_LINE:127
    LDD VAR_JOYSTICK_POLL_COUNTER
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
    LDU #VAR_JOYSTICK_POLL_COUNTER
    STU TMPPTR
    STX ,U
    ; VPy_LINE:128
    LDD VAR_JOYSTICK_POLL_COUNTER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_34
    LDD #0
    STD RESULT
    BRA CE_35
CT_34:
    LDD #1
    STD RESULT
CE_35:
    LDD RESULT
    LBEQ IF_NEXT_33
    ; VPy_LINE:129
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_JOYSTICK_POLL_COUNTER
    STU TMPPTR
    STX ,U
    ; VPy_LINE:130
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
    LDU #VAR_JOY_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:131
    LDD #VAR_JOYSTICK1_STATE_DATA
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
    LDU #VAR_JOY_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_32
IF_NEXT_33:
IF_END_32:
    ; VPy_LINE:135
    LDD VAR_JOY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #40
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_38
    LDD #0
    STD RESULT
    BRA CE_39
CT_38:
    LDD #1
    STD RESULT
CE_39:
    LDD RESULT
    BEQ AND_FALSE_40
    LDD VAR_PREV_JOY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #40
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLE CT_42
    LDD #0
    STD RESULT
    BRA CE_43
CT_42:
    LDD #1
    STD RESULT
CE_43:
    LDD RESULT
    BEQ AND_FALSE_40
    LDD #1
    STD RESULT
    BRA AND_END_41
AND_FALSE_40:
    LDD #0
    STD RESULT
AND_END_41:
    LDD RESULT
    LBEQ IF_NEXT_37
    ; VPy_LINE:136
    LDD VAR_CURRENT_LOCATION
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
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    ; VPy_LINE:137
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_NUM_LOCATIONS
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_46
    LDD #0
    STD RESULT
    BRA CE_47
CT_46:
    LDD #1
    STD RESULT
CE_47:
    LDD RESULT
    LBEQ IF_NEXT_45
    ; VPy_LINE:138
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    LBRA IF_END_44
IF_NEXT_45:
IF_END_44:
    LBRA IF_END_36
IF_NEXT_37:
    LDD VAR_JOY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-40
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_49
    LDD #0
    STD RESULT
    BRA CE_50
CT_49:
    LDD #1
    STD RESULT
CE_50:
    LDD RESULT
    BEQ AND_FALSE_51
    LDD VAR_PREV_JOY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-40
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_53
    LDD #0
    STD RESULT
    BRA CE_54
CT_53:
    LDD #1
    STD RESULT
CE_54:
    LDD RESULT
    BEQ AND_FALSE_51
    LDD #1
    STD RESULT
    BRA AND_END_52
AND_FALSE_51:
    LDD #0
    STD RESULT
AND_END_52:
    LDD RESULT
    LBEQ IF_NEXT_48
    ; VPy_LINE:140
    LDD VAR_CURRENT_LOCATION
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
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    ; VPy_LINE:141
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
    BLT CT_57
    LDD #0
    STD RESULT
    BRA CE_58
CT_57:
    LDD #1
    STD RESULT
CE_58:
    LDD RESULT
    LBEQ IF_NEXT_56
    ; VPy_LINE:142
    LDD VAR_NUM_LOCATIONS
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
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    LBRA IF_END_55
IF_NEXT_56:
IF_END_55:
    LBRA IF_END_36
IF_NEXT_48:
    LDD VAR_JOY_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #40
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_60
    LDD #0
    STD RESULT
    BRA CE_61
CT_60:
    LDD #1
    STD RESULT
CE_61:
    LDD RESULT
    BEQ AND_FALSE_62
    LDD VAR_PREV_JOY_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #40
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLE CT_64
    LDD #0
    STD RESULT
    BRA CE_65
CT_64:
    LDD #1
    STD RESULT
CE_65:
    LDD RESULT
    BEQ AND_FALSE_62
    LDD #1
    STD RESULT
    BRA AND_END_63
AND_FALSE_62:
    LDD #0
    STD RESULT
AND_END_63:
    LDD RESULT
    LBEQ IF_NEXT_59
    ; VPy_LINE:144
    LDD VAR_CURRENT_LOCATION
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
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    ; VPy_LINE:145
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_NUM_LOCATIONS
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_68
    LDD #0
    STD RESULT
    BRA CE_69
CT_68:
    LDD #1
    STD RESULT
CE_69:
    LDD RESULT
    LBEQ IF_NEXT_67
    ; VPy_LINE:146
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    LBRA IF_END_66
IF_NEXT_67:
IF_END_66:
    LBRA IF_END_36
IF_NEXT_59:
    LDD VAR_JOY_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-40
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_70
    LDD #0
    STD RESULT
    BRA CE_71
CT_70:
    LDD #1
    STD RESULT
CE_71:
    LDD RESULT
    BEQ AND_FALSE_72
    LDD VAR_PREV_JOY_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-40
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_74
    LDD #0
    STD RESULT
    BRA CE_75
CT_74:
    LDD #1
    STD RESULT
CE_75:
    LDD RESULT
    BEQ AND_FALSE_72
    LDD #1
    STD RESULT
    BRA AND_END_73
AND_FALSE_72:
    LDD #0
    STD RESULT
AND_END_73:
    LDD RESULT
    LBEQ IF_END_36
    ; VPy_LINE:148
    LDD VAR_CURRENT_LOCATION
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
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    ; VPy_LINE:149
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
    BLT CT_78
    LDD #0
    STD RESULT
    BRA CE_79
CT_78:
    LDD #1
    STD RESULT
CE_79:
    LDD RESULT
    LBEQ IF_NEXT_77
    ; VPy_LINE:150
    LDD VAR_NUM_LOCATIONS
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
    LDU #VAR_CURRENT_LOCATION
    STU TMPPTR
    STX ,U
    LBRA IF_END_76
IF_NEXT_77:
IF_END_76:
    LBRA IF_END_36
IF_END_36:
    ; VPy_LINE:152
    LDD VAR_JOY_X
    STD RESULT
    LDX RESULT
    LDU #VAR_PREV_JOY_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:153
    LDD VAR_JOY_Y
    STD RESULT
    LDX RESULT
    LDU #VAR_PREV_JOY_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:155
    LDD #VAR_JOYSTICK1_STATE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #2
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
    ; VPy_LINE:157
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
    BEQ CT_82
    LDD #0
    STD RESULT
    BRA CE_83
CT_82:
    LDD #1
    STD RESULT
CE_83:
    LDD RESULT
    LBEQ IF_NEXT_81
    ; VPy_LINE:159
    LDD #2
    STD RESULT
    LDX RESULT
    LDU #VAR_SCREEN
    STU TMPPTR
    STX ,U
    ; VPy_LINE:160
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTDOWN_ACTIVE
    STU TMPPTR
    STX ,U
    ; VPy_LINE:161
    LDD #180
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTDOWN_TIMER
    STU TMPPTR
    STX ,U
    LBRA IF_END_80
IF_NEXT_81:
IF_END_80:
    ; VPy_LINE:163
    JSR DRAW_MAP_SCREEN
    LBRA IF_END_24
IF_NEXT_25:
IF_END_24:
    ; DEBUG: Statement 3 - Discriminant(9)
    ; VPy_LINE:165
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
    BEQ CT_86
    LDD #0
    STD RESULT
    BRA CE_87
CT_86:
    LDD #1
    STD RESULT
CE_87:
    LDD RESULT
    LBEQ IF_NEXT_85
    ; VPy_LINE:167
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
    BEQ CT_90
    LDD #0
    STD RESULT
    BRA CE_91
CT_90:
    LDD #1
    STD RESULT
CE_91:
    LDD RESULT
    LBEQ IF_NEXT_89
    ; VPy_LINE:169
    LDD VAR_COUNTDOWN_TIMER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #180
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_94
    LDD #0
    STD RESULT
    BRA CE_95
CT_94:
    LDD #1
    STD RESULT
CE_95:
    LDD RESULT
    LBEQ IF_NEXT_93
    ; VPy_LINE:170
    JSR SPAWN_LEVEL_ENEMIES
    LBRA IF_END_92
IF_NEXT_93:
IF_END_92:
    ; VPy_LINE:173
    JSR DRAW_LEVEL_BACKGROUND
    ; VPy_LINE:176
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 176
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:177
    LDD #65416
    STD RESULT+0
    LDD #100
    STD RESULT+2
    LDD #120
    STD RESULT+4
    LDD #100
    STD RESULT+6
    LDD #100
    STD RESULT+8
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; VPy_LINE:183
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 183
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:184
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_7
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 184
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:187
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 187
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:188
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-85
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-20
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
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 188
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:191
    LDD VAR_COUNTDOWN_TIMER
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
    LDU #VAR_COUNTDOWN_TIMER
    STU TMPPTR
    STX ,U
    ; VPy_LINE:194
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
    BLE CT_98
    LDD #0
    STD RESULT
    BRA CE_99
CT_98:
    LDD #1
    STD RESULT
CE_99:
    LDD RESULT
    LBEQ IF_NEXT_97
    ; VPy_LINE:195
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_COUNTDOWN_ACTIVE
    STU TMPPTR
    STX ,U
    LBRA IF_END_96
IF_NEXT_97:
IF_END_96:
    LBRA IF_END_88
IF_NEXT_89:
    ; VPy_LINE:200
    LDD VAR_HOOK_ACTIVE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_102
    LDD #0
    STD RESULT
    BRA CE_103
CT_102:
    LDD #1
    STD RESULT
CE_103:
    LDD RESULT
    LBEQ IF_NEXT_101
    ; VPy_LINE:201
    LDD #VAR_JOYSTICK1_STATE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_112
    LDD #0
    STD RESULT
    BRA CE_113
CT_112:
    LDD #1
    STD RESULT
CE_113:
    LDD RESULT
    BNE OR_TRUE_110
    LDD #VAR_JOYSTICK1_STATE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #3
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_114
    LDD #0
    STD RESULT
    BRA CE_115
CT_114:
    LDD #1
    STD RESULT
CE_115:
    LDD RESULT
    BNE OR_TRUE_110
    LDD #0
    STD RESULT
    BRA OR_END_111
OR_TRUE_110:
    LDD #1
    STD RESULT
OR_END_111:
    LDD RESULT
    BNE OR_TRUE_108
    LDD #VAR_JOYSTICK1_STATE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #4
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_116
    LDD #0
    STD RESULT
    BRA CE_117
CT_116:
    LDD #1
    STD RESULT
CE_117:
    LDD RESULT
    BNE OR_TRUE_108
    LDD #0
    STD RESULT
    BRA OR_END_109
OR_TRUE_108:
    LDD #1
    STD RESULT
OR_END_109:
    LDD RESULT
    BNE OR_TRUE_106
    LDD #VAR_JOYSTICK1_STATE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #5
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_118
    LDD #0
    STD RESULT
    BRA CE_119
CT_118:
    LDD #1
    STD RESULT
CE_119:
    LDD RESULT
    BNE OR_TRUE_106
    LDD #0
    STD RESULT
    BRA OR_END_107
OR_TRUE_106:
    LDD #1
    STD RESULT
OR_END_107:
    LDD RESULT
    LBEQ IF_NEXT_105
    ; VPy_LINE:202
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_HOOK_ACTIVE
    STU TMPPTR
    STX ,U
    ; VPy_LINE:203
    LDD #-100
    STD RESULT
    LDX RESULT
    LDU #VAR_HOOK_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:206
    LDD VAR_PLAYER_X
    STD RESULT
    LDX RESULT
    LDU #VAR_HOOK_GUN_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:207
    LDD VAR_PLAYER_FACING
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_122
    LDD #0
    STD RESULT
    BRA CE_123
CT_122:
    LDD #1
    STD RESULT
CE_123:
    LDD RESULT
    LBEQ IF_NEXT_121
    ; VPy_LINE:208
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #11
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_HOOK_GUN_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_120
IF_NEXT_121:
    ; VPy_LINE:210
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #11
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_HOOK_GUN_X
    STU TMPPTR
    STX ,U
IF_END_120:
    ; VPy_LINE:211
    LDD VAR_PLAYER_Y
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
    LDU #VAR_HOOK_GUN_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:212
    LDD VAR_HOOK_GUN_Y
    STD RESULT
    LDX RESULT
    LDU #VAR_HOOK_INIT_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:215
    LDD VAR_HOOK_GUN_X
    STD RESULT
    LDX RESULT
    LDU #VAR_HOOK_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_104
IF_NEXT_105:
IF_END_104:
    LBRA IF_END_100
IF_NEXT_101:
IF_END_100:
    ; VPy_LINE:218
    LDD VAR_HOOK_ACTIVE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_126
    LDD #0
    STD RESULT
    BRA CE_127
CT_126:
    LDD #1
    STD RESULT
CE_127:
    LDD RESULT
    LBEQ IF_NEXT_125
    ; VPy_LINE:219
    LDD VAR_HOOK_Y
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
    LDU #VAR_HOOK_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:221
    LDD VAR_HOOK_Y
    STD RESULT
; NATIVE_CALL: DEBUG_PRINT(hook_y) at line 221
    LDD RESULT
    STA $C002
    STB $C000
    LDA #$FE
    STA $C001
    LDX #DEBUG_LABEL_HOOK_Y
    STX $C004
    BRA DEBUG_SKIP_DATA_128
DEBUG_LABEL_HOOK_Y:
    FCC "hook_y"
    FCB $00
DEBUG_SKIP_DATA_128:
    LDD #0
    STD RESULT
    ; VPy_LINE:224
    LDD VAR_HOOK_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_HOOK_MAX_Y
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_131
    LDD #0
    STD RESULT
    BRA CE_132
CT_131:
    LDD #1
    STD RESULT
CE_132:
    LDD RESULT
    LBEQ IF_NEXT_130
    ; VPy_LINE:225
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_HOOK_ACTIVE
    STU TMPPTR
    STX ,U
    ; VPy_LINE:226
    LDD #-100
    STD RESULT
    LDX RESULT
    LDU #VAR_HOOK_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_129
IF_NEXT_130:
IF_END_129:
    LBRA IF_END_124
IF_NEXT_125:
IF_END_124:
    ; VPy_LINE:228
    JSR DRAW_GAME_LEVEL
IF_END_88:
    LBRA IF_END_84
IF_NEXT_85:
IF_END_84:
    LEAS 2,S ; free locals
    RTS

    ; VPy_LINE:230
DRAW_MAP_SCREEN: ; function
; --- function draw_map_screen ---
    LEAS -4,S ; allocate locals
    ; VPy_LINE:232
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 232
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:233
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
    BNE DSVEX_CHK_Y_133
    LDA #1
    STA MIRROR_X
DSVEX_CHK_Y_133:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE DSVEX_CHK_XY_134
    LDA #1
    STA MIRROR_Y
DSVEX_CHK_XY_134:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE DSVEX_CALL_135
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
DSVEX_CALL_135:
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
    ; VPy_LINE:236
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
    BEQ CT_138
    LDD #0
    STD RESULT
    BRA CE_139
CT_138:
    LDD #1
    STD RESULT
CE_139:
    LDD RESULT
    LBEQ IF_NEXT_137
    ; VPy_LINE:237
    LDD VAR_LOCATION_GLOW_INTENSITY
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
    LDU #VAR_LOCATION_GLOW_INTENSITY
    STU TMPPTR
    STX ,U
    ; VPy_LINE:238
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
    BGE CT_142
    LDD #0
    STD RESULT
    BRA CE_143
CT_142:
    LDD #1
    STD RESULT
CE_143:
    LDD RESULT
    LBEQ IF_NEXT_141
    ; VPy_LINE:239
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_LOCATION_GLOW_DIRECTION
    STU TMPPTR
    STX ,U
    LBRA IF_END_140
IF_NEXT_141:
IF_END_140:
    LBRA IF_END_136
IF_NEXT_137:
    ; VPy_LINE:241
    LDD VAR_LOCATION_GLOW_INTENSITY
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
    LDU #VAR_LOCATION_GLOW_INTENSITY
    STU TMPPTR
    STX ,U
    ; VPy_LINE:242
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
    BLE CT_146
    LDD #0
    STD RESULT
    BRA CE_147
CT_146:
    LDD #1
    STD RESULT
CE_147:
    LDD RESULT
    LBEQ IF_NEXT_145
    ; VPy_LINE:243
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_LOCATION_GLOW_DIRECTION
    STU TMPPTR
    STX ,U
    LBRA IF_END_144
IF_NEXT_145:
IF_END_144:
IF_END_136:
    ; VPy_LINE:246
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
    ; VPy_LINE:247
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
    ; VPy_LINE:249
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
    BNE DSVEX_CHK_Y_148
    LDA #1
    STA MIRROR_X
DSVEX_CHK_Y_148:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE DSVEX_CHK_XY_149
    LDA #1
    STA MIRROR_Y
DSVEX_CHK_XY_149:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE DSVEX_CALL_150
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
DSVEX_CALL_150:
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

    ; VPy_LINE:252
DRAW_TITLE_SCREEN: ; function
; --- function draw_title_screen ---
    ; VPy_LINE:254
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
    ; VPy_LINE:256
    LDD VAR_TITLE_INTENSITY
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 256
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:257
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_16
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 257
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:258
; PRINT_TEXT(x, y, text) - uses BIOS defaults
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_19
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 258
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:260
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
    BEQ CT_153
    LDD #0
    STD RESULT
    BRA CE_154
CT_153:
    LDD #1
    STD RESULT
CE_154:
    LDD RESULT
    LBEQ IF_NEXT_152
    ; VPy_LINE:261
    LDD VAR_TITLE_INTENSITY
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
    LDU #VAR_TITLE_INTENSITY
    STU TMPPTR
    STX ,U
    LBRA IF_END_151
IF_NEXT_152:
IF_END_151:
    ; VPy_LINE:263
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
    BEQ CT_157
    LDD #0
    STD RESULT
    BRA CE_158
CT_157:
    LDD #1
    STD RESULT
CE_158:
    LDD RESULT
    LBEQ IF_NEXT_156
    ; VPy_LINE:264
    LDD VAR_TITLE_INTENSITY
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
    LDU #VAR_TITLE_INTENSITY
    STU TMPPTR
    STX ,U
    LBRA IF_END_155
IF_NEXT_156:
IF_END_155:
    ; VPy_LINE:266
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
    BEQ CT_161
    LDD #0
    STD RESULT
    BRA CE_162
CT_161:
    LDD #1
    STD RESULT
CE_162:
    LDD RESULT
    LBEQ IF_NEXT_160
    ; VPy_LINE:267
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_TITLE_STATE
    STU TMPPTR
    STX ,U
    LBRA IF_END_159
IF_NEXT_160:
IF_END_159:
    ; VPy_LINE:269
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
    BEQ CT_165
    LDD #0
    STD RESULT
    BRA CE_166
CT_165:
    LDD #1
    STD RESULT
CE_166:
    LDD RESULT
    LBEQ IF_NEXT_164
    ; VPy_LINE:270
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_TITLE_STATE
    STU TMPPTR
    STX ,U
    LBRA IF_END_163
IF_NEXT_164:
IF_END_163:
    RTS

    ; VPy_LINE:272
DRAW_LEVEL_BACKGROUND: ; function
; --- function draw_level_background ---
    ; VPy_LINE:274
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 274
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:277
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
    BEQ CT_169
    LDD #0
    STD RESULT
    BRA CE_170
CT_169:
    LDD #1
    STD RESULT
CE_170:
    LDD RESULT
    LBEQ IF_NEXT_168
    ; VPy_LINE:278
; DRAW_VECTOR("fuji_bg", x, y) - 6 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_FUJI_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_FUJI_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_FUJI_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_FUJI_BG_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDX #_FUJI_BG_PATH4  ; Path 4
    JSR Draw_Sync_List_At
    LDX #_FUJI_BG_PATH5  ; Path 5
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_168:
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
    BEQ CT_172
    LDD #0
    STD RESULT
    BRA CE_173
CT_172:
    LDD #1
    STD RESULT
CE_173:
    LDD RESULT
    LBEQ IF_NEXT_171
    ; VPy_LINE:280
; DRAW_VECTOR("keirin_bg", x, y) - 3 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_KEIRIN_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_KEIRIN_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_KEIRIN_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_171:
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_175
    LDD #0
    STD RESULT
    BRA CE_176
CT_175:
    LDD #1
    STD RESULT
CE_176:
    LDD RESULT
    LBEQ IF_NEXT_174
    ; VPy_LINE:282
; DRAW_VECTOR("buddha_bg", x, y) - 4 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_BUDDHA_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_BUDDHA_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_BUDDHA_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_BUDDHA_BG_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_174:
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_178
    LDD #0
    STD RESULT
    BRA CE_179
CT_178:
    LDD #1
    STD RESULT
CE_179:
    LDD RESULT
    LBEQ IF_NEXT_177
    ; VPy_LINE:284
; DRAW_VECTOR("angkor_bg", x, y) - 3 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_ANGKOR_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_ANGKOR_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_ANGKOR_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_177:
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #4
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_181
    LDD #0
    STD RESULT
    BRA CE_182
CT_181:
    LDD #1
    STD RESULT
CE_182:
    LDD RESULT
    LBEQ IF_NEXT_180
    ; VPy_LINE:286
; DRAW_VECTOR("ayers_bg", x, y) - 3 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_AYERS_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_AYERS_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_AYERS_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_180:
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_184
    LDD #0
    STD RESULT
    BRA CE_185
CT_184:
    LDD #1
    STD RESULT
CE_185:
    LDD RESULT
    LBEQ IF_NEXT_183
    ; VPy_LINE:288
; DRAW_VECTOR("taj_bg", x, y) - 4 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_TAJ_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_TAJ_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_TAJ_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_TAJ_BG_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_183:
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #6
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_187
    LDD #0
    STD RESULT
    BRA CE_188
CT_187:
    LDD #1
    STD RESULT
CE_188:
    LDD RESULT
    LBEQ IF_NEXT_186
    ; VPy_LINE:290
; DRAW_VECTOR("leningrad_bg", x, y) - 5 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_LENINGRAD_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_LENINGRAD_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_LENINGRAD_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_LENINGRAD_BG_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDX #_LENINGRAD_BG_PATH4  ; Path 4
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_186:
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #7
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_190
    LDD #0
    STD RESULT
    BRA CE_191
CT_190:
    LDD #1
    STD RESULT
CE_191:
    LDD RESULT
    LBEQ IF_NEXT_189
    ; VPy_LINE:292
; DRAW_VECTOR("paris_bg", x, y) - 5 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_PARIS_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_PARIS_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_PARIS_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_PARIS_BG_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDX #_PARIS_BG_PATH4  ; Path 4
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_189:
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #8
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_193
    LDD #0
    STD RESULT
    BRA CE_194
CT_193:
    LDD #1
    STD RESULT
CE_194:
    LDD RESULT
    LBEQ IF_NEXT_192
    ; VPy_LINE:294
; DRAW_VECTOR("london_bg", x, y) - 4 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_LONDON_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_LONDON_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_LONDON_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_LONDON_BG_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_192:
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #9
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_196
    LDD #0
    STD RESULT
    BRA CE_197
CT_196:
    LDD #1
    STD RESULT
CE_197:
    LDD RESULT
    LBEQ IF_NEXT_195
    ; VPy_LINE:296
; DRAW_VECTOR("barcelona_bg", x, y) - 4 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_BARCELONA_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_BARCELONA_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_BARCELONA_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_BARCELONA_BG_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_195:
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_199
    LDD #0
    STD RESULT
    BRA CE_200
CT_199:
    LDD #1
    STD RESULT
CE_200:
    LDD RESULT
    LBEQ IF_NEXT_198
    ; VPy_LINE:298
; DRAW_VECTOR("athens_bg", x, y) - 7 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_ATHENS_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_ATHENS_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_ATHENS_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_ATHENS_BG_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDX #_ATHENS_BG_PATH4  ; Path 4
    JSR Draw_Sync_List_At
    LDX #_ATHENS_BG_PATH5  ; Path 5
    JSR Draw_Sync_List_At
    LDX #_ATHENS_BG_PATH6  ; Path 6
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_198:
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #11
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_202
    LDD #0
    STD RESULT
    BRA CE_203
CT_202:
    LDD #1
    STD RESULT
CE_203:
    LDD RESULT
    LBEQ IF_NEXT_201
    ; VPy_LINE:300
; DRAW_VECTOR("pyramids_bg", x, y) - 4 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_PYRAMIDS_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_PYRAMIDS_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_PYRAMIDS_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_PYRAMIDS_BG_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_201:
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #12
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
    LBEQ IF_NEXT_204
    ; VPy_LINE:302
; DRAW_VECTOR("kilimanjaro_bg", x, y) - 4 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_KILIMANJARO_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_KILIMANJARO_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_KILIMANJARO_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_KILIMANJARO_BG_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_204:
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #13
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_208
    LDD #0
    STD RESULT
    BRA CE_209
CT_208:
    LDD #1
    STD RESULT
CE_209:
    LDD RESULT
    LBEQ IF_NEXT_207
    ; VPy_LINE:304
; DRAW_VECTOR("newyork_bg", x, y) - 5 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_NEWYORK_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_NEWYORK_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_NEWYORK_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_NEWYORK_BG_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDX #_NEWYORK_BG_PATH4  ; Path 4
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_207:
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #14
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_211
    LDD #0
    STD RESULT
    BRA CE_212
CT_211:
    LDD #1
    STD RESULT
CE_212:
    LDD RESULT
    LBEQ IF_NEXT_210
    ; VPy_LINE:306
; DRAW_VECTOR("mayan_bg", x, y) - 5 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_MAYAN_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_MAYAN_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_MAYAN_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_MAYAN_BG_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDX #_MAYAN_BG_PATH4  ; Path 4
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_210:
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_214
    LDD #0
    STD RESULT
    BRA CE_215
CT_214:
    LDD #1
    STD RESULT
CE_215:
    LDD RESULT
    LBEQ IF_NEXT_213
    ; VPy_LINE:308
; DRAW_VECTOR("antarctica_bg", x, y) - 4 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_ANTARCTICA_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_ANTARCTICA_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_ANTARCTICA_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_ANTARCTICA_BG_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_167
IF_NEXT_213:
    ; VPy_LINE:310
; DRAW_VECTOR("easter_bg", x, y) - 5 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_EASTER_BG_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_EASTER_BG_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_EASTER_BG_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_EASTER_BG_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDX #_EASTER_BG_PATH4  ; Path 4
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
IF_END_167:
    RTS

    ; VPy_LINE:312
DRAW_GAME_LEVEL: ; function
; --- function draw_game_level ---
    LEAS -8,S ; allocate locals
    ; VPy_LINE:314
    JSR DRAW_LEVEL_BACKGROUND
    ; VPy_LINE:317
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 317
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:318
    LDD #65416
    STD RESULT+0
    LDD #65436
    STD RESULT+2
    LDD #120
    STD RESULT+4
    LDD #65436
    STD RESULT+6
    LDD #100
    STD RESULT+8
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; VPy_LINE:321
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
    LDU #VAR_JOY_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:325
    LDD VAR_JOY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-20
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_220
    LDD #0
    STD RESULT
    BRA CE_221
CT_220:
    LDD #1
    STD RESULT
CE_221:
    LDD RESULT
    BNE OR_TRUE_218
    LDD VAR_JOY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #20
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_222
    LDD #0
    STD RESULT
    BRA CE_223
CT_222:
    LDD #1
    STD RESULT
CE_223:
    LDD RESULT
    BNE OR_TRUE_218
    LDD #0
    STD RESULT
    BRA OR_END_219
OR_TRUE_218:
    LDD #1
    STD RESULT
OR_END_219:
    LDD RESULT
    LBEQ IF_NEXT_217
    ; VPy_LINE:328
    LDD VAR_JOY_X
    STD RESULT
    LDX RESULT
    LDU #VAR_ABS_JOY
    STU TMPPTR
    STX ,U
    ; VPy_LINE:329
    LDD VAR_ABS_JOY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_226
    LDD #0
    STD RESULT
    BRA CE_227
CT_226:
    LDD #1
    STD RESULT
CE_227:
    LDD RESULT
    LBEQ IF_NEXT_225
    ; VPy_LINE:330
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD VAR_ABS_JOY
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_ABS_JOY
    STU TMPPTR
    STX ,U
    LBRA IF_END_224
IF_NEXT_225:
IF_END_224:
    ; VPy_LINE:335
    LDD VAR_ABS_JOY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #40
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_230
    LDD #0
    STD RESULT
    BRA CE_231
CT_230:
    LDD #1
    STD RESULT
CE_231:
    LDD RESULT
    LBEQ IF_NEXT_229
    ; VPy_LINE:336
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_MOVE_SPEED
    STU TMPPTR
    STX ,U
    LBRA IF_END_228
IF_NEXT_229:
    LDD VAR_ABS_JOY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #70
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_233
    LDD #0
    STD RESULT
    BRA CE_234
CT_233:
    LDD #1
    STD RESULT
CE_234:
    LDD RESULT
    LBEQ IF_NEXT_232
    ; VPy_LINE:338
    LDD #2
    STD RESULT
    LDX RESULT
    LDU #VAR_MOVE_SPEED
    STU TMPPTR
    STX ,U
    LBRA IF_END_228
IF_NEXT_232:
    LDD VAR_ABS_JOY
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_236
    LDD #0
    STD RESULT
    BRA CE_237
CT_236:
    LDD #1
    STD RESULT
CE_237:
    LDD RESULT
    LBEQ IF_NEXT_235
    ; VPy_LINE:340
    LDD #3
    STD RESULT
    LDX RESULT
    LDU #VAR_MOVE_SPEED
    STU TMPPTR
    STX ,U
    LBRA IF_END_228
IF_NEXT_235:
    ; VPy_LINE:342
    LDD #4
    STD RESULT
    LDX RESULT
    LDU #VAR_MOVE_SPEED
    STU TMPPTR
    STX ,U
IF_END_228:
    ; VPy_LINE:345
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
    BLT CT_240
    LDD #0
    STD RESULT
    BRA CE_241
CT_240:
    LDD #1
    STD RESULT
CE_241:
    LDD RESULT
    LBEQ IF_NEXT_239
    ; VPy_LINE:346
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD VAR_MOVE_SPEED
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_MOVE_SPEED
    STU TMPPTR
    STX ,U
    LBRA IF_END_238
IF_NEXT_239:
IF_END_238:
    ; VPy_LINE:348
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD VAR_MOVE_SPEED
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
    ; VPy_LINE:351
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-110
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_244
    LDD #0
    STD RESULT
    BRA CE_245
CT_244:
    LDD #1
    STD RESULT
CE_245:
    LDD RESULT
    LBEQ IF_NEXT_243
    ; VPy_LINE:352
    LDD #-110
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_242
IF_NEXT_243:
IF_END_242:
    ; VPy_LINE:353
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #110
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_248
    LDD #0
    STD RESULT
    BRA CE_249
CT_248:
    LDD #1
    STD RESULT
CE_249:
    LDD RESULT
    LBEQ IF_NEXT_247
    ; VPy_LINE:354
    LDD #110
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_246
IF_NEXT_247:
IF_END_246:
    ; VPy_LINE:357
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
    BLT CT_252
    LDD #0
    STD RESULT
    BRA CE_253
CT_252:
    LDD #1
    STD RESULT
CE_253:
    LDD RESULT
    LBEQ IF_NEXT_251
    ; VPy_LINE:358
    LDD #-1
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_FACING
    STU TMPPTR
    STX ,U
    LBRA IF_END_250
IF_NEXT_251:
    ; VPy_LINE:360
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_FACING
    STU TMPPTR
    STX ,U
IF_END_250:
    ; VPy_LINE:363
    LDD VAR_PLAYER_ANIM_COUNTER
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
    LDU #VAR_PLAYER_ANIM_COUNTER
    STU TMPPTR
    STX ,U
    ; VPy_LINE:365
    LDD VAR_PLAYER_ANIM_SPEED
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:366
    LDD VAR_JOY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-80
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_258
    LDD #0
    STD RESULT
    BRA CE_259
CT_258:
    LDD #1
    STD RESULT
CE_259:
    LDD RESULT
    BNE OR_TRUE_256
    LDD VAR_JOY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #80
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_260
    LDD #0
    STD RESULT
    BRA CE_261
CT_260:
    LDD #1
    STD RESULT
CE_261:
    LDD RESULT
    BNE OR_TRUE_256
    LDD #0
    STD RESULT
    BRA OR_END_257
OR_TRUE_256:
    LDD #1
    STD RESULT
OR_END_257:
    LDD RESULT
    LBEQ IF_NEXT_255
    ; VPy_LINE:367
    LDD VAR_PLAYER_ANIM_SPEED
    STD RESULT
    LDD RESULT
    LSRA
    RORB
    STD RESULT
    LDX RESULT
    STX 2 ,S
    LBRA IF_END_254
IF_NEXT_255:
IF_END_254:
    ; VPy_LINE:369
    LDD VAR_PLAYER_ANIM_COUNTER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_264
    LDD #0
    STD RESULT
    BRA CE_265
CT_264:
    LDD #1
    STD RESULT
CE_265:
    LDD RESULT
    LBEQ IF_NEXT_263
    ; VPy_LINE:370
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_ANIM_COUNTER
    STU TMPPTR
    STX ,U
    ; VPy_LINE:371
    LDD VAR_PLAYER_ANIM_FRAME
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
    LDU #VAR_PLAYER_ANIM_FRAME
    STU TMPPTR
    STX ,U
    ; VPy_LINE:372
    LDD VAR_PLAYER_ANIM_FRAME
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #5
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_268
    LDD #0
    STD RESULT
    BRA CE_269
CT_268:
    LDD #1
    STD RESULT
CE_269:
    LDD RESULT
    LBEQ IF_NEXT_267
    ; VPy_LINE:373
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_ANIM_FRAME
    STU TMPPTR
    STX ,U
    LBRA IF_END_266
IF_NEXT_267:
IF_END_266:
    LBRA IF_END_262
IF_NEXT_263:
IF_END_262:
    LBRA IF_END_216
IF_NEXT_217:
    ; VPy_LINE:376
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_ANIM_FRAME
    STU TMPPTR
    STX ,U
    ; VPy_LINE:377
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYER_ANIM_COUNTER
    STU TMPPTR
    STX ,U
IF_END_216:
    ; VPy_LINE:380
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 380
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:383
    LDD #0
    STD RESULT
    LDX RESULT
    STX 6 ,S
    ; VPy_LINE:384
    LDD VAR_PLAYER_FACING
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_272
    LDD #0
    STD RESULT
    BRA CE_273
CT_272:
    LDD #1
    STD RESULT
CE_273:
    LDD RESULT
    LBEQ IF_NEXT_271
    ; VPy_LINE:385
    LDD #1
    STD RESULT
    LDX RESULT
    STX 6 ,S
    LBRA IF_END_270
IF_NEXT_271:
IF_END_270:
    ; VPy_LINE:388
    LDD VAR_PLAYER_ANIM_FRAME
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_276
    LDD #0
    STD RESULT
    BRA CE_277
CT_276:
    LDD #1
    STD RESULT
CE_277:
    LDD RESULT
    LBEQ IF_NEXT_275
    ; VPy_LINE:389
; DRAW_VECTOR_EX("player_walk_1", x, y, mirror) - 17 path(s), width=19, center_x=1
    LDD VAR_PLAYER_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PLAYER_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDD 6 ,S
    STD RESULT
    LDB RESULT+1  ; Mirror mode (0=normal, 1=X, 2=Y, 3=both)
    ; Decode mirror mode into separate flags:
    CLR MIRROR_X  ; Clear X flag
    CLR MIRROR_Y  ; Clear Y flag
    CMPB #1       ; Check if X-mirror (mode 1)
    BNE DSVEX_CHK_Y_278
    LDA #1
    STA MIRROR_X
DSVEX_CHK_Y_278:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE DSVEX_CHK_XY_279
    LDA #1
    STA MIRROR_Y
DSVEX_CHK_XY_279:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE DSVEX_CALL_280
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
DSVEX_CALL_280:
    ; Set intensity override for drawing
    LDD #127
    STD RESULT
    LDA RESULT+1  ; Intensity (0-127)
    STA DRAW_VEC_INTENSITY  ; Store intensity override (function will use this)
    LDX #_PLAYER_WALK_1_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH1  ; Path 1
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH2  ; Path 2
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH3  ; Path 3
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH4  ; Path 4
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH5  ; Path 5
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH6  ; Path 6
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH7  ; Path 7
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH8  ; Path 8
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH9  ; Path 9
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH10  ; Path 10
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH11  ; Path 11
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH12  ; Path 12
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH13  ; Path 13
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH14  ; Path 14
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH15  ; Path 15
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_1_PATH16  ; Path 16
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    CLR DRAW_VEC_INTENSITY  ; Clear intensity override for next draw
    LDD #0
    STD RESULT
    LBRA IF_END_274
IF_NEXT_275:
    LDD VAR_PLAYER_ANIM_FRAME
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_282
    LDD #0
    STD RESULT
    BRA CE_283
CT_282:
    LDD #1
    STD RESULT
CE_283:
    LDD RESULT
    LBEQ IF_NEXT_281
    ; VPy_LINE:391
; DRAW_VECTOR_EX("player_walk_2", x, y, mirror) - 17 path(s), width=21, center_x=0
    LDD VAR_PLAYER_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PLAYER_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDD 6 ,S
    STD RESULT
    LDB RESULT+1  ; Mirror mode (0=normal, 1=X, 2=Y, 3=both)
    ; Decode mirror mode into separate flags:
    CLR MIRROR_X  ; Clear X flag
    CLR MIRROR_Y  ; Clear Y flag
    CMPB #1       ; Check if X-mirror (mode 1)
    BNE DSVEX_CHK_Y_284
    LDA #1
    STA MIRROR_X
DSVEX_CHK_Y_284:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE DSVEX_CHK_XY_285
    LDA #1
    STA MIRROR_Y
DSVEX_CHK_XY_285:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE DSVEX_CALL_286
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
DSVEX_CALL_286:
    ; Set intensity override for drawing
    LDD #127
    STD RESULT
    LDA RESULT+1  ; Intensity (0-127)
    STA DRAW_VEC_INTENSITY  ; Store intensity override (function will use this)
    LDX #_PLAYER_WALK_2_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH1  ; Path 1
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH2  ; Path 2
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH3  ; Path 3
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH4  ; Path 4
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH5  ; Path 5
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH6  ; Path 6
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH7  ; Path 7
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH8  ; Path 8
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH9  ; Path 9
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH10  ; Path 10
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH11  ; Path 11
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH12  ; Path 12
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH13  ; Path 13
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH14  ; Path 14
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH15  ; Path 15
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_2_PATH16  ; Path 16
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    CLR DRAW_VEC_INTENSITY  ; Clear intensity override for next draw
    LDD #0
    STD RESULT
    LBRA IF_END_274
IF_NEXT_281:
    LDD VAR_PLAYER_ANIM_FRAME
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_288
    LDD #0
    STD RESULT
    BRA CE_289
CT_288:
    LDD #1
    STD RESULT
CE_289:
    LDD RESULT
    LBEQ IF_NEXT_287
    ; VPy_LINE:393
; DRAW_VECTOR_EX("player_walk_3", x, y, mirror) - 17 path(s), width=20, center_x=1
    LDD VAR_PLAYER_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PLAYER_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDD 6 ,S
    STD RESULT
    LDB RESULT+1  ; Mirror mode (0=normal, 1=X, 2=Y, 3=both)
    ; Decode mirror mode into separate flags:
    CLR MIRROR_X  ; Clear X flag
    CLR MIRROR_Y  ; Clear Y flag
    CMPB #1       ; Check if X-mirror (mode 1)
    BNE DSVEX_CHK_Y_290
    LDA #1
    STA MIRROR_X
DSVEX_CHK_Y_290:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE DSVEX_CHK_XY_291
    LDA #1
    STA MIRROR_Y
DSVEX_CHK_XY_291:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE DSVEX_CALL_292
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
DSVEX_CALL_292:
    ; Set intensity override for drawing
    LDD #127
    STD RESULT
    LDA RESULT+1  ; Intensity (0-127)
    STA DRAW_VEC_INTENSITY  ; Store intensity override (function will use this)
    LDX #_PLAYER_WALK_3_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH1  ; Path 1
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH2  ; Path 2
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH3  ; Path 3
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH4  ; Path 4
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH5  ; Path 5
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH6  ; Path 6
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH7  ; Path 7
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH8  ; Path 8
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH9  ; Path 9
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH10  ; Path 10
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH11  ; Path 11
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH12  ; Path 12
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH13  ; Path 13
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH14  ; Path 14
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH15  ; Path 15
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_3_PATH16  ; Path 16
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    CLR DRAW_VEC_INTENSITY  ; Clear intensity override for next draw
    LDD #0
    STD RESULT
    LBRA IF_END_274
IF_NEXT_287:
    LDD VAR_PLAYER_ANIM_FRAME
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #4
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_294
    LDD #0
    STD RESULT
    BRA CE_295
CT_294:
    LDD #1
    STD RESULT
CE_295:
    LDD RESULT
    LBEQ IF_NEXT_293
    ; VPy_LINE:395
; DRAW_VECTOR_EX("player_walk_4", x, y, mirror) - 17 path(s), width=19, center_x=1
    LDD VAR_PLAYER_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PLAYER_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDD 6 ,S
    STD RESULT
    LDB RESULT+1  ; Mirror mode (0=normal, 1=X, 2=Y, 3=both)
    ; Decode mirror mode into separate flags:
    CLR MIRROR_X  ; Clear X flag
    CLR MIRROR_Y  ; Clear Y flag
    CMPB #1       ; Check if X-mirror (mode 1)
    BNE DSVEX_CHK_Y_296
    LDA #1
    STA MIRROR_X
DSVEX_CHK_Y_296:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE DSVEX_CHK_XY_297
    LDA #1
    STA MIRROR_Y
DSVEX_CHK_XY_297:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE DSVEX_CALL_298
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
DSVEX_CALL_298:
    ; Set intensity override for drawing
    LDD #127
    STD RESULT
    LDA RESULT+1  ; Intensity (0-127)
    STA DRAW_VEC_INTENSITY  ; Store intensity override (function will use this)
    LDX #_PLAYER_WALK_4_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH1  ; Path 1
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH2  ; Path 2
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH3  ; Path 3
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH4  ; Path 4
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH5  ; Path 5
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH6  ; Path 6
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH7  ; Path 7
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH8  ; Path 8
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH9  ; Path 9
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH10  ; Path 10
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH11  ; Path 11
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH12  ; Path 12
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH13  ; Path 13
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH14  ; Path 14
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH15  ; Path 15
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_4_PATH16  ; Path 16
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    CLR DRAW_VEC_INTENSITY  ; Clear intensity override for next draw
    LDD #0
    STD RESULT
    LBRA IF_END_274
IF_NEXT_293:
    ; VPy_LINE:397
; DRAW_VECTOR_EX("player_walk_5", x, y, mirror) - 17 path(s), width=19, center_x=1
    LDD VAR_PLAYER_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PLAYER_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDD 6 ,S
    STD RESULT
    LDB RESULT+1  ; Mirror mode (0=normal, 1=X, 2=Y, 3=both)
    ; Decode mirror mode into separate flags:
    CLR MIRROR_X  ; Clear X flag
    CLR MIRROR_Y  ; Clear Y flag
    CMPB #1       ; Check if X-mirror (mode 1)
    BNE DSVEX_CHK_Y_299
    LDA #1
    STA MIRROR_X
DSVEX_CHK_Y_299:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE DSVEX_CHK_XY_300
    LDA #1
    STA MIRROR_Y
DSVEX_CHK_XY_300:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE DSVEX_CALL_301
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
DSVEX_CALL_301:
    ; Set intensity override for drawing
    LDD #127
    STD RESULT
    LDA RESULT+1  ; Intensity (0-127)
    STA DRAW_VEC_INTENSITY  ; Store intensity override (function will use this)
    LDX #_PLAYER_WALK_5_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH1  ; Path 1
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH2  ; Path 2
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH3  ; Path 3
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH4  ; Path 4
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH5  ; Path 5
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH6  ; Path 6
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH7  ; Path 7
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH8  ; Path 8
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH9  ; Path 9
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH10  ; Path 10
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH11  ; Path 11
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH12  ; Path 12
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH13  ; Path 13
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH14  ; Path 14
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH15  ; Path 15
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    LDX #_PLAYER_WALK_5_PATH16  ; Path 16
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    CLR DRAW_VEC_INTENSITY  ; Clear intensity override for next draw
    LDD #0
    STD RESULT
IF_END_274:
    ; VPy_LINE:404
    LDD VAR_HOOK_ACTIVE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_304
    LDD #0
    STD RESULT
    BRA CE_305
CT_304:
    LDD #1
    STD RESULT
CE_305:
    LDD RESULT
    LBEQ IF_NEXT_303
    ; VPy_LINE:409
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 409
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:411
    LDD VAR_HOOK_GUN_X
    STD RESULT
    LDD RESULT
    STD RESULT+0
    LDD VAR_HOOK_INIT_Y
    STD RESULT
    LDD RESULT
    STD RESULT+2
    LDD VAR_HOOK_X
    STD RESULT
    LDD RESULT
    STD RESULT+4
    LDD VAR_HOOK_Y
    STD RESULT
    LDD RESULT
    STD RESULT+6
    LDD #80
    STD RESULT+8
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; VPy_LINE:413
    LDD VAR_HOOK_Y
    STD RESULT
; NATIVE_CALL: DEBUG_PRINT(hook_y) at line 413
    LDD RESULT
    STA $C002
    STB $C000
    LDA #$FE
    STA $C001
    LDX #DEBUG_LABEL_HOOK_Y
    STX $C004
    BRA DEBUG_SKIP_DATA_306
DEBUG_LABEL_HOOK_Y:
    FCC "hook_y"
    FCB $00
DEBUG_SKIP_DATA_306:
    LDD #0
    STD RESULT
    ; VPy_LINE:415
; DRAW_VECTOR_EX("hook", x, y, mirror) - 1 path(s), width=12, center_x=0
    LDD VAR_HOOK_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_HOOK_Y
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
    BNE DSVEX_CHK_Y_307
    LDA #1
    STA MIRROR_X
DSVEX_CHK_Y_307:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE DSVEX_CHK_XY_308
    LDA #1
    STA MIRROR_Y
DSVEX_CHK_XY_308:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE DSVEX_CALL_309
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
DSVEX_CALL_309:
    ; Set intensity override for drawing
    LDD #100
    STD RESULT
    LDA RESULT+1  ; Intensity (0-127)
    STA DRAW_VEC_INTENSITY  ; Store intensity override (function will use this)
    LDX #_HOOK_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors  ; Uses MIRROR_X, MIRROR_Y, and DRAW_VEC_INTENSITY
    CLR DRAW_VEC_INTENSITY  ; Clear intensity override for next draw
    LDD #0
    STD RESULT
    LBRA IF_END_302
IF_NEXT_303:
IF_END_302:
    ; VPy_LINE:418
    LDD #0
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:419
    LDD #0
    STD RESULT
    LDX RESULT
    STX 4 ,S
    ; VPy_LINE:420
WH_310: ; while start
    LDD 4 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #8
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_312
    LDD #0
    STD RESULT
    BRA CE_313
CT_312:
    LDD #1
    STD RESULT
CE_313:
    LDD RESULT
    LBEQ WH_END_311
    ; VPy_LINE:421
    LDD #VAR_ENEMY_ACTIVE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 4 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_316
    LDD #0
    STD RESULT
    BRA CE_317
CT_316:
    LDD #1
    STD RESULT
CE_317:
    LDD RESULT
    LBEQ IF_NEXT_315
    ; VPy_LINE:422
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
    LBRA IF_END_314
IF_NEXT_315:
IF_END_314:
    ; VPy_LINE:423
    LDD 4 ,S
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
    STX 4 ,S
    LBRA WH_310
WH_END_311: ; while end
    LEAS 8,S ; free locals
    RTS

    ; VPy_LINE:425
INIT_ENEMIES: ; function
; --- function init_enemies ---
    LEAS -2,S ; allocate locals
    ; VPy_LINE:427
    LDD #0
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:428
WH_318: ; while start
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #8
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_320
    LDD #0
    STD RESULT
    BRA CE_321
CT_320:
    LDD #1
    STD RESULT
CE_321:
    LDD RESULT
    LBEQ WH_END_319
    ; VPy_LINE:429
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_ACTIVE_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:430
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_X_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:431
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_Y_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:432
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_VX_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:433
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_VY_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:434
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_SIZE_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:435
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
    LBRA WH_318
WH_END_319: ; while end
    LEAS 2,S ; free locals
    RTS

    ; VPy_LINE:437
SPAWN_LEVEL_ENEMIES: ; function
; --- function spawn_level_enemies ---
    LEAS -6,S ; allocate locals
    ; VPy_LINE:439
    ; ===== Const array indexing: level_enemy_count =====
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDX #CONST_ARRAY_4
    LDD TMPPTR
    LEAX D,X
    LDD ,X
    STD RESULT
    LDX RESULT
    STX 4 ,S
    ; VPy_LINE:440
    ; ===== Const array indexing: level_enemy_speed =====
    LDD VAR_CURRENT_LOCATION
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDX #CONST_ARRAY_5
    LDD TMPPTR
    LEAX D,X
    LDD ,X
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:442
    LDD #0
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:443
WH_322: ; while start
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 4 ,S
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_324
    LDD #0
    STD RESULT
    BRA CE_325
CT_324:
    LDD #1
    STD RESULT
CE_325:
    LDD RESULT
    BEQ AND_FALSE_326
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #8
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_328
    LDD #0
    STD RESULT
    BRA CE_329
CT_328:
    LDD #1
    STD RESULT
CE_329:
    LDD RESULT
    BEQ AND_FALSE_326
    LDD #1
    STD RESULT
    BRA AND_END_327
AND_FALSE_326:
    LDD #0
    STD RESULT
AND_END_327:
    LDD RESULT
    LBEQ WH_END_323
    ; VPy_LINE:444
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_ACTIVE_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #1
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:445
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_X_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #-60
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD 4 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #40
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:446
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_Y_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:449
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_VX_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD 0 ,S
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:450
    LDD 2 ,S
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
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    ; quotient in RESULT, need remainder: A - Q*B
    LDD DIV_A
    STD TMPLEFT
    LDD RESULT
    STD MUL_A
    LDD DIV_B
    STD MUL_B
    JSR MUL16
    ; product in RESULT, subtract from original A (TMPLEFT)
    LDD TMPLEFT
    SUBD RESULT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_332
    LDD #0
    STD RESULT
    BRA CE_333
CT_332:
    LDD #1
    STD RESULT
CE_333:
    LDD RESULT
    LBEQ IF_NEXT_331
    ; VPy_LINE:451
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_VX_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
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
    SUBD TMPRIGHT
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    LBRA IF_END_330
IF_NEXT_331:
IF_END_330:
    ; VPy_LINE:453
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_VY_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:454
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_SIZE_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #4
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:456
    LDD 2 ,S
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
    STX 2 ,S
    LBRA WH_322
WH_END_323: ; while end
    LEAS 6,S ; free locals
    RTS

    ; VPy_LINE:458
UPDATE_ENEMIES: ; function
; --- function update_enemies ---
    LEAS -4,S ; allocate locals
    ; VPy_LINE:460
    LDD #0
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:461
WH_334: ; while start
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #8
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_336
    LDD #0
    STD RESULT
    BRA CE_337
CT_336:
    LDD #1
    STD RESULT
CE_337:
    LDD RESULT
    LBEQ WH_END_335
    ; VPy_LINE:462
    LDD #VAR_ENEMY_ACTIVE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_340
    LDD #0
    STD RESULT
    BRA CE_341
CT_340:
    LDD #1
    STD RESULT
CE_341:
    LDD RESULT
    LBEQ IF_NEXT_339
    ; VPy_LINE:463
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_VY_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #VAR_ENEMY_VY_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
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
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:466
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_X_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #VAR_ENEMY_X_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    ADDD RESULT
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:467
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_Y_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #VAR_ENEMY_Y_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    ADDD RESULT
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:470
    LDD #10
    STD RESULT
    LDX RESULT
    STX 2 ,S
    ; VPy_LINE:471
    LDD #VAR_ENEMY_SIZE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #4
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_344
    LDD #0
    STD RESULT
    BRA CE_345
CT_344:
    LDD #1
    STD RESULT
CE_345:
    LDD RESULT
    LBEQ IF_NEXT_343
    ; VPy_LINE:472
    LDD #25
    STD RESULT
    LDX RESULT
    STX 2 ,S
    LBRA IF_END_342
IF_NEXT_343:
    LDD #VAR_ENEMY_SIZE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_347
    LDD #0
    STD RESULT
    BRA CE_348
CT_347:
    LDD #1
    STD RESULT
CE_348:
    LDD RESULT
    LBEQ IF_NEXT_346
    ; VPy_LINE:474
    LDD #20
    STD RESULT
    LDX RESULT
    STX 2 ,S
    LBRA IF_END_342
IF_NEXT_346:
    LDD #VAR_ENEMY_SIZE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_349
    LDD #0
    STD RESULT
    BRA CE_350
CT_349:
    LDD #1
    STD RESULT
CE_350:
    LDD RESULT
    LBEQ IF_END_342
    ; VPy_LINE:476
    LDD #15
    STD RESULT
    LDX RESULT
    STX 2 ,S
    LBRA IF_END_342
IF_END_342:
    ; VPy_LINE:479
    LDD #VAR_ENEMY_X_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-110
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
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_353
    LDD #0
    STD RESULT
    BRA CE_354
CT_353:
    LDD #1
    STD RESULT
CE_354:
    LDD RESULT
    LBEQ IF_NEXT_352
    ; VPy_LINE:480
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_X_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #-110
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
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:481
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_VX_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #VAR_ENEMY_VX_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    LBRA IF_END_351
IF_NEXT_352:
    LDD #VAR_ENEMY_X_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #110
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
    SUBD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_355
    LDD #0
    STD RESULT
    BRA CE_356
CT_355:
    LDD #1
    STD RESULT
CE_356:
    LDD RESULT
    LBEQ IF_END_351
    ; VPy_LINE:483
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_X_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #110
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
    SUBD TMPRIGHT
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:484
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_VX_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #VAR_ENEMY_VX_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    LBRA IF_END_351
IF_END_351:
    ; VPy_LINE:487
    LDD #VAR_ENEMY_Y_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-80
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
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_359
    LDD #0
    STD RESULT
    BRA CE_360
CT_359:
    LDD #1
    STD RESULT
CE_360:
    LDD RESULT
    LBEQ IF_NEXT_358
    ; VPy_LINE:488
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_Y_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #-80
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
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:489
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_VY_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #VAR_ENEMY_VY_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:491
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_VY_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #VAR_ENEMY_VY_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #9
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    STD MUL_A
    LDD TMPRIGHT
    STD MUL_B
    JSR MUL16
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
    STD DIV_A
    LDD TMPRIGHT
    STD DIV_B
    JSR DIV16
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    LBRA IF_END_357
IF_NEXT_358:
IF_END_357:
    ; VPy_LINE:494
    LDD #VAR_ENEMY_Y_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #110
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_363
    LDD #0
    STD RESULT
    BRA CE_364
CT_363:
    LDD #1
    STD RESULT
CE_364:
    LDD RESULT
    LBEQ IF_NEXT_362
    ; VPy_LINE:495
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_Y_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #110
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:496
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_ENEMY_VY_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #VAR_ENEMY_VY_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    LBRA IF_END_361
IF_NEXT_362:
IF_END_361:
    LBRA IF_END_338
IF_NEXT_339:
IF_END_338:
    ; VPy_LINE:498
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
    LBRA WH_334
WH_END_335: ; while end
    LEAS 4,S ; free locals
    RTS

    ; VPy_LINE:500
DRAW_ENEMIES: ; function
; --- function draw_enemies ---
    LEAS -2,S ; allocate locals
    ; VPy_LINE:502
    LDD #0
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:503
WH_365: ; while start
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #8
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_367
    LDD #0
    STD RESULT
    BRA CE_368
CT_367:
    LDD #1
    STD RESULT
CE_368:
    LDD RESULT
    LBEQ WH_END_366
    ; VPy_LINE:504
    LDD #VAR_ENEMY_ACTIVE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_371
    LDD #0
    STD RESULT
    BRA CE_372
CT_371:
    LDD #1
    STD RESULT
CE_372:
    LDD RESULT
    LBEQ IF_NEXT_370
    ; VPy_LINE:505
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 505
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:507
    LDD #VAR_ENEMY_SIZE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #4
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_375
    LDD #0
    STD RESULT
    BRA CE_376
CT_375:
    LDD #1
    STD RESULT
CE_376:
    LDD RESULT
    LBEQ IF_NEXT_374
    ; VPy_LINE:508
; DRAW_VECTOR("bubble_huge", x, y) - 1 path(s) at position
    LDD #VAR_ENEMY_X_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #VAR_ENEMY_Y_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_BUBBLE_HUGE_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_373
IF_NEXT_374:
    LDD #VAR_ENEMY_SIZE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_378
    LDD #0
    STD RESULT
    BRA CE_379
CT_378:
    LDD #1
    STD RESULT
CE_379:
    LDD RESULT
    LBEQ IF_NEXT_377
    ; VPy_LINE:510
; DRAW_VECTOR("bubble_large", x, y) - 1 path(s) at position
    LDD #VAR_ENEMY_X_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #VAR_ENEMY_Y_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_BUBBLE_LARGE_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_373
IF_NEXT_377:
    LDD #VAR_ENEMY_SIZE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_381
    LDD #0
    STD RESULT
    BRA CE_382
CT_381:
    LDD #1
    STD RESULT
CE_382:
    LDD RESULT
    LBEQ IF_NEXT_380
    ; VPy_LINE:512
; DRAW_VECTOR("bubble_medium", x, y) - 1 path(s) at position
    LDD #VAR_ENEMY_X_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #VAR_ENEMY_Y_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_BUBBLE_MEDIUM_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_373
IF_NEXT_380:
    ; VPy_LINE:514
; DRAW_VECTOR("bubble_small", x, y) - 1 path(s) at position
    LDD #VAR_ENEMY_X_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #VAR_ENEMY_Y_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_BUBBLE_SMALL_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
IF_END_373:
    LBRA IF_END_369
IF_NEXT_370:
IF_END_369:
    ; VPy_LINE:516
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
    LBRA WH_365
WH_END_366: ; while end
    LEAS 2,S ; free locals
    RTS

    ; VPy_LINE:518
DRAW_HOOK_ROPE: ; function
; --- function draw_hook_rope ---
    LEAS -8,S ; allocate locals
    LDD VAR_ARG0
    STD 4,S ; param start_x
    LDD VAR_ARG1
    STD 6,S ; param start_y
    LDD VAR_ARG2
    STD 0,S ; param end_x
    LDD VAR_ARG3
    STD 2,S ; param end_y
    ; VPy_LINE:520
    LDD 4 ,S
    STD RESULT
    LDD RESULT
    STD RESULT+0
    LDD 6 ,S
    STD RESULT
    LDD RESULT
    STD RESULT+2
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD RESULT+4
    LDD 2 ,S
    STD RESULT
    LDD RESULT
    STD RESULT+6
    LDD #127
    STD RESULT+8
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LEAS 8,S ; free locals
    RTS

    ; VPy_LINE:522
READ_JOYSTICK1_STATE: ; function
; --- function read_joystick1_state ---
    ; VPy_LINE:524
    LDD #0
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_JOYSTICK1_STATE_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
; NATIVE_CALL: J1_X at line 524
    JSR J1X_BUILTIN
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:525
    LDD #1
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_JOYSTICK1_STATE_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
; NATIVE_CALL: J1_Y at line 525
    JSR J1Y_BUILTIN
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    ; VPy_LINE:530
    LDD #VAR_JOYSTICK1_STATE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_385
    LDD #0
    STD RESULT
    BRA CE_386
CT_385:
    LDD #1
    STD RESULT
CE_386:
    LDD RESULT
    LBEQ IF_NEXT_384
    ; VPy_LINE:532
; NATIVE_CALL: J1_BUTTON_1 at line 532
    JSR J1B1_BUILTIN
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_389
    LDD #0
    STD RESULT
    BRA CE_390
CT_389:
    LDD #1
    STD RESULT
CE_390:
    LDD RESULT
    LBEQ IF_NEXT_388
    ; VPy_LINE:534
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_JOYSTICK1_STATE_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #1
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    LBRA IF_END_387
IF_NEXT_388:
IF_END_387:
    LBRA IF_END_383
IF_NEXT_384:
    ; VPy_LINE:537
; NATIVE_CALL: J1_BUTTON_1 at line 537
    JSR J1B1_BUILTIN
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_393
    LDD #0
    STD RESULT
    BRA CE_394
CT_393:
    LDD #1
    STD RESULT
CE_394:
    LDD RESULT
    LBEQ IF_NEXT_392
    ; VPy_LINE:539
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_JOYSTICK1_STATE_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    LBRA IF_END_391
IF_NEXT_392:
IF_END_391:
IF_END_383:
    ; VPy_LINE:542
    LDD #VAR_JOYSTICK1_STATE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #3
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_397
    LDD #0
    STD RESULT
    BRA CE_398
CT_397:
    LDD #1
    STD RESULT
CE_398:
    LDD RESULT
    LBEQ IF_NEXT_396
    ; VPy_LINE:543
; NATIVE_CALL: J1_BUTTON_2 at line 543
    JSR J1B2_BUILTIN
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_401
    LDD #0
    STD RESULT
    BRA CE_402
CT_401:
    LDD #1
    STD RESULT
CE_402:
    LDD RESULT
    LBEQ IF_NEXT_400
    ; VPy_LINE:544
    LDD #3
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_JOYSTICK1_STATE_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #1
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    LBRA IF_END_399
IF_NEXT_400:
IF_END_399:
    LBRA IF_END_395
IF_NEXT_396:
    ; VPy_LINE:546
; NATIVE_CALL: J1_BUTTON_2 at line 546
    JSR J1B2_BUILTIN
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_405
    LDD #0
    STD RESULT
    BRA CE_406
CT_405:
    LDD #1
    STD RESULT
CE_406:
    LDD RESULT
    LBEQ IF_NEXT_404
    ; VPy_LINE:547
    LDD #3
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_JOYSTICK1_STATE_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    LBRA IF_END_403
IF_NEXT_404:
IF_END_403:
IF_END_395:
    ; VPy_LINE:550
    LDD #VAR_JOYSTICK1_STATE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #4
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_409
    LDD #0
    STD RESULT
    BRA CE_410
CT_409:
    LDD #1
    STD RESULT
CE_410:
    LDD RESULT
    LBEQ IF_NEXT_408
    ; VPy_LINE:551
; NATIVE_CALL: J1_BUTTON_3 at line 551
    JSR J1B3_BUILTIN
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_413
    LDD #0
    STD RESULT
    BRA CE_414
CT_413:
    LDD #1
    STD RESULT
CE_414:
    LDD RESULT
    LBEQ IF_NEXT_412
    ; VPy_LINE:552
    LDD #4
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_JOYSTICK1_STATE_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #1
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    LBRA IF_END_411
IF_NEXT_412:
IF_END_411:
    LBRA IF_END_407
IF_NEXT_408:
    ; VPy_LINE:554
; NATIVE_CALL: J1_BUTTON_3 at line 554
    JSR J1B3_BUILTIN
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_417
    LDD #0
    STD RESULT
    BRA CE_418
CT_417:
    LDD #1
    STD RESULT
CE_418:
    LDD RESULT
    LBEQ IF_NEXT_416
    ; VPy_LINE:555
    LDD #4
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_JOYSTICK1_STATE_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    LBRA IF_END_415
IF_NEXT_416:
IF_END_415:
IF_END_407:
    ; VPy_LINE:558
    LDD #VAR_JOYSTICK1_STATE_DATA
    STD RESULT
    LDD RESULT
    STD TMPPTR
    LDD #5
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    ADDD TMPPTR
    TFR D,X
    LDD ,X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_421
    LDD #0
    STD RESULT
    BRA CE_422
CT_421:
    LDD #1
    STD RESULT
CE_422:
    LDD RESULT
    LBEQ IF_NEXT_420
    ; VPy_LINE:559
; NATIVE_CALL: J1_BUTTON_4 at line 559
    JSR J1B4_BUILTIN
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_425
    LDD #0
    STD RESULT
    BRA CE_426
CT_425:
    LDD #1
    STD RESULT
CE_426:
    LDD RESULT
    LBEQ IF_NEXT_424
    ; VPy_LINE:560
    LDD #5
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_JOYSTICK1_STATE_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #1
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    LBRA IF_END_423
IF_NEXT_424:
IF_END_423:
    LBRA IF_END_419
IF_NEXT_420:
    ; VPy_LINE:562
; NATIVE_CALL: J1_BUTTON_4 at line 562
    JSR J1B4_BUILTIN
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_429
    LDD #0
    STD RESULT
    BRA CE_430
CT_429:
    LDD #1
    STD RESULT
CE_430:
    LDD RESULT
    LBEQ IF_NEXT_428
    ; VPy_LINE:563
    LDD #5
    STD RESULT
    LDD RESULT
    ASLB
    ROLA
    STD TMPPTR
    LDD #VAR_JOYSTICK1_STATE_DATA
    TFR D,X
    LDD TMPPTR
    LEAX D,X
    STX TMPPTR2
    LDD #0
    STD RESULT
    LDX TMPPTR2
    LDD RESULT
    STD ,X
    LBRA IF_END_427
IF_NEXT_428:
IF_END_427:
IF_END_419:
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
VAR_JOYSTICK1_STATE_DATA EQU $CF10+12  ; Array data (6 elements)
VAR_NUM_LOCATIONS EQU $CF10+24
VAR_CURRENT_LOCATION EQU $CF10+26
VAR_LOCATION_GLOW_INTENSITY EQU $CF10+28
VAR_LOCATION_GLOW_DIRECTION EQU $CF10+30
VAR_JOY_X EQU $CF10+32
VAR_JOY_Y EQU $CF10+34
VAR_PREV_JOY_X EQU $CF10+36
VAR_PREV_JOY_Y EQU $CF10+38
VAR_INTENSITYVAL EQU $CF10+40
VAR_COUNTDOWN_TIMER EQU $CF10+42
VAR_COUNTDOWN_ACTIVE EQU $CF10+44
VAR_MAX_LEVEL_UNLOCKED EQU $CF10+46
VAR_JOYSTICK_POLL_COUNTER EQU $CF10+48
VAR_HOOK_ACTIVE EQU $CF10+50
VAR_HOOK_X EQU $CF10+52
VAR_HOOK_Y EQU $CF10+54
VAR_HOOK_MAX_Y EQU $CF10+56
VAR_HOOK_GUN_X EQU $CF10+58
VAR_HOOK_GUN_Y EQU $CF10+60
VAR_HOOK_INIT_Y EQU $CF10+62
VAR_PLAYER_X EQU $CF10+64
VAR_PLAYER_Y EQU $CF10+66
VAR_PLAYER_SPEED EQU $CF10+68
VAR_MOVE_SPEED EQU $CF10+70
VAR_ABS_JOY EQU $CF10+72
VAR_SPEED_MULTIPLIER EQU $CF10+74
VAR_PLAYER_ANIM_FRAME EQU $CF10+76
VAR_PLAYER_ANIM_COUNTER EQU $CF10+78
VAR_PLAYER_ANIM_SPEED EQU $CF10+80
VAR_PLAYER_FACING EQU $CF10+82
VAR_ENEMY_ACTIVE_DATA EQU $CF10+84  ; Array data (8 elements)
VAR_ENEMY_X_DATA EQU $CF10+100  ; Array data (8 elements)
VAR_ENEMY_Y_DATA EQU $CF10+116  ; Array data (8 elements)
VAR_ENEMY_VX_DATA EQU $CF10+132  ; Array data (8 elements)
VAR_ENEMY_VY_DATA EQU $CF10+148  ; Array data (8 elements)
VAR_ENEMY_SIZE_DATA EQU $CF10+164  ; Array data (8 elements)
; Call argument scratch space
VAR_ARG0 EQU $C8B2
VAR_ARG1 EQU $C8B4
VAR_ARG2 EQU $C8B6
VAR_ARG3 EQU $C8B8
VAR_ARG4 EQU $C8BA
VAR_ARG5 EQU $C8BC

; ========================================
; ASSET DATA SECTION
; Embedded 32 of 42 assets (unused assets excluded)
; ========================================

; Vector asset: player_walk_1
; Generated from player_walk_1.vec (Malban Draw_Sync_List format)
; Total paths: 17, points: 62
; X bounds: min=-8, max=11, width=19
; Center: (1, 0)

_PLAYER_WALK_1_WIDTH EQU 19
_PLAYER_WALK_1_CENTER_X EQU 1
_PLAYER_WALK_1_CENTER_Y EQU 0

_PLAYER_WALK_1_VECTORS:  ; Main entry
_PLAYER_WALK_1_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0C,$FB,0,0        ; path0: header (y=12, x=-5, relative to center)
    FCB $FF,$00,$08          ; line 0: flag=-1, dy=0, dx=8
    FCB $FF,$FC,$00          ; line 1: flag=-1, dy=-4, dx=0
    FCB $FF,$00,$F8          ; line 2: flag=-1, dy=0, dx=-8
    FCB $FF,$04,$00          ; closing line: flag=-1, dy=4, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH1:
    FCB 127              ; path1: intensity
    FCB $0C,$F9,0,0        ; path1: header (y=12, x=-7, relative to center)
    FCB $FF,$00,$0C          ; line 0: flag=-1, dy=0, dx=12
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH2:
    FCB 127              ; path2: intensity
    FCB $0C,$FB,0,0        ; path2: header (y=12, x=-5, relative to center)
    FCB $FF,$00,$08          ; line 0: flag=-1, dy=0, dx=8
    FCB $FF,$02,$00          ; line 1: flag=-1, dy=2, dx=0
    FCB $FF,$00,$F8          ; line 2: flag=-1, dy=0, dx=-8
    FCB $FF,$FE,$00          ; closing line: flag=-1, dy=-2, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH3:
    FCB 127              ; path3: intensity
    FCB $08,$FA,0,0        ; path3: header (y=8, x=-6, relative to center)
    FCB $FF,$00,$0A          ; line 0: flag=-1, dy=0, dx=10
    FCB $FF,$F6,$00          ; line 1: flag=-1, dy=-10, dx=0
    FCB $FF,$00,$F6          ; line 2: flag=-1, dy=0, dx=-10
    FCB $FF,$0A,$00          ; closing line: flag=-1, dy=10, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH4:
    FCB 127              ; path4: intensity
    FCB $07,$FA,0,0        ; path4: header (y=7, x=-6, relative to center)
    FCB $FF,$FF,$FF          ; line 0: flag=-1, dy=-1, dx=-1
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH5:
    FCB 127              ; path5: intensity
    FCB $06,$F9,0,0        ; path5: header (y=6, x=-7, relative to center)
    FCB $FF,$00,$FE          ; line 0: flag=-1, dy=0, dx=-2
    FCB $FF,$FA,$00          ; line 1: flag=-1, dy=-6, dx=0
    FCB $FF,$00,$02          ; line 2: flag=-1, dy=0, dx=2
    FCB $FF,$06,$00          ; closing line: flag=-1, dy=6, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH6:
    FCB 127              ; path6: intensity
    FCB $00,$F9,0,0        ; path6: header (y=0, x=-7, relative to center)
    FCB $FF,$00,$FE          ; line 0: flag=-1, dy=0, dx=-2
    FCB $FF,$FE,$00          ; line 1: flag=-1, dy=-2, dx=0
    FCB $FF,$00,$02          ; line 2: flag=-1, dy=0, dx=2
    FCB $FF,$02,$00          ; closing line: flag=-1, dy=2, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH7:
    FCB 127              ; path7: intensity
    FCB $07,$04,0,0        ; path7: header (y=7, x=4, relative to center)
    FCB $FF,$FF,$02          ; line 0: flag=-1, dy=-1, dx=2
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH8:
    FCB 127              ; path8: intensity
    FCB $06,$06,0,0        ; path8: header (y=6, x=6, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FC,$00          ; line 1: flag=-1, dy=-4, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$04,$00          ; closing line: flag=-1, dy=4, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH9:
    FCB 127              ; path9: intensity
    FCB $04,$06,0,0        ; path9: header (y=4, x=6, relative to center)
    FCB $FF,$00,$04          ; line 0: flag=-1, dy=0, dx=4
    FCB $FF,$FF,$00          ; line 1: flag=-1, dy=-1, dx=0
    FCB $FF,$00,$FC          ; line 2: flag=-1, dy=0, dx=-4
    FCB $FF,$01,$00          ; closing line: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH10:
    FCB 127              ; path10: intensity
    FCB $03,$07,0,0        ; path10: header (y=3, x=7, relative to center)
    FCB $FF,$00,$01          ; line 0: flag=-1, dy=0, dx=1
    FCB $FF,$FE,$00          ; line 1: flag=-1, dy=-2, dx=0
    FCB $FF,$00,$FF          ; line 2: flag=-1, dy=0, dx=-1
    FCB $FF,$02,$00          ; closing line: flag=-1, dy=2, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH11:
    FCB 127              ; path11: intensity
    FCB $FE,$FB,0,0        ; path11: header (y=-2, x=-5, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FA,$00          ; line 1: flag=-1, dy=-6, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$06,$00          ; closing line: flag=-1, dy=6, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH12:
    FCB 127              ; path12: intensity
    FCB $F8,$FB,0,0        ; path12: header (y=-8, x=-5, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FA,$00          ; line 1: flag=-1, dy=-6, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$06,$00          ; closing line: flag=-1, dy=6, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH13:
    FCB 127              ; path13: intensity
    FCB $F2,$FB,0,0        ; path13: header (y=-14, x=-5, relative to center)
    FCB $FF,$00,$03          ; line 0: flag=-1, dy=0, dx=3
    FCB $FF,$FF,$00          ; line 1: flag=-1, dy=-1, dx=0
    FCB $FF,$00,$FD          ; line 2: flag=-1, dy=0, dx=-3
    FCB $FF,$01,$00          ; closing line: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH14:
    FCB 127              ; path14: intensity
    FCB $FE,$01,0,0        ; path14: header (y=-2, x=1, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FA,$00          ; line 1: flag=-1, dy=-6, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$06,$00          ; closing line: flag=-1, dy=6, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH15:
    FCB 127              ; path15: intensity
    FCB $F8,$01,0,0        ; path15: header (y=-8, x=1, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FA,$00          ; line 1: flag=-1, dy=-6, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$06,$00          ; closing line: flag=-1, dy=6, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_1_PATH16:
    FCB 127              ; path16: intensity
    FCB $F2,$01,0,0        ; path16: header (y=-14, x=1, relative to center)
    FCB $FF,$00,$03          ; line 0: flag=-1, dy=0, dx=3
    FCB $FF,$FF,$00          ; line 1: flag=-1, dy=-1, dx=0
    FCB $FF,$00,$FD          ; line 2: flag=-1, dy=0, dx=-3
    FCB $FF,$01,$00          ; closing line: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

; Vector asset: player_walk_2
; Generated from player_walk_2.vec (Malban Draw_Sync_List format)
; Total paths: 17, points: 62
; X bounds: min=-10, max=11, width=21
; Center: (0, -1)

_PLAYER_WALK_2_WIDTH EQU 21
_PLAYER_WALK_2_CENTER_X EQU 0
_PLAYER_WALK_2_CENTER_Y EQU -1

_PLAYER_WALK_2_VECTORS:  ; Main entry
_PLAYER_WALK_2_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0D,$FC,0,0        ; path0: header (y=13, x=-4, relative to center)
    FCB $FF,$00,$08          ; line 0: flag=-1, dy=0, dx=8
    FCB $FF,$FC,$00          ; line 1: flag=-1, dy=-4, dx=0
    FCB $FF,$00,$F8          ; line 2: flag=-1, dy=0, dx=-8
    FCB $FF,$04,$00          ; closing line: flag=-1, dy=4, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH1:
    FCB 127              ; path1: intensity
    FCB $0D,$FA,0,0        ; path1: header (y=13, x=-6, relative to center)
    FCB $FF,$00,$0C          ; line 0: flag=-1, dy=0, dx=12
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH2:
    FCB 127              ; path2: intensity
    FCB $0D,$FC,0,0        ; path2: header (y=13, x=-4, relative to center)
    FCB $FF,$00,$08          ; line 0: flag=-1, dy=0, dx=8
    FCB $FF,$02,$00          ; line 1: flag=-1, dy=2, dx=0
    FCB $FF,$00,$F8          ; line 2: flag=-1, dy=0, dx=-8
    FCB $FF,$FE,$00          ; closing line: flag=-1, dy=-2, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH3:
    FCB 127              ; path3: intensity
    FCB $09,$FB,0,0        ; path3: header (y=9, x=-5, relative to center)
    FCB $FF,$00,$0A          ; line 0: flag=-1, dy=0, dx=10
    FCB $FF,$F6,$00          ; line 1: flag=-1, dy=-10, dx=0
    FCB $FF,$00,$F6          ; line 2: flag=-1, dy=0, dx=-10
    FCB $FF,$0A,$00          ; closing line: flag=-1, dy=10, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH4:
    FCB 127              ; path4: intensity
    FCB $08,$FB,0,0        ; path4: header (y=8, x=-5, relative to center)
    FCB $FF,$FF,$FE          ; line 0: flag=-1, dy=-1, dx=-2
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH5:
    FCB 127              ; path5: intensity
    FCB $07,$F9,0,0        ; path5: header (y=7, x=-7, relative to center)
    FCB $FF,$00,$FE          ; line 0: flag=-1, dy=0, dx=-2
    FCB $FF,$FC,$FF          ; line 1: flag=-1, dy=-4, dx=-1
    FCB $FF,$00,$02          ; line 2: flag=-1, dy=0, dx=2
    FCB $FF,$04,$01          ; closing line: flag=-1, dy=4, dx=1
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH6:
    FCB 127              ; path6: intensity
    FCB $03,$F8,0,0        ; path6: header (y=3, x=-8, relative to center)
    FCB $FF,$00,$FE          ; line 0: flag=-1, dy=0, dx=-2
    FCB $FF,$FE,$00          ; line 1: flag=-1, dy=-2, dx=0
    FCB $FF,$00,$02          ; line 2: flag=-1, dy=0, dx=2
    FCB $FF,$02,$00          ; closing line: flag=-1, dy=2, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH7:
    FCB 127              ; path7: intensity
    FCB $08,$05,0,0        ; path7: header (y=8, x=5, relative to center)
    FCB $FF,$FF,$02          ; line 0: flag=-1, dy=-1, dx=2
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH8:
    FCB 127              ; path8: intensity
    FCB $07,$07,0,0        ; path8: header (y=7, x=7, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FC,$00          ; line 1: flag=-1, dy=-4, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$04,$00          ; closing line: flag=-1, dy=4, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH9:
    FCB 127              ; path9: intensity
    FCB $05,$07,0,0        ; path9: header (y=5, x=7, relative to center)
    FCB $FF,$00,$04          ; line 0: flag=-1, dy=0, dx=4
    FCB $FF,$FF,$00          ; line 1: flag=-1, dy=-1, dx=0
    FCB $FF,$00,$FC          ; line 2: flag=-1, dy=0, dx=-4
    FCB $FF,$01,$00          ; closing line: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH10:
    FCB 127              ; path10: intensity
    FCB $04,$08,0,0        ; path10: header (y=4, x=8, relative to center)
    FCB $FF,$00,$01          ; line 0: flag=-1, dy=0, dx=1
    FCB $FF,$FE,$00          ; line 1: flag=-1, dy=-2, dx=0
    FCB $FF,$00,$FF          ; line 2: flag=-1, dy=0, dx=-1
    FCB $FF,$02,$00          ; closing line: flag=-1, dy=2, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH11:
    FCB 127              ; path11: intensity
    FCB $FF,$FB,0,0        ; path11: header (y=-1, x=-5, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FA,$01          ; line 1: flag=-1, dy=-6, dx=1
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$06,$FF          ; closing line: flag=-1, dy=6, dx=-1
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH12:
    FCB 127              ; path12: intensity
    FCB $F9,$FE,0,0        ; path12: header (y=-7, x=-2, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FA,$00          ; line 1: flag=-1, dy=-6, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$06,$00          ; closing line: flag=-1, dy=6, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH13:
    FCB 127              ; path13: intensity
    FCB $F3,$00,0,0        ; path13: header (y=-13, x=0, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FF,$00          ; line 1: flag=-1, dy=-1, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$01,$00          ; closing line: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH14:
    FCB 127              ; path14: intensity
    FCB $FF,$02,0,0        ; path14: header (y=-1, x=2, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$F9,$01          ; line 1: flag=-1, dy=-7, dx=1
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$07,$FF          ; closing line: flag=-1, dy=7, dx=-1
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH15:
    FCB 127              ; path15: intensity
    FCB $F8,$03,0,0        ; path15: header (y=-8, x=3, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$F9,$01          ; line 1: flag=-1, dy=-7, dx=1
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$07,$FF          ; closing line: flag=-1, dy=7, dx=-1
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_2_PATH16:
    FCB 127              ; path16: intensity
    FCB $F1,$04,0,0        ; path16: header (y=-15, x=4, relative to center)
    FCB $FF,$00,$03          ; line 0: flag=-1, dy=0, dx=3
    FCB $FF,$FF,$00          ; line 1: flag=-1, dy=-1, dx=0
    FCB $FF,$00,$FD          ; line 2: flag=-1, dy=0, dx=-3
    FCB $FF,$01,$00          ; closing line: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

; Vector asset: bubble_huge
; Generated from bubble_huge.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 8
; X bounds: min=-25, max=27, width=52
; Center: (1, 0)

_BUBBLE_HUGE_WIDTH EQU 52
_BUBBLE_HUGE_CENTER_X EQU 1
_BUBBLE_HUGE_CENTER_Y EQU 0

_BUBBLE_HUGE_VECTORS:  ; Main entry
_BUBBLE_HUGE_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $00,$1A,0,0        ; path0: header (y=0, x=26, relative to center)
    FCB $FF,$12,$F8          ; line 0: flag=-1, dy=18, dx=-8
    FCB $FF,$08,$EE          ; line 1: flag=-1, dy=8, dx=-18
    FCB $FF,$F8,$EE          ; line 2: flag=-1, dy=-8, dx=-18
    FCB $FF,$EE,$F8          ; line 3: flag=-1, dy=-18, dx=-8
    FCB $FF,$EE,$08          ; line 4: flag=-1, dy=-18, dx=8
    FCB $FF,$F8,$12          ; line 5: flag=-1, dy=-8, dx=18
    FCB $FF,$08,$12          ; line 6: flag=-1, dy=8, dx=18
    FCB $FF,$12,$08          ; closing line: flag=-1, dy=18, dx=8
    FCB 2                ; End marker (path complete)

; Vector asset: player_walk_3
; Generated from player_walk_3.vec (Malban Draw_Sync_List format)
; Total paths: 17, points: 62
; X bounds: min=-9, max=11, width=20
; Center: (1, -1)

_PLAYER_WALK_3_WIDTH EQU 20
_PLAYER_WALK_3_CENTER_X EQU 1
_PLAYER_WALK_3_CENTER_Y EQU -1

_PLAYER_WALK_3_VECTORS:  ; Main entry
_PLAYER_WALK_3_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0D,$FB,0,0        ; path0: header (y=13, x=-5, relative to center)
    FCB $FF,$00,$08          ; line 0: flag=-1, dy=0, dx=8
    FCB $FF,$FC,$00          ; line 1: flag=-1, dy=-4, dx=0
    FCB $FF,$00,$F8          ; line 2: flag=-1, dy=0, dx=-8
    FCB $FF,$04,$00          ; closing line: flag=-1, dy=4, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH1:
    FCB 127              ; path1: intensity
    FCB $0D,$F9,0,0        ; path1: header (y=13, x=-7, relative to center)
    FCB $FF,$00,$0C          ; line 0: flag=-1, dy=0, dx=12
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH2:
    FCB 127              ; path2: intensity
    FCB $0D,$FB,0,0        ; path2: header (y=13, x=-5, relative to center)
    FCB $FF,$00,$08          ; line 0: flag=-1, dy=0, dx=8
    FCB $FF,$02,$00          ; line 1: flag=-1, dy=2, dx=0
    FCB $FF,$00,$F8          ; line 2: flag=-1, dy=0, dx=-8
    FCB $FF,$FE,$00          ; closing line: flag=-1, dy=-2, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH3:
    FCB 127              ; path3: intensity
    FCB $09,$FA,0,0        ; path3: header (y=9, x=-6, relative to center)
    FCB $FF,$00,$0A          ; line 0: flag=-1, dy=0, dx=10
    FCB $FF,$F6,$00          ; line 1: flag=-1, dy=-10, dx=0
    FCB $FF,$00,$F6          ; line 2: flag=-1, dy=0, dx=-10
    FCB $FF,$0A,$00          ; closing line: flag=-1, dy=10, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH4:
    FCB 127              ; path4: intensity
    FCB $08,$FA,0,0        ; path4: header (y=8, x=-6, relative to center)
    FCB $FF,$FF,$FF          ; line 0: flag=-1, dy=-1, dx=-1
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH5:
    FCB 127              ; path5: intensity
    FCB $07,$F9,0,0        ; path5: header (y=7, x=-7, relative to center)
    FCB $FF,$00,$FE          ; line 0: flag=-1, dy=0, dx=-2
    FCB $FF,$F9,$FF          ; line 1: flag=-1, dy=-7, dx=-1
    FCB $FF,$00,$02          ; line 2: flag=-1, dy=0, dx=2
    FCB $FF,$07,$01          ; closing line: flag=-1, dy=7, dx=1
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH6:
    FCB 127              ; path6: intensity
    FCB $00,$F8,0,0        ; path6: header (y=0, x=-8, relative to center)
    FCB $FF,$00,$FE          ; line 0: flag=-1, dy=0, dx=-2
    FCB $FF,$FE,$00          ; line 1: flag=-1, dy=-2, dx=0
    FCB $FF,$00,$02          ; line 2: flag=-1, dy=0, dx=2
    FCB $FF,$02,$00          ; closing line: flag=-1, dy=2, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH7:
    FCB 127              ; path7: intensity
    FCB $08,$04,0,0        ; path7: header (y=8, x=4, relative to center)
    FCB $FF,$FF,$02          ; line 0: flag=-1, dy=-1, dx=2
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH8:
    FCB 127              ; path8: intensity
    FCB $07,$06,0,0        ; path8: header (y=7, x=6, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FC,$00          ; line 1: flag=-1, dy=-4, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$04,$00          ; closing line: flag=-1, dy=4, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH9:
    FCB 127              ; path9: intensity
    FCB $05,$06,0,0        ; path9: header (y=5, x=6, relative to center)
    FCB $FF,$00,$04          ; line 0: flag=-1, dy=0, dx=4
    FCB $FF,$FF,$00          ; line 1: flag=-1, dy=-1, dx=0
    FCB $FF,$00,$FC          ; line 2: flag=-1, dy=0, dx=-4
    FCB $FF,$01,$00          ; closing line: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH10:
    FCB 127              ; path10: intensity
    FCB $04,$07,0,0        ; path10: header (y=4, x=7, relative to center)
    FCB $FF,$00,$01          ; line 0: flag=-1, dy=0, dx=1
    FCB $FF,$FE,$00          ; line 1: flag=-1, dy=-2, dx=0
    FCB $FF,$00,$FF          ; line 2: flag=-1, dy=0, dx=-1
    FCB $FF,$02,$00          ; closing line: flag=-1, dy=2, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH11:
    FCB 127              ; path11: intensity
    FCB $FF,$FA,0,0        ; path11: header (y=-1, x=-6, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$F9,$FF          ; line 1: flag=-1, dy=-7, dx=-1
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$07,$01          ; closing line: flag=-1, dy=7, dx=1
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH12:
    FCB 127              ; path12: intensity
    FCB $F8,$FB,0,0        ; path12: header (y=-8, x=-5, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FA,$00          ; line 1: flag=-1, dy=-6, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$06,$00          ; closing line: flag=-1, dy=6, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH13:
    FCB 127              ; path13: intensity
    FCB $F2,$FB,0,0        ; path13: header (y=-14, x=-5, relative to center)
    FCB $FF,$00,$03          ; line 0: flag=-1, dy=0, dx=3
    FCB $FF,$FF,$00          ; line 1: flag=-1, dy=-1, dx=0
    FCB $FF,$00,$FD          ; line 2: flag=-1, dy=0, dx=-3
    FCB $FF,$01,$00          ; closing line: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH14:
    FCB 127              ; path14: intensity
    FCB $FF,$02,0,0        ; path14: header (y=-1, x=2, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$F9,$01          ; line 1: flag=-1, dy=-7, dx=1
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$07,$FF          ; closing line: flag=-1, dy=7, dx=-1
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH15:
    FCB 127              ; path15: intensity
    FCB $F8,$03,0,0        ; path15: header (y=-8, x=3, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FA,$00          ; line 1: flag=-1, dy=-6, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$06,$00          ; closing line: flag=-1, dy=6, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_3_PATH16:
    FCB 127              ; path16: intensity
    FCB $F2,$03,0,0        ; path16: header (y=-14, x=3, relative to center)
    FCB $FF,$00,$03          ; line 0: flag=-1, dy=0, dx=3
    FCB $FF,$FF,$00          ; line 1: flag=-1, dy=-1, dx=0
    FCB $FF,$00,$FD          ; line 2: flag=-1, dy=0, dx=-3
    FCB $FF,$01,$00          ; closing line: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

; Vector asset: player_walk_4
; Generated from player_walk_4.vec (Malban Draw_Sync_List format)
; Total paths: 17, points: 62
; X bounds: min=-8, max=11, width=19
; Center: (1, -1)

_PLAYER_WALK_4_WIDTH EQU 19
_PLAYER_WALK_4_CENTER_X EQU 1
_PLAYER_WALK_4_CENTER_Y EQU -1

_PLAYER_WALK_4_VECTORS:  ; Main entry
_PLAYER_WALK_4_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0D,$FB,0,0        ; path0: header (y=13, x=-5, relative to center)
    FCB $FF,$00,$08          ; line 0: flag=-1, dy=0, dx=8
    FCB $FF,$FC,$00          ; line 1: flag=-1, dy=-4, dx=0
    FCB $FF,$00,$F8          ; line 2: flag=-1, dy=0, dx=-8
    FCB $FF,$04,$00          ; closing line: flag=-1, dy=4, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH1:
    FCB 127              ; path1: intensity
    FCB $0D,$F9,0,0        ; path1: header (y=13, x=-7, relative to center)
    FCB $FF,$00,$0C          ; line 0: flag=-1, dy=0, dx=12
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH2:
    FCB 127              ; path2: intensity
    FCB $0D,$FB,0,0        ; path2: header (y=13, x=-5, relative to center)
    FCB $FF,$00,$08          ; line 0: flag=-1, dy=0, dx=8
    FCB $FF,$02,$00          ; line 1: flag=-1, dy=2, dx=0
    FCB $FF,$00,$F8          ; line 2: flag=-1, dy=0, dx=-8
    FCB $FF,$FE,$00          ; closing line: flag=-1, dy=-2, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH3:
    FCB 127              ; path3: intensity
    FCB $09,$FA,0,0        ; path3: header (y=9, x=-6, relative to center)
    FCB $FF,$00,$0A          ; line 0: flag=-1, dy=0, dx=10
    FCB $FF,$F6,$00          ; line 1: flag=-1, dy=-10, dx=0
    FCB $FF,$00,$F6          ; line 2: flag=-1, dy=0, dx=-10
    FCB $FF,$0A,$00          ; closing line: flag=-1, dy=10, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH4:
    FCB 127              ; path4: intensity
    FCB $08,$FA,0,0        ; path4: header (y=8, x=-6, relative to center)
    FCB $FF,$FF,$FF          ; line 0: flag=-1, dy=-1, dx=-1
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH5:
    FCB 127              ; path5: intensity
    FCB $07,$F9,0,0        ; path5: header (y=7, x=-7, relative to center)
    FCB $FF,$00,$FE          ; line 0: flag=-1, dy=0, dx=-2
    FCB $FF,$FA,$00          ; line 1: flag=-1, dy=-6, dx=0
    FCB $FF,$00,$02          ; line 2: flag=-1, dy=0, dx=2
    FCB $FF,$06,$00          ; closing line: flag=-1, dy=6, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH6:
    FCB 127              ; path6: intensity
    FCB $01,$F9,0,0        ; path6: header (y=1, x=-7, relative to center)
    FCB $FF,$00,$FE          ; line 0: flag=-1, dy=0, dx=-2
    FCB $FF,$FE,$00          ; line 1: flag=-1, dy=-2, dx=0
    FCB $FF,$00,$02          ; line 2: flag=-1, dy=0, dx=2
    FCB $FF,$02,$00          ; closing line: flag=-1, dy=2, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH7:
    FCB 127              ; path7: intensity
    FCB $08,$04,0,0        ; path7: header (y=8, x=4, relative to center)
    FCB $FF,$FF,$02          ; line 0: flag=-1, dy=-1, dx=2
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH8:
    FCB 127              ; path8: intensity
    FCB $07,$06,0,0        ; path8: header (y=7, x=6, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FC,$00          ; line 1: flag=-1, dy=-4, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$04,$00          ; closing line: flag=-1, dy=4, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH9:
    FCB 127              ; path9: intensity
    FCB $05,$06,0,0        ; path9: header (y=5, x=6, relative to center)
    FCB $FF,$00,$04          ; line 0: flag=-1, dy=0, dx=4
    FCB $FF,$FF,$00          ; line 1: flag=-1, dy=-1, dx=0
    FCB $FF,$00,$FC          ; line 2: flag=-1, dy=0, dx=-4
    FCB $FF,$01,$00          ; closing line: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH10:
    FCB 127              ; path10: intensity
    FCB $04,$07,0,0        ; path10: header (y=4, x=7, relative to center)
    FCB $FF,$00,$01          ; line 0: flag=-1, dy=0, dx=1
    FCB $FF,$FE,$00          ; line 1: flag=-1, dy=-2, dx=0
    FCB $FF,$00,$FF          ; line 2: flag=-1, dy=0, dx=-1
    FCB $FF,$02,$00          ; closing line: flag=-1, dy=2, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH11:
    FCB 127              ; path11: intensity
    FCB $FF,$FA,0,0        ; path11: header (y=-1, x=-6, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$F9,$01          ; line 1: flag=-1, dy=-7, dx=1
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$07,$FF          ; closing line: flag=-1, dy=7, dx=-1
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH12:
    FCB 127              ; path12: intensity
    FCB $F8,$FD,0,0        ; path12: header (y=-8, x=-3, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$F9,$00          ; line 1: flag=-1, dy=-7, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$07,$00          ; closing line: flag=-1, dy=7, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH13:
    FCB 127              ; path13: intensity
    FCB $F1,$FF,0,0        ; path13: header (y=-15, x=-1, relative to center)
    FCB $FF,$00,$03          ; line 0: flag=-1, dy=0, dx=3
    FCB $FF,$FF,$00          ; line 1: flag=-1, dy=-1, dx=0
    FCB $FF,$00,$FD          ; line 2: flag=-1, dy=0, dx=-3
    FCB $FF,$01,$00          ; closing line: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH14:
    FCB 127              ; path14: intensity
    FCB $FF,$01,0,0        ; path14: header (y=-1, x=1, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FA,$00          ; line 1: flag=-1, dy=-6, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$06,$00          ; closing line: flag=-1, dy=6, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH15:
    FCB 127              ; path15: intensity
    FCB $F9,$01,0,0        ; path15: header (y=-7, x=1, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FA,$FF          ; line 1: flag=-1, dy=-6, dx=-1
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$06,$01          ; closing line: flag=-1, dy=6, dx=1
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_4_PATH16:
    FCB 127              ; path16: intensity
    FCB $F3,$00,0,0        ; path16: header (y=-13, x=0, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FF,$00          ; line 1: flag=-1, dy=-1, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$01,$00          ; closing line: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

; Vector asset: player_walk_5
; Generated from player_walk_5.vec (Malban Draw_Sync_List format)
; Total paths: 17, points: 62
; X bounds: min=-8, max=11, width=19
; Center: (1, 0)

_PLAYER_WALK_5_WIDTH EQU 19
_PLAYER_WALK_5_CENTER_X EQU 1
_PLAYER_WALK_5_CENTER_Y EQU 0

_PLAYER_WALK_5_VECTORS:  ; Main entry
_PLAYER_WALK_5_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0C,$FB,0,0        ; path0: header (y=12, x=-5, relative to center)
    FCB $FF,$00,$08          ; line 0: flag=-1, dy=0, dx=8
    FCB $FF,$FC,$00          ; line 1: flag=-1, dy=-4, dx=0
    FCB $FF,$00,$F8          ; line 2: flag=-1, dy=0, dx=-8
    FCB $FF,$04,$00          ; closing line: flag=-1, dy=4, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH1:
    FCB 127              ; path1: intensity
    FCB $0C,$F9,0,0        ; path1: header (y=12, x=-7, relative to center)
    FCB $FF,$00,$0C          ; line 0: flag=-1, dy=0, dx=12
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH2:
    FCB 127              ; path2: intensity
    FCB $0C,$FB,0,0        ; path2: header (y=12, x=-5, relative to center)
    FCB $FF,$00,$08          ; line 0: flag=-1, dy=0, dx=8
    FCB $FF,$02,$00          ; line 1: flag=-1, dy=2, dx=0
    FCB $FF,$00,$F8          ; line 2: flag=-1, dy=0, dx=-8
    FCB $FF,$FE,$00          ; closing line: flag=-1, dy=-2, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH3:
    FCB 127              ; path3: intensity
    FCB $08,$FA,0,0        ; path3: header (y=8, x=-6, relative to center)
    FCB $FF,$00,$0A          ; line 0: flag=-1, dy=0, dx=10
    FCB $FF,$F6,$00          ; line 1: flag=-1, dy=-10, dx=0
    FCB $FF,$00,$F6          ; line 2: flag=-1, dy=0, dx=-10
    FCB $FF,$0A,$00          ; closing line: flag=-1, dy=10, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH4:
    FCB 127              ; path4: intensity
    FCB $07,$FA,0,0        ; path4: header (y=7, x=-6, relative to center)
    FCB $FF,$FF,$FF          ; line 0: flag=-1, dy=-1, dx=-1
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH5:
    FCB 127              ; path5: intensity
    FCB $06,$F9,0,0        ; path5: header (y=6, x=-7, relative to center)
    FCB $FF,$00,$FE          ; line 0: flag=-1, dy=0, dx=-2
    FCB $FF,$FB,$00          ; line 1: flag=-1, dy=-5, dx=0
    FCB $FF,$00,$02          ; line 2: flag=-1, dy=0, dx=2
    FCB $FF,$05,$00          ; closing line: flag=-1, dy=5, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH6:
    FCB 127              ; path6: intensity
    FCB $01,$F9,0,0        ; path6: header (y=1, x=-7, relative to center)
    FCB $FF,$00,$FE          ; line 0: flag=-1, dy=0, dx=-2
    FCB $FF,$FE,$00          ; line 1: flag=-1, dy=-2, dx=0
    FCB $FF,$00,$02          ; line 2: flag=-1, dy=0, dx=2
    FCB $FF,$02,$00          ; closing line: flag=-1, dy=2, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH7:
    FCB 127              ; path7: intensity
    FCB $07,$04,0,0        ; path7: header (y=7, x=4, relative to center)
    FCB $FF,$FF,$02          ; line 0: flag=-1, dy=-1, dx=2
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH8:
    FCB 127              ; path8: intensity
    FCB $06,$06,0,0        ; path8: header (y=6, x=6, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FC,$00          ; line 1: flag=-1, dy=-4, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$04,$00          ; closing line: flag=-1, dy=4, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH9:
    FCB 127              ; path9: intensity
    FCB $04,$06,0,0        ; path9: header (y=4, x=6, relative to center)
    FCB $FF,$00,$04          ; line 0: flag=-1, dy=0, dx=4
    FCB $FF,$FF,$00          ; line 1: flag=-1, dy=-1, dx=0
    FCB $FF,$00,$FC          ; line 2: flag=-1, dy=0, dx=-4
    FCB $FF,$01,$00          ; closing line: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH10:
    FCB 127              ; path10: intensity
    FCB $03,$07,0,0        ; path10: header (y=3, x=7, relative to center)
    FCB $FF,$00,$01          ; line 0: flag=-1, dy=0, dx=1
    FCB $FF,$FE,$00          ; line 1: flag=-1, dy=-2, dx=0
    FCB $FF,$00,$FF          ; line 2: flag=-1, dy=0, dx=-1
    FCB $FF,$02,$00          ; closing line: flag=-1, dy=2, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH11:
    FCB 127              ; path11: intensity
    FCB $FE,$FB,0,0        ; path11: header (y=-2, x=-5, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FA,$00          ; line 1: flag=-1, dy=-6, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$06,$00          ; closing line: flag=-1, dy=6, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH12:
    FCB 127              ; path12: intensity
    FCB $F8,$FB,0,0        ; path12: header (y=-8, x=-5, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FA,$00          ; line 1: flag=-1, dy=-6, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$06,$00          ; closing line: flag=-1, dy=6, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH13:
    FCB 127              ; path13: intensity
    FCB $F2,$FB,0,0        ; path13: header (y=-14, x=-5, relative to center)
    FCB $FF,$00,$03          ; line 0: flag=-1, dy=0, dx=3
    FCB $FF,$FF,$00          ; line 1: flag=-1, dy=-1, dx=0
    FCB $FF,$00,$FD          ; line 2: flag=-1, dy=0, dx=-3
    FCB $FF,$01,$00          ; closing line: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH14:
    FCB 127              ; path14: intensity
    FCB $FE,$01,0,0        ; path14: header (y=-2, x=1, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FA,$00          ; line 1: flag=-1, dy=-6, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$06,$00          ; closing line: flag=-1, dy=6, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH15:
    FCB 127              ; path15: intensity
    FCB $F8,$01,0,0        ; path15: header (y=-8, x=1, relative to center)
    FCB $FF,$00,$02          ; line 0: flag=-1, dy=0, dx=2
    FCB $FF,$FA,$00          ; line 1: flag=-1, dy=-6, dx=0
    FCB $FF,$00,$FE          ; line 2: flag=-1, dy=0, dx=-2
    FCB $FF,$06,$00          ; closing line: flag=-1, dy=6, dx=0
    FCB 2                ; End marker (path complete)

_PLAYER_WALK_5_PATH16:
    FCB 127              ; path16: intensity
    FCB $F2,$01,0,0        ; path16: header (y=-14, x=1, relative to center)
    FCB $FF,$00,$03          ; line 0: flag=-1, dy=0, dx=3
    FCB $FF,$FF,$00          ; line 1: flag=-1, dy=-1, dx=0
    FCB $FF,$00,$FD          ; line 2: flag=-1, dy=0, dx=-3
    FCB $FF,$01,$00          ; closing line: flag=-1, dy=1, dx=0
    FCB 2                ; End marker (path complete)

; Vector asset: bubble_large
; Generated from bubble_large.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 24
; X bounds: min=-20, max=20, width=40
; Center: (0, 0)

_BUBBLE_LARGE_WIDTH EQU 40
_BUBBLE_LARGE_CENTER_X EQU 0
_BUBBLE_LARGE_CENTER_Y EQU 0

_BUBBLE_LARGE_VECTORS:  ; Main entry
_BUBBLE_LARGE_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $00,$14,0,0        ; path0: header (y=0, x=20, relative to center)
    FCB $FF,$05,$FF          ; line 0: flag=-1, dy=5, dx=-1
    FCB $FF,$05,$FE          ; line 1: flag=-1, dy=5, dx=-2
    FCB $FF,$04,$FD          ; line 2: flag=-1, dy=4, dx=-3
    FCB $FF,$03,$FC          ; line 3: flag=-1, dy=3, dx=-4
    FCB $FF,$02,$FB          ; line 4: flag=-1, dy=2, dx=-5
    FCB $FF,$01,$FB          ; line 5: flag=-1, dy=1, dx=-5
    FCB $FF,$FF,$FB          ; line 6: flag=-1, dy=-1, dx=-5
    FCB $FF,$FE,$FB          ; line 7: flag=-1, dy=-2, dx=-5
    FCB $FF,$FD,$FC          ; line 8: flag=-1, dy=-3, dx=-4
    FCB $FF,$FC,$FD          ; line 9: flag=-1, dy=-4, dx=-3
    FCB $FF,$FB,$FE          ; line 10: flag=-1, dy=-5, dx=-2
    FCB $FF,$FB,$FF          ; line 11: flag=-1, dy=-5, dx=-1
    FCB $FF,$FB,$01          ; line 12: flag=-1, dy=-5, dx=1
    FCB $FF,$FB,$02          ; line 13: flag=-1, dy=-5, dx=2
    FCB $FF,$FC,$03          ; line 14: flag=-1, dy=-4, dx=3
    FCB $FF,$FD,$04          ; line 15: flag=-1, dy=-3, dx=4
    FCB $FF,$FE,$05          ; line 16: flag=-1, dy=-2, dx=5
    FCB $FF,$FF,$05          ; line 17: flag=-1, dy=-1, dx=5
    FCB $FF,$01,$05          ; line 18: flag=-1, dy=1, dx=5
    FCB $FF,$02,$05          ; line 19: flag=-1, dy=2, dx=5
    FCB $FF,$03,$04          ; line 20: flag=-1, dy=3, dx=4
    FCB $FF,$04,$03          ; line 21: flag=-1, dy=4, dx=3
    FCB $FF,$05,$02          ; line 22: flag=-1, dy=5, dx=2
    FCB $FF,$05,$01          ; closing line: flag=-1, dy=5, dx=1
    FCB 2                ; End marker (path complete)

; Vector asset: newyork_bg
; Generated from newyork_bg.vec (Malban Draw_Sync_List format)
; Total paths: 5, points: 22
; X bounds: min=-25, max=25, width=50
; Center: (0, 27)

_NEWYORK_BG_WIDTH EQU 50
_NEWYORK_BG_CENTER_X EQU 0
_NEWYORK_BG_CENTER_Y EQU 27

_NEWYORK_BG_VECTORS:  ; Main entry
_NEWYORK_BG_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $21,$FB,0,0        ; path0: header (y=33, x=-5, relative to center)
    FCB $FF,$05,$00          ; line 0: flag=-1, dy=5, dx=0
    FCB $FF,$00,$0A          ; line 1: flag=-1, dy=0, dx=10
    FCB $FF,$FB,$00          ; line 2: flag=-1, dy=-5, dx=0
    FCB 2                ; End marker (path complete)

_NEWYORK_BG_PATH1:
    FCB 110              ; path1: intensity
    FCB $0D,$00,0,0        ; path1: header (y=13, x=0, relative to center)
    FCB $FF,$0F,$0A          ; line 0: flag=-1, dy=15, dx=10
    FCB $FF,$05,$F6          ; line 1: flag=-1, dy=5, dx=-10
    FCB 2                ; End marker (path complete)

_NEWYORK_BG_PATH2:
    FCB 110              ; path2: intensity
    FCB $0D,$F1,0,0        ; path2: header (y=13, x=-15, relative to center)
    FCB $FF,$CE,$00          ; line 0: flag=-1, dy=-50, dx=0
    FCB $FF,$00,$1E          ; line 1: flag=-1, dy=0, dx=30
    FCB $FF,$32,$00          ; line 2: flag=-1, dy=50, dx=0
    FCB 2                ; End marker (path complete)

_NEWYORK_BG_PATH3:
    FCB 120              ; path3: intensity
    FCB $0D,$EC,0,0        ; path3: header (y=13, x=-20, relative to center)
    FCB $FF,$0A,$05          ; line 0: flag=-1, dy=10, dx=5
    FCB $FF,$FB,$05          ; line 1: flag=-1, dy=-5, dx=5
    FCB $FF,$07,$05          ; line 2: flag=-1, dy=7, dx=5
    FCB $FF,$F9,$05          ; line 3: flag=-1, dy=-7, dx=5
    FCB $FF,$07,$05          ; line 4: flag=-1, dy=7, dx=5
    FCB $FF,$F9,$05          ; line 5: flag=-1, dy=-7, dx=5
    FCB $FF,$05,$05          ; line 6: flag=-1, dy=5, dx=5
    FCB $FF,$F6,$05          ; line 7: flag=-1, dy=-10, dx=5
    FCB 2                ; End marker (path complete)

_NEWYORK_BG_PATH4:
    FCB 100              ; path4: intensity
    FCB $DB,$E7,0,0        ; path4: header (y=-37, x=-25, relative to center)
    FCB $FF,$00,$32          ; line 0: flag=-1, dy=0, dx=50
    FCB 2                ; End marker (path complete)

; Vector asset: pyramids_bg
; Generated from pyramids_bg.vec (Malban Draw_Sync_List format)
; Total paths: 4, points: 10
; X bounds: min=-90, max=90, width=180
; Center: (0, 0)

_PYRAMIDS_BG_WIDTH EQU 180
_PYRAMIDS_BG_CENTER_X EQU 0
_PYRAMIDS_BG_CENTER_Y EQU 0

_PYRAMIDS_BG_VECTORS:  ; Main entry
_PYRAMIDS_BG_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $D3,$A6,0,0        ; path0: header (y=-45, x=-90, relative to center)
    FCB $FF,$5A,$50          ; line 0: flag=-1, dy=90, dx=80
    FCB $FF,$A6,$50          ; line 1: flag=-1, dy=-90, dx=80
    FCB 2                ; End marker (path complete)

_PYRAMIDS_BG_PATH1:
    FCB 100              ; path1: intensity
    FCB $D3,$A6,0,0        ; path1: header (y=-45, x=-90, relative to center)
    FCB $FF,$5A,$50          ; line 0: flag=-1, dy=90, dx=80
    FCB 2                ; End marker (path complete)

_PYRAMIDS_BG_PATH2:
    FCB 80              ; path2: intensity
    FCB $2D,$F6,0,0        ; path2: header (y=45, x=-10, relative to center)
    FCB $FF,$A6,$50          ; line 0: flag=-1, dy=-90, dx=80
    FCB 2                ; End marker (path complete)

_PYRAMIDS_BG_PATH3:
    FCB 90              ; path3: intensity
    FCB $D3,$1E,0,0        ; path3: header (y=-45, x=30, relative to center)
    FCB $FF,$2D,$1E          ; line 0: flag=-1, dy=45, dx=30
    FCB $FF,$D3,$1E          ; line 1: flag=-1, dy=-45, dx=30
    FCB 2                ; End marker (path complete)

; Vector asset: easter_bg
; Generated from easter_bg.vec (Malban Draw_Sync_List format)
; Total paths: 5, points: 19
; X bounds: min=-35, max=35, width=70
; Center: (0, 15)

_EASTER_BG_WIDTH EQU 70
_EASTER_BG_CENTER_X EQU 0
_EASTER_BG_CENTER_Y EQU 15

_EASTER_BG_VECTORS:  ; Main entry
_EASTER_BG_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $05,$E7,0,0        ; path0: header (y=5, x=-25, relative to center)
    FCB $FF,$1E,$00          ; line 0: flag=-1, dy=30, dx=0
    FCB $FF,$0A,$05          ; line 1: flag=-1, dy=10, dx=5
    FCB $FF,$00,$28          ; line 2: flag=-1, dy=0, dx=40
    FCB $FF,$F6,$05          ; line 3: flag=-1, dy=-10, dx=5
    FCB $FF,$E2,$00          ; line 4: flag=-1, dy=-30, dx=0
    FCB 2                ; End marker (path complete)

_EASTER_BG_PATH1:
    FCB 110              ; path1: intensity
    FCB $19,$00,0,0        ; path1: header (y=25, x=0, relative to center)
    FCB $FF,$FB,$0A          ; line 0: flag=-1, dy=-5, dx=10
    FCB 2                ; End marker (path complete)

_EASTER_BG_PATH2:
    FCB 100              ; path2: intensity
    FCB $1E,$F8,0,0        ; path2: header (y=30, x=-8, relative to center)
    FCB $FF,$05,$00          ; line 0: flag=-1, dy=5, dx=0
    FCB $FF,$00,$05          ; line 1: flag=-1, dy=0, dx=5
    FCB $FF,$FB,$00          ; line 2: flag=-1, dy=-5, dx=0
    FCB $FF,$00,$FB          ; line 3: flag=-1, dy=0, dx=-5
    FCB 2                ; End marker (path complete)

_EASTER_BG_PATH3:
    FCB 110              ; path3: intensity
    FCB $05,$E2,0,0        ; path3: header (y=5, x=-30, relative to center)
    FCB $FF,$CE,$00          ; line 0: flag=-1, dy=-50, dx=0
    FCB $FF,$00,$3C          ; line 1: flag=-1, dy=0, dx=60
    FCB $FF,$32,$00          ; line 2: flag=-1, dy=50, dx=0
    FCB 2                ; End marker (path complete)

_EASTER_BG_PATH4:
    FCB 90              ; path4: intensity
    FCB $D3,$DD,0,0        ; path4: header (y=-45, x=-35, relative to center)
    FCB $FF,$00,$46          ; line 0: flag=-1, dy=0, dx=70
    FCB 2                ; End marker (path complete)

; Vector asset: keirin_bg
; Generated from keirin_bg.vec (Malban Draw_Sync_List format)
; Total paths: 3, points: 11
; X bounds: min=-100, max=100, width=200
; Center: (0, 10)

_KEIRIN_BG_WIDTH EQU 200
_KEIRIN_BG_CENTER_X EQU 0
_KEIRIN_BG_CENTER_Y EQU 10

_KEIRIN_BG_VECTORS:  ; Main entry
_KEIRIN_BG_PATH0:    ; Path 0
    FCB 100              ; path0: intensity
    FCB $D8,$9C,0,0        ; path0: header (y=-40, x=-100, relative to center)
    FCB $FF,$46,$32          ; line 0: flag=-1, dy=70, dx=50
    FCB $FF,$0A,$32          ; line 1: flag=-1, dy=10, dx=50
    FCB $FF,$F6,$32          ; line 2: flag=-1, dy=-10, dx=50
    FCB $FF,$BA,$32          ; line 3: flag=-1, dy=-70, dx=50
    FCB 2                ; End marker (path complete)

_KEIRIN_BG_PATH1:
    FCB 80              ; path1: intensity
    FCB $EC,$BA,0,0        ; path1: header (y=-20, x=-70, relative to center)
    FCB $FF,$1E,$1E          ; line 0: flag=-1, dy=30, dx=30
    FCB $FF,$0A,$1E          ; line 1: flag=-1, dy=10, dx=30
    FCB 2                ; End marker (path complete)

_KEIRIN_BG_PATH2:
    FCB 80              ; path2: intensity
    FCB $14,$0A,0,0        ; path2: header (y=20, x=10, relative to center)
    FCB $FF,$F6,$1E          ; line 0: flag=-1, dy=-10, dx=30
    FCB $FF,$E2,$1E          ; line 1: flag=-1, dy=-30, dx=30
    FCB 2                ; End marker (path complete)

; Vector asset: barcelona_bg
; Generated from barcelona_bg.vec (Malban Draw_Sync_List format)
; Total paths: 4, points: 20
; X bounds: min=-50, max=50, width=100
; Center: (0, 22)

_BARCELONA_BG_WIDTH EQU 100
_BARCELONA_BG_CENTER_X EQU 0
_BARCELONA_BG_CENTER_Y EQU 22

_BARCELONA_BG_VECTORS:  ; Main entry
_BARCELONA_BG_PATH0:    ; Path 0
    FCB 120              ; path0: intensity
    FCB $D6,$CE,0,0        ; path0: header (y=-42, x=-50, relative to center)
    FCB $FF,$46,$00          ; line 0: flag=-1, dy=70, dx=0
    FCB $FF,$0A,$05          ; line 1: flag=-1, dy=10, dx=5
    FCB $FF,$F6,$05          ; line 2: flag=-1, dy=-10, dx=5
    FCB $FF,$BA,$00          ; line 3: flag=-1, dy=-70, dx=0
    FCB 2                ; End marker (path complete)

_BARCELONA_BG_PATH1:
    FCB 127              ; path1: intensity
    FCB $D6,$EC,0,0        ; path1: header (y=-42, x=-20, relative to center)
    FCB $FF,$4B,$00          ; line 0: flag=-1, dy=75, dx=0
    FCB $FF,$0A,$05          ; line 1: flag=-1, dy=10, dx=5
    FCB $FF,$F6,$05          ; line 2: flag=-1, dy=-10, dx=5
    FCB $FF,$B5,$00          ; line 3: flag=-1, dy=-75, dx=0
    FCB 2                ; End marker (path complete)

_BARCELONA_BG_PATH2:
    FCB 127              ; path2: intensity
    FCB $D6,$0A,0,0        ; path2: header (y=-42, x=10, relative to center)
    FCB $FF,$4B,$00          ; line 0: flag=-1, dy=75, dx=0
    FCB $FF,$0A,$05          ; line 1: flag=-1, dy=10, dx=5
    FCB $FF,$F6,$05          ; line 2: flag=-1, dy=-10, dx=5
    FCB $FF,$B5,$00          ; line 3: flag=-1, dy=-75, dx=0
    FCB 2                ; End marker (path complete)

_BARCELONA_BG_PATH3:
    FCB 120              ; path3: intensity
    FCB $D6,$28,0,0        ; path3: header (y=-42, x=40, relative to center)
    FCB $FF,$46,$00          ; line 0: flag=-1, dy=70, dx=0
    FCB $FF,$0A,$05          ; line 1: flag=-1, dy=10, dx=5
    FCB $FF,$F6,$05          ; line 2: flag=-1, dy=-10, dx=5
    FCB $FF,$BA,$00          ; line 3: flag=-1, dy=-70, dx=0
    FCB 2                ; End marker (path complete)

; Vector asset: bubble_small
; Generated from bubble_small.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 24
; X bounds: min=-10, max=10, width=20
; Center: (0, 0)

_BUBBLE_SMALL_WIDTH EQU 20
_BUBBLE_SMALL_CENTER_X EQU 0
_BUBBLE_SMALL_CENTER_Y EQU 0

_BUBBLE_SMALL_VECTORS:  ; Main entry
_BUBBLE_SMALL_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $00,$0A,0,0        ; path0: header (y=0, x=10, relative to center)
    FCB $FF,$03,$FF          ; line 0: flag=-1, dy=3, dx=-1
    FCB $FF,$02,$00          ; line 1: flag=-1, dy=2, dx=0
    FCB $FF,$02,$FE          ; line 2: flag=-1, dy=2, dx=-2
    FCB $FF,$02,$FE          ; line 3: flag=-1, dy=2, dx=-2
    FCB $FF,$00,$FE          ; line 4: flag=-1, dy=0, dx=-2
    FCB $FF,$01,$FD          ; line 5: flag=-1, dy=1, dx=-3
    FCB $FF,$FF,$FD          ; line 6: flag=-1, dy=-1, dx=-3
    FCB $FF,$00,$FE          ; line 7: flag=-1, dy=0, dx=-2
    FCB $FF,$FE,$FE          ; line 8: flag=-1, dy=-2, dx=-2
    FCB $FF,$FE,$FE          ; line 9: flag=-1, dy=-2, dx=-2
    FCB $FF,$FE,$00          ; line 10: flag=-1, dy=-2, dx=0
    FCB $FF,$FD,$FF          ; line 11: flag=-1, dy=-3, dx=-1
    FCB $FF,$FD,$01          ; line 12: flag=-1, dy=-3, dx=1
    FCB $FF,$FE,$00          ; line 13: flag=-1, dy=-2, dx=0
    FCB $FF,$FE,$02          ; line 14: flag=-1, dy=-2, dx=2
    FCB $FF,$FE,$02          ; line 15: flag=-1, dy=-2, dx=2
    FCB $FF,$00,$02          ; line 16: flag=-1, dy=0, dx=2
    FCB $FF,$FF,$03          ; line 17: flag=-1, dy=-1, dx=3
    FCB $FF,$01,$03          ; line 18: flag=-1, dy=1, dx=3
    FCB $FF,$00,$02          ; line 19: flag=-1, dy=0, dx=2
    FCB $FF,$02,$02          ; line 20: flag=-1, dy=2, dx=2
    FCB $FF,$02,$02          ; line 21: flag=-1, dy=2, dx=2
    FCB $FF,$02,$00          ; line 22: flag=-1, dy=2, dx=0
    FCB $FF,$03,$01          ; closing line: flag=-1, dy=3, dx=1
    FCB 2                ; End marker (path complete)

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

; Vector asset: angkor_bg
; Generated from angkor_bg.vec (Malban Draw_Sync_List format)
; Total paths: 3, points: 16
; X bounds: min=-60, max=60, width=120
; Center: (0, 12)

_ANGKOR_BG_WIDTH EQU 120
_ANGKOR_BG_CENTER_X EQU 0
_ANGKOR_BG_CENTER_Y EQU 12

_ANGKOR_BG_VECTORS:  ; Main entry
_ANGKOR_BG_PATH0:    ; Path 0
    FCB 120              ; path0: intensity
    FCB $D6,$EC,0,0        ; path0: header (y=-42, x=-20, relative to center)
    FCB $FF,$46,$00          ; line 0: flag=-1, dy=70, dx=0
    FCB $FF,$0F,$0A          ; line 1: flag=-1, dy=15, dx=10
    FCB $FF,$00,$14          ; line 2: flag=-1, dy=0, dx=20
    FCB $FF,$F1,$0A          ; line 3: flag=-1, dy=-15, dx=10
    FCB $FF,$BA,$00          ; line 4: flag=-1, dy=-70, dx=0
    FCB 2                ; End marker (path complete)

_ANGKOR_BG_PATH1:
    FCB 100              ; path1: intensity
    FCB $E0,$C4,0,0        ; path1: header (y=-32, x=-60, relative to center)
    FCB $FF,$32,$00          ; line 0: flag=-1, dy=50, dx=0
    FCB $FF,$0A,$0A          ; line 1: flag=-1, dy=10, dx=10
    FCB $FF,$F6,$0A          ; line 2: flag=-1, dy=-10, dx=10
    FCB $FF,$CE,$00          ; line 3: flag=-1, dy=-50, dx=0
    FCB 2                ; End marker (path complete)

_ANGKOR_BG_PATH2:
    FCB 100              ; path2: intensity
    FCB $E0,$28,0,0        ; path2: header (y=-32, x=40, relative to center)
    FCB $FF,$32,$00          ; line 0: flag=-1, dy=50, dx=0
    FCB $FF,$0A,$0A          ; line 1: flag=-1, dy=10, dx=10
    FCB $FF,$F6,$0A          ; line 2: flag=-1, dy=-10, dx=10
    FCB $FF,$CE,$00          ; line 3: flag=-1, dy=-50, dx=0
    FCB 2                ; End marker (path complete)

; Vector asset: paris_bg
; Generated from paris_bg.vec (Malban Draw_Sync_List format)
; Total paths: 5, points: 15
; X bounds: min=-50, max=50, width=100
; Center: (0, 17)

_PARIS_BG_WIDTH EQU 100
_PARIS_BG_CENTER_X EQU 0
_PARIS_BG_CENTER_Y EQU 17

_PARIS_BG_VECTORS:  ; Main entry
_PARIS_BG_PATH0:    ; Path 0
    FCB 100              ; path0: intensity
    FCB $D1,$CE,0,0        ; path0: header (y=-47, x=-50, relative to center)
    FCB $FF,$1E,$1E          ; line 0: flag=-1, dy=30, dx=30
    FCB $FF,$1E,$0A          ; line 1: flag=-1, dy=30, dx=10
    FCB 2                ; End marker (path complete)

_PARIS_BG_PATH1:
    FCB 100              ; path1: intensity
    FCB $D1,$32,0,0        ; path1: header (y=-47, x=50, relative to center)
    FCB $FF,$1E,$E2          ; line 0: flag=-1, dy=30, dx=-30
    FCB $FF,$1E,$F6          ; line 1: flag=-1, dy=30, dx=-10
    FCB 2                ; End marker (path complete)

_PARIS_BG_PATH2:
    FCB 110              ; path2: intensity
    FCB $0D,$F6,0,0        ; path2: header (y=13, x=-10, relative to center)
    FCB $FF,$14,$05          ; line 0: flag=-1, dy=20, dx=5
    FCB $FF,$00,$0A          ; line 1: flag=-1, dy=0, dx=10
    FCB $FF,$EC,$05          ; line 2: flag=-1, dy=-20, dx=5
    FCB 2                ; End marker (path complete)

_PARIS_BG_PATH3:
    FCB 127              ; path3: intensity
    FCB $21,$FB,0,0        ; path3: header (y=33, x=-5, relative to center)
    FCB $FF,$0F,$05          ; line 0: flag=-1, dy=15, dx=5
    FCB $FF,$F1,$05          ; line 1: flag=-1, dy=-15, dx=5
    FCB 2                ; End marker (path complete)

_PARIS_BG_PATH4:
    FCB 90              ; path4: intensity
    FCB $EF,$EC,0,0        ; path4: header (y=-17, x=-20, relative to center)
    FCB $FF,$00,$28          ; line 0: flag=-1, dy=0, dx=40
    FCB 2                ; End marker (path complete)

; Vector asset: buddha_bg
; Generated from buddha_bg.vec (Malban Draw_Sync_List format)
; Total paths: 4, points: 10
; X bounds: min=-80, max=80, width=160
; Center: (0, 20)

_BUDDHA_BG_WIDTH EQU 160
_BUDDHA_BG_CENTER_X EQU 0
_BUDDHA_BG_CENTER_Y EQU 20

_BUDDHA_BG_VECTORS:  ; Main entry
_BUDDHA_BG_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $14,$B0,0,0        ; path0: header (y=20, x=-80, relative to center)
    FCB $FF,$14,$14          ; line 0: flag=-1, dy=20, dx=20
    FCB $FF,$00,$78          ; line 1: flag=-1, dy=0, dx=120
    FCB $FF,$EC,$14          ; line 2: flag=-1, dy=-20, dx=20
    FCB 2                ; End marker (path complete)

_BUDDHA_BG_PATH1:
    FCB 100              ; path1: intensity
    FCB $14,$CE,0,0        ; path1: header (y=20, x=-50, relative to center)
    FCB $FF,$C4,$00          ; line 0: flag=-1, dy=-60, dx=0
    FCB 2                ; End marker (path complete)

_BUDDHA_BG_PATH2:
    FCB 100              ; path2: intensity
    FCB $14,$32,0,0        ; path2: header (y=20, x=50, relative to center)
    FCB $FF,$C4,$00          ; line 0: flag=-1, dy=-60, dx=0
    FCB 2                ; End marker (path complete)

_BUDDHA_BG_PATH3:
    FCB 100              ; path3: intensity
    FCB $D8,$BA,0,0        ; path3: header (y=-40, x=-70, relative to center)
    FCB $FF,$00,$7F          ; line 0: flag=-1, dy=0, dx=127
    FCB 2                ; End marker (path complete)

; Vector asset: taj_bg
; Generated from taj_bg.vec (Malban Draw_Sync_List format)
; Total paths: 4, points: 13
; X bounds: min=-70, max=70, width=140
; Center: (0, 22)

_TAJ_BG_WIDTH EQU 140
_TAJ_BG_CENTER_X EQU 0
_TAJ_BG_CENTER_Y EQU 22

_TAJ_BG_VECTORS:  ; Main entry
_TAJ_BG_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $12,$E2,0,0        ; path0: header (y=18, x=-30, relative to center)
    FCB $FF,$14,$0A          ; line 0: flag=-1, dy=20, dx=10
    FCB $FF,$05,$14          ; line 1: flag=-1, dy=5, dx=20
    FCB $FF,$FB,$14          ; line 2: flag=-1, dy=-5, dx=20
    FCB $FF,$EC,$0A          ; line 3: flag=-1, dy=-20, dx=10
    FCB 2                ; End marker (path complete)

_TAJ_BG_PATH1:
    FCB 110              ; path1: intensity
    FCB $12,$D8,0,0        ; path1: header (y=18, x=-40, relative to center)
    FCB $FF,$CE,$00          ; line 0: flag=-1, dy=-50, dx=0
    FCB $FF,$00,$50          ; line 1: flag=-1, dy=0, dx=80
    FCB $FF,$32,$00          ; line 2: flag=-1, dy=50, dx=0
    FCB 2                ; End marker (path complete)

_TAJ_BG_PATH2:
    FCB 100              ; path2: intensity
    FCB $D6,$BA,0,0        ; path2: header (y=-42, x=-70, relative to center)
    FCB $FF,$46,$00          ; line 0: flag=-1, dy=70, dx=0
    FCB 2                ; End marker (path complete)

_TAJ_BG_PATH3:
    FCB 100              ; path3: intensity
    FCB $D6,$46,0,0        ; path3: header (y=-42, x=70, relative to center)
    FCB $FF,$46,$00          ; line 0: flag=-1, dy=70, dx=0
    FCB 2                ; End marker (path complete)

; Vector asset: mayan_bg
; Generated from mayan_bg.vec (Malban Draw_Sync_List format)
; Total paths: 5, points: 20
; X bounds: min=-80, max=80, width=160
; Center: (0, 10)

_MAYAN_BG_WIDTH EQU 160
_MAYAN_BG_CENTER_X EQU 0
_MAYAN_BG_CENTER_Y EQU 10

_MAYAN_BG_VECTORS:  ; Main entry
_MAYAN_BG_PATH0:    ; Path 0
    FCB 100              ; path0: intensity
    FCB $D8,$B0,0,0        ; path0: header (y=-40, x=-80, relative to center)
    FCB $FF,$00,$7F          ; line 0: flag=-1, dy=0, dx=127
    FCB 2                ; End marker (path complete)

_MAYAN_BG_PATH1:
    FCB 110              ; path1: intensity
    FCB $D8,$BA,0,0        ; path1: header (y=-40, x=-70, relative to center)
    FCB $FF,$0A,$00          ; line 0: flag=-1, dy=10, dx=0
    FCB $FF,$00,$7F          ; line 1: flag=-1, dy=0, dx=127
    FCB $FF,$F6,$00          ; line 2: flag=-1, dy=-10, dx=0
    FCB 2                ; End marker (path complete)

_MAYAN_BG_PATH2:
    FCB 110              ; path2: intensity
    FCB $E2,$C4,0,0        ; path2: header (y=-30, x=-60, relative to center)
    FCB $FF,$0A,$00          ; line 0: flag=-1, dy=10, dx=0
    FCB $FF,$00,$78          ; line 1: flag=-1, dy=0, dx=120
    FCB $FF,$F6,$00          ; line 2: flag=-1, dy=-10, dx=0
    FCB 2                ; End marker (path complete)

_MAYAN_BG_PATH3:
    FCB 120              ; path3: intensity
    FCB $EC,$CE,0,0        ; path3: header (y=-20, x=-50, relative to center)
    FCB $FF,$0A,$00          ; line 0: flag=-1, dy=10, dx=0
    FCB $FF,$00,$64          ; line 1: flag=-1, dy=0, dx=100
    FCB $FF,$F6,$00          ; line 2: flag=-1, dy=-10, dx=0
    FCB 2                ; End marker (path complete)

_MAYAN_BG_PATH4:
    FCB 127              ; path4: intensity
    FCB $F6,$D8,0,0        ; path4: header (y=-10, x=-40, relative to center)
    FCB $FF,$28,$00          ; line 0: flag=-1, dy=40, dx=0
    FCB $FF,$0A,$0A          ; line 1: flag=-1, dy=10, dx=10
    FCB $FF,$00,$3C          ; line 2: flag=-1, dy=0, dx=60
    FCB $FF,$F6,$0A          ; line 3: flag=-1, dy=-10, dx=10
    FCB $FF,$D8,$00          ; line 4: flag=-1, dy=-40, dx=0
    FCB 2                ; End marker (path complete)

; Vector asset: hook
; Generated from hook.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 10
; X bounds: min=-6, max=6, width=12
; Center: (0, 0)

_HOOK_WIDTH EQU 12
_HOOK_CENTER_X EQU 0
_HOOK_CENTER_Y EQU 0

_HOOK_VECTORS:  ; Main entry
_HOOK_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $FC,$FA,0,0        ; path0: header (y=-4, x=-6, relative to center)
    FCB $FF,$0B,$06          ; line 0: flag=-1, dy=11, dx=6
    FCB $FF,$F5,$06          ; line 1: flag=-1, dy=-11, dx=6
    FCB $FF,$00,$FF          ; line 2: flag=-1, dy=0, dx=-1
    FCB $FF,$04,$FC          ; line 3: flag=-1, dy=4, dx=-4
    FCB $FF,$F8,$00          ; line 4: flag=-1, dy=-8, dx=0
    FCB $FF,$00,$FE          ; line 5: flag=-1, dy=0, dx=-2
    FCB $FF,$08,$00          ; line 6: flag=-1, dy=8, dx=0
    FCB $FF,$FC,$FC          ; line 7: flag=-1, dy=-4, dx=-4
    FCB $FF,$00,$FF          ; line 8: flag=-1, dy=0, dx=-1
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

; Vector asset: london_bg
; Generated from london_bg.vec (Malban Draw_Sync_List format)
; Total paths: 4, points: 16
; X bounds: min=-20, max=20, width=40
; Center: (0, 15)

_LONDON_BG_WIDTH EQU 40
_LONDON_BG_CENTER_X EQU 0
_LONDON_BG_CENTER_Y EQU 15

_LONDON_BG_VECTORS:  ; Main entry
_LONDON_BG_PATH0:    ; Path 0
    FCB 110              ; path0: intensity
    FCB $D3,$EC,0,0        ; path0: header (y=-45, x=-20, relative to center)
    FCB $FF,$46,$00          ; line 0: flag=-1, dy=70, dx=0
    FCB $FF,$00,$28          ; line 1: flag=-1, dy=0, dx=40
    FCB $FF,$BA,$00          ; line 2: flag=-1, dy=-70, dx=0
    FCB 2                ; End marker (path complete)

_LONDON_BG_PATH1:
    FCB 127              ; path1: intensity
    FCB $23,$F1,0,0        ; path1: header (y=35, x=-15, relative to center)
    FCB $FF,$0A,$00          ; line 0: flag=-1, dy=10, dx=0
    FCB $FF,$00,$1E          ; line 1: flag=-1, dy=0, dx=30
    FCB $FF,$F6,$00          ; line 2: flag=-1, dy=-10, dx=0
    FCB $FF,$00,$E2          ; line 3: flag=-1, dy=0, dx=-30
    FCB 2                ; End marker (path complete)

_LONDON_BG_PATH2:
    FCB 100              ; path2: intensity
    FCB $28,$00,0,0        ; path2: header (y=40, x=0, relative to center)
    FCB $FF,$05,$00          ; line 0: flag=-1, dy=5, dx=0
    FCB $FF,$FB,$08          ; line 1: flag=-1, dy=-5, dx=8
    FCB 2                ; End marker (path complete)

_LONDON_BG_PATH3:
    FCB 120              ; path3: intensity
    FCB $19,$EC,0,0        ; path3: header (y=25, x=-20, relative to center)
    FCB $FF,$0A,$05          ; line 0: flag=-1, dy=10, dx=5
    FCB $FF,$00,$1E          ; line 1: flag=-1, dy=0, dx=30
    FCB $FF,$F6,$05          ; line 2: flag=-1, dy=-10, dx=5
    FCB 2                ; End marker (path complete)

; Vector asset: leningrad_bg
; Generated from leningrad_bg.vec (Malban Draw_Sync_List format)
; Total paths: 5, points: 21
; X bounds: min=-30, max=30, width=60
; Center: (0, 30)

_LENINGRAD_BG_WIDTH EQU 60
_LENINGRAD_BG_CENTER_X EQU 0
_LENINGRAD_BG_CENTER_Y EQU 30

_LENINGRAD_BG_VECTORS:  ; Main entry
_LENINGRAD_BG_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $05,$E7,0,0        ; path0: header (y=5, x=-25, relative to center)
    FCB $FF,$14,$0A          ; line 0: flag=-1, dy=20, dx=10
    FCB $FF,$05,$0F          ; line 1: flag=-1, dy=5, dx=15
    FCB $FF,$FB,$0F          ; line 2: flag=-1, dy=-5, dx=15
    FCB $FF,$EC,$0A          ; line 3: flag=-1, dy=-20, dx=10
    FCB 2                ; End marker (path complete)

_LENINGRAD_BG_PATH1:
    FCB 127              ; path1: intensity
    FCB $1E,$00,0,0        ; path1: header (y=30, x=0, relative to center)
    FCB $FF,$0A,$00          ; line 0: flag=-1, dy=10, dx=0
    FCB 2                ; End marker (path complete)

_LENINGRAD_BG_PATH2:
    FCB 110              ; path2: intensity
    FCB $05,$E2,0,0        ; path2: header (y=5, x=-30, relative to center)
    FCB $FF,$D3,$00          ; line 0: flag=-1, dy=-45, dx=0
    FCB $FF,$00,$3C          ; line 1: flag=-1, dy=0, dx=60
    FCB $FF,$2D,$00          ; line 2: flag=-1, dy=45, dx=0
    FCB 2                ; End marker (path complete)

_LENINGRAD_BG_PATH3:
    FCB 90              ; path3: intensity
    FCB $EC,$EC,0,0        ; path3: header (y=-20, x=-20, relative to center)
    FCB $FF,$0F,$00          ; line 0: flag=-1, dy=15, dx=0
    FCB $FF,$00,$0A          ; line 1: flag=-1, dy=0, dx=10
    FCB $FF,$F1,$00          ; line 2: flag=-1, dy=-15, dx=0
    FCB $FF,$00,$F6          ; line 3: flag=-1, dy=0, dx=-10
    FCB 2                ; End marker (path complete)

_LENINGRAD_BG_PATH4:
    FCB 90              ; path4: intensity
    FCB $EC,$0A,0,0        ; path4: header (y=-20, x=10, relative to center)
    FCB $FF,$0F,$00          ; line 0: flag=-1, dy=15, dx=0
    FCB $FF,$00,$0A          ; line 1: flag=-1, dy=0, dx=10
    FCB $FF,$F1,$00          ; line 2: flag=-1, dy=-15, dx=0
    FCB $FF,$00,$F6          ; line 3: flag=-1, dy=0, dx=-10
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

; Vector asset: ayers_bg
; Generated from ayers_bg.vec (Malban Draw_Sync_List format)
; Total paths: 3, points: 13
; X bounds: min=-90, max=90, width=180
; Center: (0, 10)

_AYERS_BG_WIDTH EQU 180
_AYERS_BG_CENTER_X EQU 0
_AYERS_BG_CENTER_Y EQU 10

_AYERS_BG_VECTORS:  ; Main entry
_AYERS_BG_PATH0:    ; Path 0
    FCB 110              ; path0: intensity
    FCB $D8,$A6,0,0        ; path0: header (y=-40, x=-90, relative to center)
    FCB $FF,$32,$14          ; line 0: flag=-1, dy=50, dx=20
    FCB $FF,$19,$1E          ; line 1: flag=-1, dy=25, dx=30
    FCB $FF,$05,$28          ; line 2: flag=-1, dy=5, dx=40
    FCB $FF,$FB,$28          ; line 3: flag=-1, dy=-5, dx=40
    FCB $FF,$E7,$1E          ; line 4: flag=-1, dy=-25, dx=30
    FCB $FF,$CE,$14          ; line 5: flag=-1, dy=-50, dx=20
    FCB 2                ; End marker (path complete)

_AYERS_BG_PATH1:
    FCB 80              ; path1: intensity
    FCB $00,$CE,0,0        ; path1: header (y=0, x=-50, relative to center)
    FCB $FF,$0F,$14          ; line 0: flag=-1, dy=15, dx=20
    FCB $FF,$05,$1E          ; line 1: flag=-1, dy=5, dx=30
    FCB 2                ; End marker (path complete)

_AYERS_BG_PATH2:
    FCB 80              ; path2: intensity
    FCB $14,$00,0,0        ; path2: header (y=20, x=0, relative to center)
    FCB $FF,$FB,$1E          ; line 0: flag=-1, dy=-5, dx=30
    FCB $FF,$F1,$14          ; line 1: flag=-1, dy=-15, dx=20
    FCB 2                ; End marker (path complete)

; Vector asset: fuji_bg
; Generated from fuji_bg.vec (Malban Draw_Sync_List format)
; Total paths: 6, points: 65
; X bounds: min=-125, max=125, width=250
; Center: (0, 0)

_FUJI_BG_WIDTH EQU 250
_FUJI_BG_CENTER_X EQU 0
_FUJI_BG_CENTER_Y EQU 0

_FUJI_BG_VECTORS:  ; Main entry
_FUJI_BG_PATH0:    ; Path 0
_FUJI_BG_PATH1:
    FCB 127              ; path1: intensity
    FCB $E8,$84,0,0        ; path1: header (y=-24, x=-124, relative to center)
    FCB $FF,$0A,$1E          ; line 0: flag=-1, dy=10, dx=30
    FCB $FF,$0E,$1E          ; line 1: flag=-1, dy=14, dx=30
    FCB $FF,$0F,$15          ; line 2: flag=-1, dy=15, dx=21
    FCB $FF,$11,$17          ; line 3: flag=-1, dy=17, dx=23
    FCB $FF,$0E,$0E          ; line 4: flag=-1, dy=14, dx=14
    FCB $FF,$FE,$03          ; line 5: flag=-1, dy=-2, dx=3
    FCB $FF,$03,$04          ; line 6: flag=-1, dy=3, dx=4
    FCB $FF,$FE,$04          ; line 7: flag=-1, dy=-2, dx=4
    FCB $FF,$01,$07          ; line 8: flag=-1, dy=1, dx=7
    FCB $FF,$02,$04          ; line 9: flag=-1, dy=2, dx=4
    FCB $FF,$FD,$06          ; line 10: flag=-1, dy=-3, dx=6
    FCB $FF,$03,$03          ; line 11: flag=-1, dy=3, dx=3
    FCB $FF,$EB,$11          ; line 12: flag=-1, dy=-21, dx=17
    FCB $FF,$F4,$11          ; line 13: flag=-1, dy=-12, dx=17
    FCB $FF,$F0,$16          ; line 14: flag=-1, dy=-16, dx=22
    FCB $FF,$F6,$14          ; line 15: flag=-1, dy=-10, dx=20
    FCB $FF,$F6,$18          ; line 16: flag=-1, dy=-10, dx=24
    FCB $FF,$00,$00          ; line 17: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_FUJI_BG_PATH2:
    FCB 127              ; path2: intensity
    FCB $1A,$F1,0,0        ; path2: header (y=26, x=-15, relative to center)
    FCB $FF,$06,$03          ; line 0: flag=-1, dy=6, dx=3
    FCB $FF,$04,$03          ; line 1: flag=-1, dy=4, dx=3
    FCB $FF,$FD,$04          ; line 2: flag=-1, dy=-3, dx=4
    FCB $FF,$FC,$FC          ; line 3: flag=-1, dy=-4, dx=-4
    FCB $FF,$FD,$FA          ; line 4: flag=-1, dy=-3, dx=-6
    FCB $FF,$00,$00          ; line 5: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_FUJI_BG_PATH3:
    FCB 127              ; path3: intensity
    FCB $1F,$07,0,0        ; path3: header (y=31, x=7, relative to center)
    FCB $FF,$F9,$FD          ; line 0: flag=-1, dy=-7, dx=-3
    FCB $FF,$FA,$02          ; line 1: flag=-1, dy=-6, dx=2
    FCB $FF,$F9,$FD          ; line 2: flag=-1, dy=-7, dx=-3
    FCB $FF,$FD,$04          ; line 3: flag=-1, dy=-3, dx=4
    FCB $FF,$08,$03          ; line 4: flag=-1, dy=8, dx=3
    FCB $FF,$07,$FE          ; line 5: flag=-1, dy=7, dx=-2
    FCB $FF,$06,$01          ; line 6: flag=-1, dy=6, dx=1
    FCB $FF,$02,$FE          ; line 7: flag=-1, dy=2, dx=-2
    FCB 2                ; End marker (path complete)

_FUJI_BG_PATH4:
    FCB 127              ; path4: intensity
    FCB $21,$18,0,0        ; path4: header (y=33, x=24, relative to center)
    FCB $FF,$F7,$05          ; line 0: flag=-1, dy=-9, dx=5
    FCB $FF,$F7,$0C          ; line 1: flag=-1, dy=-9, dx=12
    FCB $FF,$0B,$FA          ; line 2: flag=-1, dy=11, dx=-6
    FCB $FF,$07,$F5          ; line 3: flag=-1, dy=7, dx=-11
    FCB 2                ; End marker (path complete)

_FUJI_BG_PATH5:
    FCB 127              ; path5: intensity
    FCB $05,$C7,0,0        ; path5: header (y=5, x=-57, relative to center)
    FCB $FF,$09,$1A          ; line 0: flag=-1, dy=9, dx=26
    FCB $FF,$EF,$F2          ; line 1: flag=-1, dy=-17, dx=-14
    FCB $FF,$1B,$22          ; line 2: flag=-1, dy=27, dx=34
    FCB $FF,$F2,$FB          ; line 3: flag=-1, dy=-14, dx=-5
    FCB $FF,$00,$03          ; line 4: flag=-1, dy=0, dx=3
    FCB $FF,$F7,$FB          ; line 5: flag=-1, dy=-9, dx=-5
    FCB $FF,$FA,$01          ; line 6: flag=-1, dy=-6, dx=1
    FCB $FF,$0E,$0E          ; line 7: flag=-1, dy=14, dx=14
    FCB $FF,$F1,$00          ; line 8: flag=-1, dy=-15, dx=0
    FCB $FF,$0A,$05          ; line 9: flag=-1, dy=10, dx=5
    FCB $FF,$EA,$06          ; line 10: flag=-1, dy=-22, dx=6
    FCB $FF,$1C,$05          ; line 11: flag=-1, dy=28, dx=5
    FCB $FF,$EF,$06          ; line 12: flag=-1, dy=-17, dx=6
    FCB $FF,$03,$01          ; line 13: flag=-1, dy=3, dx=1
    FCB $FF,$FD,$04          ; line 14: flag=-1, dy=-3, dx=4
    FCB $FF,$0B,$03          ; line 15: flag=-1, dy=11, dx=3
    FCB $FF,$F5,$05          ; line 16: flag=-1, dy=-11, dx=5
    FCB $FF,$10,$FF          ; line 17: flag=-1, dy=16, dx=-1
    FCB $FF,$EE,$13          ; line 18: flag=-1, dy=-18, dx=19
    FCB $FF,$12,$F7          ; line 19: flag=-1, dy=18, dx=-9
    FCB $FF,$F9,$0E          ; line 20: flag=-1, dy=-7, dx=14
    FCB $FF,$04,$02          ; line 21: flag=-1, dy=4, dx=2
    FCB $FF,$FC,$14          ; line 22: flag=-1, dy=-4, dx=20
    FCB 2                ; End marker (path complete)

; Vector asset: kilimanjaro_bg
; Generated from kilimanjaro_bg.vec (Malban Draw_Sync_List format)
; Total paths: 4, points: 13
; X bounds: min=-100, max=100, width=200
; Center: (0, 12)

_KILIMANJARO_BG_WIDTH EQU 200
_KILIMANJARO_BG_CENTER_X EQU 0
_KILIMANJARO_BG_CENTER_Y EQU 12

_KILIMANJARO_BG_VECTORS:  ; Main entry
_KILIMANJARO_BG_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $D6,$9C,0,0        ; path0: header (y=-42, x=-100, relative to center)
    FCB $FF,$3C,$32          ; line 0: flag=-1, dy=60, dx=50
    FCB $FF,$19,$32          ; line 1: flag=-1, dy=25, dx=50
    FCB $FF,$E7,$32          ; line 2: flag=-1, dy=-25, dx=50
    FCB $FF,$C4,$32          ; line 3: flag=-1, dy=-60, dx=50
    FCB 2                ; End marker (path complete)

_KILIMANJARO_BG_PATH1:
    FCB 110              ; path1: intensity
    FCB $1C,$E2,0,0        ; path1: header (y=28, x=-30, relative to center)
    FCB $FF,$0F,$1E          ; line 0: flag=-1, dy=15, dx=30
    FCB $FF,$F1,$00          ; line 1: flag=-1, dy=-15, dx=0
    FCB 2                ; End marker (path complete)

_KILIMANJARO_BG_PATH2:
    FCB 110              ; path2: intensity
    FCB $1C,$00,0,0        ; path2: header (y=28, x=0, relative to center)
    FCB $FF,$0F,$00          ; line 0: flag=-1, dy=15, dx=0
    FCB $FF,$F1,$1E          ; line 1: flag=-1, dy=-15, dx=30
    FCB 2                ; End marker (path complete)

_KILIMANJARO_BG_PATH3:
    FCB 90              ; path3: intensity
    FCB $F4,$BA,0,0        ; path3: header (y=-12, x=-70, relative to center)
    FCB $FF,$14,$1E          ; line 0: flag=-1, dy=20, dx=30
    FCB 2                ; End marker (path complete)

; Vector asset: athens_bg
; Generated from athens_bg.vec (Malban Draw_Sync_List format)
; Total paths: 7, points: 15
; X bounds: min=-80, max=80, width=160
; Center: (0, 22)

_ATHENS_BG_WIDTH EQU 160
_ATHENS_BG_CENTER_X EQU 0
_ATHENS_BG_CENTER_Y EQU 22

_ATHENS_BG_VECTORS:  ; Main entry
_ATHENS_BG_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $12,$B0,0,0        ; path0: header (y=18, x=-80, relative to center)
    FCB $FF,$0F,$50          ; line 0: flag=-1, dy=15, dx=80
    FCB $FF,$F1,$50          ; line 1: flag=-1, dy=-15, dx=80
    FCB 2                ; End marker (path complete)

_ATHENS_BG_PATH1:
    FCB 110              ; path1: intensity
    FCB $12,$BA,0,0        ; path1: header (y=18, x=-70, relative to center)
    FCB $FF,$CE,$00          ; line 0: flag=-1, dy=-50, dx=0
    FCB 2                ; End marker (path complete)

_ATHENS_BG_PATH2:
    FCB 110              ; path2: intensity
    FCB $12,$D8,0,0        ; path2: header (y=18, x=-40, relative to center)
    FCB $FF,$CE,$00          ; line 0: flag=-1, dy=-50, dx=0
    FCB 2                ; End marker (path complete)

_ATHENS_BG_PATH3:
    FCB 110              ; path3: intensity
    FCB $12,$F6,0,0        ; path3: header (y=18, x=-10, relative to center)
    FCB $FF,$CE,$00          ; line 0: flag=-1, dy=-50, dx=0
    FCB 2                ; End marker (path complete)

_ATHENS_BG_PATH4:
    FCB 110              ; path4: intensity
    FCB $12,$14,0,0        ; path4: header (y=18, x=20, relative to center)
    FCB $FF,$CE,$00          ; line 0: flag=-1, dy=-50, dx=0
    FCB 2                ; End marker (path complete)

_ATHENS_BG_PATH5:
    FCB 110              ; path5: intensity
    FCB $12,$32,0,0        ; path5: header (y=18, x=50, relative to center)
    FCB $FF,$CE,$00          ; line 0: flag=-1, dy=-50, dx=0
    FCB 2                ; End marker (path complete)

_ATHENS_BG_PATH6:
    FCB 100              ; path6: intensity
    FCB $E0,$B0,0,0        ; path6: header (y=-32, x=-80, relative to center)
    FCB $FF,$00,$7F          ; line 0: flag=-1, dy=0, dx=127
    FCB 2                ; End marker (path complete)

; Vector asset: antarctica_bg
; Generated from antarctica_bg.vec (Malban Draw_Sync_List format)
; Total paths: 4, points: 12
; X bounds: min=-120, max=120, width=240
; Center: (0, 15)

_ANTARCTICA_BG_WIDTH EQU 240
_ANTARCTICA_BG_CENTER_X EQU 0
_ANTARCTICA_BG_CENTER_Y EQU 15

_ANTARCTICA_BG_VECTORS:  ; Main entry
_ANTARCTICA_BG_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $DD,$B0,0,0        ; path0: header (y=-35, x=-80, relative to center)
    FCB $FF,$3C,$14          ; line 0: flag=-1, dy=60, dx=20
    FCB $FF,$C4,$14          ; line 1: flag=-1, dy=-60, dx=20
    FCB 2                ; End marker (path complete)

_ANTARCTICA_BG_PATH1:
    FCB 110              ; path1: intensity
    FCB $DD,$E2,0,0        ; path1: header (y=-35, x=-30, relative to center)
    FCB $FF,$46,$14          ; line 0: flag=-1, dy=70, dx=20
    FCB $FF,$00,$14          ; line 1: flag=-1, dy=0, dx=20
    FCB $FF,$BA,$14          ; line 2: flag=-1, dy=-70, dx=20
    FCB 2                ; End marker (path complete)

_ANTARCTICA_BG_PATH2:
    FCB 100              ; path2: intensity
    FCB $DD,$28,0,0        ; path2: header (y=-35, x=40, relative to center)
    FCB $FF,$37,$14          ; line 0: flag=-1, dy=55, dx=20
    FCB $FF,$C9,$14          ; line 1: flag=-1, dy=-55, dx=20
    FCB 2                ; End marker (path complete)

_ANTARCTICA_BG_PATH3:
    FCB 80              ; path3: intensity
    FCB $DD,$88,0,0        ; path3: header (y=-35, x=-120, relative to center)
    FCB $FF,$00,$7F          ; line 0: flag=-1, dy=0, dx=127
    FCB 2                ; End marker (path complete)

; Vector asset: bubble_medium
; Generated from bubble_medium.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 24
; X bounds: min=-15, max=15, width=30
; Center: (0, 0)

_BUBBLE_MEDIUM_WIDTH EQU 30
_BUBBLE_MEDIUM_CENTER_X EQU 0
_BUBBLE_MEDIUM_CENTER_Y EQU 0

_BUBBLE_MEDIUM_VECTORS:  ; Main entry
_BUBBLE_MEDIUM_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $00,$0F,0,0        ; path0: header (y=0, x=15, relative to center)
    FCB $FF,$04,$FF          ; line 0: flag=-1, dy=4, dx=-1
    FCB $FF,$04,$FF          ; line 1: flag=-1, dy=4, dx=-1
    FCB $FF,$03,$FE          ; line 2: flag=-1, dy=3, dx=-2
    FCB $FF,$02,$FD          ; line 3: flag=-1, dy=2, dx=-3
    FCB $FF,$01,$FC          ; line 4: flag=-1, dy=1, dx=-4
    FCB $FF,$01,$FC          ; line 5: flag=-1, dy=1, dx=-4
    FCB $FF,$FF,$FC          ; line 6: flag=-1, dy=-1, dx=-4
    FCB $FF,$FF,$FC          ; line 7: flag=-1, dy=-1, dx=-4
    FCB $FF,$FE,$FD          ; line 8: flag=-1, dy=-2, dx=-3
    FCB $FF,$FD,$FE          ; line 9: flag=-1, dy=-3, dx=-2
    FCB $FF,$FC,$FF          ; line 10: flag=-1, dy=-4, dx=-1
    FCB $FF,$FC,$FF          ; line 11: flag=-1, dy=-4, dx=-1
    FCB $FF,$FC,$01          ; line 12: flag=-1, dy=-4, dx=1
    FCB $FF,$FC,$01          ; line 13: flag=-1, dy=-4, dx=1
    FCB $FF,$FD,$02          ; line 14: flag=-1, dy=-3, dx=2
    FCB $FF,$FE,$03          ; line 15: flag=-1, dy=-2, dx=3
    FCB $FF,$FF,$04          ; line 16: flag=-1, dy=-1, dx=4
    FCB $FF,$FF,$04          ; line 17: flag=-1, dy=-1, dx=4
    FCB $FF,$01,$04          ; line 18: flag=-1, dy=1, dx=4
    FCB $FF,$01,$04          ; line 19: flag=-1, dy=1, dx=4
    FCB $FF,$02,$03          ; line 20: flag=-1, dy=2, dx=3
    FCB $FF,$03,$02          ; line 21: flag=-1, dy=3, dx=2
    FCB $FF,$04,$01          ; line 22: flag=-1, dy=4, dx=1
    FCB $FF,$04,$01          ; closing line: flag=-1, dy=4, dx=1
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


; Array literal for variable 'joystick1_state' (6 elements)
ARRAY_0:
    FDB 0   ; Element 0
    FDB 0   ; Element 1
    FDB 0   ; Element 2
    FDB 0   ; Element 3
    FDB 0   ; Element 4
    FDB 0   ; Element 5

; Array literal for variable 'enemy_active' (8 elements)
ARRAY_1:
    FDB 0   ; Element 0
    FDB 0   ; Element 1
    FDB 0   ; Element 2
    FDB 0   ; Element 3
    FDB 0   ; Element 4
    FDB 0   ; Element 5
    FDB 0   ; Element 6
    FDB 0   ; Element 7

; Array literal for variable 'enemy_x' (8 elements)
ARRAY_2:
    FDB 0   ; Element 0
    FDB 0   ; Element 1
    FDB 0   ; Element 2
    FDB 0   ; Element 3
    FDB 0   ; Element 4
    FDB 0   ; Element 5
    FDB 0   ; Element 6
    FDB 0   ; Element 7

; Array literal for variable 'enemy_y' (8 elements)
ARRAY_3:
    FDB 0   ; Element 0
    FDB 0   ; Element 1
    FDB 0   ; Element 2
    FDB 0   ; Element 3
    FDB 0   ; Element 4
    FDB 0   ; Element 5
    FDB 0   ; Element 6
    FDB 0   ; Element 7

; Array literal for variable 'enemy_vx' (8 elements)
ARRAY_4:
    FDB 0   ; Element 0
    FDB 0   ; Element 1
    FDB 0   ; Element 2
    FDB 0   ; Element 3
    FDB 0   ; Element 4
    FDB 0   ; Element 5
    FDB 0   ; Element 6
    FDB 0   ; Element 7

; Array literal for variable 'enemy_vy' (8 elements)
ARRAY_5:
    FDB 0   ; Element 0
    FDB 0   ; Element 1
    FDB 0   ; Element 2
    FDB 0   ; Element 3
    FDB 0   ; Element 4
    FDB 0   ; Element 5
    FDB 0   ; Element 6
    FDB 0   ; Element 7

; Array literal for variable 'enemy_size' (8 elements)
ARRAY_6:
    FDB 0   ; Element 0
    FDB 0   ; Element 1
    FDB 0   ; Element 2
    FDB 0   ; Element 3
    FDB 0   ; Element 4
    FDB 0   ; Element 5
    FDB 0   ; Element 6
    FDB 0   ; Element 7

; VPy_LINE:19
; Const array literal for 'location_x_coords' (17 elements)
CONST_ARRAY_0:
    FDB 40   ; Element 0
    FDB 40   ; Element 1
    FDB -40   ; Element 2
    FDB -10   ; Element 3
    FDB 20   ; Element 4
    FDB 50   ; Element 5
    FDB 80   ; Element 6
    FDB -85   ; Element 7
    FDB -50   ; Element 8
    FDB -15   ; Element 9
    FDB 15   ; Element 10
    FDB 50   ; Element 11
    FDB 85   ; Element 12
    FDB -90   ; Element 13
    FDB -45   ; Element 14
    FDB 0   ; Element 15
    FDB 45   ; Element 16

; VPy_LINE:20
; Const array literal for 'location_y_coords' (17 elements)
CONST_ARRAY_1:
    FDB 110   ; Element 0
    FDB 79   ; Element 1
    FDB -20   ; Element 2
    FDB 10   ; Element 3
    FDB 40   ; Element 4
    FDB 70   ; Element 5
    FDB 100   ; Element 6
    FDB -40   ; Element 7
    FDB -10   ; Element 8
    FDB 30   ; Element 9
    FDB 60   ; Element 10
    FDB 90   ; Element 11
    FDB 20   ; Element 12
    FDB 50   ; Element 13
    FDB 0   ; Element 14
    FDB -60   ; Element 15
    FDB -30   ; Element 16

; VPy_LINE:21
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

; VPy_LINE:24
; Const string array for 'level_backgrounds' (17 strings)
CONST_ARRAY_3_STR_0:
    FCC "FUJI_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_1:
    FCC "KEIRIN_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_2:
    FCC "BUDDHA_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_3:
    FCC "ANGKOR_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_4:
    FCC "AYERS_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_5:
    FCC "TAJ_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_6:
    FCC "LENINGRAD_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_7:
    FCC "PARIS_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_8:
    FCC "LONDON_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_9:
    FCC "BARCELONA_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_10:
    FCC "ATHENS_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_11:
    FCC "PYRAMIDS_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_12:
    FCC "KILIMANJARO_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_13:
    FCC "NEWYORK_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_14:
    FCC "MAYAN_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_15:
    FCC "ANTARCTICA_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3_STR_16:
    FCC "EASTER_BG"
    FCB $80   ; String terminator
CONST_ARRAY_3:  ; Pointer table for level_backgrounds
    FDB CONST_ARRAY_3_STR_0  ; Pointer to string
    FDB CONST_ARRAY_3_STR_1  ; Pointer to string
    FDB CONST_ARRAY_3_STR_2  ; Pointer to string
    FDB CONST_ARRAY_3_STR_3  ; Pointer to string
    FDB CONST_ARRAY_3_STR_4  ; Pointer to string
    FDB CONST_ARRAY_3_STR_5  ; Pointer to string
    FDB CONST_ARRAY_3_STR_6  ; Pointer to string
    FDB CONST_ARRAY_3_STR_7  ; Pointer to string
    FDB CONST_ARRAY_3_STR_8  ; Pointer to string
    FDB CONST_ARRAY_3_STR_9  ; Pointer to string
    FDB CONST_ARRAY_3_STR_10  ; Pointer to string
    FDB CONST_ARRAY_3_STR_11  ; Pointer to string
    FDB CONST_ARRAY_3_STR_12  ; Pointer to string
    FDB CONST_ARRAY_3_STR_13  ; Pointer to string
    FDB CONST_ARRAY_3_STR_14  ; Pointer to string
    FDB CONST_ARRAY_3_STR_15  ; Pointer to string
    FDB CONST_ARRAY_3_STR_16  ; Pointer to string

; VPy_LINE:26
; Const array literal for 'level_enemy_count' (17 elements)
CONST_ARRAY_4:
    FDB 1   ; Element 0
    FDB 1   ; Element 1
    FDB 2   ; Element 2
    FDB 2   ; Element 3
    FDB 2   ; Element 4
    FDB 3   ; Element 5
    FDB 3   ; Element 6
    FDB 3   ; Element 7
    FDB 4   ; Element 8
    FDB 4   ; Element 9
    FDB 4   ; Element 10
    FDB 5   ; Element 11
    FDB 5   ; Element 12
    FDB 5   ; Element 13
    FDB 6   ; Element 14
    FDB 6   ; Element 15
    FDB 7   ; Element 16

; VPy_LINE:27
; Const array literal for 'level_enemy_speed' (17 elements)
CONST_ARRAY_5:
    FDB 1   ; Element 0
    FDB 1   ; Element 1
    FDB 1   ; Element 2
    FDB 2   ; Element 3
    FDB 2   ; Element 4
    FDB 2   ; Element 5
    FDB 2   ; Element 6
    FDB 3   ; Element 7
    FDB 3   ; Element 8
    FDB 3   ; Element 9
    FDB 3   ; Element 10
    FDB 4   ; Element 11
    FDB 4   ; Element 12
    FDB 4   ; Element 13
    FDB 4   ; Element 14
    FDB 5   ; Element 15
    FDB 5   ; Element 16

; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "ANGKOR WAT (KH)"
    FCB $80
STR_1:
    FCC "ANTARCTICA (AQ)"
    FCB $80
STR_2:
    FCC "ATHENS (GR)"
    FCB $80
STR_3:
    FCC "AYERS ROCK (AU)"
    FCB $80
STR_4:
    FCC "BARCELONA (ES)"
    FCB $80
STR_5:
    FCC "EASTER ISLAND (CL)"
    FCB $80
STR_6:
    FCC "EMERALD BUDDHA TEMPLE (TH)"
    FCB $80
STR_7:
    FCC "GET READY"
    FCB $80
STR_8:
    FCC "LENINGRAD (RU)"
    FCB $80
STR_9:
    FCC "LONDON (UK)"
    FCB $80
STR_10:
    FCC "MAYAN RUINS (MX)"
    FCB $80
STR_11:
    FCC "MOUNT FUJI (JP)"
    FCB $80
STR_12:
    FCC "MOUNT KEIRIN (CN)"
    FCB $80
STR_13:
    FCC "MOUNT KILIMANJARO (TZ)"
    FCB $80
STR_14:
    FCC "NEW YORK (US)"
    FCB $80
STR_15:
    FCC "PARIS (FR)"
    FCB $80
STR_16:
    FCC "PRESS A BUTTON"
    FCB $80
STR_17:
    FCC "PYRAMIDS (EG)"
    FCB $80
STR_18:
    FCC "TAJ MAHAL (IN)"
    FCB $80
STR_19:
    FCC "TO START"
    FCB $80
VLINE_DX_16 EQU RESULT+10
VLINE_DY_16 EQU RESULT+12
VLINE_DX EQU RESULT+14
VLINE_DY EQU RESULT+15
VLINE_DY_REMAINING EQU RESULT+16
VLINE_STEPS EQU RESULT+18
VLINE_LIST EQU RESULT+19
DRAW_VEC_X EQU RESULT+21
DRAW_VEC_Y EQU RESULT+22
MIRROR_X EQU RESULT+23
MIRROR_Y EQU RESULT+24
DRAW_VEC_INTENSITY EQU RESULT+25
DRAW_CIRCLE_XC EQU RESULT+26
DRAW_CIRCLE_YC EQU RESULT+27
DRAW_CIRCLE_DIAM EQU RESULT+28
DRAW_CIRCLE_INTENSITY EQU RESULT+29
DRAW_CIRCLE_TEMP EQU RESULT+30
