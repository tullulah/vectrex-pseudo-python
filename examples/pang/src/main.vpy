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

# Level configuration - backgrounds and difficulty
const level_backgrounds = ["fuji_bg", "keirin_bg", "buddha_bg", "angkor_bg", "ayers_bg", "taj_bg", "leningrad_bg", "paris_bg", "london_bg", "barcelona_bg", "athens_bg", "pyramids_bg", "kilimanjaro_bg", "newyork_bg", "mayan_bg", "antarctica_bg", "easter_bg"]

const level_enemy_count = [1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 7]
const level_enemy_speed = [1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5]

joystick1_state = [0,0,0,0,0,0] # X, Y, BTN1, BTN2, BTN3, BTN4

num_locations = 17
current_location = 0
location_glow_intensity = 60
location_glow_direction = 0
joy_x = 0
joy_y = 0
prev_joy_x = 0
prev_joy_y = 0
intensityval = 0

# Game state variables
countdown_timer = 0      # Countdown from 180 frames (~3 seconds at 60fps)
countdown_active = 0     # 0=inactive, 1=active
max_level_unlocked = 0   # Player progression (0-16)
joystick_poll_counter = 0  # Counter to poll joystick every N frames

# Hook (disparo) system
hook_active = 0          # 0=inactive, 1=active
hook_x = 0               # X position of hook (captured at fire time)
hook_y = -100            # Y position of hook (starts at ground level)
hook_max_y = 127          # Maximum Y position (well up the screen, signed value)
hook_gun_x = 0           # Gun X position at fire time (stays fixed while hook travels)
hook_gun_y = 0           # Gun Y position at fire time (stays fixed while hook travels)
hook_init_y = 0          # Initial Y position of gun (stays fixed for rope start point)
# Player variables
player_x = 0             # Player X position
player_y = -100          # Player Y position (on ground)
player_speed = 2         # Base movement speed
move_speed = 0           # Current movement speed (calculated from joystick)
abs_joy = 0              # Helper for absolute value calculation
speed_multiplier = 0     # Speed multiplier based on joystick position
player_anim_frame = 1    # Current animation frame (1-5)
player_anim_counter = 0  # Frame counter for animation timing
const player_anim_speed = 5    # Frames per animation frame
player_facing = 1        # Direction facing: 1=right, -1=left

# Enemy (bubble) system - maximum 8 enemies
const MAX_ENEMIES = 8
enemy_active = [0, 0, 0, 0, 0, 0, 0, 0]        # 0=inactive, 1=active
enemy_x = [0, 0, 0, 0, 0, 0, 0, 0]             # X positions
enemy_y = [0, 0, 0, 0, 0, 0, 0, 0]             # Y positions
enemy_vx = [0, 0, 0, 0, 0, 0, 0, 0]            # X velocities
enemy_vy = [0, 0, 0, 0, 0, 0, 0, 0]            # Y velocities
enemy_size = [0, 0, 0, 0, 0, 0, 0, 0]          # Size: 4=huge, 3=large, 2=medium, 1=small

# Physics constants
const GRAVITY = 2         # Downward acceleration (increased from 1 for visible movement)
const BOUNCE_DAMPING = 9  # Velocity retained after bounce (9/10 = 90%)
const GROUND_Y = -80      # Y position of ground


def main():
    # Called once at startup
    
    # Initialize location selection variables (once at startup)
    current_location = 0
    prev_joy_x = 0
    prev_joy_y = 0
    location_glow_intensity = 80
    location_glow_direction = 0
    screen = STATE_TITLE
    
    # Initialize game state variables
    countdown_timer = 0
    countdown_active = 0
    max_level_unlocked = 0
    
    # Initialize hook system
    hook_active = 0
    hook_x = 0
    hook_y = -100
    
    # Initialize enemy system
    init_enemies()

