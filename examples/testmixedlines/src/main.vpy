META TITLE = "Test: Mixed Line Sizes"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    ; Small line (inline): from 0,0 to 0,50
    DRAW_LINE(0, 0, 0, 50, 100)
    
    ; Big line (segmented): from 0,60 to 0,200
    DRAW_LINE(0, 60, 0, 200, 80)
