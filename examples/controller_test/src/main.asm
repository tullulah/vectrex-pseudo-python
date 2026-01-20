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
    FCC "JOYSTICK"
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
DRAW_VEC_X           EQU $C880+$08   ; Vector draw X offset (1 bytes)
DRAW_VEC_Y           EQU $C880+$09   ; Vector draw Y offset (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$0A   ; Vector intensity override (0=use vector data) (1 bytes)
MIRROR_PAD           EQU $C880+$0B   ; Safety padding to prevent MIRROR flag corruption (16 bytes)
MIRROR_X             EQU $C880+$1B   ; X mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$1C   ; Y mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_LINE_ARGS       EQU $C880+$1D   ; DRAW_LINE argument buffer (x0,y0,x1,y1,intensity) (10 bytes)
VLINE_DX_16          EQU $C880+$27   ; DRAW_LINE dx (16-bit) (2 bytes)
VLINE_DY_16          EQU $C880+$29   ; DRAW_LINE dy (16-bit) (2 bytes)
VLINE_DX             EQU $C880+$2B   ; DRAW_LINE dx clamped (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$2C   ; DRAW_LINE dy clamped (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$2D   ; DRAW_LINE remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$2F   ; DRAW_LINE remaining dx for segment 2 (16-bit) (2 bytes)
VAR_POS_Y            EQU $C880+$31   ; User variable: pos_y (2 bytes)
VAR_B2               EQU $C880+$33   ; User variable: b2 (2 bytes)
VAR_JOYSTICK1_STATE  EQU $C880+$35   ; User variable: joystick1_state (2 bytes)
VAR_B4               EQU $C880+$37   ; User variable: b4 (2 bytes)
VAR_B1               EQU $C880+$39   ; User variable: b1 (2 bytes)
VAR_JOY_X            EQU $C880+$3B   ; User variable: joy_x (2 bytes)
VAR_JOY_Y            EQU $C880+$3D   ; User variable: joy_y (2 bytes)
VAR_B3               EQU $C880+$3F   ; User variable: b3 (2 bytes)
VAR_POS_X            EQU $C880+$41   ; User variable: pos_x (2 bytes)
VAR_JOYSTICK1_STATE_DATA EQU $C880+$43   ; Mutable array 'joystick1_state' data (6 elements x 2 bytes) (12 bytes)
VAR_ARG0             EQU $CFE0   ; Function argument 0 (16-bit) (2 bytes)
VAR_ARG1             EQU $CFE2   ; Function argument 1 (16-bit) (2 bytes)
VAR_ARG2             EQU $CFE4   ; Function argument 2 (16-bit) (2 bytes)
VAR_ARG3             EQU $CFE6   ; Function argument 3 (16-bit) (2 bytes)
VAR_ARG4             EQU $CFE8   ; Function argument 4 (16-bit) (2 bytes)

;***************************************************************************
; ARRAY DATA (ROM literals)
;***************************************************************************
; Arrays are stored in ROM and accessed via pointers
; At startup, main() initializes VAR_{name} to point to ARRAY_{name}_DATA

; Array literal for variable 'joystick1_state' (6 elements)
ARRAY_JOYSTICK1_STATE_DATA:
    FDB 0   ; Element 0
    FDB 0   ; Element 1
    FDB 0   ; Element 2
    FDB 0   ; Element 3
    FDB 0   ; Element 4
    FDB 0   ; Element 5


;***************************************************************************
; MAIN PROGRAM
;***************************************************************************

MAIN:
    ; Initialize global variables
    LDD #0
    STD VAR_POS_X
    LDD #0
    STD VAR_POS_Y
    ; Copy array 'joystick1_state' from ROM to RAM (6 elements)
    LDX #ARRAY_JOYSTICK1_STATE_DATA       ; Source: ROM array data
    LDU #VAR_JOYSTICK1_STATE_DATA       ; Dest: RAM array space
    LDD #6        ; Number of elements
.COPY_LOOP_0:
    LDY ,X++        ; Load word from ROM, increment source
    STY ,U++        ; Store word to RAM, increment dest
    SUBD #1         ; Decrement counter
    BNE .COPY_LOOP_0 ; Loop until done
    LDX #VAR_JOYSTICK1_STATE_DATA    ; Array now in RAM
    STX VAR_JOYSTICK1_STATE
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
    JSR read_joystick1_state
    LDX #VAR_JOYSTICK1_STATE_DATA  ; Array data
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
    STD VAR_JOY_X
    LDX #VAR_JOYSTICK1_STATE_DATA  ; Array data
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
    STD VAR_JOY_Y
    LDD VAR_POS_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_JOY_X
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_POS_X
    LDD VAR_POS_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_JOY_Y
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_POS_Y
    LDD #100
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_POS_X
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBGT .CMP_0_TRUE
    LDD #0
    LBRA .CMP_0_END
.CMP_0_TRUE:
    LDD #1
.CMP_0_END:
    STD RESULT
    LDD RESULT
    LBEQ IF_NEXT_1
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_POS_X
    LBRA IF_END_0
IF_NEXT_1:
IF_END_0:
    LDD #-100
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_POS_X
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLT .CMP_1_TRUE
    LDD #0
    LBRA .CMP_1_END
.CMP_1_TRUE:
    LDD #1
.CMP_1_END:
    STD RESULT
    LDD RESULT
    LBEQ IF_NEXT_3
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_POS_X
    LBRA IF_END_2
IF_NEXT_3:
IF_END_2:
    LDD #100
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_POS_Y
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBGT .CMP_2_TRUE
    LDD #0
    LBRA .CMP_2_END
.CMP_2_TRUE:
    LDD #1
.CMP_2_END:
    STD RESULT
    LDD RESULT
    LBEQ IF_NEXT_5
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_POS_Y
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    LDD #-100
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_POS_Y
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLT .CMP_3_TRUE
    LDD #0
    LBRA .CMP_3_END
.CMP_3_TRUE:
    LDD #1
.CMP_3_END:
    STD RESULT
    LDD RESULT
    LBEQ IF_NEXT_7
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_POS_Y
    LBRA IF_END_6
IF_NEXT_7:
IF_END_6:
    ; DRAW_VECTOR: Draw vector asset at position
    ; Asset: vector (1 paths)
    LDD VAR_POS_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD VAR_POS_Y
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
    LDX #_VECTOR_PATH0  ; Load path 0
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    LDA $C80F      ; Vec_Btn_State (updated by Read_Btns)
    ANDA #$01      ; Test bit 0
    BEQ .J1B1_0_OFF
    LDD #1
    BRA .J1B1_0_END
.J1B1_0_OFF:
    LDD #0
.J1B1_0_END:
    STD RESULT
    LDD RESULT
    STD VAR_B1
    LDA $C80F      ; Vec_Btn_State (updated by Read_Btns)
    ANDA #$02      ; Test bit 1
    BEQ .J1B2_1_OFF
    LDD #1
    BRA .J1B2_1_END
.J1B2_1_OFF:
    LDD #0
.J1B2_1_END:
    STD RESULT
    LDD RESULT
    STD VAR_B2
    LDA $C80F      ; Vec_Btn_State (updated by Read_Btns)
    ANDA #$04      ; Test bit 2
    BEQ .J1B3_2_OFF
    LDD #1
    BRA .J1B3_2_END
.J1B3_2_OFF:
    LDD #0
.J1B3_2_END:
    STD RESULT
    LDD RESULT
    STD VAR_B3
    LDA $C80F      ; Vec_Btn_State (updated by Read_Btns)
    ANDA #$08      ; Test bit 3
    BEQ .J1B4_3_OFF
    LDD #1
    BRA .J1B4_3_END
.J1B4_3_OFF:
    LDD #0
.J1B4_3_END:
    STD RESULT
    LDD RESULT
    STD VAR_B4
    LDD #1
    STD RESULT
    LDD RESULT
    PSHS D
    LDX #VAR_JOYSTICK1_STATE_DATA  ; Array data
    PSHS X
    LDD #2
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_4_TRUE
    LDD #0
    LBRA .CMP_4_END
.CMP_4_TRUE:
    LDD #1
.CMP_4_END:
    STD RESULT
    LDD RESULT
    LBEQ IF_NEXT_9
    ; PRINT_TEXT: Print text at position
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_1893755198147      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    LBRA IF_END_8
IF_NEXT_9:
IF_END_8:
    LDD #1
    STD RESULT
    LDD RESULT
    PSHS D
    LDX #VAR_JOYSTICK1_STATE_DATA  ; Array data
    PSHS X
    LDD #3
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_5_TRUE
    LDD #0
    LBRA .CMP_5_END
.CMP_5_TRUE:
    LDD #1
.CMP_5_END:
    STD RESULT
    LDD RESULT
    LBEQ IF_NEXT_11
    ; PRINT_TEXT: Print text at position
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_1893755198148      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    LBRA IF_END_10
IF_NEXT_11:
IF_END_10:
    LDD #1
    STD RESULT
    LDD RESULT
    PSHS D
    LDX #VAR_JOYSTICK1_STATE_DATA  ; Array data
    PSHS X
    LDD #4
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_6_TRUE
    LDD #0
    LBRA .CMP_6_END
.CMP_6_TRUE:
    LDD #1
.CMP_6_END:
    STD RESULT
    LDD RESULT
    LBEQ IF_NEXT_13
    ; PRINT_TEXT: Print text at position
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-80
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_1893755198149      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    LBRA IF_END_12
IF_NEXT_13:
IF_END_12:
    LDD #1
    STD RESULT
    LDD RESULT
    PSHS D
    LDX #VAR_JOYSTICK1_STATE_DATA  ; Array data
    PSHS X
    LDD #5
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_7_TRUE
    LDD #0
    LBRA .CMP_7_END
.CMP_7_TRUE:
    LDD #1
.CMP_7_END:
    STD RESULT
    LDD RESULT
    LBEQ IF_NEXT_15
    ; PRINT_TEXT: Print text at position
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-100
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_1893755198150      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    LBRA IF_END_14
IF_NEXT_15:
IF_END_14:
    RTS

; Function: read_joystick1_state
read_joystick1_state:
    LDD #0
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_JOYSTICK1_STATE_DATA  ; Array data address
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
    LDD #VAR_JOYSTICK1_STATE_DATA  ; Array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    JSR J1Y_BUILTIN
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_JOYSTICK1_STATE_DATA  ; Array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDA $C80F      ; Vec_Btn_State (updated by Read_Btns)
    ANDA #$01      ; Test bit 0
    BEQ .J1B1_4_OFF
    LDD #1
    BRA .J1B1_4_END
.J1B1_4_OFF:
    LDD #0
.J1B1_4_END:
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD #3
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_JOYSTICK1_STATE_DATA  ; Array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDA $C80F      ; Vec_Btn_State (updated by Read_Btns)
    ANDA #$02      ; Test bit 1
    BEQ .J1B2_5_OFF
    LDD #1
    BRA .J1B2_5_END
.J1B2_5_OFF:
    LDD #0
.J1B2_5_END:
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD #4
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_JOYSTICK1_STATE_DATA  ; Array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDA $C80F      ; Vec_Btn_State (updated by Read_Btns)
    ANDA #$04      ; Test bit 2
    BEQ .J1B3_6_OFF
    LDD #1
    BRA .J1B3_6_END
.J1B3_6_OFF:
    LDD #0
.J1B3_6_END:
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD #5
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_JOYSTICK1_STATE_DATA  ; Array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDA $C80F      ; Vec_Btn_State (updated by Read_Btns)
    ANDA #$08      ; Test bit 3
    BEQ .J1B4_7_OFF
    LDD #1
    BRA .J1B4_7_END
.J1B4_7_OFF:
    LDD #0
.J1B4_7_END:
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    RTS

;***************************************************************************
; RUNTIME HELPERS
;***************************************************************************

VECTREX_PRINT_TEXT:
    ; VPy signature: PRINT_TEXT(x, y, string)
    ; BIOS signature: Print_Str_d(A=Y, B=X, U=string)
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode for text rendering)
    STA >$D00C     ; VIA_cntl
    JSR $F1AA      ; DP_to_D0 - set Direct Page for BIOS/VIA access
    LDU VAR_ARG2   ; string pointer (third parameter)
    LDA VAR_ARG1+1 ; Y coordinate (second parameter, low byte)
    LDB VAR_ARG0+1 ; X coordinate (first parameter, low byte)
    JSR Print_Str_d ; Print string from U register
    ; CRITICAL: Reset ALL pen parameters after Print_Str_d (scale, position, etc.)
    JSR Reset_Pen  ; BIOS $F35B - resets scale, intensity, and beam state
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

; === JOYSTICK BUILTIN SUBROUTINES ===
; J1_X() - Read Joystick 1 X axis (INCREMENTAL - with state preservation)
; Returns: D = raw value from $C81B after Joy_Analog call
J1X_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81B from hardware)
    LDA #$98     ; VIA_cntl = $98 (restore DAC mode for drawing)
    STA $0C      ; Direct page $D00C (VIA_cntl)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81B)
    LDB $C81B    ; Vec_Joy_1_X (BIOS writes ~$FE at center)
    SEX          ; Sign-extend B to D
    ADDD #2      ; Calibrate center offset
    PULS X       ; Restore X
    RTS

