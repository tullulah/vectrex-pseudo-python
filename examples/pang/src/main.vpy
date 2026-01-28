# pang - Main entry point
# VPy game for Vectrex

META TITLE = "PANG"
META MUSIC = music1

# Game states
const STATE_TITLE = 0
const STATE_MAP = 1
const  STATE_GAME = 2

screen = STATE_TITLE
title_intensity = 30
title_state = 0 # 0 up 1 down
current_music = -1  # Track which music is playing (-1=none, 0=pang_theme, 1=map_theme)

# Location selection system - 17 locations matching the arcade game
const location_x_coords = [40, 40, -40, -10, 20, 50, 80, -85, -50, -15, 15, 50, 85, -90, -45, 0, 45]
const location_y_coords = [110, 79, -20, 10, 40, 70, 100, -40, -10, 30, 60, 90, 20, 50, 0, -60, -30]
const location_names = ["MOUNT FUJI (JP)","MOUNT KEIRIN (CN)","EMERALD BUDDHA TEMPLE (TH)", "ANGKOR WAT (KH)","AYERS ROCK (AU)", "TAJ MAHAL (IN)","LENINGRAD (RU)","PARIS (FR)","LONDON (UK)","BARCELONA (ES)","ATHENS (GR)", "PYRAMIDS (EG)", "MOUNT KILIMANJARO (TZ)", "NEW YORK (US)","MAYAN RUINS (MX)","ANTARCTICA (AQ)","EASTER ISLAND (CL)"];

# Level names for LOAD_LEVEL() calls
const level_files = ["01_mount_fuji", "02_mount_keirin", "03_emerald_buddha", "04_angkor_wat", "05_ayers_rock", "06_taj_mahal", "07_leningrad", "08_paris", "09_london", "10_barcelona", "11_athens", "12_pyramids", "13_kilimanjaro", "14_new_york", "15_mayan_ruins", "16_antarctica", "17_easter_island"]

# Current loaded level data
current_level_ptr = 0    # Pointer to loaded level data

joystick1_state = [0,0,0,0,0,0] # X, Y, BTN1, BTN2, BTN3, BTN4

const num_locations = 17
current_location = 0
location_glow_intensity = 60
location_glow_direction = 0
joy_x = 0
joy_y = 0
prev_joy_x = 0
prev_joy_y = 0

# Game state variables
countdown_timer = 0      # Countdown from 180 frames (~3 seconds at 60fps)
countdown_active = 0     # 0=inactive, 1=active
joystick_poll_counter = 0  # Counter to poll joystick every N frames

# Hook (disparo) system
hook_active = 0          # 0=inactive, 1=active
hook_x = 0               # X position of hook (captured at fire time)
hook_y = -70            # Y position of hook (starts at ground level)
const hook_max_y = 127          # Maximum Y position (well up the screen, signed value)
hook_gun_x = 0           # Gun X position at fire time (stays fixed while hook travels)
hook_gun_y = 0           # Gun Y position at fire time (stays fixed while hook travels)
hook_init_y = 0          # Initial Y position of gun (stays fixed for rope start point)
# Player variables
player_x = 0             # Player X position
const player_y = -70          # Player Y position (on ground)
move_speed = 0           # Current movement speed (calculated from joystick)
abs_joy = 0              # Helper for absolute value calculation
player_facing = 1        # Direction facing: 1=right, -1=left
player_anim_id = -1      # Animation instance ID

# Enemy (bubble) system - maximum 8 enemies
const MAX_ENEMIES = 8
enemy_active = [0, 0, 0, 0, 0, 0, 0, 0]        # 0=inactive, 1=active
enemy_x = [0, 0, 0, 0, 0, 0, 0, 0]             # X positions
enemy_y = [0, 0, 0, 0, 0, 0, 0, 0]             # Y positions
enemy_vx = [0, 0, 0, 0, 0, 0, 0, 0]            # X velocities
enemy_vy = [0, 0, 0, 0, 0, 0, 0, 0]            # Y velocities
enemy_size = [0, 0, 0, 0, 0, 0, 0, 0]          # Size: 4=huge, 3=large, 2=medium, 1=small

