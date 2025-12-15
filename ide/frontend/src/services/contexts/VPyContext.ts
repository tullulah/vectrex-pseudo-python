/**
 * VPy Language Context - Provides comprehensive context about VPy and Vectrex development
 */

export interface VPyFunction {
  name: string;
  syntax: string;
  description: string;
  parameters: Array<{
    name: string;
    type: string;
    description: string;
    required: boolean;
  }>;
  examples: string[];
  category: string;
  vectrexAddress?: string;
  notes?: string;
}

export interface VPyConstant {
  name: string;
  value: string | number;
  description: string;
  category: string;
}

export const VPY_FUNCTIONS: VPyFunction[] = [
  // Unified Functions (work in both global and vectorlist contexts)
  {
    name: "MOVE",
    syntax: "MOVE(x, y)",
    description: "Moves the electron beam to absolute coordinates without drawing",
    parameters: [
      { name: "x", type: "int", description: "X coordinate (-127 to +127)", required: true },
      { name: "y", type: "int", description: "Y coordinate (-127 to +127)", required: true }
    ],
    examples: [
      "MOVE(0, 0)  # Move to center",
      "MOVE(-100, 50)  # Move to upper left area"
    ],
    category: "unified",
    notes: "Works in both global code and vectorlist contexts with same syntax"
  },
  {
    name: "SET_INTENSITY",
    syntax: "SET_INTENSITY(level)",
    description: "Sets the electron beam intensity (brightness)",
    parameters: [
      { name: "level", type: "int", description: "Intensity level (0-127 recommended, max 255)", required: true }
    ],
    examples: [
      "SET_INTENSITY(127)  # Maximum safe brightness",
      "SET_INTENSITY(80)   # Medium brightness",
      "SET_INTENSITY(64)   # Low-medium brightness",
      "SET_INTENSITY(0)    # Invisible (off)"
    ],
    category: "unified",
    vectrexAddress: "0xF2AB",
    notes: "IMPORTANT: Use values ≤127 for safe display. Values 128-255 cause CRT oversaturation, burn-in risk, and invisible lines. Works in both global code and vectorlist contexts."
  },
  {
    name: "SET_ORIGIN",
    syntax: "SET_ORIGIN()",
    description: "Resets the coordinate system origin to center (0,0)",
    parameters: [],
    examples: [
      "SET_ORIGIN()  # Reset to center"
    ],
    category: "unified",
    vectrexAddress: "0xF354",
    notes: "Works in both global code and vectorlist contexts with same syntax"
  },
  {
    name: "RECT",
    syntax: "RECT(x, y, w, h)",
    description: "Draws a rectangle with given position and dimensions",
    parameters: [
      { name: "x", type: "int", description: "Left X coordinate", required: true },
      { name: "y", type: "int", description: "Bottom Y coordinate", required: true },
      { name: "w", type: "int", description: "Width (positive)", required: true },
      { name: "h", type: "int", description: "Height (positive)", required: true }
    ],
    examples: [
      "RECT(-50, -50, 100, 100)  # Square centered at origin",
      "RECT(0, 0, 100, 75)      # Rectangle from origin"
    ],
    category: "unified",
    notes: "Works in both global code and vectorlist contexts with same syntax"
  },
  {
    name: "CIRCLE",
    syntax: "CIRCLE(cx, cy, r) or CIRCLE(cx, cy, r, segs)",
    description: "Draws a circle or circle approximation with optional segment count",
    parameters: [
      { name: "cx", type: "int", description: "Center X coordinate", required: true },
      { name: "cy", type: "int", description: "Center Y coordinate", required: true },
      { name: "r", type: "int", description: "Radius", required: true },
      { name: "segs", type: "int", description: "Number of segments (3-64, default=16)", required: false }
    ],
    examples: [
      "CIRCLE(0, 0, 25)      # Circle at center, radius 25",
      "CIRCLE(50, 50, 30, 8) # Octagon approximation"
    ],
    category: "unified",
    notes: "Works in both global code and vectorlist contexts with same syntax"
  },
  {
    name: "DRAW_TO",
    syntax: "DRAW_TO(x, y)",
    description: "Draws a line from current position to absolute coordinates",
    parameters: [
      { name: "x", type: "int", description: "Target X coordinate (-127 to +127)", required: true },
      { name: "y", type: "int", description: "Target Y coordinate (-127 to +127)", required: true }
    ],
    examples: [
      "DRAW_TO(50, 0)   # Draw line to right",
      "DRAW_TO(0, -30)  # Draw line down",
      "DRAW_TO(25, 25)  # Draw line to diagonal"
    ],
    category: "drawing",
    vectrexAddress: "0xF3DF",
    notes: "Intensity must be > 0 to see the line. Draws from current beam position to target"
  },
  {
    name: "DRAW_POLYGON",
    syntax: "DRAW_POLYGON(N, x0, y0, x1, y1, ..., xN-1, yN-1) or DRAW_POLYGON(N, intensity, x0, y0, x1, y1, ...)",
    description: "⭐ RECOMMENDED FOR SHAPES: Draws a closed polygon with N vertices using connected lines (no gaps)",
    parameters: [
      { name: "N", type: "int", description: "Number of vertices (3+ for triangle, 4 for square, etc.)", required: true },
      { name: "intensity", type: "int", description: "Line intensity (0-127 recommended) - optional, defaults to 95", required: false },
      { name: "x0, y0, ...", type: "int", description: "Vertex coordinates in order (-127 to +127)", required: true }
    ],
    examples: [
      "# Square (4 vertices):",
      "DRAW_POLYGON(4, -30, -30, 30, -30, 30, 30, -30, 30)",
      "# Square with custom intensity:",
      "DRAW_POLYGON(4, 80, -30, -30, 30, -30, 30, 30, -30, 30)",
      "# Triangle (3 vertices):",
      "DRAW_POLYGON(3, 0, 20, -15, -10, 15, -10)",
      "# Pentagon (5 vertices):",
      "DRAW_POLYGON(5, 127, 0, 30, 28, 9, 17, -23, -17, -23, -28, 9)"
    ],
    category: "drawing",
    notes: "⭐ USE THIS for squares, triangles, and closed shapes - generates CONNECTED lines with single intensity/origin setup. Much more efficient than multiple DRAW_LINE calls. Automatically closes the polygon (last vertex connects to first)."
  },
  {
    name: "DRAW_LINE",
    syntax: "DRAW_LINE(x1, y1, x2, y2, intensity)",
    description: "🚨 DRAWS FROM CURRENT BEAM POSITION using RELATIVE deltas. Does NOT move beam to (x1,y1) automatically. Use MOVE() first to position beam. Takes absolute coords but converts to dx/dy deltas internally.",
    parameters: [
      { name: "x1", type: "int", description: "Start X position (-127 to +127, absolute screen coordinate - ONLY used to calculate dx)", required: true },
      { name: "y1", type: "int", description: "Start Y position (-127 to +127, absolute screen coordinate - ONLY used to calculate dy)", required: true },
      { name: "x2", type: "int", description: "End X position (-127 to +127, absolute screen coordinate)", required: true },
      { name: "y2", type: "int", description: "End Y position (-127 to +127, absolute screen coordinate)", required: true },
      { name: "intensity", type: "int", description: "Line intensity (0-127 ONLY, ≥128 = INVISIBLE)", required: true }
    ],
    examples: [
      "# ✅ CORRECT: Position beam first, then draw connected lines:",
      "SET_INTENSITY(80)",
      "MOVE(-30, -30)                   # Position at start",
      "DRAW_LINE(-30, -30, 30, -30, 80) # Bottom (draws dx=60, dy=0 from current pos)",
      "DRAW_LINE(30, -30, 30, 30, 80)   # Right (continues from end of previous line)",
      "DRAW_LINE(30, 30, -30, 30, 80)   # Top (connected)",
      "DRAW_LINE(-30, 30, -30, -30, 80) # Left (connected)",
      "",
      "# ⚠️ EASIER: Use DRAW_POLYGON for closed shapes (handles positioning automatically):",
      "DRAW_POLYGON(4, 80, -30,-30, 30,-30, 30,30, -30,30)  # Perfect square, no gaps"
    ],
    category: "drawing",
    notes: "⚠️ LOW-LEVEL API: Compiler calculates dx=x2-x1, dy=y2-y1 and calls BIOS Draw_Line_d(dy,dx) directly WITHOUT Moveto_d. Lines are CONNECTED if you position correctly with MOVE(). For simple shapes: USE DRAW_POLYGON (easiest) OR DRAW_TO (mid-level). DRAW_LINE is for advanced users who need explicit delta control."
  },
  {
    name: "DRAW_CIRCLE",
    syntax: "DRAW_CIRCLE(x, y, r, intensity)",
    description: "Draws a circle at specified position with given radius and intensity",
    parameters: [
      { name: "x", type: "int", description: "Center X coordinate (-127 to +127)", required: true },
      { name: "y", type: "int", description: "Center Y coordinate (-127 to +127)", required: true },
      { name: "r", type: "int", description: "Radius (1 to 110)", required: true },
      { name: "intensity", type: "int", description: "Line intensity (0-127 recommended)", required: true }
    ],
    examples: [
      "DRAW_CIRCLE(0, 0, 50, 255)    # Large circle at center",
      "DRAW_CIRCLE(-25, 25, 15, 128) # Small circle with medium intensity"
    ],
    category: "drawing",
    notes: "Draws complete circle with specified intensity"
  },
  {
    name: "DRAW_CIRCLE_SEG",
    syntax: "DRAW_CIRCLE_SEG(segments, x, y, r, intensity)",
    description: "Draws a circle approximation using specified number of line segments",
    parameters: [
      { name: "segments", type: "int", description: "Number of line segments (more = smoother)", required: true },
      { name: "x", type: "int", description: "Center X coordinate (-127 to +127)", required: true },
      { name: "y", type: "int", description: "Center Y coordinate (-127 to +127)", required: true },
      { name: "r", type: "int", description: "Radius (positive)", required: true },
      { name: "intensity", type: "int", description: "Line intensity (0-255)", required: true }
    ],
    examples: [
      "DRAW_CIRCLE_SEG(8, 0, 0, 30, 255)  # Octagon approximation",
      "DRAW_CIRCLE_SEG(16, 0, 0, 30, 200) # Smoother circle"
    ],
    category: "drawing",
    notes: "More segments = smoother but slower. Typical values: 8-32 segments"
  },
  {
    name: "sin",
    syntax: "sin(angle)",
    description: "Sine function using precalculated lookup table (fast)",
    parameters: [
      { name: "angle", type: "int", description: "Angle in range 0-127 (0-127 represents 0-2π radians)", required: true }
    ],
    examples: [
      "y = sin(angle)       # Returns -127 to +127",
      "y = sin(angle) / 2   # Scale down for smaller radius",
      "# Rotating line example:",
      "let x_end = cos(angle) / 2",
      "let y_end = sin(angle) / 2",
      "DRAW_LINE(0, 0, x_end, y_end, 127)",
      "# Rotating triangle:",
      "let x1 = cos(angle) / 2",
      "let x2 = cos(angle + 42) / 2  # +120° offset",
      "let x3 = cos(angle + 85) / 2  # +240° offset"
    ],
    category: "math",
    notes: "Uses precalculated table. Input: 0-127 (0=0°, 32=90°, 64=180°, 96=270°). Output: -127 to +127 (fits in signed byte). For DRAW_LINE coordinates, use values ≤±63 or divide by 2."
  },
  {
    name: "cos",
    syntax: "cos(angle)",
    description: "Cosine function using precalculated lookup table (fast)",
    parameters: [
      { name: "angle", type: "int", description: "Angle in range 0-127 (0-127 represents 0-2π radians)", required: true }
    ],
    examples: [
      "x = cos(angle)       # Returns -127 to +127",
      "x = cos(angle) / 2   # Scale down for smaller radius",
      "# Circular motion:",
      "MOVE(cos(t) / 2, sin(t) / 2)"
    ],
    category: "math",
    notes: "Uses precalculated table. Input: 0-127 (0=0°, 32=90°, 64=180°, 96=270°). Output: -127 to +127 (fits in signed byte). For DRAW_LINE coordinates, use values ≤±63 or divide by 2."
  },
  {
    name: "tan",
    syntax: "tan(angle)",
    description: "Tangent function using precalculated lookup table (fast)",
    parameters: [
      { name: "angle", type: "int", description: "Angle in range 0-127 (0-127 represents 0-2π radians)", required: true }
    ],
    examples: [
      "slope = tan(angle)  # Returns -120 to +120",
      "# Values near ±90° are clamped to ±120"
    ],
    category: "math",
    notes: "Uses precalculated table. Input: 0-127 maps to 0-360°. Output clamped to ±120 to avoid overflow near 90° and 270°."
  },
  {
    name: "SET_INTENSITY",
    syntax: "SET_INTENSITY(value)",
    description: "Sets the electron beam intensity (brightness) in regular VPy code",
    parameters: [
      { name: "value", type: "int", description: "Intensity level (0-255)", required: true }
    ],
    examples: [
      "SET_INTENSITY(255)  # Maximum brightness",
      "SET_INTENSITY(128)  # Half brightness", 
      "SET_INTENSITY(0)    # Beam off (invisible)"
    ],
    category: "intensity",
    vectrexAddress: "0xF2AB",
    notes: "0 = beam off, 255 = maximum brightness. Works in both global code and vectorlist contexts with same syntax."
  },
  {
    name: "PRINT_TEXT",
    syntax: "PRINT_TEXT(x, y, text)",
    description: "Displays text using Vectrex built-in character set",
    parameters: [
      { name: "x", type: "int", description: "X position (-127 to +127)", required: true },
      { name: "y", type: "int", description: "Y position (-127 to +127)", required: true },
      { name: "text", type: "string", description: "Text to display", required: true }
    ],
    examples: [
      'PRINT_TEXT(0, 100, "HELLO WORLD")',
      'PRINT_TEXT(-60, 0, "SCORE: 1000")',
      'PRINT_TEXT(0, -100, "GAME OVER")'
    ],
    category: "text",
    vectrexAddress: "0xF37A",
    notes: "Uses Vectrex ROM character set. Text is drawn at specified coordinates"
  },
  {
    name: "SET_ORIGIN",
    syntax: "SET_ORIGIN()",
    description: "Resets the coordinate reference point to screen center (0,0)",
    parameters: [],
    examples: [
      "SET_ORIGIN()  # Reset to center"
    ],
    category: "control",
    vectrexAddress: "0xF36B",
    notes: "Useful after drawing complex shapes to reset coordinate system"
  },
  {
    name: "WAIT_RECAL",
    syntax: "WAIT_RECAL()",
    description: "Waits for vertical retrace (frame synchronization)",
    parameters: [],
    examples: [
      "# ❌ NEVER add WAIT_RECAL() in your code",
      "# Backend automatically handles frame synchronization",
      "def loop():",
      "    # Just write your drawing code",
      "    SET_INTENSITY(127)",
      "    DRAW_VECTOR(\"player\")"
    ],
    category: "timing",
    notes: "❌ NEVER add WAIT_RECAL() manually - backend automatically handles frame synchronization. Adding it manually causes timing issues. Synchronizes with 60 FPS CRT refresh rate automatically."
  },
  {
    name: "MUSIC_UPDATE",
    syntax: "MUSIC_UPDATE()",
    description: "✅ Processes PSG music events (REQUIRED for music playback)",
    parameters: [],
    examples: [
      'def loop():',
      '    WAIT_RECAL()',
      '    MUSIC_UPDATE()  # Process music every frame',
      '    # Your game code...'
    ],
    category: "audio",
    notes: "✅ REQUIRED FOR MUSIC - Must be called once per frame in loop() after PLAY_MUSIC(). Increments tick counter, processes NOTE/NOISE events, programs PSG registers (R0-R10). Handles timing, loops, channel volumes. No-op if no music playing."
  },
  {
    name: "PLAY_MUSIC1",
    syntax: "PLAY_MUSIC1()",
    description: "Plays the built-in music track 1",
    parameters: [],
    examples: [
      "PLAY_MUSIC1()  # Start background music"
    ],
    category: "sound",
    notes: "Activates Vectrex built-in music. Only music1 is currently supported"
  },
  
  // Asset System Functions
  {
    name: "DRAW_VECTOR",
    syntax: "DRAW_VECTOR(name, x, y)",
    description: "✅ WORKING - Draws a vector asset at absolute position (x, y)",
    parameters: [
      { name: "name", type: "string", description: "Name of the vector asset (without .vec extension)", required: true },
      { name: "x", type: "number", description: "X coordinate (-127 to 127, center=0)", required: true },
      { name: "y", type: "number", description: "Y coordinate (-127 to 127, center=0)", required: true }
    ],
    examples: [
      'var player_x = 0',
      'var player_y = -80',
      '',
      'def loop():',
      '    SET_INTENSITY(127)',
      '    DRAW_VECTOR("player", player_x, player_y)  # Draw at position',
      '',
      '# 🚨 CRITICAL: Vector asset intensity values must be ≤127',
      '# Values >127 (like 150, 200, 255) cause INVISIBLE LINES',
      '# Example .vec file (CORRECT intensities):',
      '# {"paths":[{"intensity":127, "points":[...]}]}  # ✅ VISIBLE',
      '# {"paths":[{"intensity":200, "points":[...]}]}  # ❌ INVISIBLE!'
    ],
    category: "assets",
    notes: "✅ FULLY IMPLEMENTED. Draws vector at specified (x,y) position. Compiler auto-discovers .vec files in assets/vectors/ folder (Phase 0). Asset is embedded in ROM as _NAME_PATH0, _NAME_PATH1, etc. Uses Draw_Sync_List_At which adds x,y offset to vector coordinates. 🚨 CRITICAL: intensity values in .vec file MUST be ≤127 - higher values cause invisible lines!"
  },
  {
    name: "PLAY_MUSIC",
    syntax: "PLAY_MUSIC(name)",
    description: "✅ WORKING - Plays PSG music from embedded .vmus file (requires calling MUSIC_UPDATE in loop)",
    parameters: [
      { name: "name", type: "string", description: "Name of the music asset (without .vmus extension)", required: true }
    ],
    examples: [
      'def main():',
      '    PLAY_MUSIC("theme")  # Initialize music from assets/music/theme.vmus',
      '',
      'def loop():',
      '    WAIT_RECAL()',
      '    MUSIC_UPDATE()       # Process music events (REQUIRED for playback)',
      '    # Your game code...'
    ],
    category: "assets",
    notes: "✅ FULLY IMPLEMENTED - Auto-discovers .vmus files in assets/music/. Embeds event data in ROM. PLAY_MUSIC_RUNTIME initializes PSG, resets state. MUSIC_UPDATE() processes events (NOTE/NOISE), sets PSG registers (R0-R10), handles timing. Call MUSIC_UPDATE() once per frame in loop() for playback. PSG hardware access via VIA Port B."
  }
];

