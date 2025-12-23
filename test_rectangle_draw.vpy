META TITLE = "Rectangle Closure Test"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Draw the rectangle - all 4 edges should be visible/closed
    # Top edge: (-20,-20) to (20,-20)
    # Right edge: (20,-20) to (20,20)  
    # Bottom edge: (20,20) to (-20,20)
    # Left edge: (-20,20) to (-20,-20) <- This was missing before fix
    SET_INTENSITY(127)
    DRAW_VECTOR("test_rectangle", 0, 0)
