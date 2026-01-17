; VPy M6809 Assembly (Vectrex)
; ROM: 32768 bytes

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
    FCC "TEST 1"
    FCB $80                 ; String terminator
    FCB 0                   ; End of header

;***************************************************************************
; SYSTEM RAM VARIABLES
;***************************************************************************
CURRENT_ROM_BANK EQU $C880
RESULT EQU $CF00
TMPPTR EQU $CF02
TMPPTR2 EQU $CF04
NUM_STR EQU $CF06   ; 2-byte buffer for PRINT_NUMBER hex output
RAND_SEED EQU $CF08 ; 2-byte random seed for RAND()

; Drawing builtins parameters (bytes in RAM)
DRAW_CIRCLE_XC EQU $CF0A
DRAW_CIRCLE_YC EQU $CF0B
DRAW_CIRCLE_DIAM EQU $CF0C
DRAW_CIRCLE_INTENSITY EQU $CF0D
DRAW_CIRCLE_TEMP EQU $CF0E ; 6 bytes for runtime calculations

DRAW_RECT_X EQU $CF14
DRAW_RECT_Y EQU $CF15
DRAW_RECT_WIDTH EQU $CF16
DRAW_RECT_HEIGHT EQU $CF17
DRAW_RECT_INTENSITY EQU $CF18

; DRAW_LINE_WRAPPER variables
VLINE_DX_16 EQU $CF19         ; 16-bit dx (2 bytes)
VLINE_DY_16 EQU $CF1B         ; 16-bit dy (2 bytes)
VLINE_DX EQU $CF1D            ; 8-bit clamped dx (1 byte)
VLINE_DY EQU $CF1E            ; 8-bit clamped dy (1 byte)
VLINE_DY_REMAINING EQU $CF1F  ; Remaining dy for segment 2 (2 bytes)

; Level system variables
LEVEL_PTR EQU $CF20           ; Pointer to current level data (2 bytes)
LEVEL_WIDTH EQU $CF22          ; Level width in tiles (1 byte)
LEVEL_HEIGHT EQU $CF23         ; Level height in tiles (1 byte)
LEVEL_TILE_SIZE EQU $CF24      ; Tile size in pixels (1 byte)

; Utilities variables
FRAME_COUNTER EQU $CF26        ; Frame counter (2 bytes)
CURRENT_INTENSITY EQU $CF28    ; Current intensity for fade effects (1 byte)

; Function argument slots
VAR_ARG0 EQU $CFE0+0
VAR_ARG1 EQU $CFE0+2
VAR_ARG2 EQU $CFE0+4
VAR_ARG3 EQU $CFE0+6
VAR_ARG4 EQU $CFE0+8

; Internal builtin variables (aliases to RESULT slots)
DRAW_VEC_X EQU RESULT+0
DRAW_VEC_Y EQU RESULT+2
MIRROR_X EQU RESULT+4
MIRROR_Y EQU RESULT+6
DRAW_VEC_INTENSITY EQU RESULT+8

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
    JMP MAIN

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
    ; PRINT_TEXT: Print text at position
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_2223292      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    RTS

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

;**** PRINT_TEXT String Data ****
PRINT_TEXT_STR_2223292:
    FCC "HOLA"
    FCB $80          ; Vectrex string terminator


;***************************************************************************
; INTERRUPT VECTORS ($FFF0-$FFFF)
;***************************************************************************

    ORG $FFF0
    FDB $0000      ; Reserved
    FDB $0000      ; SWI3
    FDB $0000      ; SWI2
    FDB $0000      ; FIRQ
    FDB $0000      ; IRQ
    FDB $0000      ; SWI
    FDB $0000      ; NMI
    FDB START      ; RESET vector - entry point
