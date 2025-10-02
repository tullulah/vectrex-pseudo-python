def main():
    WAIT_RECAL()
    SET_INTENSITY(127)

def loop():
    # Solo DEBUG_PRINT básico
    DEBUG_PRINT(42)
    
    # Algo visible para confirmar que el programa funciona
    PRINT_TEXT(-20, 0, "DEBUG")