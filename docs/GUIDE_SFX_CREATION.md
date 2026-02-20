# Complete Guide to Creating SFX (Sound Effects)

## 1. Introduction: The AYFX System

VPy uses **AYFX** (a parametric format by Richard Chadd) for sound effects:
- **Format**: JSON
- **Location**: `assets/sfx/*.vsfx` (e.g. `assets/sfx/jump.vsfx`)
- **Channel**: PSG Channel C (registers 4/5, 6, 7, 10)
- **Timing**: Frame-based (updated every frame, ~20ms at 50 FPS)

## 2. Base SFX Structure

```json
{
  "version": "1.0",
  "name": "jump",
  "category": "jump",
  "duration_ms": 180,

  "oscillator": {
    "frequency": 330,
    "channel": 0,
    "duty": 50
  },

  "envelope": {
    "attack": 0,
    "decay": 30,
    "sustain": 8,
    "release": 100,
    "peak": 14
  },

  "pitch": {
    "enabled": true,
    "start_mult": 0.6,
    "end_mult": 1.3,
    "curve": 1
  },

  "noise": {
    "enabled": false,
    "period": 15,
    "volume": 12,
    "decay_ms": 100
  },

  "modulation": {
    "arpeggio": false,
    "arpeggio_notes": [],
    "arpeggio_speed": 50,
    "vibrato": false,
    "vibrato_depth": 0,
    "vibrato_speed": 8
  }
}
```

## 3. Detailed Parameters

### 3.1 Oscillator (Tone Generator)

| Field | Range | Description | Examples |
|-------|-------|-------------|---------|
| `frequency` | 55–1760 Hz | Base tone frequency | 110=A2, 220=A3, 440=A4, 880=A5 |
| `channel` | 0–2 | PSG channel (A/B/C) | 0=Channel A, 1=Channel B, 2=Channel C |
| `duty` | 0–100 | Waveform shape (% pulse width) | 50=pure square wave, 25=thinner |

**Common MIDI Frequencies:**
```
C3=131Hz    E3=165Hz    G3=196Hz    C4=262Hz
D3=147Hz    F3=175Hz    A3=220Hz    D4=294Hz
E3=165Hz    G3=196Hz    B3=247Hz    E4=330Hz
F3=175Hz    A3=220Hz    C4=262Hz    F4=349Hz
G3=196Hz    B3=247Hz    D4=294Hz    G4=392Hz
A3=220Hz    C4=262Hz    E4=330Hz    A4=440Hz
B3=247Hz    D4=294Hz    F4=349Hz    B4=494Hz
```

### 3.2 Envelope (Amplitude)

Defines the volume curve shape:

```
Volume (0-15)
│
│ ╱╲
│╱  ╲___╲
├─────────────── time
A D   S   R
```

| Field | Range | Description | Examples |
|-------|-------|-------------|---------|
| `attack` | 0–500 ms | Time to rise from 0 to peak | 0=instant, 10=soft fade, 100=very slow |
| `decay` | 0–500 ms | Drop time from peak to sustain | 0=no decay, 50=fast decay, 200=smooth |
| `sustain` | 0–15 | Hold volume (0–15) | 8=half, 15=maximum, 0=off |
| `release` | 0–1000 ms | Fade-out time (sustain to 0) | 50=short, 300=long and natural |
| `peak` | 1–15 | Peak volume during attack/decay | 12–15=loud, 5–8=quiet |

**Common Presets:**
- **Snappy (laser)**: attack=0, decay=0, sustain=12, release=100
- **Soft (powerup)**: attack=0, decay=20, sustain=10, release=100
- **Very slow attack**: attack=100, decay=200, sustain=6, release=300

### 3.3 Pitch Sweep

Changes frequency over the duration of the SFX:

