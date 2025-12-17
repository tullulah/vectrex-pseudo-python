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
- ❌ **NO Complex Data Structures**: No lists, dictionaries, tuples, sets, or custom types
- ❌ **NO Function Definitions**: Cannot define custom functions (def my_function():)
- ❌ **NO Module System**: No imports, packages, or external libraries (import module)
- ❌ **NO Exception Handling**: No try/catch or error handling constructs
- ❌ **NO String Manipulation**: No string methods like .split(), .join(), .replace()
- ❌ **NO Advanced Python Features**: No comprehensions, generators, decorators, lambdas
- **Function Parameters**: Maximum 2-3 parameters per function call (compiler limitation)
- **Primitive Variables Only**: Only int, string, and basic numeric types
- **Simple Control Flow**: Basic if/else, for/while loops only
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
- Basic arithmetic: +, -, *, /, % (modulo)
- Comparison operators: ==, !=, <, >, <=, >=
- Boolean logic: and, or, not
- Conditional statements: if x > 0:, else:
- Loop constructs: for i in range(10):, while condition:
- Two required functions: def main(): (initialization) and def loop(): (game loop)
- Comments: # This is a comment
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

## Quick Reference:

**Outside functions (Global)**:
- Keyword: 'var'

**Inside functions (Local)**:
- Keyword: 'let'
