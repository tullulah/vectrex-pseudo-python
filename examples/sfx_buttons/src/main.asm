; --- Motorola 6809 backend (Vectrex) title='SFX + MUSIC' origin=$0000 ---
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
    FCC "SFX   MUSIC"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 35 bytes
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$02   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$04   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$06   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$08   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0A   ; Pointer temp (2 bytes)
TMPPTR2              EQU $C880+$0C   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0E   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$10   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$11   ; Temporary y storage (1 bytes)
PSG_MUSIC_PTR        EQU $C880+$12   ; Current music position pointer (2 bytes)
PSG_MUSIC_START      EQU $C880+$14   ; Music start pointer (for loops) (2 bytes)
PSG_IS_PLAYING       EQU $C880+$16   ; Playing flag ($00=stopped, $01=playing) (1 bytes)
PSG_MUSIC_ACTIVE     EQU $C880+$17   ; Set during UPDATE_MUSIC_PSG (1 bytes)
PSG_FRAME_COUNT      EQU $C880+$18   ; Frame register write count (1 bytes)
PSG_DELAY_FRAMES     EQU $C880+$19   ; Frames to wait before next read (1 bytes)
SFX_PTR              EQU $C880+$1A   ; Current SFX data pointer (2 bytes)
SFX_TICK             EQU $C880+$1C   ; Current frame counter (2 bytes)
SFX_ACTIVE           EQU $C880+$1E   ; Playback state ($00=stopped, $01=playing) (1 bytes)
SFX_PHASE            EQU $C880+$1F   ; Envelope phase (0=A,1=D,2=S,3=R) (1 bytes)
SFX_VOL              EQU $C880+$20   ; Current volume level (0-15) (1 bytes)
NUM_STR              EQU $C880+$21   ; String buffer for PRINT_NUMBER (2 bytes)
PSG_MUSIC_PTR_DP   EQU $12  ; DP-relative
PSG_MUSIC_START_DP EQU $14  ; DP-relative
PSG_IS_PLAYING_DP  EQU $16  ; DP-relative
PSG_MUSIC_ACTIVE_DP EQU $17  ; DP-relative
PSG_FRAME_COUNT_DP EQU $18  ; DP-relative
PSG_DELAY_FRAMES_DP EQU $19  ; DP-relative

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
    CLR >$C80F   ; Clear Vec_Btn_State before reading (fix stale buttons on hardware)
    JSR $F1BA    ; Read_Btns
    LDA >$C80F   ; Vec_Btn_State
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
    CLR >$C80F   ; Clear Vec_Btn_State before reading (fix stale buttons on hardware)
    JSR $F1BA    ; Read_Btns
    LDA >$C80F   ; Vec_Btn_State
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
    CLR >$C80F   ; Clear Vec_Btn_State before reading (fix stale buttons on hardware)
    JSR $F1BA    ; Read_Btns
    LDA >$C80F   ; Vec_Btn_State
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
    CLR >$C80F   ; Clear Vec_Btn_State before reading (fix stale buttons on hardware)
    JSR $F1BA    ; Read_Btns
    LDA >$C80F   ; Vec_Btn_State
    ANDA #$08
    BEQ .J1B4_OFF
    JSR $F1AF    ; DP_to_C8 (restore before return)
    LDD #1
    RTS
.J1B4_OFF:
    JSR $F1AF    ; DP_to_C8 (restore before return)
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

; RAM variables (RESULT offsets - saves RAM)
sfx_pointer EQU RESULT+32    ; 2 bytes - Current AYFX frame pointer
sfx_status  EQU RESULT+34    ; 1 byte  - Active flag (0=inactive, 1=active)

AUDIO_UPDATE:
PSHS DP                 ; Save current DP
LDA #$D0                ; Set DP=$D0 (Sound_Byte REQUIRES this for VIA access)
TFR A,DP

; UPDATE MUSIC (channel B: registers 9, 11-14)
LDA >PSG_IS_PLAYING     ; Check if music is playing (EXTENDED addressing)
BEQ AU_SKIP_MUSIC       ; Skip if not

