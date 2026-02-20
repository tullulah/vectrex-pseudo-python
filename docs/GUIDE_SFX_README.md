# VPy Sound System — Resource Map

## Overview

VPy has a complete audio system based on:
- **PLAY_MUSIC()** — PSG music (channels A+B)
- **PLAY_SFX()** — AYFX effects (channel C)
- Audio update is **automatically injected** by the compiler each frame — do not call it manually.

Use this document to find exactly what you need.

---

## What Do I Need?

### "I want to create a new SFX from scratch"
→ Start with: **[GUIDE_SFX_CREATION.md](GUIDE_SFX_CREATION.md)**
1. Read Section 3 (Detailed Parameters)
2. Copy a recipe from Section 4
3. Modify parameters as needed
4. Save to `assets/sfx/my_sound.vsfx`

---

### "I want to understand how existing SFX work"
→ Start with: **[GUIDE_SFX_EXAMPLES.md](GUIDE_SFX_EXAMPLES.md)**
1. Read Section 2 (Jump analysis — simple)
2. Read Section 3 (Jump vs Coin comparison)
3. Read Section 4 (Explosion analysis — complex)
4. Try to recreate them in the SFX Editor

---

### "I want to use Arpeggio for musical chords"
→ See: **[GUIDE_SFX_CREATION.md](GUIDE_SFX_CREATION.md) — Section 3.5**
- List of musical chords (semitones)
- Examples in existing presets
- Visual arpeggio editor in the SFX Editor

Quick example:
```json
"modulation": {
  "arpeggio": true,
  "arpeggio_notes": [0, 4, 7],
  "arpeggio_speed": 50
}
```

---

### "I need a quick parameter reference"
→ See: **[GUIDE_SFX_CREATION.md](GUIDE_SFX_CREATION.md) — Sections 2–3**

Quick reference tables:
- Oscillator (frequency, channel, duty)
- Envelope (ADSR)
- Pitch Sweep (multipliers)
- Noise (period, volume, decay)
- Arpeggio (predefined chords)

---

### "I want to see examples of common SFX"
→ See: **[GUIDE_SFX_CREATION.md](GUIDE_SFX_CREATION.md) — Section 4**

6 complete recipes:
1. **Laser** — high tone that drops fast
2. **Coin** — simple, cheerful chord
3. **Jump** — note that rises
4. **Explosion** — complex with noise
5. **Hit** — short impact
6. **Powerup** — ascending chord

---

### "How do I use SFX in my VPy code?"

```python
def main():
    PLAY_SFX("jump")  # play on startup

def loop():
    if J1_BUTTON_1():
        PLAY_SFX("coin")

    DRAW_VECTOR("player", player_x, player_y)
    # Audio updates automatically each frame
```

---

### "The SFX Editor isn't showing what I want"
Location: `ide/frontend/src/components/SFXEditor.tsx`

Available features:
- ✅ Oscillator (frequency, channel)
- ✅ Envelope (ADSR)
- ✅ Pitch Sweep (curve)
- ✅ Noise (white noise)
- ✅ Arpeggio (chords)
- ✅ Real-time visualization

How to use:
1. Open the project
2. Find `assets/sfx/something.vsfx`
3. Double-click to open in SFX Editor
4. Press Play to hear it
5. Adjust sliders
6. Save with Ctrl+S

---

## Documentation Structure

```
VPy Sound System
├── [GUIDE_SFX_CREATION.md]
│   ├── 1. Introduction (what is AYFX)
│   ├── 2. Base JSON structure
│   ├── 3. Detailed parameters (tables)
│   ├── 4. Common recipes (6 examples)
│   ├── 5. Manual creation workflow
│   ├── 6. Design tips
│   ├── 7. Limitations
│   └── 8. External inspiration
│
├── [GUIDE_SFX_EXAMPLES.md]
│   ├── 1. SFX file locations
│   ├── 2. Jump analysis (simple)
│   ├── 3. Jump vs Coin comparison
│   ├── 4. Explosion analysis (complex)
│   ├── 5. Timeline visualization
│   ├── 6. Custom template
│   └── 7. Checklist and advanced tips
│
└── [SFXEditor.tsx]
    └── Interactive visual editor
        ├── Sliders for all parameters
        ├── Envelope visualization canvas
        ├── Preset buttons
        ├── Arpeggio editor
        └── Play button for preview
```

