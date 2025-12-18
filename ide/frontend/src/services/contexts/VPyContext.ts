/**
 * VPy Language Context - Provides comprehensive context about VPy and Vectrex development
 * 
 * Documentation sourced from separate markdown files in docs/ folder:
 * - docs/vpy-language.md - Language specification and rules
 * - docs/vpy-metadata.md - META fields documentation
 * - docs/vpy-assets.md - Asset system (vectors and music)
 * - docs/vectrex-hardware.md - Hardware reference
 * - docs/vpy-patterns.md - Programming patterns and best practices
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
    notes: "IMPORTANT: Use values ≤127 for safe display. Values 128-255 cause CRT oversaturation, burn-in risk, and invisible lines."
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
    name: "DRAW_VECTOR",
    syntax: "DRAW_VECTOR(name, x, y)",
    description: "Draws a vector asset at absolute position (x, y)",
    parameters: [
      { name: "name", type: "string", description: "Name of the vector asset (without .vec extension)", required: true },
      { name: "x", type: "number", description: "X coordinate (-127 to 127, center=0)", required: true },
      { name: "y", type: "number", description: "Y coordinate (-127 to 127, center=0)", required: true }
    ],
    examples: [
      "var player_x = 0",
      "var player_y = -80",
      "def loop():",
      "    SET_INTENSITY(127)",
      "    DRAW_VECTOR(\"player\", player_x, player_y)"
    ],
    category: "assets",
    notes: "IMPORTANT: intensity values in .vec file MUST be ≤127 - higher values cause invisible lines!"
  },
  {
    name: "PLAY_MUSIC",
    syntax: "PLAY_MUSIC(name)",
    description: "Plays PSG music from embedded .vmus file",
    parameters: [
      { name: "name", type: "string", description: "Name of the music asset (without .vmus extension)", required: true }
    ],
    examples: [
      "def main():",
      "    PLAY_MUSIC(\"theme\")",
      "def loop():",
      "    MUSIC_UPDATE()       # REQUIRED for playback"
    ],
    category: "assets",
    notes: "Must call MUSIC_UPDATE() every frame in loop() for actual playback"
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

/**
 * VPy Language Context String
 * For comprehensive documentation, refer to the markdown files in docs/ folder
 */
export const VPY_LANGUAGE_CONTEXT = `
# VPy Language Context

VPy (Vectrex Python) is a domain-specific language for Vectrex game development.
Refer to docs/ folder for comprehensive documentation.

## Quick Reference:

### Variable Declaration:
- 'var' = Global (outside functions)
- 'let' = Local (inside functions)

### Required Functions:
- def main(): - Initialization
- def loop(): - Game loop (60 FPS)

### Safe Intensity Values:
- ALWAYS use ≤127 (use 127, 80, 64, 48, or 0)
- NEVER use values > 127 (causes invisible lines)

### Coordinate System:
- Center: (0, 0)
- Range: -127 to +127
- X: left to right
- Y: bottom to top

### Asset System:
- Vector graphics: assets/vectors/*.vec (JSON)
- Music files: assets/music/*.vmus (JSON)
- Access: DRAW_VECTOR("name"), PLAY_MUSIC("name")

For full documentation, see docs/ folder.
`;

export const VECTREX_HARDWARE_CONTEXT = `
# Vectrex Hardware Reference

See docs/vectrex-hardware.md for comprehensive hardware information.

## Key Facts:
- 1KB RAM (0xC800-0xCFFF)
- 8K BIOS ROM (0xE000-0xFFFF)
- Motorola 6809 @ 1.5 MHz
- Vector CRT display (lines, not pixels)
- AY-3-8912 PSG (3 tone + 1 noise channel)

## Critical: Safe Intensity Values
- ALWAYS ≤127 (0x7F)
- Values 128-255 = invisible lines + CRT damage risk
- Use: 127 (max), 80 (medium), 64 (low), 48 (dim), 0 (off)

## Coordinate System
- Center: (0, 0) - NOT top-left!
- Range: -127 to +127 on both X and Y
`;

