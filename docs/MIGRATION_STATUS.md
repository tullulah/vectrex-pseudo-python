# Migration Status: Core to Buildtools

**Date**: January 18, 2026
**Status**: Critical Gaps Identified

## 1. Overview

The migration from the monolithic `core` compiler to the modular `buildtools` architecture is incomplete. While the basic structure is in place, significant functionality, data structures, and validation provided by `core` have been dropped or incompletely ported.

## 2. RAM Allocation Gap Analysis

`core` allocated variables based on semantic analysis (`rt_usage`). `buildtools` uses a simplified string-based check (`needed`) which misses critical allocations.

| Variable | Core Allocation | Buildtools Allocation | Status | Impact |
|----------|-----------------|-----------------------|--------|--------|
| `RESULT` | Always | Always | ✅ | OK |
| `TMPPTR` | Always | Always | ✅ | OK |
| `TMPLEFT` | Conditional (BinOp) | **MISSING** | ❌ | Binary ops may fail or clobber |
| `TMPRIGHT` | Conditional (BinOp) | **MISSING** | ❌ | Binary ops may fail or clobber |
| `TMPLEFT2` | Conditional (Nested) | **MISSING** | ❌ | Nested ops failure |
| `TMPRIGHT2` | Conditional (Nested) | **MISSING** | ❌ | Nested ops failure |
| `MUL_A/B/RES/TMP/CNT`| Conditional (Mul) | **MISSING** | ❌ | Multiplication fails (Assembler error) |
| `DIV_A/B/Q/R` | Conditional (Div) | **MISSING** | ❌ | Division fails (Assembler error) |
| `TEMP_YX/X/Y` | Always | `TEMP_YX` only | ⚠️ | Partial compatibility |
| `PSG_*` (Audio) | If assets exist | If audio calls exist | ✅ | OK (Logic differs but functional) |
| `SFX_*` (Audio) | If assets exist | If audio calls exist | ✅ | OK |
| `DRAW_VEC_*` | Conditional | **FIXED** (2026-01-18) | ✅ | Was missing, now present |
| `NUM_STR` | Always | Conditional | ⚠️ | Core allocates always, Buildtools conditional |

**Critical Action**: `MUL_*` and `DIV_*` variables must be added to `generate_ram_and_arrays` in `helpers.rs`. `TMPLEFT/RIGHT` must be added if binary operations use them (check `expressions.rs`).

## 3. Builtin Compatibility Matrix

`core` maintains a strict `BUILTIN_ARITIES` table. `buildtools` implemented this recently (2026-01-18) but integration is pending.

| Builtin | Core Behavior | Buildtools Behavior | Status |
|----------|---------------|---------------------|--------|
| `DRAW_VECTOR` | 3 args (name, x, y) | **FIXED** (was 1 arg allowed) | ✅ |
| `DRAW_VECTOR_EX` | 5 args | **FIXED** (was variable args) | ✅ |
| `PRINT_TEXT` | 3 or 5 args | 3 args (hardcoded) | ⚠️ |
| `PRINT_NUMBER` | 3 args | 3 args | ✅ |
| `DRAW_LINE` | 5 args | 5 args | ✅ |
| `J1_X/Y` | 0 args | 0 args | ✅ |
| `RAND` | 0 args | 0 args | ✅ |

**Note**: `buildtools` relies on manual `panic!` checks or silent emission in many places. The centralized validation system ported from `core` needs to be fully activated.

## 4. Helper Function Analysis

`core` generates helpers in `backend/m6809/helpers.rs`. `buildtools` has a similar file but implementation differs.

| Helper | Core Logic | Buildtools Logic | Discrepancy |
|--------|------------|------------------|-------------|
| `MUL16` | Standard shift-add | **MISSING** | **CRITICAL**: Code not emitted |
| `DIV16` | Standard subtract | **MISSING** | **CRITICAL**: Code not emitted |
| `MOD16` | (Likely via DIV) | **MISSING** | **CRITICAL**: Code not emitted |
| `VECTREX_PRINT_TEXT` | Sets VIA=$98 | Sets VIA=$98 | Core assumes valid state? |
| `DRAW_LINE_WRAPPER` | Checks constants >127 | Checks constants >127 | Logic seems ported |

**Impact**: Any code using `*`, `/`, or `%` with variables will fail at assembly time (Undefined symbol `MUL16`/`DIV16`).

## 5. Missing Validation Features

1.  **Arity Validation**: `core` validates all 50+ builtins. `buildtools` has just imported the table but integration is pending.
2.  **Type Checking**: `core` has some basic type inference/checking in `analysis.rs`. `buildtools` largely assumes correct input or fails at ASM time.
3.  **Symbol Resolution**: `core` resolves symbols (constants) early. `buildtools` relies on Assembler.

## 6. Action Plan

1.  **Fix RAM Allocations**: Update `helpers.rs` to allocate `MUL_*`, `DIV_*`, `TMPLEFT`, `TMPRIGHT` when needed.
2.  **Activate Arity Validation**: Call `validate_builtin_arity` in `emit_builtin`.
3.  **Port Missing Constants**: Ensure `VIA_*` constants are available.
4.  **Verify Binary Ops**: Check `expressions.rs` in `buildtools` to see if it uses `TMPLEFT`/`TMPRIGHT` or `TMPPTR`. If it uses `TMPPTR` for everything, it risks corruption.
