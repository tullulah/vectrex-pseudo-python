def main():
    WAIT_RECAL()
    SET_INTENSITY(127)

def loop():
    # Test básico de DEBUG_PRINT
    DEBUG_PRINT(42)
    DEBUG_PRINT(100)
    DEBUG_PRINT(255)
    
    # Test con etiquetas
    let x = 50
    let y = 75
    
    DEBUG_PRINT_LABELED("x_value", x)
    DEBUG_PRINT_LABELED("y_value", y)
    
    # Test con operaciones
    let sum = x + y
    DEBUG_PRINT_LABELED("sum", sum)
    
    # Cambio de valores
    x += 10
    y *= 2
    
    DEBUG_PRINT_LABELED("x_after_plus", x)
    DEBUG_PRINT_LABELED("y_after_mult", y)