; Check delay counter first
LDA >PSG_DELAY_FRAMES   ; Load delay counter (EXTENDED addressing)
BEQ AU_MUSIC_READ       ; If zero, read next frame data
DECA                    ; Decrement delay
STA >PSG_DELAY_FRAMES   ; Store back (EXTENDED addressing)
CMPA #0                 ; Check if it just reached zero
BNE AU_UPDATE_SFX       ; If not zero yet, skip this frame

; Delay just reached zero, X points to count byte already
LDX >PSG_MUSIC_PTR      ; Load music pointer (EXTENDED addressing)
BEQ AU_SKIP_MUSIC       ; Skip if null
BRA AU_MUSIC_READ_COUNT ; Skip delay read, go straight to count

AU_MUSIC_READ:
LDX >PSG_MUSIC_PTR      ; Load music pointer (EXTENDED addressing)
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
STB >PSG_DELAY_FRAMES   ; Save delay counter (EXTENDED addressing)
STX >PSG_MUSIC_PTR      ; Save pointer (EXTENDED addressing)
BRA AU_UPDATE_SFX       ; Skip reading data this frame

AU_MUSIC_PROCESS_WRITES:
PSHS B                  ; Save count

; Mark that next time we should read delay, not count
; (This is implicit - after processing, X points to next delay byte)

AU_MUSIC_WRITE_LOOP:
LDA ,X+                 ; Load register number
LDB ,X+                 ; Load register value

; Check if SFX is active - skip volume C and preserve mixer C bits if so
PSHS A,B                ; Save reg number and value
LDA >sfx_status         ; Check SFX status (EXTENDED addressing)
BEQ AU_MUSIC_WRITE_CHECK_MIXER ; No SFX, check if mixer

; SFX active - check if writing to volume C (reg 10)
PULS B,A                ; Restore (B=value, A=reg)
CMPA #10                ; Volume C register?
BEQ AU_MUSIC_SKIP_REG   ; Skip this register (SFX controls it)

; Not volume C - check if mixer (reg 7)
CMPA #7                 ; Mixer register?
BNE AU_MUSIC_WRITE_NORMAL ; Not mixer, write normally

; SFX active + mixer write - merge (preserve SFX channel C bits)
PSHS A                  ; Save register number
LDA >$C807              ; Read current mixer shadow
ANDA #$24               ; Extract SFX bits: 00100100 (bits 2,5)
STA TMPPTR              ; Save SFX bits
ANDB #$DB               ; Clear SFX bits in music: 11011011
STB TMPPTR+1            ; Save masked music mixer
LDA TMPPTR              ; Load SFX bits
ORA TMPPTR+1            ; Merge with music
TFR A,B                 ; Result in B
PULS A                  ; Restore register number (7)
BRA AU_MUSIC_WRITE_NORMAL ; Write merged value

AU_MUSIC_WRITE_CHECK_MIXER:
; No SFX active - only check if mixer needs special handling (currently none)
PULS B,A                ; Restore original values
; Fall through to write normally

AU_MUSIC_SKIP_REG:
; Skip this register write entirely (restore pointer, continue loop)
PULS B                  ; Get counter
DECB                    ; Decrement
BEQ AU_MUSIC_DONE       ; Done if count=0
PSHS B                  ; Save counter
BRA AU_MUSIC_WRITE_LOOP ; Continue without writing

AU_MUSIC_WRITE_NORMAL:
PSHS X                  ; Save pointer
; Sound_Byte REQUIRES DP=$D0 (uses <VIA_port_a/b direct page addressing)
JSR Sound_Byte          ; Write to PSG using BIOS (DP=$D0 already set)
PULS X                  ; Restore pointer
PULS B                  ; Get counter
DECB                    ; Decrement
BEQ AU_MUSIC_DONE       ; Done if count=0
PSHS B                  ; Save counter
BRA AU_MUSIC_WRITE_LOOP ; Continue

AU_MUSIC_DONE:
STX >PSG_MUSIC_PTR      ; Update music pointer (EXTENDED addressing)
BRA AU_UPDATE_SFX       ; Now update SFX

