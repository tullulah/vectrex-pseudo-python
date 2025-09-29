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
  {
    name: "MOVE_TO",
    syntax: "MOVE_TO(x, y)",
    description: "Moves the electron beam to absolute coordinates without drawing",
    parameters: [
      { name: "x", type: "int", description: "X coordinate (-127 to +127)", required: true },
      { name: "y", type: "int", description: "Y coordinate (-127 to +127)", required: true }
    ],
    examples: [
      "MOVE_TO(0, 0)  # Move to center",
      "MOVE_TO(-100, 50)  # Move to upper left area"
    ],
    category: "movement",
    vectrexAddress: "0xF312",
    notes: "Center of screen is (0,0). Coordinate system: -127 (left/bottom) to +127 (right/top)"
  },
  {
    name: "DRAW_TO",
    syntax: "DRAW_TO(x, y)",
    description: "Draws a line from current position to absolute coordinates",
    parameters: [
      { name: "x", type: "int", description: "X coordinate (-127 to +127)", required: true },
      { name: "y", type: "int", description: "Y coordinate (-127 to +127)", required: true }
    ],
    examples: [
      "DRAW_TO(50, 0)  # Draw line to right",
      "DRAW_TO(0, -30)  # Draw line down",
      "DRAW_TO(25, 25)  # Draw line to diagonal"
    ],
    category: "drawing",
    vectrexAddress: "0xF3DF",
    notes: "Intensity must be > 0 to see the line. Draws from current beam position to target"
  },
  {
    name: "DRAW_LINE",
    syntax: "DRAW_LINE(x1, y1, x2, y2, intensity)",
    description: "Draws a line from one point to another with specified intensity",
    parameters: [
      { name: "x1", type: "int", description: "Start X coordinate (-127 to +127)", required: true },
      { name: "y1", type: "int", description: "Start Y coordinate (-127 to +127)", required: true },
      { name: "x2", type: "int", description: "End X coordinate (-127 to +127)", required: true },
      { name: "y2", type: "int", description: "End Y coordinate (-127 to +127)", required: true },
      { name: "intensity", type: "int", description: "Line intensity (0-255)", required: true }
    ],
    examples: [
      "DRAW_LINE(0, 0, 50, 50, 255)  # Diagonal line at max brightness",
      "DRAW_LINE(-25, 0, 25, 0, 128)  # Horizontal line at half brightness"
    ],
    category: "drawing",
    notes: "Complete line drawing function with start point, end point and intensity"
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
    notes: "0 = beam off, 255 = maximum brightness. Use this in regular VPy code. For vectorlists, use INTENSITY without parentheses"
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
      "WAIT_RECAL()  # Wait for next frame"
    ],
    category: "timing",
    notes: "Synchronizes with 60 FPS display refresh rate. Essential for smooth animation"
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

## CRITICAL: Function Context Differences:
⚠️ **Regular VPy vs Vectorlist Functions**:
• Use MOVE_TO(x, y) in regular VPy code - NOT MOVE(x, y)
• Use SET_INTENSITY(value) in regular VPy code - NOT INTENSITY(value)
• Use DRAW_TO(x, y) in regular VPy code for drawing to absolute coordinates
• Use PRINT_TEXT(x, y, text) works in both contexts
• MOVE and INTENSITY (without parentheses) only work inside vectorlists!

Example of CORRECT usage:

    # Regular VPy code - use these functions with parentheses
    SET_INTENSITY(255)
    MOVE_TO(0, 0) 
    PRINT_TEXT(0, 50, "Hello")
    DRAW_TO(50, 50)

    # Inside vectorlist - different syntax without parentheses  
    vectorlist myshape:
        INTENSITY 128
        MOVE 0 0
        RECT -10 -10 10 10

## Supported Language Features:
✅ Variable assignments: x = 10, name = "Hello"
✅ Basic arithmetic: +, -, *, /, % (modulo)
✅ Comparison operators: ==, !=, <, >, <=, >=
✅ Boolean logic: and, or, not
✅ Conditional statements: if x > 0:, else:
✅ Loop constructs: for i in range(10):, while condition:
✅ Comments: # This is a comment
✅ Built-in Vectrex functions: MOVE_TO, DRAW_TO, DRAW_LINE, SET_INTENSITY, PRINT_TEXT, etc.

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

