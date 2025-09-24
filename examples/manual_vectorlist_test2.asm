
        INCLUDE "../include/VECTREX.I"
; Minimal Vectrex header (same pattern as other working examples)
        FCB $67,$20,$47,$43,$45,$20,$31,$39,$38,$33,$80 ; "g GCE 1983" + $80
        FDB music1
        FCB $F8,$50,$20,$20
        FCC "VL COUNT TEST"
        FCB $80
        FCB 0

; Frame loop using counted vector list
START:
        JSR Wait_Recal
        JSR Intensity_5F
        JSR Reset0Ref
        LDU #VL1
        JSR Draw_VLc            ; count, then (y,x) signed deltas
        BRA START

; Counted list: first pair is initial relative move from origin.
; We'll draw simple rectangle (30x30) then a small diamond.
VL1:
        FCB 12                  ; number of segments (pairs below)
        FCB 0,30                ; right 30
        FCB 30,0                ; down 30
        FCB 0,$E2               ; left -30
        FCB $E2,0               ; up -30 (back)
        FCB 10,10               ; diag down-right
        FCB $F6,10              ; up-right (-10,+10)
        FCB $F6,$F6             ; up-left (-10,-10)
        FCB 10,$F6              ; down-left (+10,-10)
        FCB 0,15                ; right 15
        FCB 15,0                ; down 15
        FCB 0,$F1               ; left -15
        FCB $F1,0               ; up -15
        END
