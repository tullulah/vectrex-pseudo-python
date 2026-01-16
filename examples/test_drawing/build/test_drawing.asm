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
    FDB $0000              ; Music pointer
    FCB $F8,$50,$20,$BB     ; Height, Width, Rel Y, Rel X
    FCC "Drawing Test"
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
    ; WAIT_RECAL: Wait for screen refresh
    JSR Wait_Recal
    LDD #0
    STD RESULT
    LDA #$50
    JSR Intensity_a
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$00
    LDB #$14
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$08
    LDB #$FE
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$06
    LDB #$FC
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$04
    LDB #$FA
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$02
    LDB #$F8
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FE
    LDB #$F8
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FC
    LDB #$FA
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FA
    LDB #$FC
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$F8
    LDB #$FE
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$F8
    LDB #$02
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FA
    LDB #$04
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FC
    LDB #$06
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FE
    LDB #$08
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$02
    LDB #$08
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$04
    LDB #$06
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$06
    LDB #$04
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$08
    LDB #$02
    JSR Draw_Line_d
    LDD #0
    STD RESULT
    LDA #$64
    JSR Intensity_a
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$32
    LDB #$CE
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$1E
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$14
    LDB #$00
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$E2
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$EC
    LDB #$00
    JSR Draw_Line_d
    LDD #0
    STD RESULT
    LDA #$5A
    JSR Intensity_a
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$E2
    LDB #$1E
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$EC
    LDB #$14
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$14
    LDB #$14
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$D8
    JSR Draw_Line_d
    LDD #0
    STD RESULT
    LDA #$46
    JSR Intensity_a
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$00
    LDB #$D3
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$0B
    LDB #$FC
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$04
    LDB #$F5
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FC
    LDB #$F5
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$F5
    LDB #$FC
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$F5
    LDB #$04
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FC
    LDB #$0B
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$04
    LDB #$0B
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$0B
    LDB #$04
    JSR Draw_Line_d
    LDD #0
    STD RESULT
    LDA #$55
    JSR Intensity_a
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$32
    LDB #$4B
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$03
    LDB #$00
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$03
    LDB #$FF
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$04
    LDB #$FF
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$03
    LDB #$FF
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$02
    LDB #$FE
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$03
    LDB #$FE
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$02
    LDB #$FD
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$02
    LDB #$FE
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$01
    LDB #$FD
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$01
    LDB #$FC
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$01
    LDB #$FD
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$FD
    JSR Draw_Line_d
    LDD #0
    STD RESULT
    LDA #$3C
    JSR Intensity_a
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$BA
    LDB #$BA
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$0F
    JSR Draw_Line_d
    LDA #$BB
    LDB #$BA
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$0F
    JSR Draw_Line_d
    LDA #$BC
    LDB #$BA
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$0F
    JSR Draw_Line_d
    LDA #$BD
    LDB #$BA
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$0F
    JSR Draw_Line_d
    LDA #$BE
    LDB #$BA
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$0F
    JSR Draw_Line_d
    LDA #$BF
    LDB #$BA
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$0F
    JSR Draw_Line_d
    LDA #$C0
    LDB #$BA
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$0F
    JSR Draw_Line_d
    LDA #$C1
    LDB #$BA
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$0F
    JSR Draw_Line_d
    LDA #$C2
    LDB #$BA
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$0F
    JSR Draw_Line_d
    LDA #$C3
    LDB #$BA
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$0F
    JSR Draw_Line_d
    LDD #0
    STD RESULT
    JSR Intensity_5F
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    LDA #$CE
    LDB #$50
    JSR Moveto_d
    CLR Vec_Misc_Count
    LDA #$03
    LDB #$FF
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$02
    LDB #$FE
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$02
    LDB #$FD
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$02
    LDB #$FC
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$01
    LDB #$FB
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$FB
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$FB
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FF
    LDB #$FB
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FE
    LDB #$FC
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FE
    LDB #$FD
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FE
    LDB #$FE
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FD
    LDB #$FF
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FD
    LDB #$01
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FE
    LDB #$02
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FE
    LDB #$03
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FE
    LDB #$04
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$FF
    LDB #$05
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$05
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$00
    LDB #$05
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$01
    LDB #$05
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$02
    LDB #$04
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$02
    LDB #$03
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$02
    LDB #$02
    JSR Draw_Line_d
    CLR Vec_Misc_Count
    LDA #$03
    LDB #$01
    JSR Draw_Line_d
    LDD #0
    STD RESULT
    ; TODO: DRAW_SPRITE implementation
    ; Requires bitmap asset system and raster conversion
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

