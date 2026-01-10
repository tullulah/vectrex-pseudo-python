# Test: Call graph analysis with multiple functions

META TITLE = "Call Graph Test"
META ROM_TOTAL_SIZE = 524288
META ROM_BANK_SIZE = 16384

def main():
    SET_INTENSITY(127)
    init_game()
    game_loop()

def init_game():
    load_assets()
    setup_player()

def load_assets():
    DEBUG_PRINT(1)

def setup_player():
    DEBUG_PRINT(2)

def game_loop():
    update_player()
    update_enemies()
    draw_all()

def update_player():
    check_input()
    move_player()

def check_input():
    DEBUG_PRINT(3)

def move_player():
    DEBUG_PRINT(4)

def update_enemies():
    i = 0
    while i < 10:
        move_enemy(i)
        i = i + 1

def move_enemy(enemy_id):
    DEBUG_PRINT(enemy_id)

def draw_all():
    draw_player()
    draw_enemies()

def draw_player():
    DRAW_VECTOR(0, 0, "player")

def draw_enemies():
    i = 0
    while i < 10:
        DRAW_VECTOR(0, 0, "enemy")
        i = i + 1

def loop():
    WAIT_RECAL()
    game_loop()
