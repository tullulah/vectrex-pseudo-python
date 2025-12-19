META TITLE = "Debug Test"

var count = 0
var screen = 10

def main():
    SET_INTENSITY(127)
    DEBUG_PRINT(screen)
    DEBUG_PRINT(count)

def loop():
    WAIT_RECAL()
    count = count + 1
    DEBUG_PRINT(count)
    DRAW_LINE(-50, -50, 50, 50, 127)
