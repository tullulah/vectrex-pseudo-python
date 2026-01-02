# Music Roundtrip Compilation Fix

## Date
2025-12-10

## Issue
The `test_music_roundtrip_compilation` test was failing because VMUS JSON → ASM → Binary → Decompiled VMUS was not preserving timing information.

**Failure**: Notes were being decompiled with sequential tick positions (0, 1, 2, 3) instead of their original positions (0, 24, 48, 72).

## Root Cause
The binary format was encoding:
```
[count register_1 value_1 register_2 value_2 ... ] [count ... ] ... [0]
```

But it wasn't encoding the **actual tick position** of each block. The decompiler had no way to know that:
- Block 0 = tick 0 (note 1 starts)
- Block 1 = tick 24 (note 1 ends, note 2 starts)
- Block 2 = tick 48 (note 2 ends, note 3 starts)
- Block 3 = tick 72 (note 3 ends)

## Solution
Modified the binary format to include tick positions:
```
[tick_hi tick_lo count register_1 value_1 ... ] [tick_hi tick_lo count ... ] ... [0 0]
```

### Changes Made

#### 1. Compiler (`compile_vmus_to_test_asm`)
Added tick position encoding before each block:
```rust
// NEW: Emit the tick position first!
let tick_hi = ((frame >> 8) & 0xFF) as u8;
let tick_lo = (frame & 0xFF) as u8;
asm.push_str(&format!("    FCB     {}              ; Tick position (high byte) for frame {}\n", tick_hi, frame));
asm.push_str(&format!("    FCB     {}              ; Tick position (low byte) for frame {}\n", tick_lo, frame));
asm.push_str(&format!("    FCB     {}              ; Frame {} - {} writes\n", ...));
```

#### 2. Decompiler (`decompile_music_binary`)
Now reads tick positions to determine block timing:
```rust
let tick_hi = data[offset] as u32;
let tick_lo = data[offset + 1] as u32;
let current_tick = (tick_hi << 8) | tick_lo;
let count = data[offset + 2];
offset += 3;  // Skip tick position and count bytes
```

#### 3. End Marker
Updated to use both tick bytes:
```asm
FCB     0, 0            ; End of music (tick 0, count 0)
```

## Test Result
✅ `test_music_roundtrip_compilation` now passes

The decompiled VMUS now correctly reconstructs:
- Note start positions (0, 24, 48)
- Note durations (24, 24, 24)
- All other metadata (MIDI notes, velocity, channels, tempo, etc.)

## Impact
This fix ensures that music files can be reliably round-tripped through the compilation pipeline, which is essential for:
1. Testing the music system
2. Validating compilation correctness
3. Future real-time music playback features

## Format Compatibility
⚠️ This is a **test-only format change**. The actual `musres.rs` compiler will need similar updates if music compilation is implemented in the real codebase.