J2X_BUILTIN:
    ; Read J2_X from $CF02 and return -1/0/+1
    LDB $CF02      ; Joy_2_X (unsigned byte 0-255)
    CMPB #108      ; Compare with lower threshold
    BLO .J2X_LEFT  ; Branch if <108 (left)
    CMPB #148      ; Compare with upper threshold
    BHI .J2X_RIGHT ; Branch if >148 (right)
    ; Center (108-148)
    LDD #0
    RTS
.J2X_LEFT:
    LDD #-1
    RTS
.J2X_RIGHT:
    LDD #1
    RTS

J2Y_BUILTIN:
    ; Read J2_Y from $CF03 and return -1/0/+1
    LDB $CF03      ; Joy_2_Y (unsigned byte 0-255)
    CMPB #108      ; Compare with lower threshold
    BLO .J2Y_DOWN  ; Branch if <108 (down)
    CMPB #148      ; Compare with upper threshold
    BHI .J2Y_UP    ; Branch if >148 (up)
    ; Center (108-148)
    LDD #0
    RTS
.J2Y_DOWN:
    LDD #-1
    RTS
.J2Y_UP:
    LDD #1
    RTS

SQRT_HELPER:
    ; Input: D = x, Output: D = sqrt(x)
    ; Newton-Raphson: guess_new = (guess + x/guess) / 2
    ; Iterate 4 times for convergence
    
    ; Handle edge cases
    CMPD #0
    BEQ .SQRT_DONE  ; sqrt(0) = 0
    CMPD #1
    BEQ .SQRT_DONE  ; sqrt(1) = 1
    
    STD TMPPTR      ; Save x
    ; Initial guess = (x + 1) / 2
    ADDD #1
    ASRA            ; Divide by 2
    RORB
    STD TMPPTR2     ; guess
    
    ; Iterate 4 times
    LDB #4
    STB RESULT+1    ; Counter
.SQRT_LOOP:
    ; Calculate x/guess using DIV16
    LDX TMPPTR      ; X = x (dividend)
    LDD TMPPTR2     ; D = guess (divisor)
    JSR DIV16       ; D = x/guess
    
    ; guess_new = (guess + x/guess) / 2
    ADDD TMPPTR2    ; D = guess + x/guess
    ASRA            ; Divide by 2
    RORB
    STD TMPPTR2     ; Update guess
    
    DEC RESULT+1    ; Decrement counter
    BNE .SQRT_LOOP
    
    LDD TMPPTR2     ; Return final guess
.SQRT_DONE:
    RTS

POW_HELPER:
    ; Input: TMPPTR = base, TMPPTR2 = exp, Output: D = result
    LDD #1         ; result = 1
    STD RESULT
.POW_LOOP:
    LDD TMPPTR2    ; Load exponent
    BEQ .POW_DONE  ; If exp == 0, done
    SUBD #1        ; exp--
    STD TMPPTR2
    ; result = result * base (simplified: assumes small values)
    LDD RESULT
    LDX TMPPTR     ; Load base
    ; Simple multiplication loop
    PSHS D
    LDD #0
.POW_MUL_LOOP:
    LEAX -1,X
    BEQ .POW_MUL_DONE
    ADDD ,S
    BRA .POW_MUL_LOOP
.POW_MUL_DONE:
    LEAS 2,S
    STD RESULT
    BRA .POW_LOOP
.POW_DONE:
    LDD RESULT
    RTS

