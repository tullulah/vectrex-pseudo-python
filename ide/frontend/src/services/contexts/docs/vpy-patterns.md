# VPy Programming Patterns and Common Mistakes

## Programming Patterns:

### 1. Two required functions:
- `def main():` for initialization and `def loop():` for game logic

### 2. main() runs once:
- Use for initializing variables, setting up game state

### 3. loop() runs every frame:
- Use for game logic, drawing, input handling (60 FPS)

### 4. ❌ NEVER add WAIT_RECAL() in loop():
- Backend automatically handles frame synchronization
- Adding it manually causes timing issues

### 5. Use safe intensity values:
- ALWAYS use intensity ≤127 (0x7F)
- Higher values cause invisible lines

### 6. DRAW_LINE coordinates are SCREEN POSITIONS:
- Specify start point (x1,y1) and end point (x2,y2) as absolute screen coordinates

### 7. Compiler converts to BIOS format:
- Backend calculates dx=x2-x1, dy=y2-y1 and generates: Moveto_d(x1,y1) + Draw_Line_d(dx,dy)

## Required Program Structure:

```vpy
META TITLE = "YOUR GAME"
META COPYRIGHT = "g GCE 1982"

def main():
    # Initialization code - runs ONCE at startup
    let dummy = 0  # Placeholder if no initialization needed

def loop():
    # Game loop - runs every frame (60 FPS)
    # ❌ NEVER add WAIT_RECAL() - backend handles it automatically
    
    # OPTION 1 (EASIEST): Draw connected square using DRAW_POLYGON
    DRAW_POLYGON(4, 80, -30, -30, 30, -30, 30, 30, -30, 30)
    
    # OPTION 2 (MANUAL): Draw connected square with MOVE + DRAW_TO
    # SET_INTENSITY(80)
    # MOVE(-30, -30)       # Move to first vertex (beam off)
    # DRAW_TO(30, -30)     # Draw to second vertex (connected)
    # DRAW_TO(30, 30)      # Draw to third vertex (connected)
    # DRAW_TO(-30, 30)     # Draw to fourth vertex (connected)
    # DRAW_TO(-30, -30)    # Close square (connected)
    
    # OPTION 3 (LOW-LEVEL): Connected square using MOVE + DRAW_LINE
    # SET_INTENSITY(80)
    # MOVE(-30, -30)                      # Position beam at start
    # DRAW_LINE(-30, -30, 30, -30, 80)    # Bottom edge
    # DRAW_LINE(30, -30, 30, 30, 80)      # Right edge (continues from previous end)
    # DRAW_LINE(30, 30, -30, 30, 80)      # Top edge (continues from previous end)
    # DRAW_LINE(-30, 30, -30, -30, 80)    # Left edge (continues from previous end)
    
    # EXAMPLE: House with DRAW_POLYGON (recommended for closed shapes)
    # Base rectangle
    DRAW_POLYGON(4, 80, -40, -40, 40, -40, 40, 20, -40, 20)
    
    # Roof triangle (peak ABOVE base at y=60)
    DRAW_POLYGON(3, 80, -50, 20, 0, 60, 50, 20)
    
    # Door
    DRAW_POLYGON(4, 80, -10, -40, 10, -40, 10, -15, -10, -15)
    
    # Window
    DRAW_POLYGON(4, 80, 15, -5, 30, -5, 30, -20, 15, -20)
    
    # NOTE: DRAW_POLYGON is MUCH easier than MOVE + DRAW_LINE
    # Each DRAW_POLYGON is independent - no need to reposition
```

## Common Mistakes:
- **Missing def main()**: Initialization function is required (runs once at startup)
- **Missing def loop()**: Game loop function is required (runs every frame at 60 FPS)
- **Putting game logic in main()**: main() is for initialization only, put game logic in loop()
- **Manual frame loops**: Don't use for/while loops for animation - loop() runs automatically
- **❌ CRITICAL: Using main() variables in loop()**: Variables declared in main() are NOT accessible in loop()
- **Declaring variables in main() for use in loop()**: Each function has separate scope - declare variables inside loop() instead
- **🚨 CRITICAL: Using intensity > 127**: Values 128-255 cause CRT oversaturation and INVISIBLE LINES - ALWAYS use ≤127
- **Using intensity values like 200, 255**: These are TOO HIGH and will NOT display correctly - use 64, 80, 127 instead
- **Forgetting WAIT_RECAL() at start of loop()**: Required for proper CRT synchronization
- **❌ Using multiple DRAW_LINE for shapes**: Creates disconnected lines with gaps - USE DRAW_POLYGON (easiest) OR MOVE once + multiple DRAW_TO (manual)
- **Drawing squares with 4 DRAW_LINE calls**: Each DRAW_LINE repositions beam creating gaps - options: 1) DRAW_POLYGON(4, intensity, x0,y0, x1,y1, x2,y2, x3,y3) OR 2) SET_INTENSITY + MOVE once + 4 DRAW_TO
- **❌ Putting asset name in META MUSIC**: META MUSIC requires a NUMBER ("0"-"9"), not an asset name like "space_battle". Use PLAY_MUSIC("name") in code instead
- **❌ Using DRAW_VECTOR/PLAY_MUSIC without creating asset files**: Functions work but need files in assets/vectors/*.vec and assets/music/*.vmus. If file missing, compiler shows "ERROR: asset 'name' not found"
- **❌ Calling PLAY_MUSIC but forgetting MUSIC_UPDATE()**: PLAY_MUSIC only initializes - you MUST call MUSIC_UPDATE() every frame in loop() for actual playback
- Coordinates outside -127 to +127 range
- Not considering automatic 60 FPS timing
- Trying to pass too many parameters to functions (check function documentation - varies from 0 to 5 params)
- Attempting to use unsupported Python features (classes, imports, etc.)
- Using undefined variables or complex expressions
- Thinking VPy is object-oriented (it's NOT!)
- Believing VPy was created in 1982 (completely false!)
- **Old structure**: Don't put all code in main() - separate initialization from game loop

## Common Trigonometric Patterns:

### Rotating Line:
```vpy
var angle = 0
def loop():
    WAIT_RECAL()
    let x = cos(angle) / 2
    let y = sin(angle) / 2
    DRAW_LINE(0, 0, x, y, 127)
    angle = angle + 1
    if angle > 127: angle = 0
```

### Rotating Triangle:
```vpy
var angle = 0
def loop():
    WAIT_RECAL()
    # 3 vertices at 120° intervals (42 units in 0-127 system)
    let x1 = cos(angle) / 2
    let y1 = sin(angle) / 2
    let x2 = cos(angle + 42) / 2
    let y2 = sin(angle + 42) / 2
    let x3 = cos(angle + 85) / 2
    let y3 = sin(angle + 85) / 2
    DRAW_LINE(x1, y1, x2, y2, 80)
    DRAW_LINE(x2, y2, x3, y3, 80)
    DRAW_LINE(x3, y3, x1, y1, 80)
    angle = angle + 1
    if angle > 127: angle = 0
```

### Circular Motion:
```vpy
var t = 0
def loop():
    WAIT_RECAL()
    let x = cos(t) / 3
    let y = sin(t) / 3
    # Draw a small square at current position
    DRAW_LINE(x-5, y-5, x+5, y-5, 80)
    DRAW_LINE(x+5, y-5, x+5, y+5, 80)
    DRAW_LINE(x+5, y+5, x-5, y+5, 80)
    DRAW_LINE(x-5, y+5, x-5, y-5, 80)
    t = t + 2  # Faster motion
    if t > 127: t = 0
```
