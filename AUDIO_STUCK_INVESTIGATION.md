# Audio Stuck Notes Investigation

## Problem Statement
Pang music plays correctly but **notes and noise stay stuck** at their last values instead of stopping cleanly. User hears continuous tone + percussion.

## Root Cause Analysis

### Investigation Steps
1. Compiled pang with `--bin` to check actual binary (11,017 bytes)
2. Inspected AU_MUSIC_ENDED code in pang.asm - silencing code WAS there but NOT executing
3. Examined pang_theme.vmus data in binary at offset 0x95C (2396 decimal)
4. Found PSG register frames: `0x08, 0x0C, 0x02, 0x1C, ...` (register write pairs)
5. **Critical finding**: No $FF loop marker or $00 end marker in binaries inspected

### The Real Problem: Infinite Loop Without Exit

**File**: `core/src/musres.rs` lines 370-377

```rust
if loop_start_frame < loop_end_frame && loop_end_frame > 0 {
    // Loop marker: FCB $FF (special value that can't be a frame count), FDB address
    asm.push_str(&format!("    FCB     $FF             ; Loop command\n"));
    asm.push_str(&format!("    FDB     _{}_MUSIC       ; Jump to start\n", symbol_name));
} else {
    asm.push_str("    FCB     0               ; End of music (no loop)\n");
}
```

**pang_theme.vmus Configuration:**
```json
{
  "loopStart": 0,
  "loopEnd": 384,
  ...
}
```

**What happens:**
1. Music plays normally (frames 0-N)
2. Reaches end of notes/noise (around frame ~8)
3. Loop condition is `loopStart (0) < loopEnd (384)` → TRUE
4. Compiler emits: `FCB $FF` followed by `FDB _PANG_THEME_MUSIC`
5. AUDIO_UPDATE reads $FF, executes `AU_MUSIC_LOOP`
6. `AU_MUSIC_LOOP` jumps to loopStart (0)
7. **Music never ends, only loops infinitely**
8. **AU_MUSIC_ENDED never executes** ← Silencing code can't run

### Why Silencing Code Can't Help
- Added in commit `cc3557be` but reverted in `c41b8ec6`
- The code sits in `AU_MUSIC_ENDED` (emission.rs lines 240-253)
- But `AU_MUSIC_ENDED` is only reached if music ends (count=0, no loop)
- With infinite looping, execution flow is: `AU_MUSIC_DONE` → `AU_MUSIC_LOOP` → jump → repeat
- Never executes the path that silences channels

### Why Notes Stay Stuck
1. Last frame written to PSG sets channels to specific tones/noise
2. No explicit volume=0 written when loop continues
3. Hardware PSG maintains last written values indefinitely
4. Result: continuous stuck tone from last note played

## Solution Options

### Option 1: Don't Loop (Simplest)
Set `loopEnd: 0` in pang_theme.vmus:
```json
{
  "loopStart": 0,
  "loopEnd": 0,  // No loop
  ...
}
```
Music will end with `FCB 0`, triggering AU_MUSIC_ENDED silencing.
**Downside**: Music plays once and stops, no continuous background music.

### Option 2: Add Loop Count (Medium)
Modify AUDIO_UPDATE to track loop iterations:
- First time: play normally, loop as requested
- After N loops: stop and execute AU_MUSIC_ENDED
**Downside**: Requires code changes, complexity in tracking loop count.

### Option 3: Add Silence Frame at Loop Point (Recommended)
Modify `musres.rs` compiler to:
- After last real music frame, add explicit silence frame
- Only silences inactive channels
- Keeps loop working indefinitely
- **This is what sfx_doframe already does properly**
**Advantage**: Clean exit, no stuck notes, loop still works.

### Option 4: Change Loop Semantics (Future)
New JSON option: `"loopUntilStop": true`
- Loops as configured
- But PLAY_MUSIC("name") sets internal flag
- STOP_MUSIC() clears flag, music exits at next loop point
**Advantage**: Allows intentional stop, most flexible.

## Technical Details

### Current AUDIO_UPDATE Flow
```
AU_MUSIC_DONE:
    STX >PSG_MUSIC_PTR          ; Save pointer
    BRA AU_UPDATE_SFX           ; Continue to SFX (no silence!)

AU_MUSIC_ENDED:
    CLR >PSG_IS_PLAYING         ; Set flag
    LDA #$08; LDB #$00; JSR Sound_Byte  ; Silence A
    LDA #$09; LDB #$00; JSR Sound_Byte  ; Silence B
    LDA #$0A; LDB #$00; JSR Sound_Byte  ; Silence C
    BRA AU_UPDATE_SFX           ; Continue to SFX

AU_MUSIC_LOOP:
    LDD ,X                      ; Load loop target
    STD >PSG_MUSIC_PTR          ; Set pointer to loop
    BRA AU_UPDATE_SFX           ; No silence!
```

## Immediate Recommendation

**Disable looping in pang_theme.vmus** until a proper solution is implemented:

```json
{
  "version": "1.0",
  "name": "pang_theme",
  "tempo": 120,
  "ticksPerBeat": 24,
  "totalTicks": 384,
  "notes": [ ... ],
  "noise": [ ... ],
  "loopStart": 0,
  "loopEnd": 0  // ← Change this from 384 to 0
}
```

This will:
1. Allow music to play once cleanly
2. Execute AU_MUSIC_ENDED with proper silencing
3. Stop notes from staying stuck

## Files Involved
- `core/src/musres.rs` - Music compilation to ASM
- `core/src/backend/m6809/emission.rs` - AUDIO_UPDATE implementation
- `examples/pang/assets/music/pang_theme.vmus` - Music asset
