def main():
    WAIT_RECAL()
    SET_INTENSITY(127)

def loop():
    # Test: escribir directamente a memoria
    # Escribir valor 42 a dirección $D000 (VIA)
    POKE(0xD000, 42)
    
    # Escribir valor 123 a dirección $C800 (RAM)
    POKE(0xC800, 123)
    
    # Test visible
    PRINT_TEXT(0, 0, "POKE")