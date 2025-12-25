# pang - Main entry point
# VPy game for Vectrex

META TITLE = "PANG"

screen = 0
title_intensity = 30
title_state = 0 # 0 up 1 down
current_music = -1  # Track which music is playing (-1=none, 0=pang_theme, 1=map_theme)
delay = 0
stage = 0

# Location selection system
location_y_coords = [0, 0]  # Y coordinates for locations (adjust as needed)
location_x_coords = [0, 0]  # X coordinates for locations (adjust as needed)
num_locations = 2
current_location = 0  # Currently selected location (0 to num_locations-1)
location_glow_intensity = 60
location_glow_direction = 0  # 0 = increasing, 1 = decreasing
prev_joy_x = 0  # Previous joystick X state for edge detection
intensityval = 0


def main():
    # Called once at startup - START MUSIC HERE (before first loop frame)
    
    screen = 0

def loop():
    # Pantalla de presentación con logo
    SET_INTENSITY(127)
    
    if screen == 0: # Music already started in main()
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
            screen = 1 
            current_music = -1  # Reset music tracker to trigger map_theme

    if(screen == 1):
        # Play gameplay music (only start if not already playing)
        if (delay ==0):
            STOP_MUSIC()

        if current_music != 1 and delay == 10:
            PLAY_MUSIC("map_theme")
            current_music = 1
        
        if(delay<10):
            delay += 1

        draw_map_screen()

def draw_map_screen():
    # Draw background map
    SET_INTENSITY(100)
    DRAW_VECTOR("map", 0, 20)
    
    #Joystick navigation for location selection
    joy_x = J1_X()
    
    # Edge detection: only trigger on joystick press (not continuously held)
    if joy_x == 1 and prev_joy_x == 0:  # RIGHT pressed
        current_location = current_location + 1
        if current_location >= num_locations:
            current_location = 0
    elif joy_x == -1 and prev_joy_x == 0:  # LEFT pressed
        current_location = current_location - 1
        if current_location < 0:
            current_location = num_locations - 1
    
    prev_joy_x = joy_x
    
    # Draw location markers
    i = 0
    while i < num_locations:
        y_coord = location_y_coords[i]
        x_coord = location_x_coords[i]
        
        # Glow effect on currently selected location
        if i == current_location:
            intensityval = location_glow_intensity
        else:
            intensityval = 70  # Dimmer for non-selected
        
        # Draw marker polygon (small triangle pointing up)
        DRAW_POLYGON(3, intensityval, x_coord, y_coord + 6, x_coord - 5, y_coord - 6, x_coord + 5, y_coord - 6)
        
        i = i + 1
    
    # Update glow intensity (oscillating effect)
    if location_glow_direction == 0:
        location_glow_intensity = location_glow_intensity + 3
        if location_glow_intensity >= 127:
            location_glow_direction = 1
    else:
        location_glow_intensity = location_glow_intensity - 3
        if location_glow_intensity <= 50:
            location_glow_direction = 0


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
