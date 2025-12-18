META TITLE = "Joystick Test"
META AUTHOR = "VPy Test"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Read Joystick 1 position
    let j1_x = J1_X()
    let j1_y = J1_Y()
    
    # Draw crosshair at joystick position
    # Scale from -127..127 to screen coordinates
    let center_x = 0
    let center_y = 0
    
    # Draw horizontal line through center
    MOVE(center_x - 20, center_y)
    DRAW_TO(center_x + 20, center_y)
    
    # Draw vertical line through center  
    MOVE(center_x, center_y - 20)
    DRAW_TO(center_x, center_y + 20)
    
    # Draw cursor at joystick position
    let cursor_x = j1_x
    let cursor_y = j1_y
    
    # Draw small cross at cursor
    MOVE(cursor_x - 5, cursor_y)
    DRAW_TO(cursor_x + 5, cursor_y)
    MOVE(cursor_x, cursor_y - 5)
    DRAW_TO(cursor_x, cursor_y + 5)
    
    # Draw dots at corners if buttons pressed
    if J1_BUTTON_1():
        # Top-left
        MOVE(-60, 60)
        DRAW_TO(-55, 60)
        
    if J1_BUTTON_2():
        # Top-right
        MOVE(60, 60)
        DRAW_TO(55, 60)
        
    if J1_BUTTON_3():
        # Bottom-left
        MOVE(-60, -60)
        DRAW_TO(-55, -60)
        
    if J1_BUTTON_4():
        # Bottom-right
        MOVE(60, -60)
        DRAW_TO(55, -60)
