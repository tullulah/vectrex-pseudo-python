META TITLE = "Test Unifier"

import input

player_x = 0
player_y = 0

def main():
    SET_INTENSITY(127)

def loop():

    input.get_input()
    
    player_x = player_x + 1
    player_y = player_y + 1
    
    DRAW_LINE(0, 0, player_x, player_y, 127)
