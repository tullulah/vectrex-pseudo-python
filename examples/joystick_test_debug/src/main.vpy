META TITLE = "J1X Bug Test"

# Simplest possible test
raw_value = 0
joystick1_state = [0,0,0,0,0,0] # X, Y, BTN1, BTN2, BTN3, BTN4

def main():
    SET_INTENSITY(127)

def loop():
    
    # Just call J1_X and store it
    joystick1_state[0] = J1_X()
    joystick1_state[1] = J1_Y()
    joystick1_state[2] = J1_BUTTON_1()
    joystick1_state[3] = J1_BUTTON_2()
    joystick1_state[4] = J1_BUTTON_3()
    joystick1_state[5] = J1_BUTTON_4()
    
    # Show if it's positive, negative, or zero
    SET_INTENSITY(127)
    PRINT_TEXT(-90, 60, "J1_X TEST")
    
    if joystick1_state[0] == 0:
        PRINT_TEXT(-90, 40, "ZERO")
    elif joystick1_state[0] > 2:
        PRINT_TEXT(-90, 40, "POSITIVE")
    else:
        PRINT_TEXT(-90, 40, "NEGATIVE")

   

    PRINT_TEXT(-90, 0, "B1")
    PRINT_TEXT(-90, -20, "B2")
    PRINT_TEXT(-90, -40, "B3")
    PRINT_TEXT(-90, -60, "B4")


    if(joystick1_state[2] > 0):
        PRINT_TEXT(-60, 0, "PULSADO")
    if(joystick1_state[3] > 0):
        PRINT_TEXT(-60, -20,  "PULSADO")
    if(joystick1_state[4] > 0):
        PRINT_TEXT(-60, -40,  "PULSADO")
    if(joystick1_state[5] > 0):
        PRINT_TEXT(-60, -60,  "PULSADO")

    