# Physics constants (pompas de jabón - gravedad ligera)
const GRAVITY = 1         # Gravedad ligera para efecto pompa de jabón
const BOUNCE_DAMPING = 17 # Velocity retained after bounce (17/20 = 85%) - pompas mantienen energía
const MIN_BOUNCE_VY = 10  # Velocidad mínima de rebote para evitar vibraciones (debe superar gravedad acumulada)
const GROUND_Y = -70      # Y position of ground


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
    
    # Initialize hook system
    hook_active = 0
    hook_x = 0
    hook_y = -70
    
    # Initialize player animation
    player_anim_id = CREATE_ANIM("player_anim")

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
            PLAY_SFX("laser")

    elif(screen == STATE_MAP):
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
        
        if joystick1_state[2] == 1 or joystick1_state[3] == 1 or joystick1_state[4] == 1 or joystick1_state[5] == 1:
            # Start level - transition to game state
            PLAY_SFX("laser")
            
            # Load the selected level (using if chain for string literal optimization)
            if current_location == 0:
                current_level_ptr = LOAD_LEVEL("01_mount_fuji")
            elif current_location == 1:
                current_level_ptr = LOAD_LEVEL("02_mount_keirin")
            elif current_location == 2:
                current_level_ptr = LOAD_LEVEL("03_emerald_buddha")
            elif current_location == 3:
                current_level_ptr = LOAD_LEVEL("04_angkor_wat")
            elif current_location == 4:
                current_level_ptr = LOAD_LEVEL("05_ayers_rock")
            elif current_location == 5:
                current_level_ptr = LOAD_LEVEL("06_taj_mahal")
            elif current_location == 6:
                current_level_ptr = LOAD_LEVEL("07_leningrad")
            elif current_location == 7:
                current_level_ptr = LOAD_LEVEL("08_paris")
            elif current_location == 8:
                current_level_ptr = LOAD_LEVEL("09_london")
            elif current_location == 9:
                current_level_ptr = LOAD_LEVEL("10_barcelona")
            elif current_location == 10:
                current_level_ptr = LOAD_LEVEL("11_athens")
            elif current_location == 11:
                current_level_ptr = LOAD_LEVEL("12_pyramids")
            elif current_location == 12:
                current_level_ptr = LOAD_LEVEL("13_kilimanjaro")
            elif current_location == 13:
                current_level_ptr = LOAD_LEVEL("14_new_york")
            elif current_location == 14:
                current_level_ptr = LOAD_LEVEL("15_mayan_ruins")
            elif current_location == 15:
                current_level_ptr = LOAD_LEVEL("16_antarctica")
            else:  # 16
                current_level_ptr = LOAD_LEVEL("17_easter_island")
            
            screen = STATE_GAME
            countdown_active = 1
            countdown_timer = 180  # 3 seconds at 60fps

        draw_map_screen()
    
    elif screen == STATE_GAME:
        # Countdown "GET READY" phase
        if countdown_active == 1:
            # Draw background during countdown (no enemies)
            draw_level_background()
            
            SET_INTENSITY(127)
            PRINT_TEXT(-50, 0, "GET READY")
            
            # Show selected location name
            SET_INTENSITY(127)
            PRINT_TEXT(-85, 120, location_names[current_location])
            
            # Decrement countdown timer
            countdown_timer = countdown_timer - 1
            
            # When countdown finishes, spawn and start game
            if countdown_timer <= 0:
                countdown_active = 0
                spawn_enemies()
        else:
            # Actual game after countdown
            
            # Check for shoot input (any of 4 buttons)
            if hook_active == 0:
                if (joystick1_state[2] == 1 or joystick1_state[3] == 1 or joystick1_state[4] == 1 or joystick1_state[5] == 1):
                    hook_active = 1
                    hook_y = -70  # Start from ground level (direct signed value)
                    PLAY_SFX("hit")
                    
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
                    hook_y = -70
            
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
    
    PRINT_TEXT(-120, 120, location_names[current_location])

    # Draw only the selected location with glow effect
    loc_x = location_x_coords[current_location]
    loc_y = location_y_coords[current_location]

    DRAW_VECTOR_EX("location_marker", loc_y, loc_x, 0, location_glow_intensity)
    

def draw_title_screen():
    #Dibujar logo centrado
    SET_INTENSITY(80)
    DRAW_VECTOR("logo", 0, 70)

    SET_INTENSITY(title_intensity)
    PRINT_TEXT(-90, 0, "PRESS A BUTTON")
    PRINT_TEXT(-50, -20, "TO START")

    if (title_state == 0):
        title_intensity += 1

    if (title_state == 1):
        title_intensity -= 1    

    if (title_intensity == 80):
        title_state = 1
    
    if (title_intensity == 30):
        title_state = 0