; J1_Y() - Read Joystick 1 Y axis (INCREMENTAL - with state preservation)
; Returns: D = raw value from $C81C after Joy_Analog call
J1Y_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81C from hardware)
    LDA #$98     ; VIA_cntl = $98 (restore DAC mode for drawing)
    STA $0C      ; Direct page $D00C (VIA_cntl)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81C)
    LDB $C81C    ; Vec_Joy_1_Y (BIOS writes ~$FE at center)
    SEX          ; Sign-extend B to D
    ADDD #2      ; Calibrate center offset
    PULS X       ; Restore X
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
RTS
;**** PRINT_TEXT String Data ****
PRINT_TEXT_STR_3474579779:
    FCC "vector"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_1893755198147:
    FCC "BUTTON 1"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_1893755198148:
    FCC "BUTTON 2"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_1893755198149:
    FCC "BUTTON 3"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_1893755198150:
    FCC "BUTTON 4"
    FCB $80          ; Vectrex string terminator

;***************************************************************************
; EMBEDDED ASSETS (vectors, music, levels, SFX)
;***************************************************************************

; Generated from vector.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 3
; X bounds: min=-15, max=15, width=30
; Center: (0, 5)

_VECTOR_WIDTH EQU 30
_VECTOR_CENTER_X EQU 0
_VECTOR_CENTER_Y EQU 5

_VECTOR_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _VECTOR_PATH0        ; pointer to path 0

_VECTOR_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0F,$00,0,0        ; path0: header (y=15, x=0, relative to center)
    FCB $FF,$E2,$F1          ; line 0: flag=-1, dy=-30, dx=-15
    FCB $FF,$00,$1E          ; line 1: flag=-1, dy=0, dx=30
    FCB $FF,$1E,$F1          ; closing line: flag=-1, dy=30, dx=-15
    FCB 2                ; End marker (path complete)
