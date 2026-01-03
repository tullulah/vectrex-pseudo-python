META TITLE = "BUTTON TEST"

# Counters para mantener mensajes visibles (30 frames = ~0.5 segundos)
btn1_counter = 0
btn2_counter = 0
btn3_counter = 0
btn4_counter = 0

def main():
    SET_INTENSITY(127)

def loop():
    # Read all buttons (returns 1 on rising edge, 0 otherwise)
    btn1 = J1_BUTTON_1()
    btn2 = J1_BUTTON_2()
    btn3 = J1_BUTTON_3()
    btn4 = J1_BUTTON_4()
    
    # Update counters on button press
    if btn1 == 1:
        btn1_counter = 30
    if btn2 == 1:
        btn2_counter = 30
    if btn3 == 1:
        btn3_counter = 30
    if btn4 == 1:
        btn4_counter = 30
    
    # Instructions
    PRINT_TEXT(-100, 100, "BUTTON DEBOUNCE TEST")
    PRINT_TEXT(-100, 70, "PRESS ANY BUTTON")
    PRINT_TEXT(-100, 50, "ONE PRESS = ONE MSG")
    
    # Display button states (visible for 30 frames)
    if btn1_counter > 0:
        PRINT_TEXT(-100, 0, ">>> BTN 1 PRESSED <<<")
        btn1_counter = btn1_counter - 1
    
    if btn2_counter > 0:
        PRINT_TEXT(-100, -20, ">>> BTN 2 PRESSED <<<")
        btn2_counter = btn2_counter - 1
    
    if btn3_counter > 0:
        PRINT_TEXT(-100, -40, ">>> BTN 3 PRESSED <<<")
        btn3_counter = btn3_counter - 1
    
    if btn4_counter > 0:
        PRINT_TEXT(-100, -60, ">>> BTN 4 PRESSED <<<")
        btn4_counter = btn4_counter - 1
