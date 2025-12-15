; --- Motorola 6809 backend (Vectrex) title='JETPAC' origin=$0000 ---
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
    FCC "JETPAC"
    FCB $80
    FCB 0

;***************************************************************************
; CODE SECTION
;***************************************************************************

; === RAM VARIABLE DEFINITIONS (EQU) ===
; Must be defined BEFORE builtin helpers that reference them
RESULT         EQU $C880   ; Main result temporary
PSG_MUSIC_PTR    EQU $C89C   ; Pointer to current PSG music position (RESULT+$1C, 2 bytes)
PSG_MUSIC_START  EQU $C89E   ; Pointer to start of PSG music for loops (RESULT+$1E, 2 bytes)
PSG_IS_PLAYING   EQU $C8A0   ; Playing flag (RESULT+$20, 1 byte)
PSG_MUSIC_ACTIVE EQU $C8A1   ; Set=1 during UPDATE_MUSIC_PSG (for logging, 1 byte)
PSG_FRAME_COUNT  EQU $C8A2   ; Current frame register write count (RESULT+$22, 1 byte)
PSG_MUSIC_PTR_DP   EQU $9C  ; DP-relative offset (for lwasm compatibility)
PSG_MUSIC_START_DP EQU $9E  ; DP-relative offset (for lwasm compatibility)
PSG_IS_PLAYING_DP  EQU $A0  ; DP-relative offset (for lwasm compatibility)
PSG_MUSIC_ACTIVE_DP EQU $A1 ; DP-relative offset (for lwasm compatibility)
PSG_FRAME_COUNT_DP EQU $A2  ; DP-relative offset (for lwasm compatibility)
SFX_PTR        EQU $C8A8   ; Current SFX pointer (RESULT+$28, 2 bytes)
SFX_TICK       EQU $C8AA   ; 32-bit tick counter (RESULT+$2A, 4 bytes)
SFX_EVENT      EQU $C8AE   ; Current event pointer (RESULT+$2E, 2 bytes)
SFX_ACTIVE     EQU $C8B0   ; Playback state (RESULT+$30, 1 byte)

    JMP START

VECTREX_PRINT_TEXT:
    ; CRITICAL: Print_Str_d requires DP=$D0 and signature is (Y, X, string)
    ; VPy signature: PRINT_TEXT(string, Y, X) -> args (ARG0=string, ARG1=Y, ARG2=X)
    ; BIOS signature: Print_Str_d(A=Y, B=X, U=string)
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDU VAR_ARG0   ; string pointer (ARG0 = first param)
    LDA VAR_ARG1+1 ; Y (ARG1 = second param)
    LDB VAR_ARG2+1 ; X (ARG2 = third param)
    JSR Print_Str_d
    RTS
VECTREX_MOVE_TO:
    LDA VAR_ARG1+1 ; Y
    LDB VAR_ARG0+1 ; X
    JSR Moveto_d
    ; store new current position
    LDA VAR_ARG0+1
    STA VCUR_X
    LDA VAR_ARG1+1
    STA VCUR_Y
    RTS
; Draw from current (VCUR_X,VCUR_Y) to new (x,y) provided in low bytes VAR_ARG0/1.
; Semántica: igual a MOVE_TO seguido de línea, pero preserva origen previo como punto inicial.
; Deltas pueden ser ±127 (hardware Vectrex soporta rango completo).
VECTREX_DRAW_TO:
    ; Cargar destino (x,y)
    LDA VAR_ARG0+1  ; Xdest en A temporalmente
    STA VLINE_DX    ; reutilizar buffer temporal (bajo) para Xdest
    LDA VAR_ARG1+1  ; Ydest en A
    STA VLINE_DY    ; reutilizar buffer temporal para Ydest
    ; Calcular dx = Xdest - VCUR_X
    LDA VLINE_DX
    SUBA VCUR_X
    STA VLINE_DX
    ; Calcular dy = Ydest - VCUR_Y
    LDA VLINE_DY
    SUBA VCUR_Y
    STA VLINE_DY
    ; No clamping needed - signed byte arithmetic handles ±127 correctly
    ; Mover haz al origen previo (VCUR_Y en A, VCUR_X en B)
    LDA VCUR_Y
    LDB VCUR_X
    JSR Moveto_d
    ; Dibujar línea usando deltas (A=dy, B=dx)
    LDA VLINE_DY
    LDB VLINE_DX
    JSR Draw_Line_d
    ; Actualizar posición actual al destino exacto original
    LDA VAR_ARG0+1
    STA VCUR_X
    LDA VAR_ARG1+1
    STA VCUR_Y
    RTS
VECTREX_SET_INTENSITY:
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

; PLAY_MUSIC_RUNTIME - Start PSG music playback
; Input: X = pointer to PSG music data
PLAY_MUSIC_RUNTIME:
STX >PSG_MUSIC_PTR     ; Store current music pointer (force extended)
STX >PSG_MUSIC_START   ; Store start pointer for loops (force extended)
LDA #$01
STA >PSG_IS_PLAYING ; Mark as playing (extended - var at 0xC8A0)
RTS

