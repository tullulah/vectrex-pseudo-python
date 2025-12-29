# Test optimized DRAW_CIRCLE with VIA integrator
META TITLE = "CIRCLE TEST"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Draw three circles of different sizes
    DRAW_CIRCLE(0, 50, 40, 127)      # Large circle (top)
    DRAW_CIRCLE(-50, -30, 30, 100)   # Medium circle (bottom-left)
    DRAW_CIRCLE(50, -30, 20, 80)     # Small circle (bottom-right)
