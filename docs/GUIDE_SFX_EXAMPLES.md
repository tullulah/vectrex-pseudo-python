# Analyzing Existing SFX — Practical Examples

This guide shows you how to **read and understand** the project's existing SFX, using them as a reference for creating your own.

## 1. SFX File Locations

```
assets/sfx/
├── jump.vsfx
├── explosion.vsfx
├── coin.vsfx
├── hit.vsfx
├── laser.vsfx
├── powerup.vsfx
├── blip.vsfx
└── [your SFX here]
```

## 2. Reading a Complete SFX

Let's **dissect** the "jump" SFX step by step:

### 2.1 Jump — Ascending Note

**Purpose**: Sound effect when the character jumps.

```json
{
  "version": "1.0",
  "name": "jump",
  "category": "jump",
  "duration_ms": 180
}
```
- Lasts 180ms (3 frames at 50 FPS)
- Short but audible
- Category is used for editor organization

---

### 2.2 Oscillator (Base Tone Generator)

```json
"oscillator": {
  "frequency": 330,
  "channel": 0,
  "duty": 50
}
```
- **330 Hz** = Note E4, medium-high tone
- **Channel 0** = PSG Channel A
- **Duty 50%** = Pure square wave

**Why this works for a jump**:
- Medium frequency allows clear pitch changes to be heard
- Not too low (doesn't sound "thuddy")
- Not too high (doesn't hurt the ears)

---

### 2.3 Envelope (Volume Shape)

```json
"envelope": {
  "attack": 0,
  "decay": 30,
  "sustain": 8,
  "release": 100,
  "peak": 15
}
```

**Timeline visualization:**
```
Volume (0-15)
     15│      ╱╲
       │     ╱  ╲___
        │    ╱    30ms╲___100ms
        │   ╱    sustain=8   ╲
        └──────────────────────── time
       atk=0  decay=30ms  release=100ms
```

- **attack: 0** — No fade-in, starts immediately at full volume (snappy)
- **decay: 30** — Drops from peak (15) to sustain (8) in 30ms
- **sustain: 8** — Holds at medium-low volume after decay
- **release: 100** — Final fade-out from sustain to silence in 100ms
- **peak: 15** — Maximum volume (loud)

**Why it works**:
- Instant attack = "snappy" feel (satisfying)
- Fast decay = simulates the energy of the jump
- Medium sustain = keeps sound without overdoing it
- Long release = fades naturally

---

### 2.4 Pitch Sweep (Frequency Change)

```json
"pitch": {
  "enabled": true,
  "start_mult": 0.6,
  "end_mult": 1.3,
  "curve": 1
}
```

**Visualization:**
```
Frequency multiplier
    1.3│                    ╱╱
       │                   ╱╱
     1.0│────────────────╱
       │               ╱
     0.6│╱╱╱╱╱╱╱╱╱╱╱╱
       └─────────────────── time (180ms)
            curve=1 (exponential up)
```

- **start_mult: 0.6** → starts at 330 × 0.6 = 198 Hz (G3 — low)
- **end_mult: 1.3** → ends at 330 × 1.3 = 429 Hz (A4 — high)
- **curve: 1** → exponential interpolation (smooth rise)

**Real frequencies during the effect:**
```
Start  (0ms):   330 Hz × 0.6 = 198 Hz  (G3 — low)
Middle (90ms):  ~295 Hz
End   (180ms):  330 Hz × 1.3 = 429 Hz  (A4 — high)
```

**Why it works**:
- Starts low = "energy building up"
- Rises gradually = like jumping upward
- Ends high = the jump has happened!

---

### 2.5 Noise

```json
"noise": {
  "enabled": false,
  "period": 15,
  "volume": 12,
  "decay_ms": 100
}
```

- **enabled: false** — No noise in this effect (parameters ignored)

When you'd add noise:
- If you wanted a "rough" jump sound → set enabled: true
- Would simulate friction of feet against the ground

---

### 2.6 Modulation (Arpeggio)

```json
"modulation": {
  "arpeggio": false,
  "arpeggio_notes": [],
  "arpeggio_speed": 50,
  "vibrato": false,
  "vibrato_depth": 0,
  "vibrato_speed": 8
}
```

- **arpeggio: false** — Single note only, no chord
- arpeggio_notes is empty

---

## 3. Comparison: Jump vs Coin

How **coin** differs:

### Jump (analyzed above)
```
- frequency: 330 Hz
- pitch: rises from 0.6x to 1.3x
- envelope: attack=0, decay=30, sustain=8, release=100
- noise: disabled
- arpeggio: false
→ Sound: Single note that rises (WHOOSH)
```

### Coin (arpeggio-based alternative)
```
- frequency: 880 Hz (higher)
- pitch: disabled (no sweep)
- envelope: attack=0, decay=10, sustain=12, release=80
- noise: disabled
- arpeggio: true, notes=[0, 12], speed=60
→ Sound: Two musical notes (octave), short and cheerful
```

**Key differences:**

| Parameter | Jump | Coin |
|-----------|------|------|
| Frequency | 330 Hz (medium) | 880 Hz (high) |
| Pitch sweep | ✅ Yes (0.6→1.3) | ❌ No |
| Arpeggio | ❌ No | ✅ Yes [0,12] |
| Arp speed | — | 60ms |
| Decay | 30ms (fast) | 10ms (very fast) |
| Release | 100ms (long) | 80ms (medium) |

**The audible difference:**
- **Jump**: A single note that RISES (dynamic, movement)
- **Coin**: Two notes played in sequence (simple chord, static)

---

## 4. Analysis: Explosion (Complex)

Explosion is the most **complex** SFX — here's why:

```json
{
  "version": "1.0",
  "name": "explosion",
  "category": "explosion",
  "duration_ms": 400,

  "oscillator": { "frequency": 110, "channel": 0, "duty": 50 },
  "envelope": { "attack": 5, "decay": 50, "sustain": 4, "release": 300, "peak": 15 },
  "pitch": { "enabled": true, "start_mult": 1.5, "end_mult": 0.3, "curve": -3 },
  "noise": { "enabled": true, "period": 8, "volume": 15, "decay_ms": 350 },
  "modulation": { "arpeggio": false }
}
```

### 4.1 Components

**Tone generator:**
- 110 Hz = Note A2 (very low, almost infra-bass)
- Creates low-frequency impact

**Pitch Sweep:**
- Falls from 1.5x to 0.3x (110Hz → 33Hz)
- curve=-3 = fast exponential drop
- Final frequency: 33 Hz (sub-bass territory)
- **Effect**: Simulates the "rumble" of the explosion fading

**Noise (White Noise):**
- period: 8 (high-pitched noise)
- volume: 15 (maximum)
- decay: 350ms
- **Effect**: The "shredding" / "fire" component

**Envelope:**
- attack: 5ms (very fast fade-in)
- decay: 50ms (drops from peak to sustain)
- sustain: 4 (very low, almost silent)
- release: 300ms (LONG fade-out)
- **Effect**: Strong initial impact, then a long dissipation

### 4.2 Timeline

```
Explosion timeline (400ms total):

ms     0      50     100    150    200    300    400
       │      │      │      │      │      │      │
VOLUME │█████████───────────────────╲╲╲╲╲╲╲╲╲╲╲│  Envelope
PITCH  │●●●●●●●●●●●●●●●●●●●●●╲╲╲╲╲╲╲╲╲╲╲╲╲╲│  1.5x→0.3x
NOISE  │██████████████████████╲╲╲╲╲╲╲╲╲╲╲╲╲╲│  350ms decay

Phases:
1. (0–50ms):   Attack: low tone + high noise
2. (50–100ms): Decay: volume drops, pitch continues falling
3. (100–400ms): Long release with noise fading out
```

### 4.3 Why It Sounds Good

1. **Low tone** = impact, "weight"
2. **High noise** = friction, "fire"
3. **Falling pitch** = simulates "air noise" (Doppler-like effect)
4. **Long release** = natural reverberation feel

---

## 5. Template: Create Your Own SFX Based on Examples

### I want a sound for... **Body Hit (punch)**

**Inspired by**: Explosion (noise) + Jump (pitch) + Hit (short)

```json
{
  "version": "1.0",
  "name": "punch",
  "category": "hit",
  "duration_ms": 120,

  "oscillator": { "frequency": 150, "channel": 0, "duty": 50 },
  "envelope": { "attack": 0, "decay": 15, "sustain": 3, "release": 80, "peak": 14 },
  "pitch": { "enabled": true, "start_mult": 1.2, "end_mult": 0.7, "curve": -1 },
  "noise": { "enabled": true, "period": 10, "volume": 12, "decay_ms": 100 },
  "modulation": { "arpeggio": false }
}
```

**Design decisions:**
- **150 Hz** = Low like explosion, but slightly higher
- **Pitch sweep** = Like jump, simulating the "impact"
- **Mixed noise** = Like explosion but lighter
- **120ms duration** = Short like hit, not as long as explosion
- **Low sustain** = "Dry" hit without reverb

---

## 6. Checklist: Verify an SFX Before Saving

```
□ Is version "1.0"?
□ Is name unique? (no spaces or special characters)
□ Is category valid? (custom, laser, explosion, jump, hit, coin, blip, powerup)
□ Is duration_ms in range? (20–2000ms recommended)

□ Is frequency in range? (55–1760 Hz)
□ Is channel 0–2?
□ Is duty 0–100?

□ Do attack + decay + release sum to less than duration_ms?
□ Is peak 1–15?
□ Is sustain 0–15 and less than peak?

□ Is pitch.curve -5 to +5?
□ Is noise.period 0–31?
□ Is noise.volume 0–15?

□ Is arpeggio_notes empty [] or contains numbers 0–24?
□ Is arpeggio_speed 10–200?

□ Have I listened to the effect in the editor → Does it sound good?
```

---

## 7. Reference

### MIDI Note Frequencies
```
C3:131   D3:147   E3:165   F3:175   G3:196   A3:220   B3:247
C4:262   D4:294   E4:330   F4:349   G4:392   A4:440   B4:494
C5:523   D5:587   E5:659   F5:698   G5:784   A5:880   B5:988
```

### Advanced Techniques

**For a more "game-like" feel (arcade-style):**
- Use duty < 50 (thinner waveform)
- Add pitch sweep down (start_mult > end_mult)
- Low sustain (4–6)
- Fast release (50–100ms)

**For an "epic" feel:**
- Use noise with low period (5–8) = high-pitched noise
- Strong pitch sweep down (curve: -3 to -5)
- Long duration (300+ms)
- LONG release (200+ms)

**For a "musical" feel:**
- Enable arpeggio with major chord [0,4,7]
- High sustain (10–12)
- No noise
- Small or disabled pitch sweep

---

**Next step**: Open the SFX Editor, load `assets/sfx/jump.vsfx`, press Play, and experiment by changing each parameter to hear how it affects the sound.
