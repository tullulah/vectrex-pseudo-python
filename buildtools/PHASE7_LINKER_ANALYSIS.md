# Phase 7: vpy_linker - Analysis & Port Plan

## Status: Ready to Port from core/

Date: 2026-01-17

---

## Core Linker Architecture Analysis

### Existing Implementation in core/src/linker/

The core already has a **professional-grade linker implementation** with 7 modules:

#### 1. **object.rs** (310 lines) - Object File Format
**Purpose**: .vo (Vectrex Object) file format definition

**Key Types**:
```rust
pub struct VectrexObject {
    header: ObjectHeader,
    sections: Vec<Section>,
    symbols: SymbolTable,
    relocations: Vec<Relocation>,
    debug_info: DebugInfo,
}

pub struct Section {
    name: String,             // ".text.main", ".data.player_x"
    section_type: SectionType, // Text, Data, Bss, ReadOnly
    bank_hint: Option<u8>,    // Preferred bank
    alignment: u16,
    data: Vec<u8>,
}

pub struct Symbol {
    name: String,
    section: Option<usize>,
    offset: u16,
    scope: SymbolScope,       // Local, Global, Weak
    symbol_type: SymbolType,  // Function, Variable, Constant
}

pub struct Relocation {
    section: usize,
    offset: u16,
    reloc_type: RelocationType,  // Absolute16, Relative8, CrossBank, etc.
    symbol: String,
    addend: i32,
}

pub enum RelocationType {
    Absolute16,    // JSR, LDX #addr
    Relative8,     // BRA ±127
    Relative16,    // LBRA ±32K
    Direct,        // Direct page
    High8, Low8,   // Byte access
    CrossBank,     // Needs wrapper
}
```

**Features**:
- ✅ Binary serialization (bincode)
- ✅ Magic number validation ("VObj")
- ✅ Version checking
- ✅ Save/load from disk
- ✅ Debug info preservation

#### 2. **script.rs** (103 lines) - Linker Scripts
**Purpose**: Memory layout configuration (.ld files)

**Key Types**:
```rust
pub struct LinkerScript {
    memory_regions: Vec<MemoryRegion>,
    section_rules: Vec<SectionRule>,
    entry_point: Option<String>,
    bank_register: Option<u16>,
}

pub struct MemoryRegion {
    name: String,
    start: u16,
    size: usize,
    region_type: RegionType,  // Banked, Fixed, Ram
}

pub struct SectionRule {
    pattern: String,  // Glob pattern ".text.level*"
    region: String,   // Target region "BANK15"
}
```