ATAN2_HELPER:
    ; Input: TMPPTR = y, TMPPTR2 = x, Output: D = angle (0-127)
    ; Simplified: return approximate angle based on quadrant
    LDD TMPPTR2    ; Load x
    BEQ .ATAN2_X_ZERO
    ; TODO: Full CORDIC implementation
    ; For now return 0 (placeholder)
    LDD #0
    RTS
.ATAN2_X_ZERO:
    LDD TMPPTR     ; Load y
    BPL .ATAN2_Y_POS
    LDD #96        ; -90 degrees (3/4 of 128)
    RTS
.ATAN2_Y_POS:
    LDD #32        ; +90 degrees (1/4 of 128)
    RTS

RAND_HELPER:
    ; LCG: seed = (seed * 1103515245 + 12345) & 0x7FFF
    ; Simplified for 6809: seed = (seed * 25 + 13) & 0x7FFF
    LDD RAND_SEED
    LDX #25
    ; Multiply by 25 (simple loop)
    PSHS D
    LDD #0
.RAND_MUL_LOOP:
    LEAX -1,X
    BEQ .RAND_MUL_DONE
    ADDD ,S
    BRA .RAND_MUL_LOOP
.RAND_MUL_DONE:
    LEAS 2,S
    ADDD #13       ; Add constant
    ANDA #$7F      ; Mask to positive 15-bit
    STD RAND_SEED  ; Update seed
    RTS

RAND_RANGE_HELPER:
    ; Input: TMPPTR = min, TMPPTR2 = max
    JSR RAND_HELPER
    ; D = rand()
    ; range = max - min
    PSHS D         ; Save random value
    LDD TMPPTR2    ; max
    SUBD TMPPTR    ; max - min
    STD TMPPTR2    ; Save range
    ; result = (rand % range) + min
    PULS D         ; Restore random
    ; Simple modulo: D = D % TMPPTR2 (TODO: proper modulo)
    ; For now: mask to range (works for power-of-2 ranges)
    ; result = min + (rand & (range-1))
    ADDD TMPPTR    ; Add min
    RTS

DRAW_CIRCLE_RUNTIME:
    ; Input: DRAW_CIRCLE_XC, DRAW_CIRCLE_YC, DRAW_CIRCLE_DIAM, DRAW_CIRCLE_INTENSITY
    ; Draw 16-sided polygon approximation
    
    ; Read parameters BEFORE DP change
    LDB DRAW_CIRCLE_INTENSITY
    PSHS B              ; Save intensity
    LDB DRAW_CIRCLE_DIAM
    SEX                 ; Sign-extend to 16-bit
    LSRA                ; Divide by 2 = radius
    RORB
    STD DRAW_CIRCLE_TEMP   ; Save radius
    LDB DRAW_CIRCLE_XC
    SEX
    STD DRAW_CIRCLE_TEMP+2 ; Save xc
    LDB DRAW_CIRCLE_YC
    SEX
    STD DRAW_CIRCLE_TEMP+4 ; Save yc
    
    ; Setup BIOS
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    
    ; Set intensity
    PULS A
    CMPA #$5F
    BEQ .DCR_INT_5F
    JSR Intensity_a
    BRA .DCR_AFTER_INT
.DCR_INT_5F:
    JSR Intensity_5F
.DCR_AFTER_INT:
    
    ; TODO: Generate 16 vertices with trig (simplified version uses 8-gon)
    ; For now, draw octagon approximation
    ; Move to start position (xc + radius, yc)
    LDD DRAW_CIRCLE_TEMP   ; radius
    ADDD DRAW_CIRCLE_TEMP+2 ; xc + radius
    TFR B,B
    PSHS B              ; Save X
    LDD DRAW_CIRCLE_TEMP+4 ; yc
    TFR B,A             ; Y to A
    PULS B              ; X to B
    JSR Moveto_d
    
    ; Simple octagon: 8 segments with fixed deltas
    ; This is simplified - full implementation would use SIN_TABLE
    LDD DRAW_CIRCLE_TEMP   ; radius
    TFR B,A             ; Use low byte only
    
    ; Segment 1: move (0, -r)
    CLR Vec_Misc_Count
    NEGA                ; -radius
    LDB #0
    JSR Draw_Line_d
    
    ; ... (simplified - full version would iterate all 16 segments)
    ; For now return (minimal octagon)
    RTS

