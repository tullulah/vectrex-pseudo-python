def main():
    WAIT_RECAL()
    SET_INTENSITY(127)

def loop():
    let x = 42
    let y = 100
    
    # Opción actual: solo valores
    DEBUG_PRINT(x)
    DEBUG_PRINT(y)
    
    # Lo que queremos: etiquetas + valores
    # Usando PRINT_TEXT en pantalla y DEBUG_PRINT en consola
    PRINT_TEXT(0, 80, "X=")
    PRINT_TEXT(30, 80, str(x))
    DEBUG_PRINT(x)
    
    PRINT_TEXT(0, 60, "Y=")  
    PRINT_TEXT(30, 60, str(y))
    DEBUG_PRINT(y)