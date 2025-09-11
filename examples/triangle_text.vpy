META TITLE = "TRIANGLE"
META COPYRIGHT = "g GCE 1998"
META MUSIC = "music1"

# Demo mínimo: solo texto y un triángulo estático
const INTENS = 0x5F
const TXT_X = -50
const TXT_Y = 40

def main():
    PRINT_TEXT(TXT_X, TXT_Y, "TRIANGLE")
    # Triángulo usando macro DRAW_POLYGON(N, x0,y0, x1,y1, x2,y2)
    DRAW_POLYGON(3, 30, 50, -20, -50, 80, -50)
    # Cuadrado centrado en (-60,-20) semilado 24 -> vértices CCW
    DRAW_POLYGON(4, -84, -44, -36, -44, -36, 4, -84, 4)
    # Hexágono regular (R≈30) centrado aprox en (60,0) desplazado para no solapar
    # Vértices calculados: (cx+30,0),(cx+15,26),(cx-15,26),(cx-30,0),(cx-15,-26),(cx+15,-26) con cx=60
    DRAW_POLYGON(6, 90, 0, 75, 26, 45, 26, 30, 0, 45, -26, 75, -26)
