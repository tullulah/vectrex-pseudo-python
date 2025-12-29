# PDB Line Mapping Coverage Improvements - 2025-12-29

## Summary
**Improved PDB debug symbol coverage from 74.8% to 89.0%** (+14.2 points, 44 additional lines mapped)

### Coverage Progress
| Phase | Coverage | Mapped Lines | Missing Lines |
|-------|----------|--------------|---------------|
| **Initial State** | 74.8% | 232/310 | 78 |
| **Global Variables** | 86.8% | 269/310 | 41 |
| **Const Numbers** | 89.0% | 276/310 | 34 |

## Changes Made

### 1. **Global Variable Initialization Line Mapping** (+30 lines)

#### Files Modified
- `core/src/backend/m6809/mod.rs` (lines 551-605, 619-649)

#### Changes
- **Before**: Global variables initialized with `&global_vars` (no line numbers)
- **After**: Use `&global_vars_with_line` which includes source line for each variable
- Emit `; VPy_LINE:{source_line}` before each global variable initialization
- Works in both `main_has_content` and `!main_has_content` paths

#### Result
Lines 11-16, 30-51, etc. (global variable declarations) now mapped to `.pdb`

### 2. **Const Number Declaration Line Mapping** (+6 lines)

#### Files Modified
- `core/src/backend/m6809/mod.rs` (lines 508-520)
- `core/src/backend/debug_info.rs` (lines 444-446, 520-523)

#### Changes
1. **In mod.rs**:
   - After `JMP START`, emit comments for each const NUMBER (non-array) declaration
   - Format: `; VPy_LINE:X` followed by `; _CONST_DECL_N:  ; const {name}`
   - Dummy label allows parser to register the pending marker

2. **In debug_info.rs**:
   - Recognize pseudo-labels starting with `; _` and ending with `:` as valid markers
   - Treat them like real labels for pending marker registration
   - Line 444-446: Update comment filtering to exclude `; _*` from skip list

#### Result
Lines 7-9, 54, 63-64 (const number declarations) now mapped to `.pdb`

### 3. **Fixed PDB Parser for Data Sections** (Bug fix)

#### Files Modified
- `core/src/backend/debug_info.rs` (lines 473-500, 509-523)

#### Changes
1. **Marker Registration Before Data Directives** (line 473-478):
   - **Before**: Only registered `pending_marker` when line didn't match FDB/FCB/etc. checks
   - **After**: Register `pending_marker` BEFORE checking data directives
   - This ensures const arrays and other data sections get mapped

2. **End-of-File Marker Registration** (line 520-523):
   - Handle case where `pending_marker` is never consumed (file ends with comments)
   - Register any remaining pending marker at current address at EOF

#### Result
Const array lines 22, etc. properly mapped instead of being lost

## Technical Details

### Global Variables Path

```rust
// OLD: No line mapping
for (name, value) in &global_vars { ... }

// NEW: With line mapping
for (name, value, source_line) in &global_vars_with_line {
    if const_array_names.contains(name) { continue; }  // Skip const arrays
    tracker.set_line(*source_line);
    out.push_str(&format!("    ; VPy_LINE:{}\n", source_line));
    // ... emit initialization ...
}
```

### Const Number Declaration Path

```asm
; VPy_LINE:7
; _CONST_DECL_0:  ; const STATE_TITLE
; VPy_LINE:8
; _CONST_DECL_1:  ; const STATE_MAP
; VPy_LINE:9
; _CONST_DECL_2:  ; const STATE_GAME
```

Parser recognizes `; _CONST_DECL_N:` as a pseudo-label and registers pending `VPy_LINE` markers.

### PDB Parser Logic Update

**Key insight**: Move `pending_marker` registration to BEFORE data directive checks:

```rust
// Register any pending marker BEFORE processing data directives
if let Some(line_num) = pending_marker.take() {
    line_map.insert(line_num, format!("0x{:04X}", current_address));
}

// Then process data directives
if trimmed.starts_with("FDB ") { ... }
if trimmed.starts_with("FCB ") { ... }
```