export const VPY_CONSTANTS: VPyConstant[] = [
  { name: "SCREEN_WIDTH", value: 254, description: "Total screen width in Vectrex units", category: "display" },
  { name: "SCREEN_HEIGHT", value: 254, description: "Total screen height in Vectrex units", category: "display" },
  { name: "CENTER_X", value: 0, description: "Screen center X coordinate", category: "display" },
  { name: "CENTER_Y", value: 0, description: "Screen center Y coordinate", category: "display" },
  { name: "MAX_INTENSITY", value: 255, description: "Maximum beam intensity", category: "intensity" },
  { name: "MIN_INTENSITY", value: 0, description: "Minimum beam intensity (off)", category: "intensity" },
  { name: "FPS", value: 60, description: "Vectrex refresh rate in frames per second", category: "timing" }
];

export const VPY_LANGUAGE_CONTEXT = `
# VPy Language Specification

VPy (Vectrex Python) is a domain-specific language that compiles to 6809 assembly for the Vectrex console.

## Language History and Authorship:
🏗️ **Created by**: Daniel Ferrer (Catalunya, 2025)
📅 **Development Year**: 2025 (NOT 1982 - Python didn't exist then!)
🎯 **Purpose**: Modern domain-specific language for Vectrex game development
🚫 **NOT created by GCE in 1982** - This is completely false information

## Emulator Technology:
🖥️ **Emulator**: JSVecX by raz0red
📖 **Description**: JavaScript port of the VecX Vectrex emulator originally developed by Valavan Manohararajah
🔗 **Integration**: VPy IDE uses JSVecX for real-time code execution and testing

⚠️  **CRITICAL**: VPy is NOT object-oriented programming! VPy is NOT a full Python implementation!

## Core Concepts:
- **Vector Graphics**: Vectrex uses vector (line-based) graphics, not pixels
- **Coordinate System**: Center (0,0), range -127 to +127 on both axes
- **Beam Control**: Electron beam intensity controls line brightness
- **Real-time**: Code runs at 60 FPS on real Vectrex hardware
- **Procedural Programming**: VPy is procedural, NOT object-oriented

## Current Implementation Limitations:
❌ **NO Object-Oriented Programming**: No classes, objects, methods, inheritance, encapsulation
❌ **NO Complex Data Structures**: No lists, dictionaries, tuples, sets, or custom types
❌ **NO Function Definitions**: Cannot define custom functions (def my_function():)
❌ **NO Module System**: No imports, packages, or external libraries (import module)
❌ **NO Exception Handling**: No try/catch or error handling constructs
❌ **NO String Manipulation**: No string methods like .split(), .join(), .replace()
❌ **NO Advanced Python Features**: No comprehensions, generators, decorators, lambdas
- **Function Parameters**: Maximum 2-3 parameters per function call (compiler limitation)
- **Primitive Variables Only**: Only int, string, and basic numeric types
- **Simple Control Flow**: Basic if/else, for/while loops only
- **Direct BIOS Mapping**: Functions compile directly to Vectrex BIOS calls

## UNIFIED SYNTAX: Global Functions and Vectorlist Commands
🎉 **All functions now use consistent parentheses syntax**:
• MOVE(x, y) - works in both global code and vectorlists
• SET_INTENSITY(value) - works in both global code and vectorlists  
• SET_ORIGIN() - works in both global code and vectorlists
• RECT(x, y, w, h) - works in both global code and vectorlists
• CIRCLE(cx, cy, r) - works in both global code and vectorlists
• All commands use the same syntax everywhere - no more confusion!

Example of UNIFIED syntax:

\`\`\`vpy
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
\`\`\`

    # Inside vectorlist - same syntax with parentheses  
    vectorlist myshape:
        SET_INTENSITY(128)
        MOVE(0, 0)
        RECT(-10, -10, 20, 20)
        CIRCLE(0, 0, 25, 16)

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

'var' = Global variables (declared OUTSIDE functions)
'let' = Local variables (declared INSIDE functions)

\`\`\`vpy
# ✅ CORRECT - Global variables with var
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
\`\`\`

COMMON ERRORS:

\`\`\`vpy
# ❌ INCORRECT - Using let outside functions
let player_x = 0        # ❌ ERROR: "Unexpected token Let"
let score = 0           # ❌ Syntax error - must use var

def main():
    let dummy = 0

def loop():
    MOVE(player_x, 0)   # ❌ Fails: player_x not defined
\`\`\`

\`\`\`vpy
# ❌ INCORRECT - Using var inside functions
def loop():
    var x = 10          # ❌ ERROR: Use let for local variables
    var y = 20          # ❌ Syntax error
\`\`\`

CORRECT PATTERNS:

**Pattern 1: Persistent Game State (Global Variables)**
\`\`\`vpy
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
\`\`\`

**Pattern 2: Local Calculations (Inside Functions)**
\`\`\`vpy
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
\`\`\`

Quick Reference:

Outside functions (Global):
- Keyword: 'var'
- Scope: Global
- Persistence: Permanent across frames
- Example: var score = 0

Inside functions (Local):
- Keyword: 'let'
- Scope: Local to function
- Persistence: Frame only (discarded after return)
- Example: let dx = 5

Why This Design?
- Global 'var': Stored in RAM, persists across frames (game state)
- Local 'let': Stored on stack, discarded after function returns (temporary calculations)
- Separates persistent state from temporary calculations
- Matches 6809 assembly memory model (RAM vs stack)

## VPy Game Structure (New Model):

### 🎮 **Two-Function Architecture**
VPy uses a clean separation between initialization and game logic:

#### **def main():** - Initialization (runs once)
- Called **once** when the program starts
- Use for: setting up variables, initial game state, one-time setup
- **Do NOT** put game loops or drawing code here
- **Purpose**: Initialize your game world

#### **def loop():** - Game Loop (runs every frame)  
- Called **automatically** 60 times per second (60 FPS)
- Use for: game logic, drawing, input handling, animations
- **No manual loops needed** - the function IS the loop
- **Purpose**: Update and draw your game each frame

### 🎯 **Why This Structure?**
- **Clear separation**: Initialization vs. game logic
- **Automatic timing**: No need for manual frame timing
- **Professional pattern**: Follows game engine conventions (Unity, Arduino, etc.)
- **Performance**: Optimized by the compiler for Vectrex hardware
- **Error prevention**: Compiler enforces both functions exist

### ❌ **Old vs. New Structure**

**Old (deprecated):**
\\\`\\\`\\\`vpy
def main():
    # Everything mixed together
    player_x = 0  # Init
    for frame in range(1000):  # Manual loop
        draw_player()  # Game logic
        WAIT_FRAMES(1)  # Manual timing
\\\`\\\`\\\`

**New (correct):**
\\\`\\\`\\\`vpy
def main():
    # Clean initialization
    let player_x = 0
    let score = 0

def loop():
    # Clean game logic (automatic 60 FPS)
    draw_player()
    update_score()
    # No manual timing needed!
\\\`\\\`\\\`

### 📄 **Project Metadata (META Fields)**
VPy supports exactly 3 META fields that define ROM header information:

\`\`\`vpy
META TITLE = "MY GAME"          # Game title (REQUIRED)
META COPYRIGHT = "g GCE 1982"   # Copyright string (optional)
META MUSIC = "music1"           # BIOS music symbol (optional)

# Your VPy code starts here
def main():
    # Initialization

def loop():
    # Game logic
    SET_INTENSITY(255)
    MOVE(0, 0)
    PRINT_TEXT(0, 50, "HELLO VECTREX")
\`\`\`

### 🏷️ **META Field Reference (3 fields only):**

- **TITLE**: Game title (required)
  - Example: \`META TITLE = "SPACE SHOOTER"\`
  - **CRITICAL**: Must be in UPPERCASE letters only
  - **Max length**: 24 characters
  - **Valid characters**: Letters, numbers, spaces only (special chars cleaned)
  - **Used for**: ROM header, game identification

- **COPYRIGHT**: Copyright string (optional)
  - Example: \`META COPYRIGHT = "g GCE 1982"\`
  - **Default**: "g GCE 1982"
  - **Used for**: First line display in ROM header

- **MUSIC**: Built-in BIOS music NUMBER for title screen (optional)
  - Examples: \`META MUSIC = "0"\` (no music), \`META MUSIC = "1"\` (Minestorm song 1), \`META MUSIC = "2"\` etc.
  - **Default**: "music1" (or use "0" for silence)
  - **⚠️ IMPORTANT**: This is NOT for your custom .vmus files - use \`PLAY_MUSIC("name")\` function in code for that
  - **Range**: "0" to "9" (numbers only, built-in songs)
  - **Used for**: Title screen background music (built-in songs only)

### ⚠️ **Important META Rules:**
- **Only 3 META fields supported**: TITLE, COPYRIGHT, MUSIC
- **TITLE must be UPPERCASE**: Lowercase reserved for special characters
- **TITLE is required** for proper ROM generation
- **Other fields are optional** with reasonable defaults
- **ROM dimensions fixed**: Height/width/coords ($F8,$50,$20,$AA) cannot be changed

### 📂 **Correct Project Examples:**

#### **Simple Game:**
\`\`\`vpy
META TITLE = "SQUARE DEMO"
META COPYRIGHT = "g GCE 1982"
META MUSIC = "0"

def main():
    # Initialize once

def loop():
    # Draw every frame
    SET_INTENSITY(255)
    MOVE(-25, -25)
    DRAW_TO(25, -25)
    DRAW_TO(25, 25)
    DRAW_TO(-25, 25)
    DRAW_TO(-25, -25)
\`\`\`

#### **Animation with Music:**
\`\`\`vpy
META TITLE = "ROTATING LINE"
META COPYRIGHT = "g GCE 1982"
META MUSIC = "music1"

def main():
    # Initialize animation variables once
    let x = -30
    let direction = 1

def loop():
    # Animation runs automatically every frame
    SET_INTENSITY(200)
    MOVE(x, 0)
    DRAW_TO(x + 30, 0)
    
    # Update position
    x = x + direction
    if x > 30:
        direction = -1
    if x < -30:
        direction = 1
\`\`\`

#### **Minimal Example:**
\`\`\`vpy
META TITLE = "HELLO WORLD"

def main():
    # Minimal initialization

def loop():
    # Minimal code - other META fields use defaults
    SET_INTENSITY(255)
    PRINT_TEXT(0, 0, "HELLO")
\`\`\`

### 🔧 **META Fields Usage in IDE:**
- **ROM Header Generation**: META fields directly affect Vectrex ROM header
- **Title Display**: TITLE appears in game selection and ROM info
- **Copyright Notice**: COPYRIGHT shown in ROM header first line
- **Music Integration**: MUSIC controls background audio from BIOS

## VPy IDE Features and Functionality:

### 🎮 **Integrated Vectrex Emulator**
- **Real-time Execution**: Code runs immediately in the built-in JSVecX emulator
- **Vector Display**: See your graphics as they would appear on real Vectrex hardware
- **60 FPS Rendering**: Smooth animation and real-time feedback
- **Sound Support**: AY-3-8912 PSG sound chip emulation

### 📝 **Code Editor**
- **Syntax Highlighting**: VPy syntax highlighting with Python-like coloring
- **Auto-completion**: IntelliSense for VPy functions and syntax
- **Error Detection**: Real-time syntax error highlighting
- **Code Formatting**: Automatic indentation and formatting
- **Multiple Files**: Support for multiple VPy files in projects

### 🔧 **Development Tools**
- **Compile & Run**: One-click compilation and execution
- **Debug Output**: Console showing compilation and runtime information
- **File Management**: Project explorer with file creation, deletion, renaming
- **Settings Panel**: IDE configuration and preferences
- **Responsive Layout**: Resizable panels and dockable interface

### 🤖 **PyPilot AI Assistant**
- **Context-Aware Help**: AI assistant with deep VPy knowledge
- **Code Generation**: Generate VPy code from natural language descriptions
- **Error Fixing**: Analyze and suggest fixes for compilation errors
- **Code Explanation**: Explain existing VPy code functionality
- **Optimization**: Suggest improvements for performance and clarity
- **Multiple Providers**: Support for OpenAI, Anthropic, Groq, GitHub Models, etc.

### 📁 **Project Management**
- **File Explorer**: Navigate and manage VPy source files
- **New Project**: Create new VPy projects with templates
- **Import/Export**: Save and load VPy projects
- **Example Projects**: Built-in examples and tutorials

### 🎨 **User Interface**
- **VS Code-style Layout**: Familiar interface for developers
- **Dark/Light Themes**: Multiple color themes
- **Dock Layout**: Draggable and resizable panels
- **Full Screen Mode**: Distraction-free coding environment
- **Responsive Design**: Works on different screen sizes

### 🏃 **Execution Environment**
- **Instant Feedback**: See results immediately without external tools
- **Vector Graphics**: Real-time vector rendering
- **Performance Metrics**: Monitor frame rate and execution speed
- **Memory Viewer**: Inspect Vectrex memory state
- **BIOS Integration**: Full Vectrex BIOS function support

### 🆘 **User Assistance**
- **Built-in Help**: Comprehensive help system
- **Function Reference**: Complete VPy function documentation
- **Tutorials**: Step-by-step learning materials
- **Error Messages**: Clear, helpful error descriptions
- **Code Examples**: Ready-to-run example programs

### ⚙️ **IDE Commands and Shortcuts**
- **Ctrl+S**: Save current file
- **F5**: Compile and run
- **Ctrl+N**: New file
- **Ctrl+O**: Open file
- **Ctrl+Shift+P**: Command palette
- **Ctrl+/**: Toggle comment
- **View Menu**: Access all panels (Editor, Emulator, PyPilot, File Explorer)

## PyPilot AI Assistant Commands:
- **/help** - Show available PyPilot commands
- **/explain** - Explain selected VPy code or current file
- **/fix** - Analyze and suggest fixes for errors
- **/generate** - Generate VPy code from description
- **/optimize** - Suggest code optimizations
- **/vectrex** - Get Vectrex hardware information
- **Auto-context**: Automatically includes current file content in requests

## Code Examples:
\`\`\`vpy
# Simple drawing example (CORRECT VPy code)
META TITLE = "SQUARE DEMO"
META COPYRIGHT = "g GCE 1982"

def main():
    # Initialization - runs ONCE at startup

def loop():
    # Game loop - runs every frame (60 FPS)
    SET_INTENSITY(255)     # Set bright intensity
    MOVE(-50, 0)           # Move to starting position
    DRAW_TO(50, 0)         # Draw horizontal line
    DRAW_TO(50, 50)        # Draw vertical line up
    DRAW_TO(-50, 50)       # Draw back to start
    DRAW_TO(-50, 0)        # Complete the rectangle

# Animation example with variables (CORRECT VPy code)
META TITLE = "MOVING SQUARE"  
META COPYRIGHT = "g GCE 1982"

def main():
    # Initialize variables once
    let player_x = -50
    let direction = 1

def loop():
    # Animation logic every frame
    SET_INTENSITY(255)
    MOVE(player_x, 0)
    DRAW_TO(player_x + 20, 0)
    DRAW_TO(player_x + 20, 20)
    DRAW_TO(player_x, 20)
    DRAW_TO(player_x, 0)
    
    # Update position
    player_x = player_x + direction
    if player_x > 50:
        direction = -1
    if player_x < -50:
        direction = 1

# INCORRECT - This is NOT valid VPy (NO classes):
# class Shape:           # ❌ NOT SUPPORTED
#     def __init__(self): # ❌ NOT SUPPORTED  
#         pass           # ❌ NOT SUPPORTED

# INCORRECT - This is NOT valid VPy (NO lists):
# points = [10, 20, 30]  # ❌ NOT SUPPORTED
# for point in points:   # ❌ NOT SUPPORTED

# INCORRECT - This is NOT valid VPy (NO custom functions):
# def draw_square():     # ❌ NOT SUPPORTED
#     pass              # ❌ NOT SUPPORTED
\`\`\`

## 🎨 Asset System - Embedded Graphics and Music

VPy includes a powerful asset system that allows you to embed vector graphics and music directly in your game ROM.

### 📦 How Assets Work:
1. **Auto-discovery**: Place .vec and .vmus files in \`assets/vectors/\` and \`assets/music/\` directories
2. **Compile-time embedding**: Assets are automatically discovered and embedded in ROM during compilation (Phase 0)
3. **Reference by name**: Use \`DRAW_VECTOR("name", x, y)\` and \`PLAY_MUSIC("name")\` in your code
4. **No manual loading**: Everything is compiled into the final binary

### 🎯 Vector Graphics (.vec files)

Vector graphics are stored as JSON files in \`assets/vectors/*.vec\`:

**File format**:
\`\`\`json
{
  "version": "1.0",
  "name": "player",
  "canvas": {"width": 256, "height": 256, "origin": "center"},
  "layers": [{
    "name": "default",
    "visible": true,
    "paths": [{
      "name": "ship",
      "intensity": 127,
      "closed": true,
      "points": [
        {"x": 0, "y": 20},
        {"x": -15, "y": -10},
        {"x": 15, "y": -10}
      ]
    }]
  }]
}
\`\`\`

**Key fields**:
- \`name\`: Asset identifier (used in \`DRAW_VECTOR("player")\`)
- \`intensity\`: Brightness (0-255, higher = brighter)
- \`closed\`: true = polygon, false = open line
- \`points\`: Coordinates in range -127 to +127 (Vectrex screen space)

**Usage in code**:
\`\`\`vpy
def loop():
    WAIT_RECAL()
    SET_INTENSITY(255)
    MOVE(-50, 0)
    DRAW_VECTOR("player", 0, -80)  # Draws the vector asset at position
\`\`\`

### 🎵 Music Assets (.vmus files)

Music is stored as JSON files in \`assets/music/*.vmus\`:

**File format**:
\`\`\`json
{
  "version": "1.0",
  "name": "theme",
  "author": "Composer Name",
  "tempo": 120,
  "ticksPerBeat": 24,
  "totalTicks": 384,
  "notes": [
    {"id": "note1", "note": 60, "start": 0, "duration": 48, "velocity": 12, "channel": 0},
    {"id": "note2", "note": 64, "start": 48, "duration": 48, "velocity": 12, "channel": 0},
    {"id": "note3", "note": 67, "start": 96, "duration": 48, "velocity": 12, "channel": 0}
  ],
  "noise": [
    {"id": "noise1", "start": 0, "duration": 24, "period": 15, "channels": 1, "velocity": 12}
  ],
  "loopStart": 0,
  "loopEnd": 384
}
\`\`\`

**Key fields**:
- \`note\`: MIDI note number (60=C4, 69=A4 440Hz, 72=C5)
- \`velocity\`: Volume (0-15, where 15=maximum) - Used for both notes and noise
- \`channel\`: PSG channel (0=A, 1=B, 2=C) - Only for notes
- \`period\`: Noise period (0-31, lower=higher pitch)
- \`channels\`: Noise channel mask (1=A, 2=B, 4=C, 7=all) - Only for noise

**Usage in code**:
\`\`\`vpy
def main():
    PLAY_MUSIC("theme")  # Start background music

def loop():
    # Music plays automatically in background
    # ... game logic ...
\`\`\`

### 📁 Project Structure with Assets:
\`\`\`
my_game/
├── game.vpy              # Main game code
├── assets/
│   ├── vectors/
│   │   ├── player.vec    # Player ship sprite
│   │   ├── enemy.vec     # Enemy sprite
│   │   └── bullet.vec    # Bullet graphic
│   └── music/
│       ├── theme.vmus    # Main theme music
│       ├── gameover.vmus # Game over jingle
│       └── victory.vmus  # Victory music
\`\`\`

### 🔧 Asset System Technical Details:
- **Discovery**: Automatic at compile time (Phase 0)
- **Embedding**: Data section in ROM (Phase 5)
- **Format**: FCB assembly directives for vector data
- **Access**: JSR to BIOS Draw_VLc for vectors
- **Music**: Placeholder PSG player (full implementation in progress)
- **Compilation**: Native M6809 assembler (no lwasm needed)

### ✅ Asset Best Practices:
1. Keep vector sprites simple (fewer points = faster drawing)
2. Use appropriate intensity values (127-255 for visible graphics)
3. Place commonly-used sprites first in assets/ for faster access
4. Keep music files under ~80-100 notes for optimal size
5. Use loops in music (loopStart/loopEnd) for repetition instead of duplicating notes
6. Test assets in emulator before deploying to real hardware

## Hardware Constraints:
- 1KB RAM total (0xC800-0xCFFF)
- 8K ROM BIOS (0xE000-0xFFFF) 
- Motorola 6809 CPU @ 1.5 MHz
- Vector display with X/Y deflection
- 4-channel sound via AY-3-8912 PSG

## 🚨 CRITICAL Vectrex Coordinate System:
- **Screen center is (0, 0)** - NOT top-left corner!
- **Valid range: -127 to +127** for both X and Y axes
- **X axis**: -127 (far left) → 0 (center) → +127 (far right)
- **Y axis**: -127 (bottom) → 0 (center) → +127 (top)
- **Examples**:
  - Top-left corner: (-127, 127)
  - Top-right corner: (127, 127)
  - Bottom-left corner: (-127, -127)
  - Bottom-right corner: (127, -127)
  - Center: (0, 0)

## Programming Patterns:
1. **Two required functions**: \`def main():\` for initialization and \`def loop():\` for game logic
2. **main() runs once**: Use for initializing variables, setting up game state
3. **loop() runs every frame**: Use for game logic, drawing, input handling (60 FPS)
4. **❌ NEVER add WAIT_RECAL() in loop()**: Backend automatically handles frame synchronization - adding it manually causes timing issues
5. **Use safe intensity values**: ALWAYS use intensity ≤127 (0x7F) - higher values cause invisible lines
6. **DRAW_LINE coordinates are SCREEN POSITIONS**: Specify start point (x1,y1) and end point (x2,y2) as absolute screen coordinates
7. **Compiler converts to BIOS format**: Backend calculates dx=x2-x1, dy=y2-y1 and generates: Moveto_d(x1,y1) + Draw_Line_d(dx,dy)

## Required Program Structure:
\\\`\\\`\\\`vpy
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
\\\`\\\`\\\`

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
`;