def loop():

    read_joystick1_state()

    if screen == STATE_TITLE: # Music already started in main()
        if (current_music == -1):
            PLAY_MUSIC("pang_theme")
            current_music = 0
        
        draw_title_screen()

        if (joystick1_state[2] == 1 or joystick1_state[3] == 1 or joystick1_state[4] == 1 or joystick1_state[5] == 1):
            screen = STATE_MAP 
            current_music = -1  # Reset music tracker to trigger map_theme
            # PLAY_SFX("coin")

    if(screen == STATE_MAP):
        if (current_music != 1):
            PLAY_MUSIC("map_theme")
            current_music = 1
        
        # Poll joystick only every 15 frames (to reduce CPU load from Joy_Analog)
        joystick_poll_counter += 1
        if joystick_poll_counter >= 15:
            joystick_poll_counter = 0
            joy_x = joystick1_state[0]
            joy_y = joystick1_state[1]
        
        # Navigate locations with joystick (circular navigation)
        # joy_x/joy_y are now raw values (-127 to +127), use thresholds
        if joy_x > 40 and prev_joy_x <= 40:  # Right threshold
            current_location = current_location + 1
            if current_location >= num_locations:
                current_location = 0
        elif joy_x < -40 and prev_joy_x >= -40:  # Left threshold
            current_location = current_location - 1
            if current_location < 0:
                current_location = num_locations - 1
        elif joy_y > 40 and prev_joy_y <= 40:  # Up threshold
            current_location = current_location + 1
            if current_location >= num_locations:
                current_location = 0
        elif joy_y < -40 and prev_joy_y >= -40:  # Down threshold
            current_location = current_location - 1
            if current_location < 0:
                current_location = num_locations - 1
        
        prev_joy_x = joy_x
        prev_joy_y = joy_y
        
        btn1 = joystick1_state[2]
        
        if btn1 == 1:
            # Start level - transition to game state
            screen = STATE_GAME
            countdown_active = 1
            countdown_timer = 180  # 3 seconds at 60fps

        draw_map_screen()
    
    if screen == STATE_GAME:
        # Countdown "GET READY" phase
        if countdown_active == 1:
            #Spawn enemies only once when countdown starts
            if countdown_timer >= 180:
                spawn_level_enemies()
            
            # Draw background and enemies during countdown
            draw_level_background()
             
            # Update and draw enemies during countdown
            #update_enemies()
            #draw_enemies()
            
            SET_INTENSITY(127)
            PRINT_TEXT(-50, 0, "GET READY")
            
            # Show selected location name
            SET_INTENSITY(100)
            PRINT_TEXT(-85, -20, location_names[current_location])
            
            # Decrement countdown timer
            countdown_timer = countdown_timer - 1
            
            # When countdown finishes, start actual game
            if countdown_timer <= 0:
                countdown_active = 0
        else:
            # Actual game after countdown
            
            # Check for shoot input (any of 4 buttons)
            if hook_active == 0:
                if (joystick1_state[2] == 1 or joystick1_state[3] == 1 or joystick1_state[4] == 1 or joystick1_state[5] == 1):
                    hook_active = 1
                    hook_y = -100  # Start from ground level (direct signed value)
                    
                    # Capture gun position at fire time
                    hook_gun_x = player_x
                    if player_facing == 1:
                        hook_gun_x = player_x + 11
                    else:
                        hook_gun_x = player_x - 11
                    hook_gun_y = player_y + 3
                    hook_init_y = hook_gun_y  # Save initial Y for rope start point (stays fixed)
                    
                    # Hook moves vertically from gun position
                    hook_x = hook_gun_x
            
            # Update hook position if active
            if hook_active == 1:
                hook_y = hook_y + 3  # Move hook upward

                # Check if hook reached top
                if hook_y >= hook_max_y:
                    hook_active = 0
                    hook_y = -100
            
            draw_game_level()

def draw_map_screen():
    # Draw background map
    SET_INTENSITY(80)
    DRAW_VECTOR_EX("map", 0, 20, 0, 50)
    
    # Update glow intensity (oscillating effect)
    if location_glow_direction == 0:
        location_glow_intensity = location_glow_intensity + 3
        if location_glow_intensity >= 127:
            location_glow_direction = 1
    else:
        location_glow_intensity = location_glow_intensity - 3
        if location_glow_intensity <= 80:
            location_glow_direction = 0
    
    PRINT_TEXT(-120, -80, location_names[current_location])

    # Draw only the selected location with glow effect
    loc_x = location_x_coords[current_location]
    loc_y = location_y_coords[current_location]

    DRAW_VECTOR_EX("location_marker", loc_y, loc_x, 0, location_glow_intensity)
    

def draw_title_screen():
    #Dibujar logo centrado
    SET_INTENSITY(80)
    DRAW_VECTOR("logo", 0, 20)

    SET_INTENSITY(title_intensity)
    PRINT_TEXT(-90, -40, "PRESS A BUTTON")
    PRINT_TEXT(-50, -60, "TO START")

    if (title_state == 0):
        title_intensity += 1

    if (title_state == 1):
        title_intensity -= 1    

    if (title_intensity == 80):
        title_state = 1
    
    if (title_intensity == 30):
        title_state = 0

