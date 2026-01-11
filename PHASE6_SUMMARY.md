# Phase 6: VPy Module System - IMPLEMENTATION STATUS 2026-01-11

## Status: Phase 6.4 COMPLETE ✅, Phase 6.5 IN PROGRESS (30%)

### Architecture Overview
- **Purpose**: Multi-file projects with reusable libraries and code organization
- **Status**: ✅ Phase 6.4 COMPLETE - Runtime section separated
- **Current**: 🔄 Phase 6.5 STARTED - Per-module .vo compilation (30% complete)

### Implementation Summary

**Phase 6.3 COMPLETE (100% ✅)**:
- ✅ **Dot notation**: `input.get_input()` → `INPUT_GET_INPUT()`
- ✅ **Array labels**: Variable-based naming prevents collisions
- ✅ **Assign targets**: `module.variable[i] = x` works correctly  
- ✅ **Runtime helpers**: Auto-deduplicated (unifier merges to single module)

**Phase 6.4 COMPLETE (100% ✅)**:
- ✅ **Runtime Section**: Helpers clearly separated with visual headers
- ✅ **Program Section**: User code with descriptive comments
- ✅ **Data Section**: Variables and arrays organized separately
- ✅ **Visual Organization**: Box-drawing characters for section headers

**Phase 6.5 IN PROGRESS (30% ✅)**:
- ✅ **CLI Flag**: `--separate-modules` implemented
- ✅ **Module Loading**: `resolver::load_project()` discovers all modules
- ✅ **Object Format**: `.vo` format fully defined (linker/object.rs)
- ✅ **Link Command**: `vectrexc link` can combine multiple .vo files
- ✅ **Fallback**: Currently falls back to unified compilation
- ❌ **Per-Module ASM**: Needs codegen without unifier
- ❌ **External Symbols**: Import tracking between modules
- ❌ **Relocations**: Cross-module function calls

**Key Achievement**: Infrastructure for incremental compilation is in place, currently uses unified fallback.

### Real-World Example

**input.vpy** - Input handling module:
```python
input_result = [0, 0]

def get_input():
    input_result[0] = J1_X()
    input_result[1] = J1_Y()
```

**graphics.vpy** - Graphics utilities:
```python
def draw_square(x, y, size):
    DRAW_LINE(x, y, x+size, y, 127)
    DRAW_LINE(x+size, y, x+size, y+size, 127)
    DRAW_LINE(x+size, y+size, x, y+size, 127)
    DRAW_LINE(x, y+size, x, y, 127)
```

**main.vpy** - Entry point:
```python
import input
import graphics

player_x = 0
player_y = 0

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Call imported functions
    input.get_input()              # ✅ Transforms to INPUT_GET_INPUT()
    
    # Access imported variables
    dx = input.input_result[0]     # ✅ Transforms to INPUT_INPUT_RESULT[0]
    dy = input.input_result[1]
    
    # Update local state
    player_x = player_x + dx
    player_y = player_y + dy
    
    # Call imported graphics
    graphics.draw_square(player_x, player_y, 10)
```

### Compilation and Verification

**Build Command**:
```bash
cargo run --bin vectrexc -- build examples/multi-module/src/main.vpy --bin
```

**Output**: `main.bin` (32KB)

**Verification of Helper Deduplication**:
```bash
grep -E "^(MUL16|DIV_A|DRAW_LINE_WRAPPER):" examples/multi-module/src/main.asm | wc -l
# Expected: 3 (one of each, NO duplicates)
```

**Result**: ✅ Confirmed - only 1 instance of each helper

### Technical Implementation

**Files Modified**:
1. **core/src/unifier.rs** (lines 540-675)
   - `Expr::FieldAccess` detection: Transforms `module.symbol` to unified identifier
   - `AssignTarget` rewriting: Handles `module.variable[i] = x` assignments
   - Recursive expression rewriting in all statement types

2. **core/src/backend/m6809/mod.rs** (lines 820-838, 1450-1549)
   - Array labels: `ARRAY_{name.to_uppercase()}` prevents collisions
   - Const arrays: `CONST_ARRAY_{name.to_uppercase()}`

3. **core/src/codegen.rs** (line 308)
   - Type changed: `BTreeMap<String, String>` (name → uppercase_label)

**Unified Symbol Table** (automatically generated):
```
INPUT_INPUT_RESULT   → input.input_result
INPUT_GET_INPUT      → input.get_input()
GRAPHICS_DRAW_SQUARE → graphics.draw_square()
PLAYER_X             → main.player_x (entry module, no prefix)
PLAYER_Y             → main.player_y
```

