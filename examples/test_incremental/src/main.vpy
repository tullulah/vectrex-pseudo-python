META TITLE = "TEST INCREMENTAL"
META MUSIC = music1

counter = 0

def main():
    SET_INTENSITY(127)

def loop():
    # Test 1: DRAW_LINE - Horizontal line
    DRAW_LINE(-40, 50, 40, 50, 100)
    
    # Test 2: DRAW_LINE - Vertical line  
    DRAW_LINE(-50, -40, -50, 40, 100)
    
    # Test 3: DRAW_LINE - Diagonal line
    DRAW_LINE(0, -30, 30, 0, 127)
    
    # Test 4: PRINT_TEXT - Static text
    PRINT_TEXT(-50, -50, "HELLO VECTREX")
    
    # Test 5: PRINT_NUMBER - Counter value
    PRINT_NUMBER(-50, -60, counter)
    counter = counter + 1
    if counter > 999:
        counter = 0
    
    # Test 6: J1_X / J1_Y - Joystick input
    joy_x = J1_X()
    joy_y = J1_Y()
    
    # Test 7: Draw crosshair at joystick position
    if joy_x != 0 or joy_y != 0:
        pos_x = joy_x * 20
        pos_y = joy_y * 20
        DRAW_LINE(pos_x - 5, pos_y, pos_x + 5, pos_y, 80)
        DRAW_LINE(pos_x, pos_y - 5, pos_x, pos_y + 5, 80)
    
    # Test 8: J1_BUTTON_1 - Fire button
    btn = J1_BUTTON_1()
    if btn == 1:
        PRINT_TEXT(20, 60, "FIRE!")

