META TITLE = "TEST INCREMENTAL"
META MUSIC = music1

def main():
    SET_INTENSITY(127)

def loop():
    # Test 1: Horizontal line (dx=30, dy=0)
    DRAW_LINE(0, 0, 30, 0, 127)
    
    # Test 2: Diagonal line (dx=30, dy=30) - should be different!
    DRAW_LINE(0, 0, 30, 30, 127)

