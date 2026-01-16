META TITLE = "Drawing Test"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Test DRAW_CIRCLE - circle at center
    DRAW_CIRCLE(0, 0, 40, 80)
    
    # Test DRAW_RECT - rectangle in top-left
    DRAW_RECT(-50, 50, 30, 20, 100)
    
    # Test DRAW_POLYGON - triangle in bottom-right
    DRAW_POLYGON(30, -30, 50, -50, 70, -30, 90)
