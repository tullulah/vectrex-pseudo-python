# Next Steps: Phase 2 (vpy_parser)

## Summary of What We've Done

✅ Created modular compiler architecture in `buildtools/`
✅ Implemented Phase 1 (vpy_loader) with 5 passing tests
✅ Scaffolded Phases 2-9 with proper dependencies
✅ All 9 crates compile successfully
✅ Created documentation (README, ARCHITECTURE, STATUS)
✅ Created test automation script

## Current State

```
✅ Phase 1: vpy_loader      (413 lines, 5 tests, COMPLETE)
⏳ Phase 2: vpy_parser       (Placeholder, NEXT)
⏳ Phase 3: vpy_unifier      (Placeholder)
⏳ Phase 4: vpy_bank_allocator (Placeholder, NEW)
⏳ Phase 5: vpy_codegen      (Placeholder)
⏳ Phase 6: vpy_assembler    (Placeholder)
⏳ Phase 7: vpy_linker       (Placeholder, CRITICAL)
⏳ Phase 8: vpy_binary_writer (Placeholder)
⏳ Phase 9: vpy_debug_gen    (Placeholder, NEW)
```

## Why This Matters

The current compiler architecture is fundamentally broken:
- **Multibank execution fails** (PC stuck at 0xF33F BIOS)
- **PDB debugging unreliable** (addresses reconstructed, not real)
- **No linker** (just divides ASM files)
- **No symbol table** (guessed from banks)
- **Hard to fix** (monolithic, interdependent code)

This pipeline solves all of that by:
- **Real compiler phases** (clear separation)
- **Real linker** (addresses computed once)
- **Source of truth** (only linker computes addresses)
- **Testable** (each phase has tests)
- **Fixable** (can debug one phase at a time)

## Next Work: Phase 2 (vpy_parser)

### Goal
Port the VPy parser from `core/src/parser.rs` to a standalone crate with proper AST types.

### Current State in core/
- Location: `core/src/parser.rs` (~1000 lines)
- Parses `.vpy` source to AST
- Types used: `Module`, `Item` (enum), `Expression`, `Statement`
- Already working (used by current compiler)

### Porting Plan

#### Step 1: Examine existing parser
```bash
wc -l /Users/daniel/projects/vectrex-pseudo-python/core/src/parser.rs
grep "^pub " core/src/parser.rs | head -20
```

#### Step 2: Define AST types in buildtools/vpy_parser/src/lib.rs
```rust
// Minimal but complete AST structure
pub struct Module {
    pub name: String,
    pub items: Vec<Item>,
}

pub enum Item {
    FunctionDef { name, params, body },
    VarDecl { name, value },
    // Add more as needed
}

pub enum Statement {
    If { cond, body, else_body },
    While { cond, body },
    Assignment { target, value },
    // etc.
}

pub enum Expression {
    Number(i32),
    Ident(String),
    BinOp { left, op, right },
    Call { name, args },
    // etc.
}
```

#### Step 3: Port parser logic
- Copy parser functions from core/src/parser.rs
- Adapt to use new AST types
- Ensure same behavior

#### Step 4: Create comprehensive tests
- Test single-bank programs (simple)
- Test multibank programs (with imports)
- Test error cases (syntax errors, missing brackets, etc.)
- Test all language features (if/while, functions, arrays, etc.)

#### Step 5: Verify tests pass
```bash
cd buildtools/vpy_parser
cargo test
```

### Estimated Effort
- **Examination**: 30 min
- **AST type definition**: 1 hour
- **Porting logic**: 2-3 hours (most work is copy-paste + adapt)
- **Creating tests**: 1-2 hours
- **Debugging**: 1 hour
- **Total**: 6-8 hours (~1 working day)

### Success Criteria
1. All parser code ported from core/
2. 10+ tests covering various VPy constructs
3. Tests pass for both single + multibank projects
4. Same behavior as original parser verified by comparison tests

### Files to Create/Modify

**buildtools/vpy_parser/src/lib.rs**
- AST types (Module, Item, Statement, Expression enums)
- Lexer functions (scan tokens)
- Parser functions (parse_module, parse_item, parse_expr, etc.)
- Error handling (ParseError enum)

**buildtools/vpy_parser/tests/integration_tests.rs** (optional)
- Load real .vpy files
- Parse and verify AST structure
- Test both single + multibank projects

### Dependencies
Currently:
```toml
[dependencies]
thiserror = "1.0"
```

May need to add:
- `regex` (if using for tokenizing)
- (others depend on parser implementation)

### Key Questions to Answer

1. **How does the current parser handle imports?**
   - Does it parse `import x` statements?
   - Or is that handled by unifier?

2. **What AST types are actually used?**
   - Is enum-based `Item` sufficient?
   - Are there nested structures?

3. **Error recovery?**
   - Should parser report multiple errors or fail on first?
   - How much error recovery is needed for IDE?

### After Phase 2 Complete

Next crate to port: vpy_unifier
- Resolves imports
- Merges modules
- Creates symbol table

This depends on vpy_parser output, so it can start once Phase 2 is done.

## Testing Approach (All Phases)

Each phase will have this test structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Single-bank tests
    #[test]
    fn test_simple_vpy_program() {
        // Load examples/simple/src/main.vpy
        // Parse it
        // Verify AST structure
    }
    
    // Multibank tests
    #[test]
    fn test_multimodule_project() {
        // Load examples/multi-module/src/
        // Parse all modules
        // Verify imports resolved
    }
    
    // Error tests
    #[test]
    fn test_invalid_syntax() {
        // Feed malformed input
        // Verify error is reported
    }
}
```

## Timeline

If working sequentially (~2 weeks total):

- **Day 1-2**: Phase 2 (vpy_parser) ← YOU ARE HERE
- **Day 3**: Phase 3 (vpy_unifier)
- **Day 4-5**: Phase 4 (vpy_bank_allocator) - NEW, most complex
- **Day 5-6**: Phase 5 (vpy_codegen)
- **Day 6-7**: Phase 6 (vpy_assembler)
- **Day 7-10**: Phase 7 (vpy_linker) - CRITICAL, needs real relocation logic
- **Day 11**: Phase 8 (vpy_binary_writer)
- **Day 12**: Phase 9 (vpy_debug_gen)
- **Day 13-14**: Integration + comparison tests

## Questions Before Starting Phase 2?

1. Should we start with Phase 2 now or wait?
2. Want me to examine current parser first to understand scope?
3. Preferred: Exact port or refactor parser during porting?
4. How deep should tests go (full grammar or just common cases)?

---

**Status**: Ready to start Phase 2. All infrastructure in place. 🚀
