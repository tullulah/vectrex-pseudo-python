# Linker Phase 6: VPy Module System

## Overview

Phase 6 implements a complete module system for VPy, enabling:
- Multi-file projects with `import` statements
- Automatic dependency resolution and compilation
- Per-module symbol tables and relocations
- Integrated build pipeline (compile → link)

## Motivation

**Current Problem**: VPy compiler generates monolithic code with global RAM layout:
```python
# game.vpy
def draw_player():
    SET_INTENSITY(127)  # References VAR_ARG0 (global)
    # ... generates LDA VAR_ARG0+1
```

When compiling to object mode:
```
❌ Error: SYMBOL:VAR_ARG0+1
```

**Why**: Compiler assumes all variables are defined in a single program.

**Solution**: Module system with explicit imports/exports.

## Design Goals

1. **Standard syntax**: Similar to Python's `import`
2. **Automatic linking**: Build command handles multi-file compilation
3. **Backward compatible**: Single-file programs still work
4. **Shared builtins**: VAR_ARG0, RESULT, etc. in common runtime section
5. **Per-module RAM**: Each module gets its own variable space

## Phase 6.1: Import Syntax

### Grammar Extension

```python
# Import entire module
import graphics

# Import specific functions
from physics import apply_gravity, check_collision

# Import with alias
import sound as sfx
from input import read_joystick as get_joy
```

### AST Changes

**New node types** (`core/src/ast.rs`):
```rust
pub enum Item {
    // ... existing variants
    Import {
        module: String,              // "graphics"
        alias: Option<String>,       // Some("gfx")
        source_line: usize,
    },
    ImportFrom {
        module: String,              // "physics"
        names: Vec<ImportName>,      // [(apply_gravity, None), ...]
        source_line: usize,
    },
}

pub struct ImportName {
    pub name: String,           // "apply_gravity"
    pub alias: Option<String>,  // Some("apply_grav")
}
```

### Parser Implementation

**Tokenizer** (`core/src/lexer.rs`):
- Already has `import` as keyword ✅
- Add `from` keyword
- Add `as` keyword

**Parser** (`core/src/parser.rs`):
```rust
fn parse_import(&mut self) -> Result<Item> {
    self.expect(TokenKind::Keyword("import"))?;
    
    // Check if it's "from ... import ..."
    if self.peek_is(TokenKind::Identifier(_)) {
        let module = self.parse_identifier()?;
        
        if self.consume_if(TokenKind::Keyword("as")) {
            let alias = self.parse_identifier()?;
            return Ok(Item::Import { module, alias: Some(alias), source_line });
        }
        
        return Ok(Item::Import { module, alias: None, source_line });
    }
    
    Err("Expected module name after 'import'")
}

fn parse_from_import(&mut self) -> Result<Item> {
    self.expect(TokenKind::Keyword("from"))?;
    let module = self.parse_identifier()?;
    self.expect(TokenKind::Keyword("import"))?;
    
    let mut names = Vec::new();
    loop {
        let name = self.parse_identifier()?;
        let alias = if self.consume_if(TokenKind::Keyword("as")) {
            Some(self.parse_identifier()?)
        } else {
            None
        };
        names.push(ImportName { name, alias });
        
        if !self.consume_if(TokenKind::Comma) {
            break;
        }
    }
    
    Ok(Item::ImportFrom { module, names, source_line })
}
```

## Phase 6.2: Module Resolution

### Module Lookup Strategy

**Search paths** (in order):
1. Same directory as importing file
2. Project root `src/` directory
3. Standard library path (future: `stdlib/`)

**Example**:
```
project/
├── src/
│   ├── main.vpy         # import graphics, physics
│   ├── graphics.vpy     # Exports: draw_player, draw_enemy
│   └── physics.vpy      # Exports: apply_gravity, check_collision
└── build/
    ├── main.vo
    ├── graphics.vo
    └── physics.vo
```

### Module Metadata

