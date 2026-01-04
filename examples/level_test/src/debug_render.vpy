META TITLE = "Debug Level Rendering"

def main():
    SET_INTENSITY(127)
    LOAD_LEVEL("test_level")

def loop():
    WAIT_RECAL()
    
    # Debug: Intentar dibujar fuji directamente
    DRAW_VECTOR("fuji_bg")
    
    # Luego llamar SHOW_LEVEL
    SHOW_LEVEL()
