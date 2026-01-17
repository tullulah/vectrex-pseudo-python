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
    FCC "TEST"
    FCB $80                 ; String terminator
    FCB 0                   ; End of header

;***************************************************************************
; SYSTEM RAM VARIABLES
;***************************************************************************
RESULT EQU $CF00
TMPPTR EQU $CF02

; DRAW_LINE helper variables (16-bit deltas + 8-bit clamped + remaining)
VLINE_DX_16 EQU $CF19         ; 16-bit dx (2 bytes)
VLINE_DY_16 EQU $CF1B         ; 16-bit dy (2 bytes)
VLINE_DX EQU $CF1D            ; 8-bit clamped dx (1 byte)
VLINE_DY EQU $CF1E            ; 8-bit clamped dy (1 byte)
VLINE_DY_REMAINING EQU $CF1F  ; Remaining dy for segment 2 (2 bytes)

; Function argument slots (16-bit each, 3 slots = 6 bytes)
VAR_ARG0 EQU $CFE0+0
VAR_ARG1 EQU $CFE0+2
VAR_ARG2 EQU $CFE0+4

;***************************************************************************
; USER VARIABLES
;***************************************************************************
VAR_INTENSITY EQU $CF10+0
VAR_y EQU $CF10+2
VAR_height EQU $CF10+4
VAR_INPUT_INPUT_RESULT EQU $CF10+6
VAR_x EQU $CF10+8
VAR_intensity EQU $CF10+10
VAR_size EQU $CF10+12
VAR_width EQU $CF10+14

;***************************************************************************
; ARRAY DATA
;***************************************************************************
VAR_INPUT_INPUT_RESULT_DATA EQU $CF20

; Array data storage
    ORG $CF20  ; Start of array data section
; Array: VAR_INPUT_INPUT_RESULT_DATA
    FDB 0    ; Element 0
    FDB 0    ; Element 1

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

; Function: GRAPHICS_DRAW_BOX
GRAPHICS_DRAW_BOX:
    ; SET_INTENSITY: Set drawing intensity
    LDD VAR_INTENSITY
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
    LDD #-10
    STD RESULT
    LDD RESULT
    STD TMPPTR+0    ; x0
    LDD #-10
    STD RESULT
    LDD RESULT
    STD TMPPTR+2    ; y0
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPPTR+4    ; x1
    LDD #-10
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
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPPTR+0    ; x0
    LDD #-10
    STD RESULT
    LDD RESULT
    STD TMPPTR+2    ; y0
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPPTR+4    ; x1
    LDD #10
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
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPPTR+0    ; x0
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPPTR+2    ; y0
    LDD #-10
    STD RESULT
    LDD RESULT
    STD TMPPTR+4    ; x1
    LDD #10
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
    LDD #-10
    STD RESULT
    LDD RESULT
    STD TMPPTR+0    ; x0
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPPTR+2    ; y0
    LDD #-10
    STD RESULT
    LDD RESULT
    STD TMPPTR+4    ; x1
    LDD #-10
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

; Function: GRAPHICS_DRAW_CROSS
GRAPHICS_DRAW_CROSS:
    ; SET_INTENSITY: Set drawing intensity
    LDD VAR_INTENSITY
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
    LDD #-10
    STD RESULT
    LDD RESULT
    STD TMPPTR+0    ; x0
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPPTR+2    ; y0
    LDD #10
    STD RESULT
    LDD RESULT
    STD TMPPTR+4    ; x1
    LDD #0
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
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPPTR+0    ; x0
    LDD #-10
    STD RESULT
    LDD RESULT
    STD TMPPTR+2    ; y0
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPPTR+4    ; x1
    LDD #10
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
    JSR J1X_BUILTIN
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
    JSR J1Y_BUILTIN
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD #0
    STD RESULT
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

J1X_BUILTIN:
    ; Read J1_X from $CF00 and return -1/0/+1
    LDB $CF00      ; Joy_1_X (unsigned byte 0-255)
    CMPB #108      ; Compare with lower threshold
    BLO .J1X_LEFT  ; Branch if <108 (left)
    CMPB #148      ; Compare with upper threshold
    BHI .J1X_RIGHT ; Branch if >148 (right)
    ; Center (108-148)
    LDD #0
    RTS
.J1X_LEFT:
    LDD #-1
    RTS
.J1X_RIGHT:
    LDD #1
    RTS

