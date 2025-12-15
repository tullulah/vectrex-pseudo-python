# LineMap Implementation - Final Phase

## Context
Session Oct 18, 2025: Implementing line tracking to enable VPy debugger breakpoints.

## Root Cause Discovered
- `.pdb` file had `"lineMap": {}` - completely empty
- Frontend MonacoEditorWrapper syncs breakpoints using `pdbData.lineMap[vpyLine]`
- Without lineMap, breakpoints never get addresses → never stop execution

## Infrastructure Built (COMPLETED ✅)
- `Stmt` enum updated with `source_line: usize` in ALL 12 variants  
- `Parser` captures `token.line` and passes to all Stmt constructors
- `Stmt::source_line()` helper method implemented
- ALL pattern matches fixed (~200 code locations)
- Break/Continue converted from unit → struct variants
- Tuple variants (Expr, Return) updated with line parameter
- Compiler builds successfully

## Final Integration (IN PROGRESS ⏸️)

### What We Have
- `LineTracker` struct in `backend/debug_info.rs` with:
  - `set_line(line: usize)` - Records line → address mapping
  - `current_address: u16` - Tracks code generation progress
  - `debug_info: DebugInfo` - Accumulated lineMap

### What We Need
**Wire LineTracker through code generation pipeline:**

```rust
// backend/m6809.rs

// 1. emit_with_debug (line ~59)
pub fn emit_with_debug(...) -> (String, DebugInfo) {
    let mut tracker = LineTracker::new(source_name, binary_name, 0xC800);
    // ... existing code ...
    emit_function(f, &mut out, &string_map, opts, &mut tracker); // ← ADD tracker
    // ...
    (out, tracker.debug_info) // ← Return collected debug info
}

// 2. emit_function (line ~857)
fn emit_function(
    f: &Function, 
    out: &mut String, 
    string_map: &BTreeMap<String,String>, 
    opts: &CodegenOptions,
    tracker: &mut LineTracker  // ← ADD parameter
) {
    // ... existing code ...
    for stmt in &f.body {
        emit_stmt(stmt, out, &LoopCtx::default(), &fctx, string_map, opts, tracker); // ← ADD tracker
    }
}

// 3. emit_stmt (line ~1324) - CRITICAL
fn emit_stmt(
    stmt: &Stmt, 
    out: &mut String, 
    loop_ctx: &LoopCtx, 
    fctx: &FuncCtx, 
    string_map: &BTreeMap<String,String>, 
    opts: &CodegenOptions,
    tracker: &mut LineTracker  // ← ADD parameter
) {
    // ✅ CRITICAL: Record line BEFORE emitting code
    tracker.set_line(stmt.source_line());
    
    // Existing match logic...
    match stmt {
        Stmt::Expr(call, _) => { /* ... */ }
        // Recursive calls to emit_stmt must pass tracker:
        Stmt::While { cond, body, .. } => {
            // ...
            for s in body {
                emit_stmt(s, out, &new_loop_ctx, fctx, string_map, opts, tracker); // ← Pass tracker
            }
        }
        // ... all other variants
    }
}
```

### Expected Result
After implementation, compiling `test_debug_simple.vpy` should produce:

```json
// test_debug_simple.pdb
{
  "version": "1.0",
  "source": "test_debug_simple.vpy",
  "lineMap": {
    "7": "0xC810",   // DEBUG_PRINT(42)
    "10": "0xC820",  // PRINT_TEXT(-20, 0, "DEBUG")
    "12": "0xC830",  // MOVE(0, 0)
    "13": "0xC840"   // DRAW_TO(50, 0)
  },
  "functions": {
    "loop": { "address": "0x0057" },
    "main": { "address": "0x004B" }
  }
}
```

### Implementation Checklist
- [ ] Add `tracker: &mut LineTracker` parameter to `emit_function`
- [ ] Add `tracker: &mut LineTracker` parameter to `emit_stmt`  
- [ ] Call `tracker.set_line(stmt.source_line())` at start of `emit_stmt`
- [ ] Update ALL `emit_stmt` call sites to pass `tracker` (~15 locations estimated)
- [ ] Update `emit_function` call sites to pass `tracker` (1 location)
- [ ] Test compilation: `cargo build -p vectrex_lang`
- [ ] Test .pdb generation: compile test_debug_simple.vpy
- [ ] Verify lineMap populated in .pdb file
- [ ] Test frontend breakpoint detection (F9 → address lookup)
- [ ] Test end-to-end: breakpoint stops execution at correct line

### Files to Modify
- `core/src/backend/m6809.rs` - Primary integration (emit_function, emit_stmt)
- Potentially `core/src/backend/cortexm.rs` and `arm.rs` for consistency (defer)

### Success Criteria
✅ Compiler builds without errors  
✅ .pdb file contains populated lineMap  
✅ MonacoEditorWrapper resolves lines to addresses  
✅ EmulatorPanel receives breakpoints  
✅ Execution pauses when PC matches breakpoint address  
✅ Yellow highlight appears on correct source line  

### Timeline
- **Phase 1**: Add tracker parameters + fix compilation (~30 min)
- **Phase 2**: Test .pdb generation (~15 min)
- **Phase 3**: End-to-end breakpoint testing (~30 min)
- **Total estimated**: 1-2 hours

### Notes
- LineTracker automatically calls `debug_info.add_line_mapping()` in `set_line()`
- Address tracking: need to call `tracker.advance(bytes)` after emitting instructions (future enhancement)
- For now, just recording line → initial address is sufficient for breakpoints
- Recursive emit_stmt calls in loops/ifs must pass tracker through

---
Last updated: 2025-10-18 (Session: Phase 6 breakpoints implementation)
