# VPy Linker - Complete Design Document

## Vision
Professional linking system for VPy, enabling libraries, modular compilation, and multi-bank ROM generation similar to cc65/RGBDS.

## Architecture Overview

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  game.vpy   │     │  player.vpy │     │  veclib.vpy │
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       │                   │                    │
       │ vectrexc          │ vectrexc          │ vectrexc
       ▼                   ▼                    ▼
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  game.vo    │     │  player.vo  │     │  veclib.vo  │
│ (object)    │     │ (object)    │     │ (library)   │
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       │                   │                    │
       └───────────┬───────┴────────────────────┘
                   │
                   │ vecld (linker)
                   ▼
           ┌───────────────┐
           │   game.rom    │
           │ (512KB final) │
           └───────────────┘
```

## 1. Object File Format (.vo)

### 1.1 File Structure
```rust
// .vo file structure (binary format)
struct VectrexObject {
    header: ObjectHeader,
    sections: Vec<Section>,
    symbols: SymbolTable,
    relocations: Vec<Relocation>,
    debug_info: DebugInfo,
}

struct ObjectHeader {
    magic: [u8; 4],           // "VObj" signature
    version: u16,             // Format version
    target: TargetArch,       // M6809, etc.
    flags: ObjectFlags,       // Position-independent, etc.
}

struct Section {
    name: String,             // ".text.main", ".data.player_x", ".rodata.STR_0"
    type: SectionType,        // Code, Data, BSS, ReadOnly
    bank_hint: Option<u8>,    // Preferred bank (None = linker decides)
    alignment: u16,           // Required alignment (1, 2, 256, etc.)
    data: Vec<u8>,            // Raw bytes (empty for BSS)
}

enum SectionType {
    Text,       // Code (.text)
    Data,       // Initialized data (.data)
    Bss,        // Uninitialized data (.bss)
    ReadOnly,   // Constants (.rodata)
}

struct SymbolTable {
    exports: Vec<Symbol>,     // Symbols defined in this object
    imports: Vec<Symbol>,     // External symbols needed
}

struct Symbol {
    name: String,             // "LEVEL1_INIT", "player_x", "VECTREX_SET_INTENSITY"
    section: Option<usize>,   // Section index (None for extern)
    offset: u16,              // Offset within section
    scope: SymbolScope,       // Local, Global, Weak
    type: SymbolType,         // Function, Variable, Constant
}

enum SymbolScope {
    Local,      // Visible only in this file
    Global,     // Exported to other files
    Weak,       // Can be overridden
}

struct Relocation {
    section: usize,           // Section containing reference
    offset: u16,              // Offset of reference
    type: RelocationType,     // How to patch
    symbol: String,           // Referenced symbol
    addend: i32,              // Additional offset
}

