META TITLE = "TRI DEMO"
META COPYRIGHT = "g GCE 2025"
META MUSIC = "0"

# Coordenadas de un triángulo sencillo (en rango pequeño)
const X0 = 0
const Y0 = 0
const X1 = 40
const Y1 = 0
const X2 = 20
const Y2 = 40
const INT = 95  # intensidad (0..127 aprox)

def main():
    # Tres segmentos
    DRAW_LINE(X0, Y0, X1, Y1, INT)
    DRAW_LINE(X1, Y1, X2, Y2, INT)
    DRAW_LINE(X2, Y2, X0, Y0, INT)