export const IDE_AND_GIT_CONTEXT = `
# VPy IDE Environment

## Available Tools:
- Code editor with VPy syntax highlighting
- Integrated JSVecX Vectrex emulator
- PyPilot AI assistant for code generation
- Git version control integration
- Project and asset management

## IDE Features:
- Compile & Run (F5 or Ctrl+Shift+B)
- Debug with breakpoints and call stack
- Vector (.vec) and Music (.vmus) asset creation
- Real-time code execution
- Multi-file project support

## PyPilot AI Assistant:
- Context-aware VPy expertise
- Code generation from descriptions
- Error analysis and fixes
- Optimization suggestions
- Vectrex hardware guidance

## MCP (Model Context Protocol):
23 specialized tools for AI integration and project management.

### CRITICAL: MCP Tool Usage Workflow

**ALWAYS follow this sequence when modifying code:**

1. **Modify file** using \`editor_write_document\`
   - Pass complete file content (not partial edits)
   - Provides uri (file path) and content parameters

2. **SAVE the file** immediately using \`editor_save_document\`
   - CRITICAL: Must save to disk before compilation
   - Compiler reads from disk, not editor state
   - Pass uri parameter (same as write_document)

3. **Compile the project** using \`compiler_build_and_run\`
   - This is the NEW unified tool (replaces separate build+run steps)
   - Automatically compiles AND runs in emulator if successful
   - No parameters needed (uses current project configuration)

4. **VALIDATE compilation output**
   - Check the returned \`success\` field
   - If \`success: false\`, read the \`errors\` array
   - If \`success: true\`, check \`binPath\` exists
   - Verify \`phase\` field: 'compilation' or 'execution'

5. **Handle compilation errors**
   - Parse \`errors\` array for line/column/message
   - Use \`editor_read_document\` to read source if needed
   - Fix errors and repeat from step 1

**Example workflow:**
\`\`\`
1. editor_write_document({uri: "main.vpy", content: "..."})
2. editor_save_document({uri: "main.vpy"})
3. compiler_build_and_run({})
4. Check result.success:
   - true → Program running in emulator
   - false → Read result.errors, fix code, repeat
\`\`\`

### Key MCP Tools:

**Editor Tools:**
- \`editor_write_document\` - CREATE or UPDATE file (auto-opens in editor)
- \`editor_save_document\` - SAVE file to disk (REQUIRED before compilation)
- \`editor_read_document\` - Read file content (file MUST be open first)
- \`editor_list_documents\` - List all open documents

**Compiler Tools:**
- \`compiler_build_and_run\` - **USE THIS**: Compile + Run in one step
- \`compiler_build\` - Only compile (no auto-run)
- \`compiler_get_errors\` - Get current diagnostics from editor

**Emulator Tools:**
- \`emulator_run\` - Load ROM file into emulator
- \`emulator_stop\` - Stop emulation
- \`emulator_get_state\` - Get PC, registers, cycles, FPS

**Project Tools:**
- \`project_create_vector\` - Create .vec file (validates JSON)
- \`project_create_music\` - Create .vmus file (validates JSON)
- \`project_read_file\` - Read any project file
- \`project_write_file\` - Write any project file

### Common Mistakes to AVOID:

❌ Using \`editor_read_document\` on unopened files → Use \`editor_write_document\` to create
❌ Forgetting to save after \`editor_write_document\` → **ALWAYS use \`editor_save_document\` after write**
❌ Compiling without saving → Compiler reads disk, not editor state
❌ Using \`editor_replace_range\` for new files → Requires file open first, use \`editor_write_document\`
❌ Calling \`compiler_build\` then \`emulator_run\` separately → Use \`compiler_build_and_run\` instead
❌ Not checking \`result.success\` after compilation → Must validate before assuming success
❌ Inventing tool names like \`emulator_compile_and_run\` → Tool doesn't exist, use \`compiler_build_and_run\`
❌ Passing undefined parameters → Always provide required parameters (e.g., \`uri\` for read_document)

### Parameter Requirements:

**editor_write_document:**
- \`uri\`: string (required) - File path or name
- \`content\`: string (required) - Complete file content

**editor_save_document:**
- \`uri\`: string (required) - File URI to save (must be open)

**editor_read_document:**
- \`uri\`: string (required) - MUST match open document URI exactly

**compiler_build_and_run:**
- No parameters required (uses current project)
- Optional: \`breakOnEntry\`: boolean (pause at first instruction)

**project_create_vector:**
- \`name\`: string (required) - Filename without .vec extension
- \`content\`: string (optional) - JSON content, leave empty for template

**project_create_music:**
- \`name\`: string (required) - Filename without .vmus extension  
- \`content\`: string (optional) - JSON content, leave empty for template
`;

export function getVPyContext(): string {
  const functionsDoc = VPY_FUNCTIONS.map(fn => `
### ${fn.name}
**Syntax**: \`${fn.syntax}\`
**Description**: ${fn.description}
**Parameters**:
${fn.parameters.length > 0 ? fn.parameters.map(p => `  - ${p.name} (${p.type}${p.required ? ', required' : ', optional'}): ${p.description}`).join('\n') : '  (none)'}
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

${VECTREX_HARDWARE_CONTEXT}

${IDE_AND_GIT_CONTEXT}`;
}

export function getProjectContext(activeFileName?: string, projectFiles?: string[]): string {
  return `
# Current Project Context

## Active File: ${activeFileName || 'None'}

## Project Files:
${projectFiles ? projectFiles.map(f => `- ${f}`).join('\n') : 'No files loaded'}

## Documentation Links:
- docs/vpy-language.md - Language reference
- docs/vpy-metadata.md - META fields
- docs/vpy-assets.md - Asset system
- docs/vpy-patterns.md - Programming patterns
- docs/vectrex-hardware.md - Hardware specs
`;
}
