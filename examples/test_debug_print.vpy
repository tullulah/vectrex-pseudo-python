def main():
    WAIT_RECAL()
    SET_INTENSITY(127)

def loop():
    MOVE_TO(100, 50)
    PRINT_TEXT(100, 50, "DEBUG TEST")
    
    # Test DEBUG_PRINT con diferentes valores
    DEBUG_PRINT(42)      # Valor constante
    DEBUG_PRINT(100)     # Otro valor
    DEBUG_PRINT(255)     # Valor máximo byte
    
    let x = 75
    DEBUG_PRINT(x)       # Variable
    
    x += 1
    DEBUG_PRINT(x)       # Variable modificada