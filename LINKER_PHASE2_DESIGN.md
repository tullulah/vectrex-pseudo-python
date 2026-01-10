# Linker Phase 2: Section Emission Implementation

## Status: IN PROGRESS (2026-01-08)

## Goal
Modify M6809 backend to emit section markers when `CodegenOptions.emit_sections = true`. This enables the compiler to generate `.vo` object files that the linker can process.

## Section Markers (GAS-style syntax)

```asm
.section NAME, "FLAGS", @TYPE
```

Where:
- **NAME**: Section name (e.g., `.text.main`, `.rodata`, `.bss`)
- **FLAGS**: Section attributes:
  - `a` = allocatable (loaded into memory)
  - `w` = writable (RAM)
  - `x` = executable (code)
- **TYPE**: Section type:
  - `@progbits` = initialized data (code or constants in ROM)
  - `@nobits` = uninitialized data (BSS - no data in file)

## Section Types

| Section Name | Type | Flags | Bank Hint | Contents |
|--------------|------|-------|-----------|----------|
| `.text.main` | progbits | "ax" | 31 | main() function code |
| `.text.loop` | progbits | "ax" | 31 | loop() function code (runs every frame) |
| `.text.FUNCNAME` | progbits | "ax" | auto | Other function code |
| `.text.fixed` | progbits | "ax" | 31 | Helper functions, builtins, wrappers |
| `.rodata` | progbits | "a" | 31 | String literals, const arrays |
| `.rodata.STR_N` | progbits | "a" | 31 | Individual string literals |
| `.rodata.CONST_ARRAY_N` | progbits | "a" | 31 | Const array data |
| `.bss` | nobits | "aw" | RAM | Uninitialized variables |
| `.bss.VAR_NAME` | nobits | "aw" | RAM | Individual variables (for fine-grained allocation) |

## Implementation Plan

### Step 1: Section Emission in `emit_with_debug()` ✅ COMPLETE
**Location**: `core/src/backend/m6809/mod.rs` lines 252-1400

**Changes needed**:
1. Before emitting header: Check `if opts.emit_sections`
2. If true: Emit section markers before each logical group
3. If false: Keep current monolithic ASM (backward compatibility)

**Example output** (with `emit_sections: true`):
```asm
; --- Motorola 6809 backend (vectrex) title='Game' origin=$0000 ---
        ORG $0000

.section .text.header, "ax", @progbits
;***************************************************************************
; HEADER SECTION
;***************************************************************************
        FCC "g GCE 1982"
        FCB $80
        FDB music1
        ; ... header data ...

.section .rodata, "a", @progbits
;***************************************************************************
; STRING LITERALS
;***************************************************************************
STR_0:
        FCC "HELLO"
        FCB $80

.section .text.main, "ax", @progbits
;***************************************************************************
; MAIN FUNCTION
;***************************************************************************
MAIN:
        JSR Wait_Recal
        JSR VECTREX_SET_INTENSITY
        RTS

.section .text.loop, "ax", @progbits
;***************************************************************************
; LOOP FUNCTION (60 FPS)
;***************************************************************************
LOOP_BODY:
        JSR Wait_Recal
        JSR MAIN
        ; ... loop code ...
        RTS

.section .text.fixed, "ax", @progbits
;***************************************************************************
; HELPER FUNCTIONS & BUILTINS
;***************************************************************************
VECTREX_SET_INTENSITY:
        LDA VAR_ARG0+1
        JSR Intensity_a
        RTS
        
; ... more helpers ...

.section .bss, "aw", @nobits
;***************************************************************************
; VARIABLES (RAM)
;***************************************************************************
VAR_PLAYER_X EQU $CF10+0
VAR_PLAYER_Y EQU $CF10+2
; ... (no data emitted, just EQU definitions)
```

### Step 2: ASM Parser ⏸️ PENDING
**Location**: `core/src/linker/asm_parser.rs` (NEW)

**Purpose**: Parse emitted ASM to extract sections

**Functions needed**:
```rust
pub fn parse_asm_sections(asm: &str) -> Result<Vec<Section>> {
    // 1. Split by `.section` directives
    // 2. For each section:
    //    - Parse section name, flags, type
    //    - Collect ASM lines until next `.section`
    //    - Assemble section to binary (using asm_to_binary.rs)
    //    - Create Section struct
    // 3. Return Vec<Section>
}
```

**Example**:
```rust
let sections = parse_asm_sections(&asm)?;
// sections[0]: Section { name: ".text.main", type: Text, data: [0xBD, 0xF3, 0x73, ...] }
// sections[1]: Section { name: ".rodata", type: ReadOnly, data: [0x48, 0x45, 0x4C, ...] }
```

### Step 3: Symbol Table Builder ⏸️ PENDING
**Location**: `core/src/linker/symbol_builder.rs` (NEW)

**Purpose**: Extract symbols from parsed sections

**Functions needed**:
```rust
pub fn build_symbol_table(sections: &[Section]) -> Result<SymbolTable> {
    let mut exports = Vec::new();
    let mut imports = Vec::new();
    
    for (section_idx, section) in sections.iter().enumerate() {
        // Scan ASM for labels (ends with ':')
        // Example: MAIN: → Export(name="MAIN", section=0, offset=0, scope=Global)
        
        // Scan for JSR/LDD # targets
        // Example: JSR helper → Import(name="helper", External)
        // If helper is defined in another section → mark as import
    }
    
    SymbolTable { exports, imports }
}
```

### Step 4: Relocation Collector ⏸️ PENDING
**Location**: `core/src/linker/relocation_collector.rs` (NEW)

**Purpose**: Collect relocations (addresses needing patching)