## VPy Project Structure and META Fields:

### 📄 **Project Metadata (META Fields)**
VPy supports exactly 3 META fields that define ROM header information:

\`\`\`vpy
META TITLE = "MY GAME"          # Game title (REQUIRED)
META COPYRIGHT = "g GCE 2025"   # Copyright string (optional)
META MUSIC = "music1"           # BIOS music symbol (optional)

# Your VPy code starts here
INTENSITY(255)
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
  - Example: \`META COPYRIGHT = "g GCE 2025"\`
  - **Default**: "g GCE 1998"
  - **Used for**: First line display in ROM header

- **MUSIC**: BIOS music symbol (optional)
  - Examples: \`META MUSIC = "music1"\` or \`META MUSIC = "0"\`
  - **Default**: "music1"
  - **Special**: Use "0" to disable music (FDB $0000)
  - **Used for**: Background music selection

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
META COPYRIGHT = "g DANIEL 2025"
META MUSIC = "0"

INTENSITY(255)
MOVE(-25, -25)
DRAW_LINE(50, 0)
DRAW_LINE(0, 50)
DRAW_LINE(-50, 0)
DRAW_LINE(0, -50)
\`\`\`

#### **Animation with Music:**
\`\`\`vpy
META TITLE = "ROTATING LINE"
META COPYRIGHT = "g VPY DEVELOPER 2025"
META MUSIC = "music1"

for frame in range(360):
    INTENSITY(200)
    angle = frame * 2
    x = angle % 60 - 30
    y = angle % 40 - 20
    MOVE(x, y)
    DRAW_LINE(30, 0)
    WAIT_FRAMES(1)
\`\`\`

#### **Minimal Example:**
\`\`\`vpy
META TITLE = "HELLO WORLD"

# Minimal code - other META fields use defaults
INTENSITY(255)
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
INTENSITY(255)          # Set bright intensity
MOVE(-50, 0)           # Move to starting position
DRAW_LINE(100, 0)      # Draw horizontal line
DRAW_LINE(0, 50)       # Draw vertical line up
DRAW_LINE(-100, 0)     # Draw back to start
DRAW_LINE(0, -50)      # Complete the rectangle

# Animation example with loop (CORRECT VPy code)
for frame in range(100):
    INTENSITY(255)
    angle = frame * 2
    x = angle % 100 - 50
    MOVE(x, 0)
    DRAW_LINE(0, 30)
    WAIT_FRAMES(1)

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

## Hardware Constraints:
- 1KB RAM total (0xC800-0xCFFF)
- 8K ROM BIOS (0xE000-0xFFFF) 
- Motorola 6809 CPU @ 1.5 MHz
- Vector display with X/Y deflection
- 4-channel sound via AY-3-8912 PSG

## Programming Patterns:
1. Set intensity before drawing (INTENSITY > 0)
2. Move to start position (MOVE)
3. Draw lines with relative coordinates (DRAW_LINE)
4. Use ORIGIN() to reset coordinate system
5. WAIT_FRAMES() for timing and animation

## Common Mistakes:
- Forgetting to set intensity (lines won't show)
- Using absolute coordinates for DRAW_LINE (should be relative)
- Coordinates outside -127 to +127 range
- Not considering 60 FPS timing for animations
- Trying to pass too many parameters to functions (max 2-3)
- Attempting to use unsupported Python features (classes, imports, etc.)
- Using undefined variables or complex expressions
- Thinking VPy is object-oriented (it's NOT!)
- Believing VPy was created in 1982 (completely false!)
`;

export const VECTREX_HARDWARE_CONTEXT = `
# Vectrex Hardware Context

## Display System:
- Vector CRT display (not raster/pixel-based)
- Electron beam draws lines directly
- Intensity controls line brightness
- No frame buffer - real-time drawing

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

## User Interaction Context:
The user is working in a professional IDE environment with:
- Full project management capabilities
- Real-time code execution and testing
- Comprehensive development tools
- AI-powered assistance for VPy development
- Integrated help and documentation system
`;
}