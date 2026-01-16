# VAR_ARG2 Undefined Error - Multibank Linker Issue

## Problem Description

When compiling multibank projects, Phase 6.7 fails with:
```
⚠ Warning: Multi-bank ROM generation failed: Failed to assemble helper bank 31: 
Símbolo no definido: VAR_ARG2 (buscado como VAR_ARG2)
```

## Root Cause

1. **Code analysis phase** analyzes the VPy code to compute `max_args`
   - For test_multibank_pdb: code doesn't use PRINT_TEXT
   - Result: `max_args = 2` (only needs VAR_ARG0, VAR_ARG1)

2. **Emission phase** emits VAR_ARG definitions based on max_args
   - Emits: VAR_ARG0 EQU, VAR_ARG1 EQU
   - Does NOT emit: VAR_ARG2 EQU (not needed by VPy code)

3. **Helper emission** emits runtime helpers like VECTREX_PRINT_TEXT
   - VECTREX_PRINT_TEXT uses: VAR_ARG0 (x), VAR_ARG1 (y), VAR_ARG2 (string ptr)
   - But VAR_ARG2 doesn't exist!

4. **Multibank linker split** divides main.asm into separate banks
   - bank_31.asm contains: VECTREX_PRINT_TEXT helper code
   - bank_31.asm does NOT contain: VAR_ARG2 EQU definition
   - Reason: EQU definitions are in main.asm (bank_00), not copied to bank_31

5. **Independent assembly** assembles bank_31.asm separately
   - Assembler finds: `LDU VAR_ARG2` in VECTREX_PRINT_TEXT
   - Error: VAR_ARG2 is undefined in this assembly context
   - Phase 6.7 fails

## The Flaw

The issue is a **mismatch between two phases**:
- **Phase 1**: Calculate max_args based on VPy user code only
- **Phase 2**: Emit runtime helpers that may need MORE arguments than user code

When helpers need more VAR_ARG slots than the user code, the calculation fails.

## Solutions (Priority Order)

### Solution 1: Analyze helpers when calculating max_args (BEST)
- Modify `compute_max_args_used()` to also analyze builtin helpers
- Examine VECTREX_PRINT_TEXT, VECTREX_DRAW_VL, etc.
- Calculate: `max_args = max(user_code_args, helpers_args)`
- Cost: Medium (scan helper code)
- Benefit: Automatic, always correct

### Solution 2: Multibank linker injects EQU definitions (WORKAROUND)
- In `multi_bank_linker.rs`, when splitting banks:
- Copy all VAR_ARG* definitions to EVERY bank's ASM
- Bank #31 gets: `VAR_ARG0 EQU $C880+$0D`, etc.
- Cost: Low (3-4 lines per bank)
- Benefit: Fixes multibank assembly immediately
- Tradeoff: Doesn't address root cause

### Solution 3: Force minimum VAR_ARG allocation (QUICK FIX)
- Set `max_args = max_args.max(3)` after calculation
- Always allocate at least VAR_ARG0, VAR_ARG1, VAR_ARG2
- Cost: Minimal (1 line)
- Benefit: Covers PRINT_TEXT case
- Tradeoff: Wastes 2 bytes RAM for projects that don't use helpers

## Recommendation

**Short-term (THIS SESSION)**: Implement Solution 2 (inject EQU definitions)
- Fast to implement (~20 lines in linker)
- Fixes immediate multibank compilation issue
- Unblocks debugging

**Long-term (NEXT SESSION)**: Implement Solution 1 (analyze helpers)
- More robust
- Better for future helpers
- Cleaner architecture

## Test Case

```python
# test_multibank_pdb/src/main.vpy
META ROM_TOTAL_SIZE = 524288
META ROM_BANK_SIZE = 16384

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    DRAW_LINE(0, 0, 50, 50, 127)
```

**Current Result**: Phase 6.7 fails (VAR_ARG2 undefined in bank_31)

**Expected After Fix**: Phase 6.7 succeeds, multibank ROM generated

## Files Involved

- `core/src/backend/m6809/analysis.rs`: compute_max_args_used()
- `core/src/backend/m6809/mod.rs`: emit_builtin_helpers()
- `core/src/backend/m6809/multi_bank_linker.rs`: generate_multibank_rom()