AU_MUSIC_ENDED:
CLR >PSG_IS_PLAYING     ; Stop music (EXTENDED addressing)
BRA AU_UPDATE_SFX       ; Continue to SFX

AU_MUSIC_LOOP:
LDD ,X                  ; Load loop target
STD >PSG_MUSIC_PTR      ; Set music pointer to loop (EXTENDED addressing)
CLR >PSG_DELAY_FRAMES   ; Clear delay on loop (EXTENDED addressing)
BRA AU_UPDATE_SFX       ; Continue to SFX

AU_SKIP_MUSIC:
BRA AU_UPDATE_SFX       ; Skip music, go to SFX

; UPDATE SFX (channel C: registers 4/5=tone, 6=noise, 10=volume, 7=mixer)
AU_UPDATE_SFX:
PSHS DP                 ; Save DP=$D0
LDA #$C8                ; Switch to RESULT page
TFR A,DP
LDA <sfx_status         ; Check if SFX is active (direct page access)
PULS DP                 ; Restore DP=$D0
BEQ AU_DONE             ; Skip if not active

JSR sfx_doframe         ; Process one SFX frame (uses Sound_Byte internally)

AU_DONE:
CLR >$C80F              ; Clear Vec_Btn_State (EXTENDED addressing prevents overflow)
PULS DP                 ; Restore original DP
RTS

; ============================================================================
; AYFX SOUND EFFECTS PLAYER (Richard Chadd original system)
; ============================================================================
; Uses channel C (registers 4/5=tone, 6=noise, 10=volume, 7=mixer bit2/bit5)
; RAM variables: sfx_pointer (16-bit), sfx_status (8-bit)
; AYFX format: flag byte + optional data per frame, end marker $D0 $20
; Flag bits: 0-3=volume, 4=disable tone, 5=tone data present,
;            6=noise data present, 7=disable noise
; ============================================================================
; (RAM variables defined in AUDIO_UPDATE section above)

; PLAY_SFX_RUNTIME - Start SFX playback
; Input: X = pointer to AYFX data
; Note: Called from VPy code where DP=$C8
PLAY_SFX_RUNTIME:
STX sfx_pointer        ; Store pointer (direct page, DP=$C8)
LDA #$01
STA sfx_status         ; Mark as active
RTS

; SFX_UPDATE - Process one AYFX frame (call once per frame in loop)
SFX_UPDATE:
LDA sfx_status         ; Check if active (DP=$C8)
BEQ noay               ; Not active, skip
JSR sfx_doframe        ; Process one frame
noay:
RTS

; sfx_doframe - AYFX frame parser (Richard Chadd original)
; Called from AUDIO_UPDATE where DP=$D0, must switch to $C8
sfx_doframe:
PSHS DP                 ; Save DP=$D0
LDA #$C8                ; Switch to RESULT page
TFR A,DP
LDX sfx_pointer         ; Get current frame pointer (extended addressing)
TFR X,U                 ; Transfer to U
LDB ,U                  ; Read flag byte (NO auto-increment)
CMPB #$D0              ; Check end marker (first byte)
BNE sfx_checktonefreq  ; Not end, continue
LDB 1,U                ; Check second byte at offset 1
CMPB #$20              ; End marker $D0 $20?
BEQ sfx_endofeffect    ; Yes, stop

sfx_checktonefreq:
LEAY 1,U               ; Y = pointer to tone/noise data
LDB ,U                 ; Reload flag byte (Sound_Byte corrupts B)
BITB #$20              ; Bit 5: tone data present?
BEQ sfx_checknoisefreq ; No, skip tone
; Set tone frequency (channel C = reg 4/5)
LDB 2,U                ; Get LOW byte (fine tune)
LDA #$04               ; Register 4
JSR Sound_Byte         ; Write to PSG
LDB 1,U                ; Get HIGH byte (coarse tune)
LDA #$05               ; Register 5
JSR Sound_Byte         ; Write to PSG
LEAY 2,Y               ; Skip 2 tone bytes

