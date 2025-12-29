META TITLE = "PANG DEBUG"

# Simplified test to debug enemy spawning
screen = 0
btn_pressed = 0
spawn_called = 0

def main():
    screen = 0
    btn_pressed = 0
    spawn_called = 0

def loop():
    WAIT_RECAL()
    
    # Read button
    btn = J1_BUTTON_1()
    
    # Simple state machine
    if screen == 0:
        # Title screen - show message
        SET_INTENSITY(127)
        PRINT_TEXT(-80, 80, "PRESS BTN1")
        
        # Edge detect button
        if btn == 1 and btn_pressed == 0:
            screen = 1  # Go to game
            spawn_called = 0  # Reset flag
        
        btn_pressed = btn
    
    if screen == 1:
        # Game screen - try to spawn
        SET_INTENSITY(127)
        PRINT_TEXT(-80, 80, "SPAWNING")
        
        if spawn_called == 0:
            # Attempt to spawn
            PRINT_TEXT(-80, 60, "IN SPAWN")
            spawn_called = 1
        
        # Draw test rectangle at origin
        DRAW_VECTOR_EX("test_square", 0, 0, 0, 127)
