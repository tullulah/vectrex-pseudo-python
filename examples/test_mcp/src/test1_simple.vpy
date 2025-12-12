META TITLE = "Test 1: Simple Line"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    # Solo una línea diagonal (sabemos que esto funciona)
    DRAW_LINE(0, 0, 50, 50, 127)
