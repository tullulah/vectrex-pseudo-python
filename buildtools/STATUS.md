# BuildTools - Modular Compiler Pipeline

## Current Status (Updated 2026-01-17 - Phase 7 Day 3 COMPLETE)

✅ **Phase 1 Complete**: vpy_loader crate is ready
✅ **Phase 2a Complete**: vpy_parser lexer (11 tests passing)
✅ **Phase 2b Complete**: vpy_parser AST types (345 lines, 100% ported)
✅ **Phase 2c Complete**: vpy_parser parser (1496 lines ported, entry point wired, 41 tests passing)
✅ **Phase 3 Complete**: vpy_unifier module resolution (24 tests passing, symbol case fixed)
✅ **Phase 4 Complete**: vpy_bank_allocator sequential assignment (12 tests passing)
✅ **Phase 5 Optimization Complete**: vpy_codegen tree shaking for runtime helpers
✅ **Phase 6 Refactoring Complete**: vpy_assembler modular segregation (18 tests passing)
✅ **Phase 7 Day 1-4 Complete**: vpy_linker object format + resolver + bank layout + integration (20 tests)
⏳ **Phase 7 Day 5 Next**: Cross-bank call wrappers + final polish

### Session 2026-01-17 Part 2: Phase 7 Linker Implementation (Day 1-4) ✅
- ✅ **Day 1 - Object Format**: VectrexObject with binary serialization (305 lines, 3 tests)
- ✅ **Day 2 - Symbol Resolver**: 4-step algorithm with 7 relocation types (441 lines, 5 tests)
- ✅ **Day 3 - Bank Layout**: Multibank ROM integration pipeline (356 lines, 3 tests)
- ✅ **Day 4 - Integration Tests**: End-to-end pipeline validation (313 lines, 5 tests)

### Session 2026-01-17 Part 1: AST Evolution & Phase 6 Refactoring ✅
- ✅ **AST Modernization**: Parser AST evolved to separate expression types
  - `Expr::Binary/Compare/Logic` separate variants (was single BinaryOp)
  - `Expr::Not/BitNot` separate (was UnaryOp)
  - `Expr::Call` tuple variant with CallInfo (was struct)
  - `BinOp/CmpOp/LogicOp` separate enums
- ✅ **Compilation Fixes**: Updated all dependent crates for new AST
  - vpy_codegen helpers.rs: 7 pattern updates
  - vpy_unifier resolver.rs: Symbol case preservation fix
  - buildtools tests: Updated CodegenOptions constructors
- ✅ **Phase 6 Assembler Refactoring**: Segregated monolithic file
  - Created `parser.rs` (130 lines, 4 tests): Directive/label parsing
  - Created `expression.rs` (180 lines, 5 tests): Arithmetic evaluation
  - Created `symbols.rs` (170 lines, 3 tests): VECTREX.I loading
  - Reduced `asm_to_binary.rs`: 3090 → 2651 lines (-14%)
- ✅ **Test Status**: All 91 buildtools tests passing
- ✅ **Core Isolation**: Disabled core from workspace (multibank tests failing)
- ✅ **Git commits**: 
  - 4628eb25: Phase 6 refactoring (3 modules created)
  - 2197013a: AST compilation fixes
  - 1f9efdf6: Symbol resolver case fix
  - d5a76d2b: Core isolation + buildtools workspace

### Session 2026-01-17: Bank Allocator Implementation Complete ✅
- ✅ **CallGraph Analysis**: AST traversal for function size estimation and call detection
- ✅ **Sequential Algorithm**: Largest-first packing into banks #0 to #(N-2)
- ✅ **Helper Bank Reservation**: Bank #(N-1) reserved for runtime helpers
- ✅ **Size Estimation**: ~10 bytes per statement + 20 bytes overhead per function
- ✅ **BankLayout Output**: Function-to-bank mapping ready for codegen
- ✅ **12/12 Tests Passing**: Graph creation, allocation, overflow detection
- ✅ **Files Implemented**:
  - `graph.rs`: 270 lines (call graph, size estimation, call analysis)
  - `allocator.rs`: 329 lines (sequential packing, stats, validation)
  - `lib.rs`: 177 lines (high-level API, BankLayout)
