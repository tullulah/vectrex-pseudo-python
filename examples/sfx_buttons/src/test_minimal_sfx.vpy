# Minimal SFX Test - Just play one SFX immediately
META TITLE = "MIN SFX"

def main():
    SET_INTENSITY(127)
    PLAY_SFX("laser")

def loop():
    # Just update SFX every frame
    SFX_UPDATE()
