# BuildTools Architecture - Next Steps (2026-01-16)

## Current Status - Phase 2: vpy_parser

### ✅ Completed
- **Phase 1 (vpy_loader)**: ✅ 100% COMPLETE
  - Project discovery, asset discovery, entry point detection
  - 351 lines, 5 passing tests

- **Phase 2a (vpy_parser - Lexer)**: ✅ 100% COMPLETE
  - Tokenization with indentation handling
  - 570 lines, 11 passing tests
  - All token types: operators, keywords, literals

- **Phase 2b (vpy_parser - AST)**: ✅ 100% COMPLETE
  - Full AST type hierarchy (Module, Item, Stmt, Expr)
  - 345 lines, comprehensive structure
  - Matches core/src/ast.rs types

- **Phase 2c (vpy_parser - Parser)**: ✅ 85% COMPLETE
  - 1496 lines of parser logic ported from core/src/parser.rs
  - Recursive descent parser with helper methods
  - Public functions defined but `parse_tokens()` is placeholder

### ⏳ In Progress
- **Wiring parse_tokens() → parser module**: NEXT TASK
  - Currently `parse_tokens()` is a placeholder in lib.rs
  - `parser.rs` has a `Parser` struct with all logic (1496 lines)
  - Need to expose internal `parse_module()` function

### ⏳ Next: Phase 3+
- Phase 3 (vpy_unifier): Module unification, cross-module imports
- Phase 4 (vpy_bank_allocator): Determine which functions go in which bank
- Phase 5 (vpy_codegen): Generate M6809 assembly
- Phase 6 (vpy_assembler): Assemble to object files
- Phase 7 (vpy_linker): Link banks, compute real addresses
- Phase 8+ (debug_gen, binary_writer): Debug symbols and final binary

---

## 🎯 Next Task: Wire parse_tokens() → Parser

### Goal
Connect the placeholder `parse_tokens()` in lib.rs to the actual parser implementation in parser.rs

### Current Issue
1. `parser.rs` has a `Parser` struct with private methods
2. Main entry point should be `parse_module()` function
3. `lib.rs` has `parse_tokens()` placeholder that returns error
4. Need to:
   - Make `Parser` struct public or expose parse function
   - Wire lib.rs → parser.rs
   - Add comprehensive tests

### Steps to Complete

#### Step 1: Expose parser module in lib.rs
- Currently parser module is private (`mod parser`)
- Change to expose parsing functions
- Options:
  - Option A: Make `Parser::new()` public + `parse_module()` public
  - Option B: Create standalone `pub fn parse_module()` wrapper function

**Recommendation**: Option B (cleaner API)

```rust
// In parser.rs: Add public wrapper function at module level
pub fn parse_module(tokens: &[Token], filename: &str) -> ParseResult<Module> {
    let mut parser = Parser::new(tokens, filename.to_string());
    parser.parse_module()
}
```

#### Step 2: Update lib.rs parse_tokens()
```rust
pub fn parse_tokens(tokens: &[Token], filename: &str) -> ParseResult<Module> {
    parser::parse_module(tokens, filename)
}
```

#### Step 3: Add comprehensive tests
Create test suite covering:
- Simple expressions (numbers, identifiers)
- Binary operations (addition, multiplication)
- Function calls
- Function definitions
- Variable declarations
- Control flow (if/while/for)
- Arrays and indexing
- Module imports
- Error cases (syntax errors)

**Test structure**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Test 1: Parse simple number
    #[test]
    fn test_parse_number() { ... }
    
    // Test 2: Parse binary operation
    #[test]
    fn test_parse_binop() { ... }
    
    // Test 3: Parse function call
    #[test]
    fn test_parse_call() { ... }
    
    // Test 4: Parse function definition
    #[test]
    fn test_parse_function() { ... }
    
    // Test 5: Parse variable declaration
    #[test]
    fn test_parse_let() { ... }
    
    // ... and more
}
```

#### Step 4: Add integration test
Test parsing complete VPy programs:
```rust
#[test]
fn test_parse_complete_program() {
    let code = r#"
def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    DRAW_LINE(0, 0, 50, 50, 127)
"#;
    
    let tokens = lex(code).unwrap();
    let module = parse_tokens(&tokens, "test.vpy").unwrap();
    
    assert_eq!(module.items.len(), 2);
    // Verify main() and loop() functions
}
```

#### Step 5: Run tests and verify
```bash
cd buildtools
cargo test -p vpy_parser
```

---

## Expected Effort & Timeline

| Task | Effort | Blocker |
|------|--------|---------|
| Examine parser.rs for entry point | 30 min | No |
| Create parse_module() wrapper | 15 min | No |
| Wire lib.rs → parser.rs | 15 min | No |
| Create 5+ unit tests | 45 min | No |
| Create integration test | 30 min | No |
| Debug test failures | 30-60 min | Depends on issues |
| **TOTAL** | **2-2.5 hours** | None identified |

---

## Why This Matters

1. **Unblocks Phase 3**: vpy_unifier needs `parse_module()` to work
2. **Validates parser.rs**: Tests verify port from core was correct
3. **Establishes pattern**: Same pattern used in all phases
4. **Close to shipping**: Parser mostly done, just need to wire + test

---

## Success Criteria

- [ ] All tests pass: `cargo test -p vpy_parser` → OK
- [ ] parse_tokens() no longer returns placeholder error
- [ ] Parser can handle single-file VPy programs
- [ ] Error messages include file:line:col information
- [ ] 10+ tests covering various language constructs

---

## Follow-up: Phase 3 (vpy_unifier)

Once Phase 2 is complete:

1. **Module unification**
   - Load multiple .vpy files
   - Resolve imports
   - Merge into single AST
   - Detect circular imports

2. **Symbol name mangling**
   - `input.get_input()` → `INPUT_GET_INPUT`
   - Prevent symbol collisions

3. **Validation**
   - Check all imported symbols exist
   - Verify function signatures
   - Type checking (basic)

**Estimated effort**: 3-4 hours (more complex than parser)

---

## Files to Modify

### buildtools/vpy_parser/src/parser.rs
- Add `pub fn parse_module(tokens: &[Token], filename: &str) -> ParseResult<Module>`
- Make internal `Parser::parse_module()` accessible

### buildtools/vpy_parser/src/lib.rs
- Replace placeholder in `parse_tokens()`
- Call `parser::parse_module(tokens, filename)`

### buildtools/vpy_parser/tests/parser_tests.rs (CREATE NEW)
- Comprehensive test suite for parser
- Unit tests for each language construct
- Integration tests for complete programs
