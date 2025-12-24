# SFX + Music Test - Background music + cycles through SFX
META TITLE = "SFX + MUSIC"

# Frame counter for timing
frame_count = 0
# Current SFX index (0-3)
current_sfx = 0
# Flag to track if we played this SFX
played = 0

def main():
    SET_INTENSITY(127)
    PLAY_MUSIC("bgm")

def loop():
    # Increment frame counter
    frame_count = frame_count + 1
    
    # Every 50 frames (1 second), play next SFX
    if frame_count >= 50:
        frame_count = 0
        played = 0
        current_sfx = current_sfx + 1
        if current_sfx >= 4:
            current_sfx = 0
    
    # Play SFX at start of each cycle
    if played == 0:
        played = 1
        if current_sfx == 0:
            PLAY_SFX("laser")
        if current_sfx == 1:
            PLAY_SFX("jump")
        if current_sfx == 2:
            PLAY_SFX("hit")
        if current_sfx == 3:
            PLAY_SFX("coin")
    
    # Update SFX system
    #SFX_UPDATE()
    
    # ✅ CRITICAL: MUSIC_UPDATE() at END of loop (after all logic)
    #MUSIC_UPDATE()
