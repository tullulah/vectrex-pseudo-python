# Linker Phase 4: Symbol Resolution & Linking

## Overview

Phase 4 focuses on resolving symbols across multiple object files and linking them into a cohesive program. This phase involves:
1. Modifying the assembler to support "object mode" (allow unresolved symbols)
2. Implementing the SymbolResolver
3. Assigning addresses to all sections and symbols
4. Testing multi-object linking

## Problem Analysis

### Current Limitation
The M6809 assembler (`asm_to_binary.rs`) currently **errors** when it encounters an undefined symbol:
```
❌ SÍMBOLO NO RESUELTO: 'START' (uppercase: 'START') at offset=31
Error: Failed to assemble section .text.header: Símbolo no definido: START
```

This is correct for monolithic compilation (single .bin file), but **incorrect for object files** where:
- Sections are assembled independently
- External symbols (from other .vo files or BIOS) are **expected** to be unresolved
- Relocations record where to patch addresses during linking

### Required Behavior for Object Mode

When assembling for object files, the assembler should:
1. **Allow** unresolved symbols
2. **Emit placeholder** (e.g., 0x0000 for absolute addresses)
3. **Return** list of unresolved references (symbol name, offset, type)
4. **Continue** assembly without errors

Example:
```asm
JSR Wait_Recal    ; Symbol 'Wait_Recal' unresolved
                  ; Emits: BD 00 00 (JSR + placeholder address)
                  ; Records: Relocation { offset: 1, symbol: "Wait_Recal", type: Absolute16 }
```

## Phase 4.1: Assembler Object Mode

### Design

Add `object_mode: bool` parameter to `assemble_m6809()`:
- `false` (default): Current behavior - error on undefined symbols
- `true`: Allow undefined symbols, emit placeholders, return relocations

### Implementation Steps

1. **Modify function signature** (`core/src/backend/asm_to_binary.rs`):
   ```rust
   pub fn assemble_m6809(
       asm: &str, 
       org: u16,
       object_mode: bool,  // NEW parameter
   ) -> Result<(Vec<u8>, Vec<UnresolvedRef>), String>
   ```

2. **Define UnresolvedRef struct**:
   ```rust
   pub struct UnresolvedRef {
       pub symbol: String,
       pub offset: usize,       // Byte offset in assembled output where symbol is referenced
       pub ref_type: RefType,   // Absolute16, Relative8, Relative16
   }
   
   pub enum RefType {
       Absolute16,  // JSR, LDD #, LDX #
       Relative8,   // BRA, BEQ, BNE
       Relative16,  // LBRA, LBEQ
   }
   ```

3. **Modify symbol lookup** in assembler:
   - When symbol not found:
     - If `object_mode == false`: Return error (current behavior)
     - If `object_mode == true`: 
       - Emit placeholder (0x0000 for absolute, 0x00 for relative)
       - Add to unresolved_refs list
       - Continue assembly

4. **Update all callers**:
   - Monolithic compilation: `assemble_m6809(asm, org, false)`
   - Object file generation: `assemble_m6809(asm, org, true)`

### Testing

Create test in `asm_to_binary.rs`:
```rust
#[test]
fn test_object_mode_unresolved_symbols() {
    let asm = r#"
        JSR External_Func
        LDD #EXTERNAL_VAR
        RTS
    "#;
    
    let (binary, unresolved) = assemble_m6809(asm, 0x0000, true).unwrap();
    
    // Verify binary contains placeholders
    assert_eq!(binary[0], 0xBD);  // JSR opcode
    assert_eq!(binary[1], 0x00);  // Placeholder high byte
    assert_eq!(binary[2], 0x00);  // Placeholder low byte
    
    // Verify unresolved list
    assert_eq!(unresolved.len(), 2);
    assert_eq!(unresolved[0].symbol, "External_Func");
    assert_eq!(unresolved[0].offset, 1);
    assert_eq!(unresolved[0].ref_type, RefType::Absolute16);
}
```

## Phase 4.2: SymbolResolver Implementation ✅ COMPLETE (2025-01-02)

**Status**: ✅ Implemented and tested

**Commit**: 1e0de558 - "feat: linker Phase 4.2 - SymbolResolver implementation"

### Purpose
Merge symbol tables from multiple .vo files and detect conflicts.

### Design

