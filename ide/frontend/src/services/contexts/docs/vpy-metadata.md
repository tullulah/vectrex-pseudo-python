# VPy Metadata Fields (META)

**CRITICAL: Correct syntax is `META FIELD = value` (assignment), NOT `META(field=value)` (function call)**

VPy supports exactly 3 META fields that define ROM header information:

```vpy
META TITLE = "MY GAME"          # Game title (REQUIRED)
META COPYRIGHT = "g GCE 1982"   # Copyright string (optional)
META MUSIC = 1                  # BIOS music number 0-9 (optional)

# Your VPy code starts here
def main():
    # Initialization

def loop():
    # Game logic
    SET_INTENSITY(255)
    MOVE(0, 0)
    PRINT_TEXT(0, 50, "HELLO VECTREX")
```

**❌ WRONG - DO NOT USE THIS:**
```vpy
META(title="Game", author="Name", year=2025)  # INVALID SYNTAX
```

## META Field Reference (3 fields only):

### TITLE
- Game title (required)
- Example: `META TITLE = "SPACE SHOOTER"`
- **CRITICAL**: Must be in UPPERCASE letters only
- **Max length**: 24 characters
- **Valid characters**: Letters, numbers, spaces only (special chars cleaned)
- **Used for**: ROM header, game identification

### COPYRIGHT
- Copyright string (optional)
- Example: `META COPYRIGHT = "g GCE 1982"`
- **Default**: "g GCE 1982"
- **Used for**: First line display in ROM header

### MUSIC
- Built-in BIOS music NUMBER for title screen (optional)
- Examples: `META MUSIC = 0` (no music), `META MUSIC = 1` (Minestorm song 1), `META MUSIC = 2` etc.
- **Default**: 1 (Minestorm song 1) - use 0 for silence
- **⚠️ IMPORTANT**: This is NOT for your custom .vmus files - use `PLAY_MUSIC("name")` function in code for that
- **Range**: 0 to 9 (numbers only, built-in songs)
- **Used for**: Title screen background music (built-in songs only)

## Important META Rules:
- **SYNTAX**: Use `META FIELD = value` (assignment), NOT `META(field=value)` (function call)
- **Only 3 META fields supported**: TITLE, COPYRIGHT, MUSIC
- **NO other fields exist**: author, description, year are NOT valid
- **TITLE must be UPPERCASE**: Lowercase reserved for special characters
- **TITLE is required** for proper ROM generation
- **Other fields are optional** with reasonable defaults
- **ROM dimensions fixed**: Height/width/coords ($F8,$50,$20,$AA) cannot be changed

## Common Mistakes:
❌ `META(title="Game", author="Name")` - WRONG syntax (function call)
❌ `META AUTHOR = "Name"` - Field doesn't exist
❌ `META DESCRIPTION = "..."` - Field doesn't exist
❌ `META YEAR = 2025` - Field doesn't exist
✅ `META TITLE = "MY GAME"` - CORRECT syntax
✅ `META COPYRIGHT = "g GCE 1982"` - CORRECT syntax
✅ `META MUSIC = 1` - CORRECT syntax

## Correct Project Examples:

### Simple Game:
```vpy
META TITLE = "SQUARE DEMO"
META COPYRIGHT = "g GCE 1982"
META MUSIC = 0

def main():
    # Initialize once
    SET_INTENSITY(255)

def loop():
    # Draw every frame
    WAIT_RECAL()
    MOVE(-25, -25)
    DRAW_TO(25, -25)
    DRAW_TO(25, 25)
    DRAW_TO(-25, 25)
    DRAW_TO(-25, -25)
```

### Animation with Music:
```vpy
META TITLE = "ROTATING LINE"
META COPYRIGHT = "g GCE 1982"
META MUSIC = 1

# CRITICAL: Variables must be declared in loop() where they are used
# Functions have separate scopes - main() variables NOT accessible in loop()
var x = -30
var direction = 1

def main():
    # Initialize once (intensity, etc.)
    SET_INTENSITY(200)

def loop():
    # Animation runs automatically every frame
    WAIT_RECAL()
    MOVE(x, 0)
    DRAW_TO(x + 30, 0)
    
    # Update position
    x = x + direction
    if x > 30:
        direction = -1
    if x < -30:
        direction = 1
```

### Minimal Example:
```vpy
META TITLE = "HELLO WORLD"

def main():
    # Minimal initialization
    SET_INTENSITY(255)

def loop():
    # Minimal code - other META fields use defaults
    WAIT_RECAL()
    MOVE(0, 0)
    PRINT_TEXT(0, 0, "HELLO")
```

## META Fields Usage in IDE:
- **ROM Header Generation**: META fields directly affect Vectrex ROM header
- **Title Display**: TITLE appears in game selection and ROM info
- **Copyright Notice**: COPYRIGHT shown in ROM header first line
- **Music Integration**: MUSIC controls background audio from BIOS
