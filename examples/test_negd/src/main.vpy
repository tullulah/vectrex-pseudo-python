META TITLE = "NEGD Test"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Test NEGD builtin
    x = 100
    y = -x
    
    DRAW_LINE(0, 0, x, y, 127)
