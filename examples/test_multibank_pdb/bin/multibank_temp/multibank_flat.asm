; AUTO-GENERATED FLATTENED MULTIBANK ASM
; Banks: 32 | Bank size: 16384 bytes | Total: 524288 bytes

ORG $0000

;***************************************************************************
; DEFINE SECTION
;***************************************************************************
    INCLUDE "VECTREX.I"

; ===== BANK #00 (physical offset $00000) =====
; VPy M6809 Assembly (Vectrex)
; ROM: 524288 bytes
; Multibank cartridge: 32 banks (16KB each)
; Helpers bank: 31 (fixed bank at $4000-$7FFF)

; ================================================


    ORG $0000

;***************************************************************************
; DEFINE SECTION
;***************************************************************************

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


; ===== BANK #01 (physical offset $04000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #02 (physical offset $08000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #03 (physical offset $0C000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #04 (physical offset $10000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #05 (physical offset $14000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #06 (physical offset $18000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #07 (physical offset $1C000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #08 (physical offset $20000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #09 (physical offset $24000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #10 (physical offset $28000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #11 (physical offset $2C000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #12 (physical offset $30000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #13 (physical offset $34000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #14 (physical offset $38000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #15 (physical offset $3C000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #16 (physical offset $40000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #17 (physical offset $44000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #18 (physical offset $48000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #19 (physical offset $4C000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #20 (physical offset $50000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #21 (physical offset $54000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #22 (physical offset $58000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #23 (physical offset $5C000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #24 (physical offset $60000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #25 (physical offset $64000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #26 (physical offset $68000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #27 (physical offset $6C000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #28 (physical offset $70000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #29 (physical offset $74000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #30 (physical offset $78000) =====

    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================


; ===== BANK #31 (physical offset $7C000) =====
    ORG $4000  ; Fixed bank window (runtime helpers + interrupt vectors)


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



;***************************************************************************
; INTERRUPT VECTORS (Bank #31 Fixed Window)
;***************************************************************************
ORG $FFFE
FDB CUSTOM_RESET
