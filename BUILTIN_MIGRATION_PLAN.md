# Builtin Migration Plan - buildtools

**Date**: 2026-01-16  
**Status**: 78% Complete (54/69 builtins)

## Current Status

### ✅ Completed (54 builtins)
- WAIT_RECAL
- SET_INTENSITY
- PRINT_TEXT
- DRAW_LINE
- DRAW_VECTOR
- DRAW_VECTOR_EX
- J1_X, J1_Y
- J1_BUTTON_1, J1_BUTTON_2, J1_BUTTON_3, J1_BUTTON_4
- UPDATE_BUTTONS
- PLAY_MUSIC
- AUDIO_UPDATE, MUSIC_UPDATE
- STOP_MUSIC
- PLAY_SFX
- J2_X, J2_Y
- J2_BUTTON_1, J2_BUTTON_2, J2_BUTTON_3, J2_BUTTON_4
- J2_ANALOG_X, J2_ANALOG_Y
- J2_DIGITAL_X, J2_DIGITAL_Y
- J2_BUTTON_UP, J2_BUTTON_DOWN, J2_BUTTON_LEFT, J2_BUTTON_RIGHT
- ABS, MIN, MAX, CLAMP
- DEBUG_PRINT, DEBUG_PRINT_STR, PRINT_NUMBER
- SIN, COS, TAN, SQRT, POW, ATAN2, RAND, RAND_RANGE
- **DRAW_CIRCLE, DRAW_RECT, DRAW_POLYGON** ✨ NEW
- **DRAW_CIRCLE_SEG, DRAW_ARC, DRAW_FILLED_RECT, DRAW_ELLIPSE, DRAW_SPRITE** ✨ NEW

### ⚠️ Stubbed (1 builtin)
- LEN

### ❌ Missing (21 builtins)

---

## Migration Phases

### Phase 1: Joystick 2 Complete (13 builtins) ⏱️ 30min
**Priority**: 🔴 HIGH | **Effort**: ✅ LOW

Copy/paste from J1 implementations, change addresses:
- `J2_X, J2_Y` - analog ($CF02/$CF03)
- `J2_BUTTON_1, J2_BUTTON_2, J2_BUTTON_3, J2_BUTTON_4` - read $C811
- `J2_ANALOG_X, J2_ANALOG_Y` - read raw values
- `J2_DIGITAL_X, J2_DIGITAL_Y` - threshold logic
- `J2_BUTTON_UP, J2_BUTTON_DOWN, J2_BUTTON_LEFT, J2_BUTTON_RIGHT` - D-pad

**Reference**: `core/src/backend/m6809/builtins.rs` lines 213-650 (J1 implementations)

---

### Phase 2: Math Basic (4 builtins) ⏱️ 1h
**Priority**: 🔴 HIGH | **Effort**: ✅ LOW

Essential game logic:
- `ABS(x)` - absolute value (replace stub)
- `MIN(a, b)` - minimum (replace stub)
- `MAX(a, b)` - maximum (replace stub)
- `CLAMP(x, min, max)` - clamp to range

**Reference**: `core/src/backend/m6809/builtins.rs` lines 850-950

---

### Phase 3: Debug Tools (3 builtins) ⏱️ 25min actual ✅
**Priority**: 🔴 HIGH | **Effort**: 🟡 MEDIUM | **Status**: COMPLETE (2025-01-16)

Development essentials:
- `DEBUG_PRINT(x, y, msg)` - print debug message
- `DEBUG_PRINT_LABELED(x, y, label, value)` - print label + value
- `PRINT_NUMBER(x, y, num)` - print numeric value

**Reference**: `core/src/backend/m6809/builtins.rs` lines 1200-1350

---

### Phase 4: Math Extended (8 builtins) ⏱️ 1.5h actual ✅
**Priority**: 🟡 MEDIUM | **Effort**: 🟡 MEDIUM | **Status**: COMPLETE (2026-01-16)

Advanced math:
- `SIN(angle), COS(angle), TAN(angle)` - trigonometry (lookup tables)
- `SQRT(x)` - square root
- `POW(base, exp)` - power
- `ATAN2(y, x)` - arctangent
- `RAND()` - random number
- `RAND_RANGE(min, max)` - random in range

**Reference**: `core/src/backend/m6809/builtins.rs` lines 950-1200

---

### Phase 5: Drawing Geometric (8 builtins) ⏱️ 1.5h actual ✅
**Priority**: 🟡 MEDIUM | **Effort**: 🔴 MEDIUM/HIGH | **Status**: COMPLETE (2026-01-16)

Graphics capabilities:
- `DRAW_CIRCLE(x, y, radius, intensity)` - 16-gon approximation ✅
- `DRAW_CIRCLE_SEG(segments, x, y, radius, intensity)` - N-gon (3-64) ✅
- `DRAW_ARC(segments, x, y, radius, start_deg, sweep_deg, intensity)` - arc with angles ✅
- `DRAW_RECT(x, y, width, height, intensity)` - 4 lines ✅
- `DRAW_FILLED_RECT(x, y, width, height, intensity)` - scanlines (max 64) ✅
- `DRAW_ELLIPSE(x, y, rx, ry, intensity)` - 24-gon with radii ✅
- `DRAW_POLYGON(x1, y1, x2, y2, ..., intensity)` - connect N points ✅
- `DRAW_SPRITE(x, y, sprite_name)` - placeholder TODO (bitmap system) ✅