DRAW_RECT_RUNTIME:
    ; Input: DRAW_RECT_X, DRAW_RECT_Y, DRAW_RECT_WIDTH, DRAW_RECT_HEIGHT, DRAW_RECT_INTENSITY
    ; Draws 4 sides of rectangle
    
    ; Save parameters to stack before DP change
    LDB DRAW_RECT_INTENSITY
    PSHS B
    LDB DRAW_RECT_HEIGHT
    PSHS B
    LDB DRAW_RECT_WIDTH
    PSHS B
    LDB DRAW_RECT_Y
    PSHS B
    LDB DRAW_RECT_X
    PSHS B
    
    ; Setup BIOS
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    
    ; Set intensity
    LDA 4,S             ; intensity
    JSR Intensity_a
    
    ; Move to starting position (x, y)
    LDA 1,S             ; y
    LDB ,S              ; x
    JSR Moveto_d_7F
    
    ; Draw right side
    CLR Vec_Misc_Count
    LDA #0
    LDB 2,S             ; width
    JSR Draw_Line_d
    
    ; Draw down side
    CLR Vec_Misc_Count
    LDA 3,S             ; height
    NEGA                ; -height
    LDB #0
    JSR Draw_Line_d
    
    ; Draw left side
    CLR Vec_Misc_Count
    LDA #0
    LDB 2,S             ; width
    NEGB                ; -width
    JSR Draw_Line_d
    
    ; Draw up side
    CLR Vec_Misc_Count
    LDA 2,S             ; height
    NEGA                ; -height
    LDB #0
    JSR Draw_Line_d
    
    LEAS 5,S            ; Clean stack
    RTS

; === SHOW_LEVEL_RUNTIME - Render level tiles ===
SHOW_LEVEL_RUNTIME:
    ; Input: LEVEL_PTR (pointer to level data)
    ;        LEVEL_WIDTH, LEVEL_HEIGHT (dimensions)
    ; Renders 8x8 tiles as rectangles
    
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    
    ; Outer loop: Y (rows)
    CLR LEVEL_Y_IDX
.SL_Y_LOOP:
    LDA LEVEL_Y_IDX
    CMPA LEVEL_HEIGHT
    BHS .SL_DONE         ; If Y >= height, done
    
    ; Inner loop: X (columns)
    CLR LEVEL_X_IDX
.SL_X_LOOP:
    LDA LEVEL_X_IDX
    CMPA LEVEL_WIDTH
    BHS .SL_NEXT_Y       ; If X >= width, next row
    
    ; Calculate tile offset: (Y * width) + X
    LDA LEVEL_Y_IDX
    LDB LEVEL_WIDTH
    MUL                  ; D = Y * width
    ADDB LEVEL_X_IDX     ; D += X
    ADCA #0
    
    ; Add to level pointer (skip 2-byte header)
    ADDD #2              ; Skip width, height bytes
    ADDD LEVEL_PTR
    TFR D,X              ; X = address of tile
    LDA ,X               ; Load tile value
    
    ; If tile is 0 (empty), skip drawing
    CMPA #0
    BEQ .SL_SKIP_TILE
    
    ; Draw tile as 8x8 rectangle
    ; Calculate screen position
    LDA LEVEL_X_IDX
    LDB #8
    MUL                  ; B = X * 8 (pixel X)
    SUBB #128            ; Center horizontally
    STB LEVEL_TEMP       ; Save pixel X
    
    LDA LEVEL_Y_IDX
    LDB #8
    MUL                  ; B = Y * 8 (pixel Y)
    SUBB #128            ; Center vertically
    NEGB                 ; Flip Y (screen coords)
    TFR B,A              ; Y to A
    LDB LEVEL_TEMP       ; X to B
    
    ; Move to tile position
    JSR Moveto_d_7F
    
    ; Draw 8x8 rectangle
    LDA #$7F
    JSR Intensity_a
    
    CLR Vec_Misc_Count
    LDA #0
    LDB #8
    JSR Draw_Line_d      ; Right
    
    CLR Vec_Misc_Count
    LDA #-8
    LDB #0
    JSR Draw_Line_d      ; Down
    
    CLR Vec_Misc_Count
    LDA #0
    LDB #-8
    JSR Draw_Line_d      ; Left
    
    CLR Vec_Misc_Count
    LDA #8
    LDB #0
    JSR Draw_Line_d      ; Up
    