| Field | Range | Description | Examples |
|-------|-------|-------------|---------|
| `enabled` | true/false | Enable pitch sweep | |
| `start_mult` | 0.1–4.0 | Starting frequency multiplier | 0.5=half pitch, 1.0=normal, 2.0=double |
| `end_mult` | 0.1–4.0 | Ending frequency multiplier | |
| `curve` | -5 to +5 | Interpolation curve | -3=exponential down (laser), 0=linear, +2=exponential up (powerup) |

**Sweep Examples:**
- **Laser (down)**: start=2.0, end=0.5, curve=-2
- **Powerup (up)**: start=0.8, end=1.5, curve=2
- **Jump (up)**: start=0.6, end=1.3, curve=1
- **Explosion (down)**: start=1.5, end=0.3, curve=-3

### 3.4 Noise

Adds white noise (useful for explosions, percussive effects):

| Field | Range | Description | Examples |
|-------|-------|-------------|---------|
| `enabled` | true/false | Enable noise | |
| `period` | 0–31 | Noise period (lower=higher pitch) | 8=high, 15=medium, 31=low |
| `volume` | 0–15 | Noise volume | 12–15=loud, 5–8=quiet |
| `decay_ms` | 10–1000 | Noise fade-out | 100=fast, 300=slow |

**Presets:**
- **Explosion**: period=8, volume=15, decay=350
- **Impact (hit)**: period=12, volume=14, decay=80

### 3.5 Arpeggio (Chords)

Plays multiple notes in sequence:

| Field | Range | Description | Examples |
|-------|-------|-------------|---------|
| `enabled` | true/false | Enable arpeggio | |
| `arpeggio_notes` | [0–24] | Semitone offsets | [0,4,7]=C-E-G (major chord) |
| `arpeggio_speed` | 10–200 ms | Time between notes | 40=fast, 60=normal, 100=slow |

**Musical Chords (semitones from root note):**
```
C Major        [0, 4, 7]        (C-E-G)
C Minor        [0, 3, 7]        (C-Eb-G)
C Dominant 7   [0, 4, 7, 10]   (C-E-G-Bb)
C Major 7      [0, 4, 7, 11]   (C-E-G-B)
Octave         [0, 12]          (C-C up)
Power chord    [0, 7]           (C-G)
Open fifth     [0, 7, 12]       (C-G-C up)
```

**Examples from presets:**
- **Coin**: [0, 12] = simple octave, speed=60
- **Powerup**: [0, 4, 7, 12] = major chord, speed=40

## 4. Common SFX Recipes

### 4.1 Classic Laser
```json
{
  "name": "laser",
  "duration_ms": 150,
  "oscillator": { "frequency": 880, "channel": 0, "duty": 50 },
  "envelope": { "attack": 0, "decay": 0, "sustain": 12, "release": 100, "peak": 15 },
  "pitch": { "enabled": true, "start_mult": 2.0, "end_mult": 0.5, "curve": -2 },
  "noise": { "enabled": false },
  "modulation": { "arpeggio": false }
}
```
**Effect**: High tone that drops rapidly (classic laser shot)

### 4.2 Coin / Pickup
```json
{
  "name": "coin",
  "duration_ms": 120,
  "oscillator": { "frequency": 880, "channel": 0, "duty": 50 },
  "envelope": { "attack": 0, "decay": 10, "sustain": 12, "release": 80, "peak": 15 },
  "pitch": { "enabled": false },
  "noise": { "enabled": false },
  "modulation": {
    "arpeggio": true,
    "arpeggio_notes": [0, 12],
    "arpeggio_speed": 60
  }
}
```
**Effect**: Cheerful ascending note (simple octave chord)

### 4.3 Jump
```json
{
  "name": "jump",
  "duration_ms": 180,
  "oscillator": { "frequency": 330, "channel": 0, "duty": 50 },
  "envelope": { "attack": 0, "decay": 30, "sustain": 8, "release": 100, "peak": 14 },
  "pitch": { "enabled": true, "start_mult": 0.6, "end_mult": 1.3, "curve": 1 },
  "noise": { "enabled": false },
  "modulation": { "arpeggio": false }
}
```
**Effect**: Note that rises during the jump, decaying volume

