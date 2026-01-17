# Graphics module - drawing utilities

def draw_box(x, y, width, height, intensity):
    # Simple box with constant size (no complex expressions)
    # Coordinates are absolute constants to avoid DRAW_LINE_WRAPPER
    SET_INTENSITY(intensity)
    # DRAW_LINE(-10, -10, 10, -10, intensity)  # Top
    # DRAW_LINE(10, -10, 10, 10, intensity)     # Right
    # DRAW_LINE(10, 10, -10, 10, intensity)     # Bottom
    # DRAW_LINE(-10, 10, -10, -10, intensity)   # Left

def draw_cross(x, y, size, intensity):
    # Simple cross at origin
    SET_INTENSITY(intensity)
    DRAW_LINE(-10, 0, 10, 0, intensity)
    DRAW_LINE(0, -10, 0, 10, intensity)
