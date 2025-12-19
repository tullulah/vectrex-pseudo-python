META TITLE = "ForIn Test"

# Test for-in loop with arrays  
# Array must be global for now (local arrays on stack not yet supported by for-in)
var scores = [100, 250, 350, 500, 750]

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Test: iterate through array and draw lines
    let y = 50
    for score in scores:
        DRAW_LINE(0, y, score / 4, y, 127)
        y = y - 10
