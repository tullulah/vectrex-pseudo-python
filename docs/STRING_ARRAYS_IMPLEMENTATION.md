# String Arrays Implementation - Complete

## Feature Summary
Full support for const string arrays in VPy, allowing dynamic text display using array indexing.

## Syntax
```python
const greetings = ["HELLO", "WORLD", "VECTREX"]

def loop():
    WAIT_RECAL()
    msg = greetings[0]
    PRINT_TEXT(-50, 50, msg)
```

## Implementation Details

### 1. Data Structure (codegen.rs)
Added `const_string_arrays: BTreeSet<String>` to track which const arrays contain strings.

### 2. Detection & Population (m6809/mod.rs)
During const var collection, detect string arrays:
```rust
for (name, value) in &const_vars {
    if let Expr::List(elements) = value {
        let is_string_array = elements.iter().all(|e| matches!(e, Expr::StringLit(_)));
        if is_string_array {
            opts.const_string_arrays.insert(name.clone());
        }
    }
}
```

### 3. Assembly Emission (m6809/mod.rs)
String arrays emit differently than number arrays:

**Number Array** (stores values):
```asm
CONST_ARRAY_0:
    FDB 10   ; Element 0
    FDB 20   ; Element 1
```

**String Array** (stores pointers):
```asm
; Individual strings
CONST_ARRAY_0_STR_0:
    FCC "HELLO"
    FCB $80   ; String terminator

CONST_ARRAY_0_STR_1:
    FCC "WORLD"
    FCB $80

; Pointer table
CONST_ARRAY_0:
    FDB CONST_ARRAY_0_STR_0  ; Pointer
    FDB CONST_ARRAY_0_STR_1  ; Pointer
```

### 4. Indexing Behavior (m6809/expressions.rs)
Array indexing checks `opts.const_string_arrays`:

**String Array** - Returns pointer (address):
```asm
; ===== Const array indexing: greetings =====
LDD VAR_INDEX        ; Load index
ASLB                 ; Multiply by 2 (pointers are 2 bytes)
ROLA
STD TMPPTR
LDX #CONST_ARRAY_0   ; Load table address
LDD TMPPTR
LEAX D,X             ; Add offset
; String array - load pointer from table
LDD ,X               ; Load pointer (NOT value)
STD RESULT           ; Result contains address
```

**Number Array** - Returns value (same code, different semantics):
```asm
; Same code, but semantically loads VALUE not pointer
LDD ,X
STD RESULT
```

### 5. PRINT_TEXT Integration (emission.rs)
PRINT_TEXT already expects pointer in ARG2:
```asm
VECTREX_PRINT_TEXT:
    LDU VAR_ARG2   ; Load string pointer
    LDA VAR_ARG1+1 ; Y coordinate
    LDB VAR_ARG0+1 ; X coordinate
    JSR Print_Str_d
    RTS
```

Works seamlessly with string array indexing result.

## Files Modified

1. **core/src/codegen.rs** (lines 187-190, 313-317)
   - Added `const_string_arrays: BTreeSet<String>` field
   - Initialize empty set in constructor

2. **core/src/backend/m6809/mod.rs** (lines 283-299, 1078-1105)
   - Populate `const_string_arrays` during const var processing
   - Dual emission logic: FCC strings + FDB pointer table

3. **core/src/backend/m6809/expressions.rs** (lines 239-267)
   - Check `const_string_arrays` during indexing
   - Return pointer for string arrays (instead of dereferencing)

4. **core/src/main.rs** (lines 501-519, 537-552)
   - Initialize `const_string_arrays` in all CodegenOptions constructors

## Testing

### Test 1: Simple String Array (test_string_arrays.vpy)
```python
const greetings = ["HELLO", "WORLD", "VECTREX"]
index = 0

def loop():
    WAIT_RECAL()
    msg = greetings[index]
    PRINT_TEXT(-50, 50, msg)
```
✅ Compiles successfully (1242 bytes)
✅ Generates correct FCC strings + pointer table
✅ Indexing returns pointer

### Test 2: Real-World Usage (pang/src/main.vpy)
```python
const location_names = ["MOUNT FUJI - JAPAN", ... 17 locations ...]

def loop():
    PRINT_TEXT(-70, -120, location_names[current_location])
```
✅ Compiles successfully (7602 bytes)
✅ All 17 strings emitted correctly
✅ Dynamic location name display works

## Key Insights

1. **No PRINT_TEXT changes needed**: Function already expects pointer, string array indexing now returns pointer → perfect match

2. **Semantic distinction**: Number vs string arrays handled at indexing time, not emission time. Both use `LDD ,X` but semantic meaning differs:
   - Number array: Load VALUE from ROM
   - String array: Load POINTER from table

3. **Zero overhead**: String arrays don't allocate RAM. Pointer table and strings both in ROM.

4. **Backward compatible**: Number arrays continue working unchanged.

## Limitations & Future Work

- ⚠️ Mixed arrays not supported: `["hello", 123]` will fail
- ⚠️ Nested arrays not supported: `[["a", "b"], ["c", "d"]]`
- 💡 Future: Multi-dimensional string arrays
- 💡 Future: String concatenation/manipulation

## Commit Message Suggestion
```
feat: Implement const string arrays with pointer tables

- Add const_string_arrays tracking to CodegenOptions
- Dual emission: FCC strings + FDB pointer table for string arrays
- Indexing returns pointer for string arrays (not value)
- PRINT_TEXT works seamlessly with string array results
- Tested with 17-location array in Pang game (7.6KB binary)
- Zero RAM overhead, all data in ROM
```

---
**Implementation Date**: 2025-12-27
**Status**: ✅ Complete and tested
**Binary Size Impact**: Minimal (~40 bytes overhead for pointer table vs direct strings)
