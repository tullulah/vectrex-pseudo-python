# TestController - Test joystick input
# VPy game for Vectrex

META TITLE = "TestController"

# Global position (persists across frames)
player_x = 0
player_y = 0

def main():
    SET_INTENSITY(127)

def loop():
    
    # Read digital joystick (-1/0/+1)
    joy_x = J1_X()
    joy_y = J1_Y()
    
    # Read analog joystick (-127 to +127)
    # joy_x_analog = J1_X_ANALOG()
    # joy_y_analog = J1_Y_ANALOG()
    
    btn1 = J1_BUTTON_1()
    btn2 = J1_BUTTON_2()
    
    DEBUG_PRINT(joy_x)
    DEBUG_PRINT(joy_y)
    # DEBUG_PRINT(joy_x_analog)
    # DEBUG_PRINT(joy_y_analog)
    
    # Move with digital input
    if joy_x == -1:
        player_x = player_x - 3
    if joy_x == 1:
        player_x = player_x + 3
    if joy_y == -1:
        player_y = player_y - 3
    if joy_y == 1:
        player_y = player_y + 3
    
    # Boundaries
    if player_x < -110:
        player_x = -110
    if player_x > 110:
        player_x = 110
    if player_y < -90:
        player_y = -90
    if player_y > 90:
        player_y = 90
    
    DRAW_VECTOR("test", player_x, player_y)

