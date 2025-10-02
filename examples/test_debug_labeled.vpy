def main():
    WAIT_RECAL()
    SET_INTENSITY(127)

def loop():
    let x = 42
    let y = 100
    
    # Nueva función que incluye etiqueta + valor
    DEBUG_PRINT_LABELED("x", x)
    DEBUG_PRINT_LABELED("y", y)
    
    # Cambiar valores y mostrar de nuevo
    x += 5
    y *= 2
    
    DEBUG_PRINT_LABELED("x_new", x)
    DEBUG_PRINT_LABELED("y_doubled", y)