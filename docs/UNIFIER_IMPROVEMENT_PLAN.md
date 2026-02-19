# Unifier Improvement Plan (2026-01-15)

## Problem Statement
- Unifier is identified as a "weak point" of the core
- Basic functionality works, but code quality/coverage is poor
- No comprehensive tests → hidden bugs likely
- Tree shaking disabled "for safety"

## Current State Assessment
- **Lines of Code**: 678 (unifier.rs)
- **Unit Tests**: 3 (only basic ID/naming tests)
- **Integration Tests**: 0 (compile tests in main.rs only)
- **Code Coverage**: ~30% estimated
- **Working Examples**: ✅ (3-module multi-module project compiles)

## Improvement Strategy (4 Phases)

### PHASE A: Improve Test Coverage (Week 1)
**Goal**: Add comprehensive unit tests for all phases

#### A.1: Phase 1 Tests (Export Collection)
```rust
#[test]
fn test_export_collection_explicit() {
    // Module with explicit exports list
    // Verify only exported symbols included
}

#[test]
fn test_export_collection_default() {
    // Module without exports
    // Verify all top-level definitions exported
}

#[test]
fn test_export_collection_mixed() {
    // Multiple modules with different export strategies
    // Verify correct isolation
}
```

#### A.2: Phase 2 Tests (Alias Building)
```rust
#[test]
fn test_import_named_symbol() {
    // from lib import func
    // Verify: alias["func"] = ("lib", "func")
}

#[test]
fn test_import_module() {
    // import lib
    // Verify: module "*" symbols available
}

#[test]
fn test_import_all() {
    // from lib import *
    // Verify: all lib symbols available
}

#[test]
fn test_import_conflict_resolution() {
    // Two modules export same symbol
    // Verify correct module wins (or error)
}

#[test]
fn test_circular_import_detection() {
    // A imports B, B imports A
    // Should fail gracefully with error
}
```

#### A.3: Phase 3 Tests (Name Generation)
```rust
#[test]
fn test_entry_module_no_prefix() {
    // Variables in entry module: no prefix
    // Verify: player_x stays "player_x"
}

#[test]
fn test_imported_module_prefix() {
    // Variables in imported module: get prefix
    // Verify: input module → INPUT_XXX
}

#[test]
fn test_name_collision_handling() {
    // Multiple modules with same variable names
    // Verify: each gets unique prefixed name
}
```

#### A.4: Phase 4 Tests (Item Rewriting)
```rust
#[test]
fn test_function_call_rewriting() {
    // Local call vs imported call
    // Verify correct names used
}

#[test]
fn test_variable_reference_rewriting() {
    // Local ref vs imported ref
    // Verify correct prefixing
}

#[test]
fn test_array_access_rewriting() {
    // array[index] with imported array
    // Verify correct RAM offset generated
}
```

#### A.5: Expression Rewriting Tests
```rust
#[test]
fn test_module_method_pattern() {
    // input.get_input()
    // Verify: transformed to INPUT_GET_INPUT()
}

#[test]
fn test_module_field_pattern() {
    // input.input_result
    // Verify: transformed to INPUT_INPUT_RESULT
}

#[test]
fn test_nested_field_not_supported() {
    // module.submodule.field
    // Should error or skip with warning
}

#[test]
fn test_module_field_array_access() {
    // input.result[0]
    // Verify: correct offset calculation
}
```

**Effort**: ~40 lines of test code per category = ~200 lines total  
**Time**: 2-3 hours

---

### PHASE B: Refactor Complex Logic (Week 2)
**Goal**: Improve code clarity and reduce bugs

#### B.1: Extract module pattern detection
Current: ~30 lines of inline logic in `rewrite_expr`
Proposal: Create dedicated function
```rust
fn is_module_method_call(expr: &Expr, aliases: &HashMap<String, ...>) -> Option<(String, String)> {
    // Returns (module, method) if matches pattern
    // Centralizes complex detection logic
}
```

#### B.2: Simplify identifier resolution
Current: 20-line function with 3-step fallback
Proposal: Use match statement instead of if-chain
```rust
fn resolve_identifier(name: &str, ...) -> String {
    match (check_alias(name), check_current_module(name)) {
        (Some(alias), _) => apply_alias(alias),
        (_, Some(local)) => local,
        _ => name.to_string(),
    }
}
```

#### B.3: Add error handling for edge cases
Current: Silent failures
Proposal: Return Result<> for phases 2-4
```rust
fn build_import_aliases(...) -> Result<HashMap<...>, UnifyError> {
    // Better error reporting
    // Circular import detection
    // Name conflict detection
}
```

