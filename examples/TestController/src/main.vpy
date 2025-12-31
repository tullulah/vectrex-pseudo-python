# TestController - Test joystick input
# VPy game for Vectrex

META TITLE = "TestController"

# Global position (persists across frames)
player_x = 0
player_y = 0

# Button debounce system (same as Pang - for real hardware testing)
btn_debounce = 0
const BTN_DEBOUNCE_FRAMES = 15
startup_delay = 0

# Button state variables
btn1 = 0
btn2 = 0
btn3 = 0
btn4 = 0

def main():
    SET_INTENSITY(127)
    btn_debounce = 0
    startup_delay = 0

def loop():
    
    # Decrement debounce counter each frame
    if btn_debounce > 0:
        btn_debounce = btn_debounce - 1
    
    # Wait for hardware to stabilize at startup (60 frames = ~1 second at 60 FPS)
    if startup_delay < 60:
        startup_delay = startup_delay + 1
    
    # Read digital joystick (-1/0/+1)
    joy_x = J1_X()
    joy_y = J1_Y()
    
    # Read buttons ALWAYS (to see raw state)
    btn1 = J1_BUTTON_1()
    btn2 = J1_BUTTON_2()
    btn3 = J1_BUTTON_3()
    btn4 = J1_BUTTON_4()
    
    # If any button pressed after startup, trigger debounce
    if startup_delay >= 60 and btn_debounce == 0:
        if btn1 == 1 or btn2 == 1 or btn3 == 1 or btn4 == 1:
            btn_debounce = BTN_DEBOUNCE_FRAMES
    
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
    
    # Draw player sprite
    DRAW_VECTOR("test", player_x, player_y)
    
    # Display button states on screen
    SET_INTENSITY(100)
    
    # Button 1 state
    if btn1 == 1:
        PRINT_TEXT(-120, 80, "BTN1: ON ")
    else:
        PRINT_TEXT(-120, 80, "BTN1: OFF")
    
    # Button 2 state
    if btn2 == 1:
        PRINT_TEXT(-120, 60, "BTN2: ON ")
    else:
        PRINT_TEXT(-120, 60, "BTN2: OFF")
    
    # Button 3 state
    if btn3 == 1:
        PRINT_TEXT(-120, 40, "BTN3: ON ")
    else:
        PRINT_TEXT(-120, 40, "BTN3: OFF")
    
    # Button 4 state
    if btn4 == 1:
        PRINT_TEXT(-120, 20, "BTN4: ON ")
    else:
        PRINT_TEXT(-120, 20, "BTN4: OFF")
    
    # Display debounce counter (for debugging)
    if btn_debounce > 0:
        PRINT_TEXT(-120, -20, "DEBOUNCE")
    
    # Display startup status
    if startup_delay < 60:
        PRINT_TEXT(-120, -40, "STARTING...")
