; --- Motorola 6809 backend (Vectrex) title='TRIANGLE' origin=$0000 ---
        ORG $0000
;***************************************************************************
; DEFINE SECTION
;***************************************************************************
; --- BIOS equates (verified against VECTREXR.I) ---
WAIT_RECAL     EQU $F192
INTENSITY_5F   EQU $F2A5
INTENSITY_A    EQU $F2AB ; variable intensity (A)
PRINT_STR_D    EQU $F37A
MOVETO_D       EQU $F312 ; move to absolute coordinate in D (A=Y,B=X)
RESET0REF      EQU $F354 ; reset zero reference
DP_TO_C8       EQU $F1AF ; set DP=$C8 (vectors)
DRAW_VL        EQU $F3DD ; Draw_VL (y x y x ... )
MUSIC1         EQU $FD0D

;***************************************************************************
; HEADER SECTION
;***************************************************************************
    FCC "g GCE 1982"
    FCB $80
    FCB $F8 ; height
    FCB $50 ; width
    FCB $20 ; rel y
    FCB $D0 ; rel x (-$30)
    ; Title (high-bit on last char)
    FCB $54
    FCB $52
    FCB $49
    FCB $41
    FCB $4E
    FCB $47
    FCB $4C
    FCB $C5
    RMB $0030-* ; pad header to $30
    ORG $0030

;***************************************************************************
; CODE SECTION
;***************************************************************************
; Init then implicit frame loop (auto_loop enabled)
INIT_START: JSR >WAIT_RECAL
    JSR >INTENSITY_5F
    JSR >RESET0REF
    BRA ENTRY_LOOP
ENTRY_LOOP: JSR >WAIT_RECAL
    ; lean loop: FRAME_BEGIN wrapper (called by user) sets intensity + RESET0REF.
    JSR MAIN
    BRA ENTRY_LOOP

X0 EQU 0
Y0 EQU 0
X1 EQU 40
Y1 EQU 0
X2 EQU 20
Y2 EQU 40
INT EQU 95
DRAW_FRAME: ; function
; --- function draw_frame ---
    LDD VAR_ARG0
    STD VAR_I
    LDD VAR_I
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #32
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    ANDA TMPRIGHT+1
    ANDB TMPRIGHT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    LDD #0
    STD RESULT
    BEQ CT_2
    BRA CE_3
CT_2:
    LDD #1
    STD RESULT
CE_3:
    LDD RESULT
    LBEQ IF_NEXT_1
    LDD #48
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR >VECTREX_FRAME_BEGIN
    CLRA
    CLRB
    STD RESULT
    LBRA IF_END_0
IF_NEXT_1:
    LDD #95
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR >VECTREX_FRAME_BEGIN
    CLRA
    CLRB
    STD RESULT
IF_END_0:
    LDD #65496
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #50
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #STR_1
    STX RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR >VECTREX_PRINT_TEXT
    CLRA
    CLRB
    STD RESULT
    LDD #65476
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #65476
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
    LDD #10
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
    LDD #0
    STD RESULT
    RTS

MAIN: ; function
; --- function main ---
    LDD VAR_SOUND_STARTED
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #0
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    LDD #0
    STD RESULT
    BEQ CT_6
    BRA CE_7
CT_6:
    LDD #1
    STD RESULT
CE_7:
    LDD RESULT
    LBEQ IF_NEXT_5
    JSR >VECTREX_PLAY_MUSIC1
    CLRA
    CLRB
    STD RESULT
    LDD #1
    STD RESULT
    LDX RESULT
    LDU #VAR_SOUND_STARTED
    STU TMPPTR
    STX ,U
    LDD #0
    STD RESULT
    LDX RESULT
    LDU #VAR_FRAME_COUNTER
    STU TMPPTR
    STX ,U
    LBRA IF_END_4
IF_NEXT_5:
IF_END_4:
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    JSR >DRAW_FRAME
    LDD #0
    STD RESULT
    RTS

;***************************************************************************
; RUNTIME SECTION
;***************************************************************************
; --- Vectrex built-in wrappers ---
VECTREX_VECTOR_PHASE_BEGIN:
    ; Cambia a DP=$C8 para rutinas de lista de vectores y recentra.
    JSR DP_TO_C8
    JSR RESET0REF
    RTS
VECTREX_DBG_STATIC_VL:
    ; Draw static debug vector list (one horizontal line).
    JSR DP_TO_C8
    LDU #DBG_STATIC_LIST
    LDA #$5F
    JSR INTENSITY_A
    JSR DRAW_VL
    RTS
DBG_STATIC_LIST:
    FCB $80,$20 ; end bit set, dy=0, dx=32
VECTREX_PRINT_TEXT:
    ; Wait_Recal set DP=$D0 and zeroed beam; just load U,Y,X and call BIOS
    LDU VAR_ARG2   ; string pointer (high-bit terminated)
    LDA VAR_ARG1+1 ; Y
    LDB VAR_ARG0+1 ; X
    JSR PRINT_STR_D
    RTS
; Draw single line using vector list. Args: (x0,y0,x1,y1,intensity) low bytes.
; Assumes WAIT_RECAL already left DP at $D0. Only switches to $C8 for DRAW_VL.
VECTREX_DRAW_LINE:
    ; Set intensity
    LDA VAR_ARG4+1
    JSR INTENSITY_A
    LDA VAR_ARG1+1
    LDB VAR_ARG0+1
    JSR MOVETO_D
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
    JSR DP_TO_C8
    LDU #VLINE_LIST
    JSR DRAW_VL
    RTS
VECTREX_FRAME_BEGIN:
    LDA VAR_ARG0+1
    JSR INTENSITY_A
    JSR RESET0REF
    RTS
VECTREX_PLAY_MUSIC1:
    JSR MUSIC1
    RTS
;***************************************************************************
; DATA SECTION
;***************************************************************************
    ORG $C880 ; begin runtime variables in RAM
; Variables (in RAM)
RESULT:   FDB 0
TMPLEFT:  FDB 0
TMPRIGHT: FDB 0
TMPPTR:   FDB 0
VAR_FRAME_COUNTER: FDB 0
VAR_I: FDB 0
VAR_SOUND_STARTED: FDB 0
; String literals (high-bit terminated for Vectrex PRINT_STR_D)
STR_0:
    FCB $44
    FCB $45
    FCB $4D
    FCB $CF
STR_1:
    FCB $54
    FCB $52
    FCB $C9
; Call argument scratch space
VAR_ARG0: FDB 0
VAR_ARG1: FDB 0
VAR_ARG2: FDB 0
VAR_ARG3: FDB 0
VAR_ARG4: FDB 0
; Line drawing temps
VLINE_DX: FCB 0
VLINE_DY: FCB 0
VLINE_STEPS: FCB 0
VLINE_LIST: FCB 0,0 ; 2-byte vector list (Y|endbit, X)
