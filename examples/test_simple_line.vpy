META TITLE = "SIMPLE LINE"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    DRAW_LINE(0, 0, 50, 50, 127)
