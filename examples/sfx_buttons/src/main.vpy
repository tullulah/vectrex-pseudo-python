# SFX + Music Test - Background music + button-triggered SFX
META TITLE = "SFX + MUSIC"

# Button state tracking (simple vars, no array)
btn1_prev = 0
btn2_prev = 0
btn3_prev = 0
btn4_prev = 0

# Flag to track if music started
music_started = 0
current_music = 0  # Track current music (0=pang_theme, 1=map_theme)
joystick1_state = [0,0,0,0,0,0] # X, Y, BTN1, BTN2, BTN3, BTN4


def main():
    SET_INTENSITY(127)

def loop():
    # Start music on first frame
    if music_started == 0:
        music_started = 1
        PLAY_MUSIC("pang_theme")
    
    # Draw a vector shape at center
    DRAW_VECTOR_EX("vector", 0, 0, 0, 100)
    
    # Read button states directly
    btn1 = J1_BUTTON_1()
    btn2 = J1_BUTTON_2()
    btn3 = J1_BUTTON_3()
    btn4 = J1_BUTTON_4()
    
    # Button 1: Play "laser" SFX + change music
    if btn1 == 1:
        if btn1_prev == 0:
            PLAY_SFX("laser")
            if current_music == 0:
                PLAY_MUSIC("map_theme")
                current_music = 1
            else:
                PLAY_MUSIC("pang_theme")
                current_music = 0
    btn1_prev = btn1
    
    # Button 2: Play "jump" SFX + change music
    if btn2 == 1:
        if btn2_prev == 0:
            PLAY_SFX("jump")
            if current_music == 0:
                PLAY_MUSIC("map_theme")
                current_music = 1
            else:
                PLAY_MUSIC("pang_theme")
                current_music = 0
    btn2_prev = btn2
    
    # Button 3: Play "hit" SFX + change music
    if btn3 == 1:
        if btn3_prev == 0:
            PLAY_SFX("hit")
            if current_music == 0:
                PLAY_MUSIC("map_theme")
                current_music = 1
            else:
                PLAY_MUSIC("pang_theme")
                current_music = 0
    btn3_prev = btn3
    
    # Button 4: Play "coin" SFX + change music
    if btn4 == 1:
        if btn4_prev == 0:
            PLAY_SFX("coin")
            if current_music == 0:
                PLAY_MUSIC("map_theme")
                current_music = 1
            else:
                PLAY_MUSIC("pang_theme")
                current_music = 0
    btn4_prev = btn4
    
    # Display instructions
    PRINT_TEXT(-100, 80, "SFX + MUSIC TEST")
    PRINT_TEXT(-100, 60, "BTN 1 LASER")
    PRINT_TEXT(-100, 40, "BTN 2 JUMP")
    PRINT_TEXT(-100, 20, "BTN 3 HIT")
    PRINT_TEXT(-100, 0, "BTN 4 COIN")

