META TITLE = "State Machine Complete Test"

# Test complete state machine implementation
anim_id = -1
current_state = 0
frame_counter = 0
joy_x = 0

def main():
    SET_INTENSITY(127)
    # Create animation with states
    anim_id = CREATE_ANIM("player")
    
    # Start in idle state (state 0)
    SET_ANIM_STATE(anim_id, 0)

def loop():
    WAIT_RECAL()
    
    # Read joystick
    joy_x = J1_X()
    
    # State machine logic
    if joy_x == 0:
        # No movement - idle state
        if current_state != 0:
            current_state = 0
            SET_ANIM_STATE(anim_id, 0)
    else:
        # Moving - walking state
        if current_state != 1:
            current_state = 1
            SET_ANIM_STATE(anim_id, 1)
    
    # Update and draw animation
    UPDATE_ANIM(anim_id, 0, 30)
    DRAW_ANIM(anim_id)
    
    frame_counter = frame_counter + 1
