META TITLE = "Test Small Lines - Should inline"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    ; Small line (50px) - should inline as Draw_Line_d
    DRAW_LINE(0, 0, 0, 50, 100)