**Effort**: ~100 lines of refactored code  
**Time**: 3-4 hours

---

### PHASE C: Feature Improvements (Week 3)
**Goal**: Fix/complete incomplete features

#### C.1: Investigate tree shaking
Current: Disabled by default
Action:
1. Read code to understand why disabled
2. Create test cases for tree shaking
3. Fix or document limitation

#### C.2: Add circular import detection
Current: Not detected, may cause issues
Action:
1. Track import graph during Phase 2
2. Detect cycles using DFS
3. Return meaningful error message

#### C.3: Add name conflict detection  
Current: Silent handling, undefined behavior
Action:
1. Track all exported symbols per module
2. Detect conflicts during name generation (Phase 3)
3. Report which module wins/loses

**Effort**: ~150 lines of new code  
**Time**: 4-5 hours

---

### PHASE D: Documentation & Validation (Week 4)
**Goal**: Document behavior, add to testing

#### D.1: Document limitations explicitly
Add to unifier module docs:
- ✅ Supported: Named imports, module imports, all imports
- ✅ Supported: Multi-module with dot notation
- ✅ Supported: Array field access
- ❌ Not supported: Re-exports (import then export)
- ❌ Not supported: Nested imports (import in functions)
- ⚠️ Disabled: Tree shaking (needs testing)

#### D.2: Add examples for each phase
```rust
/// # Examples
/// 
/// ## Named Import
/// ```vpy
/// from math import add
/// result = add(5, 3)
/// ```
/// 
/// ## Module Import
/// ```vpy
/// import math
/// result = math.add(5, 3)
/// ```
```

#### D.3: Create integration test suite
File: `core/tests/unifier_integration.rs`
- Multi-module compile tests
- Error case tests (circular imports, conflicts)
- Performance tests (large module graphs)

**Effort**: ~100 lines of docs + tests  
**Time**: 2-3 hours

---

## Implementation Checklist

### PHASE A: Test Coverage
- [ ] Phase 1 (Export Collection) - 3 tests
- [ ] Phase 2 (Alias Building) - 5 tests
- [ ] Phase 3 (Name Generation) - 3 tests
- [ ] Phase 4 (Item Rewriting) - 3 tests
- [ ] Expression Rewriting - 4 tests
- [ ] Run all tests, verify passing

### PHASE B: Refactoring
- [ ] Extract `is_module_method_call()` function
- [ ] Refactor `resolve_identifier()` with match
- [ ] Add error handling to Phase 2-4
- [ ] Test refactored code with existing tests

### PHASE C: Features
- [ ] Investigate tree shaking
- [ ] Implement circular import detection
- [ ] Implement name conflict detection
- [ ] Document each feature

### PHASE D: Documentation
- [ ] Write limitation docs
- [ ] Add code examples
- [ ] Create integration test suite
- [ ] Update PHASE3_SUMMARY.md

---

## Success Criteria

✅ 30+ new unit tests (10x current coverage)  
✅ All tests passing  
✅ Code coverage >80%  
✅ Circular import detection  
✅ Name conflict detection  
✅ Explicit limitation documentation  
✅ Refactored code with improved readability  
✅ Tree shaking status clarified

---

## Estimated Timeline
- **Total Effort**: 11-15 hours
- **PHASE A**: 2-3 hours
- **PHASE B**: 3-4 hours
- **PHASE C**: 4-5 hours
- **PHASE D**: 2-3 hours

## Risk Assessment
- **Low Risk**: Adding tests (Phase A)
- **Medium Risk**: Refactoring (Phase B) - may break existing functionality
- **Medium Risk**: New features (Phase C) - circular detection may be complex
- **Low Risk**: Documentation (Phase D)

**Mitigation**: Run full compiler tests after each phase, verify multi-module example still works

---

## Potential Issues to Watch

1. **Phase 2 - Import Resolution**:
   - What happens if import fails silently?
   - How does module loader handle missing files?

2. **Phase 3 - Name Collision**:
   - Two modules export `counter` → What happens?
   - Current code appears to allow duplication

3. **Phase 4 - Complex Expression Rewriting**:
   - Nested module references (already not supported)
   - Assignment to module fields with index

4. **Tree Shaking**:
   - Why disabled? Scope issue? Incomplete?
   - Worth investigating or document as deprecated

---

## Next Steps
1. Start with PHASE A (lowest risk, high impact)
2. Get 30+ tests written and passing
3. Then proceed to PHASE B (refactoring)
4. Validate with multi-module example after each phase
5. Document findings in PHASE3_SUMMARY.md
