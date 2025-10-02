def main():
    WAIT_RECAL()
    SET_INTENSITY(127)

def loop():
    # Manual debug: usar inline ASM para escribir a $DFFF
    ASM("LDA #42")
    ASM("STA $DFFF")
    
    # Algo visible
    PRINT_TEXT(0, 0, "ASM")