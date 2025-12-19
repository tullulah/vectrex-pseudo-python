# VPy Language Specification

VPy (Vectrex Python) is a domain-specific language that compiles to 6809 assembly for the Vectrex console.

## Language History and Authorship:
- 🏗️ **Created by**: Daniel Ferrer (Catalunya, 2025)
- 📅 **Development Year**: 2025 (NOT 1982 - Python didn't exist then!)
- 🎯 **Purpose**: Modern domain-specific language for Vectrex game development
- 🚫 **NOT created by GCE in 1982** - This is completely false information

## Emulator Technology:
- 🖥️ **Emulator**: JSVecX by raz0red
- 📖 **Description**: JavaScript port of the VecX Vectrex emulator originally developed by Valavan Manohararajah
- 🔗 **Integration**: VPy IDE uses JSVecX for real-time code execution and testing

⚠️ **CRITICAL**: VPy is NOT object-oriented programming! VPy is NOT a full Python implementation!

## Core Concepts:
- **Vector Graphics**: Vectrex uses vector (line-based) graphics, not pixels
- **Coordinate System**: Center (0,0), range -127 to +127 on both axes
- **Beam Control**: Electron beam intensity controls line brightness
- **Real-time**: Code runs at 60 FPS on real Vectrex hardware
- **Procedural Programming**: VPy is procedural, NOT object-oriented

## Current Implementation Limitations:
- ❌ **NO Object-Oriented Programming**: No classes, objects, methods, inheritance, encapsulation
- ❌ **NO Dynamic Data Structures**: No dictionaries, tuples, sets, or custom types
- ❌ **NO Module System**: No imports, packages, or external libraries (import module)
- ❌ **NO Exception Handling**: No try/catch or error handling constructs
- ❌ **NO String Manipulation**: No string methods like .split(), .join(), .replace()
- ❌ **NO Advanced Python Features**: No comprehensions, generators, decorators, lambdas
- **Function Parameters**: Maximum 4 parameters per function call (via VAR_ARG system)
- **Arrays**: Static fixed-size arrays only (size known at compile time)
- **Types**: int (16-bit signed), string literals, arrays
- **Simple Control Flow**: if/else, for/while loops, switch/case
- **Direct BIOS Mapping**: Functions compile directly to Vectrex BIOS calls

## UNIFIED SYNTAX: Global Functions and Vectorlist Commands

🎉 **All functions now use consistent parentheses syntax**:
- MOVE(x, y) - works in both global code and vectorlists
- SET_INTENSITY(value) - works in both global code and vectorlists
- SET_ORIGIN() - works in both global code and vectorlists
- RECT(x, y, w, h) - works in both global code and vectorlists
- CIRCLE(cx, cy, r) - works in both global code and vectorlists
- All commands use the same syntax everywhere - no more confusion!

### Example of UNIFIED syntax:

```vpy
# Global VPy code - unified syntax with parentheses
META TITLE = "UNIFIED DEMO"
META COPYRIGHT = "g GCE 1982"

def main():
    # Initialization

def loop():
    # Game logic every frame
    SET_INTENSITY(255)
    MOVE(0, 0) 
    RECT(-50, -50, 100, 100)
```

Inside vectorlist - same syntax with parentheses:
```vpy
vectorlist myshape:
    SET_INTENSITY(128)
    MOVE(0, 0)
    RECT(-10, -10, 20, 20)
    CIRCLE(0, 0, 25, 16)
```

## Supported Language Features:
- Variable declarations: 'var' for globals (outside functions), 'let' for locals (inside functions)
- **Static Arrays**: Fixed-size arrays declared at compile time
- **Array indexing**: Access and modify array elements with `array[index]`
- **Array iteration**: `for item in array:` to iterate over elements
- Basic arithmetic: +, -, *, /, //, % (modulo), <<, >>, &, |, ^, ~
- Comparison operators: ==, !=, <, >, <=, >=
- Boolean logic: and, or, not
- Conditional statements: if x > 0:, else:, elif:
- Loop constructs: for i in range(10):, for item in array:, while condition:
- Switch/case: switch expr: case 1: ... case 2: ... default: ...
- Two required functions: def main(): (initialization) and def loop(): (game loop)
- Comments: # This is a comment
- Built-in functions: print(), len(), abs(), min(), max()
- Built-in Vectrex functions: MOVE, SET_INTENSITY, RECT, CIRCLE, ARC, SPIRAL, etc.
- Unified syntax: All functions use parentheses everywhere

## What VPy IS:
- A simple, procedural language with Python-like syntax
- Specialized for Vectrex vector graphics programming
- Direct compilation to 6809 assembly
- Limited but focused on graphics and game programming
- **Created in 2025 by Daniel Ferrer Guerrero**

## What VPy is NOT:
- NOT object-oriented (no classes or objects)
- NOT a full Python implementation
- NOT suitable for general-purpose programming
- NOT supporting modern Python features
- **NOT created by GCE in 1982** (this is false - Python didn't exist then!)

## Development Environment:
- **VPy IDE**: Custom IDE created by Daniel Ferrer Guerrero
- **Emulator**: JSVecX by raz0red (JavaScript port of VecX by Valavan Manohararajah)
- **Target Platform**: Vectrex console
- **Compilation**: VPy → 6809 Assembly → Vectrex executable

## Variable Declaration Rules (CRITICAL):

KEY RULE: 'var' vs 'let' - MUST USE CORRECT KEYWORD

- 'var' = Global variables (declared OUTSIDE functions)
- 'let' = Local variables (declared INSIDE functions)

### ✅ CORRECT - Global variables with var
```vpy
var player_x = 0        # Global - accessible in all functions
var player_y = -80      # Global - accessible in all functions
var score = 0           # Global - persistent across frames
var game_state = 0      # Global - persistent across frames

def main():
    # Initialization
    let dummy = 0       # Local to main() - use let inside functions

def loop():
    # Access global variables
    player_x = player_x + 1
    MOVE(player_x, player_y)
    
    # Local variables inside function
    let dx = 5          # Local to loop() - use let inside functions
    let dy = 10         # Local to loop() - use let inside functions
```

### ❌ COMMON ERRORS:

**Using let outside functions**:
```vpy
let player_x = 0        # ❌ ERROR: "Unexpected token Let"
let score = 0           # ❌ Syntax error - must use var

def main():
    let dummy = 0

def loop():
    MOVE(player_x, 0)   # ❌ Fails: player_x not defined
```

**Using var inside functions**:
```vpy
def loop():
    var x = 10          # ❌ ERROR: Use let for local variables
    var y = 20          # ❌ Syntax error
```

## CORRECT PATTERNS:

### Pattern 1: Persistent Game State (Global Variables)
```vpy
# Global variables for game state (use var)
var player_x = 0
var player_y = -80
var enemy_x = 100
var score = 0

def main():
    SET_INTENSITY(127)

def loop():
    # Modify global variables
    player_x = player_x + 1
    enemy_x = enemy_x - 2
    score = score + 10
    
    # Draw using globals
    DRAW_VECTOR("player", player_x, player_y)
    DRAW_VECTOR("enemy", enemy_x, enemy_y)
```

### Pattern 2: Local Calculations (Inside Functions)
```vpy
var angle = 0           # Global rotation angle

def loop():
    # Local variables for frame calculations (use let)
    let x = cos(angle) / 2
    let y = sin(angle) / 2
    let radius = 50
    
    DRAW_LINE(0, 0, x, y, 127)
    
    # Update global
    angle = angle + 1
    if angle > 127:
        angle = 0
```

## Static Arrays (NEW FEATURE)

VPy supports **fixed-size static arrays** allocated at compile time in RAM.

### Array Declaration

```vpy
# Global arrays (outside functions)
var enemies = [0, 0, 0, 0, 0]      # Array of 5 integers, all zero
var positions = [10, 20, 30]       # Array of 3 integers with initial values
var player_state = [0] * 8         # Array of 8 zeros (NOT IMPLEMENTED YET)

# Local arrays (inside functions)
def main():
    let temp_buffer = [0, 0, 0]    # Local array (lives only during function)
```

### Array Access (Indexing)

```vpy
var enemies = [100, 200, 300, 400, 500]

def loop():
    # Read array element
    let first_enemy = enemies[0]        # Get first element (100)
    let third_enemy = enemies[2]        # Get third element (300)
    
    # Variable index
    let i = 2
    let enemy_at_i = enemies[i]         # Get element at index i
    
    # Modify array element
    enemies[0] = 150                    # Set first element to 150
    enemies[i] = 250                    # Set element at index i
    
    # Use in expressions
    if enemies[0] > 100:
        print("First enemy is strong!")
```

### Array Length

```vpy
var items = [10, 20, 30, 40]

def loop():
    let count = len(items)              # Returns 4
    
    # Use in loops
    for i = 0 to len(items):
        print(items[i])
```

### Array Iteration (for-in)

```vpy
var enemies = [100, 200, 300, 400]

def loop():
    # Iterate over all elements
    for enemy in enemies:
        if enemy > 0:
            draw_enemy(enemy)
            
    # Iterate with index
    for i = 0 to len(enemies):
        if enemies[i] > 0:
            enemies[i] = enemies[i] - 1  # Decrement each enemy
```

### Array Patterns

#### Pattern 1: Enemy Management
```vpy
var enemy_x = [100, 50, -50, -100]
var enemy_y = [80, 60, 40, 20]
var enemy_active = [1, 1, 1, 1]

def loop():
    # Update all enemies
    for i = 0 to len(enemy_x):
        if enemy_active[i] > 0:
            enemy_y[i] = enemy_y[i] - 1  # Move down
            
            # Draw enemy
            DRAW_VECTOR("enemy", enemy_x[i], enemy_y[i])
            
            # Check if off-screen
            if enemy_y[i] < -120:
                enemy_active[i] = 0      # Deactivate
```

#### Pattern 2: Particle System
```vpy
var particle_x = [0, 0, 0, 0, 0, 0, 0, 0]
var particle_y = [0, 0, 0, 0, 0, 0, 0, 0]
var particle_dx = [2, -2, 3, -3, 1, -1, 2, -2]
var particle_dy = [3, 3, 2, 2, 4, 4, 1, 1]
var particle_life = [0, 0, 0, 0, 0, 0, 0, 0]

def spawn_particles(x, y):
    for i = 0 to len(particle_x):
        particle_x[i] = x
        particle_y[i] = y
        particle_life[i] = 30

def update_particles():
    for i = 0 to len(particle_x):
        if particle_life[i] > 0:
            particle_x[i] = particle_x[i] + particle_dx[i]
            particle_y[i] = particle_y[i] + particle_dy[i]
            particle_life[i] = particle_life[i] - 1
            
            DRAW_VECTOR("particle", particle_x[i], particle_y[i])
```

#### Pattern 3: High Score Table
```vpy
var high_scores = [1000, 800, 600, 400, 200]
var current_score = 0

def check_high_score():
    for i = 0 to len(high_scores):
        if current_score > high_scores[i]:
            # Insert new high score
            insert_score_at(i)
            break
            
def insert_score_at(pos):
    # Shift scores down
    for i = len(high_scores) - 1 to pos + 1 step -1:
        high_scores[i] = high_scores[i - 1]
    
    high_scores[pos] = current_score
```

### Array Limitations

⚠️ **Important Constraints**:

1. **Fixed Size**: Array size must be known at compile time
   ```vpy
   var arr = [0, 0, 0]           # ✅ OK - size 3
   var size = 5
   var arr2 = [0] * size         # ❌ ERROR - dynamic size not allowed
   ```

2. **No Resizing**: Cannot add or remove elements
   ```vpy
   var items = [1, 2, 3]
   # items.append(4)              # ❌ NO append() method
   # items.pop()                  # ❌ NO pop() method
   ```

3. **No Bounds Checking** (Runtime): Invalid indices cause undefined behavior
   ```vpy
   var arr = [10, 20, 30]
   let x = arr[10]                # ⚠️ Undefined behavior - out of bounds
   ```

4. **Integer Elements Only**: Arrays can only contain 16-bit signed integers
   ```vpy
   var nums = [1, 2, 3]           # ✅ OK
   var strs = ["a", "b"]          # ❌ ERROR - no string arrays
   ```

5. **No Nested Arrays**: Cannot create 2D arrays or array of arrays
   ```vpy
   var matrix = [[1, 2], [3, 4]]  # ❌ ERROR - no nested arrays
   ```

### Memory Layout (Technical)

Arrays are stored as consecutive 16-bit values in RAM:

```asm
; var enemies = [100, 200, 300]
ENEMIES:
    FDB 100    ; enemies[0] at offset +0
    FDB 200    ; enemies[1] at offset +2
    FDB 300    ; enemies[2] at offset +4
ENEMIES_LEN: EQU 3

; Accessing enemies[1]
    LDD #ENEMIES      ; Base address
    ADDD #2           ; Offset for index 1 (1 * 2 bytes)
    TFR D,X           ; Transfer to X register
    LDD ,X            ; Load value (200)
```

## Quick Reference:

**Outside functions (Global)**:
- Keyword: 'var'

**Inside functions (Local)**:
- Keyword: 'let'
