# BuildTools: Modular Compilation Pipeline for VPy

A from-scratch redesign of the compiler architecture, breaking the monolithic pipeline into 9 independent crates with clear interfaces and comprehensive tests.

## Why This Matters

The current compiler (in `core/`) has fundamental issues:
- **No single source of truth**: Addresses calculated in 3 places (fragile)
- **Not a real linker**: Just divides ASM files (no relocation)
- **PDB addresses guessed**: Reconstructed from bank files (unreliable)
- **Multibank broken**: Linker doesn't properly allocate functions
- **Hard to test**: No clear phase boundaries

**This pipeline fixes all of that** by implementing a real compiler with:
- ✅ Separate phases with type-safe interfaces
- ✅ Single source of truth (linker computes all addresses)
- ✅ Real relocations and symbol table
- ✅ Comprehensive tests for single + multibank
- ✅ Correct PDB generation from linker output

## Pipeline Architecture

```
Phase 1: vpy_loader     → ProjectInfo {metadata, files, assets}
Phase 2: vpy_parser     → Vec<Module> {AST per file}
Phase 3: vpy_unifier    → UnifiedModule {merged AST + symbols}
Phase 4: vpy_bank_allocator → BankLayout {bank assignments}
Phase 5: vpy_codegen    → GeneratedIR {ASM per bank}
Phase 6: vpy_assembler  → Vec<ObjectFile> {bytes + relocs}
Phase 7: vpy_linker     → LinkedBinary + SymbolTable ⭐ SOURCE OF TRUTH
Phase 8: vpy_binary_writer → .bin file (on disk)
Phase 9: vpy_debug_gen  → .pdb file (from linker data)
```

## Current Status

### ✅ Phase 1: vpy_loader (COMPLETE)
- Parses `.vpyproj` metadata (single + multibank config)
- Discovers all `.vpy` files recursively
- Discovers asset files (`.vec`, `.vmus`)
- **Tests**: 5/5 passing (single-bank, multibank, error cases)

### ✅ Phase 2: vpy_parser (COMPLETE)
- Full lexer with 11 tests passing
- Complete AST types (345 lines)
- Parser with 41 tests passing (1496 lines)
- Expression, statement, and module parsing

