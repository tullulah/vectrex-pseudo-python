META TITLE = "Level System Test"
META AUTHOR = "VPy Team"

# Test level system builtins - WITHOUT DRAW_LINE

def main():
    SET_INTENSITY(127)
    # Load level once at startup
    LOAD_LEVEL("test_level")

def loop():
    # Display title
    PRINT_TEXT(-90, 100, "LEVEL SYSTEM TEST")
    PRINT_TEXT(-90, 80, "NO DRAW_LINE")
    
    # Draw all level objects automatically
    # This draws: background (mountain), gameplay (bubble + coin)
    SHOW_LEVEL()
    
    # Update level state (placeholder for now)
    UPDATE_LEVEL()
