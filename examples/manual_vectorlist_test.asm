        ORG $0000
        INCLUDE "../include/VECTREX.I"
; Minimal Vectrex header
        FCB $67,$20,$47,$43,$45,$20,$31,$39,$38,$33,$80 ; "g GCE 1983" + $80
        FDB music1
        FCB $F8,$50,$20,$20
        FCC "MANUAL VL TEST"
        FCB $80
        FCB 0

; Start code (simple frame loop drawing a vector list)
START:
        JSR Wait_Recal      ; calibrate, BIOS sets DP=$D0
        JSR Intensity_5F    ; standard intensity
        JSR Reset0Ref       ; center origin
        LDU #SIMPLE_VL      ; U points to vector list
        JSR Draw_VL         ; draw it
        BRA START

; SIMPLE_VL format: sequence of (dy, dx) pairs (signed), high bit of final dy marks end.
; We'll draw a small square and an internal triangle combined (open path style) to verify visibility.
; Square: ( -20,-20 ) -> (20,0) -> (0,20) -> (-20,0) -> close
; We approximate with relative deltas starting at top-left corner.
; Start by moving to top-left via first delta; pattern is cumulative.
; Note: End flag: set bit7 on last dy.

SIMPLE_VL:
        FCB $EC,$EC      ; dy=-20 dx=-20 (to top-left from origin)
        FCB $00,$28      ; dy=0   dx=+40 (top edge)
        FCB $28,$00      ; dy=+40 dx=0   (right edge)
        FCB $00,$D8      ; dy=0   dx=-40 (bottom edge)
        FCB $D8,$00      ; dy=-40 dx=0   (left edge back to start)
        ; Triangle starting from current position (top-left): go to center-ish
        FCB $0A,$14      ; dy=+10 dx=+20
        FCB $F6,$F6      ; dy=-10 dx=-10
        FCB $F6,$0A|$80  ; dy=-10|end dx=+10  (end flag)

        END
