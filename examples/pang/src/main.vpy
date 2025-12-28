# pang - Main entry point
# VPy game for Vectrex

META TITLE = "PANG"

# Game states
const STATE_TITLE = 0
const STATE_MAP = 1
const STATE_GAME = 2

screen = STATE_TITLE
title_intensity = 30
title_state = 0 # 0 up 1 down
current_music = -1  # Track which music is playing (-1=none, 0=pang_theme, 1=map_theme)
delay = 0
stage = 0

# Location selection system - 17 locations matching the arcade game
const location_x_coords = [40, 40, -40, -10, 20, 50, 80, -85, -50, -15, 15, 50, 85, -90, -45, 0, 45]
const location_y_coords = [110, 79, -20, 10, 40, 70, 100, -40, -10, 30, 60, 90, 20, 50, 0, -60, -30]

const location_names = ["MOUNT FUJI (JP)","MOUNT KEIRIN (CN)","EMERALD BUDDHA TEMPLE (TH)", "ANGKOR WAT (KH)","AYERS ROCK (AU)", "TAJ MAHAL (IN)","LENINGRAD (RU)","PARIS (FR)","LONDON (UK)","BARCELONA (ES)","ATHENS (GR)", "PYRAMIDS (EG)", "MOUNT KILIMANJARO (TZ)", "NEW YORK (US)","MAYAN RUINS (MX)","ANTARCTICA (AQ)","EASTER ISLAND (CL)"];

num_locations = 17
current_location = 0
location_glow_intensity = 60
location_glow_direction = 0
prev_joy_x = 0
prev_joy_y = 0
intensityval = 0

# Game state variables
countdown_timer = 0      # Countdown from 180 frames (~3 seconds at 60fps)
countdown_active = 0     # 0=inactive, 1=active
max_level_unlocked = 0   # Player progression (0-16)
prev_btn_1 = 0           # Edge detection for button 1

# Player variables
player_x = 0             # Player X position
player_y = -100          # Player Y position (on ground)
player_speed = 2         # Movement speed
player_anim_frame = 1    # Current animation frame (1-5)
player_anim_counter = 0  # Frame counter for animation timing
player_anim_speed = 5    # Frames per animation frame
player_facing = 1        # Direction facing: 1=right, -1=left


def main():
    # Called once at startup
    
    # Initialize location selection variables (once at startup)
    current_location = 0
    prev_joy_x = 0
    prev_joy_y = 0
    location_glow_intensity = 60
    location_glow_direction = 0
    screen = STATE_TITLE
    
    # Initialize game state variables
    countdown_timer = 0
    countdown_active = 0
    max_level_unlocked = 0
    prev_btn_1 = 0

def loop():
    # Pantalla de presentación con logo
    
    if screen == STATE_TITLE: # Music already started in main()
        if (current_music == -1):
            PLAY_MUSIC("pang_theme")
            current_music = 0
        
        draw_title_screen()

        btn1 = J1_BUTTON_1()
        btn2 = J1_BUTTON_2()
        btn3 = J1_BUTTON_3()
        btn4 = J1_BUTTON_4()

        if (btn1 == 1 or btn2 == 1 or btn3 == 1 or btn4 == 1):
            #PLAY_SFX("coin")
            screen = STATE_MAP 
            current_music = -1  # Reset music tracker to trigger map_theme

    if(screen == STATE_MAP):
        # Play gameplay music (only start if not already playing)
        if (delay ==0):
            STOP_MUSIC()

        if current_music != 1 and delay == 10:
            PLAY_MUSIC("map_theme")
            current_music = 1
        
        if(delay<10):
            delay += 1

        # Handle joystick input for location selection
        joy_x = J1_X()
        joy_y = J1_Y()
        
        # Navigate locations with joystick (circular navigation)
        if joy_x == 1 and prev_joy_x != 1:
            current_location = current_location + 1
            if current_location >= num_locations:
                current_location = 0
        elif joy_x == -1 and prev_joy_x != -1:
            current_location = current_location - 1
            if current_location < 0:
                current_location = num_locations - 1
        elif joy_y == 1 and prev_joy_y != 1:
            current_location = current_location + 1
            if current_location >= num_locations:
                current_location = 0
        elif joy_y == -1 and prev_joy_y != -1:
            current_location = current_location - 1
            if current_location < 0:
                current_location = num_locations - 1
        
        prev_joy_x = joy_x
        prev_joy_y = joy_y
        
        # Button detection (edge-triggered) to start level
        btn1 = J1_BUTTON_1()
        if btn1 == 1 and prev_btn_1 == 0:
            # Start level - transition to game state
            screen = STATE_GAME
            countdown_active = 1
            countdown_timer = 180  # 3 seconds at 60fps
        
        prev_btn_1 = btn1

        draw_map_screen()
    
    if screen == STATE_GAME:
        # Countdown "GET READY" phase
        if countdown_active == 1:
            SET_INTENSITY(127)
            PRINT_TEXT(-50, 40, "GET READY")
            
            # Show selected location name
            SET_INTENSITY(100)
            PRINT_TEXT(-127, 10, location_names[current_location], -6, 60)
            
            # Decrement countdown timer
            countdown_timer = countdown_timer - 1
            
            # When countdown finishes, start actual game
            if countdown_timer <= 0:
                countdown_active = 0
        else:
            # Actual game after countdown
            draw_game_level()

