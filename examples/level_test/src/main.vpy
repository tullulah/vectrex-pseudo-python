META TITLE = "Level System Test"
META AUTHOR = "VPy Team"

# Test level system builtins - Automatic drawing

def main():
    SET_INTENSITY(127)
    # Load level once at startup
    LOAD_LEVEL("test_level")

def loop():
    # Display title
    PRINT_TEXT(-90, 100, "LEVEL SYSTEM TEST")
    PRINT_TEXT(-90, 80, "AUTO-DRAW MODE")
    
    # Draw world bounds frame FIRST (so we see if SHOW_LEVEL overwrites it)
    DRAW_LINE(-100, -100, 100, -100, 60)  # Bottom
    DRAW_LINE(100, -100, 100, 100, 60)    # Right
    DRAW_LINE(100, 100, -100, 100, 60)    # Top
    DRAW_LINE(-100, 100, -100, -100, 60)  # Left
    
    # Draw all level objects automatically
    # This draws: background (mountain), gameplay (bubble + coin)
    SHOW_LEVEL()
    
    # Update level state (placeholder for now)
    UPDATE_LEVEL()
