META TITLE = "Test Sin DRAW_LINE"

player_x = 0
player_y = 0

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Solo movimiento sin dibujo
    joy_x = J1_X()
    joy_y = J1_Y()
    
    player_x = player_x + joy_x
    player_y = player_y + joy_y
