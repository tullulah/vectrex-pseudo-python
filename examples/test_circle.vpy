META TITLE = "Circle Test - VIA Direct Control"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Draw ONE large circle for debugging
    DRAW_CIRCLE(0, 0, 100)
