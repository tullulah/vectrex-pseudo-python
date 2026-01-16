# Phase 3: VPy Unifier Design

## Overview
**Purpose**: Load multiple .vpy files, resolve imports, and merge into a single unified AST
**Input**: File system + parsed Module structs from Phase 2
**Output**: Single UnifiedModule ready for Phase 4 (codegen)
**Complexity**: Medium - Graph-based dependency resolution

## Architecture

### 1. Module Dependency Graph

```
main.vpy
├── imports input.vpy
│   ├── imports utils.vpy
│   └── imports math.vpy
├── imports graphics.vpy
│   └── imports vec_resources.vpy (if needed)
└── imports config.vpy
```

**Detection**: Parse `import X` statements to build graph
**Validation**: Detect circular imports before merging
**Resolution**: Topological sort for merge order

### 2. Symbol Namespace Management

**Before Merge**:
```
input.vpy:
  - function get_input() -> INPUT_GET_INPUT after merge
  - variable input_state -> INPUT_INPUT_STATE

graphics.vpy:
  - function draw_square() -> GRAPHICS_DRAW_SQUARE
  - variable screen_x -> GRAPHICS_SCREEN_X

main.vpy:
  - function main() -> MAIN (no prefix, entry module)
  - function loop() -> LOOP
```

**Rules**:
1. Entry module (main.vpy): NO prefix
2. Imported modules: `UPPERCASE_MODULENAME_SYMBOL` format
3. Avoid collisions: Hash all module names first (O(n))

### 3. Merge Process

```
Phase 3.1: Load all .vpy files from project root recursively
Phase 3.2: Build import graph (detect cycles)
Phase 3.3: Topologically sort modules (dependencies first)
Phase 3.4: Merge items (functions, variables) with renamed symbols
Phase 3.5: Fix references (update call sites, variable accesses)
Phase 3.6: Validate cross-module references exist
Phase 3.7: Return UnifiedModule (single Module with all items)
```

## Implementation Plan

### Step 1: Core Types (0.5 hours)

```rust
// error.rs
pub enum UnifierError {
    CircularImport(Vec<String>),
    ModuleNotFound(String),
    SymbolConflict(String, String),
    InvalidImport(String),
    UnresolvedSymbol(String),
}

// graph.rs (NEW)
pub struct ModuleGraph {
    modules: HashMap<String, Module>,
    dependencies: HashMap<String, Vec<String>>,
}

impl ModuleGraph {
    pub fn new() -> Self { ... }
    pub fn add_module(&mut self, name: String, module: Module) { ... }
    pub fn add_dependency(&mut self, from: String, to: String) { ... }
    pub fn detect_cycles(&self) -> Option<Vec<String>> { ... }
    pub fn topological_sort(&self) -> UnifierResult<Vec<String>> { ... }
}

// resolver.rs (NEW)
pub struct SymbolResolver {
    original_names: HashMap<String, String>,  // resolved -> original
    module_prefixes: HashMap<String, String>, // module -> prefix
}

impl SymbolResolver {
    pub fn new() -> Self { ... }
    pub fn register_module(&mut self, module: &str) -> String { ... }
    pub fn resolve_symbol(&self, symbol: &str, module: &str) -> String { ... }
}
```

### Step 2: Graph Building (1 hour)

```rust
// Phase 3.1-3.2: Load and detect cycles
pub fn load_and_build_graph(project_path: &Path) -> UnifierResult<ModuleGraph> {
    let mut graph = ModuleGraph::new();
    
    // Load all .vpy files recursively
    for entry in recursive_find_vpy(project_path)? {
        let module = parse_file(&entry)?;
        let module_name = entry.file_stem().to_string();
        graph.add_module(module_name, module);
    }
    
    // Extract import statements and build dependency edges
    for (name, module) in &graph.modules {
        for item in &module.items {
            if let Item::Import(import_name) = item {
                graph.add_dependency(name.clone(), import_name.clone())?;
            }
        }
    }
    
    // Validate no cycles
    if let Some(cycle) = graph.detect_cycles() {
        return Err(UnifierError::CircularImport(cycle));
    }
    
    Ok(graph)
}
```

### Step 3: Symbol Renaming (0.5 hours)

```rust
// Phase 3.3-3.4: Rename symbols and merge
pub fn merge_modules(
    graph: ModuleGraph,
    entry_module: &str,
) -> UnifierResult<Module> {
    let mut resolver = SymbolResolver::new();
    let sort_order = graph.topological_sort()?;
    let mut merged_items = Vec::new();
    let mut merged_meta = ModuleMeta::default();
    
    for module_name in sort_order {
        let module = graph.modules.get(&module_name).unwrap();
        let prefix = resolver.register_module(&module_name);
        
        for item in &module.items {
            // Rename symbols based on module prefix
            let renamed_item = rename_symbols_in_item(item, &prefix, &module_name, entry_module)?;
            merged_items.push(renamed_item);
        }
        
        // Merge metadata (first defined wins, or configured priority)
        if module_name == entry_module {
            merged_meta = module.meta.clone();
        }
    }
    
    Ok(Module {
        items: merged_items,
        meta: merged_meta,
    })
}
```