export const VECTREX_HARDWARE_CONTEXT = `
# Vectrex Hardware Context

## Display System:
- Vector CRT display (not raster/pixel-based)
- Electron beam draws lines directly
- Intensity controls line brightness (CRITICAL: use ≤127 for safe display)
- No frame buffer - real-time drawing

## Safe Intensity Values (ALWAYS USE THESE):
- **127 (0x7F)**: Maximum safe brightness (bright, clear lines)
- **80 (0x50)**: Medium brightness (recommended for most graphics)
- **64 (0x40)**: Low-medium brightness (good for background elements)
- **48 (0x30)**: Dim (subtle effects)
- **0**: Invisible (beam off)

⚠️ **NEVER use intensity values above 127** - values like 150, 200, 255 cause:
  - CRT phosphor oversaturation
  - Lines become invisible or distorted
  - Potential burn-in damage on real hardware
  - Emulator may show incorrect behavior

## Memory Map:
- 0x0000-0xBFFF: Cartridge ROM space
- 0xC800-0xCFFF: System RAM (1KB)
- 0xD000-0xD7FF: 6522 VIA (I/O)
- 0xE000-0xFFFF: System ROM (BIOS)

## Sound System:
- General Instruments AY-3-8912 PSG
- 3 tone channels + 1 noise channel
- Memory-mapped at 0xD000-0xD001

## Input System:
- 4 controller ports
- Analog joystick (X/Y axes)
- 4 digital buttons per controller
- Light pen support

## BIOS Functions:
- F312: Move beam to absolute position
- F3DF: Draw line with relative displacement  
- F373: Set beam intensity
- F37A: Print text using ROM character set
- F36B: Reset coordinate origin
`;

