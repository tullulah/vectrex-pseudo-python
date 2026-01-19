# TEST 4: Joystick control
# Variables work, now test Joy_Analog

META TITLE = "JOYSTICK"
META MUSIC = music1

pos_x = 0
pos_y = 0
joystick1_state = [0,0,0,0,0,0] # X, Y, BTN1, BTN2, BTN3, BTN4


def main():
    SET_INTENSITY(127)

def loop():
    # Read joystick
    read_joystick1_state()

    joy_x = joystick1_state[0]
    joy_y = joystick1_state[1]
   
    # Move position based on joystick
    pos_x = pos_x + joy_x
    pos_y = pos_y + joy_y
    
    # Clamp to screen bounds
    if pos_x > 100:
        pos_x = 100
    if pos_x < -100:
        pos_x = -100
    if pos_y > 100:
        pos_y = 100
    if pos_y < -100:
        pos_y = -100
    
    DRAW_VECTOR("vector", pos_x, pos_y)

    b1 = J1_BUTTON_1()
    b2 = J1_BUTTON_2()
    b3 = J1_BUTTON_3()
    b4 = J1_BUTTON_4()

    if(joystick1_state[2] == 1):
        PRINT_TEXT(-50, -40, "BUTTON 1")
    if(joystick1_state[3] == 1):
        PRINT_TEXT(-50, -60, "BUTTON 2")
    if(joystick1_state[4] == 1):
        PRINT_TEXT(-50, -80, "BUTTON 3")
    if(joystick1_state[5] == 1):
        PRINT_TEXT(-50, -100, "BUTTON 4")

def read_joystick1_state():
    
    # UPDATE_BUTTONS is now auto-injected at start of loop() by compiler
    # No need to call it explicitly - $C80F is already populated
    
    joystick1_state[0] = J1_X()
    joystick1_state[1] = J1_Y()
        
    # Read cached button states (fast, no BIOS calls)
    joystick1_state[2] = J1_BUTTON_1()
    joystick1_state[3] = J1_BUTTON_2()
    joystick1_state[4] = J1_BUTTON_3()
    joystick1_state[5] = J1_BUTTON_4()