; ============================================================================
; UPDATE_MUSIC_PSG - Update PSG (call every frame)
; ============================================================================
UPDATE_MUSIC_PSG:
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
; Silence all channels (write $00 to volume regs 8,9,10)
LDA #8
LDB #$00
PSHS X
STA VIA_port_a
LDA #$19
STA VIA_port_b
LDA #$01
STA VIA_port_b
LDA VIA_port_a
STB VIA_port_a
LDB #$11
STB VIA_port_b
LDB #$01
STB VIA_port_b
LDA #9
LDB #$00
STA VIA_port_a
LDA #$19
STA VIA_port_b
LDA #$01
STA VIA_port_b
LDA VIA_port_a
STB VIA_port_a
LDB #$11
STB VIA_port_b
LDB #$01
STB VIA_port_b
LDA #10
LDB #$00
STA VIA_port_a
LDA #$19
STA VIA_port_b
LDA #$01
STA VIA_port_b
LDA VIA_port_a
STB VIA_port_a
LDB #$11
STB VIA_port_b
LDB #$01
STB VIA_port_b
PULS X
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

; Silence all PSG channels (write $00 to volume registers)
LDA #8                 ; Channel A volume
LDB #$00
PSHS X
STA VIA_port_a
LDA #$19
STA VIA_port_b
LDA #$01
STA VIA_port_b
LDA VIA_port_a
STB VIA_port_a
LDB #$11
STB VIA_port_b
LDB #$01
STB VIA_port_b
LDA #9                 ; Channel B volume
LDB #$00
STA VIA_port_a
LDA #$19
STA VIA_port_b
LDA #$01
STA VIA_port_b
LDA VIA_port_a
STB VIA_port_a
LDB #$11
STB VIA_port_b
LDB #$01
STB VIA_port_b
LDA #10                ; Channel C volume
LDB #$00
STA VIA_port_a
LDA #$19
STA VIA_port_b
LDA #$01
STA VIA_port_b
LDA VIA_port_a
STB VIA_port_a
LDB #$11
STB VIA_port_b
LDB #$01
STB VIA_port_b
PULS X

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
    STD VAR_SCREEN
    LDD #0
    STD VAR_LOGO_TIMER
    LDD #0
    STD VAR_GAME_STATE
    LDD #0
    STD VAR_FUEL_LEVEL
    LDD #0
    STD VAR_PLAYER_X
    LDD #65456
    STD VAR_PLAYER_Y
    LDD #0
    STD VAR_PLAYER_DX
    LDD #0
    STD VAR_PLAYER_DY
    LDD #0
    STD VAR_PART1_STATE
    LDD #0
    STD VAR_PART2_STATE
    LDD #0
    STD VAR_PART3_STATE
    LDD #0
    STD VAR_CARRIED_PART
    LDD #65466
    STD VAR_PLAT1_Y
    LDD #65516
    STD VAR_PLAT2_Y
    LDD #30
    STD VAR_PLAT3_Y
    LDD #65456
    STD VAR_PART1_X
    LDD #65481
    STD VAR_PART1_Y
    LDD #0
    STD VAR_PART2_X
    LDD #65531
    STD VAR_PART2_Y
    LDD #80
    STD VAR_PART3_X
    LDD #45
    STD VAR_PART3_Y
    LDD #0
    STD VAR_SHIP_BASE_X
    LDD #65451
    STD VAR_SHIP_BASE_Y
    LDD #100
    STD VAR_ENEMY_X
    LDD #50
    STD VAR_ENEMY_Y
    LDD #65534
    STD VAR_ENEMY_DX
    ; VPy_LINE:48
    LDD #0
    STD RESULT

MAIN:
    JSR UPDATE_MUSIC_PSG   ; Update PSG registers (before Wait_Recal)
    JSR Wait_Recal
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    BRA MAIN

LOOP_BODY:
    JSR UPDATE_MUSIC_PSG   ; Update PSG registers (before loop body)
    ; DEBUG: Processing 2 statements in loop() body
    ; DEBUG: Statement 0 - Discriminant(7)
    ; VPy_LINE:53
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
    ; VPy_LINE:54
    JSR DRAW_LOGO_SCREEN
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    ; DEBUG: Statement 1 - Discriminant(7)
    ; VPy_LINE:55
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
    ; VPy_LINE:56
    JSR DRAW_GAME_SCREEN
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    RTS

DRAW_LOGO_SCREEN: ; function
; --- function draw_logo_screen ---
    ; VPy_LINE:60
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 60
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:61
; DRAW_VECTOR("jetpac_logo", x, y) - 10 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #20
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_JETPAC_LOGO_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_JETPAC_LOGO_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_JETPAC_LOGO_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_JETPAC_LOGO_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDX #_JETPAC_LOGO_PATH4  ; Path 4
    JSR Draw_Sync_List_At
    LDX #_JETPAC_LOGO_PATH5  ; Path 5
    JSR Draw_Sync_List_At
    LDX #_JETPAC_LOGO_PATH6  ; Path 6
    JSR Draw_Sync_List_At
    LDX #_JETPAC_LOGO_PATH7  ; Path 7
    JSR Draw_Sync_List_At
    LDX #_JETPAC_LOGO_PATH8  ; Path 8
    JSR Draw_Sync_List_At
    LDX #_JETPAC_LOGO_PATH9  ; Path 9
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
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
    ; VPy_LINE:65
    LDD #65486
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65476
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_0
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 65
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:68
    LDD VAR_LOGO_TIMER
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
    LDU #VAR_LOGO_TIMER
    STU TMPPTR
    STX ,U
    ; VPy_LINE:69
    LDD VAR_LOGO_TIMER
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #300
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_10
    LDD #0
    STD RESULT
    BRA CE_11
CT_10:
    LDD #1
    STD RESULT
CE_11:
    LDD RESULT
    LBEQ IF_NEXT_9
    ; VPy_LINE:70
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_SCREEN
    STU TMPPTR
    STX ,U
    ; VPy_LINE:71
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_LOGO_TIMER
    STU TMPPTR
    STX ,U
    LBRA IF_END_8
IF_NEXT_9:
IF_END_8:
    RTS

