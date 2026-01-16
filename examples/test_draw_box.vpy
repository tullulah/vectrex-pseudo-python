META TITLE = "DRAW BOX TEST"

def main():
    SET_INTENSITY(127)

def loop():
    # Test 1: Solo una línea
    DRAW_LINE(-20, -20, 20, 20, 127)
    
    # Test 2: Cuatro líneas (como draw_box pero hardcoded)
    # DRAW_LINE(-20, -20, 20, -20, 127)  # Top
    # DRAW_LINE(20, -20, 20, 20, 127)    # Right
    # DRAW_LINE(20, 20, -20, 20, 127)    # Bottom
    # DRAW_LINE(-20, 20, -20, -20, 127)  # Left