```rust
// core/src/linker/resolver.rs

pub struct GlobalSymbolTable {
    pub symbols: HashMap<String, ResolvedSymbol>,
}

pub struct ResolvedSymbol {
    pub name: String,
    pub address: u16,           // Assigned address (after linking)
    pub section: String,        // Which section it belongs to
    pub source_file: String,    // Which .vo file it came from
}

impl SymbolResolver {
    /// Collect all exports from all object files
    pub fn collect_symbols(objects: &[VectrexObject]) -> Result<GlobalSymbolTable, String> {
        let mut global = GlobalSymbolTable { symbols: HashMap::new() };
        
        for obj in objects {
            for (name, symbol) in &obj.symbols.exports {
                // Check for duplicate definitions
                if global.symbols.contains_key(name) {
                    return Err(format!(
                        "Duplicate symbol '{}' defined in {} and {}",
                        name, 
                        global.symbols[name].source_file,
                        obj.header.source_file
                    ));
                }
                
                // Add to global table (address not yet assigned)
                global.symbols.insert(name.clone(), ResolvedSymbol {
                    name: name.clone(),
                    address: 0,  // Will be assigned in Phase 4.3
                    section: symbol.section.clone(),
                    source_file: obj.header.source_file.clone(),
                });
            }
        }
        
        Ok(global)
    }
    
    /// Verify all imports have matching exports
    pub fn verify_imports(
        objects: &[VectrexObject], 
        global: &GlobalSymbolTable
    ) -> Result<(), String> {
        for obj in objects {
            for import_name in obj.symbols.imports.keys() {
                if !global.symbols.contains_key(import_name) {
                    return Err(format!(
                        "Undefined reference to '{}' in {}",
                        import_name,
                        obj.header.source_file
                    ));
                }
            }
        }
        Ok(())
    }
}
```

### Testing

```rust
#[test]
fn test_symbol_resolver_duplicate_detection() {
    let obj1 = create_test_object("main.vo", vec!["MAIN"]);
    let obj2 = create_test_object("lib.vo", vec!["MAIN"]);  // Duplicate!
    
    let result = SymbolResolver::collect_symbols(&[obj1, obj2]);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Duplicate symbol 'MAIN'"));
}

#[test]
fn test_symbol_resolver_undefined_import() {
    let obj = create_test_object_with_import("main.vo", "UNDEFINED_FUNC");
    let global = GlobalSymbolTable { symbols: HashMap::new() };
    
    let result = SymbolResolver::verify_imports(&[obj], &global);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Undefined reference to 'UNDEFINED_FUNC'"));
}
```

## Phase 4.3: Address Assignment

### Purpose
Assign absolute addresses to all sections and symbols.

### Design

```rust
impl SymbolResolver {
    /// Assign addresses to all sections and update symbol table
    pub fn assign_addresses(
        objects: &mut [VectrexObject],
        global: &mut GlobalSymbolTable,
        base_address: u16,
    ) -> Result<(), String> {
        let mut current_address = base_address;
        
        for obj in objects.iter_mut() {
            for section in &mut obj.sections {
                // Assign section base address
                section.base_address = Some(current_address);
                
                // Update symbols in this section
                for (name, symbol) in &obj.symbols.exports {
                    if symbol.section == section.name {
                        let symbol_address = current_address + symbol.value;
                        
                        // Update global table
                        if let Some(global_sym) = global.symbols.get_mut(name) {
                            global_sym.address = symbol_address;
                        }
                    }
                }
                
                // Advance address for next section
                current_address += section.data.len() as u16;
                
                // Apply alignment if needed
                let alignment = section.alignment as u16;
                if alignment > 1 {
                    current_address = (current_address + alignment - 1) / alignment * alignment;
                }
            }
        }
        
        Ok(())
    }
}
```

### Example

Given two object files:
```
main.vo:
  .text.main: size=100 bytes
    MAIN @ offset 0
    call_helper @ offset 50

lib.vo:
  .text.helper: size=50 bytes
    helper_function @ offset 0
```

After address assignment (base=0xC880):
```
main.vo:
  .text.main: base=0xC880
    MAIN: 0xC880
    call_helper: 0xC8B2
    
lib.vo:
  .text.helper: base=0xC8E4 (0xC880 + 100)
    helper_function: 0xC8E4
```

## Phase 4.4: Relocation Patching

### Purpose
Patch all placeholder addresses with resolved symbol addresses.

### Design

```rust
impl SymbolResolver {
    /// Apply relocations using resolved symbols
    pub fn apply_relocations(
        objects: &mut [VectrexObject],
        global: &GlobalSymbolTable,
    ) -> Result<(), String> {
        for obj in objects.iter_mut() {
            for reloc in &obj.relocations {
                // Lookup symbol address
                let symbol = global.symbols.get(&reloc.symbol)
                    .ok_or_else(|| format!("Symbol '{}' not found", reloc.symbol))?;
                
                // Find target section
                let section = obj.sections.iter_mut()
                    .find(|s| s.name == reloc.section)
                    .ok_or_else(|| format!("Section '{}' not found", reloc.section))?;
                
                // Calculate address to patch
                let target_address = symbol.address;
                
                // Apply relocation based on type
                match reloc.reloc_type {
                    RelocationType::Absolute16 => {
                        // Patch 2 bytes with absolute address (big-endian)
                        section.data[reloc.offset] = (target_address >> 8) as u8;
                        section.data[reloc.offset + 1] = (target_address & 0xFF) as u8;
                    }
                    RelocationType::Relative8 => {
                        // Calculate relative offset
                        let base = section.base_address.unwrap() + reloc.offset as u16 + 2;
                        let offset = (target_address as i32 - base as i32) as i8;
                        section.data[reloc.offset] = offset as u8;
                    }
                    RelocationType::Relative16 => {
                        // Calculate relative offset (16-bit)
                        let base = section.base_address.unwrap() + reloc.offset as u16 + 4;
                        let offset = (target_address as i32 - base as i32) as i16;
                        section.data[reloc.offset] = (offset >> 8) as u8;
                        section.data[reloc.offset + 1] = (offset & 0xFF) as u8;
                    }
                }
            }
        }
        
        Ok(())
    }
}
```