DRAW_GAME_SCREEN: ; function
; --- function draw_game_screen ---
    LEAS -2,S ; allocate locals
    ; VPy_LINE:75
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 75
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:76
; DRAW_VECTOR("platform", x, y) - 4 path(s) at position
    LDD #65456
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PLAT1_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_PLATFORM_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_PLATFORM_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_PLATFORM_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_PLATFORM_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    ; VPy_LINE:77
; DRAW_VECTOR("platform", x, y) - 4 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PLAT2_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_PLATFORM_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_PLATFORM_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_PLATFORM_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_PLATFORM_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    ; VPy_LINE:78
; DRAW_VECTOR("platform", x, y) - 4 path(s) at position
    LDD #80
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PLAT3_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_PLATFORM_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_PLATFORM_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_PLATFORM_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_PLATFORM_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    ; VPy_LINE:81
    LDD VAR_PART1_STATE
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
    ; VPy_LINE:82
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 82
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:83
; DRAW_VECTOR("rocket_base", x, y) - 4 path(s) at position
    LDD VAR_PART1_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PART1_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_ROCKET_BASE_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_ROCKET_BASE_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_ROCKET_BASE_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_ROCKET_BASE_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_12
IF_NEXT_13:
IF_END_12:
    ; VPy_LINE:85
    LDD VAR_PART2_STATE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
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
    ; VPy_LINE:86
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 86
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:87
; DRAW_VECTOR("rocket_middle", x, y) - 4 path(s) at position
    LDD VAR_PART2_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PART2_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_ROCKET_MIDDLE_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_ROCKET_MIDDLE_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_ROCKET_MIDDLE_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_ROCKET_MIDDLE_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_16
IF_NEXT_17:
IF_END_16:
    ; VPy_LINE:89
    LDD VAR_PART3_STATE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
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
    ; VPy_LINE:90
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 90
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:91
; DRAW_VECTOR("rocket_top", x, y) - 2 path(s) at position
    LDD VAR_PART3_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PART3_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_ROCKET_TOP_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_ROCKET_TOP_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_20
IF_NEXT_21:
IF_END_20:
    ; VPy_LINE:94
    LDD VAR_PART1_STATE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
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
    ; VPy_LINE:95
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 95
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:96
; DRAW_VECTOR("rocket_base", x, y) - 4 path(s) at position
    LDD VAR_SHIP_BASE_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_SHIP_BASE_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_ROCKET_BASE_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_ROCKET_BASE_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_ROCKET_BASE_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_ROCKET_BASE_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_24
IF_NEXT_25:
IF_END_24:
    ; VPy_LINE:98
    LDD VAR_PART2_STATE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
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
    LBEQ IF_NEXT_29
    ; VPy_LINE:99
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 99
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:100
; DRAW_VECTOR("rocket_middle", x, y) - 4 path(s) at position
    LDD VAR_SHIP_BASE_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_SHIP_BASE_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_ROCKET_MIDDLE_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_ROCKET_MIDDLE_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_ROCKET_MIDDLE_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_ROCKET_MIDDLE_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_28
IF_NEXT_29:
IF_END_28:
    ; VPy_LINE:102
    LDD VAR_PART3_STATE
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
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
    ; VPy_LINE:103
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 103
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:104
; DRAW_VECTOR("rocket_top", x, y) - 2 path(s) at position
    LDD VAR_SHIP_BASE_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_SHIP_BASE_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #30
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_ROCKET_TOP_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_ROCKET_TOP_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_32
IF_NEXT_33:
IF_END_32:
    ; VPy_LINE:107
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 107
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:108
; DRAW_VECTOR("astronaut", x, y) - 6 path(s) at position
    LDD VAR_PLAYER_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PLAYER_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_ASTRONAUT_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_ASTRONAUT_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_ASTRONAUT_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_ASTRONAUT_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDX #_ASTRONAUT_PATH4  ; Path 4
    JSR Draw_Sync_List_At
    LDX #_ASTRONAUT_PATH5  ; Path 5
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    ; VPy_LINE:111
    LDD VAR_CARRIED_PART
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
    ; VPy_LINE:112
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 112
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:113
; DRAW_VECTOR("rocket_base", x, y) - 4 path(s) at position
    LDD VAR_PLAYER_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #12
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_ROCKET_BASE_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_ROCKET_BASE_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_ROCKET_BASE_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_ROCKET_BASE_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_36
IF_NEXT_37:
IF_END_36:
    ; VPy_LINE:114
    LDD VAR_CARRIED_PART
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #2
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
    ; VPy_LINE:115
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 115
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:116
; DRAW_VECTOR("rocket_middle", x, y) - 4 path(s) at position
    LDD VAR_PLAYER_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #12
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_ROCKET_MIDDLE_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_ROCKET_MIDDLE_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_ROCKET_MIDDLE_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_ROCKET_MIDDLE_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_40
IF_NEXT_41:
IF_END_40:
    ; VPy_LINE:117
    LDD VAR_CARRIED_PART
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #3
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
    ; VPy_LINE:118
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 118
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:119
; DRAW_VECTOR("rocket_top", x, y) - 2 path(s) at position
    LDD VAR_PLAYER_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #12
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_ROCKET_TOP_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_ROCKET_TOP_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    LBRA IF_END_44
IF_NEXT_45:
IF_END_44:
    ; VPy_LINE:122
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 122
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:123
; DRAW_VECTOR("enemy_alien", x, y) - 7 path(s) at position
    LDD VAR_ENEMY_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_ENEMY_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDX #_ENEMY_ALIEN_PATH0  ; Path 0
    JSR Draw_Sync_List_At
    LDX #_ENEMY_ALIEN_PATH1  ; Path 1
    JSR Draw_Sync_List_At
    LDX #_ENEMY_ALIEN_PATH2  ; Path 2
    JSR Draw_Sync_List_At
    LDX #_ENEMY_ALIEN_PATH3  ; Path 3
    JSR Draw_Sync_List_At
    LDX #_ENEMY_ALIEN_PATH4  ; Path 4
    JSR Draw_Sync_List_At
    LDX #_ENEMY_ALIEN_PATH5  ; Path 5
    JSR Draw_Sync_List_At
    LDX #_ENEMY_ALIEN_PATH6  ; Path 6
    JSR Draw_Sync_List_At
    LDD #0
    STD RESULT
    ; VPy_LINE:126
    LDD VAR_ENEMY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD VAR_ENEMY_DX
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:127
    LDD VAR_ENEMY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #65436
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_50
    LDD #0
    STD RESULT
    BRA CE_51
