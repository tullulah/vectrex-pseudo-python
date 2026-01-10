# Linker Phase 3: ASM Parser Implementation

## Status: ✅ PHASE 3.1 COMPLETE (2026-01-10)

## Summary
Phase 3.1 successfully implemented the ASM parser that extracts sections, symbols, and prepares for relocation collection. The parser can now read ASM with section markers and build object file metadata.

## Implementation Overview

### Module: `core/src/linker/asm_parser.rs` (323 lines, 3 tests passing)

#### Core Functions

1. **`extract_sections(asm: &str) -> Result<Vec<Section>, String>`**
   - Parses `.section` markers in ASM text
   - Groups ASM lines by section
   - Returns Vec<Section> with metadata
   - Binary assembly deferred to Phase 3.2

2. **`build_symbol_table(sections: &[Section], asm: &str) -> Result<SymbolTable, String>`**
   - Scans all sections for label definitions
   - Identifies exports (labels ending with `:`)
   - Skips local labels (starting with `.`)
   - Detects duplicate symbol definitions
   - Estimates symbol offsets within sections

3. **`collect_relocations(sections, symbols, asm) -> Result<Vec<Relocation>, String>`**
   - Placeholder for Phase 3.2
   - Will scan for JSR, BRA, LDD # instructions
   - Will create Relocation entries for external references

#### Helper Functions

- **`parse_section_markers(asm) -> Vec<ParsedSection>`**: Splits ASM by `.section` directives
- **`parse_flags(flags_str) -> u32`**: Converts "ax", "aw" to bitflags
- **`estimate_instruction_size(line) -> u32`**: Rough size estimation (for symbol offsets)

### Section Type Mapping

| GAS Directive | SectionType | Usage |
|---------------|-------------|-------|
| `.section NAME, "ax", @progbits` | `SectionType::Text` | Executable code |
| `.section NAME, "a", @progbits` | `SectionType::ReadOnly` | Constants, strings |
| `.section NAME, "aw", @nobits` | `SectionType::Bss` | Uninitialized variables |
| `.section NAME, "aw", @progbits` | `SectionType::Data` | Initialized data |

### Test Coverage

```rust
#[test]
fn test_parse_section_markers() {
    // Tests parsing 3 sections (.text.header, .text.main, .rodata)
    // Verifies section names extracted correctly
}

#[test]
fn test_parse_flags() {
    // Tests flag parsing: "ax" → 0x05, "a" → 0x01, "aw" → 0x03
}

#[test]
fn test_build_symbol_table() {
    // Tests symbol extraction from ASM
    // Verifies MAIN and LOOP_BODY symbols found
}
```

## Example Usage

```rust
use vectrex_lang::linker::{extract_sections, build_symbol_table};

let asm = r#"
.section .text.main, "ax", @progbits
MAIN:
    JSR Wait_Recal
    RTS

.section .rodata, "a", @progbits
STR_0:
    FCC "HELLO"
    FCB $80
"#;

// Extract sections
let sections = extract_sections(asm)?;
assert_eq!(sections.len(), 2);

// Build symbol table
let symbols = build_symbol_table(&sections, asm)?;
assert_eq!(symbols.exports.len(), 2); // MAIN, STR_0
```

## Integration with Compiler

### Future Workflow (Phase 3.2-3.3)

```rust
// In main.rs - object file generation
fn compile_to_object(source: &Path) -> Result<VectrexObject> {
    // Phase 1-3: Parse VPy to AST
    let module = parse_vpy(source)?;
    
    // Phase 4: Generate ASM with sections
    let asm = emit_asm_with_sections(&module)?;
    
    // Phase 5: Parse ASM → sections + symbols
    let sections = extract_sections(&asm)?;
    let symbols = build_symbol_table(&sections, &asm)?;
    let relocations = collect_relocations(&sections, &symbols, &asm)?;
    
    // Phase 6: Assemble sections to binary (Phase 3.2)
    for section in &mut sections {
        section.data = assemble_section(&section.name, &asm)?;
    }
    
    // Phase 7: Write .vo file
    let obj = VectrexObject {
        header: ObjectHeader { /* ... */ },
        sections,
        symbols,
        relocations,
        debug_info: DebugInfo::default(),
    };
    
    obj.write(output)?;
    Ok(obj)
}
```

## Current Limitations (To Address in Phase 3.2)

1. **Binary Assembly**: Sections contain empty `data: Vec<u8>`
   - Need to integrate with `asm_to_binary.rs` assembler
   - Or call external assembler (lwasm) per section

2. **Symbol Offsets**: Using rough size estimates
   - Need accurate instruction size calculation
   - Should match real assembler output

3. **Relocation Detection**: Placeholder implementation
   - Need to parse JSR, BRA, LDD #, etc.
   - Need to identify external vs internal references
   - Need to determine relocation types (Absolute16, Relative8, etc.)

4. **Local Labels**: Currently skipped (`.label:`)
   - May be needed for complex control flow
   - Consider preserving for debugging

## Next Steps

### Phase 3.2: Binary Assembly Integration
- Integrate with `core/src/backend/asm_to_binary.rs`
- Assemble each section independently
- Collect actual instruction sizes for symbol offsets
- Handle forward references within sections

### Phase 3.3: Relocation Collection
- Parse JSR, BRA, LDD #, LDX #, etc.
- Identify target symbols
- Determine relocation types:
  - JSR → Absolute16
  - BRA/BEQ → Relative8
  - LBRA → Relative16
- Create Relocation entries with offset and symbol

### Phase 3.4: Testing with Real Programs
- Test with `test_section_emission.vpy`
- Verify all sections extracted correctly
- Verify all symbols found
- Compare symbol offsets with actual ASM

## Files Modified

1. **core/src/linker/asm_parser.rs** (NEW - 323 lines)
   - Complete ASM parsing implementation
   - 3/3 tests passing

2. **core/src/linker/mod.rs** (MODIFIED)
   - Added `pub mod asm_parser;`
   - Added public exports: `extract_sections`, `build_symbol_table`, `collect_relocations`

## Testing Results

```
running 3 tests
test linker::asm_parser::tests::test_parse_flags ... ok
test linker::asm_parser::tests::test_parse_section_markers ... ok
test linker::asm_parser::tests::test_build_symbol_table ... ok

test result: ok. 3 passed; 0 failed; 0 ignored
```

## Commit History

- `feat: linker Phase 3.1 - ASM parser with section extraction` (2026-01-10)
  - Created asm_parser.rs with 3 core functions
  - Added 3 unit tests (all passing)
  - Integrated with linker module

---
**Progress**: 2.5/5 phases complete (50%)  
**Next**: Phase 3.2 - Binary assembly integration
