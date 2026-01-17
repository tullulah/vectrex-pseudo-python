# Input module - joystick and button handling

# Global array to store input (instead of local array in function)
input_result = [0, 0]

def get_input():
    # Read joystick input and store in global array (no local variables)
    input_result[0] = J1_X()
    input_result[1] = J1_Y()
    return 0  # Return dummy value

def check_fire():
    # Check if fire button is pressed
    return J1_BUTTON_1()
