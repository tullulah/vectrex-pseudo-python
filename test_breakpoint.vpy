# Test breakpoints - simple program for debugging
# Click gutter on lines below to set breakpoints

def main():
    # Línea 6 - Set breakpoint here
    WAIT_RECAL()
    SET_INTENSITY(80)

def loop():
    # Línea 11 - Ejecutar en loop
    MOVE(10, 10)
    # Línea 13 - Another breakpoint location
    DRAW_TO(50, 50)
    DRAW_TO(-50, 0)
