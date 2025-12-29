# testcircle - Test circles using DRAW_VECTOR (pre-calculated .vec)
# VPy game for Vectrex
# Uses bubble.vec created with VectorEditor Circle tool (16 segments)

META TITLE = "testcircle"

pos = [-110, -110]
delay = 0

def main():
    SET_INTENSITY(127)

def loop():
    
    delay = 0
    pos[0] = pos[0] + 1
    pos[1] = pos[1] + 1
    
    if(pos[0] >= 120):
        pos[0] = 0
    if(pos[1] >= 120):
        pos[1] = 0


    # Draw bubble using vector asset at origin (much simpler!)
    # DRAW_VECTOR(name, x_offset, y_offset)
    DRAW_VECTOR("bubble", pos[0], pos[1])