CT_50:
    LDD #1
    STD RESULT
CE_51:
    LDD RESULT
    LBEQ IF_NEXT_49
    ; VPy_LINE:128
    LDD #2
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY_DX
    STU TMPPTR
    STX ,U
    LBRA IF_END_48
IF_NEXT_49:
IF_END_48:
    ; VPy_LINE:129
    LDD VAR_ENEMY_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_54
    LDD #0
    STD RESULT
    BRA CE_55
CT_54:
    LDD #1
    STD RESULT
CE_55:
    LDD RESULT
    LBEQ IF_NEXT_53
    ; VPy_LINE:130
    LDD #65534
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY_DX
    STU TMPPTR
    STX ,U
    LBRA IF_END_52
IF_NEXT_53:
IF_END_52:
    ; VPy_LINE:133
    LDD #64
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 133
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:134
    LDD #65426
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #110
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_1
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
; NATIVE_CALL: VECTREX_PRINT_TEXT at line 134
    JSR VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:137
    LDD VAR_FUEL_LEVEL
    STD RESULT
    LDD RESULT
    LSRA
    RORB
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:138
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_58
    LDD #0
    STD RESULT
    BRA CE_59
CT_58:
    LDD #1
    STD RESULT
CE_59:
    LDD RESULT
    LBEQ IF_NEXT_57
    ; VPy_LINE:139
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 139
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:140
    LDD #65426
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_MOVE_TO at line 140
    JSR VECTREX_MOVE_TO
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:141
    LDD #65426
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
; NATIVE_CALL: VECTREX_DRAW_TO at line 141
    JSR VECTREX_DRAW_TO
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_56
IF_NEXT_57:
IF_END_56:
    LEAS 2,S ; free locals
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
; Variables (in RAM)
TMPLEFT   EQU RESULT+2
TMPRIGHT  EQU RESULT+4
TMPPTR    EQU RESULT+6
DIV_A   EQU RESULT+18
DIV_B   EQU RESULT+20
DIV_Q   EQU RESULT+22
DIV_R   EQU RESULT+24
TEMP_YX   EQU RESULT+26   ; Temporary y,x storage (2 bytes)
VL_PTR     EQU $CF80      ; Current position in vector list
VL_Y       EQU $CF82      ; Y position (1 byte)
VL_X       EQU $CF83      ; X position (1 byte)
VL_SCALE   EQU $CF84      ; Scale factor (1 byte)
VAR_BAR_WIDTH EQU $CF00+0
VAR_CARRIED_PART EQU $CF00+2
VAR_ENEMY_DX EQU $CF00+4
VAR_ENEMY_X EQU $CF00+6
VAR_ENEMY_Y EQU $CF00+8
VAR_FUEL_LEVEL EQU $CF00+10
VAR_GAME_STATE EQU $CF00+12
VAR_LOGO_TIMER EQU $CF00+14
VAR_PART1_STATE EQU $CF00+16
VAR_PART1_X EQU $CF00+18
VAR_PART1_Y EQU $CF00+20
VAR_PART2_STATE EQU $CF00+22
VAR_PART2_X EQU $CF00+24
VAR_PART2_Y EQU $CF00+26
VAR_PART3_STATE EQU $CF00+28
VAR_PART3_X EQU $CF00+30
VAR_PART3_Y EQU $CF00+32
VAR_PLAT1_Y EQU $CF00+34
VAR_PLAT2_Y EQU $CF00+36
VAR_PLAT3_Y EQU $CF00+38
VAR_PLAYER_DX EQU $CF00+40
VAR_PLAYER_DY EQU $CF00+42
VAR_PLAYER_X EQU $CF00+44
VAR_PLAYER_Y EQU $CF00+46
VAR_SCREEN EQU $CF00+48
VAR_SHIP_BASE_X EQU $CF00+50
VAR_SHIP_BASE_Y EQU $CF00+52
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30

; ========================================
; ASSET DATA SECTION
; Embedded 7 of 14 assets (unused assets excluded)
; ========================================

; Vector asset: jetpac_logo
; Generated from jetpac_logo.vec (Malban Draw_Sync_List format)
; Total paths: 10, points: 87

_JETPAC_LOGO_VECTORS:  ; Main entry
_JETPAC_LOGO_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $28,$A6,0,0        ; path0: header (y=40, x=-90, next_y=0, next_x=0)
    FCB $FF,$00,$14          ; line 0: flag=-1, dy=0, dx=20
    FCB $FF,$D8,$00          ; line 1: flag=-1, dy=-40, dx=0
    FCB $FF,$F6,$FB          ; line 2: flag=-1, dy=-10, dx=-5
    FCB $FF,$00,$F6          ; line 3: flag=-1, dy=0, dx=-10
    FCB $FF,$0A,$FB          ; line 4: flag=-1, dy=10, dx=-5
    FCB 2                ; End marker