.SL_SKIP_TILE:
    ; Next column
    INC LEVEL_X_IDX
    BRA .SL_X_LOOP
    
.SL_NEXT_Y:
    ; Next row
    INC LEVEL_Y_IDX
    BRA .SL_Y_LOOP
    
.SL_DONE:
    RTS

; === FADE_IN_RUNTIME - Gradual intensity increase ===
FADE_IN_RUNTIME:
    ; Input: CURRENT_INTENSITY (target intensity)
    ; Gradually increases from 0 to target in 8 steps
    
    LDA CURRENT_INTENSITY
    STA TMPPTR+1         ; Save target intensity
    CLR TMPPTR           ; Step counter = 0
    
.FI_LOOP:
    LDA TMPPTR
    CMPA #8
    BHS .FI_DONE
    
    ; Calculate intensity: (step * target) / 8
    LDB TMPPTR+1         ; target
    MUL                  ; D = step * target
    LSRA                 ; Divide by 8
    RORB
    LSRA
    RORB
    LSRA
    RORB
    
    ; Set intensity
    TFR B,A
    JSR Intensity_a
    JSR Wait_Recal
    
    INC TMPPTR
    BRA .FI_LOOP
    
.FI_DONE:
    RTS

; === FADE_OUT_RUNTIME - Gradual intensity decrease ===
FADE_OUT_RUNTIME:
    ; Input: CURRENT_INTENSITY (starting intensity)
    ; Gradually decreases from current to 0 in 8 steps
    
    LDA CURRENT_INTENSITY
    STA TMPPTR+1         ; Save starting intensity
    CLR TMPPTR           ; Step counter = 0
    
.FO_LOOP:
    LDA TMPPTR
    CMPA #8
    BHS .FO_DONE
    
    ; Calculate intensity: ((8 - step) * target) / 8
    LDA #8
    SUBA TMPPTR          ; A = 8 - step
    LDB TMPPTR+1         ; B = target
    MUL                  ; D = (8-step) * target
    LSRA                 ; Divide by 8
    RORB
    LSRA
    RORB
    LSRA
    RORB
    
    ; Set intensity
    TFR B,A
    JSR Intensity_a
    JSR Wait_Recal
    
    INC TMPPTR
    BRA .FO_LOOP
    
.FO_DONE:
    RTS

