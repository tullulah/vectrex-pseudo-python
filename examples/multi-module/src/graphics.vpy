# Graphics module - drawing utilities

def draw_box(x, y, width, height, intensity):
    # Draw a rectangle
    # Top line
    DRAW_LINE(x, y, x + width, y, intensity)
    # Right line  
    DRAW_LINE(x + width, y, x + width, y + height, intensity)
    # Bottom line
    DRAW_LINE(x + width, y + height, x, y + height, intensity)
    # Left line
    DRAW_LINE(x, y + height, x, y, intensity)

def draw_cross(x, y, size, intensity):
    # Draw a cross marker (simplified - no division)
    DRAW_LINE(x - 10, y, x + 10, y, intensity)
    DRAW_LINE(x, y - 10, x, y + 10, intensity)
