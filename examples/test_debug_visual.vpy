def main():
    WAIT_RECAL()
    SET_INTENSITY(127)

def loop():
    # Test básico con dibujo visible
    DEBUG_PRINT_LABELED("frame_start", 1)
    
    # Dibujar una X en el centro
    MOVE_TO(0, 50)
    DRAW_TO(50, -50)
    
    DEBUG_PRINT_LABELED("line1_drawn", 1)
    
    MOVE_TO(50, 50) 
    DRAW_TO(0, -50)
    
    DEBUG_PRINT_LABELED("line2_drawn", 1)
    
    # Dibujar un texto
    PRINT_TEXT(-80, 80, "DEBUG TEST")
    
    DEBUG_PRINT_LABELED("text_drawn", 1)
    
    # Test con operaciones y resultados visibles
    let x = 30
    let y = 40
    
    DEBUG_PRINT_LABELED("x_value", x)
    DEBUG_PRINT_LABELED("y_value", y)
    
    # Dibujar un cuadrado usando los valores
    MOVE_TO(-x, y)
    DRAW_TO(x, y)
    DRAW_TO(x, -y)
    DRAW_TO(-x, -y)
    DRAW_TO(-x, y)
    
    DEBUG_PRINT_LABELED("square_drawn", 1)
    
    DEBUG_PRINT_LABELED("frame_end", 1)