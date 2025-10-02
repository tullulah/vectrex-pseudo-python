def main():
    WAIT_RECAL()
    SET_INTENSITY(127)

def loop():
    # Test simple: DEBUG_PRINT y luego verificar memoria manualmente
    
    # Primero hacer DEBUG_PRINT
    DEBUG_PRINT(42)
    # PRINT_TEXT(-120, 0, "DEBUG_PRINT DONE")
    
    # Ahora escribir manualmente a memoria debug CF00-CF01 (end of RAM)
    POKE(0xCF00, 99)    # Escribir valor  
    POKE(0xCF01, 0x66)  # Escribir marker diferente para distinguir
    
    # Leer de vuelta
    let value = PEEK(0xCF00)
    let marker = PEEK(0xCF01)
    
    # Mostrar en pantalla
    # PRINT_TEXT(-100, 40, "MANUAL TEST:")
    PRINT_TEXT(-20, 0, "VAL=")
    PRINT_NUMBER(0, 0, value)
    
    # PRINT_TEXT(-100, 0, "MRK=")  
    # PRINT_NUMBER(-20, 0, marker)
    
    # Verificar resultado
    if value == 99:
        PRINT_TEXT(-40, -20, "VAL OK")
    else:
        PRINT_TEXT(-40, -20, "VAL BAD")

    if marker == 0x66:
        PRINT_TEXT(-40, -40, "MRK OK")
    else:
        PRINT_TEXT(-40, -40, "MRK BAD")