### 4.4 Explosion
```json
{
  "name": "explosion",
  "duration_ms": 400,
  "oscillator": { "frequency": 110, "channel": 0, "duty": 50 },
  "envelope": { "attack": 5, "decay": 50, "sustain": 4, "release": 300, "peak": 15 },
  "pitch": { "enabled": true, "start_mult": 1.5, "end_mult": 0.3, "curve": -3 },
  "noise": { "enabled": true, "period": 8, "volume": 15, "decay_ms": 350 },
  "modulation": { "arpeggio": false }
}
```
**Effect**: Low tone that drops fast + loud white noise (classic explosion)

### 4.5 Impact (Hit)
```json
{
  "name": "hit",
  "duration_ms": 100,
  "oscillator": { "frequency": 220, "channel": 0, "duty": 50 },
  "envelope": { "attack": 0, "decay": 10, "sustain": 6, "release": 50, "peak": 15 },
  "pitch": { "enabled": false },
  "noise": { "enabled": true, "period": 12, "volume": 14, "decay_ms": 80 },
  "modulation": { "arpeggio": false }
}
```
**Effect**: Short impact sound (noise + low tone)

### 4.6 Powerup (Ascending)
```json
{
  "name": "powerup",
  "duration_ms": 200,
  "oscillator": { "frequency": 440, "channel": 0, "duty": 50 },
  "envelope": { "attack": 0, "decay": 20, "sustain": 10, "release": 100, "peak": 15 },
  "pitch": { "enabled": true, "start_mult": 0.8, "end_mult": 1.5, "curve": 2 },
  "noise": { "enabled": false },
  "modulation": {
    "arpeggio": true,
    "arpeggio_notes": [0, 4, 7, 12],
    "arpeggio_speed": 40
  }
}
```
**Effect**: Major chord with rising pitch (power-up sound)

## 5. Manual Creation Workflow

### Step 1: Create the base file
```
assets/sfx/custom_sound.vsfx
```

### Step 2: Define basic parameters
1. **Choose base frequency**: Low (110–220 Hz)? Medium (330–440 Hz)? High (880–1760 Hz)?
2. **Define duration**: Short (50–150ms)? Medium (150–300ms)? Long (300+ms)?
3. **Choose a simple envelope**: attack=0, decay=30, sustain=8, release=100

### Step 3: Test and adjust
1. Open the SFX Editor
2. Load your file
3. Press "Play" to hear it
4. Adjust parameters visually
5. Save when it sounds right

### Step 4: Add advanced features
- Need pitch sweep? Enable `pitch.enabled` and set start/end
- Need noise? Enable `noise.enabled`
- Need a chord? Enable `modulation.arpeggio` and define notes

## 6. SFX Design Tips

| Desired Effect | Technique | Parameters |
|---|---|---|
| **Punchy** (snappy) | attack=0, short decay, low sustain | Percussive, short |
| **Soft** | Small attack, long decay | More natural |
| **Retro 8-bit** | duty < 50, strong pitch sweep | Classic arcade |
| **Deep bass** | Low frequency (55–110 Hz) | Impactful |
| **Bright treble** | High frequency (1320+ Hz) | Shiny |
| **Rising** | Pitch sweep up (start < end) | Ascending feel |
| **Falling** | Pitch sweep down (start > end) | Descending feel |
| **Musical** | Arpeggio with major chord [0,4,7] | Harmonic |

## 7. Limitations

- Channel C is **for SFX only** (music uses channels A+B)
- Maximum recommended duration: ~2 seconds (longer impacts performance)
- Arpeggio plays notes **sequentially** (not simultaneously)
- Maximum volume (15) may **clip** on real Vectrex hardware
- Frequencies below 55 Hz or above 1760 Hz cause undefined behavior

## 8. Inspiration Sources

- **SFXR** (web tool): generates random retro SFX
- **Soundly**: professional SFX editor
- **Zapsplat**: free SFX library
- **Atari games**: for authentic 8-bit sounds
