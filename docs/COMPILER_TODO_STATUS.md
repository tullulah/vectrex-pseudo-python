# Compiler TODO Status - 2026-01-11

Estado completo de features NO implementadas, bugs conocidos y trabajo pendiente.

---

## 🔴 CRITICAL - Runtime Issues

### Multi-Bank ROM Execution Failure
**Status**: ✅ Compilation working, ❌ Runtime broken
**Problem**: Emulator se detiene en PC=0x8039 después de arrancar
**File**: `examples/pang_multi/build/pang.rom` (512KB)
**Details**:
- Compilación exitosa: 512KB ROM generado correctamente
- Todos los símbolos resueltos
- Runtime helpers incluidos en todos los bancos
- Emulator output: `PC: 0x8039 | Debug State: STOPPED | JSVecx: STOPPED`

**Investigation needed**:
- [ ] Verificar vector de reset en Bank #31 (fixed bank)
- [ ] Comprobar si PC=0x8039 está en región válida
- [ ] Revisar bank switching logic
- [ ] Comparar con single-bank .bin que funciona
- [ ] Debug con JSVecx para ver qué instrucción falla

**Files to check**:
- `core/src/backend/m6809/multi_bank_linker.rs` (líneas 180-250)
- `examples/pang_multi/build/pang.asm` (header section)

---

## 🟡 MEDIUM - Language Features

### 1. Structs System (PLANNED - Not Started)
**Status**: ❌ Not implemented
**Document**: `STRUCTS_IMPLEMENTATION_PLAN.md` exists but not implemented
**Needed for**:
- Complex data structures
- Better code organization
- Type safety

**Estimated effort**: 2-3 weeks
**Priority**: Medium (workaround: use parallel arrays)

### 2. String Operations
**Status**: ⚠️ Partial

**Implemented**:
- ✅ String literals: `"HELLO"`
- ✅ PRINT_TEXT builtin
- ✅ Const string arrays: `const names = ["A", "B"]`

**Missing**:
- [ ] String concatenation
- [ ] String comparison (STR_CMP)
- [ ] String length (STR_LEN)
- [ ] Runtime string building
- [ ] String indexing: `name[0]`
- [ ] String slicing: `name[0:5]`

**Blocker**: ROM-only design makes runtime string manipulation difficult

### 3. Const Array Advanced Features
**Status**: ⚠️ Partial

**Implemented**:
- ✅ Const number arrays: `const values = [10, 20]`
- ✅ Const string arrays: `const names = ["A", "B"]`
- ✅ Array indexing: `x = values[i]`

**Missing**:
- [ ] Multi-dimensional arrays: `const matrix = [[1,2],[3,4]]`
- [ ] Const struct data
- [ ] Passing const arrays to functions
- [ ] Bounds checking at compile time
- [ ] Mixed type arrays: `[1, "hello"]` (intentionally not supported)

### 4. Function Parameters - Const Arrays
**Status**: ❌ Not implemented
**Problem**: Can't pass const arrays to functions directly
**Workaround**: Use indices and global const arrays

