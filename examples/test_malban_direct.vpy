META TITLE = "MALBAN STYLE"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Dibujar 4 líneas formando un cuadrado
    # Línea 1: abajo izquierda a abajo derecha
    DRAW_LINE(-40, -40, 40, -40, 127)
    
    # Línea 2: abajo derecha a arriba derecha
    DRAW_LINE(40, -40, 40, 40, 127)
    
    # Línea 3: arriba derecha a arriba izquierda
    DRAW_LINE(40, 40, -40, 40, 127)
    
    # Línea 4: arriba izquierda a abajo izquierda
    DRAW_LINE(-40, 40, -40, -40, 127)
