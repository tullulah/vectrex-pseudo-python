# BuildTools - Modular Compiler Pipeline

## Current Status (Updated 2026-01-16 - Phase 2c COMPLETE)

✅ **Phase 1 Complete**: vpy_loader crate is ready
✅ **Phase 2a Complete**: vpy_parser lexer (11 tests passing)
✅ **Phase 2b Complete**: vpy_parser AST types (345 lines, 100% ported)
✅ **Phase 2c Complete**: vpy_parser parser (1496 lines ported, entry point wired, 41 tests passing)
⏳ **Phase 3 Next**: vpy_unifier (module merging, import resolution)

### Session 2026-01-16: VECTREX.I Refactoring Complete ✅
- ✅ All hardcoded BIOS addresses in buildtools eliminated
- ✅ Dynamic resolution from VECTREX.I (single source of truth)
- ✅ Wait_Recal ($F192), DP_to_D0 ($F1AA), DP_to_C8 ($F1AF) verified
- ✅ buildtools compiles cleanly with no hardcoded BIOS addresses

### Completed Work

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
├── vpy_unifier/         ⏳ NEXT (Phase 3)
│   ├── src/lib.rs (placeholder)
│   └── Cargo.toml
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

Phase 2c: Parser Entry Point Wiring
- ✅ Created pub fn parse_module() in parser.rs
- ✅ Connected parse_tokens() in lib.rs to parser module
- ✅ Added 15 comprehensive tests
- ✅ All 41 tests passing
- ✅ buildtools compiles cleanly

### Immediate (Phase 3 Preparation - Estimated 3-4 hours)

**Status**: Phase 2 (Parsing) 100% complete, Phase 3 ready to start

1. **Module unification** (~1.5-2 hours)
   - Load multiple .vpy files via vpy_loader
   - Parse each file via vpy_parser
   - Merge into single unified AST
   - Detect circular imports

2. **Symbol name mangling** (~1 hour)
   - Transform `input.get_input()` → `INPUT_GET_INPUT`
   - Prevent symbol name collisions
   - Build unified symbol table

3. **Validation** (~30 min)
   - Verify all imported symbols exist
   - Check for duplicate definitions
   - Type compatibility checking (basic)

4. **Comprehensive testing** (~30 min)
   - Single-file programs (passthrough)
   - Multi-file programs (import resolution)
   - Error cases (missing imports, circular deps)

### After Phase 3
5. **Phase 4: vpy_bank_allocator** (~2 hours)
   - Determine function-to-bank mapping
   - Generate call graph
   - Allocate functions to banks

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
