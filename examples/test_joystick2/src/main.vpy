META TITLE = "Test Joystick 2"

player2_x = 0
player2_y = 0

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Test J2 digital (threshold-based)
    dx = J2_X()
    dy = J2_Y()
    
    player2_x = player2_x + dx
    player2_y = player2_y + dy
    
    # Test J2 buttons
    if J2_BUTTON_1() == 1:
        PRINT_TEXT(-70, 60, "J2 BTN1")
    
    if J2_BUTTON_2() == 1:
        PRINT_TEXT(-70, 50, "J2 BTN2")
    
    if J2_BUTTON_3() == 1:
        PRINT_TEXT(-70, 40, "J2 BTN3")
    
    if J2_BUTTON_4() == 1:
        PRINT_TEXT(-70, 30, "J2 BTN4")
    
    # Test J2 D-pad
    if J2_BUTTON_UP() == 1:
        PRINT_TEXT(-70, 20, "J2 UP")
    
    if J2_BUTTON_DOWN() == 1:
        PRINT_TEXT(-70, 10, "J2 DOWN")
    
    if J2_BUTTON_LEFT() == 1:
        PRINT_TEXT(-70, 0, "J2 LEFT")
    
    if J2_BUTTON_RIGHT() == 1:
        PRINT_TEXT(-70, -10, "J2 RIGHT")
    
    # Test J2 analog (raw 0-255 values)
    analog_x = J2_ANALOG_X()
    analog_y = J2_ANALOG_Y()
    
    # Draw crosshair at player position
    DRAW_LINE(player2_x-5, player2_y, player2_x+5, player2_y, 127)
    DRAW_LINE(player2_x, player2_y-5, player2_x, player2_y+5, 127)
