META TITLE = "TEST INCREMENTAL"
META MUSIC = music1

def main():
    SET_INTENSITY(127)

def loop():
    # Test DRAW_LINE: Different types of lines
    
    #Horizontal line
    DRAW_LINE(-40, 50, 40, 50, 100)
    
    # Vertical line  
    DRAW_LINE(-50, -40, -50, 40, 100)
    
    # Diagonal line (45 degrees)
    DRAW_LINE(-30, -30, 30, 30, 127)
    
    # Different diagonal
    DRAW_LINE(30, -30, -30, 30, 90)