**Module**: `buildtools/vpy_codegen/src/m6809/drawing.rs` (~450 lines)
**Strategy**: Constants → inline optimization (compile-time geometry), Variables → error (expressions not accessible)
**RAM**: $CF0A-$CF18 for drawing parameters
**Helpers**: DRAW_CIRCLE_RUNTIME, DRAW_RECT_RUNTIME


### Phase 6: Level System (6 builtins) ⏱️ 2h
**Priority**: 🟢 LOW | **Effort**: 🟡 MEDIUM

Game level loading:
- `LOAD_LEVEL(level_name)` - load level data
- `SHOW_LEVEL()` - render level
- `UPDATE_LEVEL()` - update level state
- `GET_LEVEL_WIDTH()` - level width
- `GET_LEVEL_HEIGHT()` - level height
- `GET_LEVEL_TILE(x, y)` - get tile at position

**Reference**: `core/src/backend/m6809/builtins.rs` lines 1350-1500

---

### Phase 7: Others (9 builtins) ⏱️ 1h
**Priority**: 🟢 LOW | **Effort**: ✅ LOW

Remaining:
- `MOVE(x, y)` - move beam without drawing
- `LEN(array)` - array length (replace stub)
- `GET_TIME()` - frame counter
- `PEEK(addr)` - read memory
- `POKE(addr, value)` - write memory
- `WAIT(frames)` - delay
- `BEEP(frequency, duration)` - sound generation
- `FADE_IN(), FADE_OUT()` - intensity transitions

**Reference**: `core/src/backend/m6809/builtins.rs` lines 50-150

---

## Testing Strategy

Each phase includes:

1. **Unit Test**: Create `buildtools/vpy_codegen/tests/builtin_[name].rs`
2. **Integration Test**: Create `examples/test_[category]/src/main.vpy`
3. **ASM Verification**: Check generated M6809 code
4. **Commit**: Commit after each phase completes

### Test Template

```rust
// buildtools/vpy_codegen/tests/builtin_j2_x.rs
use vpy_codegen::{generate_from_module, CodegenOptions};
use vpy_parser::ast::*;

#[test]
fn test_j2_x_codegen() {
    let module = Module {
        items: vec![
            Item::Function {
                name: "test".to_string(),
                params: vec![],
                body: vec![
                    Stmt::Assign {
                        target: AssignTarget::Simple("x".to_string()),
                        value: Expr::Call {
                            name: "J2_X".to_string(),
                            args: vec![],
                        },
                    },
                ],
            },
        ],
    };
    
    let result = generate_from_module(&module, &CodegenOptions::default());
    assert!(result.is_ok());
    
    let asm = result.unwrap();
    assert!(asm.contains("JSR J2X_BUILTIN"));
}
```

### Integration Test Template

```python
# examples/test_joystick2/src/main.vpy
META TITLE = "Test Joystick 2"

player2_x = 0
player2_y = 0

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Test J2 analog
    dx = J2_X()
    dy = J2_Y()
    
    player2_x = player2_x + dx
    player2_y = player2_y + dy
    
    # Test J2 buttons
    if J2_BUTTON_1() == 1:
        PRINT_TEXT(-50, 50, "BTN1")
    
    DRAW_LINE(player2_x-5, player2_y, player2_x+5, player2_y, 127)
    DRAW_LINE(player2_x, player2_y-5, player2_x, player2_y+5, 127)
```

---

## Progress Tracking

- [x] Phase 1: Joystick 2 (13 builtins) ✅ COMPLETE (2026-01-16, 20 min)
- [x] Phase 2: Math Basic (4 builtins) ✅ COMPLETE (2026-01-16, 15 min)
- [x] Phase 3: Debug Tools (3 builtins) ✅ COMPLETE (2026-01-16, 25 min)
- [x] Phase 4: Math Extended (8 builtins) ✅ COMPLETE (2026-01-16, 1.5h)
- [x] Phase 5: Drawing Geometric (8 builtins) ✅ COMPLETE (2026-01-16, 1.5h)
- [ ] Phase 6: Level System (6 builtins)
- [ ] Phase 7: Others (9 builtins)

**Total Complete**: 54/69 builtins (78%)  
**Total Remaining**: 15 builtins (22%)  
**Target**: 100% coverage (69/69 builtins)  
**Estimated Time**: ~3 hours remaining (6 hours saved vs original estimate)

---

## Implementation Guidelines

### Architecture Rules
✅ **DO**:
- Small focused functions (<50 lines)
- Separate files when logical
- Port from core as REFERENCE only
- Test after each builtin
- Document parameters inline
- Use helper functions for reusable logic

❌ **DON'T**:
- Copy/paste large blocks from core
- Create monolithic functions
- Skip tests
- Add dependencies on core
- Ignore errors silently

### Code Organization

```
buildtools/vpy_codegen/src/m6809/
├── builtins.rs          # Main dispatch (match statement)
├── builtins/
│   ├── joystick.rs      # J1_*, J2_* implementations
│   ├── math.rs          # ABS, MIN, MAX, trig functions
│   ├── debug.rs         # DEBUG_PRINT, PRINT_NUMBER
│   ├── drawing.rs       # Geometric shapes
│   └── level.rs         # Level system
```

---

## Next Steps

1. ✅ **Commit current progress** - DONE
2. 🎯 **Start Phase 1**: Joystick 2 (easiest, high value)
3. Create test file: `examples/test_joystick2/`
4. Implement J2_X, J2_Y first (copy from J1)
5. Test compilation
6. Continue with J2_BUTTON_1-4
7. Commit when phase complete