_JETPAC_LOGO_PATH1:
    FCB 127              ; path1: intensity
    FCB $28,$C4,0,0        ; path1: header (y=40, x=-60, next_y=0, next_x=0)
    FCB $FF,$CE,$00          ; line 0: flag=-1, dy=-50, dx=0
    FCB $FF,$00,$14          ; line 1: flag=-1, dy=0, dx=20
    FCB $FF,$14,$00          ; line 2: flag=-1, dy=20, dx=0
    FCB $FF,$00,$F1          ; line 3: flag=-1, dy=0, dx=-15
    FCB $FF,$0A,$00          ; line 4: flag=-1, dy=10, dx=0
    FCB $FF,$00,$0F          ; line 5: flag=-1, dy=0, dx=15
    FCB $FF,$14,$00          ; line 6: flag=-1, dy=20, dx=0
    FCB $FF,$00,$EC          ; line 7: flag=-1, dy=0, dx=-20
    FCB 2                ; End marker

_JETPAC_LOGO_PATH2:
    FCB 127              ; path2: intensity
    FCB $28,$E2,0,0        ; path2: header (y=40, x=-30, next_y=0, next_x=0)
    FCB $FF,$00,$14          ; line 0: flag=-1, dy=0, dx=20
    FCB $FF,$EC,$00          ; line 1: flag=-1, dy=-20, dx=0
    FCB $FF,$00,$05          ; line 2: flag=-1, dy=0, dx=5
    FCB $FF,$14,$00          ; line 3: flag=-1, dy=20, dx=0
    FCB $FF,$00,$05          ; line 4: flag=-1, dy=0, dx=5
    FCB $FF,$EC,$00          ; line 5: flag=-1, dy=-20, dx=0
    FCB $FF,$00,$05          ; line 6: flag=-1, dy=0, dx=5
    FCB $FF,$14,$00          ; line 7: flag=-1, dy=20, dx=0
    FCB $FF,$00,$05          ; line 8: flag=-1, dy=0, dx=5
    FCB $FF,$EC,$00          ; line 9: flag=-1, dy=-20, dx=0
    FCB $FF,$00,$05          ; line 10: flag=-1, dy=0, dx=5
    FCB $FF,$E2,$00          ; line 11: flag=-1, dy=-30, dx=0
    FCB $FF,$00,$E2          ; line 12: flag=-1, dy=0, dx=-30
    FCB $FF,$1E,$00          ; line 13: flag=-1, dy=30, dx=0
    FCB $FF,$00,$05          ; line 14: flag=-1, dy=0, dx=5
    FCB 2                ; End marker

_JETPAC_LOGO_PATH3:
    FCB 127              ; path3: intensity
    FCB $F6,$19,0,0        ; path3: header (y=-10, x=25, next_y=0, next_x=0)
    FCB $FF,$32,$00          ; line 0: flag=-1, dy=50, dx=0
    FCB $FF,$00,$14          ; line 1: flag=-1, dy=0, dx=20
    FCB $FF,$FB,$05          ; line 2: flag=-1, dy=-5, dx=5
    FCB $FF,$F1,$00          ; line 3: flag=-1, dy=-15, dx=0
    FCB $FF,$FB,$FB          ; line 4: flag=-1, dy=-5, dx=-5
    FCB $FF,$00,$EC          ; line 5: flag=-1, dy=0, dx=-20
    FCB 2                ; End marker

_JETPAC_LOGO_PATH4:
    FCB 127              ; path4: intensity
    FCB $F6,$3C,0,0        ; path4: header (y=-10, x=60, next_y=0, next_x=0)
    FCB $FF,$32,$00          ; line 0: flag=-1, dy=50, dx=0
    FCB $FF,$00,$05          ; line 1: flag=-1, dy=0, dx=5
    FCB $FF,$EC,$0F          ; line 2: flag=-1, dy=-20, dx=15
    FCB $FF,$E2,$00          ; line 3: flag=-1, dy=-30, dx=0
    FCB $FF,$00,$FB          ; line 4: flag=-1, dy=0, dx=-5
    FCB $FF,$19,$00          ; line 5: flag=-1, dy=25, dx=0
    FCB $FF,$00,$F6          ; line 6: flag=-1, dy=0, dx=-10
    FCB $FF,$E7,$00          ; line 7: flag=-1, dy=-25, dx=0
    FCB 2                ; End marker

_JETPAC_LOGO_PATH5:
    FCB 127              ; path5: intensity
    FCB $28,$6E,0,0        ; path5: header (y=40, x=110, next_y=0, next_x=0)
    FCB $FF,$00,$EC          ; line 0: flag=-1, dy=0, dx=-20
    FCB $FF,$CE,$00          ; line 1: flag=-1, dy=-50, dx=0
    FCB $FF,$00,$14          ; line 2: flag=-1, dy=0, dx=20
    FCB $FF,$05,$00          ; line 3: flag=-1, dy=5, dx=0
    FCB $FF,$00,$F1          ; line 4: flag=-1, dy=0, dx=-15
    FCB $FF,$28,$00          ; line 5: flag=-1, dy=40, dx=0
    FCB $FF,$00,$0F          ; line 6: flag=-1, dy=0, dx=15
    FCB 2                ; End marker

_JETPAC_LOGO_PATH6:
    FCB 100              ; path6: intensity
    FCB $EC,$A1,0,0        ; path6: header (y=-20, x=-95, next_y=0, next_x=0)
    FCB $FF,$00,$7F          ; line 0: flag=-1, dy=0, dx=127
    FCB 2                ; End marker

