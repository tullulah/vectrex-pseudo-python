META TITLE = "Test 3: Combined"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    # Primero Malban
    DRAW_VECTOR_LIST("SQUARE", 0, 0, 80)
    # Después una línea simple
    DRAW_LINE(-50, -50, 50, -50, 127)
