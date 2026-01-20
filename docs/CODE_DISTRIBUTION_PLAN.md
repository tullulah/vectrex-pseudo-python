# Code Distribution Plan for Large Multibank Projects

## Current State (2026-01-xx)

### What Works
- ✅ Asset distribution across banks (bin-packing, 8KB threshold)
- ✅ test_incremental compiles (1529 bytes assets, kept in Bank #0)
- ✅ DRAW_VECTOR_BANKED wrapper using $DF00 bank switch register
- ✅ Single-bank projects up to 32KB

### What Doesn't Work
- ❌ pang_multi: CODE is >16KB, causes "Branch offset OUT OF RANGE"
- ❌ No automatic code distribution across banks
- ❌ No cross-bank function call wrappers

## Problem Analysis

### pang_multi Statistics
- **VPy source**: 484 lines, 11 functions
- **Generated ASM**: 12892 lines total
  - Bank #0 code: 5454 lines → **>16KB binary**
  - Bank #1 assets: 6122 lines (39 assets)
  - Banks #2-30: Empty

### Root Cause
The 11 VPy functions compile to >16KB of M6809 instructions, exceeding Bank #0's capacity.

## Solution Options

### Option 1: Automatic Code Distribution (Complex)
- Analyze function sizes during compilation
- Use bin-packing to distribute functions across banks
- Generate cross-bank call wrappers automatically
- **Effort**: High (2-3 days)
- **Risk**: Medium (complexity in call graph analysis)

### Option 2: Manual @bank Decorator (Medium)
```python
@bank(2)
def draw_map_screen():
    ...

@bank(2) 
def draw_title_screen():
    ...
```
- User explicitly assigns functions to banks
- Compiler generates wrappers for cross-bank calls
- **Effort**: Medium (1 day)
- **Risk**: Low (user has control)

### Option 3: Code Size Optimization (Incremental)
- Optimize common patterns to reduce ASM size
- Better use of BIOS routines
- Inline small functions
- **Effort**: Low-Medium (ongoing)
- **Risk**: Low (gradual improvement)

## Recommended Approach

**Phase 1** (Immediate): Document limitation, let users work around it
- Split large projects into smaller files manually
- Use BIOS routines where possible

**Phase 2** (Short term): Implement @bank decorator
- Simple, explicit control
- No complex analysis required
- Cross-bank call wrappers via known patterns

**Phase 3** (Long term): Automatic code distribution
- Size analysis per function
- Optimal bin-packing
- Full automation

## Cross-Bank Call Wrapper Pattern

For functions in different banks:
```asm
; In Bank #0 (caller's bank)
DRAW_MAP_SCREEN_WRAPPER:
    PSHS A              ; Save A register
    LDA CURRENT_ROM_BANK ; Read current bank
    PSHS A              ; Save on stack
    LDA #2              ; Target bank
    STA CURRENT_ROM_BANK
    STA $DF00           ; Switch bank
    JSR DRAW_MAP_SCREEN ; Call real function
    PULS A              ; Restore original bank
    STA CURRENT_ROM_BANK
    STA $DF00
    PULS A              ; Restore A
    RTS
```

## Memory Layout for Code Distribution

```
Bank #0 ($0000-$3FFF): 
  - START, MAIN, LOOP
  - Critical functions (must be in entry bank)
  - Wrappers to other banks

Bank #1 ($0000-$3FFF switchable):
  - Assets (vectors, music, SFX)

Bank #2-30 ($0000-$3FFF switchable):
  - User functions (distributed by size or @bank)

Bank #31 ($4000-$7FFF fixed):
  - Runtime helpers
  - Wrappers
  - Always visible
```

## Open Questions

1. **Stack safety**: Cross-bank calls must preserve stack properly
2. **Recursion**: Self-recursive functions across banks need care
3. **Data access**: Global variables accessible from all banks (in RAM)
4. **Interrupts**: IRQ handler must be in fixed bank (Bank #31)

## Timeline

- [ ] Document limitation in copilot-instructions.md
- [ ] Implement @bank decorator (Phase 2)
- [ ] Test with pang_multi using manual bank assignment
- [ ] Consider automatic distribution (Phase 3)