_JETPAC_LOGO_PATH7:
    FCB 80              ; path7: intensity
    FCB $32,$9C,0,0        ; path7: header (y=50, x=-100, next_y=0, next_x=0)
    FCB $FF,$FD,$02          ; line 0: flag=-1, dy=-3, dx=2
    FCB $FF,$01,$03          ; line 1: flag=-1, dy=1, dx=3
    FCB $FF,$FD,$FE          ; line 2: flag=-1, dy=-3, dx=-2
    FCB $FF,$FD,$01          ; line 3: flag=-1, dy=-3, dx=1
    FCB $FF,$02,$FC          ; line 4: flag=-1, dy=2, dx=-4
    FCB $FF,$FE,$FC          ; line 5: flag=-1, dy=-2, dx=-4
    FCB $FF,$03,$01          ; line 6: flag=-1, dy=3, dx=1
    FCB $FF,$03,$FE          ; line 7: flag=-1, dy=3, dx=-2
    FCB $FF,$FF,$03          ; line 8: flag=-1, dy=-1, dx=3
    FCB $FF,$03,$02          ; closing line: flag=-1, dy=3, dx=2
    FCB 2                ; End marker

_JETPAC_LOGO_PATH8:
    FCB 80              ; path8: intensity
    FCB $32,$64,0,0        ; path8: header (y=50, x=100, next_y=0, next_x=0)
    FCB $FF,$FD,$02          ; line 0: flag=-1, dy=-3, dx=2
    FCB $FF,$01,$03          ; line 1: flag=-1, dy=1, dx=3
    FCB $FF,$FD,$FE          ; line 2: flag=-1, dy=-3, dx=-2
    FCB $FF,$FD,$01          ; line 3: flag=-1, dy=-3, dx=1
    FCB $FF,$02,$FC          ; line 4: flag=-1, dy=2, dx=-4
    FCB $FF,$FE,$FC          ; line 5: flag=-1, dy=-2, dx=-4
    FCB $FF,$03,$01          ; line 6: flag=-1, dy=3, dx=1
    FCB $FF,$03,$FE          ; line 7: flag=-1, dy=3, dx=-2
    FCB $FF,$FF,$03          ; line 8: flag=-1, dy=-1, dx=3
    FCB $FF,$03,$02          ; closing line: flag=-1, dy=3, dx=2
    FCB 2                ; End marker

_JETPAC_LOGO_PATH9:
    FCB 80              ; path9: intensity
    FCB $37,$00,0,0        ; path9: header (y=55, x=0, next_y=0, next_x=0)
    FCB $FF,$FD,$02          ; line 0: flag=-1, dy=-3, dx=2
    FCB $FF,$01,$03          ; line 1: flag=-1, dy=1, dx=3
    FCB $FF,$FD,$FE          ; line 2: flag=-1, dy=-3, dx=-2
    FCB $FF,$FD,$01          ; line 3: flag=-1, dy=-3, dx=1
    FCB $FF,$02,$FC          ; line 4: flag=-1, dy=2, dx=-4
    FCB $FF,$FE,$FC          ; line 5: flag=-1, dy=-2, dx=-4
    FCB $FF,$03,$01          ; line 6: flag=-1, dy=3, dx=1
    FCB $FF,$03,$FE          ; line 7: flag=-1, dy=3, dx=-2
    FCB $FF,$FF,$03          ; line 8: flag=-1, dy=-1, dx=3
    FCB $FF,$03,$02          ; closing line: flag=-1, dy=3, dx=2
    FCB 2                ; End marker

; Vector asset: rocket_middle
; Generated from rocket_middle.vec (Malban Draw_Sync_List format)
; Total paths: 4, points: 12

_ROCKET_MIDDLE_VECTORS:  ; Main entry
_ROCKET_MIDDLE_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $00,$F4,0,0        ; path0: header (y=0, x=-12, next_y=0, next_x=0)
    FCB $FF,$14,$00          ; line 0: flag=-1, dy=20, dx=0
    FCB $FF,$00,$18          ; line 1: flag=-1, dy=0, dx=24
    FCB $FF,$EC,$00          ; line 2: flag=-1, dy=-20, dx=0
    FCB $FF,$00,$E8          ; closing line: flag=-1, dy=0, dx=-24
    FCB 2                ; End marker

_ROCKET_MIDDLE_PATH1:
    FCB 100              ; path1: intensity
    FCB $08,$FA,0,0        ; path1: header (y=8, x=-6, next_y=0, next_x=0)
    FCB $FF,$04,$00          ; line 0: flag=-1, dy=4, dx=0
    FCB $FF,$00,$0C          ; line 1: flag=-1, dy=0, dx=12
    FCB $FF,$FC,$00          ; line 2: flag=-1, dy=-4, dx=0
    FCB $FF,$00,$F4          ; closing line: flag=-1, dy=0, dx=-12
    FCB 2                ; End marker

_ROCKET_MIDDLE_PATH2:
    FCB 80              ; path2: intensity
    FCB $05,$F6,0,0        ; path2: header (y=5, x=-10, next_y=0, next_x=0)
    FCB $FF,$0A,$00          ; line 0: flag=-1, dy=10, dx=0
    FCB 2                ; End marker

_ROCKET_MIDDLE_PATH3:
    FCB 80              ; path3: intensity
    FCB $05,$0A,0,0        ; path3: header (y=5, x=10, next_y=0, next_x=0)
    FCB $FF,$0A,$00          ; line 0: flag=-1, dy=10, dx=0
    FCB 2                ; End marker

