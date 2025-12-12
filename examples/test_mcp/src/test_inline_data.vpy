def main():
    SET_INTENSITY(127)

def loop():
    # Load vector data pointer
    asm("LDX #VECTOR_DATA")
    # Read intensity
    asm("LDA ,X+")
    asm("JSR $F2AB")
    # Read y_start
    asm("LDB ,X+")
    # Read x_start
    asm("LDA ,X+")
    asm("PSHS D")
    # Reset
    asm("CLR VIA_shift_reg")
    asm("LDA #$CC")
    asm("STA VIA_cntl")
    asm("CLR VIA_port_a")
    asm("LDA #$82")
    asm("STA VIA_port_b")
    asm("NOP")
    asm("NOP")
    asm("NOP")
    asm("NOP")
    asm("NOP")
    asm("LDA #$83")
    asm("STA VIA_port_b")
    # Move to position
    asm("PULS D")
    asm("STB VIA_port_a")
    asm("PSHS A")
    asm("LDA #$CE")
    asm("STA VIA_cntl")
    asm("CLR VIA_port_b")
    asm("LDA #1")
    asm("STA VIA_port_b")
    asm("PULS A")
    asm("STA VIA_port_a")
    asm("LDA #$7F")
    asm("STA VIA_t1_cnt_lo")
    asm("CLR VIA_t1_cnt_hi")
    # Skip next_y, next_x
    asm("LEAX 2,X")
    # Wait
    asm("W1:")
    asm("LDA VIA_int_flags")
    asm("ANDA #$40")
    asm("BEQ W1")
    # Read flag
    asm("LDA ,X+")
    # Check if draw (flag < 0)
    asm("TSTA")
    asm("BPL DONE")
    # Read dy
    asm("LDB ,X+")
    # Read dx
    asm("LDA ,X+")
    asm("PSHS A")
    # Draw
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
    asm("DONE:")
    # Data
    asm("VECTOR_DATA:")
    asm("FCB 100")
    asm("FCB 0,0,0,0")
    asm("FCB $FF,20,20")
    asm("FCB 2")
