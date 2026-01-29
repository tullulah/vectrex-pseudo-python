META TITLE = "Animation State Test"

# Test SET_ANIM_STATE functionality (Phase 4)
anim_id = -1
current_state = 0
frame_counter = 0

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Stub: Phase 4 state machines implementado
    # SET_ANIM_STATE resets frame counter for now
    if frame_counter % 120 == 0:
        current_state = 1 - current_state
    
    frame_counter = frame_counter + 1
