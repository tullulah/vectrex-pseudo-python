# Input module - Test Phase 3 unifier
# Now with joystick builtins J1_X, J1_Y

joy_x = 0
joy_y = 0

def update_input():
    joy_x = J1_X()
    joy_y = J1_Y()

def print_input():
    PRINT_TEXT(-70, 50, "JOY INPUT TEST")
    PRINT_TEXT(-70, 30, "X:")