def draw_level_background():
    # Draws the background specific to current_location
    SET_INTENSITY(60)
    
    # Select background based on current_location (0-16)
    if current_location == 0:
        DRAW_VECTOR("fuji_bg", 0, 50)
    elif current_location == 1:
        DRAW_VECTOR("keirin_bg", 0, 50)
    elif current_location == 2:
        DRAW_VECTOR("buddha_bg", 0, 50)
    elif current_location == 3:
        DRAW_VECTOR("angkor_bg", 0, 50)
    elif current_location == 4:
        DRAW_VECTOR("ayers_bg", 0, 50)
    elif current_location == 5:
        DRAW_VECTOR("taj_bg", 0, 50)
    elif current_location == 6:
        DRAW_VECTOR("leningrad_bg", 0, 50)
    elif current_location == 7:
        DRAW_VECTOR("paris_bg", 0, 50)
    elif current_location == 8:
        DRAW_VECTOR("london_bg", 0, 50)
    elif current_location == 9:
        DRAW_VECTOR("barcelona_bg", 0, 50)
    elif current_location == 10:
        DRAW_VECTOR("athens_bg", 0, 50)
    elif current_location == 11:
        DRAW_VECTOR("pyramids_bg", 0, 50)
    elif current_location == 12:
        DRAW_VECTOR("kilimanjaro_bg", 0, 50)
    elif current_location == 13:
        DRAW_VECTOR("newyork_bg", 0, 50)
    elif current_location == 14:
        DRAW_VECTOR("mayan_bg", 0, 50)
    elif current_location == 15:
        DRAW_VECTOR("antarctica_bg", 0, 50)
    else:  # current_location == 16
        DRAW_VECTOR("easter_bg", 0, 50)

def draw_game_level():
    # Draw level-specific background
    draw_level_background()
    
    # Handle player movement - use analog values for speed control
    joy_x = joystick1_state[0]
    # Note: No axis inversion needed - hardware provides correct values
    
    # Apply deadzone threshold (±20)
    if joy_x < -20 or joy_x > 20:
        # Simple proportional speed: map joystick to 1-4 pixels per frame
        # joy_x range: -127 to +127 (excluding deadzone ±20)
        abs_joy = joy_x
        if abs_joy < 0:
            abs_joy = -abs_joy
        
        # Map 20-127 to speed 1-4
        # At minimum tilt (20): speed = 1
        # At maximum tilt (127): speed = 4
        if abs_joy < 40:
            move_speed = 1
        elif abs_joy < 70:
            move_speed = 2
        elif abs_joy < 100:
            move_speed = 3
        else:
            move_speed = 4
        
        # Apply direction
        if joy_x < 0:
            move_speed = -move_speed
        
        player_x = player_x + move_speed
        
        # Clamp to screen bounds
        if player_x < -110:
            player_x = -110
        if player_x > 110:
            player_x = 110
        
        # Update facing direction
        if joy_x < 0:
            player_facing = -1
        else:
            player_facing = 1
        
        # Animate walking (faster animation with higher speed)
        player_anim_counter = player_anim_counter + 1
        # Animation speed inversely proportional to joystick value
        anim_threshold = player_anim_speed
        if joy_x < -80 or joy_x > 80:
            anim_threshold = player_anim_speed / 2  # Faster animation
        
        if player_anim_counter >= anim_threshold:
            player_anim_counter = 0
            player_anim_frame = player_anim_frame + 1
            if player_anim_frame > 5:
                player_anim_frame = 1
    else:
        # Not moving - reset to idle frame
        player_anim_frame = 1
        player_anim_counter = 0
   
    # Determine mirror mode based on facing direction
    mirror_mode = 0
    if player_facing == -1:
        mirror_mode = 1  # Mirror X (flip horizontally)
    
    # Draw animated sprite
    if player_anim_frame == 1:
        DRAW_VECTOR_EX("player_walk_1", player_x, player_y, mirror_mode, 80)
    elif player_anim_frame == 2:
        DRAW_VECTOR_EX("player_walk_2", player_x, player_y, mirror_mode, 80)
    elif player_anim_frame == 3:
        DRAW_VECTOR_EX("player_walk_3", player_x, player_y, mirror_mode, 80)
    elif player_anim_frame == 4:
        DRAW_VECTOR_EX("player_walk_4", player_x, player_y, mirror_mode, 80)
    else:
        DRAW_VECTOR_EX("player_walk_5", player_x, player_y, mirror_mode, 80)
    
    # Update and draw enemies
    #update_enemies()
    #draw_enemies()
    
    # Draw hook rope and hook if active
    if hook_active == 1:
        # Draw the rope from the initial gun position (fixed) to the current hook position
        # Both hook_init_y and hook_y are direct signed values (-100..72)
        draw_hook_rope(hook_gun_x, hook_init_y, hook_x, hook_y)
        # Draw hook at the end of the rope
        SET_INTENSITY(100)

        DRAW_VECTOR_EX("hook", hook_x, hook_y, 0, 100)
    
    # DEBUG: Show enemy count
    active_count = 0
    i = 0
    while i < MAX_ENEMIES:
        if enemy_active[i] == 1:
            active_count = active_count + 1
        i = i + 1