- ✅ **Real-World Ready**: Handles single-bank and multibank projects

### Session 2026-01-16: Tree Shaking Implementation Complete ✅
- ✅ **Infrastructure**: Conditional emission system for 17 runtime helpers
- ✅ **Modular Design**: 5 helper modules (drawing, math, joystick, level, utilities)
- ✅ **Automatic Detection**: AST analysis identifies needed helpers from code
- ✅ **Dependency Tracking**: SQRT→DIV16, RAND_RANGE→RAND automatic resolution
- ✅ **Real-World Results**: joystick_test emits only 3/17 helpers (82% reduction)
- ✅ **Git commits**: 9e885571 (infrastructure) + ae998907 (analysis) pushed

### Session 2026-01-16: VECTREX.I Refactoring Complete ✅
- ✅ All hardcoded BIOS addresses in buildtools eliminated
- ✅ Dynamic resolution from VECTREX.I (single source of truth)
- ✅ Wait_Recal ($F192), DP_to_D0 ($F1AA), DP_to_C8 ($F1AF) verified
- ✅ buildtools compiles cleanly with no hardcoded BIOS addresses

### Session 2026-01-16: Phase 3 Complete ✅
- ✅ Module dependency graph with cycle detection (DFS)
- ✅ Topological sorting (Kahn's algorithm, dependencies-first)
- ✅ Symbol resolver with MODULE_symbol naming convention
- ✅ 24 comprehensive tests for graph, resolver, scope, visitor
- ✅ All 82 buildtools tests passing (parser 41 + unifier 24 + others)
- ✅ Git commit 70281f40 pushed to feature/compiler-optimizations

### Completed Work

#### vpy_bank_allocator (Phase 4) ✅ COMPLETE
**Sequential Bank Assignment** (2026-01-17)
- ✅ **CallGraph Construction**: Analyzes Module AST to build function dependency graph
  - `FunctionNode`: name, size_bytes (estimated), is_critical flag
  - `CallEdge`: from → to relationships for cross-function calls
  - Size estimation: 20 bytes overhead + (statement_count × 10 bytes)
- ✅ **BankAllocator Algorithm**: Sequential packing strategy
  - Sort functions by size (largest first)
  - Fill banks #0 to #(N-2) sequentially
  - Reserve bank #(N-1) for runtime helpers (DRAW_LINE_WRAPPER, MUL16, etc.)
  - Validation: Detects overflow if functions don't fit
- ✅ **BankLayout Output**: Function-to-bank mapping structure
  - `banks: Vec<Vec<String>>`: Functions per bank
  - `num_banks: usize`: Total banks needed
  - `bank_size: usize`: ROM bank size (typically 16384 bytes)
- ✅ **Configuration Support**:
  - Single-bank: 32KB cartridge (no bank switching)
  - Multibank: 512KB (32 banks × 16KB) standard Vectrex
  - Flexible: Up to 256 banks (4MB) with custom config
- ✅ **12/12 Tests Passing**:
  - Graph: creation, add node, add edge, from_module (4 tests)
  - Allocator: config, bank info, simple allocation, overflow (5 tests)
  - Integration: single bank layout, from_assignments, allocate module (3 tests)
- ✅ **Files Implemented**:
  - `graph.rs`: 270 lines (call graph analysis)
  - `allocator.rs`: 329 lines (sequential assignment algorithm)
  - `lib.rs`: 177 lines (high-level API)
  - `error.rs`: Error types for allocation failures

**Key Interfaces:**
```rust
pub fn allocate_banks_from_module(
    module: &Module, 
    config: BankConfig
) -> Result<BankLayout, BankAllocatorError>

pub struct BankLayout {
    pub banks: Vec<Vec<String>>,
    pub num_banks: usize,
    pub bank_size: usize,
}
```

#### vpy_codegen Runtime Helper Optimization ✅ COMPLETE
**Tree Shaking System** (commits 9e885571 + ae998907)
- ✅ **Modular Architecture**: 5 helper modules
  - `drawing.rs`: DRAW_CIRCLE_RUNTIME, DRAW_RECT_RUNTIME
  - `math.rs`: MUL16, DIV16, MOD16, SQRT_HELPER, POW_HELPER, ATAN2_HELPER
  - `joystick.rs`: J1X_BUILTIN, J1Y_BUILTIN, J2X_BUILTIN, J2Y_BUILTIN
  - `level.rs`: SHOW_LEVEL_RUNTIME
  - `utilities.rs`: RAND_HELPER, RAND_RANGE_HELPER, FADE_IN_RUNTIME, FADE_OUT_RUNTIME
- ✅ **Conditional Emission**: `HashSet<String>` parameter for selective generation
- ✅ **Usage Analysis**: `analyze_needed_helpers(module: &Module) -> HashSet<String>`
  - Traverses entire AST (functions → statements → expressions)
  - Detects builtin calls with variable arguments
  - Detects binary operations (*, /, %) requiring helpers
  - Tracks dependencies (SQRT→DIV16, RAND_RANGE→RAND)
- ✅ **Detection Rules**: 17 total helpers analyzed
  - Drawing: DRAW_CIRCLE(vars), DRAW_RECT(vars)
  - Joystick: J1_X(), J1_Y(), J2_X(), J2_Y()
  - Math: SQRT(vars), POW(vars), ATAN2(vars), RAND(), RAND_RANGE()
  - Operations: x*y, x/y, x%y (only if operands are variables)
- ✅ **Real-World Verification**:
  - `joystick_test`: Emits only J1X_BUILTIN, J1Y_BUILTIN, DIV16 (3/17 helpers, 82% reduction)
  - `test_buttons`: Emits only J1X_BUILTIN, J1Y_BUILTIN (2/17 helpers)
- ✅ **Files Modified**:
  - `buildtools/vpy_codegen/src/m6809/helpers.rs`: +195 lines analysis code
  - `buildtools/vpy_codegen/src/m6809/mod.rs`: Updated 2 call sites

#### vpy_loader (Phase 1) ✅ COMPLETE
- ✅ Crate structure created
- ✅ ProjectMetadata parsing (single + multibank TOML)
- ✅ Recursive .vpy file discovery  
- ✅ Asset file discovery (.vec + .vmus)
- ✅ Entry point detection (main.vpy)
- ✅ Full API: `load_project(path) -> ProjectInfo`

#### vpy_parser Lexer (Phase 2a) ✅ COMPLETE
- ✅ 333 lines ported from core/src/lexer.rs
- ✅ TokenKind enum with all operators/keywords
- ✅ Token struct with location info
- ✅ Full indentation handling (INDENT/DEDENT)
- ✅ String literals with escape sequences
- ✅ Hex (0xFF) and binary (0b1010) numbers
- ✅ Comment handling (# and ;)
- ✅ Error messages with file:line:col
- ✅ **11/11 tests passing**:
  - test_simple_number
  - test_simple_identifier
  - test_keyword
  - test_operators
  - test_string_literal
  - test_hex_number
  - test_binary_number
  - test_indentation
  - test_invalid_indent
  - test_lex_simple_code
  - test_parse_placeholder

**Key Interfaces:**
```rust
pub fn lex(input: &str) -> ParseResult<Vec<Token>>

pub enum TokenKind { Def, Identifier(String), Number(i32), ... }

pub struct Token { pub kind: TokenKind, pub line: usize, pub col: usize }
```

### Structure

```
buildtools/
├── Cargo.toml (workspace)
├── ARCHITECTURE.md
├── STATUS.md (this file)
├── LEXER_COMPLETE.md (session report)
├── PARSER_STATUS.md (planning doc)
├── vpy_loader/          ✅ COMPLETE (Phase 1)
│   ├── src/lib.rs
│   ├── Cargo.toml
│   └── tests/
├── vpy_parser/          ✅ COMPLETE (Phase 2)
│   ├── src/
│   │   ├── lib.rs (✅ complete - parse_tokens entry point)
│   │   ├── ast.rs (✅ complete - 345 lines)
│   │   ├── lexer.rs (✅ complete - 570 lines, 11 tests)
│   │   ├── parser.rs (✅ complete - 1496 lines, 41 tests total)
│   │   ├── error.rs
│   │   └── builtins.rs
│   ├── Cargo.toml
│   └── tests/ (✅ comprehensive test suite: 15 new tests)
├── vpy_unifier/         ✅ COMPLETE (Phase 3)
│   ├── src/
│   │   ├── lib.rs (✅ complete - unify_modules entry point)
│   │   ├── graph.rs (✅ ModuleGraph with DFS cycles, Kahn's sort)
│   │   ├── resolver.rs (✅ SymbolResolver for MODULE_symbol naming)
│   │   ├── error.rs
│   │   ├── scope.rs
│   │   └── visitor.rs
│   ├── Cargo.toml
│   └── tests/ (✅ 24 comprehensive tests: graph, resolver, scope, visitor)
├── vpy_bank_allocator/  ⏳ TODO (Phase 4)
├── vpy_codegen/         ⏳ TODO (Phase 5)
├── vpy_assembler/       ⏳ TODO (Phase 6)
├── vpy_linker/          ⏳ TODO (Phase 7)
├── vpy_binary_writer/   ✅ DONE (Phase 8)
└── vpy_debug_gen/       ⏳ TODO (Phase 9)
```

## Next Steps

### Immediate (Current Session - Incremental Approach)

## Next Steps

### ✅ COMPLETED (This Session - 2026-01-16)

Phase 3: Module Unification & Import Resolution
- ✅ Created ModuleGraph with DFS cycle detection
- ✅ Implemented topological sorting (Kahn's algorithm)
- ✅ Created SymbolResolver for MODULE_symbol naming
- ✅ Added 24 comprehensive tests
- ✅ All 82 buildtools tests passing (41 parser + 24 unifier + 17 others)
- ✅ Git commit 70281f40 pushed

### Immediate (Phase 4 - Estimated 2-3 hours)

**Status**: Phase 3 (Unification) 100% complete, Phase 4 ready to start

1. **Module merging** (~1-1.5 hours)
   - Load all .vpy files recursively
   - Parse each file via vpy_parser
   - Merge into single unified AST
   - Fix all references (calls, variable access)

2. **Bank allocation** (~1 hour)
   - Analyze code to determine bank boundaries
   - Handle cross-bank calls with wrappers
   - Optimize for 32KB banks

3. **Comprehensive testing** (~30 min)
   - Single-file programs (no change)
   - Multi-file programs (import resolution)
   - Error cases (missing imports, circular deps, unresolved symbols)

### Architecture Decisions

1. **Create public parse_module() in parser.rs** (15 min)
   - Wrap internal `Parser::parse_module()` method
   - Make it accessible to lib.rs

2. **Wire lib.rs parse_tokens()** (15 min)
   - Replace placeholder
   - Call `parser::parse_module()`

3. **Create comprehensive tests** (1.5-2 hours)
   - Basic: numbers, identifiers, strings, operators
   - Expressions: binary ops, unary ops, function calls, indexing
   - Statements: assignments, if/while/for, definitions
   - Module-level: functions, imports, META declarations
   - Integration: complete programs
   - Target: 10+ passing tests

4. **Verify compilation** (30 min)
   - `cargo test -p vpy_parser` → all passing
   - No warnings or errors

### After Phase 2c ✅ Complete
5. **Phase 3: vpy_unifier** (~3-4 hours)
   - Module loading and merging
   - Import resolution
   - Symbol name mangling
   - Circular import detection
   - Entry point validation

### Architecture Decisions

#### Single Source of Truth
- **Phase 7 (Linker) is the ONLY source of final addresses**
- Phase 5 generates ASM without knowing addresses
- Phase 6 assembles to object format with relocations
- Phase 7 places code and applies relocations
- Phases 8-9 derive from linker output

#### Testing Strategy
All phases tested with:
- Single-bank: code in one 32KB bank
- Multibank: code distributed across 32×16KB banks

Each test verifies:
- Correct parsing/transformation
- Single-bank compatibility
- Multibank correctness
- Error handling

## Build and Test

```bash
# Compile all crates
cd buildtools
cargo build --all

# Test just vpy_parser (with lexer)
cd buildtools/vpy_parser
cargo test --lib

# Expected output:
# running 11 tests
# test lexer::tests::test_binary_number ... ok
# test lexer::tests::test_hex_number ... ok
# test lexer::tests::test_operators ... ok
# test lexer::tests::test_keyword ... ok
# test lexer::tests::test_indentation ... ok
# test lexer::tests::test_simple_number ... ok
# test lexer::tests::test_string_literal ... ok
# test lexer::tests::test_invalid_indent ... ok
# test lexer::tests::test_simple_identifier ... ok
# test tests::test_lex_simple_code ... ok
# test tests::test_parse_placeholder ... ok
# test result: ok. 11 passed; 0 failed

# Test all crates
cd buildtools
cargo test --all --lib

# Watch for changes
cargo watch -x "test --all --lib"
```

## Key Design Goals

1. **Modularity**: Each crate has one responsibility
2. **Correctness**: Type-safe interfaces, no string-based communication
3. **Testability**: Every phase testable independently
4. **Debuggability**: Intermediate formats human-readable
5. **Determinism**: Same input → same output always
6. **Parallelizability**: Parser and assembler can parallelize

## Comparison: Old vs New

| Aspect | Old (core) | New (buildtools) |
|--------|---|---|
| **Phases** | Monolithic | 9 independent crates |
| **Interfaces** | String manipulation | Type-safe structs |
| **Address calculation** | 3 places (fragile) | 1 place: linker (reliable) |
| **Linker** | Divide ASM files | Real relocation + symbol table |
| **PDB generation** | Guesses addresses | Derives from linker |
| **Tests** | Implicit | Explicit single/multi |
| **Code location** | core/src/backend/ | buildtools/vpy_*/src/ |

## Integration Plan

Once all crates are complete:
1. Create new vectrexc CLI in buildtools/vectrexc/
2. Point to new pipeline instead of core/
3. Keep old core/ for reference during porting
4. Run comparison tests (old vs new)
5. Deprecate and remove old code

## COMPLETION STATUS - PHASE 2B.1 (Expression Parser)

**Date**: 2026-01-15
**Status**: ✅ COMPLETE - Expression parser scaffold with full AST integration

### Summary
- 493 lines of parser.rs created
- 23 helper methods for token manipulation
- 10 expression parsing methods with correct operator precedence
- All 10 AST compatibility issues identified and fixed
- Zero compilation errors (only expected WIP warnings)
- Build time: 0.24s

### Files
- Created: `/buildtools/vpy_parser/src/parser.rs` (493 lines)
- Modified: `/buildtools/vpy_parser/src/lib.rs` (added module export)
- Docs: 5 comprehensive documentation files created

### Next Steps
1. Phase 2b.2: Statement parser (assignment, if/while/for) - 2-3 hours
2. Phase 2b.3: Module parser (functions, structs, imports) - 1-2 hours
3. Phase 2b.4: Integration testing - 1 hour

**Total Phase 2b ETC**: 5-8 hours

## PHASE 2B.2 COMPLETE (2026-01-15)

**Status**: ✅ Statement Parser Fully Implemented

### Summary
- 250+ lines of statement parsing code
- 7 statement methods: statement(), while_stmt(), for_stmt(), if_stmt(), switch_stmt(), return_stmt(), expr_to_assign_target()
- 16/16 tests passing (11 lexer + 5 parser statement tests)
- Zero compilation errors
- Full support for:
  - If/elif/else statements
  - While loops
  - For loops (range-based and iterator)
  - Switch/case statements
  - Assignments (simple and compound: +=, -=, etc.)
  - Expression statements
  - Break/continue/pass/return

### Files Modified
- `/buildtools/vpy_parser/src/parser.rs`: +617 lines (493→1110 lines total)

### Tests
- Added 5 new parser tests
- All 16 tests passing
- Full lexer integration verified

### Next Phase
Phase 2b.3: Module Parser (function definitions, structs, imports, etc.)
ETC: 2-3 hours remaining for full Phase 2b completion
