META TITLE = "Linker Test - Simple Demo"

# Two drawing functions that will be compiled together
# Then we'll manually create a second .vo that calls them

def draw_box():
    # Draw a box using literal coordinates
    SET_INTENSITY(100)
    DRAW_LINE(-20, -20, 20, -20, 100)
    DRAW_LINE(20, -20, 20, 20, 100)
    DRAW_LINE(20, 20, -20, 20, 100)
    DRAW_LINE(-20, 20, -20, -20, 100)

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    draw_box()
