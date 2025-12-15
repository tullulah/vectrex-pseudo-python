META TITLE = "JETPAC"
META COPYRIGHT = "g 2025"
META MUSIC = 1

# === GAME STATE ===
var screen = 0          # 0=logo, 1=game
var logo_timer = 0      # Frame counter for logo
var game_state = 0      # 0=collect parts, 1=refuel, 2=complete
var fuel_level = 0

# === PLAYER ===
var player_x = 0
var player_y = -80
var player_dx = 0
var player_dy = 0

# === SHIP PARTS (0=on platform, 1=carried, 2=assembled) ===
var part1_state = 0     # Bottom section
var part2_state = 0     # Middle section  
var part3_state = 0     # Top section
var carried_part = 0    # Which part player carries (0=none)

# === PLATFORMS (Y coordinates) ===
var plat1_y = -70       # Bottom
var plat2_y = -20       # Middle
var plat3_y = 30        # Top

# === SHIP PARTS POSITIONS ===
var part1_x = -80
var part1_y = -55
var part2_x = 0  
var part2_y = -5
var part3_x = 80
var part3_y = 45

# === SHIP ASSEMBLY POSITION ===
var ship_base_x = 0
var ship_base_y = -85

# === ENEMY (DECLARADAS GLOBALMENTE) ===
var enemy_x = 100
var enemy_y = 50
var enemy_dx = -2

def main():
    # Initialization
    PLAY_MUSIC("jetpac_theme") 
    let i =0

def loop():
    MUSIC_UPDATE()
    
    if screen == 0:
        draw_logo_screen()
    if screen == 1:
        draw_game_screen()

def draw_logo_screen():
    # Draw logo usando jetpac_logo.vec
    SET_INTENSITY(127)
    DRAW_VECTOR("jetpac_logo", 0, 20)
    
    # # # Copyright
    # SET_INTENSITY(80)
    # PRINT_TEXT(-50, -60, "2025")
    
    # # Timer
    logo_timer = logo_timer + 1
    if logo_timer > 300:    # 5 seconds at 60 FPS
        screen = 1
        logo_timer = 0

def draw_game_screen():
    # === DRAW PLATFORMS usando platform.vec ===
    SET_INTENSITY(80)
    DRAW_VECTOR("platform", -80, plat1_y)
    DRAW_VECTOR("platform", 0, plat2_y)
    DRAW_VECTOR("platform", 80, plat3_y)
    
    # === DRAW SHIP PARTS (not yet assembled) ===
    if part1_state == 0:
        SET_INTENSITY(127)
        DRAW_VECTOR("rocket_base", part1_x, part1_y)
    
    if part2_state == 0:
        SET_INTENSITY(127)
        DRAW_VECTOR("rocket_middle", part2_x, part2_y)
    
    if part3_state == 0:
        SET_INTENSITY(127)
        DRAW_VECTOR("rocket_top", part3_x, part3_y)
    
    # === DRAW ASSEMBLED SHIP ===
    if part1_state == 2:
        SET_INTENSITY(127)
        DRAW_VECTOR("rocket_base", ship_base_x, ship_base_y)
    
    if part2_state == 2:
        SET_INTENSITY(127)
        DRAW_VECTOR("rocket_middle", ship_base_x, ship_base_y + 15)
    
    if part3_state == 2:
        SET_INTENSITY(127)
        DRAW_VECTOR("rocket_top", ship_base_x, ship_base_y + 30)
    
    # === DRAW PLAYER (astronaut.vec) ===
    SET_INTENSITY(127)
    DRAW_VECTOR("astronaut", player_x, player_y)
    
    # === DRAW CARRIED PART ===
    if carried_part == 1:
        SET_INTENSITY(100)
        DRAW_VECTOR("rocket_base", player_x, player_y + 12)
    if carried_part == 2:
        SET_INTENSITY(100)
        DRAW_VECTOR("rocket_middle", player_x, player_y + 12)
    if carried_part == 3:
        SET_INTENSITY(100)
        DRAW_VECTOR("rocket_top", player_x, player_y + 12)
    
    # === DRAW ENEMY ===
    SET_INTENSITY(127)
    DRAW_VECTOR("enemy_alien", enemy_x, enemy_y)
    
    # === UPDATE ENEMY MOVEMENT ===
    enemy_x = enemy_x + enemy_dx
    if enemy_x < -100:
        enemy_dx = 2
    if enemy_x > 100:
        enemy_dx = -2
    
    # # === DRAW UI ===
    # SET_INTENSITY(64)
    # PRINT_TEXT(-110, 110, "FUEL")
    
    # Fuel bar
    let bar_width = fuel_level / 2
    if bar_width > 0:
        SET_INTENSITY(80)
        MOVE(-110, 100)
        DRAW_TO(-110 + bar_width, 100)
