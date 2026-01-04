META TITLE = "Level System Test"
META AUTHOR = "VPy Team"

# Test level system builtins - Automatic drawing
# TODO: Level system not yet implemented in this branch

def main():
    SET_INTENSITY(127)
    # Load level once at startup
    LOAD_LEVEL("test_level")  # Disabled - not implemented

def loop():
    SHOW_LEVEL()
    DRAW_VECTOR("fuji_bg", 0, -60)
    DRAW_VECTOR_EX("coin", -40, -40, 0, 0 )
    # Draw all level objects automatically
    # This draws: background (mountain), gameplay (bubble + coin)
 

    

    # Manual drawing fallback

    
    # Update level state (placeholder for now)
    # UPDATE_LEVEL()  # TODO: Not implemented yet
