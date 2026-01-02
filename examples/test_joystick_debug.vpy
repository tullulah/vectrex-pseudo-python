META TITLE = "Joystick Debug"

joy_x = 0
joy_y = 0
joy_x_raw = 0  # Store raw value before processing
joy_y_raw = 0

def main():
    SET_INTENSITY(127)

def loop():
    
    # Test known values first
    SET_INTENSITY(127)
    PRINT_TEXT(-90, 100, "KNOWN VALUES:")
    PRINT_TEXT(-90, 85, "0=")
    PRINT_NUMBER(-70, 85, 0)
    PRINT_TEXT(-90, 70, "5=")
    PRINT_NUMBER(-70, 70, 5)
    PRINT_TEXT(-90, 55, "-5=")
    PRINT_NUMBER(-70, 55, -5)
    PRINT_TEXT(-90, 40, "127=")
    PRINT_NUMBER(-70, 40, 127)
    PRINT_TEXT(-90, 25, "-127=")
    PRINT_NUMBER(-70, 25, -127)
    
    # Now read joystick
    joy_x = J1_X()
    joy_y = J1_Y()
    
    PRINT_TEXT(-90, 0, "JOYSTICK:")
    PRINT_TEXT(-90, -15, "X=")
    PRINT_NUMBER(-70, -15, joy_x)
    PRINT_TEXT(-90, -30, "Y=")
    PRINT_NUMBER(-70, -30, joy_y)
    
    # Test comparison
    if joy_x < -20 or joy_x > 20:
        PRINT_TEXT(-90, -50, "MOVE")
    else:
        PRINT_TEXT(-90, -50, "STOP")
