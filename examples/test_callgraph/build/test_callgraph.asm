; VPy M6809 Assembly (Vectrex)
; ROM: 524288 bytes
; Multibank cartridge: 32 banks (16KB each)
; Helpers bank: 31 (fixed bank at $4000-$7FFF)

; ================================================
; BANK #0 - Entry point and main code
; ================================================

    ORG $0000

;***************************************************************************
; DEFINE SECTION
;***************************************************************************
    INCLUDE "VECTREX.I"

;***************************************************************************
; CARTRIDGE HEADER
;***************************************************************************
    FCC "g GCE 2025"
    FCB $80                 ; String terminator
    FDB $0000              ; Music pointer
    FCB $F8,$50,$20,$BB     ; Height, Width, Rel Y, Rel X
    FCC "Call Graph Test"
    FCB $80                 ; String terminator
    FCB 0                   ; End of header

;***************************************************************************
; CODE SECTION
;***************************************************************************

START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS
    CLR $C80E        ; Initialize Vec_Prev_Btns
    LDA #$80
    STA VIA_t1_cnt_lo
    LDS #$CBFF       ; Initialize stack
; Bank 0 ($0000) is active; fixed bank 31 ($4000-$7FFF) always visible
    JMP MAIN

;***************************************************************************
; === RAM VARIABLE DEFINITIONS ===
;***************************************************************************
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPPTR               EQU $C880+$02   ; Temporary pointer (2 bytes)
TMPPTR2              EQU $C880+$04   ; Temporary pointer 2 (2 bytes)
TEMP_YX              EQU $C880+$06   ; Temporary Y/X coordinate storage (2 bytes)
VLINE_DX_16          EQU $C880+$08   ; DRAW_LINE dx (16-bit) (2 bytes)
VLINE_DY_16          EQU $C880+$0A   ; DRAW_LINE dy (16-bit) (2 bytes)
VLINE_DX             EQU $C880+$0C   ; DRAW_LINE dx clamped (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$0D   ; DRAW_LINE dy clamped (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$0E   ; DRAW_LINE remaining dy for segment 2 (1 bytes)
VAR_ENEMY2_X         EQU $C880+$0F   ; User variable: ENEMY2_X (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$11   ; User variable: ENEMY3_Y (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$13   ; User variable: FRAME_COUNT (2 bytes)
VAR_ENEMY1_X         EQU $C880+$15   ; User variable: ENEMY1_X (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$17   ; User variable: ENEMY1_Y (2 bytes)
VAR_A                EQU $C880+$19   ; User variable: A (2 bytes)
VAR_ENEMY3_X         EQU $C880+$1B   ; User variable: ENEMY3_X (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$1D   ; User variable: ENEMY2_Y (2 bytes)
VAR_ARG0             EQU $CFE0   ; Function argument 0 (16-bit) (2 bytes)
VAR_ARG1             EQU $CFE2   ; Function argument 1 (16-bit) (2 bytes)
VAR_ARG2             EQU $CFE4   ; Function argument 2 (16-bit) (2 bytes)
VAR_ARG3             EQU $CFE6   ; Function argument 3 (16-bit) (2 bytes)
VAR_ARG4             EQU $CFE8   ; Function argument 4 (16-bit) (2 bytes)

; Internal builtin variables (aliases to RESULT slots)
DRAW_VEC_X EQU RESULT+0
DRAW_VEC_Y EQU RESULT+2
MIRROR_X EQU RESULT+4
MIRROR_Y EQU RESULT+6
DRAW_VEC_INTENSITY EQU RESULT+8


;***************************************************************************
; MAIN PROGRAM
;***************************************************************************

MAIN:
    ; Initialize global variables
    LDD #-50
    STD VAR_ENEMY1_X
    LDD #60
    STD VAR_ENEMY1_Y
    LDD #0
    STD VAR_ENEMY2_X
    LDD #0
    STD VAR_ENEMY2_Y
    LDD #50
    STD VAR_ENEMY3_X
    LDD #-60
    STD VAR_ENEMY3_Y
    LDD #0
    STD VAR_FRAME_COUNT
    ; Call main() for initialization
    ; SET_INTENSITY: Set drawing intensity
    LDD #127
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    ; TODO: Statement Pass { source_line: 21 }

.MAIN_LOOP:
    JSR LOOP_BODY
    BRA .MAIN_LOOP

LOOP_BODY:
    JSR Wait_Recal   ; Synchronize with screen refresh (mandatory)
    JSR Reset0Ref    ; Reset beam to center (0,0)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    LDD #12
    STD RESULT
    LDD RESULT
    STD VAR_A
    LDD VAR_A
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #15
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_A
    ; SET_INTENSITY: Set drawing intensity
    LDD #100
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    LDD VAR_A
    STD RESULT
    ; DEBUG_PRINT(A)
    LDD RESULT
    STA $C002
    STB $C000
    LDA #$FE
    STA $C001
    LDX #DEBUG_LABEL_A
    STX $C004
    BRA .DEBUG_0
DEBUG_LABEL_A:
    FCC "A"
    FCB $00
.DEBUG_0:
    LDD #0
    STD RESULT
    JSR UPDATE_PLAYER
    JSR UPDATE_ENEMIES
    JSR DRAW_ALL
    LDD VAR_A
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #15
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_A
    LDD VAR_FRAME_COUNT
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_FRAME_COUNT
    RTS

; Function: UPDATE_PLAYER
UPDATE_PLAYER:
    JSR CHECK_INPUT
    JSR MOVE_PLAYER
    RTS

; Function: CHECK_INPUT
CHECK_INPUT:
    ; TODO: Statement Pass { source_line: 48 }
    RTS

; Function: MOVE_PLAYER
MOVE_PLAYER:
    ; TODO: Statement Pass { source_line: 52 }
    RTS

; Function: UPDATE_ENEMIES
UPDATE_ENEMIES:
    LDD VAR_ENEMY1_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_ENEMY1_X
    LDD VAR_ENEMY1_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #100
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBGT .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_ENEMY1_X
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_ENEMY2_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_ENEMY2_Y
    LDD VAR_ENEMY2_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #100
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBGT .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_ENEMY2_Y
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_ENEMY3_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    SUBD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_ENEMY3_X
    LDD VAR_ENEMY3_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    SUBD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_ENEMY3_Y
    LDD VAR_ENEMY3_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #-100
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLT .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ENEMY3_X
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_ENEMY3_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #-100
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLT .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ENEMY3_Y
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    RTS

; Function: DRAW_ALL
DRAW_ALL:
    JSR DRAW_PLAYER
    JSR DRAW_ENEMIES
    RTS

; Function: DRAW_PLAYER
DRAW_PLAYER:
    ; DRAW_VECTOR: Draw vector asset at position
    ; Asset: player
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD #0
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA TMPPTR+1  ; Save Y to temporary storage
    LDA TMPPTR    ; X position
    STA DRAW_VEC_X
    LDA TMPPTR+1  ; Y position
    STA DRAW_VEC_Y
    CLR MIRROR_X
    CLR MIRROR_Y
    CLR DRAW_VEC_INTENSITY  ; Use intensity from vector data
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_PLAYER_PATH0  ; Load first path
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    RTS

; Function: DRAW_ENEMIES
DRAW_ENEMIES:
    ; DRAW_VECTOR: Draw vector asset at position
    ; Asset: enemy
    LDD VAR_ENEMY1_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD VAR_ENEMY1_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA TMPPTR+1  ; Save Y to temporary storage
    LDA TMPPTR    ; X position
    STA DRAW_VEC_X
    LDA TMPPTR+1  ; Y position
    STA DRAW_VEC_Y
    CLR MIRROR_X
    CLR MIRROR_Y
    CLR DRAW_VEC_INTENSITY  ; Use intensity from vector data
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_ENEMY_PATH0  ; Load first path
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    ; DRAW_VECTOR: Draw vector asset at position
    ; Asset: enemy
    LDD VAR_ENEMY2_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD VAR_ENEMY2_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA TMPPTR+1  ; Save Y to temporary storage
    LDA TMPPTR    ; X position
    STA DRAW_VEC_X
    LDA TMPPTR+1  ; Y position
    STA DRAW_VEC_Y
    CLR MIRROR_X
    CLR MIRROR_Y
    CLR DRAW_VEC_INTENSITY  ; Use intensity from vector data
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_ENEMY_PATH0  ; Load first path
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    ; DRAW_VECTOR: Draw vector asset at position
    ; Asset: enemy
    LDD VAR_ENEMY3_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD VAR_ENEMY3_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA TMPPTR+1  ; Save Y to temporary storage
    LDA TMPPTR    ; X position
    STA DRAW_VEC_X
    LDA TMPPTR+1  ; Y position
    STA DRAW_VEC_Y
    CLR MIRROR_X
    CLR MIRROR_Y
    CLR DRAW_VEC_INTENSITY  ; Use intensity from vector data
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_ENEMY_PATH0  ; Load first path
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    RTS


; ================================================
; BANK #1 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #2 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #3 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #4 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #5 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #6 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #7 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #8 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #9 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #10 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #11 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #12 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #13 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #14 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #15 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #16 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #17 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #18 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #19 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #20 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #21 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #22 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #23 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #24 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #25 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #26 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #27 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #28 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #29 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #30 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #31 - 0 function(s) [HELPERS ONLY]
; ================================================
    ORG $4000  ; Fixed bank (always visible at $4000-$7FFF)
    ; Runtime helpers (accessible from all banks)

;***************************************************************************
; RUNTIME HELPERS
;***************************************************************************

MOD16:
    ; Modulo 16-bit X % D -> D
    PSHS X,D
.MOD16_LOOP:
    PSHS D         ; Save D
    LDD 4,S        ; Load dividend (after PSHS D)
    CMPD 2,S       ; Compare with divisor (after PSHS D)
    PULS D         ; Restore D
    BLT .MOD16_END
    LDX 2,S
    LDD ,S
    LEAX D,X
    STX 2,S
    BRA .MOD16_LOOP
.MOD16_END:
    LDD 2,S        ; Remainder
    LEAS 4,S
    RTS

Draw_Sync_List_At_With_Mirrors:
; Unified mirror support using flags: MIRROR_X and MIRROR_Y
; Conditionally negates X and/or Y coordinates and deltas
; NOTE: Caller must ensure DP=$D0 for VIA access
LDA DRAW_VEC_INTENSITY  ; Check if intensity override is set
BNE DSWM_USE_OVERRIDE   ; If non-zero, use override
LDA ,X+                 ; Otherwise, read intensity from vector data
BRA DSWM_SET_INTENSITY
DSWM_USE_OVERRIDE:
LEAX 1,X                ; Skip intensity byte in vector data
DSWM_SET_INTENSITY:
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
LBRA DSWM_LOOP          ; Long branch
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
LBRA DSWM_LOOP          ; Long branch
DSWM_DONE:
LDA #$C8                ; CRITICAL: Restore DP to $C8 for RAM access
TFR A,DP
RTS
;***************************************************************************
; EMBEDDED ASSETS (vectors, music, levels, SFX)
;***************************************************************************

; Generated from player.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 3
; X bounds: min=-15, max=15, width=30
; Center: (0, 5)

_PLAYER_WIDTH EQU 30
_PLAYER_CENTER_X EQU 0
_PLAYER_CENTER_Y EQU 5

_PLAYER_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _PLAYER_PATH0        ; pointer to path 0

_PLAYER_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0F,$00,0,0        ; path0: header (y=15, x=0, relative to center)
    FCB $FF,$E2,$F1          ; line 0: flag=-1, dy=-30, dx=-15
    FCB $FF,$00,$1E          ; line 1: flag=-1, dy=0, dx=30
    FCB $FF,$1E,$F1          ; closing line: flag=-1, dy=30, dx=-15
    FCB 2                ; End marker (path complete)
; Generated from enemy.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 4
; X bounds: min=-10, max=10, width=20
; Center: (0, 0)

_ENEMY_WIDTH EQU 20
_ENEMY_CENTER_X EQU 0
_ENEMY_CENTER_Y EQU 0

_ENEMY_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _ENEMY_PATH0        ; pointer to path 0

_ENEMY_PATH0:    ; Path 0
    FCB 100              ; path0: intensity
    FCB $0A,$F6,0,0        ; path0: header (y=10, x=-10, relative to center)
    FCB $FF,$00,$14          ; line 0: flag=-1, dy=0, dx=20
    FCB $FF,$EC,$00          ; line 1: flag=-1, dy=-20, dx=0
    FCB $FF,$00,$EC          ; line 2: flag=-1, dy=0, dx=-20
    FCB $FF,$14,$00          ; closing line: flag=-1, dy=20, dx=0
    FCB 2                ; End marker (path complete)
