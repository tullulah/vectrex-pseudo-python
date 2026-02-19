# Unifier Analysis & Recommendations (2026-01-15)

## Current Status
- ✅ Compiles: multi-module example works (3 modules, ~27KB ASM)
- ⚠️ Identified as weak point by user
- 🔍 Requires investigation of specific bugs/edge cases

## Architecture Overview (4 Phases)

### Phase 1: Export Collection (lines 70-105)
- Gathers exported symbols from each module
- Default: export ALL top-level definitions if no explicit exports
- Builds: `exports: HashMap<String, HashSet<String>>` (module → exported names)
- **Status**: Basic functionality appears correct

### Phase 2: Alias Building (lines 107-139)
- Processes import declarations from each module
- Builds: `aliases: HashMap<String, (String, String)>` (local_name → (origin_module, original_name))
- Handles:
  - Named imports: `from module import symbol`
  - Module imports: `import module`
  - All imports: `from module import *`
- **Status**: Complex, potential edge case issues

### Phase 3: Name Generation (lines 141-165)
- Creates unified names for all symbols
- Entry module symbols: keep original names (main, loop)
- Other modules: prefix with module name (e.g., `input_get_input`)
- Builds: `name_map: HashMap<(String, String), String>` (module, name → unified_name)
- **Status**: Design seems sound, implementation may have gaps

### Phase 4: Item Rewriting (lines 167-256)
- Rewrites module items with unified names
- Handles:
  - Functions: rewrite body with resolved calls
  - Const/GlobalLet: rewrite initializer expressions  
  - Imports are removed (metadata, not code)
  - Exports are removed (metadata, not code)
- **Status**: Critical code path, likely where bugs hide

## Expression Rewriting (Lines 467-575)
**CRITICAL SECTION** - Special handling for module patterns:

### module.method() Pattern (Lines 437-468)
```rust
// Detects: import alias pointing to module (symbol == "*")
// Converts: input.get_input() → INPUT_GET_INPUT()
```
**Potential Issues**:
- Complex logic for detecting module.method calls
- May fail with nested modules or indirect imports
- No visibility of test coverage for this path

### module.field Pattern (Lines 545-575)
```rust
// Detects: import alias for field access
// Converts: input.input_result → INPUT_INPUT_RESULT
```
**Potential Issues**:
- Array field access: `input.input_result[0]` - does it work?
- Nested field access: `module.submodule.field` - not supported?
- Assignment targets: `module.field[i] = x` - complex rewriting needed

## Identified Weak Points

### 1. **No Test Coverage Beyond 3 Unit Tests**
- Only 3 tests in file (lines 667-678):
  - `test_module_id_from_path` - basic ID extraction
  - `test_generate_unified_name_entry` - naming for entry module
  - `test_generate_unified_name_non_entry` - naming for other modules
- **Missing**: Integration tests with real module patterns
- **Missing**: Edge case tests (circular imports, symbol conflicts, deep nesting)

### 2. **Tree Shaking Disabled "For Safety"**
```rust
pub tree_shake: bool,  // Currently disabled by default
```
- Suggests incomplete implementation
- Not tested in current codebase
- Could indicate unknown behavior

### 3. **Module.Field in Assignments - Complex**
- Assignment target rewriting (lines 606-632)
- Array indexing adds complexity: `module.array[i] = x`
- May not handle all forms of assignment correctly

### 4. **Symbol Resolution Fallback**
```rust
// Lines 643-663: resolve_identifier
// 3-step lookup, but what if symbol is in wrong order?
```
- Potential ordering issues in symbol resolution
- Built-in handling may have gaps

## Test Strategy to Identify Bugs

### Test 1: Array Field Access
```python
# graphics.vpy
my_positions = [1, 2, 3, 4]

# main.vpy
import graphics
graphics.my_positions[0] = 5  # Does assignment work?
x = graphics.my_positions[0]  # Does reading work?
```

### Test 2: Multiple Imports from Same Module
```python
# input.vpy
input_x = 0
input_y = 0

def update_input():
    input_x = 10
    input_y = 20

# main.vpy
import input
input.update_input()
# Does it see input_x and input_y correctly?
```

### Test 3: Module Method Returning Value
```python
# math.vpy
def add(a, b):
    return a + b

# main.vpy
import math
result = math.add(5, 3)
```

### Test 4: Nested Function Calls
```python
# lib.vpy
def init():
    setup_graphics()
    setup_input()

def setup_graphics():
    pass

def setup_input():
    pass

# main.vpy
import lib
lib.init()
```

### Test 5: Global Variable Access Across Modules
```python
# state.vpy
counter = 0

def increment():
    counter = counter + 1

# main.vpy
import state
state.increment()
# Does counter actually increment?
```

## Immediate Actions

### Priority 1: Create Comprehensive Tests
- Write integration tests for 5 patterns above
- Run against unifier to find failures
- Document failure modes precisely

### Priority 2: Fix Identified Bugs
- Address each failure systematically
- Update Phase 2/3/4 as needed
- Maintain backward compatibility

### Priority 3: Enable Tree Shaking
- Understand why it's disabled
- Test and enable if feasible
- Or document limitations

### Priority 4: Improve Test Coverage
- Add unit tests for each phase
- Test edge cases (empty modules, circular imports, etc.)
- Achieve >90% code coverage

## Success Criteria

✅ All multi-module integration tests pass
✅ Dot notation works reliably for method calls
✅ Array field access works in reads and writes
✅ Global variables shared across modules
✅ No regression with single-module code

---
**Next Step**: Run the 5 tests above against current unifier to identify specific bugs.