**Default Vectrex Script**:
- 31 switchable banks (0-30): $0000-$3FFF (16KB each)
- 1 fixed bank (#31): $4000-$5FFF (8KB)
- RAM: $C880-$CBFF (896 bytes)
- Section rules: main/loop in BANK31, wrappers in BANK31, data in RAM

#### 3. **resolver.rs** (479 lines) - Symbol Resolution
**Purpose**: Build global symbol table and resolve references

**Key Functions**:
```rust
impl SymbolResolver {
    // Collect all exports from .vo files
    pub fn collect_symbols(objects: &[VectrexObject]) 
        -> Result<GlobalSymbolTable, String>;
    
    // Verify all imports have exports
    pub fn verify_imports(objects: &[VectrexObject], global: &GlobalSymbolTable) 
        -> Result<(), String>;
    
    // Assign addresses to sections and symbols
    pub fn assign_addresses(
        objects: &[VectrexObject],
        global: &mut GlobalSymbolTable,
        base_address: u16,
    ) -> Result<HashMap<(usize, usize), u16>, String>;
    
    // Apply relocations (patch addresses in binary)
    pub fn apply_relocations(
        objects: &mut [VectrexObject],
        global: &GlobalSymbolTable,
        section_bases: &HashMap<(usize, usize), u16>,
    ) -> Result<(), String>;
}
```

**Algorithm**:
1. Collect all exported symbols into global table
2. Check for duplicate definitions
3. Verify all imports have matching exports
4. Assign base addresses to sections
5. Update symbol addresses
6. Apply relocations (patch binary with real addresses)

#### 4. **bank_allocator.rs** (Unknown size)
**Purpose**: Assign sections to banks
- NOTE: May overlap with buildtools/vpy_bank_allocator (Phase 4)

#### 5. **rom_writer.rs** (Unknown size)
**Purpose**: Write final ROM binary
- Assembles banks into final .bin file
- Handles bank switching logic

#### 6. **asm_parser.rs** (Unknown size)
**Purpose**: Extract sections from ASM
- Parse assembly to extract sections
- Build symbol table from ASM
- Collect relocations

#### 7. **mod.rs** - Public API
**Exports**:
- All types from object.rs
- LinkerScript
- SymbolResolver, GlobalSymbolTable
- BankAllocator, RomWriter
- ASM parsing utilities

---

## Port Strategy: Core → Buildtools

### Phase 7.1: Port Core Types (1 day)
**Goal**: Copy object format and types

**Steps**:
1. Create `buildtools/vpy_linker/` structure
2. Port `object.rs` → `buildtools/vpy_linker/src/object.rs`
   - VectrexObject, Section, Symbol, Relocation
   - Serialization/deserialization
   - File I/O
3. Port `script.rs` → `buildtools/vpy_linker/src/script.rs`
   - LinkerScript, MemoryRegion, SectionRule
   - Default Vectrex configuration
4. Add tests for object file I/O

**Result**: Type definitions and serialization working

### Phase 7.2: Port Symbol Resolution (1 day)
**Goal**: Symbol table and resolution logic

**Steps**:
1. Port `resolver.rs` → `buildtools/vpy_linker/src/resolver.rs`
   - GlobalSymbolTable, ResolvedSymbol
   - SymbolResolver implementation
2. Add tests:
   - Collect symbols from multiple .vo files
   - Detect duplicate definitions
   - Verify imports
   - Assign addresses

**Result**: Symbol resolution working with tests

### Phase 7.3: Integrate Bank Allocator (0.5 days)
**Goal**: Use Phase 4 allocator instead of core's

**Decision Point**:
- **Option A**: Use buildtools/vpy_bank_allocator (Phase 4)
- **Option B**: Port core's bank_allocator.rs
- **Recommendation**: Option A (already tested, 12 tests passing)

**Integration**:
```rust
// Use Phase 4 output
use vpy_bank_allocator::BankLayout;

impl Linker {
    pub fn link(
        objects: Vec<VectrexObject>,
        layout: BankLayout,  // From Phase 4
        script: LinkerScript,
    ) -> Result<LinkedBinary, LinkError> {
        // Use layout.banks to assign sections
    }
}
```

### Phase 7.4: Relocation Engine (1 day)
**Goal**: Apply address patches

**Steps**:
1. Port relocation logic from `resolver.rs`
2. Implement each RelocationType:
   - `Absolute16`: Full 16-bit address (JSR, LDX)
   - `Relative8`: Branch ±127 bytes (BRA, BEQ)
   - `Relative16`: Long branch ±32K (LBRA)
   - `Direct`: Direct page (8-bit)
   - `High8/Low8`: Split address
   - `CrossBank`: Wrapper function calls
3. Add tests for each relocation type

**Critical Logic**:
```rust
fn apply_relocation(
    data: &mut [u8],
    reloc: &Relocation,
    symbol_addr: u16,
    current_addr: u16,
) -> Result<(), String> {
    match reloc.reloc_type {
        RelocationType::Absolute16 => {
            // Patch 16-bit address
            let addr = symbol_addr.wrapping_add(reloc.addend as u16);
            data[offset] = (addr >> 8) as u8;      // High byte
            data[offset + 1] = (addr & 0xFF) as u8; // Low byte
        }
        RelocationType::Relative8 => {
            // Calculate PC-relative offset
            let target = symbol_addr.wrapping_add(reloc.addend as u16);
            let pc = current_addr.wrapping_add(1); // After instruction
            let offset = target.wrapping_sub(pc);
            if offset > 127 || offset < -128 {
                return Err("Branch offset out of range".into());
            }
            data[offset] = offset as u8;
        }
        // ... other types
    }
}
```

### Phase 7.5: ROM Writer (0.5 days)
**Goal**: Assemble final binary

**Steps**:
1. Port `rom_writer.rs` or create simplified version
2. Write banks to final .bin file
3. Handle bank padding/alignment

**Output Format**:
```
ROM Binary (.bin):
  Bytes 0x0000-0x3FFF:  Bank #0  (16KB)
  Bytes 0x4000-0x7FFF:  Bank #1  (16KB)
  ...
  Bytes 0x7C000-0x7FFFF: Bank #31 (16KB, fixed at $4000)
```

### Phase 7.6: High-Level API (0.5 days)
**Goal**: Simple linker interface

**API Design**:
```rust
pub struct Linker {
    script: LinkerScript,
    objects: Vec<VectrexObject>,
}

impl Linker {
    pub fn new(script: LinkerScript) -> Self;
    
    pub fn add_object(&mut self, obj: VectrexObject);
    
    pub fn link(self, bank_layout: BankLayout) 
        -> Result<LinkedBinary, LinkError>;
}

pub struct LinkedBinary {
    pub banks: Vec<BankBinary>,
    pub symbol_table: GlobalSymbolTable,
    pub debug_info: DebugInfo,
}

pub struct BankBinary {
    pub bank_id: u8,
    pub base_address: u16,
    pub data: Vec<u8>,
}
```

### Phase 7.7: Integration Tests (1 day)
**Goal**: End-to-end verification

**Test Cases**:
1. **Single-bank linking**:
   - All code in Bank #31
   - No cross-bank calls
   - Verify addresses
2. **Multibank linking**:
   - Code distributed across banks
   - Cross-bank calls with wrappers
   - Verify bank switching
3. **Relocation accuracy**:
   - JSR to local function
   - JSR to function in other bank
   - Data access (LDX #data)
4. **Symbol table correctness**:
   - All symbols resolved
   - Addresses match binary
5. **Error handling**:
   - Undefined symbol
   - Duplicate definition
   - Overflow (code too large)

---

## Differences from Core Implementation

### What to Keep
✅ **Object file format**: .vo format is solid
✅ **Symbol resolution**: Algorithm is correct
✅ **Relocation types**: Comprehensive coverage
✅ **Linker script concept**: Flexible memory layout

### What to Change
🔄 **Bank allocator**: Use Phase 4 (vpy_bank_allocator) instead of core's
🔄 **ROM writer**: Simplify (core may be overcomplicated)
🔄 **ASM parser**: Not needed (assembler outputs .vo directly)

### What's New in Buildtools
✨ **Integration with Phase 4**: Uses existing BankLayout
✨ **No ASM parsing**: Assembler (Phase 6) outputs .vo
✨ **Direct PDB generation**: Phase 9 uses symbol table
✨ **Cleaner tests**: Single + multibank test suites

---

## File Structure (Buildtools)

```
buildtools/vpy_linker/
├── Cargo.toml
├── src/
│   ├── lib.rs           (high-level API, Linker struct)
│   ├── object.rs        (VectrexObject, Section, Symbol, Relocation)
│   ├── script.rs        (LinkerScript, MemoryRegion)
│   ├── resolver.rs      (SymbolResolver, GlobalSymbolTable)
│   ├── relocator.rs     (apply_relocations implementation)
│   ├── rom_writer.rs    (assemble banks to .bin)
│   ├── types.rs         (common types: Address, BankId)
│   └── error.rs         (LinkError types)
└── tests/
    ├── object_io.rs     (save/load .vo files)
    ├── symbol_resolution.rs
    ├── relocations.rs   (each relocation type)
    ├── single_bank.rs   (end-to-end single-bank)
    ├── multibank.rs     (end-to-end multibank)
    └── errors.rs        (undefined symbols, overflow)
```

---

## Dependencies

### Cargo.toml
```toml
[package]
name = "vpy_linker"
version.workspace = true
edition.workspace = true
authors.workspace = true

[lints]
workspace = true

[dependencies]
vpy_bank_allocator = { path = "../vpy_bank_allocator" }
bincode = "1.3"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"

[dev-dependencies]
# For testing
```

---

## Timeline Estimate

| Task | Days | Cumulative |
|------|------|------------|
| Port object.rs + script.rs | 1.0 | 1.0 |
| Port resolver.rs | 1.0 | 2.0 |
| Integrate bank allocator | 0.5 | 2.5 |
| Relocation engine | 1.0 | 3.5 |
| ROM writer | 0.5 | 4.0 |
| High-level API | 0.5 | 4.5 |
| Integration tests | 1.0 | 5.5 |
| **Total** | **5.5** | **Phase 7 Complete** |

---

## Success Criteria

### Functionality
✅ Accept .vo files from Phase 6 assembler
✅ Use BankLayout from Phase 4 allocator
✅ Resolve all symbols (exports + imports)
✅ Apply all relocation types correctly
✅ Generate ROM binary (.bin)
✅ Output symbol table for Phase 9 PDB

### Testing
✅ 15+ tests passing:
  - 3 object I/O tests
  - 4 symbol resolution tests
  - 6 relocation tests (one per type)
  - 2 end-to-end tests (single + multibank)

### Integration
✅ Phase 6 assembler outputs .vo
✅ Phase 7 linker consumes .vo
✅ Phase 9 PDB uses linker symbol table
✅ No address guessing anywhere

---

## Next Steps After Phase 7

### Phase 8: vpy_binary_writer
- Trivial wrapper around std::fs::write
- Maybe add CRC checksum
- **Estimated**: 0.5 days

### Phase 9: vpy_debug_gen
- Consume linker symbol table
- Generate .pdb JSON
- Line map from linker debug info
- **Estimated**: 1 day

### Total Pipeline Complete
- **Phase 1-7**: 5.5 days (linker)
- **Phase 8-9**: 1.5 days (binary + debug)
- **Grand Total**: ~2 weeks for complete buildtools pipeline

---

## Critical Design Insight

**The linker is the single source of truth**:
- ❌ Old approach: Guess addresses from ASM files
- ✅ New approach: Linker computes authoritative addresses
- ✅ PDB derives from linker (always correct)
- ✅ Breakpoints work reliably
- ✅ Multibank addressing is accurate

This is **the most important architectural improvement** in the new pipeline.

---

## Port Checklist

### Before Starting
- [ ] Review core/src/linker/ files (done ✅)
- [ ] Create buildtools/vpy_linker/ directory
- [ ] Set up Cargo.toml with dependencies
- [ ] Plan test structure

### During Port
- [ ] Copy object.rs (preserve all types)
- [ ] Copy script.rs (preserve memory layout)
- [ ] Copy resolver.rs (preserve algorithm)
- [ ] Adapt for Phase 4 integration
- [ ] Write comprehensive tests
- [ ] Document relocation types

### After Port
- [ ] All tests passing (15+)
- [ ] Single-bank linking works
- [ ] Multibank linking works
- [ ] Symbol table accurate
- [ ] Ready for Phase 9 integration

---

**Status**: Ready to port from core/
**Complexity**: Medium (existing code is solid, need careful port + testing)
**Priority**: Critical (blocks Phase 9 PDB generation)
