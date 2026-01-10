META TITLE = "BigGame - Multi-Bank Test"
META MUSIC = "0"
META YEAR = "2026"
META COPYRIGHT = "Test Project"
META ROM_TOTAL_SIZE = 524288  # 512KB ROM (32 banks x 16KB)

# This project tests multi-bank compilation with many functions
# to force usage of multiple banks

# ============================================
# LEVEL 1 - Functions (should go to Bank #0)
# ============================================

def level1_init():
    SET_INTENSITY(100)
    PRINT_TEXT(-50, 50, "LEVEL 1")

def level1_update():
    x = 10
    y = 20
    DRAW_LINE(x, y, x+20, y+20, 80)

def level1_render():
    SET_INTENSITY(127)
    DRAW_LINE(-30, -30, 30, 30, 100)

def level1_enemy1():
    enemy_x = 0
    enemy_y = 0
    enemy_x += 1

def level1_enemy2():
    enemy_x = 10
    enemy_y = 10
    enemy_x -= 1

def level1_collision():
    hit = 0
    if hit == 0:
        hit = 1

# ============================================
# LEVEL 2 - Functions (should go to Bank #1 or #0)
# ============================================

def level2_init():
    SET_INTENSITY(110)
    PRINT_TEXT(-50, 40, "LEVEL 2")

def level2_update():
    x = 15
    y = 25
    DRAW_LINE(x, y, x+25, y+25, 90)

def level2_render():
    SET_INTENSITY(120)
    DRAW_LINE(-40, -40, 40, 40, 110)

def level2_enemy1():
    enemy_x = 5
    enemy_y = 5
    enemy_x += 2

def level2_enemy2():
    enemy_x = 15
    enemy_y = 15
    enemy_x -= 2

def level2_boss():
    boss_x = 0
    boss_y = 0
    boss_hp = 100

# ============================================
# LEVEL 3 - Functions (should go to Bank #2 or #1)
# ============================================

def level3_init():
    SET_INTENSITY(120)
    PRINT_TEXT(-50, 30, "LEVEL 3")

def level3_update():
    x = 20
    y = 30
    DRAW_LINE(x, y, x+30, y+30, 100)

def level3_render():
    SET_INTENSITY(110)
    DRAW_LINE(-50, -50, 50, 50, 120)

def level3_enemy1():
    enemy_x = 10
    enemy_y = 10
    enemy_x += 3

def level3_enemy2():
    enemy_x = 20
    enemy_y = 20
    enemy_x -= 3

def level3_boss():
    boss_x = 0
    boss_y = 0
    boss_hp = 150

# ============================================
# MENU - Functions (could go to separate bank)
# ============================================

def menu_init():
    SET_INTENSITY(127)
    PRINT_TEXT(-60, 60, "MAIN MENU")

def menu_update():
    joy_x = J1_X()
    joy_y = J1_Y()
    
def menu_render():
    SET_INTENSITY(127)
    DRAW_LINE(-60, 40, 60, 40, 127)

# ============================================
# HUD - Functions (could go to separate bank)
# ============================================

def hud_draw_score():
    PRINT_TEXT(-100, 100, "SCORE")

def hud_draw_lives():
    PRINT_TEXT(-100, 90, "LIVES")

def hud_draw_level():
    PRINT_TEXT(-100, 80, "LEVEL")

# ============================================
# GAME STATE - Global variables
# ============================================

current_level = 0
player_score = 0
player_lives = 3
game_state = 0  # 0=menu, 1=playing, 2=game_over

# ============================================
# MAIN - Entry point (Bank #31 - Fixed)
# ============================================

def main():
    SET_INTENSITY(127)
    current_level = 0
    game_state = 0
    menu_init()

# ============================================
# LOOP - Main game loop (Bank #31 - Fixed)
# ============================================

def loop():
    # Game state machine with cross-bank calls
    if game_state == 0:
        # Menu
        menu_update()
        menu_render()
    elif game_state == 1:
        # Playing
        if current_level == 0:
            level1_update()
            level1_render()
        elif current_level == 1:
            level2_update()
            level2_render()
        elif current_level == 2:
            level3_update()
            level3_render()
        
        # HUD (cross-bank call)
        hud_draw_score()
        hud_draw_lives()
        hud_draw_level()
