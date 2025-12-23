# SFX Test - Play all effects in sequence with proper timing
META TITLE = "SFX TEST - COIN / JUMP / HIT"

sfx_index = 0
frame_counter = 0
sfx_started = 0

def main():
    SET_INTENSITY(127)
    # Start background music
    PLAY_MUSIC("bgm")
    # Initialize with first SFX to avoid garbage at startup
    # PLAY_SFX("coin")
    sfx_started = 1

def loop():
    # AUDIO_UPDATE() is auto-injected here by compiler after WAIT_RECAL
    # It handles both music and SFX in one synchronized pass
    
    # Increment frame counter ONLY if SFX not just started
    if sfx_started == 0:
        frame_counter += 1
    else:
        sfx_started = 0
    
    # Each SFX duration: coin ~250ms, jump ~180ms, hit ~300ms
    # Play next SFX every 350ms (17 frames at 50 FPS)
    if frame_counter >= 17:
        frame_counter = 0
        
        if sfx_index == 0:
            PLAY_SFX("coin")
            sfx_index = 1
        elif sfx_index == 1:
            PLAY_SFX("jump")
            sfx_index = 2
        elif sfx_index == 2:
            PLAY_SFX("hit")
            sfx_index = 3
        elif sfx_index >= 3:
            # Loop back to coin
            PLAY_SFX("coin")
            sfx_index = 1