def init_enemies():
    # Initialize enemy system (called once at startup)
    i = 0
    while i < MAX_ENEMIES:
        enemy_active[i] = 0
        enemy_x[i] = 0
        enemy_y[i] = 0
        enemy_vx[i] = 0
        enemy_vy[i] = 0
        enemy_size[i] = 0
        i = i + 1

def spawn_level_enemies():
    # Spawn enemies for current level
    num_enemies = level_enemy_count[current_location]
    base_speed = level_enemy_speed[current_location]
  
    i = 0
    while i < num_enemies and i < MAX_ENEMIES:
        enemy_active[i] = 1
        enemy_x[i] = -60 + (i * 40)  # Spread across screen
        enemy_y[i] = 0
        
        # Initial velocity
        enemy_vx[i] = base_speed
        if i % 2 == 0:
            enemy_vx[i] = -base_speed
        
        enemy_vy[i] = 0  # Start falling
        enemy_size[i] = 4  # Start with huge bubbles
        
        i = i + 1

def update_enemies():
    # Update physics for all active enemies
    i = 0
    while i < MAX_ENEMIES:
        if enemy_active[i] == 1:
            enemy_vy[i] = enemy_vy[i] - GRAVITY
            
            # Update position
            enemy_x[i] = enemy_x[i] + enemy_vx[i]
            enemy_y[i] = enemy_y[i] + enemy_vy[i]
            
            # Get bubble radius based on size (huge=25, large=20, medium=15, small=10)
            radius = 10
            if enemy_size[i] == 4:
                radius = 25
            elif enemy_size[i] == 3:
                radius = 20
            elif enemy_size[i] == 2:
                radius = 15
            
            # Bounce off walls
            if enemy_x[i] < -110 + radius:
                enemy_x[i] = -110 + radius
                enemy_vx[i] = -enemy_vx[i]
            elif enemy_x[i] > 110 - radius:
                enemy_x[i] = 110 - radius
                enemy_vx[i] = -enemy_vx[i]
            
            # Bounce off ground
            if enemy_y[i] < GROUND_Y + radius:
                enemy_y[i] = GROUND_Y + radius
                enemy_vy[i] = -enemy_vy[i]
                # Apply damping (90% bounce)
                enemy_vy[i] = (enemy_vy[i] * BOUNCE_DAMPING) / 10
            
            # Bounce off ceiling
            if enemy_y[i] > 110:
                enemy_y[i] = 110
                enemy_vy[i] = -enemy_vy[i]
        
        i = i + 1

def draw_enemies():
    # Draw all active enemies
    i = 0
    while i < MAX_ENEMIES:
        if enemy_active[i] == 1:
            SET_INTENSITY(80)
            # Draw bubble based on size (4=huge, 3=large, 2=medium, 1=small)
            if enemy_size[i] == 4:
                DRAW_VECTOR("bubble_huge", enemy_x[i], enemy_y[i])
            elif enemy_size[i] == 3:
                DRAW_VECTOR("bubble_large", enemy_x[i], enemy_y[i])
            elif enemy_size[i] == 2:
                DRAW_VECTOR("bubble_medium", enemy_x[i], enemy_y[i])
            else:
                DRAW_VECTOR("bubble_small", enemy_x[i], enemy_y[i])
        
        i = i + 1

def draw_hook_rope(start_x, start_y, end_x, end_y):
    # Draw a straight line from gun position to hook position (using direct signed values)
    DRAW_LINE(start_x, start_y, end_x, end_y, 127)

def read_joystick1_state():
    
    joystick1_state [0] = J1_X()
    joystick1_state [1] = J1_Y()
    
    # Button 1: Only read when we detect hardware released (J1_BUTTON_1 == 0)
    # If hardware shows button pressed (1) while we have it locked (1), keep it locked
    # If hardware shows released (0), allow new press next time
    if joystick1_state [2] == 0:
        # Our state says not pressed, check hardware
        if J1_BUTTON_1() == 1:
            # Hardware pressed, lock it in
            joystick1_state [2] = 1
    else:
        # Our state says pressed, wait for hardware release
        if J1_BUTTON_1() == 0:
            # Hardware released, unlock
            joystick1_state [2] = 0
    
    # Button 2: Same logic
    if joystick1_state [3] == 0:
        if J1_BUTTON_2() == 1:
            joystick1_state [3] = 1
    else:
        if J1_BUTTON_2() == 0:
            joystick1_state [3] = 0
    
    # Button 3: Same logic
    if joystick1_state [4] == 0:
        if J1_BUTTON_3() == 1:
            joystick1_state [4] = 1
    else:
        if J1_BUTTON_3() == 0:
            joystick1_state [4] = 0
    
    # Button 4: Same logic
    if joystick1_state [5] == 0:
        if J1_BUTTON_4() == 1:
            joystick1_state [5] = 1
    else:
        if J1_BUTTON_4() == 0:
            joystick1_state [5] = 0


