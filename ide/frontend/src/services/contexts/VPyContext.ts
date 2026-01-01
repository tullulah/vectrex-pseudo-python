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
    name: "PRINT_TEXT",
    syntax: "PRINT_TEXT(x, y, text[, height, width])",
    description: "Displays text on screen at specified position with optional custom size",
    parameters: [
      { name: "x", type: "number", description: "X position (-127 to 127, center=0)", required: true },
      { name: "y", type: "number", description: "Y position (-127 to 127, center=0)", required: true },
      { name: "text", type: "string", description: "String to display (or array element returning pointer)", required: true },
      { name: "height", type: "int", description: "Text height (-128 to -1, NEGATIVE, larger magnitude = taller). Optional, defaults to BIOS value.", required: false },
      { name: "width", type: "int", description: "Text width (1 to 127, POSITIVE, larger = wider). Optional, defaults to BIOS value.", required: false }
    ],
    examples: [
      "# Basic usage (3 parameters, uses BIOS defaults)",
      "PRINT_TEXT(0, 50, \"HELLO\")",
      "PRINT_TEXT(-60, -60, \"SCORE: 1000\")",
      "",
      "# With string arrays",
      "const location_names = [\"MOUNT FUJI\", \"PARIS\", \"NEW YORK\"]",
      "PRINT_TEXT(-70, -120, location_names[current_location])",
      "",
      "# Custom size (5 parameters) - Small text",
      "PRINT_TEXT(-70, -120, location_names[i], -4, 32)",
      "",
      "# Large title text",
      "PRINT_TEXT(-50, 0, \"GAME OVER\", -12, 96)",
      "",
      "# Medium menu text",
      "PRINT_TEXT(-40, 30, \"START GAME\", -6, 48)"
    ],
    category: "unified",
    vectrexAddress: "$F373 (Print_Str_d), $C82A (Vec_Text_Height), $C82B (Vec_Text_Width)",
    notes: "The last 2 parameters (height, width) are OPTIONAL. When used: height MUST be NEGATIVE (-4 = small, -6 = medium, -8 to -12 = large), width MUST be POSITIVE (32 = narrow, 48 = medium, 72-96 = wide). Custom size writes to Vec_Text_Height/Width before rendering."
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
      "player_x = 0  # Global variable",
      "player_y = -80  # Global variable",
      "def loop():",
      "    SET_INTENSITY(127)",
      "    DRAW_VECTOR(\"player\", player_x, player_y)"
    ],
    category: "assets",
    notes: "IMPORTANT: intensity values in .vec file MUST be ≤127 - higher values cause invisible lines!"
  },
  {
    name: "DRAW_VECTOR_EX",
    syntax: "DRAW_VECTOR_EX(name, x, y, mirror, intensity)",
    description: "Draws a vector asset with position offset, mirror transformation, and custom intensity",
    parameters: [
      { name: "name", type: "string", description: "Name of the vector asset (without .vec extension)", required: true },
      { name: "x", type: "number", description: "X position offset (-127 to 126)", required: true },
      { name: "y", type: "number", description: "Y position offset (-120 to 120)", required: true },
      { name: "mirror", type: "number", description: "Mirror mode: 0=normal, 1=X-flip (horizontal), 2=Y-flip (vertical), 3=both (180° rotation)", required: true },
      { name: "intensity", type: "number", description: "Custom intensity (0-127, overrides .vec file intensity)", required: true }
    ],
    examples: [
      "# Glow effect with dynamic intensity",
      "glow_intensity = 60",
      "def loop():",
      "    # Draw with variable brightness",
      "    DRAW_VECTOR_EX(\"star\", 0, 50, 0, glow_intensity)",
      "    # Update glow animation",
      "    glow_intensity = glow_intensity + 3",
      "    if glow_intensity >= 127:",
      "        glow_intensity = 30"
    ],
    category: "assets",
    notes: "The intensity parameter allows dynamic brightness control, perfect for glow effects, pulsing animations, or dimming sprites. Mirror modes: 0=normal, 1=X-flip (left-right), 2=Y-flip (top-bottom), 3=XY-flip (180° rotation). Intensity overrides the value in .vec file."
  },
  {
    name: "PLAY_MUSIC",
    syntax: "PLAY_MUSIC(name)",
    description: "Starts playback of PSG music from embedded .vmus asset",
    parameters: [
      { name: "name", type: "string", description: "Name of the music asset (without .vmus extension)", required: true }
    ],
    examples: [
      "def main():",
      "    PLAY_MUSIC(\"theme\")  # Start theme music",
      "",
      "def loop():",
      "    SET_INTENSITY(127)",
      "    DRAW_VECTOR(\"player\", x, y)  # All drawing here",
      "    # Audio updates auto-injected by compiler"
    ],
    category: "assets",
    notes: "IMPORTANT: Audio (music + SFX) is automatically updated every frame via AUDIO_UPDATE (auto-injected by compiler). No manual MUSIC_UPDATE() calls needed. Music continues playing across frames once started."
  },
  {
    name: "PLAY_SFX",
    syntax: "PLAY_SFX(name)",
    description: "Plays a sound effect from embedded .vsfx AYFX asset",
    parameters: [
      { name: "name", type: "string", description: "Name of the SFX asset (without .vsfx extension)", required: true }
    ],
    examples: [
      "def main():",
      "    PLAY_SFX(\"coin\")  # Load SFX (doesn't start yet)",
      "",
      "def loop():",
      "    if J1_BUTTON_1():",
      "        PLAY_SFX(\"jump\")  # Trigger jump sound effect",
      "    # Audio updates auto-injected (plays SFX frame-by-frame)"
    ],
    category: "assets",
    notes: "SFX uses AYFX format (Richard Chadd system, channel C). Can play simultaneously with PLAY_MUSIC. Audio updates are automatic via AUDIO_UPDATE (compiler auto-injected). Each PLAY_SFX() call restarts the effect from beginning."
  },
  {
    name: "AUDIO_UPDATE",
    syntax: "AUDIO_UPDATE()",
    description: "Updates both music and SFX playback (auto-injected - call not required)",
    parameters: [],
    examples: [
      "def loop():",
      "    WAIT_RECAL()  # 50 FPS sync",
      "    # AUDIO_UPDATE() is HERE (auto-injected by compiler)",
      "    ",
      "    # Your drawing code",
      "    DRAW_VECTOR(\"player\", x, y)",
      "    # Audio updates completed automatically"
    ],
    category: "assets",
    notes: "AUTOMATIC: Compiler auto-injects this after WAIT_RECAL in loop(). Updates music (channel B) and SFX (channel C) together. Sets DP=$D0 for BIOS Sound_Byte calls. No manual call needed - it's built-in."
  },
  {
    name: "J1_X",
    syntax: "J1_X()",
    description: "Reads Joystick 1 X axis position (DIGITAL by default)",
    parameters: [],
    examples: [
      "def loop():",
      "    x = J1_X()  # Returns -1 (left), 0 (center), +1 (right)",
      "    paddle_x = paddle_x + x * 4  # Multiply for speed (4 = medium, 8 = fast)"
    ],
    category: "input",
    vectrexAddress: "$F1F8 (Joy_Digital)",
    notes: "Default to DIGITAL mode (-1/0/+1). For analog values use J1_X_ANALOG(). Digital is MUCH faster and suitable for 60fps games."
  },
  {
    name: "J1_X_DIGITAL",
    syntax: "J1_X_DIGITAL()",
    description: "Reads Joystick 1 X axis position via BIOS Joy_Digital (explicit)",
    parameters: [],
    examples: [
      "def loop():",
      "    x = J1_X_DIGITAL()  # Returns -1 (left), 0 (center), +1 (right)",
      "    paddle_x = paddle_x + x * 4  # Multiply for speed"
    ],
    category: "input",
    vectrexAddress: "$F1F8 (Joy_Digital)",
    notes: "Explicit digital version. Returns -1/0/+1. Fast and suitable for 60fps games. Multiply by constant for speed control (e.g., x*2 for slow, x*4 for medium, x*8 for fast)."
  },
  {
    name: "J1_X_ANALOG",
    syntax: "J1_X_ANALOG()",
    description: "Reads Joystick 1 X axis position via BIOS Joy_Analog (full range)",
    parameters: [],
    examples: [
      "def loop():",
      "    x = J1_X_ANALOG()  # Returns -127 (full left) to +127 (full right)",
      "    paddle_x = paddle_x + x / 32  # Divide for smooth proportional control"
    ],
    category: "input",
    vectrexAddress: "$F1F5 (Joy_Analog)",
    notes: "SLOW! Analog version returns full range -127 to +127. May cause frame drops or freezing. Use only if you need fine-grained analog control. Digital version recommended for most games."
  },
  {
    name: "J1_Y",
    syntax: "J1_Y()",
    description: "Reads Joystick 1 Y axis position (DIGITAL by default)",
    parameters: [],
    examples: [
      "def loop():",
      "    y = J1_Y()  # Returns -1 (down), 0 (center), +1 (up)",
      "    ship_y = ship_y + y * 4  # Multiply for speed"
    ],
    category: "input",
    vectrexAddress: "$F1F8 (Joy_Digital)",
    notes: "Default to DIGITAL mode (-1/0/+1). For analog values use J1_Y_ANALOG(). Digital is MUCH faster."
  },
  {
    name: "J1_Y_DIGITAL",
    syntax: "J1_Y_DIGITAL()",
    description: "Reads Joystick 1 Y axis position via BIOS Joy_Digital (explicit)",
    parameters: [],
    examples: [
      "def loop():",
      "    y = J1_Y_DIGITAL()  # Returns -1 (down), 0 (center), +1 (up)",
      "    ship_y = ship_y + y * 4"
    ],
    category: "input",
    vectrexAddress: "$F1F8 (Joy_Digital)",
    notes: "Explicit digital version. Returns -1/0/+1. Fast and suitable for 60fps games."
  },
  {
    name: "J1_Y_ANALOG",
    syntax: "J1_Y_ANALOG()",
    description: "Reads Joystick 1 Y axis position via BIOS Joy_Analog (full range)",
    parameters: [],
    examples: [
      "def loop():",
      "    y = J1_Y_ANALOG()  # Returns -127 (full down) to +127 (full up)",
      "    ship_y = ship_y + y / 32  # Divide for smooth proportional control"
    ],
    category: "input",
    vectrexAddress: "$F1F5 (Joy_Analog)",
    notes: "SLOW! Analog version returns full range -127 to +127. May cause frame drops or freezing. Digital version recommended."
  },
  {
    name: "J1_BUTTON_1",
    syntax: "J1_BUTTON_1()",
    description: "Reads Joystick 1 Button 1 state via BIOS Read_Btns routine",
    parameters: [],
    examples: [
      "def loop():",
      "    if J1_BUTTON_1():",
      "        # Button 1 pressed - fire weapon",
      "        fire_bullet()"
    ],
    category: "input",
    vectrexAddress: "$F1BA (Read_Btns)",
    notes: "Uses official BIOS routine. Returns 0 (released) or 1 (pressed). Reads from Vec_Btn_State ($C80F) bit 0."
  },
  {
    name: "J1_BUTTON_2",
    syntax: "J1_BUTTON_2()",
    description: "Reads Joystick 1 Button 2 state via BIOS Read_Btns routine",
    parameters: [],
    examples: [
      "def loop():",
      "    if J1_BUTTON_2():",
      "        # Button 2 pressed"
    ],
    category: "input",
    vectrexAddress: "$F1BA (Read_Btns)",
    notes: "Returns 0 (released) or 1 (pressed). Reads from Vec_Btn_State ($C80F) bit 1."
  },
  {
    name: "J1_BUTTON_3",
    syntax: "J1_BUTTON_3()",
    description: "Reads Joystick 1 Button 3 state via BIOS Read_Btns routine",
    parameters: [],
    examples: [
      "def loop():",
      "    if J1_BUTTON_3():",
      "        # Button 3 pressed"
    ],
    category: "input",
    vectrexAddress: "$F1BA (Read_Btns)",
    notes: "Returns 0 (released) or 1 (pressed). Reads from Vec_Btn_State ($C80F) bit 2."
  },
  {
    name: "J1_BUTTON_4",
    syntax: "J1_BUTTON_4()",
    description: "Reads Joystick 1 Button 4 state via BIOS Read_Btns routine",
    parameters: [],
    examples: [
      "def loop():",
      "    if J1_BUTTON_4():",
      "        # Button 4 pressed"
    ],
    category: "input",
    vectrexAddress: "$F1BA (Read_Btns)",
    notes: "Returns 0 (released) or 1 (pressed). Reads from Vec_Btn_State ($C80F) bit 3."
  },
  {
    name: "J2_X",
    syntax: "J2_X()",
    description: "Reads Joystick 2 X axis position via BIOS Joy_Digital routine",
    parameters: [],
    examples: [
      "def loop():",
      "    x = J2_X()  # Returns -1 (left), 0 (center), or 1 (right)"
    ],
    category: "input",
    vectrexAddress: "$F1F8 (Joy_Digital)",
    notes: "Uses official BIOS routine. Returns digital value: -1/0/+1. Reads from Vec_Joy_2_X ($C81D)."
  },
  {
    name: "J2_Y",
    syntax: "J2_Y()",
    description: "Reads Joystick 2 Y axis position via BIOS Joy_Digital routine",
    parameters: [],
    examples: [
      "def loop():",
      "    y = J2_Y()  # Returns -1 (down), 0 (center), or 1 (up)"
    ],
    category: "input",
    vectrexAddress: "$F1F8 (Joy_Digital)",
    notes: "Uses official BIOS routine. Returns digital value: -1/0/+1. Reads from Vec_Joy_2_Y ($C81E)."
  },
  {
    name: "J2_BUTTON_1",
    syntax: "J2_BUTTON_1()",
    description: "Reads Joystick 2 Button 1 state via BIOS Read_Btns routine",
    parameters: [],
    examples: [
      "def loop():",
      "    if J2_BUTTON_1():",
      "        # Player 2 button 1 pressed"
    ],
    category: "input",
    vectrexAddress: "$F1BA (Read_Btns)",
    notes: "Returns 0 (released) or 1 (pressed). Reads from Vec_Btn_State ($C80F) bit 4."
  },
  {
    name: "J2_BUTTON_2",
    syntax: "J2_BUTTON_2()",
    description: "Reads Joystick 2 Button 2 state via BIOS Read_Btns routine",
    parameters: [],
    examples: [
      "def loop():",
      "    if J2_BUTTON_2():",
      "        # Player 2 button 2 pressed"
    ],
    category: "input",
    vectrexAddress: "$F1BA (Read_Btns)",
    notes: "Returns 0 (released) or 1 (pressed). Reads from Vec_Btn_State ($C80F) bit 5."
  },
  {
    name: "J2_BUTTON_3",
    syntax: "J2_BUTTON_3()",
    description: "Reads Joystick 2 Button 3 state via BIOS Read_Btns routine",
    parameters: [],
    examples: [
      "def loop():",
      "    if J2_BUTTON_3():",
      "        # Player 2 button 3 pressed"
    ],
    category: "input",
    vectrexAddress: "$F1BA (Read_Btns)",
    notes: "Returns 0 (released) or 1 (pressed). Reads from Vec_Btn_State ($C80F) bit 6."
  },
  {
    name: "J2_BUTTON_4",
    syntax: "J2_BUTTON_4()",
    description: "Reads Joystick 2 Button 4 state via BIOS Read_Btns routine",
    parameters: [],
    examples: [
      "def loop():",
      "    if J2_BUTTON_4():",
      "        # Player 2 button 4 pressed"
    ],
    category: "input",
    vectrexAddress: "$F1BA (Read_Btns)",
    notes: "Returns 0 (released) or 1 (pressed). Reads from Vec_Btn_State ($C80F) bit 7."
  },
  
  // === BUILT-IN LANGUAGE FUNCTIONS (NEW) ===
  {
    name: "len",
    syntax: "len(array)",
    description: "Returns the length (number of elements) of an array",
    parameters: [
      { name: "array", type: "array", description: "Array to get length of", required: true }
    ],
    examples: [
      "enemies = [0, 0, 0, 0, 0]  # Global array",
      "count = len(enemies)  # Returns 5",
      "",
      "for i = 0 to len(enemies):",
      "    print(enemies[i])"
    ],
    category: "builtin",
    notes: "Returns compile-time size of static arrays. Size must be known at compile time."
  },
  {
    name: "print",
    syntax: "print(value) or print(label, value)",
    description: "Prints a value to the emulator debug console for debugging",
    parameters: [
      { name: "value", type: "int", description: "Value to print (integer)", required: true },
      { name: "label", type: "string", description: "Optional label text", required: false }
    ],
    examples: [
      "score = 100  # Global variable",
      "print(score)              # Prints: 100",
      "print(\"Score:\", score)   # Prints: Score: 100",
      "",
      "def loop():",
      "    x = player_x + 10  # Local variable",
      "    print(\"Player X:\", x)"
    ],
    category: "builtin",
    notes: "For debugging only. Output appears in emulator console, NOT on Vectrex screen. Use PRINT_TEXT() for on-screen text."
  },
  {
    name: "abs",
    syntax: "abs(x)",
    description: "Returns the absolute value of x (removes negative sign)",
    parameters: [
      { name: "x", type: "int", description: "Number to get absolute value of", required: true }
    ],
    examples: [
      "distance = abs(player_x - enemy_x)  # Local variable",
      "speed = abs(velocity)",
      "",
      "if abs(ball_x) > 120:",
      "    # Ball hit edge"
    ],
    category: "builtin",
    notes: "Useful for calculating distances and collision detection."
  },
  {
    name: "min",
    syntax: "min(a, b)",
    description: "Returns the smaller of two values",
    parameters: [
      { name: "a", type: "int", description: "First value", required: true },
      { name: "b", type: "int", description: "Second value", required: true }
    ],
    examples: [
      "x = min(player_x, 100)  # Clamp to maximum 100",
      "lowest_score = min(score1, score2)",
      "",
      "# Clamp value to range",
      "player_x = max(-100, min(player_x, 100))"
    ],
    category: "builtin",
    notes: "Often used with max() for clamping values to ranges."
  },
  {
    name: "max",
    syntax: "max(a, b)",
    description: "Returns the larger of two values",
    parameters: [
      { name: "a", type: "int", description: "First value", required: true },
      { name: "b", type: "int", description: "Second value", required: true }
    ],
    examples: [
      "x = max(player_x, -100)  # Clamp to minimum -100",
      "highest_score = max(score1, score2)",
      "",
      "# Ensure non-negative",
      "health = max(0, player_health)"
    ],
    category: "builtin",
    notes: "Often used with min() for clamping values to ranges."
  },
  {
    name: "DRAW_POLYGON",
    syntax: "DRAW_POLYGON(n_sides, intensity?, x0, y0, x1, y1, ..., xn, yn)",
    description: "Draws a closed polygon with N sides at specified vertices",
    parameters: [
      { name: "n_sides", type: "int", description: "Number of polygon sides (minimum 3)", required: true },
      { name: "intensity", type: "int", description: "Optional beam intensity (0-127, default 95)", required: false },
      { name: "x0, y0, ..., xn, yn", type: "int", description: "N pairs of (x, y) coordinates for vertices", required: true }
    ],
    examples: [
      "# Triangle (3 vertices)",
      "DRAW_POLYGON(3, 127, -15, 20, 15, 20, 0, -20)",
      "",
      "# Square with intensity",
      "DRAW_POLYGON(4, 100, -50, -50, 50, -50, 50, 50, -50, 50)",
      "",
      "# Pentagon (no intensity specified, uses default)",
      "DRAW_POLYGON(5, 0, -50, 30, -10, 50, 40, 20, 30, -50)"
    ],
    category: "drawing",
    notes: "Polygon automatically closes (last vertex connects to first). All vertices drawn with CLR Vec_Misc_Count for proper line continuity. Variable arity function."
  },
  {
    name: "DRAW_CIRCLE",
    syntax: "DRAW_CIRCLE(x_center, y_center, diameter, intensity?)",
    description: "Draws an approximate circle using a 16-sided polygon",
    parameters: [
      { name: "x_center", type: "int", description: "Center X coordinate", required: true },
      { name: "y_center", type: "int", description: "Center Y coordinate", required: true },
      { name: "diameter", type: "int", description: "Circle diameter (width)", required: true },
      { name: "intensity", type: "int", description: "Optional beam intensity (0-127, default 95)", required: false }
    ],
    examples: [
      "# Circle at center with diameter 60",
      "DRAW_CIRCLE(0, 0, 60, 127)",
      "",
      "# Smaller circle at position",
      "DRAW_CIRCLE(50, -30, 40, 80)"
    ],
    category: "drawing",
    notes: "Approximates circle with 16-point polygon. All points calculated from center using trigonometry. Variable arity function."
  },
  {
    name: "DRAW_CIRCLE_SEG",
    syntax: "DRAW_CIRCLE_SEG(n_segments, x_center, y_center, diameter, intensity?)",
    description: "Draws an approximate circle with custom number of segments",
    parameters: [
      { name: "n_segments", type: "int", description: "Number of segments (3-64, typically 8-32)", required: true },
      { name: "x_center", type: "int", description: "Center X coordinate", required: true },
      { name: "y_center", type: "int", description: "Center Y coordinate", required: true },
      { name: "diameter", type: "int", description: "Circle diameter", required: true },
      { name: "intensity", type: "int", description: "Optional beam intensity (0-127, default 95)", required: false }
    ],
    examples: [
      "# Detailed circle with 32 segments",
      "DRAW_CIRCLE_SEG(32, 0, 0, 60, 127)",
      "",
      "# Quick circle with 8 segments (octagon)",
      "DRAW_CIRCLE_SEG(8, -40, 40, 50, 100)"
    ],
    category: "drawing",
    notes: "More segments = smoother circle but slower. Fewer segments = faster but more angular. Clamped to 3-64 segments. Variable arity function."
  },
  {
    name: "WAIT_RECAL",
    syntax: "WAIT_RECAL()",
    description: "Waits for screen refresh and recalibrates the vector beam (50 FPS sync)",
    parameters: [],
    examples: [
      "def loop():",
      "    WAIT_RECAL()  # Sync with screen refresh (~20ms at 50 FPS)",
      "    SET_INTENSITY(127)",
      "    DRAW_VECTOR(\"player\", x, y)",
      "    # AUDIO_UPDATE auto-injected here by compiler"
    ],
    category: "control",
    vectrexAddress: "0xF192 (Wait_Recal)",
    notes: "MANDATORY at start of loop(). Synchronizes execution with screen refresh (20ms). Auto-injected by compiler if missing. AUDIO_UPDATE auto-injected immediately after."
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
- **ARRAYS**: Static fixed-size arrays supported!
  - **MUTABLE arrays** (stored in RAM):
    var enemies = [0, 0, 0, 0, 0]  # Array of 5 integers in RAM
    let x = enemies[0]              # Access element
    enemies[2] = 100                # Modify element (writes to RAM)
    for enemy in enemies:           # Iterate
    let count = len(enemies)        # Get length
  
  - **CONST arrays** (immutable, stored in ROM only - 2025-12-19):
    const player_coords = [10, 20, 30]  # Read-only in ROM (no RAM allocation)
    const health_pool = [100, 100, 100] # No initialization overhead
    # ⚠️ NOTE: Direct const array usage/indexing still in development
    # Currently emitted to ROM correctly, indexing support coming soon

### Structs (User-Defined Types):
- **Define**: struct Name: followed by field definitions
- **Fields**: Indented, name: type (only 'int' supported currently)
- **Instantiate**: variable = StructName() or with constructor args
- **Constructors**: Define **def __init__(param1, param2, ...):** to initialize fields
  - Use **self.field = param** inside constructor to set initial values
  - Called automatically when creating instance: **Entity(x, y, dx, dy)**
  - Constructor params passed as ARG1, ARG2, etc. (ARG0 is struct pointer)
- **Access**: variable.field_name (read or write)
- **Memory**: Optimized - structs stored directly on stack, 2 bytes per field
- **Methods**: Structs can have methods (functions inside struct definition)
  - Use implicit **self** keyword to access own fields (self.x, self.y)
  - Call methods with: **object.method_name(args)**
  - ✅ **Full read/write support**: Methods can read AND write self.field = value
  - True OOP behavior: Objects modify their own internal state
  - Pattern: Encapsulate behavior + data in methods
- **Example**:
  struct Entity:
      x: int
      y: int
      dx: int
      dy: int
      
      def __init__(init_x, init_y, init_dx, init_dy):
          # Constructor: initialize fields with parameters
          self.x = init_x
          self.y = init_y
          self.dx = init_dx
          self.dy = init_dy
      
      def update_position():
          # Note: NO explicit 'self' parameter, it's implicit
          # ✅ CAN read AND write self.field
          self.x = self.x + self.dx    # Modifies internal state
          self.y = self.y + self.dy    # State persists after method returns
      
      def handle_bounce(min_x, max_x, speed):
          # Complex logic with conditional writes
          if self.x < min_x:
              self.dx = speed      # Bounce right
          if self.x > max_x:
              self.dx = -speed     # Bounce left
      
      def distance_from_origin():
          # Read-only methods still work
          dist_sq = self.x * self.x + self.y * self.y
          return dist_sq / 10
  
  def loop():
      # Create with constructor - fields initialized automatically
      entity = Entity(100, 50, -2, 0)
      # Call methods - object modifies itself:
      entity.update_position()           # x and dx updated internally
      entity.handle_bounce(-100, 100, 2) # dx changed if out of bounds

**Technical Implementation**:
- Constructors generate **STRUCTNAME_INIT** function, called during instantiation
- Constructor receives struct pointer in ARG0, params in ARG1-ARG4 (max 4 params)
- Method calls on local structs use **LEAX offset,S** to compute struct address
- VAR_ARG0 receives pointer to struct, methods access fields via offset
- Self field access: **LDX VAR_ARG0; LDD offset,X** (read) or **STD offset,X** (write)
- Type tracking: Constructor calls auto-tracked for method resolution
- Global structs not yet supported - use globals + local struct pattern
- Pattern for persistence: global vars → local struct → methods → write back to globals

### Control Flow:
- if/elif/else - Conditional branching
- while - While loop
- for...in - For loop over range or array
- break - Exit loop early
- continue - Skip to next iteration
- pass - No-op placeholder (empty function/block)

### Required Functions:
- def main(): - Initialization (runs once)
- def loop(): - Game loop (60 FPS, WAIT_RECAL auto-added by compiler)

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

### META Fields (ROM Header):
**SYNTAX: Use assignment, NOT function call**
✅ CORRECT:
  META TITLE = "MY GAME"       # Required (UPPERCASE only)
  META COPYRIGHT = "g GCE 1982" # Optional
  META MUSIC = 1                # Optional (0-9 for BIOS songs)

❌ WRONG (DO NOT USE):
  META(title="Game", author="Name", year=2025)  # Invalid syntax
  META AUTHOR = "..."    # Field doesn't exist
  META DESCRIPTION = "..." # Field doesn't exist
  META YEAR = 2025       # Field doesn't exist

Only 3 META fields exist: TITLE, COPYRIGHT, MUSIC

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

## Const Arrays (ROM-Only Storage - 2025-12-19)
Immutable arrays stored in ROM with zero memory corruption risk:

### Number Arrays
**Syntax**:
\`\`\`python
const player_x = [10, 20, 30]    # Array stored in ROM only
const player_y = [40, 50, 60]    # No RAM allocation, no initialization
current_player = 0               # Regular mutable variable (RAM)
\`\`\`

**Benefits**:
- ✅ Zero RAM overhead (stored in ROM)
- ✅ No initialization code (no startup LDX/STX)
- ✅ Stable variable offsets (no memory corruption from shifting arrays)
- ✅ Read-only, immutable data

### String Arrays (NEW - 2025-12-27)
**Syntax**:
\`\`\`python
const location_names = ["MOUNT FUJI - JAPAN", "PARIS - FRANCE", "NEW YORK - USA"]

def loop():
    current_location = 0
    # Indexing returns pointer to string (not the string itself)
    name_ptr = location_names[current_location]
    # Use directly with PRINT_TEXT
    PRINT_TEXT(-70, -120, location_names[current_location])
\`\`\`

**Implementation Details**:
- Each string stored as FCC (Form Constant Character) with $80 terminator
- Pointer table (FDB) with addresses to each string
- Indexing returns POINTER (not value), perfect for PRINT_TEXT
- Zero RAM overhead - all data in ROM

**Generated Assembly**:
\`\`\`asm
; Individual strings
CONST_ARRAY_0_STR_0:
    FCC "MOUNT FUJI - JAPAN"
    FCB $80   ; Vectrex string terminator

CONST_ARRAY_0_STR_1:
    FCC "PARIS - FRANCE"
    FCB $80

; Pointer table
CONST_ARRAY_0:
    FDB CONST_ARRAY_0_STR_0  ; Address of first string
    FDB CONST_ARRAY_0_STR_1  ; Address of second string
\`\`\`

**Key Differences from regular arrays**:
| Feature | Regular Array | Const Number Array | Const String Array |
|---------|--|--|--|
| Storage | RAM | ROM | ROM |
| RAM allocation | Yes (+2 bytes per array) | No | No |
| Mutable | Yes | No | No |
| Element size | 2 bytes | 2 bytes | Variable (string length) |
| Indexing returns | Value | Value | Pointer |
| Usage | Modifiable data | Fixed lookup tables | Text/labels |

**Problem Solved**:
Previously, adding/removing array variables caused variable offsets to shift,
corrupting adjacent memory. Const arrays eliminate this by storing data in ROM,
keeping RAM allocation stable and predictable.

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

### COMMUNICATION STYLE:
**BE CONCISE** - Execute tools silently without announcing them:
❌ BAD: "I'll use editor/write_document to create the file"
❌ BAD: "Now calling compiler/build_and_run"
✅ GOOD: Just execute the tools and show the result
✅ GOOD: Brief confirmation like "✅ Created paddle.vec" after execution

Do NOT explain which tools you're using unless the user asks. Just do the work.

### CRITICAL: MCP Tool Usage Workflow

**ALWAYS follow this sequence when modifying code:**

1. **Modify file** using tool: **editor/write_document**
   - Pass complete file content (not partial edits)
   - Provides uri (file path) and content parameters

2. **SAVE the file** immediately using tool: **editor/save_document**
   - CRITICAL: Must save to disk before compilation
   - Compiler reads from disk, not editor state
   - Pass uri parameter (same as write_document)

3. **Compile the project** using tool: **compiler/build_and_run**
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
   - Use tool **editor/read_document** to read source if needed
   - Fix errors and repeat from step 1

**Example workflow:**
\`\`\`
1. editor/write_document (uri: "main.vpy", content: "...")
2. editor/save_document (uri: "main.vpy")
3. compiler/build_and_run ()
4. Check result.success:
   - true → Program running in emulator
   - false → Read result.errors, fix code, repeat
\`\`\`

### Key MCP Tools:

**CRITICAL: Tool names use SLASH notation, NOT underscores**

**Editor Tools:**
- **editor/write_document** - CREATE or UPDATE file (auto-opens in editor)
- **editor/save_document** - SAVE file to disk (REQUIRED before compilation)
- **editor/read_document** - Read file content (file MUST be open first)
- **editor/list_documents** - List all open documents

**Compiler Tools:**
- **compiler/build_and_run** - USE THIS: Compile + Run in one step
- **compiler/build** - Only compile (no auto-run)
- **compiler/get_errors** - Get current diagnostics from editor

**Emulator Tools:**
- **emulator/run** - Load ROM file into emulator
- **emulator/stop** - Stop emulation
- **emulator/get_state** - Get PC, registers, cycles, FPS

**Project Tools:**
- **project/get_structure** - List ALL project files (USE THIS to verify assets exist)
- **project/create_vector** - Create .vec file (validates JSON format)
- **project/create_music** - Create .vmus file (validates JSON format)
- **project/read_file** - Read any project file (uses RELATIVE paths from project root)
- **project/write_file** - Write any project file (uses RELATIVE paths from project root)

**CRITICAL Asset Workflow:**
1. ALWAYS call **project/get_structure** FIRST to see available assets
2. Check 'assets/vectors/*.vec' and 'assets/music/*.vmus' in response
3. ONLY use asset names that PHYSICALLY exist in the project
4. Example: If you see 'rocket_base.vec', use DRAW_VECTOR("rocket_base")
5. NEVER assume generic names like "player", "enemy", "ship_part1" exist
6. If asset doesn't exist: Ask user or create with project/create_vector

**BEST PRACTICE: Prefer Assets Over Manual Drawing**
✅ **USE ASSETS (.vec files)** as the DEFAULT approach for all game objects:
- Ships, enemies, bullets, UI elements, text → Create as .vec assets
- Benefits: Reusable, efficient, separates art from logic, easier to edit
- Example: Create 'player.vec' and use DRAW_VECTOR("player") instead of multiple DRAW_LINE calls

❌ **AVOID manual DRAW_LINE/MOVE** except for:
- Debug visualization (showing hitboxes, paths)
- Dynamic effects (lasers, explosions with procedural animation)
- Simple UI elements that change frequently

**MANDATORY .vec Creation Workflow:**
1. ALWAYS use **project/create_vector** tool with exact JSON format below
2. Copy template EXACTLY - change only: name, points coordinates
3. Verify "paths" (NOT "vectors"), "points" (NOT "type":"line"), version "1.0" (string)
4. Use DRAW_VECTOR("name") in code

**TEMPLATE - Copy this EXACTLY (remove outer quotes when using):**
"{\\"version\\":\\"1.0\\",\\"name\\":\\"object_name\\",\\"canvas\\":{\\"width\\":256,\\"height\\":256,\\"origin\\":\\"center\\"},\\"layers\\":[{\\"name\\":\\"default\\",\\"visible\\":true,\\"paths\\":[{\\"name\\":\\"path1\\",\\"intensity\\":127,\\"closed\\":true,\\"points\\":[{\\"x\\":0,\\"y\\":20},{\\"x\\":-15,\\"y\\":-10},{\\"x\\":15,\\"y\\":-10}]}]}]}"

**Formatted for readability (same content):**
- version: "1.0" (string)
- name: "object_name" (change this)
- canvas: {width: 256, height: 256, origin: "center"}
- layers[0].paths[0].points: Array of {x, y} coordinates (change these)
- closed: true for polygons, false for lines

### Common Mistakes to AVOID:

❌ Using **editor/read_document** on unopened files → Use **editor/write_document** to create
❌ Forgetting to save after **editor/write_document** → ALWAYS use **editor/save_document** after write
❌ Compiling without saving → Compiler reads disk, not editor state
❌ Using **editor/replace_range** for new files → Requires file open first, use **editor/write_document**
❌ Calling **compiler/build** then **emulator/run** separately → Use **compiler/build_and_run** instead
❌ Not checking \`result.success\` after compilation → Must validate before assuming success
❌ Inventing tool names → ONLY use documented tools with slash notation
❌ Passing undefined parameters → Always provide required parameters
❌ Using underscores in tool names (editor_write_document) → WRONG, use slashes (editor/write_document)
❌ Using wrong paths for project/read_file ("main.vpy") → Use relative path ("src/main.vpy")
❌ Treating tool names as file paths → "project/create_vector" is TOOL NAME, NOT path
❌ **INVENTING asset names** ("player", "ship_part1") → ALWAYS check project/get_structure FIRST
❌ Assuming generic names exist → Verify with project/get_structure before using DRAW_VECTOR/PLAY_MUSIC
❌ **INVENTING .vec format** (using "vectors", "type": "line", x1/y1/x2/y2) → MUST use exact "paths"/"points" format
❌ Using version: 1 (number) in .vec → MUST be "version": "1.0" (string)

### MCP Tools Reference

**EDITOR TOOLS (7 tools):**

1. **editor/list_documents** - List all open documents
   - No parameters required

2. **editor/read_document** - Read content of OPEN document
   - \`uri\`: string (required) - Document URI (must be currently open in editor)
   - ⚠️ ONLY works for files already open - for new files use editor/write_document

3. **editor/write_document** - Create OR update document (auto-opens if new)
   - \`uri\`: string (required) - File path/name (e.g., "main.vpy", "src/game.vpy")
   - \`content\`: string (required) - Complete file content
   - Auto-detects language (.vpy → VPy, .vec/.vmus/.json → JSON)

4. **editor/save_document** - Save to disk and mark clean
   - \`uri\`: string (required) - File URI to save (must be open)
   - ⚠️ CRITICAL: Use after editor/write_document BEFORE compilation

5. **editor/get_diagnostics** - Get compilation/lint errors
   - \`uri\`: string (optional) - All diagnostics if omitted

6. **editor/replace_range** - Replace specific LINES (not offsets)
   - \`uri\`: string (required) - Document URI (must be open)
   - \`startLine\`: number (required) - Start line (1-indexed)
   - \`startColumn\`: number (required) - Start column (1-indexed)
   - \`endLine\`: number (required) - End line (1-indexed)
   - \`endColumn\`: number (required) - End column (1-indexed)
   - \`newText\`: string (required) - Replacement text

7. **editor/insert_at** - Insert text at position
   - \`uri\`: string, \`line\`: number, \`column\`: number, \`text\`: string

8. **editor/delete_range** - Delete text in range
   - \`uri\`: string, \`startLine\`, \`startColumn\`, \`endLine\`, \`endColumn\`

**COMPILER TOOLS (3 tools):**

1. **compiler/build** - Build current project (F7 equivalent)
   - No parameters - uses current project config

2. **compiler/get_errors** - Get latest compilation errors
   - No parameters

3. **compiler/build_and_run** - Build + run in emulator
   - \`breakOnEntry\`: boolean (optional) - Pause at entry point

**EMULATOR TOOLS (3 tools):**

1. **emulator/run** - Run compiled ROM
   - \`romPath\`: string (required) - Path to .bin file
   - \`breakOnEntry\`: boolean (optional)

2. **emulator/get_state** - Get current state
   - Returns: PC, registers, cycles

3. **emulator/stop** - Stop execution
   - No parameters

**MEMORY TOOLS (3 tools):**

1. **memory/dump** - Get RAM snapshot (hex dump)
   - \`start\`: number (optional, default: 0xC800 = RAM start)
   - \`end\`: number (optional, default: 0xCFFF = RAM end)
   - \`format\`: "hex" | "decimal" (optional, default: "hex")

2. **memory/list_variables** - Get all variables from PDB
   - No parameters
   - Returns: name, address, size, type for each variable
   - Sorted by size (largest first) for RAM optimization analysis

3. **memory/read_variable** - Read current value
   - \`name\`: string (required) - Variable name without VAR_ prefix
   - Example: "player_x" not "VAR_PLAYER_X"

**DEBUGGER TOOLS (2 tools):**

1. **debugger/add_breakpoint** - Add breakpoint
   - \`uri\`: string (required), \`line\`: number (required, 1-indexed)

2. **debugger/get_callstack** - Get call stack
   - No parameters

**PROJECT TOOLS (8 tools):**

1. **project/get_structure** - Get complete project structure
   - No parameters

2. **project/read_file** - Read any project file
   - \`path\`: string (required) - RELATIVE path from project root
   - Example: "src/main.vpy" NOT "main.vpy"

3. **project/write_file** - Write/update any file (auto-opens in editor)
   - \`path\`: string (required) - Relative path (e.g., "src/game.vpy")
   - \`content\`: string (required) - Complete file content

4. **project/close** - Close current project
   - No parameters

5. **project/open** - Open existing project
   - \`path\`: string (required) - Full path to .vpyproj file

6. **project/create** - Create new project
   - \`name\`: string (required) - Project name
   - \`path\`: string (optional) - Directory path (shows dialog if omitted)

7. **project/create_vector** - Create .vec file with JSON validation
   - \`name\`: string (required) - Filename WITHOUT .vec extension
   - \`content\`: string (optional) - Valid JSON or empty for template
   - Format: \`{"version":"1.0","name":"shape","canvas":{"width":256,"height":256,"origin":"center"},"layers":[{"name":"default","visible":true,"paths":[{"name":"path1","intensity":127,"closed":false,"points":[{"x":0,"y":0},{"x":10,"y":10}]}]}]}\`
   - ⚠️ NEVER use: "vectors", "type":"line", x1/y1/x2/y2, version as number
   - ✅ ALWAYS use: "paths", "points" array, "version":"1.0" (string)

8. **project/create_music** - Create .vmus file with JSON validation
   - \`name\`: string (required) - Filename WITHOUT .vmus extension
   - \`content\`: string (optional) - Valid JSON or empty for template
   - Format: \`{"version":"1.0","name":"Song","author":"Composer","tempo":120,"ticksPerBeat":24,"totalTicks":384,"notes":[{"id":"note1","note":60,"start":0,"duration":48,"velocity":12,"channel":0}],"noise":[{"id":"noise1","start":0,"duration":24,"period":15,"channels":1}],"loopStart":0,"loopEnd":384}\`
   - Note fields: \`note\` (MIDI 0-127, 60=C4), \`velocity\` (0-15 volume), \`channel\` (0=A, 1=B, 2=C)
   - Noise fields: \`period\` (0-31, lower=higher pitch), \`channels\` (bitmask: 1=A, 2=B, 4=C, 7=all)

## IDE Diagnostics and Code Actions

The IDE provides real-time code analysis and Quick Fixes:

### Variable Usage Analysis

**Automatic Detection:**
- **Unused Variables**: Variables declared but never read → WARNING (yellow underline)
- **Const Suggestions**: Variables initialized once and never modified → HINT (can save 2 bytes RAM)

**Diagnostics appear during compilation:**
- Yellow underline on variable name
- Hover to see diagnostic message
- Lightbulb icon (💡) for Quick Fixes

### Quick Fixes (Code Actions)

**Available Actions:**

1. **Convert to const** (for variables that never change)
   - Saves 2 bytes RAM per variable
   - Automatically adds \`const\` keyword
   - Example: \`player_speed = 3\` → \`const player_speed = 3\`

2. **Remove unused variable** (for variables declared but never used)
   - Cleans up dead code
   - Deletes entire line
   - Example: Removes \`temp_x = 0\` if never read

**How to use:**
1. Compile your code (diagnostics appear automatically)
2. Hover over yellow-underlined variable
3. Click lightbulb icon (💡) or press \`Cmd+.\` (macOS) / \`Ctrl+.\` (Windows/Linux)
4. Select desired Quick Fix
5. Changes apply immediately

**Examples:**

\`\`\`vpy
# Before - Compiler suggests optimizations:
num_locations = 17        # ⚠️ Never changes → suggest const
hook_max_y = 40           # ⚠️ Never changes → suggest const
player_speed = 2          # ⚠️ Never changes → suggest const
temp_value = 0            # ⚠️ Declared but never used

# After applying Quick Fixes:
const num_locations = 17  # ✅ Saves 2 bytes RAM
const hook_max_y = 40     # ✅ Saves 2 bytes RAM
const player_speed = 2    # ✅ Saves 2 bytes RAM
# temp_value removed        # ✅ Dead code eliminated
\`\`\`

**Benefits:**
- Automatic RAM optimization suggestions
- Clean code (removes unused variables)
- Real-time feedback during development
- One-click fixes (no manual editing)
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
