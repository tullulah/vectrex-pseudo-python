META TITLE = "Test Array Assignment"

joystick1_state = [0,0,0,0,0,0]

def main():
    SET_INTENSITY(127)

def read_joystick():
    joystick1_state[0] = J1_X()
    joystick1_state[1] = J1_Y()
    joystick1_state[2] = 1

def loop():
    read_joystick()