### Why Deduplication Works Automatically

**Architecture**:
```
Phase 3.5: Multi-file import resolution
  ↓
Unifier: Merge all modules into ONE unified module
  ↓
Codegen: Generate code for SINGLE module
  ↓
Runtime helpers emitted ONCE (no duplicates possible)
```

**Key Insight**: Since the unifier creates a single merged module BEFORE codegen, there's only one place to emit helpers. This eliminates duplication without special logic.

### Next Steps (Phase 6.5 Completion)

**Phase 6.5: Per-Module .vo Generation** (30% ✅ → 100%):
- **Goal**: Compile each module to separate object file
- **Benefits**: True incremental compilation (change one module, recompile only that one)
- **Status**: Infrastructure ready, needs implementation

**What's Ready**:
1. ✅ `.vo` format (linker/object.rs) - Binary format for compiled modules
2. ✅ Module loading (resolver) - Discovers all dependencies
3. ✅ Link command - Combines multiple .vo files
4. ✅ CLI flag (`--separate-modules`)

**What's Needed**:
1. ❌ **Per-Module Codegen**: Bypass unifier, emit ASM for single module
   - Export list: Functions/variables this module exports
   - Import list: External symbols this module needs
   - Relocations: References to external symbols

2. ❌ **Symbol Tables**: Track what each module provides/needs
   - `INPUT.vo` exports: `INPUT_GET_INPUT`, `INPUT_INPUT_RESULT`
   - `MAIN.vo` imports: `INPUT_GET_INPUT`, `INPUT_INPUT_RESULT`

3. ❌ **Linker Enhancements**: Resolve cross-module references
   - Replace import placeholders with real addresses
   - Apply relocations for JSR to external functions

**Reason to Defer**: Current unified approach works well. No projects near limits yet.

### ASM Output Organization (Phase 6.4)

**Generated ASM Structure**:
```asm
;
; ┌─────────────────────────────────────────────────────────────────┐
; │ RUNTIME SECTION - VPy Builtin Helpers & System Functions       │
; │ This section contains reusable code shared across all VPy       │
; │ programs. These helpers are emitted once per compilation unit.  │
; └─────────────────────────────────────────────────────────────────┘
;

; === JOYSTICK BUILTIN SUBROUTINES ===
J1X_BUILTIN:
    PSHS X
    JSR $F1AA  ; DP_to_D0
    ...

; === DRAW_LINE_WRAPPER ===
DRAW_LINE_WRAPPER:
    ...

; === MUL16 ===
MUL16:
    ...

;
; ┌─────────────────────────────────────────────────────────────────┐
; │ PROGRAM CODE SECTION - User VPy Code                            │
; │ This section contains the compiled user program logic.          │
; └─────────────────────────────────────────────────────────────────┘
;

START:
    LDA #$D0
    TFR A,DP
    ...

INPUT_GET_INPUT:
    ...

GRAPHICS_DRAW_SQUARE:
    ...

;
; ┌─────────────────────────────────────────────────────────────────┐
; │ DATA SECTION - RAM Variables & ROM Constants                    │
; │ This section defines all variables, arrays, and const data.     │
; └─────────────────────────────────────────────────────────────────┘
;

; Variables (in RAM)
VAR_PLAYER_X EQU $CF10+0
VAR_PLAYER_Y EQU $CF10+2

; Array data (in ROM)
ARRAY_INPUT_INPUT_RESULT:
    FDB 0
    FDB 0
```

**Benefits**:
- **Readability**: Clear separation of concerns in generated ASM
- **Debugging**: Easy to locate runtime helpers vs user code vs data
- **Navigation**: Visual headers make large files easier to navigate
- **Consistency**: Standardized organization across all compilations

### Conclusion

✅ **Phase 6.3 is COMPLETE** - Multi-module system fully functional
✅ **Phase 6.4 is COMPLETE** - ASM sections clearly organized
⚠️ **Phase 6.5 is PARTIAL (30%)** - Infrastructure ready, implementation paused

**Working Features**:
- Import statements working
- Dot notation resolving correctly
- Array label collisions prevented
- Runtime helpers automatically deduplicated
- Real-world example compiling successfully (32KB binary)
- Generated ASM organized in clear sections with visual headers

**Phase 6.5 Status**:
- CLI flag `--separate-modules` implemented
- VectrexObject (.vo format) fully defined
- Link command functional
- **Implementation paused**: Unified compilation is sufficient for current needs
- **See**: `PHASE6_FUTURE_WORK.md` for detailed roadmap and future optimizations

**Recommendation**: No additional work needed on Phase 6 until projects >50KB or build time >3 seconds.