sfx_checknoisefreq:
LDB ,U                 ; Reload flag byte
BITB #$40              ; Bit 6: noise data present?
BEQ sfx_checkvolume    ; No, skip noise
LDB ,Y                 ; Get noise period
LDA #$06               ; Register 6
JSR Sound_Byte         ; Write to PSG
LEAY 1,Y               ; Skip 1 noise byte

sfx_checkvolume:
LDB ,U                 ; Reload flag byte
ANDB #$0F              ; Get volume from bits 0-3
LDA #$0A               ; Register 10 (volume C)
JSR Sound_Byte         ; Write to PSG

sfx_checktonedisable:
LDB ,U                 ; Reload flag byte
BITB #$10              ; Bit 4: disable tone?
BEQ sfx_enabletone
sfx_disabletone:
LDB >$C807             ; Read mixer shadow (EXTENDED addressing)
ORB #$04               ; Set bit 2 (disable tone C)
LDA #$07               ; Register 7 (mixer)
JSR Sound_Byte         ; Write to PSG
BRA sfx_checknoisedisable  ; Continue to noise check

sfx_enabletone:
LDB >$C807             ; Read mixer shadow (EXTENDED addressing)
ANDB #$FB              ; Clear bit 2 (enable tone C)
LDA #$07               ; Register 7 (mixer)
JSR Sound_Byte         ; Write to PSG

sfx_checknoisedisable:
LDB ,U                 ; Reload flag byte
BITB #$80              ; Bit 7: disable noise?
BEQ sfx_enablenoise
sfx_disablenoise:
LDB >$C807             ; Read mixer shadow (EXTENDED addressing)
ORB #$20               ; Set bit 5 (disable noise C)
LDA #$07               ; Register 7 (mixer)
JSR Sound_Byte         ; Write to PSG
BRA sfx_nextframe      ; Done, update pointer

sfx_enablenoise:
LDB >$C807             ; Read mixer shadow (EXTENDED addressing)
ANDB #$DF              ; Clear bit 5 (enable noise C)
LDA #$07               ; Register 7 (mixer)
JSR Sound_Byte         ; Write to PSG

sfx_nextframe:
STY sfx_pointer         ; Update pointer for next frame (extended addressing)
PULS DP                 ; Restore DP=$D0
RTS

sfx_endofeffect:
; Stop SFX - set volume to 0
CLR <sfx_status         ; Mark as inactive (direct page OK for 8-bit)
PULS DP                 ; Restore DP=$D0 before BIOS call
LDA #$0A                ; Register 10 (volume C)
LDB #$00                ; Volume = 0
JSR Sound_Byte
PSHS DP                 ; Save again to clear pointer
LDA #$C8
TFR A,DP
LDD #$0000
STD sfx_pointer         ; Clear pointer (extended addressing)
PULS DP
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
JSR $F1AF    ; DP_to_C8 (restore DP after vector drawing)
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
    ; VPy_LINE:5
    LDD #0
    STD VAR_FRAME_COUNT
    ; VPy_LINE:7
    LDD #0
    STD VAR_CURRENT_SFX
    ; VPy_LINE:9
    LDD #0
    STD VAR_PLAYED
    ; VPy_LINE:12
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 12
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:13
; PLAY_MUSIC("bgm") - play music asset
    LDX #_BGM_MUSIC
    JSR PLAY_MUSIC_RUNTIME
    LDD #0
    STD RESULT
; VPy_LINE:11

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

    ; VPy_LINE:15
LOOP_BODY:
    JSR $F1AF    ; DP_to_C8 (ensure DP for variable access)
    JSR Wait_Recal ; Auto-injected: sync with vector beam
    ; DEBUG: Processing 3 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(0)
    ; VPy_LINE:17
    LDD VAR_FRAME_COUNT
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
    LDU #VAR_FRAME_COUNT
    STU TMPPTR
    STX ,U
    ; DEBUG: Statement 1 - Discriminant(9)
    ; VPy_LINE:20
    LDD VAR_FRAME_COUNT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #50
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_2
    LDD #0
    STD RESULT
    BRA CE_3
CT_2:
    LDD #1
    STD RESULT
