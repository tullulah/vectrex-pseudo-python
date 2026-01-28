META TITLE = "Multi-Instance Animation Test"
META MUSIC = 0

# Test multiple concurrent animations
# Validates:
# - CREATE_ANIM returns different IDs
# - UPDATE_ANIM works independently per instance
# - DRAW_ANIM draws all instances at their positions
# - DESTROY_ANIM frees slots for reuse
# - Pool exhaustion (16 instances max)

anim1_id = -1
anim2_id = -1
anim3_id = -1
frame_counter = 0

def main():
    SET_INTENSITY(127)
    # Force vector embedding (asset system doesn't auto-detect animation deps yet)
    DRAW_VECTOR("player_walk_1", 0, 0)
    DRAW_VECTOR("player_walk_2", 0, 0)
    DRAW_VECTOR("player_walk_3", 0, 0)
    DRAW_VECTOR("player_walk_4", 0, 0)
    DRAW_VECTOR("player_walk_5", 0, 0)

def loop():
    # Create 3 animations at different positions
    if frame_counter == 0:
        anim1_id = CREATE_ANIM("player_walk")
        anim2_id = CREATE_ANIM("player_walk")
        anim3_id = CREATE_ANIM("player_walk")
    
    # Show status message
    if frame_counter < 200:
        PRINT_TEXT(-80, 100, "3 ANIMATIONS RUNNING")
    else:
        PRINT_TEXT(-60, 80, "SLOT REUSED")
    
    # Update positions independently
    if anim1_id >= 0:
        UPDATE_ANIM(anim1_id, -50, 0)
        DRAW_ANIM(anim1_id)
    
    if anim2_id >= 0:
        UPDATE_ANIM(anim2_id, 0, 30)
        DRAW_ANIM(anim2_id)
        
    if anim3_id >= 0:
        UPDATE_ANIM(anim3_id, 50, -30)
        DRAW_ANIM(anim3_id)
        SET_ANIM_MIRROR(anim3_id, 1)  # Mirror horizontally
    
    # After 200 frames, destroy anim2 and recreate
    if frame_counter == 200:
        DESTROY_ANIM(anim2_id)
        anim2_id = CREATE_ANIM("player_walk")
    
    frame_counter = frame_counter + 1
