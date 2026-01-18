META TITLE = "TEST INCREMENTAL"
META MUSIC = music1

def main():
    SET_INTENSITY(127)

def loop():
    # reset0ref()
    DRAW_LINE(-40, 0, 40, 0, 100)

    # Ahora texto - debería aparecer en posiciones absolutas
    PRINT_TEXT(-60, 60, "LINE 1")
    PRINT_TEXT(-60, 40, "LINE 2")
    PRINT_TEXT(-60, 20, "LINE 3")
    
    # Test DRAW_VECTOR - should draw pang logo (requires x, y position)
    DRAW_VECTOR("logo", 0, 0)
    
    # Test DRAW_RECT - should draw two rectangles
    # DRAW_RECT(-80, -30, 40, 30, 127)
    # DRAW_RECT(40, -30, 40, 30, 100)

    # # Una línea simple
    # DRAW_LINE(-40, -10, 40,-10, 100)