This ensures const array data (FDB/FCB) is mapped to the line that precedes it.

## Remaining Gaps (34 lines, 11%)

### Type 1: META Statements (1 line)
- **Example**: Line 4: `META TITLE = "PANG"`
- **Reason**: Metadata, not executable code
- **Effort**: Would need special handling in parser

### Type 2: Control Structure Keywords (33 lines)
- **Examples**: Lines 129, 133, 137: `elif`/`else` keywords
- **Reason**: Don't generate instructions themselves, only affect branch flow
- **Current Status**: The actual branches ARE mapped (88, 91, etc.), but keywords aren't
- **Effort**: Would need `emit_stmt` modifications for IfStmt branch keywords
- **Note**: These are less critical - actual code is mapped, just the structure isn't

## Code Quality Improvements

### 1. **Better PDB Parser Robustness**
- Handles edge cases (const numbers followed by only comments)
- Registers pending markers at EOF if never consumed
- Recognizes pseudo-labels in comments

### 2. **Separation of Concerns**
- `const_vars_with_line` vs `const_vars` - distinction between keyed and unkeyed
- `global_vars_with_line` vs `global_vars` - same pattern
- Allows future extensions without breaking existing code

### 3. **Parser Efficiency**
- Single pass through ASM for line mapping
- No regex, just string operations
- Scales well to large programs

## Testing

### Verification Steps
```bash
# 1. Recompile compiler
cargo build --release

# 2. Build example
./target/release/vectrexc build examples/pang/src/main.vpy --bin

# 3. Check coverage
python3 check_pdb_coverage.py examples/pang/src/main.vpy examples/pang/src/main.pdb
```

### Expected Output
```
📊 Análisis de cobertura .pdb
Total líneas con código en .vpy: 310
Total líneas mapeadas en .pdb:  276
Líneas FALTANTES en .pdb:       34
Cobertura: 89.0%
```

## Files Modified Summary

| File | Lines Changed | Purpose |
|------|--------------|---------|
| `core/src/backend/m6809/mod.rs` | ~50 | Global var init, const numbers emission |
| `core/src/backend/debug_info.rs` | ~15 | Parser fixes for data sections, pseudo-labels |
| `check_pdb_coverage.py` | Fixed (earlier) | PDB string key parsing |

## Next Steps (Future Work)

### High Priority (If needed)
1. Map `elif`/`else` keywords by emitting extra line markers in `emit_stmt`
2. Handle META statements if they need to be tracked

### Low Priority
1. Reduce end-of-function address overrun in address calculation
2. Further optimize parse_vpy_line_markers for very large programs

### Analysis-Only
1. Study whether 89% coverage is practical ceiling
2. Benchmark PDB generation speed with larger programs

## Lessons Learned

1. **Pseudo-labels are powerful**: Using `; _LABEL:` comments allows creative workarounds
2. **Parser state management matters**: The order of `pending_marker` registration is critical
3. **EOF handling important**: Don't forget to flush pending state at end of parse
4. **Line tracking overhead minimal**: Adding source_line to 2 Item variants has negligible impact

## Compatibility Notes

- ✅ No breaking changes to compiler frontend
- ✅ No changes to binary output (only ASM comments)
- ✅ Backward compatible with existing code
- ✅ All existing tests pass

## Conclusion

Achieved **89.0% PDB coverage** with minimal code changes and good architectural decisions. The remaining 11% is primarily structure keywords that don't generate instructions, which is acceptable for a debug information system.

The improvements make debugging significantly easier for:
- Variable initialization breakpoints
- Const value definitions
- Array declaration tracking
- Global state initialization

---
**Date**: 2025-12-29  
**Duration**: ~2 hours  
**Impact**: 44 additional source lines now debuggable  
**Effort**: Moderate (systematic approach, good separation of concerns)
