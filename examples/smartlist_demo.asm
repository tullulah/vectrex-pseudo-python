; smartlist demo (uses reusable runtime)
        ORG $0000
        INCLUDE "../include/VECTREX.I"
; --- Cartridge Header MUST start at $0000 for emulator to detect (g GCE 1983) ---
        FCB $67,$20,$47,$43,$45,$20,$31,$39,$38,$33,$80 ; 'g GCE 1983' + 0x80
        FDB music1                                      ; music pointer (or 0)
        FCB $F8,$50,$20,$20                             ; copyright & reserved
        FCC "SMARTLIST DEMO"                            ; game title (max 20 inc $80)
        FCB $80                                         ; title terminator
        FCB 0                                           ; end of header reserved byte
        JMP START                                       ; skip over runtime blob

START:
        LDA #$80
        STA VIA_t1_cnt_lo
        LDX #Vec_Default_Stk
        TFR X,S

MAIN_LOOP:
        JSR Wait_Recal
        LDA #$D0
        TFR A,DP
        JSR Intensity_5F
        JSR Reset0Ref
        LDX #VL_SHAPE
        JSR Run_VectorList
        BRA MAIN_LOOP

; Vector list with an intensity change mid-way
; count = number of triples (START/LINE/END/INT). INT triple is followed by immediate intensity byte.
VL_SHAPE:
        FCB 14 ; + ZEROREF command before diamond
        ; Parameter constants for easy tuning
CENTER_Y      EQU 0
CENTER_X      EQU 0
SQUARE_HALF   EQU 16
DIAMOND_HALF  EQU 16
DIAMOND_OFS_Y EQU 0   ; tweak vertical centering (signed 8-bit)
DIAMOND_OFS_X EQU 0   ; tweak horizontal centering (signed 8-bit)
; Precomputed derived values (wrap to 8-bit two's complement)
SQ_TOP_LEFT_Y    EQU $F0 ; -16
SQ_TOP_LEFT_X    EQU $F0 ; -16
SQ_WIDTH_POS     EQU $20 ; +32
SQ_WIDTH_NEG     EQU $E0 ; -32
DM_TOP_Y         EQU $F0 ; -16
DM_TOP_X         EQU 0
DM_DELTA_POS     EQU $10 ; +16
DM_DELTA_NEG     EQU $F0 ; -16
        ; center reference (not strictly needed but explicit)
        FCB 0,0,CMD_START          ; (1) center
        ; outer square starting at top-left (-16,-16)
        FCB $F0,$F0,CMD_START ; to top-left (-16,-16)
        FCB 0,$20,CMD_LINE    ; right 32
        FCB $20,0,CMD_LINE    ; down 32
        FCB 0,$E0,CMD_LINE    ; left 32
        FCB $E0,0,CMD_LINE    ; up 32
        ; intensity change before diamond
        FCB 0,0,CMD_INT
        FCB $3F                    ; intensity byte
        ; zero reference to eliminate drift before diamond
        FCB 0,0,CMD_ZERO
        ; diamond starting at top (0,-16)
        FCB $F0,0,CMD_START ; diamond top (0,-16)
        FCB $10,$10,CMD_LINE      ; top -> right
        FCB $10,$F0,CMD_LINE      ; right -> bottom
        FCB $F0,$F0,CMD_LINE      ; bottom -> left
        FCB $F0,$10,CMD_LINE      ; left -> top
        FCB 0,0,CMD_END            ; terminate

        INCLUDE "../runtime/vectorlist_runtime.asm"     ; interpreter placed after code

        END
