META TITLE = "Malban Init Test"
META AUTHOR = "Test"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    # Inicialización Malban (una vez por frame)
    ASM("LDA #$7F")
    ASM("STA $D004")  # VIA_t1_cnt_lo = scale
    ASM("LDA #$CE")
    ASM("STA $D00B")  # VIA_cntl = integrator mode
    
    # Ahora dibujar líneas (Malban lines 74-81)
    DRAW_LINE(-40, -40, 40, -40, 127)  # Bottom
    DRAW_LINE(40, -40, 40, 40, 127)     # Right
    DRAW_LINE(40, 40, -40, 40, 127)     # Top
    DRAW_LINE(-40, 40, -40, -40, 127)   # Left