### Example

Given relocation:
```
Relocation {
    section: ".text.main",
    offset: 51,  // After "JSR" opcode at call_helper
    symbol: "helper_function",
    reloc_type: Absolute16,
}
```

And resolved symbol:
```
helper_function: address=0xC8E4
```

Patching:
```
Before: section.data[51..53] = [0x00, 0x00]  // Placeholder
After:  section.data[51..53] = [0xC8, 0xE4]  // Real address
```

## Phase 4.5: Multi-Object Linking Test

### Test Setup

Create two VPy programs:
```python
# lib.vpy
def helper_function():
    SET_INTENSITY(127)
```

```python
# main.vpy
import helper_function from lib

def main():
    helper_function()
    
def loop():
    WAIT_RECAL()
```

### Test Steps

1. Compile both to .vo:
   ```bash
   vectrexc compile-object lib.vpy -o lib.vo
   vectrexc compile-object main.vpy -o main.vo
   ```

2. Link:
   ```bash
   vecld link main.vo lib.vo -o program.bin
   ```

3. Verify:
   - All symbols resolved
   - Relocations applied correctly
   - Binary runs correctly

## Implementation Order

1. ✅ Phase 4.1: Assembler object mode (COMPLETE - 2025-12-30)
   - Modify `assemble_m6809()` signature
   - Add `UnresolvedRef` struct
   - Emit placeholders for undefined symbols
   - Test with unresolved symbols
   - Commit: d8f50bbc

2. ✅ Phase 4.2: SymbolResolver (COMPLETE - 2025-01-02)
   - Implement `collect_symbols()`
   - Implement `verify_imports()`
   - Implement `assign_addresses()`
   - Test duplicate detection
   - Test undefined imports
   - 5/5 tests passing
   - Commit: 1e0de558

3. ✅ Phase 4.3: Address assignment (COMPLETE - 2025-01-02)
   - Implemented in Phase 4.2 as assign_addresses()
   - Assigns base addresses to sections
   - Updates symbol addresses in global table
   - Commit: 1e0de558

4. ✅ Phase 4.4: Relocation patching (COMPLETE - 2025-01-02)
   - Implement apply_relocations()
   - Support Absolute16, Relative8, Relative16
   - Support Direct, High8, Low8
   - Test JSR patching
   - 6/6 tests passing
   - Commit: 8482a7ae

5. ✅ Phase 4.5: Multi-object linking test (COMPLETE - 2025-01-10)
   - Implemented `link_cmd()` with full pipeline
   - Load multiple .vo files
   - Resolve symbols globally
   - Assign addresses to sections
   - Apply relocations
   - Merge sections into final binary
   - Commit: 51bd3b16
   - Note: Full end-to-end test pending module system (VPy currently requires main/loop in each file)

6. ⏸️ Phase 5: ROM Writing (future work)
   - Test undefined reference detection

3. ⏸️ Phase 4.3: Address assignment (2 hours)
   - Implement `assign_addresses()`
   - Test with multiple sections
   - Test with alignment requirements

4. ⏸️ Phase 4.4: Relocation patching (3 hours)
   - Implement `apply_relocations()`
   - Test Absolute16 patching
   - Test Relative8/16 patching
   - Test complex multi-object scenarios

5. ⏸️ Phase 4.5: Integration testing (2 hours)
   - Create multi-file VPy test programs
   - Verify end-to-end linking
   - Compare with monolithic compilation

**Total estimated time: 12 hours**

## Success Criteria

- [x] Assembler supports object mode
- [ ] SymbolResolver detects duplicate symbols
- [ ] SymbolResolver detects undefined references
- [ ] Address assignment handles multiple sections
- [ ] Relocations patch correctly (all types)
- [ ] Multi-object programs link and run successfully
- [ ] Tests pass for all sub-phases

## Files to Create/Modify

**New**:
- `LINKER_PHASE4_DESIGN.md` (this file)

**Modified**:
- `core/src/backend/asm_to_binary.rs` - Add object mode
- `core/src/linker/resolver.rs` - Implement SymbolResolver
- `core/src/linker/asm_parser.rs` - Use object mode when extracting sections
- `core/src/main.rs` - Add `link` command for vecld

**Testing**:
- Add tests to `asm_to_binary.rs` for object mode
- Add tests to `linker/resolver.rs` for symbol resolution
- Create multi-file VPy programs for integration testing

---

**Status**: Phase 4 design complete, ready for implementation
**Next Step**: Implement Phase 4.1 (Assembler object mode)