def draw_map_screen():
    # Draw background map
    SET_INTENSITY(100)
    PRINT_TEXT(-127, -100, "SELECT A STAGE")  # Uses BIOS defaults
    PRINT_TEXT(-127, -120, location_names[current_location], -6, 60)  # Small size (auto-restores defaults after)

    DRAW_VECTOR_EX("map", 0, 20, 0, 50)
    
    # Update glow intensity (oscillating effect)
    if location_glow_direction == 0:
        location_glow_intensity = location_glow_intensity + 3
        if location_glow_intensity >= 127:
            location_glow_direction = 1
    else:
        location_glow_intensity = location_glow_intensity - 3
        if location_glow_intensity <= 30:
            location_glow_direction = 0
    
    # Draw only the selected location with glow effect
    loc_x = location_x_coords[current_location]
    loc_y = location_y_coords[current_location]

    DRAW_VECTOR_EX("location_marker", loc_y, loc_x, 0, location_glow_intensity)
    

def draw_title_screen():
    #Dibujar logo centrado
    DRAW_VECTOR("logo", 0, 20)

    SET_INTENSITY(title_intensity)
    PRINT_TEXT(-90, -40, "PRESS A BUTTON")
    PRINT_TEXT(-50, -60, "TO START")

    if (title_state == 0):
        title_intensity += 1

    if (title_state == 1):
        title_intensity -= 1    

    if (title_intensity == 127):
        title_state = 1
    
    if (title_intensity == 30):
        title_state = 0

def draw_game_level():
    # Draw Monte Fuji background
    SET_INTENSITY(60)
    DRAW_VECTOR("fuji_bg", 0, 50)
    
    # Draw ground line
    SET_INTENSITY(100)
    DRAW_LINE(-120, -100, 120, -100, 100)
    
    # Handle player movement
    joy_x = J1_X()
    
    if joy_x == -1:
        player_x = player_x - player_speed
        if player_x < -110:
            player_x = -110
        player_facing = -1
        
        # Animate walking
        player_anim_counter = player_anim_counter + 1
        if player_anim_counter >= player_anim_speed:
            player_anim_counter = 0
            player_anim_frame = player_anim_frame + 1
            if player_anim_frame > 5:
                player_anim_frame = 1
    elif joy_x == 1:
        player_x = player_x + player_speed
        if player_x > 110:
            player_x = 110
        player_facing = 1
        
        # Animate walking
        player_anim_counter = player_anim_counter + 1
        if player_anim_counter >= player_anim_speed:
            player_anim_counter = 0
            player_anim_frame = player_anim_frame + 1
            if player_anim_frame > 5:
                player_anim_frame = 1
    else:
        # Not moving - reset to idle frame
        player_anim_frame = 1
        player_anim_counter = 0
    
    # Draw player with animation
    SET_INTENSITY(127)
    
    # Determine mirror mode based on facing direction
    mirror_mode = 0
    if player_facing == -1:
        mirror_mode = 1  # Mirror X (flip horizontally)
    
    # Draw animated sprite
    if player_anim_frame == 1:
        DRAW_VECTOR_EX("player_walk_1", player_x, player_y, mirror_mode, 127)
    elif player_anim_frame == 2:
        DRAW_VECTOR_EX("player_walk_2", player_x, player_y, mirror_mode, 127)
    elif player_anim_frame == 3:
        DRAW_VECTOR_EX("player_walk_3", player_x, player_y, mirror_mode, 127)
    elif player_anim_frame == 4:
        DRAW_VECTOR_EX("player_walk_4", player_x, player_y, mirror_mode, 127)
    else:
        DRAW_VECTOR_EX("player_walk_5", player_x, player_y, mirror_mode, 127)
