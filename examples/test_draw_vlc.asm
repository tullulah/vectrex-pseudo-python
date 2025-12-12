; Test Draw_VLc - formato correcto verificado contra BIOS
; Draw_VLc espera:
;   B = número de segmentos (líneas)
;   X = puntero a datos: FCB dy,dx, FCB dy,dx, ...

    INCLUDE "include/VECTREX.I"
    ORG $0000

; Header
    FCC "g GCE 1982"
    FCB $80
    FDB 1
    FCB $F8, $50, $20
    FCB -$33
    FCC "DRAW_VLC TEST"
    FCB $80
    FCB 0

START:
    JSR Wait_Recal
    LDA #$80
    STA VIA_t1_cnt_lo

MAIN_LOOP:
    JSR Wait_Recal
    
    ; Reset al centro
    JSR Reset0Ref
    
    ; Configurar escala
    LDA #$7F
    STA VIA_t1_cnt_lo
    
    ; Set DP para hardware
    LDA #$D0
    TFR A,DP
    
    ; Intensidad
    LDA #127
    JSR Intensity_a
    
    ; Mover al punto inicial del triángulo (0, 30)
    LDA #30     ; dy
    LDB #0      ; dx
    JSR Moveto_d
    
    ; Dibujar triángulo: 3 líneas
    ; Formato esperado por Draw_VLc:
    ;   X -> FCB dy,dx (delta 0)
    ;        FCB dy,dx (delta 1)
    ;        FCB dy,dx (delta 2)
    ;   B = 3 (número de líneas)
    
    LDX #TRIANGLE_DATA
    LDB #3      ; 3 líneas
    JSR Draw_VLc
    
    BRA MAIN_LOOP

; Data del triángulo (3 puntos cerrado)
; Punto 0: (0, 30) - ya posicionado con Moveto_d
; Punto 1: (-25, -15) -> delta = (-25, -45)
; Punto 2: (25, -15) -> delta = (50, 0)
; Punto 3: (0, 30) [cerrar] -> delta = (-25, 45)
TRIANGLE_DATA:
    FCB -45, -25    ; Línea 0: de (0,30) a (-25,-15)
    FCB 0, 50       ; Línea 1: de (-25,-15) a (25,-15)
    FCB 45, -25     ; Línea 2: de (25,-15) a (0,30) [cerrar]

    END START