; Vector asset: enemy_alien
; Generated from enemy_alien.vec (Malban Draw_Sync_List format)
; Total paths: 7, points: 18

_ENEMY_ALIEN_VECTORS:  ; Main entry
_ENEMY_ALIEN_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0A,$F6,0,0        ; path0: header (y=10, x=-10, next_y=0, next_x=0)
    FCB $FF,$08,$00          ; line 0: flag=-1, dy=8, dx=0
    FCB $FF,$00,$14          ; line 1: flag=-1, dy=0, dx=20
    FCB $FF,$F8,$00          ; line 2: flag=-1, dy=-8, dx=0
    FCB $FF,$00,$EC          ; closing line: flag=-1, dy=0, dx=-20
    FCB 2                ; End marker

_ENEMY_ALIEN_PATH1:
    FCB 127              ; path1: intensity
    FCB $0A,$F8,0,0        ; path1: header (y=10, x=-8, next_y=0, next_x=0)
    FCB $FF,$F1,$00          ; line 0: flag=-1, dy=-15, dx=0
    FCB $FF,$00,$10          ; line 1: flag=-1, dy=0, dx=16
    FCB $FF,$0F,$00          ; line 2: flag=-1, dy=15, dx=0
    FCB $FF,$00,$F0          ; closing line: flag=-1, dy=0, dx=-16
    FCB 2                ; End marker

_ENEMY_ALIEN_PATH2:
    FCB 100              ; path2: intensity
    FCB $12,$FA,0,0        ; path2: header (y=18, x=-6, next_y=0, next_x=0)
    FCB $FF,$06,$FE          ; line 0: flag=-1, dy=6, dx=-2
    FCB 2                ; End marker

_ENEMY_ALIEN_PATH3:
    FCB 100              ; path3: intensity
    FCB $12,$06,0,0        ; path3: header (y=18, x=6, next_y=0, next_x=0)
    FCB $FF,$06,$02          ; line 0: flag=-1, dy=6, dx=2
    FCB 2                ; End marker

_ENEMY_ALIEN_PATH4:
    FCB 90              ; path4: intensity
    FCB $FB,$FA,0,0        ; path4: header (y=-5, x=-6, next_y=0, next_x=0)
    FCB $FF,$F9,$FE          ; line 0: flag=-1, dy=-7, dx=-2
    FCB 2                ; End marker

_ENEMY_ALIEN_PATH5:
    FCB 90              ; path5: intensity
    FCB $FB,$00,0,0        ; path5: header (y=-5, x=0, next_y=0, next_x=0)
    FCB $FF,$F9,$00          ; line 0: flag=-1, dy=-7, dx=0
    FCB 2                ; End marker

_ENEMY_ALIEN_PATH6:
    FCB 90              ; path6: intensity
    FCB $FB,$06,0,0        ; path6: header (y=-5, x=6, next_y=0, next_x=0)
    FCB $FF,$F9,$02          ; line 0: flag=-1, dy=-7, dx=2
    FCB 2                ; End marker

; Vector asset: platform
; Generated from platform.vec (Malban Draw_Sync_List format)
; Total paths: 4, points: 8

_PLATFORM_VECTORS:  ; Main entry
_PLATFORM_PATH0:    ; Path 0
    FCB 100              ; path0: intensity
    FCB $00,$E7,0,0        ; path0: header (y=0, x=-25, next_y=0, next_x=0)
    FCB $FF,$00,$32          ; line 0: flag=-1, dy=0, dx=50
    FCB 2                ; End marker

_PLATFORM_PATH1:
    FCB 80              ; path1: intensity
    FCB $00,$EC,0,0        ; path1: header (y=0, x=-20, next_y=0, next_x=0)
    FCB $FF,$F8,$FB          ; line 0: flag=-1, dy=-8, dx=-5
    FCB 2                ; End marker

_PLATFORM_PATH2:
    FCB 80              ; path2: intensity
    FCB $00,$14,0,0        ; path2: header (y=0, x=20, next_y=0, next_x=0)
    FCB $FF,$F8,$05          ; line 0: flag=-1, dy=-8, dx=5
    FCB 2                ; End marker

_PLATFORM_PATH3:
    FCB 127              ; path3: intensity
    FCB $F8,$E7,0,0        ; path3: header (y=-8, x=-25, next_y=0, next_x=0)
    FCB $FF,$00,$32          ; line 0: flag=-1, dy=0, dx=50
    FCB 2                ; End marker

; Vector asset: rocket_top
; Generated from rocket_top.vec (Malban Draw_Sync_List format)
; Total paths: 2, points: 7

_ROCKET_TOP_VECTORS:  ; Main entry
_ROCKET_TOP_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $14,$F6,0,0        ; path0: header (y=20, x=-10, next_y=0, next_x=0)
    FCB $FF,$0F,$0A          ; line 0: flag=-1, dy=15, dx=10
    FCB $FF,$F1,$0A          ; line 1: flag=-1, dy=-15, dx=10
    FCB $FF,$00,$EC          ; closing line: flag=-1, dy=0, dx=-20
    FCB 2                ; End marker

_ROCKET_TOP_PATH1:
    FCB 127              ; path1: intensity
    FCB $14,$F6,0,0        ; path1: header (y=20, x=-10, next_y=0, next_x=0)
    FCB $FF,$02,$00          ; line 0: flag=-1, dy=2, dx=0
    FCB $FF,$00,$14          ; line 1: flag=-1, dy=0, dx=20
    FCB $FF,$FE,$00          ; line 2: flag=-1, dy=-2, dx=0
    FCB $FF,$00,$EC          ; closing line: flag=-1, dy=0, dx=-20
    FCB 2                ; End marker

