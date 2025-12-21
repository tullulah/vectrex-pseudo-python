# Test SFX functionality
META TITLE = "SFX Test"

def main():
    SET_INTENSITY(127)

def loop():
    # Play a sound effect on each frame (for testing)
    PLAY_SFX("laser")
    SFX_UPDATE()
