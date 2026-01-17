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
VAR_INPUT_INPUT_RESULT_DATA EQU $CF18

; Array data storage
    ORG $CF18  ; Start of array data section
; Array: VAR_INPUT_INPUT_RESULT_DATA
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
    JSR INPUT_get_input
    LDD VAR_PLAYER_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDX #VAR_INPUT_INPUT_RESULT_DATA  ; Array data address
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
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_PLAYER_X
    LDD VAR_PLAYER_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDX #VAR_INPUT_INPUT_RESULT_DATA  ; Array data address
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
    JSR GRAPHICS_draw_box
    JSR INPUT_check_fire
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
    JSR GRAPHICS_draw_cross
    BRA .IF_END
.IF_ELSE:
.IF_END:
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

