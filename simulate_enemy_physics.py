#!/usr/bin/env python3
"""
Simulador de física de enemigos de Pang
Reproduce la lógica exacta del código VPy para validar movimiento
"""

# Constants (same as VPy)
GRAVITY = 2
GROUND_Y = -80
MAX_ENEMIES = 8

# Level config for location 0 (Mount Fuji)
level_enemy_count = [1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 7]
level_enemy_speed = [1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5]

# Enemy state (arrays like in VPy)
enemy_active = [0] * MAX_ENEMIES
enemy_x = [0] * MAX_ENEMIES
enemy_y = [0] * MAX_ENEMIES
enemy_vx = [0] * MAX_ENEMIES
enemy_vy = [0] * MAX_ENEMIES
enemy_size = [0] * MAX_ENEMIES

def spawn_enemies(current_location=0):
    """Spawn enemies for current level - called ONCE when game starts"""
    count = level_enemy_count[current_location]
    speed = level_enemy_speed[current_location]
    
    print(f"=== SPAWN ENEMIES ===")
    print(f"Location: {current_location}, Count: {count}, Speed: {speed}")
    
    for i in range(count):
        enemy_active[i] = 1
        enemy_size[i] = 4        # Size 4 (huge bubble, radius 25)
        enemy_x[i] = -80 + (i * 50)  # Spread horizontally
        enemy_y[i] = 60          # Start high up
        enemy_vx[i] = speed      # Horizontal speed
        if i % 2 == 1:
            enemy_vx[i] = -speed # Alternate direction
        enemy_vy[i] = 0          # Start stationary (gravity will pull down)
        
        print(f"Enemy {i}: active={enemy_active[i]}, pos=({enemy_x[i]}, {enemy_y[i]}), vel=({enemy_vx[i]}, {enemy_vy[i]}), size={enemy_size[i]}")
    print()

def update_enemies():
    """Update all active enemies - called every frame during gameplay"""
    for i in range(MAX_ENEMIES):
        if enemy_active[i] == 1:
            # Apply gravity
            enemy_vy[i] = enemy_vy[i] - GRAVITY
            
            # Move
            enemy_x[i] = enemy_x[i] + enemy_vx[i]
            enemy_y[i] = enemy_y[i] + enemy_vy[i]
            
            # Collision radius (huge=25, large=20, medium=15, small=10)
            radius = 25
            if enemy_size[i] == 3:
                radius = 20
            elif enemy_size[i] == 2:
                radius = 15
            elif enemy_size[i] == 1:
                radius = 10
            
            # Wall bounce
            if enemy_x[i] <= -110 + radius:
                enemy_x[i] = -110 + radius
                enemy_vx[i] = -enemy_vx[i]
            if enemy_x[i] >= 110 - radius:
                enemy_x[i] = 110 - radius
                enemy_vx[i] = -enemy_vx[i]
            
            # Ground bounce
            if enemy_y[i] <= GROUND_Y + radius:
                enemy_y[i] = GROUND_Y + radius
                enemy_vy[i] = -enemy_vy[i]
                # Bounce damping: 80% energy retained
                enemy_vy[i] = (enemy_vy[i] * 4) // 5  # Integer division like VPy
            
            # Ceiling bounce
            if enemy_y[i] >= 110:
                enemy_y[i] = 110
                enemy_vy[i] = -enemy_vy[i]

def print_state(frame):
    """Print current state of all active enemies"""
    print(f"Frame {frame:3d}:", end="")
    for i in range(MAX_ENEMIES):
        if enemy_active[i] == 1:
            print(f"  E{i}[x={enemy_x[i]:4d}, y={enemy_y[i]:4d}, vx={enemy_vx[i]:3d}, vy={enemy_vy[i]:4d}]", end="")
    print()

def simulate(frames=50, location=0):
    """Simulate enemy physics for N frames"""
    print(f"╔{'═'*80}╗")
    print(f"║ PANG Enemy Physics Simulation - {frames} frames")
    print(f"╚{'═'*80}╝\n")
    
    # Spawn enemies
    spawn_enemies(location)
    
    # Initial state
    print("=== SIMULATION START ===")
    print_state(0)
    
    # Run simulation
    for frame in range(1, frames + 1):
        update_enemies()
        print_state(frame)
        
        # Highlight key events
        for i in range(MAX_ENEMIES):
            if enemy_active[i] == 1:
                # Detect bounce
                if enemy_y[i] == GROUND_Y + 25:  # At ground level
                    if enemy_vy[i] > 0:  # Moving up (just bounced)
                        print(f"         └─> Enemy {i} BOUNCED! (vy={enemy_vy[i]})")
                
                # Detect wall hit
                if enemy_x[i] == -110 + 25 or enemy_x[i] == 110 - 25:
                    print(f"         └─> Enemy {i} HIT WALL! (vx={enemy_vx[i]})")
    
    print("\n=== SIMULATION END ===")
    print(f"\nFinal state after {frames} frames:")
    for i in range(MAX_ENEMIES):
        if enemy_active[i] == 1:
            print(f"  Enemy {i}: position=({enemy_x[i]:4d}, {enemy_y[i]:4d}), velocity=({enemy_vx[i]:3d}, {enemy_vy[i]:4d})")

if __name__ == "__main__":
    import sys
    
    frames = 50
    location = 0
    
    if len(sys.argv) > 1:
        frames = int(sys.argv[1])
    if len(sys.argv) > 2:
        location = int(sys.argv[2])
    
    simulate(frames, location)
    
    print(f"\n💡 Usage: python {sys.argv[0]} [frames] [location]")
    print(f"   Example: python {sys.argv[0]} 100 3  (simulate 100 frames in location 3)")