enum RelocationType {
    Absolute16,    // Full 16-bit address
    Relative8,     // PC-relative branch (BRA, BEQ)
    Relative16,    // Long branch (LBRA, LBEQ)
    Direct,        // Direct page (8-bit)
    High8,         // High byte of address
    Low8,          // Low byte of address
}
```

### 1.2 Example Object File
```
game.vo:
  Sections:
    .text.main (Bank #31 hint):
      0x0000: JSR Wait_Recal
      0x0003: JSR [VECTREX_SET_INTENSITY]  ; Reloc: Absolute16
      0x0006: JSR [level1_init]            ; Reloc: CrossBank
      0x0009: RTS
    
    .text.level1_init (Bank #0 hint):
      0x0000: LDD #100
      0x0003: JSR [VECTREX_SET_INTENSITY]  ; Reloc: Absolute16
      0x0006: RTS
  
  Symbols:
    Exports:
      - main (Global, .text.main+0x0000)
      - level1_init (Global, .text.level1_init+0x0000)
    Imports:
      - VECTREX_SET_INTENSITY (Extern)
  
  Relocations:
    - .text.main+0x0004: Absolute16 → VECTREX_SET_INTENSITY
    - .text.main+0x0007: Absolute16 → level1_init
    - .text.level1_init+0x0004: Absolute16 → VECTREX_SET_INTENSITY
```

## 2. Linker Script (.ld)

### 2.1 Script Format
```
MEMORY {
    # Define memory regions
    BANK0:   START = 0x0000, SIZE = 16K, TYPE = BANKED;
    BANK1:   START = 0x0000, SIZE = 16K, TYPE = BANKED;
    ...
    BANK30:  START = 0x0000, SIZE = 16K, TYPE = BANKED;
    BANK31:  START = 0x4000, SIZE = 8K,  TYPE = FIXED;
    RAM:     START = 0xC880, SIZE = 896,  TYPE = RAM;
}

SECTIONS {
    # Fixed bank (always visible)
    .text.fixed BANK31 : {
        *(.text.main)           # main() always in fixed bank
        *(.text.loop)           # loop() always in fixed bank
        *(.text.helpers)        # VECTREX_* helpers
        *(.text.wrappers)       # Cross-bank wrappers
    }
    
    # Switchable banks (automatic assignment)
    .text.bank0 BANK0 : {
        *(.text.level1*)        # Level 1 functions
    }
    
    .text.bank1 BANK1 : {
        *(.text.level2*)        # Level 2 functions
    }
    
    # Read-only data (constants, strings)
    .rodata BANK31 : {
        *(.rodata*)
    }
    
    # Variables (RAM)
    .bss RAM : {
        *(.bss*)
    }
}

# Entry point
ENTRY(main)

# Bank switching behavior
BANK_REGISTER = 0x4000;

# Symbol definitions
VECTREX_SET_INTENSITY = 0x4100;  # In fixed bank
VECTREX_PRINT_TEXT = 0x4200;
```

### 2.2 Default Linker Script
If no .ld provided, use builtin defaults:
- main/loop → Bank #31
- All other functions → Best-fit decreasing (current algorithm)
- Helpers/wrappers → Bank #31

## 3. Compiler Changes (vectrexc)

### 3.1 New Compilation Modes
```bash
# Current: Direct to binary (single-file)
vectrexc build game.vpy --bin          # → game.bin (512KB)

# New: Compile to object (modular)
vectrexc compile game.vpy -o game.vo   # → game.vo (object)
vectrexc compile player.vpy -o player.vo
vectrexc compile veclib.vpy -o veclib.vo

# Link objects to ROM
vecld game.vo player.vo veclib.vo -o game.rom --script=game.ld
```

### 3.2 Object Generation Process
```rust
// In main.rs - new compilation mode
fn compile_to_object(source: &Path, output: &Path) -> Result<()> {
    // Phase 1-3: Parse (same as before)
    let module = parse_vpy(source)?;
    
    // Phase 4: Generate ASM with section markers
    let asm = emit_asm_with_sections(&module)?;
    
    // Phase 5: Parse ASM into sections
    let sections = extract_sections(&asm)?;
    
    // Phase 6: Build symbol table (exports/imports)
    let symbols = build_symbol_table(&sections)?;
    
    // Phase 7: Collect relocations
    let relocations = collect_relocations(&sections, &symbols)?;
    
    // Phase 8: Write .vo file
    write_object_file(output, sections, symbols, relocations)?;
    
    Ok(())
}
```

### 3.3 Section Markers in ASM
```asm
; Current ASM (monolithic)
MAIN:
    JSR Wait_Recal
    JSR VECTREX_SET_INTENSITY
    RTS

LEVEL1_INIT:
    LDD #100
    RTS

; New ASM (with sections)
.section .text.main, "ax", @progbits   ; Executable code
MAIN:
    JSR Wait_Recal
    JSR VECTREX_SET_INTENSITY          ; Mark as external reference
    RTS

.section .text.level1_init, "ax", @progbits
LEVEL1_INIT:
    LDD #100
    RTS

.section .rodata, "a", @progbits       ; Read-only data
STR_0:
    FCC "HELLO"
    FCB $80
```

## 4. Linker Implementation (vecld)

### 4.1 Command-Line Interface
```bash
vecld [OPTIONS] <input.vo>... -o <output.rom>

Options:
  -o, --output <FILE>        Output ROM file
  -T, --script <FILE>        Linker script (default: builtin)
  -L, --library-path <DIR>   Search path for libraries
  -l, --library <NAME>       Link with library (e.g., -lveclib → veclib.vo)
  --map <FILE>               Generate memory map
  --symbols <FILE>           Generate symbol file (.sym)
  --banks <N>                Number of banks (default: 32)
  --bank-size <SIZE>         Bank size (default: 16384)
  --rom-size <SIZE>          Total ROM size (default: 524288)
  -v, --verbose              Verbose output
```

### 4.2 Linker Algorithm
```rust
struct Linker {
    objects: Vec<VectrexObject>,
    script: LinkerScript,
    symbol_table: GlobalSymbolTable,
    memory_layout: MemoryLayout,
}

impl Linker {
    fn link(&mut self) -> Result<Vec<u8>> {
        // Step 1: Load all objects and linker script
        self.load_objects()?;
        self.load_script()?;
        
        // Step 2: Build global symbol table
        self.build_global_symbols()?;
        
        // Step 3: Assign sections to memory regions
        self.assign_sections_to_banks()?;
        
        // Step 4: Calculate final addresses
        self.calculate_addresses()?;
        
        // Step 5: Resolve relocations (patch all references)
        self.resolve_relocations()?;
        
        // Step 6: Generate cross-bank wrappers
        self.generate_bank_wrappers()?;
        
        // Step 7: Write final ROM
        let rom = self.write_rom()?;
        
        // Step 8: Generate debug files
        self.write_map_file()?;
        self.write_symbol_file()?;
        
        Ok(rom)
    }
    
    fn build_global_symbols(&mut self) -> Result<()> {
        // Collect all exported symbols from all objects
        for obj in &self.objects {
            for symbol in &obj.symbols.exports {
                if self.symbol_table.contains(&symbol.name) {
                    return Err(format!("Duplicate symbol: {}", symbol.name));
                }
                self.symbol_table.insert(symbol.clone());
            }
        }
        
        // Check all imports can be resolved
        for obj in &self.objects {
            for symbol in &obj.symbols.imports {
                if !self.symbol_table.contains(&symbol.name) {
                    return Err(format!("Undefined symbol: {}", symbol.name));
                }
            }
        }
        
        Ok(())
    }
    
    fn assign_sections_to_banks(&mut self) -> Result<()> {
        // Follow linker script rules
        for rule in &self.script.section_rules {
            let sections = self.find_matching_sections(&rule.pattern);
            let bank = self.memory_layout.get_region(&rule.region);
            
            for section in sections {
                bank.add_section(section)?;
            }
        }
        
        // Assign remaining sections (best-fit decreasing)
        let unassigned = self.find_unassigned_sections();
        self.best_fit_decreasing(unassigned)?;
        
        Ok(())
    }
    
    fn resolve_relocations(&mut self) -> Result<()> {
        for obj in &mut self.objects {
            for reloc in &obj.relocations {
                let target_symbol = self.symbol_table.get(&reloc.symbol)?;
                let target_addr = target_symbol.final_address;
                
                // Check if cross-bank reference
                let source_bank = self.get_section_bank(reloc.section);
                let target_bank = self.get_symbol_bank(&target_symbol);
                
                if source_bank != target_bank && target_bank != 31 {
                    // Cross-bank call - use wrapper
                    let wrapper_addr = self.get_or_create_wrapper(&reloc.symbol, target_bank)?;
                    self.patch_relocation(reloc, wrapper_addr)?;
                } else {
                    // Same bank or fixed bank - direct call
                    self.patch_relocation(reloc, target_addr)?;
                }
            }
        }
        
        Ok(())
    }
}
```

### 4.3 Memory Map Output
```
Memory Configuration:
  BANK0   : origin = 0x0000, length = 16K (BANKED)
  BANK31  : origin = 0x4000, length = 8K  (FIXED)
  RAM     : origin = 0xC880, length = 896

Section Assignments:

.text.main (BANK31)
  0x4000 - 0x4050 (80 bytes)
    0x4000  MAIN
    0x4030  LOOP_BODY

.text.level1 (BANK0)
  0x0000 - 0x0200 (512 bytes)
    0x0000  LEVEL1_INIT
    0x0050  LEVEL1_UPDATE
    0x0100  LEVEL1_RENDER

Symbol Table:
  MAIN                0x4000  (BANK31) Global
  LOOP_BODY           0x4030  (BANK31) Global
  LEVEL1_INIT         0x0000  (BANK0)  Global
  VECTREX_SET_INTENSITY 0x4100 (BANK31) Global

Cross-Bank Wrappers:
  level1_init_WRAPPER 0x4200  (BANK31) → LEVEL1_INIT (BANK0)

Bank Usage:
  BANK0:  512 / 16384 bytes (3.1%)
  BANK31: 1024 / 8192 bytes (12.5%)
  ROM:    512KB (32 banks)
```

## 5. Library Support

### 5.1 Creating Libraries
```bash
# Compile library sources to objects
vectrexc compile veclib/print.vpy -o veclib/print.vo
vectrexc compile veclib/math.vpy -o veclib/math.vo
vectrexc compile veclib/sound.vpy -o veclib/sound.vo

# Package into library archive (optional)
vecar rcs veclib.vla veclib/*.vo
```

### 5.2 Using Libraries
```bash
# Link with library objects
vecld game.vo player.vo veclib/print.vo veclib/math.vo -o game.rom

# Or with library archive
vecld game.vo player.vo -L./veclib -lveclib -o game.rom
```

### 5.3 Standard Library (stdlib.vla)
Pre-compiled standard library included with VPy:
- `veclib/io.vo` - PRINT_TEXT, PRINT_NUMBER, etc.
- `veclib/graphics.vo` - DRAW_LINE, DRAW_VECTOR, etc.
- `veclib/math.vo` - MUL16, DIV16, SQRT, etc.
- `veclib/sound.vo` - PLAY_MUSIC, PLAY_SFX, etc.

## 6. Implementation Roadmap

### Phase 1: Object File Format (Week 1) ✅ COMPLETE
- [x] Define .vo binary format (Rust structs)
- [x] Implement serialization/deserialization (bincode)
- [x] Write tests for round-trip
- [x] Test: object creation, symbol lookup, relocations
- **Status**: 2 tests passing, ready for Phase 2

### Phase 2: Compiler Changes (Week 2) 🔄 IN PROGRESS
- [ ] Add section markers to ASM emission
- [ ] Extract sections from ASM
- [ ] Build symbol table (exports/imports)
- [ ] Collect relocations
- [ ] Write .vo files

### Phase 3: Linker Core (Week 3-4)
- [ ] Parse linker script (.ld)
- [ ] Load .vo files
- [ ] Build global symbol table
- [ ] Assign sections to banks
- [ ] Calculate final addresses
- [ ] Resolve relocations
- [ ] Generate cross-bank wrappers
- [ ] Write ROM output

### Phase 4: Integration (Week 5)
- [ ] Update IDE build system
- [ ] Add library search paths
- [ ] Update error messages
- [ ] Generate memory map
- [ ] Generate symbol file

### Phase 5: Standard Library (Week 6)
- [ ] Split builtins into library modules
- [ ] Pre-compile standard library
- [ ] Update documentation

## 7. Benefits

### For Users:
- ✅ Compile only changed files (faster iteration)
- ✅ Reusable libraries across projects
- ✅ Better memory control with linker scripts
- ✅ Cleaner separation of code modules

### For IDE:
- ✅ Incremental compilation
- ✅ Library browser/manager
- ✅ Better IntelliSense (from object metadata)
- ✅ Visual memory map viewer

### For Debugging:
- ✅ Precise symbol information per module
- ✅ Source line mapping preserved
- ✅ Memory map for optimization

## 8. Compatibility

### Backward Compatibility:
```bash
# Old workflow (still works)
vectrexc build game.vpy --bin    # Single-file, direct to .bin

# New workflow (opt-in)
vectrexc compile game.vpy -o game.vo
vecld game.vo -o game.rom
```

Both produce identical ROMs for single-file projects.

### Migration Path:
1. Phase 1-5: Implement linker (vectrexc build still works)
2. Update IDE to use linker internally
3. Expose library features to users
4. Eventually deprecate direct .bin generation

## 9. Future Extensions

Once linker exists:
- **Dynamic linking** (load banks at runtime)
- **Overlays** (swap code sections on demand)
- **Position-independent code** (relocatable libraries)
- **Link-time optimization** (dead code elimination, inlining)
- **Compressed banks** (unpack at runtime)

---

**Estimated Total Effort:** 6 weeks (1 developer)
**Current Progress:** Architecture designed ✓
**Next Step:** Implement object file format (Phase 1)