**New struct** (`core/src/codegen.rs`):
```rust
pub struct ModuleInfo {
    pub name: String,           // "graphics"
    pub file_path: PathBuf,     // "src/graphics.vpy"
    pub exports: Vec<String>,   // ["draw_player", "draw_enemy"]
    pub imports: Vec<String>,   // ["SET_INTENSITY", "DRAW_LINE"]
}

pub struct ModuleRegistry {
    modules: HashMap<String, ModuleInfo>,
    compiled: HashSet<String>,  // Already compiled modules
}

impl ModuleRegistry {
    pub fn resolve(&mut self, module_name: &str, from_file: &Path) -> Result<PathBuf> {
        // 1. Check cache
        if let Some(info) = self.modules.get(module_name) {
            return Ok(info.file_path.clone());
        }
        
        // 2. Search paths
        let search_paths = vec![
            from_file.parent().unwrap(),
            Path::new("src"),
        ];
        
        for path in search_paths {
            let candidate = path.join(format!("{}.vpy", module_name));
            if candidate.exists() {
                return Ok(candidate);
            }
        }
        
        Err(format!("Module '{}' not found", module_name))
    }
}
```

## Phase 6.3: Symbol Generation

### Export Detection

**Strategy**: All top-level functions are exported by default
```python
# graphics.vpy
def draw_player(x, y):  # ✅ Exported
    pass

def draw_enemy(x, y):   # ✅ Exported
    pass

def _internal_helper(): # ❌ Not exported (starts with _)
    pass
```

**Implementation** (`core/src/backend/m6809/mod.rs`):
```rust
fn collect_exports(module: &Module) -> Vec<Symbol> {
    let mut exports = Vec::new();
    
    for item in &module.items {
        if let Item::Function { name, .. } = item {
            // Skip internal functions (start with _)
            if !name.starts_with('_') {
                exports.push(Symbol {
                    name: name.clone(),
                    section: Some(0),  // Text section
                    offset: 0,         // Filled during codegen
                    scope: SymbolScope::Global,
                    symbol_type: SymbolType::Function,
                });
            }
        }
    }
    
    exports
}
```

### Import References

**Problem**: When main.vpy calls `graphics.draw_player()`:
- Compiler doesn't know where `draw_player` is located
- Must generate placeholder + relocation

**Solution**: Track imported functions
```rust
struct ImportedFunction {
    name: String,           // "draw_player"
    module: String,         // "graphics"
    call_sites: Vec<usize>, // Offsets where function is called
}

// During codegen, when emitting JSR:
if let Some(import) = imported_functions.get(func_name) {
    // Emit JSR $0000 (placeholder)
    out.push_str("    JSR $0000\n");
    
    // Record relocation
    relocations.push(Relocation {
        section: 0,
        offset: current_offset + 1,  // Address bytes of JSR
        reloc_type: RelocationType::Absolute16,
        symbol_name: func_name.to_string(),
        addend: 0,
    });
}
```

## Phase 6.4: Shared Runtime Section

### Problem: Builtin Work Areas

All modules need access to:
- `VAR_ARG0` through `VAR_ARG4` (function arguments)
- `RESULT` (return value)
- `TMPPTR` (temporary pointer)
- Builtin helpers (MUL16, DIV16, etc.)

### Solution: Common Runtime Section

**Generate once, link with all modules**:
```asm
; runtime.asm (generated automatically)
    ORG $C800

; === Work areas (shared by all modules) ===
VAR_ARG0    EQU $C800
VAR_ARG1    EQU $C802
VAR_ARG2    EQU $C804
VAR_ARG3    EQU $C806
VAR_ARG4    EQU $C808
RESULT      EQU $C80A
TMPPTR      EQU $C80C

; === Builtin helpers ===
MUL16:
    ; ... multiplication code
    RTS

DIV16:
    ; ... division code
    RTS

; ... other builtins
```

**Compile to**: `runtime.vo` with exports for all symbols

**Link**: Every project links with `runtime.vo` automatically

## Phase 6.5: Per-Module RAM Allocation

### Strategy

Each module gets its own RAM space for local variables:
```
Memory Layout:
$C800-$C81F: Runtime work areas (shared)
$C820-$C87F: Module A variables
$C880-$C8DF: Module B variables
$C8E0-$C93F: Module C variables
...
```

**Implementation**:
```rust
struct ModuleRamAllocator {
    next_base: u16,         // Next available RAM address
    allocations: HashMap<String, (u16, u16)>, // module -> (base, size)
}

impl ModuleRamAllocator {
    fn new() -> Self {
        Self {
            next_base: 0xC820,  // After runtime work areas
            allocations: HashMap::new(),
        }
    }
    
    fn allocate(&mut self, module: &str, size: usize) -> u16 {
        let base = self.next_base;
        self.allocations.insert(module.to_string(), (base, size as u16));
        self.next_base += size as u16;
        base
    }
}
```

