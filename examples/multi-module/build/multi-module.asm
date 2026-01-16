; VPy M6809 Assembly (Vectrex)
; ROM: 524288 bytes
; Multibank cartridge: 32 banks (16KB each)
; Helpers bank: 31 (fixed bank at $4000-$7FFF)

; === BANK 0 ===
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
    FCC "TEST"
    FCB $80                 ; String terminator
    FCB 0                   ; End of header

;***************************************************************************
; SYSTEM RAM VARIABLES
;***************************************************************************
CURRENT_ROM_BANK EQU $C880
RESULT EQU $CF00
TMPPTR EQU $CF02
TMPPTR2 EQU $CF04

;***************************************************************************
; USER VARIABLES
;***************************************************************************
VAR_PLAYER_SIZE EQU $CF10+0
VAR_INPUT_INPUT_RESULT EQU $CF10+2
VAR_PLAYER_X EQU $CF10+4
VAR_PLAYER_Y EQU $CF10+6

;***************************************************************************
; ARRAY DATA
;***************************************************************************
VAR_INPUT_INPUT_RESULT_DATA:
    FDB 0    ; Element 0
    FDB 0    ; Element 1

; Function argument slots
VAR_ARG0 EQU $CFE0+0
VAR_ARG1 EQU $CFE0+2
VAR_ARG2 EQU $CFE0+4
VAR_ARG3 EQU $CFE0+6
VAR_ARG4 EQU $CFE0+8

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
; MAIN PROGRAM
;***************************************************************************

MAIN:

.MAIN_LOOP:
    JSR LOOP_BODY
    BRA .MAIN_LOOP

LOOP_BODY:
    JSR Wait_Recal   ; Synchronize with screen refresh (mandatory)
    RTS

; Function: MAIN
MAIN:
    ; SET_INTENSITY: Set drawing intensity
    LDD #127
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    RTS

; Function: LOOP
LOOP:
    JSR INPUT_GET_INPUT
    LDD VAR_INPUT_INPUT_RESULT
    STD RESULT
    LDX RESULT  ; Array base address
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
    STD VAR_DX
    LDD VAR_INPUT_INPUT_RESULT
    STD RESULT
    LDX RESULT  ; Array base address
    PSHS X
    LDD #1
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    STD VAR_DY
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_DX
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_PLAYER_X
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_DY
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_PLAYER_Y
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #-100
    STD RESULT
    LDD RESULT
    CMPD ,S++
    BLT .CMP_TRUE
    LDD #0
    BRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    BEQ .IF_ELSE
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_PLAYER_X
    BRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #100
    STD RESULT
    LDD RESULT
    CMPD ,S++
    BGT .CMP_TRUE
    LDD #0
    BRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    BEQ .IF_ELSE
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_PLAYER_X
    BRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #-100
    STD RESULT
    LDD RESULT
    CMPD ,S++
    BLT .CMP_TRUE
    LDD #0
    BRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    BEQ .IF_ELSE
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_PLAYER_Y
    BRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #100
    STD RESULT
    LDD RESULT
    CMPD ,S++
    BGT .CMP_TRUE
    LDD #0
    BRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    BEQ .IF_ELSE
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_PLAYER_Y
    BRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_PLAYER_SIZE
    STD RESULT
    LDD RESULT
    SUBD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_PLAYER_SIZE
    STD RESULT
    LDD RESULT
    SUBD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_PLAYER_SIZE
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #2
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR MUL16   ; D = X * D
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_PLAYER_SIZE
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #2
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR MUL16   ; D = X * D
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR GRAPHICS_DRAW_BOX
    JSR INPUT_CHECK_FIRE
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    BEQ .CMP_TRUE
    LDD #0
    BRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    BEQ .IF_ELSE
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    JSR GRAPHICS_DRAW_CROSS
    BRA .IF_END
.IF_ELSE:
.IF_END:
    RTS

; Function: GRAPHICS_DRAW_BOX
GRAPHICS_DRAW_BOX:
    ; DRAW_VECTOR: Draw vector asset at position
    ; Asset: box
    LDD VAR_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD VAR_Y
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
    LDX #_BOX_PATH0  ; Load first path
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    RTS

; Function: GRAPHICS_DRAW_CROSS
GRAPHICS_DRAW_CROSS:
    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
    LDD VAR_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #10
    STD RESULT
    LDD RESULT
    SUBD ,S++
    STD RESULT
    LDD RESULT
    STD TMPPTR+0    ; x0
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD TMPPTR+2    ; y0
    LDD VAR_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #10
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD TMPPTR+4    ; x1
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    STD TMPPTR+6    ; y1
    LDD VAR_INTENSITY
    STD RESULT
    LDD RESULT
    STD TMPPTR+8    ; intensity
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD TMPPTR+0    ; x0
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #10
    STD RESULT
    LDD RESULT
    SUBD ,S++
    STD RESULT
    LDD RESULT
    STD TMPPTR+2    ; y0
    LDD VAR_X
    STD RESULT
    LDD RESULT
    STD TMPPTR+4    ; x1
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #10
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD TMPPTR+6    ; y1
    LDD VAR_INTENSITY
    STD RESULT
    LDD RESULT
    STD TMPPTR+8    ; intensity
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    RTS

; Function: INPUT_GET_INPUT
INPUT_GET_INPUT:
    JSR J1X_BUILTIN
    STD RESULT
    LDD RESULT
    STD VAR_INPUT_X
    JSR J1Y_BUILTIN
    STD RESULT
    LDD RESULT
    STD VAR_INPUT_Y
    LDD #0
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_INPUT_INPUT_RESULT_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD VAR_INPUT_X
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD #1
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_INPUT_INPUT_RESULT_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD VAR_INPUT_Y
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD #0
    STD RESULT
    RTS
    RTS

; Function: INPUT_CHECK_FIRE
INPUT_CHECK_FIRE:
    LDA $C811      ; Vec_Button_1_1 (transition bits)
    ANDA #$01      ; Test bit 0
    BEQ .J1B1_OFF
    LDD #1
    BRA .J1B1_END
.J1B1_OFF:
    LDD #0
.J1B1_END:
    STD RESULT
    RTS
    RTS


; === BANK 31 ===
    ORG $4000
    ; Fixed bank (always visible at $4000-$7FFF)
    ; Contains runtime helpers for all banks

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

MUL16:
    ; Multiply 16-bit X * D -> D
    ; Simple implementation (can be optimized)
    PSHS X,B,A
    LDD #0         ; Result accumulator
    LDX 2,S        ; Multiplier
.MUL16_LOOP:
    BEQ .MUL16_END
    ADDD ,S        ; Add multiplicand
    LEAX -1,X
    BRA .MUL16_LOOP
.MUL16_END:
    LEAS 4,S
    RTS

DIV16:
    ; Divide 16-bit X / D -> D
    ; Simple implementation
    PSHS X,D
    LDD #0         ; Quotient
.DIV16_LOOP:
    PSHS D         ; Save quotient
    LDD 4,S        ; Load dividend (after PSHS D)
    CMPD 2,S       ; Compare with divisor (after PSHS D)
    PULS D         ; Restore quotient
    BLT .DIV16_END
    ADDD #1        ; Increment quotient
    LDX 2,S
    PSHS D
    LDD 2,S        ; Divisor
    LEAX D,X       ; Subtract divisor
    STX 4,S
    PULS D
    BRA .DIV16_LOOP
.DIV16_END:
    LEAS 4,S
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