### Step 4: Reference Fixing (1 hour)

```rust
// Phase 3.5: Update all references to use renamed symbols
pub fn fix_cross_module_references(
    module: &mut Module,
    resolver: &SymbolResolver,
) -> UnifierResult<()> {
    let mut visitor = ReferenceFixerVisitor::new(resolver);
    
    for item in &mut module.items {
        visitor.visit_item(item)?;
    }
    
    Ok(())
}

// Example: convert input.get_input() → INPUT_GET_INPUT()
pub fn fix_field_access(
    expr: &mut Expr,
    resolver: &SymbolResolver,
) -> UnifierResult<()> {
    match expr {
        Expr::FieldAccess(module_name, symbol) => {
            let prefix = resolver.resolve_module_prefix(module_name)?;
            *expr = Expr::Identifier(format!("{}_{}", prefix, symbol));
        }
        Expr::Call(target, args) => {
            fix_field_access(target, resolver)?;
            for arg in args {
                fix_field_access(arg, resolver)?;
            }
        }
        // ... other cases
        _ => {}
    }
    Ok(())
}
```

### Step 5: Validation (0.5 hours)

```rust
// Phase 3.6: Verify all references are satisfied
pub fn validate_references(module: &Module) -> UnifierResult<()> {
    let mut collector = SymbolCollector::new();
    
    // Collect all defined symbols
    for item in &module.items {
        collector.collect_definitions(item);
    }
    
    // Verify all references are defined
    let mut visitor = ReferenceValidator::new(collector.symbols);
    for item in &module.items {
        visitor.validate_item(item)?;
    }
    
    Ok(())
}
```

## Testing Strategy

### Test 1: Single Module (No Imports)
```python
# main.vpy
def main(): pass
def loop(): pass
```
→ Should pass through unchanged

### Test 2: Two-Module Import
```python
# input.vpy
def get_input(): pass

# main.vpy
import input
def loop():
    input.get_input()
```
→ After merge: `INPUT_GET_INPUT()` (call updated)

### Test 3: Circular Import Detection
```python
# a.vpy
import b

# b.vpy
import a
```
→ Should error: CircularImport([a, b])

### Test 4: Multi-Level Imports
```python
# utils.vpy (no imports)
def helper(): pass

# input.vpy
import utils
def get_input():
    utils.helper()

# main.vpy
import input
def loop():
    input.get_input()
```
→ Topological sort: [utils, input, main]
→ Merged calls: `UTILS_HELPER()`, `INPUT_GET_INPUT()`, etc.

### Test 5: Symbol Conflict Detection
```python
# module_a.vpy
player_x = 0

# module_b.vpy
player_x = 0

# main.vpy
import module_a
import module_b
```
→ Renamed to: `MODULEA_PLAYER_X`, `MODULEB_PLAYER_X` (no conflict)

## Timeline

| Phase | Task | Time | Status |
|-------|------|------|--------|
| 3.1 | Core types (error, graph, resolver) | 0.5h | ⏳ TODO |
| 3.2 | Graph building (load, parse, cycles) | 1h | ⏳ TODO |
| 3.3 | Symbol renaming (merge, rename) | 0.5h | ⏳ TODO |
| 3.4 | Reference fixing (visitors) | 1h | ⏳ TODO |
| 3.5 | Validation & error handling | 0.5h | ⏳ TODO |
| 3.6 | Comprehensive tests | 0.5h | ⏳ TODO |
| **TOTAL** | Phase 3 Complete | **3.5h** | ⏳ TODO |

## Known Challenges

1. **Circular Import Detection**: Need DFS cycle detection (O(V+E))
2. **Symbol Resolver**: Must handle nested references (module.module.symbol pattern)
3. **Reference Fixing**: AST visitor must traverse ALL expression types
4. **Entry Point**: main.vpy must be entry; ensure it exists and contains main()/loop()
5. **Import Order**: Topological sort handles dependencies but need stable order

## Success Criteria

- ✅ Load all .vpy files from project recursively
- ✅ Detect and reject circular imports
- ✅ Rename symbols correctly (MODULE_NAME_SYMBOL format)
- ✅ Update all references (calls, variable access, imports)
- ✅ Validate no unresolved symbols remain
- ✅ Single unified Module output ready for Phase 4
- ✅ 5+ comprehensive tests passing
- ✅ buildtools cargo test: 41+ total tests
- ✅ Zero compiler warnings

---
Created: 2026-01-16
Status: Ready for implementation
Next: Start Phase 3.1 (core types)