## Phase 6.6: Build Pipeline Integration

### New Build Command

```bash
# Old (single file):
vectrexc build game.vpy --bin

# New (multi-file, auto-detect dependencies):
vectrexc build src/main.vpy --bin
```

**Pipeline**:
```
1. Parse main.vpy
   ↓
2. Discover imports (graphics, physics)
   ↓
3. Recursively parse imported modules
   ↓
4. Build dependency graph
   ↓
5. Compile each module to .vo (topological order)
   ↓
6. Generate runtime.vo
   ↓
7. Link all .vo files → game.bin
```

**Implementation** (`core/src/main.rs`):
```rust
fn build_multi_module(entry: &Path, opts: &BuildOptions) -> Result<()> {
    let mut registry = ModuleRegistry::new();
    let mut compile_queue = Vec::new();
    
    // 1. Discover modules
    discover_dependencies(entry, &mut registry, &mut compile_queue)?;
    
    // 2. Compile modules
    let mut object_files = Vec::new();
    for module_path in compile_queue {
        let vo_path = compile_module_to_object(&module_path, &registry)?;
        object_files.push(vo_path);
    }
    
    // 3. Generate runtime
    let runtime_vo = generate_runtime_vo()?;
    object_files.push(runtime_vo);
    
    // 4. Link
    link_objects(&object_files, &opts.output)?;
    
    Ok(())
}

fn discover_dependencies(
    file: &Path,
    registry: &mut ModuleRegistry,
    queue: &mut Vec<PathBuf>,
) -> Result<()> {
    // Parse file
    let source = fs::read_to_string(file)?;
    let module = parse(&source)?;
    
    // Extract imports
    for item in &module.items {
        match item {
            Item::Import { module, .. } => {
                let path = registry.resolve(module, file)?;
                if !registry.compiled.contains(module) {
                    queue.push(path.clone());
                    discover_dependencies(&path, registry, queue)?;
                }
            }
            Item::ImportFrom { module, .. } => {
                let path = registry.resolve(module, file)?;
                if !registry.compiled.contains(module) {
                    queue.push(path.clone());
                    discover_dependencies(&path, registry, queue)?;
                }
            }
            _ => {}
        }
    }
    
    Ok(())
}
```

## Example Usage

### Project Structure
```
vectrex-game/
├── src/
│   ├── main.vpy        # Entry point
│   ├── graphics.vpy    # Drawing routines
│   ├── physics.vpy     # Game physics
│   └── input.vpy       # Input handling
└── build/
    ├── main.vo
    ├── graphics.vo
    ├── physics.vo
    ├── input.vo
    ├── runtime.vo
    └── game.bin        # Final linked binary
```

### main.vpy
```python
import graphics
import physics
import input

player_x = 0
player_y = 0

def main():
    graphics.init_screen()

def loop():
    WAIT_RECAL()
    
    # Update input
    dx, dy = input.read_joystick()
    player_x += dx
    player_y += dy
    
    # Apply physics
    physics.apply_bounds(player_x, player_y)
    
    # Draw
    graphics.draw_player(player_x, player_y)
```

### graphics.vpy
```python
def init_screen():
    SET_INTENSITY(127)

def draw_player(x, y):
    DRAW_LINE(x-5, y, x+5, y, 100)
    DRAW_LINE(x, y-5, x, y+5, 100)

def draw_enemy(x, y):
    # Draw enemy sprite
    pass
```

### Compilation
```bash
$ vectrexc build src/main.vpy --bin -o game.bin

Analyzing dependencies...
  ✓ main.vpy
  ✓ graphics.vpy (imported by main)
  ✓ physics.vpy (imported by main)
  ✓ input.vpy (imported by main)

Compiling modules...
  [1/4] graphics.vpy → graphics.vo (2 exports, 2 imports)
  [2/4] physics.vpy → physics.vo (1 export, 0 imports)
  [3/4] input.vpy → input.vo (1 export, 2 imports)
  [4/4] main.vpy → main.vo (2 exports, 6 imports)

Generating runtime...
  ✓ runtime.vo (15 exports: VAR_ARG0-4, RESULT, MUL16, DIV16, ...)

Linking...
  ✓ Linked 5 objects
  ✓ Resolved 23 relocations
  ✓ game.bin (3.2 KB)

Build complete!
```

## Implementation Plan

