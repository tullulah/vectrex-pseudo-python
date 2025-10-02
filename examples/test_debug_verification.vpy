def main():
    WAIT_RECAL()
    SET_INTENSITY(127)

def loop():
    # Test 1: DEBUG_PRINT básico
    DEBUG_PRINT(42)
    
    # Test 2: Verificación manual de escritura/lectura de memoria debug
    # Escribir directamente a la memoria debug D800
    POKE(0xD800, 99)    # Escribir valor 99 
    POKE(0xD801, 0x42)  # Escribir marker debug
    
    # Leer de vuelta para verificar
    let value = PEEK(0xD800)
    let marker = PEEK(0xD801)
    
    # Mostrar resultados en pantalla
    if value == 99:
        PRINT_TEXT(-50, 40, "WRITE OK")
    else:
        PRINT_TEXT(-50, 40, "WRITE FAIL")
    
    if marker == 0x42:
        PRINT_TEXT(-50, 20, "MARKER OK")
    else:
        PRINT_TEXT(-50, 20, "MARKER FAIL")
    
    # Mostrar los valores leídos
    PRINT_TEXT(-50, 0, "VAL:")
    PRINT_NUMBER(-10, 0, value)
    
    PRINT_TEXT(-50, -20, "MRK:")
    PRINT_NUMBER(-10, -20, marker)
    
    # Texto de confirmación general
    PRINT_TEXT(-50, -40, "DEBUG TEST")