META TITLE = "Level System Test"
META AUTHOR = "VPy Team"

# Test level system builtins - Automatic drawing

def main():
    SET_INTENSITY(127)
    # Load level once at startup
    LOAD_LEVEL("test_level")

def loop():
    # Draw all level objects automatically
    # This draws: background (mountain), gameplay (bubble + coin)
    SHOW_LEVEL()
    
    # Update level state (placeholder for now)
    UPDATE_LEVEL()
