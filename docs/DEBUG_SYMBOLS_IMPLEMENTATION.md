# Debug Symbols (.pdb) Implementation

## Overview

The VPy compiler now generates `.pdb` (Program Database) files alongside `.asm` and `.bin` outputs. These files contain debug symbols that map VPy source code to compiled binary addresses, enabling debugger features like breakpoints, step-through execution, and source-level debugging.

## File Format

The `.pdb` file is a JSON document with the following structure:

```json
{
  "version": "1.0",
  "source": "main.vpy",
  "binary": "main.bin",
  "entry_point": "0x0000",
  "symbols": {
    "START": "0x0000",
    "MAIN": "0x0000",
    "LOOP_BODY": "0x0000",
    "my_function": "0x0000"
  },
  "lineMap": {
    "5": "0x0000",
    "10": "0x0020",
    "15": "0x0040"
  }
}
```

### Fields

- **version**: Debug format version (currently "1.0")
- **source**: Source VPy filename (e.g., "main.vpy")
- **binary**: Compiled binary filename (e.g., "main.bin")
- **entry_point**: Program entry point address in hex format
- **symbols**: Map of symbol names → addresses
  - `START`: Entry point label (if main() has content)
  - `MAIN`: Main loop label
  - `LOOP_BODY`: Loop function subroutine
  - Custom function names (uppercase)
- **lineMap**: Map of VPy line numbers → binary addresses
  - Keys are line numbers as strings (1-indexed)
  - Values are hex addresses matching source positions

## Current Implementation

### Phase 1 (Completed - Oct 16, 2025)

✅ **Basic Symbol Generation**:
- Function symbols (main, loop, custom functions)
- Entry point tracking
- JSON serialization with pretty-printing
- Automatic .pdb generation during compilation

**Limitations**:
- Symbol addresses are placeholder (0x0000) - actual addresses not tracked yet
- Line mapping is empty - requires AST line tracking during codegen
- Only M6809/Vectrex backend supported

### Phase 2 (Planned - Future)

🔲 **Line Mapping**:
- Track AST line numbers during code generation
- Map VPy statements to generated ASM addresses
- Populate lineMap with accurate source→binary mappings

🔲 **Accurate Address Tracking**:
- Calculate actual symbol addresses during emission
- Track ORG directives and address progression
- Update symbols with real memory locations

🔲 **IDE Integration**:
- Load .pdb in IDE debugger frontend
- Translate UI breakpoints (line numbers) → addresses
- Send breakpoint addresses to JSVecx emulator
- Highlight current line during step-through

## Usage

### Compilation

The `.pdb` file is generated automatically when compiling VPy code:

```bash
# From project root
cargo run --bin vectrexc -- build my_program.vpy

# Generates:
# my_program.asm  (assembly source)
# my_program.pdb  (debug symbols)
```

With binary generation:

```bash
cargo run --bin vectrexc -- build my_program.vpy --bin

# Generates:
# my_program.asm  (assembly source)
# my_program.bin  (compiled binary)
# my_program.pdb  (debug symbols)
```

### IDE Integration (Planned)

```typescript
// Load debug symbols
const pdb = JSON.parse(fs.readFileSync('program.pdb'));

// Map breakpoint line → address
const lineNumber = 10;
const address = pdb.lineMap[lineNumber]; // e.g., "0x0040"

// Send to JSVecx
jsvecx.setBreakpoint(parseInt(address, 16));
```

## Technical Details

### Code Locations

- **Definition**: `core/src/backend/debug_info.rs`
- **Generation**: `core/src/backend/m6809.rs` (`emit_with_debug()`)
- **Serialization**: `core/src/main.rs` (Phase 5.5)
- **Format**: Serde JSON with `#[serde(rename)]` for camelCase keys

### Dependencies

- `serde = { version = "1.0", features = ["derive"] }`
- `serde_json = "1.0"`

### Backend Architecture

```
emit() → emit_with_debug() → (String, DebugInfo)
                                      ↓
                              DebugInfo::to_json()
                                      ↓
                              write to .pdb file
```

## Future Enhancements

### Short Term

1. **Accurate Symbol Addresses**: Track actual addresses during codegen
2. **Basic Line Mapping**: Map main()/loop() bodies to addresses
3. **IDE .pdb Loader**: Parse and use .pdb in Monaco Editor

### Medium Term

4. **Full Line Tracking**: AST line numbers throughout codegen pipeline
5. **Local Variable Symbols**: Track function-local variables
6. **Call Stack Reconstruction**: JSR/RTS tracking for stack traces

### Long Term

7. **Source Maps v3**: Migrate to standard SourceMap format
8. **Multi-Backend Support**: ARM/CortexM debug info
9. **DWARF Integration**: Standard debugging format support

## Naming Convention

The `.pdb` extension was chosen for recognizability:
- Familiar to developers (VS/Windows debugging)
- Distinct from `.map` (linker maps) and `.dbg` (generic debug)
- Easy to filter in IDE file pickers
- Standard association with program databases

## References

- **User Request**: "llamalo pdb para que por la extension sea reconocible"
- **Vectrex Constraints**: 8KB .bin limit → .pdb is SEPARATE file
- **Implementation Date**: October 16, 2025
- **Status**: Phase 1 Complete ✅

## Testing

Verify .pdb generation:

```bash
# Compile test program
cd core
cargo run --bin vectrexc -- build ../rotating_line_correct.vpy

# Check .pdb exists
cat ../rotating_line_correct.pdb

# Expected output:
# {
#   "version": "1.0",
#   "source": "rotating_line_correct.vpy",
#   "binary": "rotating_line_correct.bin",
#   "entry_point": "0x0000",
#   "symbols": { "START": "0x0000", ... },
#   "lineMap": {}
# }
```

## Architecture Compliance

✅ **1:1 Verification**: N/A (original feature, not ported from Vectrexy)
✅ **No Synthetic Data**: Debug info derived from real AST/codegen
✅ **BIOS Real**: Not applicable (debug symbols, not emulation)
✅ **PowerShell v5.1**: JSON I/O uses standard Rust std::fs
✅ **Git Branch**: Commits to `master` (NOT `main`)