---

## Quick Start Paths

### Path 1: "I want a quick laser SFX" (5 min)
```
1. Open SFX Editor
2. Press the "laser" preset button
3. Press "Play" to hear it
4. Done — you have a laser
```

### Path 2: "I want to understand everything" (60 min)
```
1. Read GUIDE_SFX_CREATION.md (20 min)
2. Read GUIDE_SFX_EXAMPLES.md (25 min)
3. Open SFX Editor (15 min)
   - Load each preset
   - Press Play
   - Change parameters
   - Listen to the differences
```

### Path 3: "I want to create my own unique sound" (30 min)
```
1. Pick an inspiration (GUIDE_SFX_EXAMPLES.md — Section 5)
2. Copy a base recipe (GUIDE_SFX_CREATION.md — Section 4)
3. Create assets/sfx/my_sound.vsfx
4. Open in SFX Editor
5. Adjust parameters
6. Press Play (iterate until satisfied)
7. Save
8. Use in code: PLAY_SFX("my_sound")
```

---

## Key Concepts

### Envelope (ADSR)
**What it is**: The volume curve of the sound

```
Attack (A)   = fade-in time (0–500ms)
Decay (D)    = drop to sustain (0–500ms)
Sustain (S)  = held volume (0–15)
Release (R)  = final fade-out (0–1000ms)
Peak         = maximum volume (1–15)
```

**Practical effect:**
- A=0: Starts loud (snappy)
- A=100: Starts quiet (fade-in)
- R=50: Short (dry sound)
- R=300: Long (natural sound)

---

### Pitch Sweep
**What it is**: Frequency change during the effect

```
start_mult = 0.5  → starts at half pitch
end_mult = 2.0    → ends at double pitch
curve = 1         → smooth interpolation

Result: Sound that RISES (like a powerup "POP")
```

---

### Arpeggio (Chords)
**What it is**: Plays multiple notes in sequence

```
[0, 4, 7]      → C-E-G (major chord)
[0, 12]        → C one octave up
[0, 3, 7, 10]  → C minor 7

speed: 50ms    → how fast it cycles between notes
```

---

### Noise (White Noise)
**What it is**: Unpitched sound (noise)

```
period: 8      → high-pitched noise
period: 20     → low-pitched noise
volume: 15     → very loud
decay: 350ms   → fades slowly
```

**Use for**: explosions, impacts, friction

---

## Troubleshooting

### "SFX doesn't play in the game"
1. Check that `PLAY_SFX("name")` matches the filename exactly
2. The file must exist at `assets/sfx/name.vsfx`
3. Recompile the project
4. Test in the emulator

### "SFX sounds different in the SFX Editor vs the game"
- The editor uses Web Audio API (approximation)
- The game uses real PSG hardware (Vectrex)
- Minor differences are normal

### "How do I edit an existing SFX?"
1. Open `assets/sfx/name.vsfx`
2. Edit the JSON directly, or
3. Double-click to open in SFX Editor
4. Adjust with sliders
5. Save

---

## Current Status

| Feature | Status |
|---------|--------|
| Basic SFX | ✅ Complete |
| Oscillator | ✅ Complete |
| Envelope | ✅ Complete |
| Pitch Sweep | ✅ Complete |
| Noise | ✅ Complete |
| Arpeggio | ✅ Complete |
| Visual Editor | ✅ Complete |
| Presets | ✅ 7 presets (laser, coin, jump, explosion, hit, powerup, blip) |
