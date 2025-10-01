META TITLE = "TRIANGLE TEST"
META COPYRIGHT = "g GCE 1982"
META MUSIC = "0"

def main():
    let dummy = 0

def loop():
    # Triángulo simple con tres líneas estáticas
    DRAW_LINE(0, -30, 25, 15, 255)    # Línea 1: base derecha
    DRAW_LINE(25, 15, -25, 15, 255)   # Línea 2: base
    DRAW_LINE(-25, 15, 0, -30, 255)   # Línea 3: base izquierda