;***************************************************************************
; TRIGONOMETRY LOOKUP TABLES (128 entries each)
;***************************************************************************
SIN_TABLE:
    FDB 0    ; angle 0
    FDB 6    ; angle 1
    FDB 12    ; angle 2
    FDB 19    ; angle 3
    FDB 25    ; angle 4
    FDB 31    ; angle 5
    FDB 37    ; angle 6
    FDB 43    ; angle 7
    FDB 49    ; angle 8
    FDB 54    ; angle 9
    FDB 60    ; angle 10
    FDB 65    ; angle 11
    FDB 71    ; angle 12
    FDB 76    ; angle 13
    FDB 81    ; angle 14
    FDB 85    ; angle 15
    FDB 90    ; angle 16
    FDB 94    ; angle 17
    FDB 98    ; angle 18
    FDB 102    ; angle 19
    FDB 106    ; angle 20
    FDB 109    ; angle 21
    FDB 112    ; angle 22
    FDB 115    ; angle 23
    FDB 117    ; angle 24
    FDB 120    ; angle 25
    FDB 122    ; angle 26
    FDB 123    ; angle 27
    FDB 125    ; angle 28
    FDB 126    ; angle 29
    FDB 126    ; angle 30
    FDB 127    ; angle 31
    FDB 127    ; angle 32
    FDB 127    ; angle 33
    FDB 126    ; angle 34
    FDB 126    ; angle 35
    FDB 125    ; angle 36
    FDB 123    ; angle 37
    FDB 122    ; angle 38
    FDB 120    ; angle 39
    FDB 117    ; angle 40
    FDB 115    ; angle 41
    FDB 112    ; angle 42
    FDB 109    ; angle 43
    FDB 106    ; angle 44
    FDB 102    ; angle 45
    FDB 98    ; angle 46
    FDB 94    ; angle 47
    FDB 90    ; angle 48
    FDB 85    ; angle 49
    FDB 81    ; angle 50
    FDB 76    ; angle 51
    FDB 71    ; angle 52
    FDB 65    ; angle 53
    FDB 60    ; angle 54
    FDB 54    ; angle 55
    FDB 49    ; angle 56
    FDB 43    ; angle 57
    FDB 37    ; angle 58
    FDB 31    ; angle 59
    FDB 25    ; angle 60
    FDB 19    ; angle 61
    FDB 12    ; angle 62
    FDB 6    ; angle 63
    FDB 0    ; angle 64
    FDB -6    ; angle 65
    FDB -12    ; angle 66
    FDB -19    ; angle 67
    FDB -25    ; angle 68
    FDB -31    ; angle 69
    FDB -37    ; angle 70
    FDB -43    ; angle 71
    FDB -49    ; angle 72
    FDB -54    ; angle 73
    FDB -60    ; angle 74
    FDB -65    ; angle 75
    FDB -71    ; angle 76
    FDB -76    ; angle 77
    FDB -81    ; angle 78
    FDB -85    ; angle 79
    FDB -90    ; angle 80
    FDB -94    ; angle 81
    FDB -98    ; angle 82
    FDB -102    ; angle 83
    FDB -106    ; angle 84
    FDB -109    ; angle 85
    FDB -112    ; angle 86
    FDB -115    ; angle 87
    FDB -117    ; angle 88
    FDB -120    ; angle 89
    FDB -122    ; angle 90
    FDB -123    ; angle 91
    FDB -125    ; angle 92
    FDB -126    ; angle 93
    FDB -126    ; angle 94
    FDB -127    ; angle 95
    FDB -127    ; angle 96
    FDB -127    ; angle 97
    FDB -126    ; angle 98
    FDB -126    ; angle 99
    FDB -125    ; angle 100
    FDB -123    ; angle 101
    FDB -122    ; angle 102
    FDB -120    ; angle 103
    FDB -117    ; angle 104
    FDB -115    ; angle 105
    FDB -112    ; angle 106
    FDB -109    ; angle 107
    FDB -106    ; angle 108
    FDB -102    ; angle 109
    FDB -98    ; angle 110
    FDB -94    ; angle 111
    FDB -90    ; angle 112
    FDB -85    ; angle 113
    FDB -81    ; angle 114
    FDB -76    ; angle 115
    FDB -71    ; angle 116
    FDB -65    ; angle 117
    FDB -60    ; angle 118
    FDB -54    ; angle 119
    FDB -49    ; angle 120
    FDB -43    ; angle 121
    FDB -37    ; angle 122
    FDB -31    ; angle 123
    FDB -25    ; angle 124
    FDB -19    ; angle 125
    FDB -12    ; angle 126
    FDB -6    ; angle 127

