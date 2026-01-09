META TITLE = "RAM Usage Test"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    PRINT_TEXT(-60, 80, "RAM TEST OK")
    PRINT_TEXT(-60, 60, "NO LEVELS")
    
    # Initialize variables in loop (they persist between frames)
    x = 0
    y = 0
    count = 0
    
    # Use variables
    x = x + 1
    y = y + 1
    count = count + 1
    
    if count > 100:
        count = 0