CE_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    ; VPy_LINE:21
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_FRAME_COUNT
    STU TMPPTR
    STX ,U
    ; VPy_LINE:22
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYED
    STU TMPPTR
    STX ,U
    ; VPy_LINE:23
    LDD VAR_CURRENT_SFX
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
    LDU #VAR_CURRENT_SFX
    STU TMPPTR
    STX ,U
    ; VPy_LINE:24
    LDD VAR_CURRENT_SFX
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #4
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGE CT_6
    LDD #0
    STD RESULT
    BRA CE_7
CT_6:
    LDD #1
    STD RESULT
CE_7:
    LDD RESULT
    LBEQ IF_NEXT_5
    ; VPy_LINE:25
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_CURRENT_SFX
    STU TMPPTR
    STX ,U
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    ; DEBUG: Statement 2 - Discriminant(9)
    ; VPy_LINE:28
    LDD VAR_PLAYED
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BEQ CT_10
    LDD #0
    STD RESULT
    BRA CE_11
CT_10:
    LDD #1
    STD RESULT
CE_11:
    LDD RESULT
    LBEQ IF_NEXT_9
    ; VPy_LINE:29
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_PLAYED
    STU TMPPTR
    STX ,U
    ; VPy_LINE:30
    LDD VAR_CURRENT_SFX
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
    LBEQ IF_NEXT_13
    ; VPy_LINE:31
; PLAY_SFX("laser") - play sound effect (one-shot)
    LDX #_LASER_SFX
    JSR PLAY_SFX_RUNTIME
    LDD #0
    STD RESULT
    LBRA IF_END_12
IF_NEXT_13:
IF_END_12:
    ; VPy_LINE:32
    LDD VAR_CURRENT_SFX
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
    LBEQ IF_NEXT_17
    ; VPy_LINE:33
; PLAY_SFX("jump") - play sound effect (one-shot)
    LDX #_JUMP_SFX
    JSR PLAY_SFX_RUNTIME
    LDD #0
    STD RESULT
    LBRA IF_END_16
IF_NEXT_17:
IF_END_16:
    ; VPy_LINE:34
    LDD VAR_CURRENT_SFX
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
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
    LBEQ IF_NEXT_21
    ; VPy_LINE:35
; PLAY_SFX("hit") - play sound effect (one-shot)
    LDX #_HIT_SFX
    JSR PLAY_SFX_RUNTIME
    LDD #0
    STD RESULT
    LBRA IF_END_20
IF_NEXT_21:
IF_END_20:
    ; VPy_LINE:36
    LDD VAR_CURRENT_SFX
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
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
    ; VPy_LINE:37
; PLAY_SFX("coin") - play sound effect (one-shot)
    LDX #_COIN_SFX
    JSR PLAY_SFX_RUNTIME
    LDD #0
    STD RESULT
    LBRA IF_END_24
IF_NEXT_25:
IF_END_24:
    LBRA IF_END_8
IF_NEXT_9:
IF_END_8:
    JSR AUDIO_UPDATE  ; Auto-injected: update music + SFX (after all game logic)
    RTS

;***************************************************************************
; DATA SECTION
;***************************************************************************
VL_PTR     EQU $CF80      ; Current position in vector list
VL_Y       EQU $CF82      ; Y position (1 byte)
VL_X       EQU $CF83      ; X position (1 byte)
VL_SCALE   EQU $CF84      ; Scale factor (1 byte)
VAR_FRAME_COUNT EQU $CF10+0
VAR_CURRENT_SFX EQU $CF10+2
VAR_PLAYED EQU $CF10+4
; Call argument scratch space
VAR_ARG0 EQU $C8B2
VAR_ARG1 EQU $C8B4

; ========================================
; ASSET DATA SECTION
; Embedded 5 of 10 assets (unused assets excluded)
; ========================================

; Generated from bgm.vmus (internal name: Background Music)
; Tempo: 120 BPM, Total events: 16 (PSG Direct format)
; Format: FCB count, FCB reg, val, ... (per frame), FCB 0 (end)