export function getVPyContext(): string {
  const functionsDoc = VPY_FUNCTIONS.map(fn => `
### ${fn.name}
**Syntax**: \`${fn.syntax}\`
**Description**: ${fn.description}
**Parameters**:
${fn.parameters.map(p => `  - ${p.name} (${p.type}${p.required ? ', required' : ', optional'}): ${p.description}`).join('\n')}
**Examples**:
\`\`\`vpy
${fn.examples.join('\n')}
\`\`\`
${fn.notes ? `**Notes**: ${fn.notes}` : ''}
${fn.vectrexAddress ? `**BIOS Address**: ${fn.vectrexAddress}` : ''}
`).join('\n');

  const constantsDoc = VPY_CONSTANTS.map(c => `- **${c.name}**: ${c.value} - ${c.description}`).join('\n');

  return `${VPY_LANGUAGE_CONTEXT}

## Available Functions:
${functionsDoc}

## Constants:
${constantsDoc}

${VECTREX_HARDWARE_CONTEXT}`;
}

export function getProjectContext(activeFileName?: string, projectFiles?: string[]): string {
  return `
# Current Project Context

## Active File: ${activeFileName || 'None'}

## Project Structure:
${projectFiles ? projectFiles.map(f => `- ${f}`).join('\n') : 'No files loaded'}

## VPy IDE Environment:
- **IDE Type**: VS Code-style integrated development environment
- **Target Platform**: Vectrex console emulation via JSVecX
- **Language**: VPy (Vectrex Python) - procedural, not object-oriented
- **Compiler**: VPy → 6809 Assembly → Vectrex executable
- **Emulator**: Built-in JSVecX Vectrex emulator
## Common Trigonometric Patterns:

### Rotating Line:
\`\`\`vpy
var angle = 0
def loop():
    WAIT_RECAL()
    let x = cos(angle) / 2
    let y = sin(angle) / 2
    DRAW_LINE(0, 0, x, y, 127)
    angle = angle + 1
    if angle > 127: angle = 0
\`\`\`

### Rotating Triangle:
\`\`\`vpy
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
\`\`\`
### Circular Motion:
\`\`\`vpy
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
\`\`\` t > 127: t = 0
\`\`\`

- **AI Assistant**: PyPilot with context-aware VPy expertise

## Available IDE Features:
- 📝 **Code Editor**: Syntax highlighting, auto-completion, error detection
- 🎮 **Integrated Emulator**: Real-time VPy code execution and vector graphics
- 🤖 **PyPilot AI**: Code generation, error fixing, optimization, explanations
- 📁 **File Management**: Project explorer, file operations, multiple files
- 🔧 **Development Tools**: Compile & run, debug output, settings
- 🎨 **Customizable UI**: Dockable panels, themes, responsive layout

## PyPilot AI Assistant:
The user has access to PyPilot, an AI assistant specialized in VPy development that can:
- Generate VPy code from natural language descriptions
- Explain existing code and its functionality
- Fix compilation and runtime errors
- Optimize code for better performance
- Provide VPy language and Vectrex hardware guidance
- Answer questions about IDE features and usage

### MCP Integration (Model Context Protocol):
PyPilot connects to the IDE via MCP protocol with **22 specialized tools**:

#### 📝 Editor Tools (7):
- **editor_write_document**: Create NEW file OR replace ENTIRE content (preferred for full file updates)
- **editor_read_document**: Read ALREADY OPEN document (fails if document not open - see editor_list_documents first)
- **editor_list_documents**: List open documents (use before editor_read_document)
- **editor_replace_range**: Replace specific LINES (NOT character offsets) in open document (requires startLine/endLine)
- **editor_insert_at**: Insert text at line/column in open document
- **editor_delete_range**: Delete text range by line/column in open document
- **editor_get_diagnostics**: Get compilation/lint errors

#### 📁 Project Tools (8):
- **project_create**: Create new VPy project (with folder dialog)
- **project_open**: Open existing project
- **project_close**: Close current project
- **project_get_structure**: Get project file tree
- **project_read_file**: Read any project file
- **project_write_file**: Write any project file
- **project_create_vector**: Create .vec file with **JSON validation**
- **project_create_music**: Create .vmus file with **JSON validation**

#### 🔧 Compiler Tools (2):
- **compiler_build**: Compile VPy program
- **compiler_get_errors**: Get compilation errors

#### 🎮 Emulator Tools (3):
- **emulator_run**: Execute compiled ROM
- **emulator_get_state**: Get CPU state (PC, registers, cycles)
- **emulator_stop**: Stop emulation

#### 🐛 Debugger Tools (2):
- **debugger_add_breakpoint**: Add breakpoint at line
- **debugger_get_callstack**: Get current call stack

### Asset File Formats (CRITICAL):

#### Vector Graphics (.vec) - JSON FORMAT ONLY:

**✅ VERIFIED WORKING FORMAT** (as of 2025-12-12):
\`\`\`json
{
  "version": "1.0",
  "name": "asset_name",
  "canvas": {"width": 256, "height": 256, "origin": "center"},
  "layers": [{
    "name": "default",
    "visible": true,
    "paths": [{
      "name": "path_id",
      "intensity": 127,
      "closed": true,
      "points": [
        {"x": 0, "y": 20},
        {"x": -15, "y": -10},
        {"x": 15, "y": -10}
      ]
    }]
  }]
}
\`\`\`

**🔑 MANDATORY FIELDS (DO NOT OMIT)**:
- **version**: Always \`"1.0"\`
- **name**: Asset identifier (matches filename without .vec)
- **canvas**: \`{"width": 256, "height": 256, "origin": "center"}\`
- **layers[].name**: Any string (typically \`"default"\`)
- **layers[].visible**: **MUST BE \`true\`** - only visible layers render
- **paths[].name**: Unique identifier for path
- **paths[].intensity**: 0-255 (brightness, recommended 80-127)
- **paths[].closed**: \`true\`=polygon (auto-closes), \`false\`=open line
- **paths[].points**: Array of \`{"x": int, "y": int}\` objects
- **Point coordinates**: -127 to +127 (origin at center)

**Multi-Path Example (Spaceship with Wings)**:
\`\`\`json
{
  "version": "1.0",
  "name": "spaceship",
  "canvas": {"width": 256, "height": 256, "origin": "center"},
  "layers": [{
    "name": "default",
    "visible": true,
    "paths": [
      {
        "name": "hull",
        "intensity": 127,
        "closed": true,
        "points": [
          {"x": 0, "y": 25},
          {"x": -10, "y": -15},
          {"x": 0, "y": -10},
          {"x": 10, "y": -15}
        ]
      },
      {
        "name": "wing_left",
        "intensity": 100,
        "closed": false,
        "points": [
          {"x": -10, "y": -15},
          {"x": -20, "y": -20}
        ]
      },
      {
        "name": "wing_right",
        "intensity": 100,
        "closed": false,
        "points": [
          {"x": 10, "y": -15},
          {"x": 20, "y": -20}
        ]
      }
    ]
  }]
}
\`\`\`

**✅ PROVEN WORKING ASSETS** (verified 2025-12-12):
- **moon.vec**: 3 paths (circle + 2 craters)
- **enemigo.vec**: 9 paths (complex alien character)
- **astronauta.vec**: 6 paths (humanoid with limbs)
- **bullet.vec**: 2 paths (core + trail)
- **cohete_base.vec**: 5 paths (rocket with fins)
- **ejemplo.vec**: 6 paths (character with arms/legs)
- **player.vec**: 2 paths (ship body + cockpit)
- **nave3d.vec**: 3 paths (ship body + 2 wings)

**❌ COMMON MISTAKES TO AVOID**:
- Missing \`"visible": true\` → paths won't render
- Coordinates outside -127 to +127 → clipped or invisible
- Intensity > 127 → CRT oversaturation (use ≤127 for safety)
- Forgetting to close \`"closed": true\` polygons properly
- One-line JSON → hard to debug (always use formatted JSON)

**🎯 Multi-Path Architecture**:
- Each path draws independently (separate JSR Draw_Sync_List)
- DRAW_VECTOR("name") iterates all paths in order
- Each path ends with FCB 2 marker in compiled ASM
- Maximum tested: 9 paths (enemigo.vec) ✅

❌ **REJECTED FORMATS**: VECTOR_START, MOVE, DRAW_TO, or any text-based format
✅ **VALIDATION**: project_create_vector validates JSON structure and rejects invalid formats

#### Music Files (.vmus) - JSON FORMAT ONLY:
\`\`\`json
{
  "version": "1.0",
  "name": "My Song",
  "author": "Composer Name",
  "tempo": 120,
  "ticksPerBeat": 24,
  "totalTicks": 384,
  "notes": [
    {
      "id": "note1",
      "note": 60,
      "start": 0,
      "duration": 48,
      "velocity": 12,
      "channel": 0
    },
    {
      "id": "note2",
      "note": 64,
      "start": 48,
      "duration": 48,
      "velocity": 10,
      "channel": 0
    }
  ],
  "noise": [
    {
      "id": "noise1",
      "start": 0,
      "duration": 24,
      "period": 15,
      "channels": 1,
      "velocity": 12
    }
  ],
  "loopStart": 0,
  "loopEnd": 384
}
\`\`\`

**CRITICAL Field Definitions**:
- **note**: MIDI note number (0-127, where 60=middle C, 72=C5) - Only for notes
- **velocity**: Volume (0-15, where 15=max volume) - Used for BOTH notes AND noise (NEW: noise supports velocity since 2025-12-15)
- **period**: Noise period (0-31, lower=higher pitch noise) - Only for noise
- **channels**: Bitmask for noise (1=A, 2=B, 4=C, 7=all) - Only for noise
- **channel**: PSG channel (0=A, 1=B, 2=C) - Only for notes
- **start/duration**: Time in ticks (ticksPerBeat * beats)
- **id**: Unique identifier string for each note/noise event

**SIZE LIMITS (UPDATED)**:
✅ **Limit expanded**: max_tokens increased from 2000 to 8000 (~100 notes approx)
⚠️ **Recommendation**: Keep songs under ~80-100 total notes to avoid truncation
💡 **Best practice**: For longer songs, use short loops + loopStart/loopEnd for repetition
💡 **Loop advantage**: Smaller files, more efficient, same musical effect

❌ **REJECTED FORMATS**: Using "pitch" (Hz) instead of "note" (MIDI), "frequency" instead of "period", missing required fields, files >4KB
✅ **VALIDATION**: project_create_music validates JSON structure and rejects invalid formats

### MCP Tool Usage Rules:

#### Creating New Files:
✅ **Use editor_write_document**: Create .vpy files, general text files
✅ **Use project_create_vector**: Create .vec files (validates JSON)
✅ **Use project_create_music**: Create .vmus files (validates JSON)
❌ **Don't use editor_read_document**: Fails if file doesn't exist yet
❌ **Don't use editor_replace_range**: Requires file to be open first

#### Editing Existing Files:
1. **For complete replacement**: Use **editor_write_document** (replaces entire content)
2. **For partial edits**:
   - First: **editor_list_documents** (verify file is open)
   - Then: **editor_replace_range** (requires startLine/endLine, NOT offsets)
   - Or: **editor_insert_at** / **editor_delete_range**

#### Common Mistakes:
❌ **editor_read_document** on new file → "Document not found" error
❌ **editor_replace_range** with start/end offsets → "Missing line parameters" error  
✅ **editor_write_document** for new/existing → Works always
✅ **project_create_music** for .vmus → JSON validated automatically

### MCP Tool Behavior:
- **Auto-open files**: All created files automatically open in editor
- **Auto-detect language**: .vpy → VPy, .vec/.vmus/.json → JSON
- **File metadata**: mtime, size added automatically (files marked as saved, no asterisk)
- **Auto-create directories**: Creates assets/vectors/, assets/music/ as needed
- **Validation errors**: Show correct format example when validation fails
- **Learning feedback**: AI learns correct format through validation errors

### Important MCP Rules:
1. **Use specialized tools**: project_create_vector for .vec (NOT editor_write_document)
2. **JSON is mandatory**: .vec and .vmus files MUST be valid JSON
3. **No invented formats**: Tool descriptions and validation enforce correct structure
4. **Tool names**: Use snake_case (editor_write_document, not editor/write/document)
5. **Verify before using**: Check tools/list to confirm available tools
6. **ALWAYS pass required arguments**: Every tool call MUST include required parameters

### MCP Tool Call Examples:
\`\`\`javascript
// CORRECT - Create vector with name only (uses template)
{"tool": "project_create_vector", "arguments": {"name": "spaceship"}}

// CORRECT - Create vector with custom JSON content
{"tool": "project_create_vector", "arguments": {
  "name": "triangle",
  "content": "{\\"version\\":\\"1.0\\",\\"layers\\":[{\\"paths\\":[{\\"closed\\":true,\\"points\\":[{\\"x\\":0,\\"y\\":20},{\\"x\\":-15,\\"y\\":-10},{\\"x\\":15,\\"y\\":-10}]}]}]}"
}}

// INCORRECT - Missing required "name" argument
{"tool": "project_create_vector", "arguments": {}}  // ❌ WILL FAIL

// INCORRECT - Passing content without name
{"tool": "project_create_vector", "arguments": {"content": "..."}}  // ❌ WILL FAIL
\`\`\`

## User Interaction Context:
The user is working in a professional IDE environment with:
- Full project management capabilities
- Real-time code execution and testing
- Comprehensive development tools
- AI-powered assistance for VPy development via MCP protocol
- Integrated help and documentation system
- Validated asset creation (vectors, music) with JSON enforcement
`;
}