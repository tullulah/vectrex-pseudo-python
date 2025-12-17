# VPy Asset System - Embedded Graphics and Music

VPy includes a powerful asset system that allows you to embed vector graphics and music directly in your game ROM.

## 📦 How Assets Work:
1. **Auto-discovery**: Place .vec and .vmus files in `assets/vectors/` and `assets/music/` directories
2. **Compile-time embedding**: Assets are automatically discovered and embedded in ROM during compilation (Phase 0)
3. **Reference by name**: Use `DRAW_VECTOR("name", x, y)` and `PLAY_MUSIC("name")` in your code
4. **No manual loading**: Everything is compiled into the final binary

## 🎯 Vector Graphics (.vec files)

Vector graphics are stored as JSON files in `assets/vectors/*.vec`:

### File format:
```json
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
```

### Key fields:
- `name`: Asset identifier (used in `DRAW_VECTOR("player")`)
- `intensity`: Brightness (0-255, higher = brighter)
- `closed`: true = polygon, false = open line
- `points`: Coordinates in range -127 to +127 (Vectrex screen space)

### Usage in code:
```vpy
def loop():
    WAIT_RECAL()
    SET_INTENSITY(255)
    MOVE(-50, 0)
    DRAW_VECTOR("player", 0, -80)  # Draws the vector asset at position
```

## 🎵 Music Assets (.vmus files)

Music is stored as JSON files in `assets/music/*.vmus`:

### File format:
```json
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
```

### Key fields:
- `note`: MIDI note number (60=C4, 69=A4 440Hz, 72=C5)
- `velocity`: Volume (0-15, where 15=maximum) - Used for both notes and noise
- `channel`: PSG channel (0=A, 1=B, 2=C) - Only for notes
- `period`: Noise period (0-31, lower=higher pitch)
- `channels`: Noise channel mask (1=A, 2=B, 4=C, 7=all) - Only for noise

### Usage in code:
```vpy
def main():
    PLAY_MUSIC("theme")  # Start background music

def loop():
    # Music plays automatically in background
    # ... game logic ...
```

## 📁 Project Structure with Assets:
```
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
```

## 🔧 Asset System Technical Details:
- **Discovery**: Automatic at compile time (Phase 0)
- **Embedding**: Data section in ROM (Phase 5)
- **Format**: FCB assembly directives for vector data
- **Access**: JSR to BIOS Draw_VLc for vectors
- **Music**: Placeholder PSG player (full implementation in progress)
- **Compilation**: Native M6809 assembler (no lwasm needed)

## ✅ Asset Best Practices:
1. Keep vector sprites simple (fewer points = faster drawing)
2. Use appropriate intensity values (127-255 for visible graphics)
3. Place commonly-used sprites first in assets/ for faster access
4. Keep music files under ~80-100 notes for optimal size
5. Use loops in music (loopStart/loopEnd) for repetition instead of duplicating notes
6. Test assets in emulator before deploying to real hardware