_BGM_MUSIC:
    ; Frame-based PSG register writes
    FCB     0              ; Delay 0 frames (maintain previous state)
    FCB     6              ; Frame 0 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $66             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     6              ; Frame 25 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $1C             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     6              ; Frame 50 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $EF             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     6              ; Frame 75 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $B3             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     6              ; Frame 100 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $EF             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     24              ; Delay 24 frames (maintain previous state)
    FCB     6              ; Frame 124 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $1C             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     26              ; Delay 26 frames (maintain previous state)
    FCB     6              ; Frame 150 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $66             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     6              ; Frame 175 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $DE             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     6              ; Frame 200 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $3F             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     24              ; Delay 24 frames (maintain previous state)
    FCB     6              ; Frame 224 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $0C             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     6              ; Frame 249 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $D5             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     26              ; Delay 26 frames (maintain previous state)
    FCB     6              ; Frame 275 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $9F             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     6              ; Frame 300 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $D5             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $00             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     6              ; Frame 325 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $0C             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     6              ; Frame 350 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $3F             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     6              ; Frame 375 - 6 register writes
    FCB     8               ; Reg 8 number
    FCB     $00             ; Reg 8 value
    FCB     2               ; Reg 2 number
    FCB     $AA             ; Reg 2 value
    FCB     3               ; Reg 3 number
    FCB     $01             ; Reg 3 value
    FCB     9               ; Reg 9 number
    FCB     $0A             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FD             ; Reg 7 value
    FCB     25              ; Delay 25 frames before loop
    FCB     $FF             ; Loop command ($FF never valid as count)
    FDB     _BGM_MUSIC       ; Jump to start (absolute address)


; ========================================
; SFX Asset: jump (from /Users/daniel/projects/vectrex-pseudo-python/examples/sfx_buttons/assets/sfx/jump.vsfx)
; ========================================
_JUMP_SFX:
    ; SFX: jump (jump)
    ; Duration: 180ms (9fr), Freq: 330Hz, Channel: 0
    FCB $A0         ; Frame 0 - flags (vol=0, tone=Y, noise=N)
    FCB $00, $AA  ; Tone period = 170 (big-endian)
    FCB $AE         ; Frame 1 - flags (vol=14, tone=Y, noise=N)
    FCB $00, $CA  ; Tone period = 202 (big-endian)
    FCB $AD         ; Frame 2 - flags (vol=13, tone=Y, noise=N)
    FCB $00, $EA  ; Tone period = 234 (big-endian)
    FCB $AC         ; Frame 3 - flags (vol=12, tone=Y, noise=N)
    FCB $01, $0A  ; Tone period = 266 (big-endian)
    FCB $AC         ; Frame 4 - flags (vol=12, tone=Y, noise=N)
    FCB $01, $2A  ; Tone period = 298 (big-endian)
    FCB $AC         ; Frame 5 - flags (vol=12, tone=Y, noise=N)
    FCB $01, $4A  ; Tone period = 330 (big-endian)
    FCB $AC         ; Frame 6 - flags (vol=12, tone=Y, noise=N)
    FCB $01, $6A  ; Tone period = 362 (big-endian)
    FCB $AC         ; Frame 7 - flags (vol=12, tone=Y, noise=N)
    FCB $01, $8A  ; Tone period = 394 (big-endian)
    FCB $A6         ; Frame 8 - flags (vol=6, tone=Y, noise=N)
    FCB $01, $AA  ; Tone period = 426 (big-endian)
    FCB $D0, $20    ; End of effect marker


