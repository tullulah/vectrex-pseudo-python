; --- Motorola 6809 backend (Vectrex) title='UNTITLED' origin=$0000 ---
        ORG $0000
;***************************************************************************
; DEFINE SECTION
;***************************************************************************
    INCLUDE "../include/VECTREX.I"

;***************************************************************************
; HEADER SECTION
;***************************************************************************
    FCC "g GCE 1982"
    FCB $80
    FDB $0000 ; music pointer (0 = none)
    FCB $F8 ; height
    FCB $50 ; width
    FCB $20 ; rel y
    FCB $D0 ; rel x (-$30)
    FCC "UNTITLED"
    FCB $80 ; title terminator
    FCB 0 ; reserved
    RMB $0030-* ; pad header to $30
    ORG $0030

;***************************************************************************
; CODE SECTION
;***************************************************************************
; Init then implicit frame loop (auto_loop enabled)
INIT_START: LDS #Vec_Default_Stk
    JSR >Init_OS_RAM
    JSR >Init_VIA
    JSR >Clear_Sound
    JSR >Wait_Recal
    JSR VECTREX_BLINK_INT
    JSR >Reset0Ref
    JSR VECTREX_DEBUG_DRAW
    BRA ENTRY_LOOP
ENTRY_LOOP: JSR >Wait_Recal
    JSR VECTREX_SILENCE
    JSR VECTREX_BLINK_INT
    JSR >Reset0Ref
    JSR MAIN
    BRA ENTRY_LOOP

MAIN: ; function
; --- function main ---
    LDD #95
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR >VECTREX_FRAME_BEGIN
    CLRA
    CLRB
    STD RESULT
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_0
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR >VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #95
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR >VECTREX_DRAW_LINE
    CLRA
    CLRB
    STD RESULT
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #95
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR >VECTREX_DRAW_LINE
    CLRA
    CLRB
    STD RESULT
    LDD #20
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #40
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    LDD #95
    STD RESULT
    LDD RESULT
    STD VAR_ARG4
    JSR >VECTREX_DRAW_LINE
    CLRA
    CLRB
    STD RESULT
    RTS

;***************************************************************************
; RUNTIME SECTION
;***************************************************************************
; --- Vectrex built-in wrappers ---
VECTREX_VECTOR_PHASE_BEGIN:
    ; Cambia a DP=$C8 para rutinas de lista de vectores y recentra.
    JSR DP_to_C8
    JSR Reset0Ref
    RTS
VECTREX_DBG_STATIC_VL:
    ; Draw static debug vector list (one horizontal line).
    JSR DP_to_C8
    LDU #DBG_STATIC_LIST
    LDA #$5F
    JSR Intensity_a
    JSR Draw_VL
    RTS
DBG_STATIC_LIST:
    FCB $80,$20 ; end bit set, dy=0, dx=32
VECTREX_SILENCE:
    ; Comprehensive AY silence: zero tone periods (0-5), noise (6), mixer (7=0x3F), vols (8-10).
    LDA #0
    STA $D001 ; reg 0 select
    CLR $D000 ; tone A coarse/low (write twice for 0 & 1)
    LDA #1
    STA $D001
    CLR $D000
    LDA #2
    STA $D001
    CLR $D000 ; tone B low
    LDA #3
    STA $D001
    CLR $D000 ; tone B high
    LDA #4
    STA $D001
    CLR $D000 ; tone C low
    LDA #5
    STA $D001
    CLR $D000 ; tone C high
    LDA #6
    STA $D001
    CLR $D000 ; noise period
    LDA #7
    STA $D001
    LDA #$3F ; disable tone+noise all channels
    STA $D000
    LDA #8
    STA $D001
    CLR $D000 ; vol A
    LDA #9
    STA $D001
    CLR $D000 ; vol B
    LDA #10
    STA $D001
    CLR $D000 ; vol C
    RTS
VECTREX_PRINT_TEXT:
    ; Wait_Recal set DP=$D0 and zeroed beam; just load U,Y,X and call BIOS
    LDU VAR_ARG2   ; string pointer (high-bit terminated)
    LDA VAR_ARG1+1 ; Y
    LDB VAR_ARG0+1 ; X
    JSR Print_Str_d
    RTS
; Draw single line using vector list. Args: (x0,y0,x1,y1,intensity) low bytes.
; Assumes WAIT_RECAL already left DP at $D0. Only switches to $C8 for Draw_VL.
VECTREX_DRAW_LINE:
    ; Set intensity
    LDA VAR_ARG4+1
    JSR Intensity_a
    LDA VAR_ARG1+1
    LDB VAR_ARG0+1
    JSR Moveto_d
    ; Compute deltas (end - start) using low bytes
    LDA VAR_ARG2+1
    SUBA VAR_ARG0+1
    STA VLINE_DX
    LDA VAR_ARG3+1
    SUBA VAR_ARG1+1
    STA VLINE_DY
    ; Clamp to +/-63
    LDA VLINE_DX
    CMPA #63
    BLE VLX_OK_HI
    LDA #63
VLX_OK_HI: CMPA #-64
    BGE VLX_OK_LO
    LDA #-64
VLX_OK_LO: STA VLINE_DX
    LDA VLINE_DY
    CMPA #63
    BLE VLY_OK_HI
    LDA #63
VLY_OK_HI: CMPA #-64
    BGE VLY_OK_LO
    LDA #-64
VLY_OK_LO: STA VLINE_DY
    ; Build 2-byte vector list (Y|endbit, X)
    LDA VLINE_DY
    ORA #$80
    STA VLINE_LIST
    LDA VLINE_DX
    STA VLINE_LIST+1
    ; Switch to vector DP and draw, no restore (next WAIT_RECAL resets)
    JSR DP_to_C8
    LDU #VLINE_LIST
    JSR Draw_VL
    RTS
VECTREX_FRAME_BEGIN:
    LDA VAR_ARG0+1
    JSR Intensity_a
    JSR Reset0Ref
    RTS
    IF * < $2000
PADSIZE SET $2000-*
    FILL $FF,PADSIZE
    ENDC
;***************************************************************************
; DATA SECTION
;***************************************************************************
; Variables (in RAM)
RESULT    EQU $C880
; String literals (high-bit terminated for Vectrex PRINT_STR_D)
STR_0:
    FCB $48
    FCB $45
    FCB $4C
    FCB $4C
    FCB $CF
; Call argument scratch space
VAR_ARG0 EQU RESULT+26
VAR_ARG1 EQU RESULT+28
VAR_ARG2 EQU RESULT+30
VAR_ARG3 EQU RESULT+32
VAR_ARG4 EQU RESULT+34
VLINE_DX EQU RESULT+36
VLINE_DY EQU RESULT+37
VLINE_STEPS EQU RESULT+38
VLINE_LIST EQU RESULT+39
BLINK_STATE EQU RESULT+41
