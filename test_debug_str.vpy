META TITLE = "Debug String Test"

var texto = "abcd"
var count = 0

def main():
    SET_INTENSITY(127)
    DEBUG_PRINT_STR(texto)
    DEBUG_PRINT(count)

def loop():
    WAIT_RECAL()
    
    # Increment counter
    count = count + 1
    DEBUG_PRINT(count)
    
    # Change string on frame 30
    if count == 30:
        texto = "xyz!"
        DEBUG_PRINT_STR(texto)
    
    # Draw something simple
    DRAW_LINE(-50, -50, 50, 50, 127)
