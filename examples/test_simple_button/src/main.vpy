META TITLE = "BUTTON TEST"

def main():
    SET_INTENSITY(127)

def loop():
    b1 = J1_BUTTON_1()
    if b1 == 1:
        PRINT_TEXT(-50, 0, "PRESSED")
    else:
        PRINT_TEXT(-50, 0, "NOT PRESSED")
