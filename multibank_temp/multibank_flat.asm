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
    FDB $0000              ; Music pointer
    FCB $F8,$50,$20,$BB     ; Height, Width, Rel Y, Rel X
    FCC "Test Multi-Bank Variable"
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
;***************************************************************************
; ARRAY DATA (ROM literals)
;***************************************************************************
; Arrays are stored in ROM and accessed via pointers
; At startup, main() initializes VAR_{name} to point to ARRAY_{name}_DATA

; Array literal for variable 'my_array' (4 elements)
ARRAY_MY_ARRAY_DATA:
    FDB 127   ; Element 0
    FDB 100   ; Element 1
    FDB 80   ; Element 2
    FDB 60   ; Element 3


;***************************************************************************
; MAIN PROGRAM (Bank #0)
;***************************************************************************

MAIN:
    ; Initialize global variables
    LDD #10
    STD VAR_MY_VAR
    ; Copy array 'my_array' from ROM to RAM (4 elements)
    LDX #ARRAY_MY_ARRAY_DATA       ; Source: ROM array data
    LDU #VAR_MY_ARRAY_DATA       ; Dest: RAM array space
    LDD #4        ; Number of elements
.COPY_LOOP_0:
    LDY ,X++        ; Load word from ROM, increment source
    STY ,U++        ; Store word to RAM, increment dest
    SUBD #1         ; Decrement counter
    LBNE .COPY_LOOP_0 ; Loop until done (LBNE for long branch)
    LDX #VAR_MY_ARRAY_DATA    ; Array now in RAM
    STX VAR_MY_ARRAY
    ; === Initialize Joystick (one-time setup) ===
    JSR $F1AF    ; DP_to_C8 (required for RAM access)
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
    JSR WAIT_RECAL
    LDX #VAR_MY_ARRAY_DATA  ; Array data
    PSHS X
    LDD #0
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    STD VAR_VAL
    ; SET_INTENSITY: Set drawing intensity
    LDD VAR_VAL
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    RTS

;***************************************************************************
; EMBEDDED ASSETS (vectors, music, levels, SFX)
;***************************************************************************

; Generated from asym.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 3
; X bounds: min=0, max=20, width=20
; Center: (10, 10)

_ASYM_WIDTH EQU 20
_ASYM_CENTER_X EQU 10
_ASYM_CENTER_Y EQU 10

_ASYM_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _ASYM_PATH0        ; pointer to path 0

_ASYM_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $F6,$F6,0,0        ; path0: header (y=-10, x=-10, relative to center)
    FCB $FF,$14,$0A          ; line 0: flag=-1, dy=20, dx=10
    FCB $FF,$EC,$0A          ; line 1: flag=-1, dy=-20, dx=10
    FCB 2                ; End marker (path complete)

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



;***************************************************************************
; INTERRUPT VECTORS (Bank #31 Fixed Window)
;***************************************************************************
ORG $FFFE
FDB CUSTOM_RESET
