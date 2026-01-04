META TITLE = "Debug Layers"

def main():
    SET_INTENSITY(127)
    LOAD_LEVEL("test_level")
    
def loop():
    WAIT_RECAL()
    
    # Dibujar directamente los vectores para verificar que existen
    PRINT_TEXT(-60, 80, "BG:")
    DRAW_VECTOR("mountain")
    
    PRINT_TEXT(-60, 60, "GP:")
    DRAW_VECTOR("bubble_large")
    
    # Ahora probar SHOW_LEVEL
    PRINT_TEXT(-60, 40, "LEVEL:")
    SHOW_LEVEL()
