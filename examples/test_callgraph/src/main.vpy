# Test: Call graph analysis with multiple functions

META TITLE = "Call Graph Test"
META ROM_TOTAL_SIZE = 524288
META ROM_BANK_SIZE = 16384

# Enemy positions
enemy1_x = -50
enemy1_y = 60
enemy2_x = 0
enemy2_y = 0
enemy3_x = 50
enemy3_y = -60

def main():
    SET_INTENSITY(127)
    init_game()

def init_game():
    load_assets()
    setup_player()

def load_assets():
    pass

def setup_player():
    pass

def game_loop():
    update_player()
    update_enemies()
    draw_all()

def update_player():
    check_input()
    move_player()

def check_input():
    pass

def move_player():
    pass

def update_enemies():
    # Move enemy 1 horizontally
    enemy1_x = enemy1_x + 1
    if enemy1_x > 100:
        enemy1_x = -100
    
    # Move enemy 2 vertically
    enemy2_y = enemy2_y + 1
    if enemy2_y > 100:
        enemy2_y = -100
    
    # Move enemy 3 diagonally
    enemy3_x = enemy3_x - 1
    enemy3_y = enemy3_y - 1
    if enemy3_x < -100:
        enemy3_x = 100
    if enemy3_y < -100:
        enemy3_y = 100

def move_enemy(enemy_id):
    pass

def draw_all():
    draw_player()
    draw_enemies()
    draw_bank_info()

def draw_player():
    DRAW_VECTOR("player", 0, 0)

def draw_enemies():
    # Draw enemies at their current positions
    DRAW_VECTOR("enemy", enemy1_x, enemy1_y)
    DRAW_VECTOR("enemy", enemy2_x, enemy2_y)
    DRAW_VECTOR("enemy", enemy3_x, enemy3_y)

def draw_bank_info():
    # Show text indicating bank switching is working
    PRINT_TEXT(-120, 100, "BANK SWITCHING TEST")
    PRINT_TEXT(-120, 85, "MAIN: BANK 31 (FIXED)")
    PRINT_TEXT(-120, 70, "GAME: BANK 0 (AUTO)")

def loop():
    game_loop()