J1Y_BUILTIN:
    ; Read J1_Y from $CF01 and return -1/0/+1
    LDB $CF01      ; Joy_1_Y (unsigned byte 0-255)
    CMPB #108      ; Compare with lower threshold
    BLO .J1Y_DOWN  ; Branch if <108 (down)
    CMPB #148      ; Compare with upper threshold
    BHI .J1Y_UP    ; Branch if >148 (up)
    ; Center (108-148)
    LDD #0
    RTS
.J1Y_DOWN:
    LDD #-1
    RTS
.J1Y_UP:
    LDD #1
    RTS

DRAW_LINE_WRAPPER:
    ; Line drawing wrapper with segmentation for lines > 127 pixels
    ; Args: TMPPTR+0=x0, TMPPTR+2=y0, TMPPTR+4=x1, TMPPTR+6=y1, TMPPTR+8=intensity
    ; Calculate deltas (16-bit signed)
    LDD TMPPTR+4    ; x1
    SUBD TMPPTR+0   ; x1 - x0
    STD VLINE_DX_16 ; Store 16-bit dx
    
    LDD TMPPTR+6    ; y1
    SUBD TMPPTR+2   ; y1 - y0
    STD VLINE_DY_16 ; Store 16-bit dy
    
    ; === SEGMENT 1: Clamp deltas to ±127 ===
    ; Check dy: if > 127, clamp to 127; if < -128, clamp to -128
    LDD VLINE_DY_16
    CMPD #127       ; Compare with max positive
    LBLE DLW_SEG1_DY_LO ; Branch if <= 127
    LDD #127        ; Clamp to 127
    STD VLINE_DY_16
DLW_SEG1_DY_LO:
    LDD VLINE_DY_16
    CMPD #-128      ; Compare with min negative
    LBGE DLW_SEG1_DY_READY ; Branch if >= -128
    LDD #-128       ; Clamp to -128
    STD VLINE_DY_16
DLW_SEG1_DY_READY:
    LDB VLINE_DY_16+1 ; Load low byte (8-bit clamped)
    STB VLINE_DY
    
    ; Check dx: if > 127, clamp to 127; if < -128, clamp to -128
    LDD VLINE_DX_16
    CMPD #127
    LBLE DLW_SEG1_DX_LO
    LDD #127
    STD VLINE_DX_16
DLW_SEG1_DX_LO:
    LDD VLINE_DX_16
    CMPD #-128
    LBGE DLW_SEG1_DX_READY
    LDD #-128
    STD VLINE_DX_16
DLW_SEG1_DX_READY:
    LDB VLINE_DX_16+1 ; Load low byte (8-bit clamped)
    STB VLINE_DX
    
    ; Set intensity
    LDA TMPPTR+8+1  ; Load intensity (low byte)
    JSR Intensity_a
    
    ; Move to start position (x0, y0)
    CLR Vec_Misc_Count
    LDA TMPPTR+2+1  ; y0 (low byte)
    LDB TMPPTR+0+1  ; x0 (low byte)
    JSR Moveto_d
    
    ; Draw first segment (clamped deltas)
    LDA VLINE_DY    ; 8-bit clamped dy
    LDB VLINE_DX    ; 8-bit clamped dx
    JSR Draw_Line_d
    
    ; === CHECK IF SEGMENT 2 NEEDED ===
    ; Original dy still in VLINE_DY_16, check if exceeds ±127
    LDD TMPPTR+6    ; Reload original y1
    SUBD TMPPTR+2   ; y1 - y0
    CMPD #127
    LBGT DLW_NEED_SEG2 ; dy > 127
    CMPD #-128
    LBLT DLW_NEED_SEG2 ; dy < -128
    LBRA DLW_DONE   ; No second segment needed
    
DLW_NEED_SEG2:
    ; Calculate remaining dy
    LDD TMPPTR+6    ; y1
    SUBD TMPPTR+2   ; y1 - y0
    ; Check sign: if positive, subtract 127; if negative, add 128
    CMPD #0
    LBGE DLW_SEG2_DY_POS
    ADDD #128       ; dy was negative, add 128
    LBRA DLW_SEG2_DY_DONE
DLW_SEG2_DY_POS:
    SUBD #127       ; dy was positive, subtract 127
DLW_SEG2_DY_DONE:
    STD VLINE_DY_REMAINING
    
    ; Draw second segment (remaining dy, dx=0)
    LDA VLINE_DY_REMAINING ; Low byte of remaining (it's already 8-bit)
    LDB #0          ; dx = 0 for vertical segment
    JSR Draw_Line_d
    
DLW_DONE:
    RTS

;**** PRINT_TEXT String Data ****
PRINT_TEXT_STR_2223292:
    FCC "HOLA"
    FCB $80          ; Vectrex string terminator

