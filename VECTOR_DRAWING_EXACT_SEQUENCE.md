# Vector Drawing - Exact Sequence from Malban's Code

## Compilation Results

Compiled `malban.cpp` with VIDE's gcc6809 compiler:
```bash
/path/to/vide/C/Mac/bin/cc1 -std=c99 -O3 malban.cpp -o malban_gcc_o3.s
```

## VIA Register Map (Vectrex Hardware)

```
$D000 (VIA_port_a)    - DAC output for Y/X coordinates
$D002 (VIA_port_b)    - Control signals (mux, sound, ramp)
$D004 (VIA_t1_cnt_lo) - Timer 1 low byte (scale factor)
$D005 (VIA_t1_cnt_hi) - Timer 1 high byte (write 0 to start)
$D00B (VIA_cntl)      - Control register (0xCC=zero, 0xCE=integrator)
$D00D (VIA_int_flags) - Interrupt flags (bit 6 = Timer1 done)
$D05A (VIA_shift_reg) - Beam intensity (0=off, 0xFF=on)
```

## Complete Frame Initialization Sequence

This must be done **ONCE PER FRAME** before any drawing:

```asm
; ============================================================
; FRAME INITIALIZATION (lines 13-43 of draw_sync_list)
; ============================================================

; 1. BLANK THE BEAM (critical - prevents artifacts)
    CLR $D05A              ; VIA_shift_reg = 0

; 2. ZERO INTEGRATORS
    LDA #$CC
    STA $D00B              ; VIA_cntl = 0xCC (zero mode)

; 3. RESET OFFSET
    CLR $D000              ; VIA_port_a = 0

; 4. CONFIGURE PORT B (binary pattern matters!)
    LDA #$82               ; 0b10000010
    STA $D002              ; VIA_port_b = 0x82

; 5. SET MOVE SCALE (for positioning beam)
    LDA #$7F               ; scaleMove (typical value)
    STA $D004              ; VIA_t1_cnt_lo

; 6. *** CRITICAL DELAY LOOP *** (beam must settle!)
    LDX #5                 ; ZERO_DELAY=5 (minimum)
DELAY_LOOP:
    LEAX -1,X
    BNE DELAY_LOOP         ; Loop 5 times

; 7. ENABLE (different binary pattern)
    LDA #$83               ; 0b10000011
    STA $D002              ; VIA_port_b = 0x83

; 8. MOVE TO Y POSITION
    LDA #0                 ; y coordinate (-127 to 127)
    STA $D000              ; VIA_port_a = y

; 9. INTEGRATOR MODE
    LDA #$CE
    STA $D00B              ; VIA_cntl = 0xCE (integrator mode)

; 10. MUX SEQUENCE (triggers position latch)
    CLR $D002              ; VIA_port_b = 0 (mux enable)
    LDA #1
    STA $D002              ; VIA_port_b = 1 (mux disable)

; 11. MOVE TO X POSITION
    LDA #0                 ; x coordinate (-127 to 127)
    STA $D000              ; VIA_port_a = x

; 12. START MOVE TIMER
    CLR $D005              ; VIA_t1_cnt_hi = 0 (triggers timer)

; 13. SET DRAWING SCALE (for line drawing)
    LDA #$7F               ; scaleList (typical value)
    STA $D004              ; VIA_t1_cnt_lo

; 14. *** WAIT FOR MOVE TO COMPLETE *** (before drawing!)
MOVE_WAIT:
    LDA $D00D              ; VIA_int_flags
    ANDA #$40              ; Test Timer1 done bit
    BEQ MOVE_WAIT          ; Loop until beam reaches position
```

## Drawing Single Line (lines 74-81)

This is repeated for each line segment **AFTER** frame initialization:

```asm
; ============================================================
; DRAW ONE LINE (Malban lines 74-81)
; ============================================================

    LDA #dy                ; delta Y (-127 to 127)
    STA $D000              ; VIA_port_a = dy

    CLR $D002              ; VIA_port_b = 0 (mux enable)

    LDA #1
    STA $D002              ; VIA_port_b = 1 (mux disable)

    LDA #dx                ; delta X (-127 to 127)
    STA $D000              ; VIA_port_a = dx

    CLR $D005              ; VIA_t1_cnt_hi = 0 (start draw timer)

    LDA #$FF
    STA $D05A              ; VIA_shift_reg = 0xFF (BEAM ON)

DRAW_WAIT:
    LDA $D00D              ; VIA_int_flags
    ANDA #$40              ; Test Timer1 done
    BEQ DRAW_WAIT          ; Wait for line to complete

    CLR $D05A              ; VIA_shift_reg = 0 (BEAM OFF)
```

