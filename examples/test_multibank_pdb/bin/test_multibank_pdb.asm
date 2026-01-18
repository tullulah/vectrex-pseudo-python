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
    FDB music1              ; Music pointer
    FCB $F8,$50,$20,$BB     ; Height, Width, Rel Y, Rel X
    FCC "MULTIBANK PDB TEST"
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
    ; Call main() for initialization
    ; SET_INTENSITY: Set drawing intensity
    LDD #127
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT

.MAIN_LOOP:
    JSR LOOP_BODY
    BRA .MAIN_LOOP

LOOP_BODY:
    JSR Wait_Recal   ; Synchronize with screen refresh (mandatory)
    JSR Reset0Ref    ; Reset beam to center (0,0)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; PRINT_TEXT: Print text at position
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_68624562      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    ; PRINT_TEXT: Print text at position
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_82781042      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
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

VECTREX_PRINT_TEXT:
    ; VPy signature: PRINT_TEXT(x, y, string)
    ; BIOS signature: Print_Str_d(A=Y, B=X, U=string)
    JSR $F1AA      ; DP_to_D0 - set Direct Page for BIOS/VIA access
    LDU VAR_ARG2   ; string pointer (third parameter)
    LDA VAR_ARG1+1 ; Y coordinate (second parameter, low byte)
    LDB VAR_ARG0+1 ; X coordinate (first parameter, low byte)
    JSR Print_Str_d ; Print string from U register
    JSR $F1AF      ; DP_to_C8 - restore DP before return
    RTS

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

;**** PRINT_TEXT String Data ****
PRINT_TEXT_STR_68624562:
    FCC "HELLO"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_82781042:
    FCC "WORLD"
    FCB $80          ; Vectrex string terminator

