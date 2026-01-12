# Test: Call graph analysis with multiple functions
# CORRECTED STRUCTURE: main() runs once, loop() runs every frame

META TITLE = "Call Graph Test"
META MUSIC = "0"
# META ROM_TOTAL_SIZE = 524288
# META ROM_BANK_SIZE = 16384

# Game state - shared across frames
enemy1_x = -50
enemy1_y = 60
enemy2_x = 0
enemy2_y = 0
enemy3_x = 50
enemy3_y = -60
frame_count = 0

# ==================== MAIN (runs once at startup) ====================
def main():
    SET_INTENSITY(127)
    # Initialization if needed - kept minimal for debugging

# ==================== LOOP (runs every frame) ====================
def loop():
    a = 12
    a = a + 15

    SET_INTENSITY(100)
    
    # Update game logic
    update_player()
    update_enemies()
    draw_all()
    a = a + 15
    frame_count = frame_count + 1

# ==================== GAME LOGIC ====================

def update_player():
    # Check input and move player
    check_input()
    move_player()

def check_input():
    # Placeholder - read joystick in real game
    pass

def move_player():
    # Placeholder - update player position
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

# ==================== RENDERING ====================

def draw_all():
    # Called every frame to render current state
    draw_player()
    draw_enemies()

def draw_player():
    # Draw player at center
    DRAW_VECTOR("player", 0, 0)

def draw_enemies():
    # Draw all enemies at their current positions
    DRAW_VECTOR("enemy", enemy1_x, enemy1_y)
    DRAW_VECTOR("enemy", enemy2_x, enemy2_y)
    DRAW_VECTOR("enemy", enemy3_x, enemy3_y)
