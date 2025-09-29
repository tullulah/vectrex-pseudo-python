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
    category: "movement",
    vectrexAddress: "0xF312",
    notes: "Center of screen is (0,0). Coordinate system: -127 (left/bottom) to +127 (right/top)"
  },
  {
    name: "DRAW_LINE",
    syntax: "DRAW_LINE(dx, dy)",
    description: "Draws a line from current position by relative displacement",
    parameters: [
      { name: "dx", type: "int", description: "X displacement (-127 to +127)", required: true },
      { name: "dy", type: "int", description: "Y displacement (-127 to +127)", required: true }
    ],
    examples: [
      "DRAW_LINE(50, 0)  # Horizontal line right",
      "DRAW_LINE(0, -30)  # Vertical line down",
      "DRAW_LINE(25, 25)  # Diagonal line"
    ],
    category: "drawing",
    vectrexAddress: "0xF3DF",
    notes: "Intensity must be > 0 to see the line. Uses current beam position as starting point"
  },
  {
    name: "INTENSITY",
    syntax: "INTENSITY(value)",
    description: "Sets the electron beam intensity (brightness)",
    parameters: [
      { name: "value", type: "int", description: "Intensity level (0-255)", required: true }
    ],
    examples: [
      "INTENSITY(255)  # Maximum brightness",
      "INTENSITY(128)  # Half brightness", 
      "INTENSITY(0)    # Beam off (invisible)"
    ],
    category: "intensity",
    vectrexAddress: "0xF373",
    notes: "0 = beam off, 255 = maximum brightness. Affects all subsequent drawing operations"
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
    notes: "Uses Vectrex ROM character set. Limited to 3 parameters in current VPy implementation (no size parameter yet)"
  },
  {
    name: "ORIGIN",
    syntax: "ORIGIN()",
    description: "Resets the coordinate reference point to screen center (0,0)",
    parameters: [],
    examples: [
      "ORIGIN()  # Reset to center"
    ],
    category: "control",
    vectrexAddress: "0xF36B",
    notes: "Useful after drawing complex shapes to reset coordinate system"
  },
  {
    name: "WAIT_FRAMES",
    syntax: "WAIT_FRAMES(frames)",
    description: "Pauses execution for specified number of video frames",
    parameters: [
      { name: "frames", type: "int", description: "Number of frames to wait (60 fps)", required: true }
    ],
    examples: [
      "WAIT_FRAMES(60)  # Wait 1 second",
      "WAIT_FRAMES(30)  # Wait 0.5 seconds"
    ],
    category: "timing",
    notes: "Vectrex runs at 60 FPS, so 60 frames = 1 second"
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

## Core Concepts:
- **Vector Graphics**: Vectrex uses vector (line-based) graphics, not pixels
- **Coordinate System**: Center (0,0), range -127 to +127 on both axes
- **Beam Control**: Electron beam intensity controls line brightness
- **Real-time**: Code runs at 60 FPS on real Vectrex hardware

## Current Implementation Limitations:
- **Function Parameters**: Maximum 2-3 parameters per function call (compiler limitation)
- **No Object-Oriented Programming**: Classes, objects, inheritance not implemented
- **No Complex Data Structures**: No lists, dictionaries, tuples, or custom types
- **Primitive Variables Only**: Only int, string, and basic numeric types
- **Simple Control Flow**: Basic if/else, for/while loops only
- **No Module System**: No imports, packages, or external libraries
- **No Exception Handling**: No try/catch or error handling constructs
- **Direct BIOS Mapping**: Functions compile directly to Vectrex BIOS calls
- **No Function Definitions**: Cannot define custom functions (yet)
- **No String Manipulation**: Limited string operations, mostly for display

## Supported Language Features:
- Python-like syntax with Vectrex-specific functions
- Variable assignments: x = 10, name = "Hello"
- Basic arithmetic: +, -, *, /, % (modulo)
- Comparison operators: ==, !=, <, >, <=, >=
- Boolean logic: and, or, not
- Conditional statements: if x > 0:, else:
- Loop constructs: for i in range(10):, while condition:
- Comments: # This is a comment

## Code Examples:
\`\`\`vpy
# Simple drawing example
INTENSITY(255)          # Set bright intensity
MOVE(-50, 0)           # Move to starting position
DRAW_LINE(100, 0)      # Draw horizontal line
DRAW_LINE(0, 50)       # Draw vertical line up
DRAW_LINE(-100, 0)     # Draw back to start
DRAW_LINE(0, -50)      # Complete the rectangle

# Animation example with loop
for frame in range(100):
    INTENSITY(255)
    angle = frame * 2
    x = angle % 100 - 50
    MOVE(x, 0)
    DRAW_LINE(0, 30)
    WAIT_FRAMES(1)
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

## Development Environment:
- IDE: VPy IDE (VS Code-like interface)
- Target: Vectrex console emulation
- Language: VPy (Vectrex Python)
- Compiler: VPy → 6809 Assembly
- Emulator: Built-in Vectrex emulator
`;
}