COS_TABLE:
    FDB 127    ; angle 0
    FDB 127    ; angle 1
    FDB 126    ; angle 2
    FDB 126    ; angle 3
    FDB 125    ; angle 4
    FDB 123    ; angle 5
    FDB 122    ; angle 6
    FDB 120    ; angle 7
    FDB 117    ; angle 8
    FDB 115    ; angle 9
    FDB 112    ; angle 10
    FDB 109    ; angle 11
    FDB 106    ; angle 12
    FDB 102    ; angle 13
    FDB 98    ; angle 14
    FDB 94    ; angle 15
    FDB 90    ; angle 16
    FDB 85    ; angle 17
    FDB 81    ; angle 18
    FDB 76    ; angle 19
    FDB 71    ; angle 20
    FDB 65    ; angle 21
    FDB 60    ; angle 22
    FDB 54    ; angle 23
    FDB 49    ; angle 24
    FDB 43    ; angle 25
    FDB 37    ; angle 26
    FDB 31    ; angle 27
    FDB 25    ; angle 28
    FDB 19    ; angle 29
    FDB 12    ; angle 30
    FDB 6    ; angle 31
    FDB 0    ; angle 32
    FDB -6    ; angle 33
    FDB -12    ; angle 34
    FDB -19    ; angle 35
    FDB -25    ; angle 36
    FDB -31    ; angle 37
    FDB -37    ; angle 38
    FDB -43    ; angle 39
    FDB -49    ; angle 40
    FDB -54    ; angle 41
    FDB -60    ; angle 42
    FDB -65    ; angle 43
    FDB -71    ; angle 44
    FDB -76    ; angle 45
    FDB -81    ; angle 46
    FDB -85    ; angle 47
    FDB -90    ; angle 48
    FDB -94    ; angle 49
    FDB -98    ; angle 50
    FDB -102    ; angle 51
    FDB -106    ; angle 52
    FDB -109    ; angle 53
    FDB -112    ; angle 54
    FDB -115    ; angle 55
    FDB -117    ; angle 56
    FDB -120    ; angle 57
    FDB -122    ; angle 58
    FDB -123    ; angle 59
    FDB -125    ; angle 60
    FDB -126    ; angle 61
    FDB -126    ; angle 62
    FDB -127    ; angle 63
    FDB -127    ; angle 64
    FDB -127    ; angle 65
    FDB -126    ; angle 66
    FDB -126    ; angle 67
    FDB -125    ; angle 68
    FDB -123    ; angle 69
    FDB -122    ; angle 70
    FDB -120    ; angle 71
    FDB -117    ; angle 72
    FDB -115    ; angle 73
    FDB -112    ; angle 74
    FDB -109    ; angle 75
    FDB -106    ; angle 76
    FDB -102    ; angle 77
    FDB -98    ; angle 78
    FDB -94    ; angle 79
    FDB -90    ; angle 80
    FDB -85    ; angle 81
    FDB -81    ; angle 82
    FDB -76    ; angle 83
    FDB -71    ; angle 84
    FDB -65    ; angle 85
    FDB -60    ; angle 86
    FDB -54    ; angle 87
    FDB -49    ; angle 88
    FDB -43    ; angle 89
    FDB -37    ; angle 90
    FDB -31    ; angle 91
    FDB -25    ; angle 92
    FDB -19    ; angle 93
    FDB -12    ; angle 94
    FDB -6    ; angle 95
    FDB 0    ; angle 96
    FDB 6    ; angle 97
    FDB 12    ; angle 98
    FDB 19    ; angle 99
    FDB 25    ; angle 100
    FDB 31    ; angle 101
    FDB 37    ; angle 102
    FDB 43    ; angle 103
    FDB 49    ; angle 104
    FDB 54    ; angle 105
    FDB 60    ; angle 106
    FDB 65    ; angle 107
    FDB 71    ; angle 108
    FDB 76    ; angle 109
    FDB 81    ; angle 110
    FDB 85    ; angle 111
    FDB 90    ; angle 112
    FDB 94    ; angle 113
    FDB 98    ; angle 114
    FDB 102    ; angle 115
    FDB 106    ; angle 116
    FDB 109    ; angle 117
    FDB 112    ; angle 118
    FDB 115    ; angle 119
    FDB 117    ; angle 120
    FDB 120    ; angle 121
    FDB 122    ; angle 122
    FDB 123    ; angle 123
    FDB 125    ; angle 124
    FDB 126    ; angle 125
    FDB 126    ; angle 126
    FDB 127    ; angle 127