**Functions needed**:
```rust
pub fn collect_relocations(
    sections: &[Section], 
    symbols: &SymbolTable
) -> Result<Vec<Relocation>> {
    let mut relocations = Vec::new();
    
    for (section_idx, section) in sections.iter().enumerate() {
        // Scan for relocatable instructions:
        // - JSR addr (Absolute16)
        // - LDD #addr (Absolute16)
        // - BRA offset (Relative8)
        // - LBRA offset (Relative16)
        
        // Example: JSR HELPER at offset 0x0003
        // → Relocation { 
        //     section: 0, 
        //     offset: 0x0003,
        //     reloc_type: Absolute16,
        //     symbol: "HELPER"
        //   }
    }
    
    relocations
}
```

### Step 5: Object File Writer ⏸️ PENDING
**Location**: `core/src/linker/object_writer.rs` (NEW) or integrate into `object.rs`

**Purpose**: Write complete `.vo` file

**Integration with main.rs**:
```rust
// In main.rs compile command:
if opts.emit_sections {
    // Generate ASM with section markers
    let (asm, debug_info, diagnostics) = codegen::emit_asm_with_debug(&module, target, &opts);
    
    // Parse sections
    let sections = linker::asm_parser::parse_asm_sections(&asm)?;
    
    // Build symbol table
    let symbols = linker::symbol_builder::build_symbol_table(&sections)?;
    
    // Collect relocations
    let relocations = linker::relocation_collector::collect_relocations(&sections, &symbols)?;
    
    // Create object
    let mut obj = VectrexObject::new(source_path.to_string());
    obj.sections = sections;
    obj.symbols = symbols;
    obj.relocations = relocations;
    obj.debug_info = debug_info; // Preserve debug info for .pdb
    
    // Write .vo file
    let vo_path = output_path.with_extension("vo");
    let mut file = File::create(&vo_path)?;
    obj.write(&mut file)?;
    
    println!("✓ Object file written: {}", vo_path.display());
} else {
    // Monolithic ASM mode (current behavior)
    // ...continue with asm_to_binary...
}
```

## Debug Info Preservation

**Critical**: The `.pdb` file must continue working!

**Current flow**:
```
.vpy → codegen → .asm → asm_to_binary → .bin + .pdb
```

**New flow** (with linker):
```
.vpy → codegen (emit_sections=true) → .vo (with debug_info)
.vo + .vo → vecld (linker) → .rom + .pdb (updated addresses)
```

**DebugInfo in .vo**:
```rust
pub struct DebugInfo {
    pub line_map: HashMap<u16, usize>,  // address → source line (RELATIVE to section)
    pub source_lines: Vec<String>,       // Original source code
}
```

**Linker updates addresses**:
1. After section placement, linker knows final addresses
2. Walk through `line_map` and add section base address to each entry
3. Generate final `.pdb` with absolute addresses

**Example**:
```rust
// In .vo:
line_map: { 0x0000 => 5, 0x0003 => 6, 0x0010 => 10 }

// After linking (section placed at 0x4000):
line_map: { 0x4000 => 5, 0x4003 => 6, 0x4010 => 10 }
```

## Testing Plan

### Test 1: Simple compilation (hello.vpy)
```bash
cargo run --bin vectrexc -- compile examples/hello/hello.vpy --emit-sections -o hello.vo
```
**Expected**:
- Generates `hello.vo` (binary object file)
- Contains sections: `.text.main`, `.text.loop`, `.rodata`, `.bss`
- Symbol table has exports: MAIN, LOOP_BODY
- No imports (self-contained)

### Test 2: Multi-file compilation (game.vpy + player.vpy)
```bash
cargo run --bin vectrexc -- compile game.vpy --emit-sections -o game.vo
cargo run --bin vectrexc -- compile player.vpy --emit-sections -o player.vo
cargo run --bin vecld -- game.vo player.vo -o game.rom --map game.map -v
```
**Expected**:
- `game.vo` has imports: `player_init`, `player_update`
- `player.vo` has exports: `player_init`, `player_update`
- Linker resolves imports → final ROM works
- Debug symbols preserved in `game.pdb`

### Test 3: Backward compatibility (no --emit-sections)
```bash
cargo run --bin vectrexc -- build examples/hello/hello.vpy
```
**Expected**:
- Generates monolithic ASM (no section markers)
- Assembles to `.bin` directly (current behavior)
- `.pdb` generated with correct line mappings
- No breaking changes

## Next Steps (Immediate)

1. ✅ **DONE**: Initialize `emit_sections: false` in all CodegenOptions locations
2. ⏸️ **TODO**: Add CLI flag `--emit-sections` to main.rs
3. ⏸️ **TODO**: Modify `emit_with_debug()` to emit section markers when enabled
4. ⏸️ **TODO**: Test with simple example (verify section markers appear)
5. ⏸️ **TODO**: Implement ASM parser (Step 2)
6. ⏸️ **TODO**: Implement symbol builder (Step 3)
7. ⏸️ **TODO**: Implement relocation collector (Step 4)
8. ⏸️ **TODO**: Write .vo files (Step 5)
9. ⏸️ **TODO**: Test end-to-end (compile → link → run)

## Notes

- Section emission is **opt-in** via `emit_sections: true` (default false)
- Backward compatibility maintained (monolithic ASM when `emit_sections: false`)
- Debug info flows through `.vo` to final `.pdb` (addresses updated by linker)
- Bank hints embedded in section names (e.g., `.text.level1` → Bank #0)
- Cross-bank wrappers automatically generated by linker (Phase 3)

---
Last updated: 2026-01-08 - Phase 2 design complete, Step 1 in progress