### Phase 6.1: Parser (Week 1) - ✅ COMPLETADO
- [x] Add `from` and `as` keywords to lexer
- [x] Implement `parse_import()` and `parse_from_import()`
- [x] Add `Item::Import` and `Item::ImportFrom` to AST
- [x] Tests for import syntax parsing

### Phase 6.2: Module Resolution (Week 1-2) - ✅ COMPLETADO
- [x] Implement `ModuleRegistry`
- [x] Module path resolution logic
- [x] Dependency discovery (recursive import scanning)
- [x] Tests for module resolution

### Phase 6.3: Symbol Generation (Week 2) - ✅ COMPLETADO
- [x] Export collection from functions
- [x] Import tracking during codegen (via unifier)
- [x] Dot notation transformation (module.symbol → MODULE_SYMBOL)
- [x] Tests for export/import detection (examples/multi-module)

### Phase 6.4: Runtime Section (Week 2) - ⏸️ PAUSED
- [ ] Generate `runtime.vo` with builtin symbols
- [x] Runtime helpers auto-deduplicated by unifier
- [ ] Per-module RAM allocation (not needed - unifier merges)
- [ ] Tests for runtime linking

**Note**: Phase 6.4 paused - current unifier approach sufficient

### Phase 6.5: Build Integration (Week 3) - 🔧 PARCIAL (30%)
- [x] Multi-file import resolution working
- [x] Unified compilation (all modules merged)
- [ ] Per-module .vo generation (alternative approach documented in PHASE6_FUTURE_WORK.md)
- [x] End-to-end test with multi-module project (examples/multi-module)

**Note**: Phase 6.5 infrastructure ready but per-module compilation paused (see PHASE6_FUTURE_WORK.md)

### Phase 6.6: Documentation (Week 3) - ✅ COMPLETADO
- [x] Phase 6 implementation guide (PHASE6_SUMMARY.md)
- [x] Future work roadmap (PHASE6_FUTURE_WORK.md)
- [x] copilot-instructions.md updated
- [x] Examples in `examples/multi-module/`

### Phase 6.7: Multi-Bank ROM (2026-01-11) - ✅ COMPLETADO
- [x] Multi-bank linker includes INCLUDE directives in all banks
- [x] Multi-bank linker includes runtime helpers in all banks
- [x] 512KB ROM generation working (pang_multi compiles successfully)
- [ ] Runtime execution issue (emulator stops at PC=0x8039) - TODO

## Success Criteria

Phase 6 complete when:
- [x] Parser accepts `import` and `from ... import` syntax
- [x] Compiler resolves module dependencies automatically
- [x] Multi-module projects compile (unified approach)
- [x] Dot notation transforms correctly (module.symbol → MODULE_SYMBOL)
- [x] Multi-module project builds successfully (examples/multi-module)
- [x] Documentation updated (PHASE6_SUMMARY.md, PHASE6_FUTURE_WORK.md)
- [x] Example projects included (examples/multi-module/)

**Status**: ✅ Phase 6.1-6.3 COMPLETE, Phase 6.4-6.5 PAUSED (ROI analysis)
**Multi-Bank**: ✅ Compilation working, ⚠️ Runtime issue pending (PC=0x8039)

## Files to Create/Modify

**Created**:
- ✅ `LINKER_PHASE6_DESIGN.md` (this file)
- ✅ `PHASE6_SUMMARY.md` - Implementation guide
- ✅ `PHASE6_FUTURE_WORK.md` - Roadmap for Phase 6.5+
- ✅ `examples/multi-module/` - Example multi-file projects

**Modified**:
- ✅ `core/src/lexer.rs` - Add `from`, `as` keywords
- ✅ `core/src/parser.rs` - Parse import statements
- ✅ `core/src/ast.rs` - Add import AST nodes
- ✅ `core/src/unifier.rs` - Symbol transformation (module.symbol → MODULE_SYMBOL)
- ✅ `core/src/backend/m6809/mod.rs` - Array label collision fix
- ✅ `core/src/backend/m6809/multi_bank_linker.rs` - Include runtime helpers in all banks
- ✅ `core/src/main.rs` - Multi-module build pipeline
- ✅ `.github/copilot-instructions.md` - Section 23 updated

---

**Status**: ✅ Phase 6.1-6.3 COMPLETE (2026-01-11)
**Next Step**: Investigate runtime issue (PC=0x8039) in multi-bank ROM
**Achievement**: Multi-module system fully functional, multi-bank compilation working
