META TITLE = "Level System Test"
META AUTHOR = "VPy Team"

# Test level system builtins - Automatic drawing
# TODO: Level system not yet implemented in this branch

def main():
    SET_INTENSITY(127)
    # Load level once at startup
    LOAD_LEVEL("test_level")

def loop():
    SHOW_LEVEL()

    # Manual drawing fallback

    
    # Update level state (placeholder for now)
    UPDATE_LEVEL()  # TODO: Not implemented yet