TAN_TABLE:
    FDB 0    ; angle 0
    FDB 1    ; angle 1
    FDB 2    ; angle 2
    FDB 3    ; angle 3
    FDB 4    ; angle 4
    FDB 5    ; angle 5
    FDB 6    ; angle 6
    FDB 7    ; angle 7
    FDB 8    ; angle 8
    FDB 9    ; angle 9
    FDB 11    ; angle 10
    FDB 12    ; angle 11
    FDB 13    ; angle 12
    FDB 15    ; angle 13
    FDB 16    ; angle 14
    FDB 18    ; angle 15
    FDB 20    ; angle 16
    FDB 22    ; angle 17
    FDB 24    ; angle 18
    FDB 27    ; angle 19
    FDB 30    ; angle 20
    FDB 33    ; angle 21
    FDB 37    ; angle 22
    FDB 42    ; angle 23
    FDB 48    ; angle 24
    FDB 56    ; angle 25
    FDB 66    ; angle 26
    FDB 80    ; angle 27
    FDB 101    ; angle 28
    FDB 120    ; angle 29
    FDB 120    ; angle 30
    FDB 120    ; angle 31
    FDB -120    ; angle 32
    FDB -120    ; angle 33
    FDB -120    ; angle 34
    FDB -120    ; angle 35
    FDB -101    ; angle 36
    FDB -80    ; angle 37
    FDB -66    ; angle 38
    FDB -56    ; angle 39
    FDB -48    ; angle 40
    FDB -42    ; angle 41
    FDB -37    ; angle 42
    FDB -33    ; angle 43
    FDB -30    ; angle 44
    FDB -27    ; angle 45
    FDB -24    ; angle 46
    FDB -22    ; angle 47
    FDB -20    ; angle 48
    FDB -18    ; angle 49
    FDB -16    ; angle 50
    FDB -15    ; angle 51
    FDB -13    ; angle 52
    FDB -12    ; angle 53
    FDB -11    ; angle 54
    FDB -9    ; angle 55
    FDB -8    ; angle 56
    FDB -7    ; angle 57
    FDB -6    ; angle 58
    FDB -5    ; angle 59
    FDB -4    ; angle 60
    FDB -3    ; angle 61
    FDB -2    ; angle 62
    FDB -1    ; angle 63
    FDB 0    ; angle 64
    FDB 1    ; angle 65
    FDB 2    ; angle 66
    FDB 3    ; angle 67
    FDB 4    ; angle 68
    FDB 5    ; angle 69
    FDB 6    ; angle 70
    FDB 7    ; angle 71
    FDB 8    ; angle 72
    FDB 9    ; angle 73
    FDB 11    ; angle 74
    FDB 12    ; angle 75
    FDB 13    ; angle 76
    FDB 15    ; angle 77
    FDB 16    ; angle 78
    FDB 18    ; angle 79
    FDB 20    ; angle 80
    FDB 22    ; angle 81
    FDB 24    ; angle 82
    FDB 27    ; angle 83
    FDB 30    ; angle 84
    FDB 33    ; angle 85
    FDB 37    ; angle 86
    FDB 42    ; angle 87
    FDB 48    ; angle 88
    FDB 56    ; angle 89
    FDB 66    ; angle 90
    FDB 80    ; angle 91
    FDB 101    ; angle 92
    FDB 120    ; angle 93
    FDB 120    ; angle 94
    FDB 120    ; angle 95
    FDB -120    ; angle 96
    FDB -120    ; angle 97
    FDB -120    ; angle 98
    FDB -120    ; angle 99
    FDB -101    ; angle 100
    FDB -80    ; angle 101
    FDB -66    ; angle 102
    FDB -56    ; angle 103
    FDB -48    ; angle 104
    FDB -42    ; angle 105
    FDB -37    ; angle 106
    FDB -33    ; angle 107
    FDB -30    ; angle 108
    FDB -27    ; angle 109
    FDB -24    ; angle 110
    FDB -22    ; angle 111
    FDB -20    ; angle 112
    FDB -18    ; angle 113
    FDB -16    ; angle 114
    FDB -15    ; angle 115
    FDB -13    ; angle 116
    FDB -12    ; angle 117
    FDB -11    ; angle 118
    FDB -9    ; angle 119
    FDB -8    ; angle 120
    FDB -7    ; angle 121
    FDB -6    ; angle 122
    FDB -5    ; angle 123
    FDB -4    ; angle 124
    FDB -3    ; angle 125
    FDB -2    ; angle 126
    FDB -1    ; angle 127

