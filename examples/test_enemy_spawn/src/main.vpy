META TITLE = "Enemy Spawn Test"

# Minimal test to verify enemy spawning
enemy_active = [0, 0, 0, 0, 0, 0, 0, 0]
enemy_x = [0, 0, 0, 0, 0, 0, 0, 0]
enemy_y = [0, 0, 0, 0, 0, 0, 0, 0]
enemy_vx = [0, 0, 0, 0, 0, 0, 0, 0]
enemy_vy = [0, 0, 0, 0, 0, 0, 0, 0]

spawned = 0

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Spawn once
    if spawned == 0:
        enemy_active[0] = 1
        enemy_x[0] = -80
        enemy_y[0] = 60
        enemy_vx[0] = 1
        enemy_vy[0] = 0
        spawned = 1
    
    # Update if active
    if enemy_active[0] == 1:
        # Apply gravity
        enemy_vy[0] = enemy_vy[0] - 2
        
        # Move
        enemy_x[0] = enemy_x[0] + enemy_vx[0]
        enemy_y[0] = enemy_y[0] + enemy_vy[0]
        
        # Bounce off ground at y=-55
        if enemy_y[0] <= -55:
            enemy_y[0] = -55
            enemy_vy[0] = -enemy_vy[0]
            enemy_vy[0] = (enemy_vy[0] * 4) / 5
    
    # Draw enemy position as text
    PRINT_TEXT(-120, 80, "ENEMY TEST")
    PRINT_TEXT(-120, 60, "Active:")
    # Note: Can't print int directly, just show if non-zero
    PRINT_TEXT(-120, 40, "X:")
    PRINT_TEXT(-120, 20, "Y:")
    PRINT_TEXT(-120, 0, "VX:")
    PRINT_TEXT(-120, -20, "VY:")
