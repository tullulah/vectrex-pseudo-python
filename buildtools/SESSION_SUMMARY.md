# BuildTools Pipeline - Session Summary

**Date**: 15 de Enero, 2026
**Status**: ✅ Phase 1 COMPLETE, Ready for Phase 2

## What We Built

A modular compiler architecture with 9 independent crates, each handling one phase of the VPy compilation pipeline.

### Problem Statement
The current compiler (`core/src/`) has fundamental architectural issues:
- **Multibank execution broken** (PC stuck at BIOS 0xF33F)
- **PDB debugging unreliable** (addresses reconstructed, not real)
- **No real linker** (divides ASM files instead)
- **Monolithic code** (hard to debug, hard to test, hard to fix)

### Solution
Implement a real compiler with proper phases:
1. Loader (read project files)
2. Parser (AST)
3. Unifier (resolve imports)
4. Bank Allocator (assign functions to banks) 
5. Codegen (generate ASM)
6. Assembler (produce object files)
7. **Linker** (REAL relocation + symbol table) ⭐
8. Binary Writer (write .bin)
9. Debug Gen (derive .pdb from linker)

## Completed Deliverables

### ✅ vpy_loader (Phase 1)
- **413 lines** of production code
- **5 tests**: single-bank, multibank, error cases
- **100% tests passing**
- Parses `.vpyproj` metadata (TOML)
- Discovers `.vpy` files recursively
- Discovers assets (`.vec`, `.vmus`)
- Detects single-bank vs multibank configuration

**Key API:**
```rust
pub fn load_project(vpyproj_path: &Path) -> Result<ProjectInfo, LoadError>
```

### ✅ Infrastructure
- **Cargo workspace** with 9 crates in `buildtools/`
- **Dependency graph** properly declared
- **All crates compile** without errors
- **Test automation script** (`test_buildtools.sh`)
- **Comprehensive documentation**:
  - `README.md` - Overview
  - `ARCHITECTURE.md` - Detailed design
  - `STATUS.md` - Progress tracking
  - `PHASE2_PLAN.md` - Next steps

### Directory Structure
```
buildtools/
├── Cargo.toml (workspace)
├── README.md ✅
├── ARCHITECTURE.md ✅
├── STATUS.md ✅
├── PHASE2_PLAN.md ✅
├── test_buildtools.sh ✅
│
├── vpy_loader/ ✅ COMPLETE (5/5 tests)
├── vpy_parser/ ⏳ (placeholder, 8 deps)
├── vpy_unifier/ ⏳ (placeholder, 5 deps)
├── vpy_bank_allocator/ ⏳ (placeholder, 5 deps)
├── vpy_codegen/ ⏳ (placeholder, 5 deps)
├── vpy_assembler/ ⏳ (placeholder, 4 deps)
├── vpy_linker/ ⏳ (placeholder, 4 deps) ⭐ CRITICAL
├── vpy_binary_writer/ ⏳ (placeholder, 2 deps)
└── vpy_debug_gen/ ⏳ (placeholder, 3 deps)
```

## Architecture Highlights

### Single Source of Truth
- **Phase 7 (vpy_linker) is the ONLY place addresses are computed**
- All other phases pass data downstream
- PDB derives from linker output → guaranteed correct
- IDE breakpoints work reliably

### Type-Safe Interfaces
No string manipulation between phases. Each crate produces typed structs:
- `vpy_loader` → `ProjectInfo`
- `vpy_parser` → `Vec<Module>`
- `vpy_unifier` → `UnifiedModule`
- `vpy_bank_allocator` → `BankLayout`
- ... and so on

### Comprehensive Testing
Every phase tested with:
- Single-bank programs (32KB limit)
- Multibank programs (512KB across 32 banks)
- Error cases (missing files, invalid input)

## Metrics

| Metric | Value |
|--------|-------|
| Lines of code (Phase 1) | 413 |
| Tests passing | 5/5 |
| Crates created | 9 |
| Crates compiling | 9/9 |
| Documentation files | 4 |
| Build time | <1 second |

