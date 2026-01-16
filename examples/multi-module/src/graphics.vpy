# Graphics module - drawing utilities

def draw_box(x, y, width, height, intensity):
    # Draw a rectangle using vector asset (better performance than 4 DRAW_LINE calls)
    # Note: box.vec is centered, so position is already the center
    DRAW_VECTOR("box", x, y)

def draw_cross(x, y, size, intensity):
    # Draw a cross marker (simplified - no division)
    DRAW_LINE(x - 10, y, x + 10, y, intensity)
    DRAW_LINE(x, y - 10, x, y + 10, intensity)
