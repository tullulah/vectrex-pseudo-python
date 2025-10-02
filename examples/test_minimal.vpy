def main():
    WAIT_RECAL()
    SET_INTENSITY(127)

def loop():
    # Test mínimo: solo mostrar texto
    PRINT_TEXT(0, 0, "HELLO")