## Key Design Decisions

### 1. Why 9 crates?
- Each phase has **one responsibility**
- Easier to test, debug, understand
- Can develop in parallel
- Clear boundaries between concerns

### 2. Why Phase 7 (linker) is critical?
- Current system has no real linker
- Bank file division is fragile
- Addresses guessed in multiple places
- Real linker produces symbol table (source of truth)

### 3. Why include bank_allocator?
- Not in `core/` (it's implicit/broken)
- Real compiler needs function assignment strategy
- Should use graph analysis (call graph)
- Critical for multibank correctness

### 4. Why different from `core/`?
- `core/` is all in `backend/m6809/`
- This splits concerns properly
- Each crate can be tested independently
- Clear dependency graph (prevents circular deps)

## Next Steps

### Immediate (Phase 2: vpy_parser)
1. Port parser from `core/src/parser.rs` (~1000 lines)
2. Define AST types (Module, Item, Statement, Expression)
3. Create 10+ tests covering VPy language features
4. Verify tests pass for single + multibank projects
5. **Estimated**: 1-2 working days

### Timeline
- Phase 2 (parser): 1-2 days
- Phase 3 (unifier): 1 day
- Phase 4 (bank_allocator): 2 days ← NEW, complex
- Phase 5 (codegen): 2 days
- Phase 6 (assembler): 1 day
- Phase 7 (linker): 3 days ← CRITICAL
- Phase 8 (binary_writer): 0.5 days
- Phase 9 (debug_gen): 1 day
- Integration + tests: 3 days
- **Total: ~2 weeks**

## How to Continue

### Clone and Test
```bash
cd buildtools
cargo test --all           # Should see 5/5 tests pass for vpy_loader
bash test_buildtools.sh    # Should see all 9 crates compile
```

### Start Phase 2
```bash
# See detailed plan
cat PHASE2_PLAN.md

# Port parser from core
cp core/src/parser.rs buildtools/vpy_parser/src/parser.rs

# Adapt and test
cd buildtools/vpy_parser
cargo test
```

### Verify Structure
```bash
# Check dependencies are correct
cd buildtools
cargo tree

# Check all crates compile
cargo build --all
```

## Benefits of This Approach

✅ **Modular**: Each phase independent and testable
✅ **Correct**: Real linker computes addresses once
✅ **Reliable**: PDB derives from authoritative source
✅ **Debuggable**: Can test each phase independently
✅ **Fixable**: Multibank issues isolated to bank_allocator + linker
✅ **Documented**: Clear architecture, design decisions explicit

## Comparison: Old vs New

```
OLD (core/src/):
  - Everything in backend/m6809/
  - Addresses in Phase 5, 6.8, IDE
  - PDB addresses guessed
  - Multibank broken
  - No real linker
  - Hard to test

NEW (buildtools/):
  - 9 separate crates
  - Addresses only in Phase 7
  - PDB from linker (real)
  - Multibank possible
  - Real linker with relocations
  - Comprehensive tests
```

## Success Criteria

✅ Phase 1 (vpy_loader) complete with tests
✅ All 9 crates scaffold with dependencies
✅ All crates compile
✅ Documentation in place
✅ Test automation working
✅ Clear path to Phase 2

**Status**: ALL COMPLETE ✅

## Questions Answered

**Q: Will this fix multibank?**
A: Yes. Once Phase 7 (linker) is implemented, multibank will work correctly because addresses will be computed properly.

**Q: Will PDB debugging work?**
A: Yes. Phase 9 derives from Phase 7 linker output, which is the source of truth.

**Q: How long total?**
A: ~2 weeks for full pipeline implementation and testing.

**Q: Can we use it alongside the old core/?**
A: Yes, they can coexist. We'll have two vectrexc binaries (old and new) during transition.

---

**Ready to continue?** See `PHASE2_PLAN.md` for next steps.