; Vector asset: rocket_base
; Generated from rocket_base.vec (Malban Draw_Sync_List format)
; Total paths: 4, points: 14

_ROCKET_BASE_VECTORS:  ; Main entry
_ROCKET_BASE_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $EC,$F1,0,0        ; path0: header (y=-20, x=-15, next_y=0, next_x=0)
    FCB $FF,$14,$00          ; line 0: flag=-1, dy=20, dx=0
    FCB $FF,$00,$1E          ; line 1: flag=-1, dy=0, dx=30
    FCB $FF,$EC,$00          ; line 2: flag=-1, dy=-20, dx=0
    FCB $FF,$00,$E2          ; closing line: flag=-1, dy=0, dx=-30
    FCB 2                ; End marker

_ROCKET_BASE_PATH1:
    FCB 100              ; path1: intensity
    FCB $F1,$F1,0,0        ; path1: header (y=-15, x=-15, next_y=0, next_x=0)
    FCB $FF,$FB,$F6          ; line 0: flag=-1, dy=-5, dx=-10
    FCB $FF,$00,$0A          ; line 1: flag=-1, dy=0, dx=10
    FCB $FF,$05,$00          ; closing line: flag=-1, dy=5, dx=0
    FCB 2                ; End marker

_ROCKET_BASE_PATH2:
    FCB 100              ; path2: intensity
    FCB $F1,$0F,0,0        ; path2: header (y=-15, x=15, next_y=0, next_x=0)
    FCB $FF,$FB,$0A          ; line 0: flag=-1, dy=-5, dx=10
    FCB $FF,$00,$F6          ; line 1: flag=-1, dy=0, dx=-10
    FCB $FF,$05,$00          ; closing line: flag=-1, dy=5, dx=0
    FCB 2                ; End marker

_ROCKET_BASE_PATH3:
    FCB 90              ; path3: intensity
    FCB $EC,$F6,0,0        ; path3: header (y=-20, x=-10, next_y=0, next_x=0)
    FCB $FF,$FB,$00          ; line 0: flag=-1, dy=-5, dx=0
    FCB $FF,$00,$14          ; line 1: flag=-1, dy=0, dx=20
    FCB $FF,$05,$00          ; line 2: flag=-1, dy=5, dx=0
    FCB $FF,$00,$EC          ; closing line: flag=-1, dy=0, dx=-20
    FCB 2                ; End marker

; Vector asset: astronaut
; Generated from astronaut.vec (Malban Draw_Sync_List format)
; Total paths: 6, points: 16

_ASTRONAUT_VECTORS:  ; Main entry
_ASTRONAUT_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0F,$F8,0,0        ; path0: header (y=15, x=-8, next_y=0, next_x=0)
    FCB $FF,$05,$00          ; line 0: flag=-1, dy=5, dx=0
    FCB $FF,$00,$10          ; line 1: flag=-1, dy=0, dx=16
    FCB $FF,$FB,$00          ; line 2: flag=-1, dy=-5, dx=0
    FCB $FF,$00,$F0          ; closing line: flag=-1, dy=0, dx=-16
    FCB 2                ; End marker

_ASTRONAUT_PATH1:
    FCB 127              ; path1: intensity
    FCB $0F,$FA,0,0        ; path1: header (y=15, x=-6, next_y=0, next_x=0)
    FCB $FF,$F1,$00          ; line 0: flag=-1, dy=-15, dx=0
    FCB $FF,$00,$0C          ; line 1: flag=-1, dy=0, dx=12
    FCB $FF,$0F,$00          ; line 2: flag=-1, dy=15, dx=0
    FCB $FF,$00,$F4          ; closing line: flag=-1, dy=0, dx=-12
    FCB 2                ; End marker

_ASTRONAUT_PATH2:
    FCB 100              ; path2: intensity
    FCB $0C,$FA,0,0        ; path2: header (y=12, x=-6, next_y=0, next_x=0)
    FCB $FF,$FC,$FA          ; line 0: flag=-1, dy=-4, dx=-6
    FCB 2                ; End marker

_ASTRONAUT_PATH3:
    FCB 100              ; path3: intensity
    FCB $0C,$06,0,0        ; path3: header (y=12, x=6, next_y=0, next_x=0)
    FCB $FF,$FC,$06          ; line 0: flag=-1, dy=-4, dx=6
    FCB 2                ; End marker

_ASTRONAUT_PATH4:
    FCB 100              ; path4: intensity
    FCB $00,$FC,0,0        ; path4: header (y=0, x=-4, next_y=0, next_x=0)
    FCB $FF,$F4,$00          ; line 0: flag=-1, dy=-12, dx=0
    FCB 2                ; End marker

_ASTRONAUT_PATH5:
    FCB 100              ; path5: intensity
    FCB $00,$04,0,0        ; path5: header (y=0, x=4, next_y=0, next_x=0)
    FCB $FF,$F4,$00          ; line 0: flag=-1, dy=-12, dx=0
    FCB 2                ; End marker

; String literals (classic FCC + $80 terminator)
STR_0:
    FCC "2025"
    FCB $80
STR_1:
    FCC "FUEL"
    FCB $80
VCUR_X EQU RESULT+54
VCUR_Y EQU RESULT+55
VLINE_DX EQU RESULT+56
VLINE_DY EQU RESULT+57
VLINE_STEPS EQU RESULT+58
VLINE_LIST EQU RESULT+59
DRAW_VEC_X EQU RESULT+61
DRAW_VEC_Y EQU RESULT+62
