# testcircle - Test circles using DRAW_VECTOR (pre-calculated .vec)
# VPy game for Vectrex
# Uses bubble.vec created with VectorEditor Circle tool (16 segments)

META TITLE = "testcircle"

def main():
    SET_INTENSITY(127)

def loop():
    
    
    # Draw bubble using vector asset at origin (much simpler!)
    # DRAW_VECTOR(name, x_offset, y_offset)
    DRAW_VECTOR("bubble", 0, 0)

