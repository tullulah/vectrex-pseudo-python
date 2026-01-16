# Fix: Duplicate Builtins in Multibank Flat ASM File

## Problem
The multibank flat file (`multibank_flat.asm`) contained duplicate builtin helper functions, specifically:
- `J1X_BUILTIN` appeared 2 times (lines 841 and 1381)
- `DRAW_LINE_WRAPPER` appeared 2 times (lines 939 and 1479)
- All other runtime helpers appeared twice

This happened both in:
- **Multibank mode**: Flat file for inspection/debugging
- **Singlebank mode**: Main ASM file (less noticeable)

## Root Cause
The issue was in `core/src/backend/m6809/mod.rs` where `emit_builtin_helpers()` was called twice:

1. **First call (line 1204)**: Inside the multibank block `if !opts.function_bank_map.is_empty()` 
   - This emits helpers at the end of the fixed bank (#31)
   - Only executed in multibank mode

2. **Second call (line 1505)**: After processing all items
   - This was UNCONDITIONAL (no guard)
   - Executed in BOTH multibank and singlebank modes
   - In multibank mode: emitted helpers a second time, creating duplicates

## Solution
Modified `core/src/backend/m6809/mod.rs` lines 1498-1517 to guard the second emission:

```rust
// Emit builtin helpers AFTER all user code (cleaner organization)
// CRITICAL FIX: Only emit helpers if NOT in multibank mode
// In multibank mode, helpers are already emitted at end of fixed bank (#31)
// Avoid emitting twice - once in fixed bank and once here
if opts.function_bank_map.is_empty() {  // <-- NEW GUARD
    // Emit section marker for helper functions
    emit_helpers_section(&mut out, opts);
    
    if !suppress_runtime {
        // Emit builtin helpers AFTER user code but before DATA section
        if !opts.skip_builtins {
            emit_builtin_helpers(&mut out, &rt_usage, opts, module, &mut debug_info);
        }
        
        if rt_usage.needs_mul_helper { emit_mul_helper(&mut out); }
        if rt_usage.needs_div_helper { emit_div_helper(&mut out); }
    }
}  // <-- NEW CLOSING BRACE
```

**Change**: Wrap the second `emit_builtin_helpers` call with `if opts.function_bank_map.is_empty()` so it only executes in singlebank mode.

## Results

### Before Fix
- **Singlebank** (main.asm):
  - J1X_BUILTIN: 1 occurrence ✓
  - DRAW_LINE_WRAPPER: 1 occurrence ✓
  
- **Multibank** (multibank_flat.asm):
  - J1X_BUILTIN: 2 occurrences ❌ (lines 841, 1381)
  - DRAW_LINE_WRAPPER: 2 occurrences ❌ (lines 939, 1479)
  - Total file size: 1972 lines (excessive)

### After Fix
- **Singlebank** (main.asm):
  - J1X_BUILTIN: 1 occurrence ✓
  - DRAW_LINE_WRAPPER: 1 occurrence ✓
  - (no change - still works correctly)
  
- **Multibank** (multibank_flat.asm):
  - J1X_BUILTIN: 1 occurrence ✓ (line 841 only)
  - DRAW_LINE_WRAPPER: 1 occurrence ✓ (line 939 only)
  - Total file size: 1431 lines (541 lines removed)
  - **Improvement**: -27% reduction in duplicate code

## Verification
✅ Singlebank compilation: Still works correctly
✅ Multibank compilation: Phase 6.7 SUCCESS (512KB ROM generated)
✅ No duplicate builtins in multibank flat file
✅ No duplicate builtins in singlebank main.asm

## Files Modified
- `core/src/backend/m6809/mod.rs` (lines 1498-1517)

## Commit
```
fix: Eliminate duplicate builtins in multibank code generation

- Fixed double emission of emit_builtin_helpers() in emit() function
- Guard second call to only execute in singlebank mode (is_empty check)
- Multibank flat file now clean (no J1X_BUILTIN, DRAW_LINE_WRAPPER dupes)
- Singlebank mode unaffected (helpers still correctly emitted)
- File size reduction: -27% for multibank flat file (1972 → 1431 lines)
- Phase 6.7 multibank ROM generation still succeeds (512KB, 32 banks)

Related: Multibank quality improvement, flat file cleanup for inspection
```

## Technical Context
The problem was a "fallthrough" bug where code that should have been conditional (only for singlebank) was unconditionally executed for all modes. The fix uses the same pattern already established in the code:
- Line 1035: `if !opts.function_bank_map.is_empty()` for multibank-specific code
- Line 1221: `if opts.function_bank_map.is_empty()` for singlebank-specific code
- Lines 1498+: (NEW) Same guard for avoiding double emission

This ensures each path (multibank vs singlebank) emits helpers exactly once.