def draw_level_background():
    # Draws the background from loaded level data
    if current_level_ptr != 0:
        SHOW_LEVEL()

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
        
        # Set animation state to walking
        SET_ANIM_STATE(player_anim_id, 1)  # State 1 = walking
    else:
        # Not moving - set to idle
        SET_ANIM_STATE(player_anim_id, 0)  # State 0 = idle
   
    # Set mirror based on facing direction
    mirror_mode = 0
    if player_facing == -1:
        mirror_mode = 1  # Mirror X (flip horizontally)
    SET_ANIM_MIRROR(player_anim_id, mirror_mode)
    
    # Update and draw player animation
    UPDATE_ANIM(player_anim_id, player_x, player_y)
    DRAW_ANIM(player_anim_id)
    
    # Update and draw enemies
    update_enemies()
    draw_enemies()
    
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

# ===== ENEMY SYSTEM - Simple and Clear =====

def spawn_enemies():
    #"""Spawn enemies for current level - called ONCE when game starts"""
    # TODO: Get from loaded level gameConfig
    count = 1  # Start simple for testing
    speed = 1
    
    i = 0
    while i < count:
        enemy_active[i] = 1
        enemy_size[i] = 4        # Size 4 (huge bubble, radius 25)
        enemy_x[i] = -80 + (i * 50)  # Spread horizontally
        enemy_y[i] = 60          # Start high up
        enemy_vx[i] = speed      # Horizontal speed
        if i % 2 == 1:
            enemy_vx[i] = -speed # Alternate direction
        enemy_vy[i] = 0          # Start stationary (gravity will pull down)
        i = i + 1

def update_enemies():
    #"""Update all active enemies - called every frame during gameplay"""
    i = 0
    while i < MAX_ENEMIES:
        if enemy_active[i] == 1:
            # Apply gravity directly to array
            enemy_vy[i] = enemy_vy[i] - GRAVITY
            
            # Move directly with arrays
            enemy_x[i] = enemy_x[i] + enemy_vx[i]
            enemy_y[i] = enemy_y[i] + enemy_vy[i]
            
            # Ground bounce (pompas de jabón - mantienen rebote)
            if enemy_y[i] <= GROUND_Y:
                enemy_y[i] = GROUND_Y
                enemy_vy[i] = -enemy_vy[i]
                enemy_vy[i] = (enemy_vy[i] * BOUNCE_DAMPING) / 20
                # Asegurar velocidad mínima para que no se queden estáticas
                if enemy_vy[i] < MIN_BOUNCE_VY:
                    enemy_vy[i] = MIN_BOUNCE_VY
            
            # Wall bounce
            if enemy_x[i] <= -85:
                enemy_x[i] = -85
                enemy_vx[i] = -enemy_vx[i]
            if enemy_x[i] >= 85:
                enemy_x[i] = 85
                enemy_vx[i] = -enemy_vx[i]
        
        i = i + 1
       


def draw_enemies():
    #"""Draw all active enemies - called every frame during gameplay"""
    i = 0
    while i < MAX_ENEMIES:
        if enemy_active[i] == 1:
            SET_INTENSITY(80)
            if enemy_size[i] == 4:
                DRAW_VECTOR("bubble_huge", enemy_x[i], enemy_y[i])
            elif enemy_size[i] == 3:
                DRAW_VECTOR("bubble_large", enemy_x[i], enemy_y[i])
            elif enemy_size[i] == 2:
                DRAW_VECTOR("bubble_medium", enemy_x[i], enemy_y[i])
            else:
                DRAW_VECTOR("bubble_small", enemy_x[i], enemy_y[i])
        i = i + 1

# ===== END ENEMY SYSTEM =====

def draw_hook_rope(start_x, start_y, end_x, end_y):
    # Draw a straight line from gun position to hook position (using direct signed values)
    DRAW_LINE(start_x, start_y, end_x, end_y, 127)

def read_joystick1_state():
    
    # UPDATE_BUTTONS is now auto-injected at start of loop() by compiler
    # No need to call it explicitly - $C80F is already populated
    
    joystick1_state[0] = J1_X()
    joystick1_state[1] = J1_Y()
        
    # Read cached button states (fast, no BIOS calls)
    joystick1_state[2] = J1_BUTTON_1()
    joystick1_state[3] = J1_BUTTON_2()
    joystick1_state[4] = J1_BUTTON_3()
    joystick1_state[5] = J1_BUTTON_4()