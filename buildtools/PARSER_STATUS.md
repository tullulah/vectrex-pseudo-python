# BuildTools vpy_parser - Current Status & Next Steps

## ✅ COMPLETED (Phase 1)
- **vpy_loader**: Phase 1 is COMPLETE
  - Load projects (single + multibank)
  - Discover .vpy files
  - Discover assets (.vec, .vmus)
  - Parse .vpyproj TOML
  - API: `load_project(path) -> ProjectInfo`
  - Status: Ready to use

## ✅ IN PLACE (Partially)
- **vpy_parser/src/ast.rs**: Complete AST structure (345 lines)
  - All types ported from core/src/ast.rs
  - Module, Function, Stmt, Expr, AssignTarget, etc.
  - Type-safe representation of VPy language
  - Status: Ready (no changes needed)

- **vpy_parser/src/builtins.rs**: Exists
- **vpy_parser/src/error.rs**: Error types exist

## ⏳ NEEDED (Phase 2)
- **vpy_parser/src/lexer.rs** (~333 lines)
  - Port from core/src/lexer.rs
  - Tokenize VPy source code
  - TokenKind enum and Token struct
  - Handles indentation (INDENT/DEDENT tokens)
  - Handles string literals, numbers, keywords, operators

- **vpy_parser/src/parser.rs** (~956 lines)
  - Port from core/src/parser.rs
  - Recursive descent parser
  - Converts tokens → AST
  - Entry point: `parse(tokens) -> Result<Module>`
  - Handles:
    - Module-level items (functions, const, imports, structs)
    - Statements (assign, if, while, for, etc.)
    - Expressions (binary ops, calls, indexing, etc.)
    - Type checking and semantic analysis

- **vpy_parser/src/lib.rs** (update)
  - Public API: `parse_with_filename(filename) -> Result<Module>`
  - May need to expose lexer for IDE use

## 🔧 TECHNICAL NOTES

### Parser Architecture
```
Input:  .vpy file (text)
  ↓
[Phase 1: Load] → Discover file
  ↓ (vpy_loader)
[Phase 2: Lex]  → Tokenize content
  ↓ (vpy_parser/lexer)
[Phase 2: Parse] → AST
  ↓ (vpy_parser/parser)
Output: Module (AST)
```

### Key Dependencies
- **No external parser library** - implement from scratch (like core)
- **Indentation-aware** - need INDENT/DEDENT tokens (Python-style)
- **Error recovery** - good error messages with file:line:col

### Multibank Considerations
- Parser is **language-agnostic** - works same for single/multibank
- Multibank config detected in META declarations
- Bank allocation happens in Phase 4 (vpy_bank_allocator)

## 📋 PORTING CHECKLIST

### For lexer.rs
- [ ] Copy token types (TokenKind, Token)
- [ ] Copy lexer struct and main lexing loop
- [ ] Handle indentation properly
- [ ] Adapt error handling to use ParseError
- [ ] Run tests with sample VPy code

### For parser.rs
- [ ] Copy parser struct and helper functions
- [ ] Copy expression parser (precedence climbing)
- [ ] Copy statement parser (if/while/for/etc)
- [ ] Copy import/export handling
- [ ] Copy META handling
- [ ] Adapt error messages
- [ ] Run tests with sample VPy code

### For lib.rs
- [ ] Add lexer module export
- [ ] Add parser module export
- [ ] Update `parse_with_filename` to use real parser
- [ ] Add integration tests

## ⏱️ ESTIMATED EFFORT
- **Lexer**: 2-3 hours (straightforward port)
- **Parser**: 4-6 hours (larger, needs careful porting)
- **Testing**: 2-3 hours (comprehensive test suite)
- **Total**: 8-12 hours of focused work

## 🎯 SUCCESS CRITERIA
1. ✅ `cargo test --lib vpy_parser` passes
2. ✅ Parse simple VPy: `def main(): SET_INTENSITY(127)`
3. ✅ Parse complex VPy: imports, structs, META, multibank
4. ✅ Error messages are clear and actionable
5. ✅ Lexer + Parser handle all VPy language features
6. ✅ Performance: parse 100KB+ files in <100ms

## 🚀 NEXT SESSION
1. Port lexer.rs from core (TokenKind, Token, Lexer struct)
2. Port parser.rs from core (Parser struct, parse functions)
3. Update lib.rs to expose real parser
4. Create integration tests
5. Verify single-bank and multibank parsing works
