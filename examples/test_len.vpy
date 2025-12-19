META TITLE = "Len Test"

# Test len() builtin function
var numbers = [10, 20, 30, 40, 50]

def main():
    SET_INTENSITY(127)
    let size = len(numbers)
    DEBUG_PRINT(size)  # Should print 5

def loop():
    WAIT_RECAL()
    
    # Draw a line whose length depends on array size
    let array_size = len(numbers)
    let line_length = array_size * 20
    DRAW_LINE(-50, 0, line_length, 0, 127)
