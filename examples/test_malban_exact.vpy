META TITLE = "Malban Exact"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Frame initialization (Malban lines 13-43)
    ASM("CLR $D05A")              # VIA_shift_reg = 0 (blank)
    ASM("LDA #$CC")
    ASM("STA $D00B")              # VIA_cntl = 0xCC (zero)
    ASM("CLR $D000")              # VIA_port_a = 0
    ASM("LDA #$82")
    ASM("STA $D002")              # VIA_port_b = 0x82
    ASM("LDA #$7F")
    ASM("STA $D004")              # VIA_t1_cnt_lo = scaleMove
    
    # CRITICAL DELAY LOOP
    ASM("LDX #$05")
    ASM("DELAY_LOOP:")
    ASM("LEAX -1,X")
    ASM("BNE DELAY_LOOP")
    
    ASM("LDA #$83")
    ASM("STA $D002")              # VIA_port_b = 0x83
    ASM("CLR $D000")              # VIA_port_a = y (0)
    ASM("LDA #$CE")
    ASM("STA $D00B")              # VIA_cntl = 0xCE (integrator)
    ASM("CLR $D002")              # VIA_port_b = 0 (mux enable)
    ASM("LDA #$01")
    ASM("STA $D002")              # VIA_port_b = 1 (mux disable)
    ASM("CLR $D000")              # VIA_port_a = x (0)
    ASM("CLR $D005")              # VIA_t1_cnt_hi = 0 (start)
    ASM("LDA #$7F")
    ASM("STA $D004")              # VIA_t1_cnt_lo = scaleList
    
    # Wait for move timer
    ASM("MOVE_WAIT:")
    ASM("LDA $D00D")
    ASM("ANDA #$40")
    ASM("BEQ MOVE_WAIT")
    
    # Draw square with inline VIA (4 lines)
    DRAW_LINE(-40, -40, 40, -40, 127)   # Bottom
    DRAW_LINE(40, -40, 40, 40, 127)     # Right
    DRAW_LINE(40, 40, -40, 40, 127)     # Top
    DRAW_LINE(-40, 40, -40, -40, 127)   # Left
