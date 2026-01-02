# Pang Compilation Fix - AUDIO_UPDATE Symbol Resolution

## Problem Statement

When compiling the minimal `pang` project (or any VPy file without music/SFX), the compiler failed with:

```
❌ PHASE 6 FAILED: Binary assembly error
   Error: Native assembler failed: Símbolo no definido: AUDIO_UPDATE (buscado como AUDIO_UPDATE)
```

## Root Cause Analysis

The issue was a **conditional compilation mismatch**:

1. **In `mod.rs` (line 524)**: The compiler ALWAYS auto-injects `JSR AUDIO_UPDATE` into the `loop()` function:
   ```rust
   out.push_str("    JSR AUDIO_UPDATE  ; Auto-injected: update music + SFX\n");
   ```

2. **In `emission.rs` (line 214)**: The `AUDIO_UPDATE` symbol was only CONDITIONALLY emitted:
   ```rust
   if w.contains("PLAY_MUSIC_RUNTIME") || w.contains("STOP_MUSIC_RUNTIME") || has_music_assets {
       // ... emit AUDIO_UPDATE and all related functions
   }
   ```

3. **When neither music nor SFX were used**: The condition was FALSE, so `AUDIO_UPDATE` was never defined, but the assembler still tried to link it, causing the undefined symbol error.

4. **Secondary issue**: PSG RAM variable definitions (`PSG_MUSIC_PTR`, `PSG_MUSIC_START`, etc.) were commented out instead of actual EQU statements:
   ```asm
   ; PSG_MUSIC_PTR    EQU RESULT+26  (2 bytes)   ← Only a comment, not a real EQU
   ```

## Solution Implemented

### Change 1: Make AUDIO_UPDATE Unconditional (emission.rs:216)

Changed from:
```rust
if w.contains("PLAY_MUSIC_RUNTIME") || w.contains("STOP_MUSIC_RUNTIME") || has_music_assets {
```

To:
```rust
if true /* ALWAYS emit AUDIO_UPDATE and related functions */ {
```

**Rationale**: Since `AUDIO_UPDATE` is auto-injected unconditionally in `mod.rs`, it MUST be defined unconditionally in `emission.rs`. This ensures that all projects, even those without audio, have the `AUDIO_UPDATE` symbol available.

### Change 2: Uncomment PSG RAM Variables (emission.rs:241-244)

Changed from:
```asm
; RAM variables (defined in RAM section above)
; PSG_MUSIC_PTR    EQU RESULT+26  (2 bytes)
; PSG_MUSIC_START  EQU RESULT+28  (2 bytes)
; PSG_IS_PLAYING   EQU RESULT+30  (1 byte)
; PSG_MUSIC_ACTIVE EQU RESULT+31  (1 byte) - Set=1 during UPDATE_MUSIC_PSG
```

To:
```asm
; RAM variables (defined in RAM section above)
PSG_MUSIC_PTR    EQU RESULT+26  ; 2 bytes
PSG_MUSIC_START  EQU RESULT+28  ; 2 bytes
PSG_IS_PLAYING   EQU RESULT+30  ; 1 byte
PSG_MUSIC_ACTIVE EQU RESULT+31  ; 1 byte - Set=1 during UPDATE_MUSIC_PSG
```

**Rationale**: The PSG music code in `PLAY_MUSIC_RUNTIME` and `UPDATE_MUSIC_PSG` references these variables. They must be actual EQU definitions, not comments, for the assembler to resolve the symbols correctly.

## Results

After applying these fixes:

✅ **Pang project compiles successfully:**
```
✓ Phase 4 SUCCESS: Generated 20045 bytes of assembly
✓ Phase 5 SUCCESS: Written to examples/pang/src/main.asm
✓ Native assembler successful
✓ Assembler generated: 1105 bytes
✓ Padded to 8192 bytes (available space: 7087 bytes / 6 KB)
✓ NATIVE ASSEMBLER SUCCESS: examples/pang/src/main.bin -> 8192
✓ Phase 6 SUCCESS: Binary generation complete
```

✅ **Verified working:**
- Empty VPy projects (no PLAY_MUSIC() calls) now compile
- Binary generation produces correct 8192-byte ROM file
- No undefined symbol errors

## Architecture Notes

### AUDIO_UPDATE Flow

```
mod.rs line 524: Loop function generation
    ↓
Unconditional: out.push_str("JSR AUDIO_UPDATE")
    ↓
emission.rs line 216: Helper function emission
    ↓
Now unconditional: if true { emit AUDIO_UPDATE and PSG functions }
    ↓
Assembler: Can always resolve AUDIO_UPDATE symbol ✓
```

### PSG System Architecture

When `AUDIO_UPDATE` is called (every frame):
1. Updates music (channels B via BIOS Sound_Byte)
2. Updates SFX (channel C via BIOS Sound_Byte)
3. Both use unified PSG register access through BIOS

The PSG variables store:
- `PSG_MUSIC_PTR`: Current position in music data stream
- `PSG_MUSIC_START`: Loop restart position
- `PSG_IS_PLAYING`: Music active flag
- `PSG_MUSIC_ACTIVE`: Flag used during UPDATE_MUSIC_PSG for logging

## Testing

To verify the fix works with minimal projects:

```bash
# Compile pang (empty loop, no audio)
cargo run --bin vectrexc -- build examples/pang/src/main.vpy --bin

# Expected output
✓ Phase 6 SUCCESS: Binary generation complete
```

## Files Modified

- `core/src/backend/m6809/emission.rs` (2 changes)
  - Line 216: Made AUDIO_UPDATE emission unconditional
  - Lines 241-244: Uncommented PSG RAM variable EQU definitions

## Commit

```
commit 67f8d891...
Fix: Always emit AUDIO_UPDATE and PSG RAM variable definitions

Problem:
- AUDIO_UPDATE was auto-injected but only conditionally defined
- Caused undefined symbol error in projects without music/SFX
- PSG variables were commented instead of actual EQU

Solution:
- Make AUDIO_UPDATE emission unconditional
- Uncomment PSG RAM variable definitions
- Results: pang.vpyproj compiles successfully
```

## Future Improvements

For even cleaner architecture, consider:

1. **Dynamic injection based on usage**: Instead of always injecting `AUDIO_UPDATE`, only inject it if the program actually calls `MUSIC_UPDATE()` or `SFX_UPDATE()` or has assets.

2. **Stub pattern**: Similar to `sfx_doframe` stub (line 536-540), could emit empty `AUDIO_UPDATE` stub that just does `RTS` when not needed.

3. **Documentation**: Update the compiler documentation to clarify that `AUDIO_UPDATE` is always present even without audio assets.

## Related Code Sections

- `core/src/backend/m6809/mod.rs:524` - AUDIO_UPDATE auto-injection
- `core/src/backend/m6809/emission.rs:210-545` - Audio helper functions
- `core/src/backend/m6809/emission.rs:536-540` - sfx_doframe stub pattern (reference)
- `examples/pang/` - Test case (minimal VPy project)
