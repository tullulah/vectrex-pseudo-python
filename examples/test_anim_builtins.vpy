META TITLE = "Animation Test"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    # Test animation builtins (Phase 2)
    anim_id = CREATE_ANIM("player_walk")
    UPDATE_ANIM(anim_id, 0, 0)
    DRAW_ANIM(anim_id)
    SET_ANIM_MIRROR(anim_id, 0)
