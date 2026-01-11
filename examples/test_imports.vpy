# Test import syntax parsing

import graphics
import physics as phys
from input import read_joystick, read_buttons
from sound import play_sfx as sfx

def main():
    SET_INTENSITY(127)
    
def loop():
    WAIT_RECAL()
    # Test calling imported functions
    read_joystick()
    read_buttons()
