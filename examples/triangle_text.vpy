META TITLE = "TRIANGLE"
META COPYRIGHT = "g GCE 1998"
META MUSIC = "music1"

# Demo mínimo: solo texto y un triángulo estático
const INTENS = 0x5F
const TXT_X = -50
const TXT_Y = 40

def main():
    PRINT_TEXT(TXT_X, TXT_Y, "TRIANGLE")
    # Triángulo (forma A: sin intensidad explícita, usa 0x5F)
    DRAW_POLYGON(3, 30, 50, -20, -50, 80, -50)
    # Cuadrado (forma B: intensidad 0x5F explícita)
    DRAW_POLYGON(4, 0x5F, -84, -44, -36, -44, -36, 4, -84, 4)
    # Hexágono (forma B)
    DRAW_POLYGON(6, 0x5F, 90, 0, 75, 26, 45, 26, 30, 0, 45, -26, 75, -26)
    # Círculo aproximado (16 lados) centrado en (0,-80) diámetro 40
    DRAW_CIRCLE(0, -80, 40)
    # Círculo con segmentos variables (32) y menor intensidad
    DRAW_CIRCLE_SEG(32, 0, 60, 50, 0x4A)
    # Arco: 24 segmentos, centro (0,0), radio 70, inicio 0°, barrido 180°
    DRAW_ARC(24, 0, 0, 70, 0, 180, 0x5F)
    # Espiral: 80 segmentos, radio inicial 10 final 60, 2.5 vueltas
    DRAW_SPIRAL(80, -90, 0, 10, 60, 2, 0x10)

    DRAW_ARC(14)

    
