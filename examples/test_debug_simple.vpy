META TITLE = "Debug Test"

x = 0
y = 0

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    x = x + 1
    if x > 10:
        x = 0
    
    DRAW_LINE(0, 0, x * 10, y * 10, 127)
