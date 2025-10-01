META TITLE = "SINGLE LINE"
META COPYRIGHT = "g GCE 1998"
META MUSIC = "music1"

# Dibujar una sola línea (0,0) -> (50,100) intensidad 0x5F
const X0 = 0
const Y0 = 0
const X1 = 50
const Y1 = 100
const INT = 0x5F

def main():
    DRAW_LINE(X0, Y0, X1, Y1, INT)