; ========================================
; SFX Asset: coin (from /Users/daniel/projects/vectrex-pseudo-python/examples/sfx_buttons/assets/sfx/coin.vsfx)
; ========================================
_COIN_SFX:
    ; SFX: coin (coin)
    ; Duration: 250ms (12fr), Freq: 880Hz, Channel: 0
    FCB $A0         ; Frame 0 - flags (vol=0, tone=Y, noise=N)
    FCB $00, $6B  ; Tone period = 107 (big-endian)
    FCB $AE         ; Frame 1 - flags (vol=14, tone=Y, noise=N)
    FCB $00, $6B  ; Tone period = 107 (big-endian)
    FCB $AE         ; Frame 2 - flags (vol=14, tone=Y, noise=N)
    FCB $00, $6B  ; Tone period = 107 (big-endian)
    FCB $AD         ; Frame 3 - flags (vol=13, tone=Y, noise=N)
    FCB $00, $55  ; Tone period = 85 (big-endian)
    FCB $AC         ; Frame 4 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $55  ; Tone period = 85 (big-endian)
    FCB $AC         ; Frame 5 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $47  ; Tone period = 71 (big-endian)
    FCB $AC         ; Frame 6 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $47  ; Tone period = 71 (big-endian)
    FCB $AC         ; Frame 7 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $47  ; Tone period = 71 (big-endian)
    FCB $AC         ; Frame 8 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $35  ; Tone period = 53 (big-endian)
    FCB $AC         ; Frame 9 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $35  ; Tone period = 53 (big-endian)
    FCB $A7         ; Frame 10 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $6B  ; Tone period = 107 (big-endian)
    FCB $A3         ; Frame 11 - flags (vol=3, tone=Y, noise=N)
    FCB $00, $6B  ; Tone period = 107 (big-endian)
    FCB $D0, $20    ; End of effect marker


; ========================================
; SFX Asset: hit (from /Users/daniel/projects/vectrex-pseudo-python/examples/sfx_buttons/assets/sfx/hit.vsfx)
; ========================================
_HIT_SFX:
    ; SFX: hit (hit)
    ; Duration: 300ms (15fr), Freq: 200Hz, Channel: 0
    FCB $60         ; Frame 0 - flags (vol=0, tone=Y, noise=Y)
    FCB $00, $8C  ; Tone period = 140 (big-endian)
    FCB $08         ; Noise period
    FCB $6F         ; Frame 1 - flags (vol=15, tone=Y, noise=Y)
    FCB $00, $AA  ; Tone period = 170 (big-endian)
    FCB $08         ; Noise period
    FCB $6F         ; Frame 2 - flags (vol=15, tone=Y, noise=Y)
    FCB $00, $C8  ; Tone period = 200 (big-endian)
    FCB $08         ; Noise period
    FCB $6E         ; Frame 3 - flags (vol=14, tone=Y, noise=Y)
    FCB $00, $E6  ; Tone period = 230 (big-endian)
    FCB $08         ; Noise period
    FCB $6D         ; Frame 4 - flags (vol=13, tone=Y, noise=Y)
    FCB $01, $04  ; Tone period = 260 (big-endian)
    FCB $08         ; Noise period
    FCB $6C         ; Frame 5 - flags (vol=12, tone=Y, noise=Y)
    FCB $01, $22  ; Tone period = 290 (big-endian)
    FCB $08         ; Noise period
    FCB $6C         ; Frame 6 - flags (vol=12, tone=Y, noise=Y)
    FCB $01, $40  ; Tone period = 320 (big-endian)
    FCB $08         ; Noise period
    FCB $6C         ; Frame 7 - flags (vol=12, tone=Y, noise=Y)
    FCB $01, $5E  ; Tone period = 350 (big-endian)
    FCB $08         ; Noise period
    FCB $6C         ; Frame 8 - flags (vol=12, tone=Y, noise=Y)
    FCB $01, $7C  ; Tone period = 380 (big-endian)
    FCB $08         ; Noise period
    FCB $6C         ; Frame 9 - flags (vol=12, tone=Y, noise=Y)
    FCB $01, $9A  ; Tone period = 410 (big-endian)
    FCB $08         ; Noise period
    FCB $6C         ; Frame 10 - flags (vol=12, tone=Y, noise=Y)
    FCB $01, $B8  ; Tone period = 440 (big-endian)
    FCB $08         ; Noise period
    FCB $6C         ; Frame 11 - flags (vol=12, tone=Y, noise=Y)
    FCB $01, $D6  ; Tone period = 470 (big-endian)
    FCB $08         ; Noise period
    FCB $69         ; Frame 12 - flags (vol=9, tone=Y, noise=Y)
    FCB $01, $F4  ; Tone period = 500 (big-endian)
    FCB $08         ; Noise period
    FCB $66         ; Frame 13 - flags (vol=6, tone=Y, noise=Y)
    FCB $02, $12  ; Tone period = 530 (big-endian)
    FCB $08         ; Noise period
    FCB $63         ; Frame 14 - flags (vol=3, tone=Y, noise=Y)
    FCB $02, $30  ; Tone period = 560 (big-endian)
    FCB $08         ; Noise period
    FCB $D0, $20    ; End of effect marker


