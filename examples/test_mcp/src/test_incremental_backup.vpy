META TITLE = "Incremental Draw Test"

def main():
    SET_INTENSITY(127)

# Dummy function to force asset embedding (never called)
def _force_embed():
    DRAW_VECTOR("triangle")

def loop():
    # ITERACIÓN 10: DRAW_VECTOR usa datos embebidos, pero sin loop aún
    WAIT_RECAL()
    # Versión con asm manual para iterar
    asm("LDX #_TRIANGLE_VECTORS")  # Datos de triangle.vec embebidos en ROM
    asm("JSR Draw_Sync_List")    # Setup
    
    # Loop inline (Iteración 9)
    asm("W1:")
    asm("LDA VIA_int_flags")
    asm("ANDA #$40")
    asm("BEQ W1")
    asm("LDA ,X+")
    asm("CMPA #2")
    asm("BEQ DONE")
    asm("LDB ,X+")
    asm("LDA ,X+")
    asm("PSHS A")
    asm("STB VIA_port_a")
    asm("CLR VIA_port_b")
    asm("LDA #1")
    asm("STA VIA_port_b")
    asm("PULS A")
    asm("STA VIA_port_a")
    asm("CLR VIA_t1_cnt_hi")
    asm("LDA #$FF")
    asm("STA VIA_shift_reg")
    asm("W2:")
    asm("LDA VIA_int_flags")
    asm("ANDA #$40")
    asm("BEQ W2")
    asm("CLR VIA_shift_reg")
    asm("BRA W1")
    asm("DONE:")
