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
    FCC "TEST INCREMENTAL"
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
    JMP MAIN

;***************************************************************************
; === RAM VARIABLE DEFINITIONS ===
;***************************************************************************
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPPTR               EQU $C880+$02   ; Temporary pointer (2 bytes)
TMPPTR2              EQU $C880+$04   ; Temporary pointer 2 (2 bytes)
TEMP_YX              EQU $C880+$06   ; Temporary Y/X coordinate storage (2 bytes)
NUM_STR              EQU $C880+$08   ; Buffer for PRINT_NUMBER hex output (2 bytes)
DRAW_LINE_ARGS       EQU $C880+$0A   ; DRAW_LINE argument buffer (x0,y0,x1,y1,intensity) (10 bytes)
VLINE_DX_16          EQU $C880+$14   ; DRAW_LINE dx (16-bit) (2 bytes)
VLINE_DY_16          EQU $C880+$16   ; DRAW_LINE dy (16-bit) (2 bytes)
VLINE_DX             EQU $C880+$18   ; DRAW_LINE dx clamped (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$19   ; DRAW_LINE dy clamped (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$1A   ; DRAW_LINE remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$1C   ; DRAW_LINE remaining dx for segment 2 (16-bit) (2 bytes)
VAR_POS_X            EQU $C880+$1E   ; User variable: POS_X (2 bytes)
VAR_COUNTER          EQU $C880+$20   ; User variable: COUNTER (2 bytes)
VAR_BTN              EQU $C880+$22   ; User variable: BTN (2 bytes)
VAR_JOY_X            EQU $C880+$24   ; User variable: JOY_X (2 bytes)
VAR_POS_Y            EQU $C880+$26   ; User variable: POS_Y (2 bytes)
VAR_JOY_Y            EQU $C880+$28   ; User variable: JOY_Y (2 bytes)
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
    LDD #0
    STD VAR_COUNTER
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
    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
    LDD #-40
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+0    ; x0
    LDD #50
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+2    ; y0
    LDD #40
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+4    ; x1
    LDD #50
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+6    ; y1
    LDD #100
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+8    ; intensity
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
    LDD #-50
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+0    ; x0
    LDD #-40
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+2    ; y0
    LDD #-50
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+4    ; x1
    LDD #40
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+6    ; y1
    LDD #100
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+8    ; intensity
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
    LDD #0
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+0    ; x0
    LDD #-30
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+2    ; y0
    LDD #30
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+4    ; x1
    LDD #0
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+6    ; y1
    LDD #127
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+8    ; intensity
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; PRINT_TEXT: Print text at position
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_3189042615754409911      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    ; PRINT_NUMBER(x, y, num)
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0    ; X position
    LDD #-60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1    ; Y position
    LDD VAR_COUNTER
    STD RESULT
    LDD RESULT
    STD VAR_ARG2    ; Number value
    JSR VECTREX_PRINT_NUMBER
    LDD #0
    STD RESULT
    LDD VAR_COUNTER
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_COUNTER
    LDD VAR_COUNTER
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #999
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
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_COUNTER
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    JSR J1X_BUILTIN
    STD RESULT
    LDD RESULT
    STD VAR_JOY_X
    JSR J1Y_BUILTIN
    STD RESULT
    LDD RESULT
    STD VAR_JOY_Y
    ; TODO: Expr Logic { op: Or, left: Compare { op: Ne, left: Ident(IdentInfo { name: "JOY_X", source_line: 33, col: 3 }), right: Number(0) }, right: Compare { op: Ne, left: Ident(IdentInfo { name: "JOY_Y", source_line: 33, col: 17 }), right: Number(0) } }
    LDD #0
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_JOY_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #20
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR MUL16   ; D = X * D
    STD RESULT
    LDD RESULT
    STD VAR_POS_X
    LDD VAR_JOY_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #20
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR MUL16   ; D = X * D
    STD RESULT
    LDD RESULT
    STD VAR_POS_Y
    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
    LDD VAR_POS_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #5
    STD RESULT
    LDD RESULT
    SUBD ,S++
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+0    ; x0
    LDD VAR_POS_Y
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+2    ; y0
    LDD VAR_POS_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #5
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+4    ; x1
    LDD VAR_POS_Y
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+6    ; y1
    LDD #80
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+8    ; intensity
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
    LDD VAR_POS_X
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+0    ; x0
    LDD VAR_POS_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #5
    STD RESULT
    LDD RESULT
    SUBD ,S++
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+2    ; y0
    LDD VAR_POS_X
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+4    ; x1
    LDD VAR_POS_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #5
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+6    ; y1
    LDD #80
    STD RESULT
    LDD RESULT
    STD DRAW_LINE_ARGS+8    ; intensity
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDA $C811      ; Vec_Button_1_1 (transition bits)
    ANDA #$01      ; Test bit 0
    BEQ .J1B1_OFF
    LDD #1
    BRA .J1B1_END
.J1B1_OFF:
    LDD #0
.J1B1_END:
    STD RESULT
    LDD RESULT
    STD VAR_BTN
    LDD VAR_BTN
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    ; PRINT_TEXT: Print text at position
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_66902187      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    LBRA .IF_END
.IF_ELSE:
.IF_END:
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

VECTREX_PRINT_NUMBER:
    ; VPy signature: PRINT_NUMBER(x, y, num)
    ; Convert number to hex string and print
    JSR $F1AA      ; DP_to_D0 - set Direct Page for BIOS/VIA access
    LDA VAR_ARG1+1   ; Y position
    LDB VAR_ARG0+1   ; X position
    JSR Moveto_d     ; Move to position
    
    ; Convert number to string (show low byte as hex)
    LDA VAR_ARG2+1   ; Load number value
    
    ; Convert high nibble to ASCII
    LSRA
    LSRA
    LSRA
    LSRA
    ANDA #$0F
    CMPA #10
    BLO PN_DIGIT1
    ADDA #7          ; A-F
PN_DIGIT1:
    ADDA #'0'
    STA NUM_STR      ; Store first digit
    
    ; Convert low nibble to ASCII  
    LDA VAR_ARG2+1
    ANDA #$0F
    CMPA #10
    BLO PN_DIGIT2
    ADDA #7          ; A-F
PN_DIGIT2:
    ADDA #'0'
    ORA #$80         ; Set high bit for string termination
    STA NUM_STR+1    ; Store second digit with high bit
    
    ; Print the string
    LDU #NUM_STR     ; Point to our number string
    JSR Print_Str_d  ; Print using BIOS
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

; DRAW_LINE unified wrapper - handles 16-bit signed coordinates
; Args: DRAW_LINE_ARGS+0=x0, +2=y0, +4=x1, +6=y1, +8=intensity
; ALWAYS sets intensity. Does NOT reset origin (allows connected lines).
DRAW_LINE_WRAPPER:
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode for vector drawing)
    STA >$D00C     ; VIA_cntl
    ; Set DP to hardware registers
    LDA #$D0
    TFR A,DP
    ; ALWAYS set intensity (no optimization)
    LDA DRAW_LINE_ARGS+8+1  ; intensity (low byte of 16-bit value)
    JSR Intensity_a
    ; Move to start ONCE (y in A, x in B) - use low bytes (8-bit signed -127..+127)
    LDA DRAW_LINE_ARGS+2+1  ; Y start (low byte of 16-bit value)
    LDB DRAW_LINE_ARGS+0+1  ; X start (low byte of 16-bit value)
    JSR Moveto_d
    ; Compute deltas using 16-bit arithmetic
    ; dx = x1 - x0 (treating as signed 16-bit)
    LDD DRAW_LINE_ARGS+4    ; x1 (16-bit)
    SUBD DRAW_LINE_ARGS+0   ; subtract x0 (16-bit)
    STD VLINE_DX_16 ; Store full 16-bit dx
    ; dy = y1 - y0 (treating as signed 16-bit)
    LDD DRAW_LINE_ARGS+6    ; y1 (16-bit)
    SUBD DRAW_LINE_ARGS+2   ; subtract y0 (16-bit)
    STD VLINE_DY_16 ; Store full 16-bit dy
    ; SEGMENT 1: Clamp dy to ±127 and draw
    LDD VLINE_DY_16 ; Load full dy
    CMPD #127
    BLE DLW_SEG1_DY_LO
    LDA #127        ; dy > 127: use 127
    BRA DLW_SEG1_DY_READY
DLW_SEG1_DY_LO:
    CMPD #-128
    BGE DLW_SEG1_DY_NO_CLAMP  ; -128 <= dy <= 127: use original (sign-extended)
    LDA #$80        ; dy < -128: use -128
    BRA DLW_SEG1_DY_READY
DLW_SEG1_DY_NO_CLAMP:
    LDA VLINE_DY_16+1  ; Use original low byte (already in valid range)
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
    BGE DLW_SEG1_DX_NO_CLAMP  ; -128 <= dx <= 127: use original (sign-extended)
    LDB #$80        ; dx < -128: use -128
    BRA DLW_SEG1_DX_READY
DLW_SEG1_DX_NO_CLAMP:
    LDB VLINE_DX_16+1  ; Use original low byte (already in valid range)
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
    ; SEGMENT 2: Draw remaining dy and dx
    ; Calculate remaining dy
    LDD VLINE_DY_16 ; Load original full dy
    CMPD #127
    BGT DLW_SEG2_DY_POS  ; dy > 127
    ; dy < -128, so we drew -128 in segment 1
    ; remaining = dy - (-128) = dy + 128
    ADDD #128       ; Add back the -128 we already drew
    BRA DLW_SEG2_DY_DONE
DLW_SEG2_DY_POS:
    ; dy > 127, so we drew 127 in segment 1
    ; remaining = dy - 127
    SUBD #127       ; Subtract 127 we already drew
DLW_SEG2_DY_DONE:
    STD VLINE_DY_REMAINING  ; Store remaining dy (16-bit)
    ; Calculate remaining dx
    LDD VLINE_DX_16 ; Load original full dx
    CMPD #127
    BLE DLW_SEG2_DX_CHECK_NEG
    ; dx > 127, so we drew 127 in segment 1
    ; remaining = dx - 127
    SUBD #127
    BRA DLW_SEG2_DX_DONE
DLW_SEG2_DX_CHECK_NEG:
    CMPD #-128
    BGE DLW_SEG2_DX_NO_REMAIN  ; -128 <= dx <= 127: no remaining dx
    ; dx < -128, so we drew -128 in segment 1
    ; remaining = dx - (-128) = dx + 128
    ADDD #128
    BRA DLW_SEG2_DX_DONE
DLW_SEG2_DX_NO_REMAIN:
    LDD #0          ; No remaining dx
DLW_SEG2_DX_DONE:
    STD VLINE_DX_REMAINING  ; Store remaining dx (16-bit)
    ; Setup for Draw_Line_d: A=dy, B=dx (CRITICAL: order matters!)
    LDA VLINE_DY_REMAINING+1  ; Low byte of remaining dy
    LDB VLINE_DX_REMAINING+1  ; Low byte of remaining dx
    CLR Vec_Misc_Count
    JSR Draw_Line_d ; Beam continues from segment 1 endpoint
DLW_DONE:
    LDA #$C8       ; CRITICAL: Restore DP to $C8 for our code
    TFR A,DP
    RTS

;**** PRINT_TEXT String Data ****
PRINT_TEXT_STR_66902187:
    FCC "FIRE!"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_3189042615754409911:
    FCC "HELLO VECTREX"
    FCB $80          ; Vectrex string terminator