**Example (doesn't work)**:
```python
const colors = [127, 100, 80]

def draw_with_color(color_array):  # ❌ Not supported
    SET_INTENSITY(color_array[0])
```

**Workaround (current)**:
```python
const colors = [127, 100, 80]

def draw_with_color(index):  # ✅ Pass index instead
    SET_INTENSITY(colors[index])
```

---

## 🟢 LOW - Optimizations

### 1. Dead Code Elimination
**Status**: ❌ Not implemented
**Impact**: Unused functions still in binary
**Estimated saving**: 10-20% binary size for large projects

### 2. Constant Folding
**Status**: ⚠️ Partial
**Implemented**: Basic arithmetic at parse time
**Missing**:
- [ ] Cross-statement constant propagation
- [ ] Dead store elimination
- [ ] Unused variable detection
- [ ] Tests for optimization correctness

### 3. Register Allocation
**Status**: ❌ Not implemented
**Current**: All variables in RAM
**Optimization**: Keep frequently used variables in registers A/B/D/X/Y
**Estimated gain**: 20-30% speed improvement

### 4. Inline Small Functions
**Status**: ❌ Not implemented
**Impact**: Function call overhead for tiny functions
**Example**: `def get_x(): return player_x` could be inlined

---

## 🔵 PAUSED - Phase 6.5 Features

### Per-Module Object File Generation
**Status**: 30% (infrastructure ready, implementation paused)
**Document**: `PHASE6_FUTURE_WORK.md`
**Reason**: ROI negative (10-14 hours work vs no current need)

**Implemented**:
- ✅ CLI flag: `--separate-modules`
- ✅ .vo object file format
- ✅ link command functional
- ✅ extract_sections_with_binary() working

**Missing**:
- [ ] Per-module compilation without unifier
- [ ] Symbol resolution for separate modules
- [ ] Cross-module reference transformation
- [ ] Incremental build system (Phase 6.6)
- [ ] Parallel compilation (Phase 6.7)
- [ ] Build cache (Phase 6.8)

**Alternative approach** (if needed):
1. Unified compilation (current working system)
2. Extract sections by module from unified ASM
3. Create .vo per module
4. Link .vo files
**Estimated time**: 2-3 hours vs 10-14 hours for full implementation

---

## 📋 Known Limitations

### 1. Variable Scoping
**Status**: ✅ Working as designed, but different from Python
**Behavior**: Functions have separate scopes (not shared)

**Example**:
```python
def main():
    player_x = 0  # ❌ NOT accessible in loop()

def loop():
    player_x = 0  # ✅ Must declare in loop() where used
```

**Validation**: ✅ Compiler detects cross-function variable use and shows error

### 2. Integer Truncation
**Status**: ✅ Working as designed
**Behavior**: All integers are 16-bit signed (-32768 to 32767)
**Overflow**: Silent wrap-around (no error)

**Example**:
```python
x = 32767
x = x + 1  # x becomes -32768 (wraps)
```

**Documentation**: Should be in SUPER_SUMMARY.md (TODO S5)

### 3. Array Mutability
**Status**: ✅ Working as designed
**Const arrays**: Read-only (in ROM)
**Regular arrays**: Mutable (in RAM)

**No mixing**: Can't have mutable arrays of const data

### 4. Memory Limits
**Status**: ✅ Working as designed
**RAM**: ~2KB available (0xC800-0xCFFF)
**ROM**: 32KB standard, 512KB multi-bank
**Stack**: Grows down from 0xCFFF

**Large projects**: Use multi-bank ROM (but see runtime issue above)

---

## 🐛 Known Bugs

### 1. Multi-Bank ROM Runtime Failure (CRITICAL)
See section at top of document.

### 2. Joystick RAM Collision (RESOLVED 2025-12-18)
**Status**: ✅ Fixed
**Solution**: Moved J1_X/J1_Y to $CF00/$CF01
**Note**: If new collision, update addresses in both:
- `core/src/backend/m6809/builtins.rs`
- `ide/frontend/src/components/panels/EmulatorPanel.tsx`

### 3. DRAW_LINE Segmentation Edge Cases
**Status**: ⚠️ Known limitation
**Problem**: Lines with |dx| > 127 AND |dy| > 127 only segment dy
**Impact**: Very rare (screen is only 256x256)
**Workaround**: Break into multiple smaller lines

---

## 🔧 Infrastructure TODO

### 1. LSP Integration for Semantic Errors
**Status**: ⚠️ Partial
**Document**: `LSP_IMPROVEMENTS_PLAN.md`
**Current**: Semantic errors shown in terminal
**Missing**: Real-time diagnostics in editor

**Task S6** (from copilot-instructions):
- [ ] Expose `Vec<Diagnostic>` from emit_asm_with_debug()
- [ ] LSP consumes diagnostics for red squiggles
- [ ] Show errors inline as user types

### 2. Test Coverage Improvements
**Status**: ⚠️ Incomplete

**Good coverage**:
- ✅ Opcodes (256 tests in tests/opcodes/)
- ✅ Components (19 tests in tests/components/)
- ✅ Basic compilation (examples/)

**Missing coverage**:
- [ ] Constant folding correctness
- [ ] Dead store elimination
- [ ] Multi-module edge cases
- [ ] Error message quality
- [ ] Recovery from parse errors

### 3. Documentation Gaps
**Status**: ⚠️ Partial

**Excellent docs**:
- ✅ SUPER_SUMMARY.md (comprehensive)
- ✅ Phase 6 (PHASE6_SUMMARY.md, PHASE6_FUTURE_WORK.md)
- ✅ copilot-instructions.md (updated)

**Missing docs**:
- [ ] VPy language reference (user-facing)
- [ ] Tutorial for beginners
- [ ] API reference for builtins
- [ ] Asset system guide (vectors/music/sfx)
- [ ] Troubleshooting guide

**Task S5** (from copilot-instructions):
- [ ] Document integer truncation in SUPER_SUMMARY.md
- [ ] Add examples of overflow behavior

---

## 🎯 Prioritized Roadmap

### Immediate (This Week)
1. **🔴 CRITICAL**: Fix multi-bank ROM runtime issue (PC=0x8039)
2. Document integer truncation behavior
3. Add troubleshooting section to docs

### Short Term (This Month)
1. Implement string comparison builtin (STR_CMP)
2. Implement string length builtin (STR_LEN)
3. LSP integration for semantic diagnostics
4. Improve test coverage for edge cases

### Medium Term (This Quarter)
1. Structs system implementation
2. Dead code elimination optimization
3. Multi-dimensional const arrays
4. Function parameter improvements (const array passing)

### Long Term (Future)
1. Register allocation optimization
2. Inline small functions
3. Per-module .vo generation (if needed)
4. Incremental build system
5. Parallel compilation

---

## 📊 Completion Status

### Compiler Core
- **Lexer/Parser**: 95% ✅
- **AST**: 90% ✅
- **Semantic Analysis**: 85% ✅
- **Code Generation**: 90% ✅
- **Optimization**: 20% ⚠️

### Backend (M6809)
- **Instruction Emission**: 100% ✅
- **Builtins**: 95% ✅
- **Runtime Helpers**: 100% ✅
- **Multi-Bank**: 90% ⚠️ (compilation ✅, runtime ❌)

### Module System
- **Import Syntax**: 100% ✅
- **Module Resolution**: 100% ✅
- **Symbol Transformation**: 100% ✅
- **Per-Module .vo**: 30% ⏸️ (paused)

### Asset System
- **Vector Files (.vec)**: 100% ✅
- **Music Files (.vmus)**: 100% ✅
- **SFX Files (.vsfx)**: 100% ✅
- **Runtime Players**: 90% ✅ (music player working, sfx working)

### Tools
- **Compiler (vectrexc)**: 95% ✅
- **Linker (vectrexc link)**: 80% ✅
- **LSP Server**: 70% ⚠️
- **IDE Integration**: 85% ✅

### Documentation
- **Internal Docs**: 90% ✅
- **User Docs**: 40% ⚠️
- **API Reference**: 30% ⚠️
- **Tutorials**: 20% ⚠️

---

## 🔍 How to Use This Document

**For continuing work**:
1. Start with 🔴 CRITICAL section (multi-bank runtime issue)
2. Check 📋 Known Limitations before implementing new features
3. Review 🐛 Known Bugs to avoid duplicating issues
4. Follow 🎯 Prioritized Roadmap for feature work

**For new contributors**:
1. Read SUPER_SUMMARY.md for architecture overview
2. Check PHASE6_SUMMARY.md for module system understanding
3. Review this document for what's pending
4. Start with 🟢 LOW priority items for easy wins

**For bug reports**:
1. Check 🐛 Known Bugs section first
2. Check 📋 Known Limitations (might be by design)
3. If new, add to appropriate section

---

**Last Updated**: 2026-01-11
**Next Review**: After multi-bank runtime issue resolution
**Maintainer**: Update this file when completing TODOs or discovering new issues
