def main():
    WAIT_RECAL()
    SET_INTENSITY(127)

def loop():
    # Test básico: escribir y leer memoria debug usando assembly inline
    
    # Escribir directamente en la dirección D800 usando ensamblador
    asm_inline("""
        LDA #99      ; Cargar valor 99
        STA $D800    ; Escribir a memoria debug D800
        LDA #$42     ; Cargar marker debug  
        STA $D801    ; Escribir marker a D801
    """)
    
    # Leer de vuelta usando ensamblador
    let value = 0
    let marker = 0
    
    asm_inline("""
        LDA $D800    ; Leer valor de D800
        STA value    ; Guardarlo en variable
        LDA $D801    ; Leer marker de D801  
        STA marker   ; Guardarlo en variable
    """)
    
    # Mostrar resultados
    PRINT_TEXT(-60, 60, "DEBUG DIRECT TEST")
    
    PRINT_TEXT(-60, 40, "VALUE:")
    PRINT_NUMBER(0, 40, value)
    
    PRINT_TEXT(-60, 20, "MARKER:")  
    PRINT_NUMBER(0, 20, marker)
    
    # Test de debug print básico también
    DEBUG_PRINT(42)
    PRINT_TEXT(-60, 0, "DEBUG_PRINT DONE")
    
    # Verificar resultado
    if value == 99:
        PRINT_TEXT(-60, -20, "WRITE SUCCESS")
    else:
        PRINT_TEXT(-60, -20, "WRITE FAILED")
        
    if marker == 0x42:
        PRINT_TEXT(-60, -40, "MARKER SUCCESS")
    else:
        PRINT_TEXT(-60, -40, "MARKER FAILED")