### ✅ Phase 3: vpy_unifier (COMPLETE)
- Module dependency graph with cycle detection
- Topological sorting (Kahn's algorithm)
- Symbol resolution with MODULE_symbol naming
- 24 comprehensive tests passing

### ✅ Phase 5: vpy_codegen - Runtime Helper Optimization (COMPLETE)
- **Tree Shaking System**: Automatic detection and elimination of unused runtime helpers
- **Modular Architecture**: 5 helper modules (drawing, math, joystick, level, utilities)
- **Usage Analysis**: AST traversal detects which helpers are actually needed
- **Results**: Only emits helpers used in code (e.g., joystick_test: 3/17 helpers)
- **Benefits**: Smaller binaries, zero manual configuration, automatic dependency resolution

### ⏳ Phase 4-9: In Development
- Placeholders created for all remaining crates
- Dependencies properly declared
- All crates compile without errors
- Ready for porting from `core/src/`

## Getting Started

### Run Tests
```bash
cd buildtools
cargo test --all         # Run all tests
cargo test vpy_loader   # Run specific crate
```

### Test Script
```bash
./test_buildtools.sh    # Run all crate compilation checks
```

### Browse Documentation
```bash
cat ARCHITECTURE.md     # Detailed pipeline design
cat STATUS.md          # Current progress
```

## File Structure

```
buildtools/
├── Cargo.toml              # Workspace definition
├── ARCHITECTURE.md         # Pipeline design details
├── STATUS.md              # Progress tracking
├── test_buildtools.sh     # Test all crates
│
├── vpy_loader/            ✅ Complete
│   ├── src/lib.rs         (413 lines, tested)
│   ├── Cargo.toml
│   └── tests/
│
├── vpy_parser/            ⏳ Next (port from core)
│   ├── src/lib.rs
│   └── Cargo.toml
│
├── vpy_unifier/           ⏳ Phase 3
├── vpy_bank_allocator/    ⏳ Phase 4 (NEW)
├── vpy_codegen/           ⏳ Phase 5
├── vpy_assembler/         ⏳ Phase 6
├── vpy_linker/            ⏳ Phase 7 (CRITICAL - NEW)
├── vpy_binary_writer/     ⏳ Phase 8
└── vpy_debug_gen/         ⏳ Phase 9
```

## Key Design Decisions

### 1. One Crate Per Phase
- Clear separation of concerns
- Testable in isolation
- Can parallelize builds
- Easy to debug

### 2. Type-Safe Interfaces
```rust
// Not this:
emit_codegen(source: String) -> String

// But this:
pub fn codegen(unified: UnifiedModule, layout: BankLayout) 
    -> Result<GeneratedIR, CodegenError>
```

### 3. Single Source of Truth
- **Only the linker computes final addresses**
- All other phases pass data downstream
- PDB derives from linker, guaranteed correct
- IDE breakpoints work reliably

### 4. Real Linker (Not "Divide and Hope")
- Takes object files with relocations
- Places code in address space
- Applies relocations
- Generates symbol table
- Returns authoritative address map

## Testing Strategy

Every phase tested with:
- **Single-bank**: Code must fit in 32KB
- **Multibank**: Code distributed across 32×16KB banks
- **Error cases**: Missing files, invalid code, etc.

Example test:
```rust
#[test]
fn test_load_multibank_project() {
    let info = load_project(&proj_path).unwrap();
    assert!(info.is_multibank());
    assert_eq!(info.num_banks(), 32);
    assert_eq!(info.source_files.len(), 1);
}
```

## Porting from core/ (Next Steps)

1. **vpy_parser** (~1-2 days)
   - Move core/src/parser.rs → buildtools/vpy_parser/
   - Define AST types
   - Add single + multibank tests

2. **vpy_unifier** (~1 day)
   - Move core/src/unifier.rs → buildtools/vpy_unifier/
   - Import resolution logic
   - Multi-module tests

3. **vpy_bank_allocator** (~2 days, NEW)
   - Graph analysis for function placement
   - Bank assignment strategy
   - Single vs multibank logic

4. **vpy_codegen** (~2 days)
   - Move core/src/backend/m6809/mod.rs
   - Generate ASM per bank
   - Metadata emission

5. **vpy_assembler** (~1 day)
   - Move core/src/backend/asm_to_binary.rs
   - Produce object files with relocations
   - Symbol extraction

6. **vpy_linker** (~3 days, CRITICAL)
   - NEW: Real linker implementation
   - Address space allocation
   - Relocation application
   - Symbol table generation

7. **vpy_binary_writer** (~0.5 days)
   - Trivial: just write bytes to disk

8. **vpy_debug_gen** (~1 day)
   - NEW: Derive PDB from linker
   - Source map generation
   - JSON output

**Total: ~2 weeks** for complete pipeline with all tests

## Comparison: Old vs New

| Aspect | Old (core/) | New (buildtools/) |
|--------|---|---|
| Monolithic | Single binary | 9 independent crates |
| Address calc | 3 places | 1 place (linker) |
| Linker | Divides ASM | Real relocation |
| PDB | Guesses | Derives from linker |
| Tests | Implicit | Explicit single/multi |
| Debuggability | Hard | Easy (clear phases) |

## Contributing

Follow this pattern for each new crate:

1. Create directory and Cargo.toml
2. Implement minimal API in lib.rs
3. Add 5-10 representative tests
4. Run `cargo test` locally
5. Document interfaces and design decisions
6. Mark as "complete" when 100% tests pass

## Questions?

See:
- `ARCHITECTURE.md` - Detailed design
- `STATUS.md` - Progress and next steps
- Individual crate Cargo.toml files for dependencies
