# Build Tools Architecture

## Overview

The `buildtools` directory contains a modular compiler pipeline for VPy. Each crate handles one phase of compilation, with clear interfaces and comprehensive tests.

## Design Principles

1. **Single Responsibility**: Each crate does one job
2. **Testability**: Single-bank and multibank configurations tested at every phase
3. **Source of Truth**: The linker (Phase 7) is the only source for final addresses
4. **Clear Interfaces**: Type-safe APIs between crates (no string-based communication)
5. **Debuggability**: Each phase produces human-readable intermediate data

## Pipeline Phases

### Phase 1: vpy_loader
- **Input**: `.vpyproj` + `.vpy` files
- **Output**: `ProjectInfo { metadata, files, assets }`
- **Key Logic**:
  - Parse TOML metadata
  - Discover all `.vpy` files recursively
  - Detect single-bank vs multibank config
  - Calculate number of banks from ROM size

**Tests**:
- Load single-bank project
- Load multibank project (32 banks)
- Handle missing main.vpy
- Discover multiple modules

### Phase 2: vpy_parser
- **Input**: `.vpy` source files
- **Output**: `Vec<Module> { AST }`
- **Status**: Placeholder (will port from core/src/parser.rs)

### Phase 3: vpy_unifier
- **Input**: `Vec<Module>` + import graph
- **Output**: `UnifiedModule { merged items + symbol table }`
- **Status**: Placeholder (will port from core/src/unifier.rs)

### Phase 4: vpy_bank_allocator
- **Input**: `UnifiedModule` + `ProjectMetadata`
- **Output**: `BankLayout { bank assignments + symbol map }`
- **Status**: Placeholder (NEW implementation needed)
- **Key Responsibility**:
  - Graph analysis of function calls
  - Assign functions to minimize cross-bank calls
  - For single-bank: all code in bank 0
  - For multibank: distribute across 32 banks, builtins in bank 31

### Phase 5: vpy_codegen
- **Input**: `UnifiedModule` + `BankLayout`
- **Output**: `GeneratedIR { per-bank ASM + metadata }`
- **Status**: Placeholder (will port from core/src/backend/m6809/)

### Phase 6: vpy_assembler
- **Input**: `GeneratedIR`
- **Output**: `Vec<ObjectFile> { bytes + symbols + relocations }`
- **Status**: Placeholder (will port from core/src/backend/asm_to_binary.rs)

### Phase 7: vpy_linker ⭐ CRITICAL
- **Input**: `Vec<ObjectFile>` + `BankLayout`
- **Output**: `LinkedBinary { final addresses + symbol table }`
- **Status**: Placeholder (NEW implementation needed)
- **Key Responsibility**:
  - Place each bank in address space (0x0000-0x4000 for switchable, 0x4000-0x8000 for fixed)
  - Apply relocations
  - Generate symbol table with FINAL addresses
  - **THIS IS THE SOURCE OF TRUTH** - all other tools derive from this

### Phase 8: vpy_binary_writer
- **Input**: `LinkedBinary`
- **Output**: `.bin` file (on disk)
- **Status**: Complete (simple I/O wrapper)

### Phase 9: vpy_debug_gen
- **Input**: `LinkedBinary` + source maps
- **Output**: `.pdb` file (JSON)
- **Status**: Placeholder (will refactor from core Phase 6.8)
- **Key Responsibility**:
  - Derive symbols from linker output (guaranteed correct)
  - Map addresses to source lines
  - Output JSON for IDE integration

## Testing Strategy

Each phase has tests covering:

### Single-Bank Tests
- Small project (main.vpy only)
- Multiple modules (main + input + graphics)
- Assets (vectors + music)
- Verification: Code fits in 32KB, addresses 0x0000-0x7FFF

### Multibank Tests
- 32-bank configuration
- Functions distributed across banks
- Cross-bank function calls
- Verification: Code fits in 512KB, correct bank assignments, relocations applied

## Building and Testing

```bash
# Test single crate
cd buildtools/vpy_loader
cargo test

# Test all crates
cd buildtools
cargo test --all

# Build entire workspace
cargo build --release
```

## Implementation Order

1. ✅ Setup workspace + vpy_loader (DONE)
2. Port vpy_parser from core/src/parser.rs
3. Port vpy_unifier from core/src/unifier.rs
4. Port vpy_bank_allocator (NEW - from bank assignment logic)
5. Port vpy_codegen from core/src/backend/m6809/
6. Port vpy_assembler from core/src/backend/asm_to_binary.rs
7. Implement vpy_linker (NEW - proper relocation)
8. Integrate vpy_binary_writer
9. Implement vpy_debug_gen (NEW - from linker data)
10. Integrate into vectrexc CLI
11. Full pipeline tests (single + multibank)
12. Compare output with legacy core compiler

## Key Architectural Differences from Current Code

| Aspect | Current (core) | New (buildtools) |
|--------|---|---|
| Address calculation | 3 places (phase 5, 6.8, IDE) | 1 place (linker only) |
| Linker | Divides ASM files | Real relocation + symbol table |
| PDB generation | Reconstructs addresses | Derives from linker |
| Test coverage | Implicit | Explicit single/multi tests |
| Interfaces | String-based | Type-safe structs |

## Next Steps

1. Run tests for vpy_loader to verify single/multibank detection
2. Port vpy_parser (larger task, ~1 day)
3. Setup integration tests that run full pipeline
4. Create test projects for single/multibank validation
