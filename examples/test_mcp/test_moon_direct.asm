; Test directo de dibujo de luna - ASM puro con BIOS
        ORG $0000

; BIOS includes
    INCLUDE "../../core/src/backend/VECTREX.I"

; Header
    FCC "g GCE 1982"
    FCB $80
    FDB 1
    FCB $F8, $50, $20, $BB
    FCC "MOON TEST"
    FCB $80, 0

    JMP START

START:
    ; Inicializar
    LDA #$80
    STA VIA_t1_cnt_lo
    LDX #Vec_Default_Stk
    TFR X,S

MAIN:
    JSR Wait_Recal
    LDA #$80
    STA VIA_t1_cnt_lo
    
    ; Llamar al loop de dibujado
    JSR DRAW_MOON
    
    BRA MAIN

DRAW_MOON:
    ; Configurar DP para registros VIA
    LDA #$D0
    TFR A,DP
    
    ; Reset integrator al centro
    JSR Reset0Ref
    
    ; Establecer escala máxima
    LDA #$7F
    STA VIA_t1_cnt_lo
    
    ; Establecer intensidad alta
    LDA #200
    JSR Intensity_a
    
    ; Dibujar el círculo de la luna
    ; Mover al primer punto (0, 30)
    LDA #30
    LDB #0
    JSR Moveto_d
    
    ; Dibujar el círculo usando Draw_VLc
    LDX #MOON_CIRCLE_DATA
    JSR Draw_VLc
    
    ; Apagar el beam para saltar al primer cráter
    LDA #0
    JSR Intensity_a
    
    ; Mover al inicio del primer cráter (-10, 8)
    ; Delta desde (0, 30) es: dy=-22, dx=-10
    LDA #-22
    LDB #-10
    JSR Moveto_d
    
    ; Encender el beam
    LDA #200
    JSR Intensity_a
    
    ; Dibujar primer cráter
    LDX #CRATER1_DATA
    JSR Draw_VLc
    
    ; Apagar beam para saltar al segundo cráter
    LDA #0
    JSR Intensity_a
    
    ; Mover al segundo cráter (8, -5)
    ; Delta desde (-10, 5) es: dy=-10, dx=18
    LDA #-10
    LDB #18
    JSR Moveto_d
    
    ; Encender beam
    LDA #200
    JSR Intensity_a
    
    ; Dibujar segundo cráter
    LDX #CRATER2_DATA
    JSR Draw_VLc
    
    ; Apagar beam para saltar al tercer cráter
    LDA #0
    JSR Intensity_a
    
    ; Mover al tercer cráter (-5, -12)
    ; Delta desde (8, -8) es: dy=-4, dx=-13
    LDA #-4
    LDB #-13
    JSR Moveto_d
    
    ; Encender beam
    LDA #200
    JSR Intensity_a
    
    ; Dibujar tercer cráter
    LDX #CRATER3_DATA
    JSR Draw_VLc
    
    RTS

; Datos del círculo (23 líneas)
MOON_CIRCLE_DATA:
    FCB 23              ; Número de líneas
    FCB -1, 8           ; delta 0
    FCB -3, 7           ; delta 1
    FCB -5, 6           ; delta 2
    FCB -6, 5           ; delta 3
    FCB -7, 3           ; delta 4
    FCB -8, 1           ; delta 5
    FCB -8, -1          ; delta 6
    FCB -7, -3          ; delta 7
    FCB -6, -5          ; delta 8
    FCB -5, -6          ; delta 9
    FCB -3, -7          ; delta 10
    FCB -1, -8          ; delta 11
    FCB 1, -8           ; delta 12
    FCB 3, -7           ; delta 13
    FCB 5, -6           ; delta 14
    FCB 6, -5           ; delta 15
    FCB 7, -3           ; delta 16
    FCB 8, -1           ; delta 17
    FCB 8, 1            ; delta 18
    FCB 7, 3            ; delta 19
    FCB 6, 5            ; delta 20
    FCB 5, 6            ; delta 21
    FCB 3, 7            ; delta 22

; Primer cráter (7 líneas)
CRATER1_DATA:
    FCB 7               ; Número de líneas
    FCB 2, -2           ; delta 0
    FCB 3, 0            ; delta 1
    FCB 2, 2            ; delta 2
    FCB 0, 3            ; delta 3
    FCB -2, 2           ; delta 4
    FCB -3, 0           ; delta 5
    FCB -2, -2          ; delta 6

; Segundo cráter (7 líneas)
CRATER2_DATA:
    FCB 7               ; Número de líneas
    FCB 2, -2           ; delta 0
    FCB 3, 0            ; delta 1
    FCB 2, 2            ; delta 2
    FCB 0, 3            ; delta 3
    FCB -2, 2           ; delta 4
    FCB -3, 0           ; delta 5
    FCB -2, -2          ; delta 6

; Tercer cráter (8 líneas)
CRATER3_DATA:
    FCB 8               ; Número de líneas
    FCB 0, 3            ; delta 0
    FCB -2, 2           ; delta 1
    FCB -3, 0           ; delta 2
    FCB -2, -2          ; delta 3
    FCB 0, -3           ; delta 4
    FCB 2, -2           ; delta 5
    FCB 3, 0            ; delta 6
    FCB 2, 2            ; delta 7
