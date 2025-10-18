# test_debug_simple.vpy - Simple test program for debug symbols (.pdb) testing
# This program demonstrates basic VPy features for debugger testing

def main():
    # Initialize once at startup
    WAIT_RECAL()
    SET_INTENSITY(5)

def loop():
    # Draw a simple square pattern
    MOVE(0, 0)
    DRAW_TO(50, 0)
    DRAW_TO(50, 50)
    DRAW_TO(0, 50)
    DRAW_TO(0, 0)