; ========================================
; SFX Asset: laser (from /Users/daniel/projects/vectrex-pseudo-python/examples/sfx_buttons/assets/sfx/laser.vsfx)
; ========================================
_LASER_SFX:
    ; SFX: laser (laser)
    ; Duration: 500ms (25fr), Freq: 880Hz, Channel: 0
    FCB $A0         ; Frame 0 - flags (vol=0, tone=Y, noise=N)
    FCB $00, $34  ; Tone period = 52 (big-endian)
    FCB $AF         ; Frame 1 - flags (vol=15, tone=Y, noise=N)
    FCB $00, $3A  ; Tone period = 58 (big-endian)
    FCB $AC         ; Frame 2 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $42  ; Tone period = 66 (big-endian)
    FCB $AC         ; Frame 3 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $48  ; Tone period = 72 (big-endian)
    FCB $AC         ; Frame 4 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $4E  ; Tone period = 78 (big-endian)
    FCB $AC         ; Frame 5 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $56  ; Tone period = 86 (big-endian)
    FCB $AC         ; Frame 6 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $5C  ; Tone period = 92 (big-endian)
    FCB $AC         ; Frame 7 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $62  ; Tone period = 98 (big-endian)
    FCB $AC         ; Frame 8 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $6A  ; Tone period = 106 (big-endian)
    FCB $AC         ; Frame 9 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $70  ; Tone period = 112 (big-endian)
    FCB $AC         ; Frame 10 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $76  ; Tone period = 118 (big-endian)
    FCB $AC         ; Frame 11 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $7C  ; Tone period = 124 (big-endian)
    FCB $AC         ; Frame 12 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $84  ; Tone period = 132 (big-endian)
    FCB $AC         ; Frame 13 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $8A  ; Tone period = 138 (big-endian)
    FCB $AC         ; Frame 14 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $90  ; Tone period = 144 (big-endian)
    FCB $AC         ; Frame 15 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $98  ; Tone period = 152 (big-endian)
    FCB $AC         ; Frame 16 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $9E  ; Tone period = 158 (big-endian)
    FCB $AC         ; Frame 17 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $A4  ; Tone period = 164 (big-endian)
    FCB $AC         ; Frame 18 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $AC  ; Tone period = 172 (big-endian)
    FCB $AC         ; Frame 19 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $B2  ; Tone period = 178 (big-endian)
    FCB $AC         ; Frame 20 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $B8  ; Tone period = 184 (big-endian)
    FCB $A9         ; Frame 21 - flags (vol=9, tone=Y, noise=N)
    FCB $00, $C0  ; Tone period = 192 (big-endian)
    FCB $A7         ; Frame 22 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $C6  ; Tone period = 198 (big-endian)
    FCB $A4         ; Frame 23 - flags (vol=4, tone=Y, noise=N)
    FCB $00, $CC  ; Tone period = 204 (big-endian)
    FCB $A2         ; Frame 24 - flags (vol=2, tone=Y, noise=N)
    FCB $00, $D4  ; Tone period = 212 (big-endian)
    FCB $D0, $20    ; End of effect marker


DRAW_VEC_X EQU RESULT+6
DRAW_VEC_Y EQU RESULT+7
MIRROR_X EQU RESULT+8
MIRROR_Y EQU RESULT+9
DRAW_VEC_INTENSITY EQU RESULT+10
DRAW_CIRCLE_XC EQU RESULT+11
DRAW_CIRCLE_YC EQU RESULT+12
DRAW_CIRCLE_DIAM EQU RESULT+13
DRAW_CIRCLE_INTENSITY EQU RESULT+14
DRAW_CIRCLE_TEMP EQU RESULT+15
