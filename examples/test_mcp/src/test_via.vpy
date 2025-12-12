META TITLE = "VIA Test"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    # Inicialización VIA (una vez por frame)
    ASM("LDA #$7F")
    ASM("STA $D004")
    ASM("LDA #$CE")
    ASM("STA $D00B")
    # Cuadrado con inline VIA
    DRAW_LINE(-40, -40, 40, -40, 127)
    DRAW_LINE(40, -40, 40, 40, 127)
    DRAW_LINE(40, 40, -40, 40, 127)
    DRAW_LINE(-40, 40, -40, -40, 127)
