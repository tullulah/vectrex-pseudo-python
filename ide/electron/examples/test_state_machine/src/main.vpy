META TITLE = "Animation State Test"

# Test SET_ANIM_STATE functionality
anim_id = -1
current_state = 0
frame_counter = 0

def main():
    SET_INTENSITY(127)
    # Create animation instance
    anim_id = CREATE_ANIM("player_walk")

def loop():
    WAIT_RECAL()
    
    # Every 120 frames (2 seconds), change state
    if frame_counter % 120 == 0:
        # Toggle between state 0 and state 1
        current_state = 1 - current_state
        SET_ANIM_STATE(anim_id, current_state)
    
    # Update and draw animation
    UPDATE_ANIM(anim_id, 0, 30)
    DRAW_ANIM(anim_id)
    
    frame_counter = frame_counter + 1