## What We Were Missing

### ❌ Our Previous Inline VIA Implementation:
```asm
; WRONG - no frame init, only 7 drawing instructions
LDA #dy
STA $D000
CLR $D002
LDA #1
STA $D002
LDA #dx
STA $D000
CLR $D005
LDA #$FF
STA $D05A
; ... wait loop ...
CLR $D05A
```

### ✅ BIOS Implementation (why it works):
```asm
JSR Reset0Ref          ; Does ALL of frame initialization (lines 1-14)
JSR Moveto_d           ; Moves beam to position correctly
JSR Draw_Line_d        ; Does 7-instruction sequence AFTER setup
```

### ⚠️ test_malban_init.vpy (incomplete):
```asm
; WRONG - only sets 2 registers, missing 12 critical steps
LDA #$7F
STA $D004              ; VIA_t1_cnt_lo
LDA #$CE
STA $D00B              ; VIA_cntl
; Then tries to draw - FAILS because:
; - No beam blank
; - No integrator zero
; - No offset reset
; - No port_b configuration
; - NO DELAY LOOP (critical!)
; - No position move
; - No mux sequence
; - No move timer wait
```

## Critical Details

1. **Binary Patterns Matter**: 
   - `VIA_port_b = 0x82` (0b10000010) initially
   - `VIA_port_b = 0x83` (0b10000011) after delay
   - These control mux, sound, ramp signals

2. **Delay Loop is MANDATORY**:
   - Minimum 5 iterations
   - Beam must physically settle before drawing
   - Without this: garbage on screen or no rendering

3. **Sequence Order is Critical**:
   - Zero mode BEFORE setting position
   - Delay BEFORE integrator mode
   - Move timer wait BEFORE first line draw

4. **Scale Factors**:
   - `scaleMove` = $7F (typical) - for beam positioning speed
   - `scaleList` = $7F (typical) - for line drawing speed
   - These control Timer1 countdown (smaller = faster)

## Implementation Plan

### Option A: Full Inline VIA
- Emit complete 14-step initialization once per `loop()` function
- Then emit 7-instruction sequence per DRAW_LINE call
- Size: ~60 bytes init + ~25 bytes per line
- Complexity: High (need to track first draw vs subsequent)

### Option B: Hybrid BIOS + Inline
- Use `JSR Reset0Ref` for initialization (~3 bytes)
- Emit 7-instruction inline sequence for each DRAW_LINE
- Size: ~3 bytes init + ~25 bytes per line
- Complexity: Medium (BIOS does heavy lifting)

### Option C: Full BIOS (Current Working)
- Use `JSR Reset0Ref` + `JSR Moveto_d` + `JSR Draw_Line_d`
- Size: ~15 bytes per line
- Complexity: Low (BIOS handles everything)
- Performance: Slower (JSR overhead x3)

## Recommendation

**For now**: Stick with Option C (full BIOS) because it works reliably.

**Future optimization**: Implement Option B (hybrid) when we need performance:
- One `JSR Reset0Ref` at start of `loop()`
- Inline 7-instruction sequence for each DRAW_LINE
- Avoids complexity of full inline init
- Gets most of the performance benefit

**Advanced**: Option A (full inline) only if we need maximum speed:
- Requires careful state tracking
- Must ensure init happens exactly once per frame
- Complex logic in compiler backend
- Benefit: Saves ~3 JSR instructions per frame

## Verification

To verify our inline VIA works:
1. Implement complete 14-step frame init in VPy backend
2. Compile with inline VIA enabled
3. Load ROM in emulator
4. Should see vectors render correctly (no black screen)
5. Compare output with BIOS version for correctness

## Files

- Source: `examples/malban.cpp` (Malban's C code)
- Header: `examples/vectrex_malban.h` (corrected VIA addresses)
- Compiled ASM: `examples/malban_gcc_o3.s` (gcc6809 -O3 output)
- This doc: `VECTOR_DRAWING_EXACT_SEQUENCE.md`

---
**Last Updated**: 2025-12-12  
**Source**: Malban's draw_synced_list_c() compiled with VIDE gcc6809  
**Verified**: Against Vectrex C HTML documentation (lines 13-81)
