# Linker End-to-End Test

This directory contains test files for the VPy linker (Phase 5).

## Files

- **lib.asm**: Reference assembly for library function (documentation only)
- **main.asm**: Reference assembly for main function (documentation only)
- **create_test_objects.py**: Early Python generator attempt (abandoned - wrong binary format)

## How to Generate Test Objects

The test objects are generated using a Rust program that produces valid bincode-serialized .vo files:

```bash
cargo run --bin create_test_vo
```

This creates:
- **lib.vo**: Object file exporting `helper_function`
- **main.vo**: Object file importing `helper_function` with relocation

## How to Link

```bash
cargo run --bin vectrexc -- link lib.vo main.vo -o linked.bin
```

Expected output:
```
✓ SUCCESS: Linked binary generated
  Output: linked.bin
  Size: 15 bytes
  Base: 0xC880
  Objects linked: 2
  Total symbols: 2
```

## Binary Layout

```
Address   Hex bytes       Disassembly         Section
--------  -------------   -----------------   --------
0xC880    86 7f           LDA #127            lib
0xC882    bd f2 ab        JSR $F2AB           (Intensity_a BIOS)
0xC885    39              RTS
0xC886    86 7f           LDA #127            main
0xC888    bd f2 ab        JSR $F2AB           (Intensity_a BIOS)
0xC88B    bd c8 80        JSR $C880           ✅ Patched to helper_function
0xC88E    39              RTS
```

## What This Tests

1. **Phase 1**: Load multiple .vo files
2. **Phase 2**: Build global symbol table
3. **Phase 3**: Resolve imports (main imports helper_function from lib)
4. **Phase 4**: Assign addresses (lib @ 0xC880, main @ 0xC886)
5. **Phase 5**: Apply relocations (patch JSR placeholder with helper_function address)
6. **Phase 6**: Merge sections into single binary
7. **Phase 7**: Write final binary

## Key Implementation Details

### Relocation Offset Calculation
For the JSR instruction at offset 5 in main section:
```
Offset  Bytes          Instruction
------  -------------  -----------
0-1     86 7f          LDA #127
2-4     bd f2 ab       JSR $F2AB
5       bd             JSR opcode
6-7     00 00          Address placeholder ← Relocation target
8       39             RTS
```

The relocation must point to offset **6** (first byte of address), not 5 (opcode) or 7 (second address byte).

### Big-Endian (M6809)
The address is written big-endian: `BD C8 80` means JSR $C880 (not $80C8).

## Current Limitations

### VPy Compiler Doesn't Support Modular Compilation
The VPy compiler currently generates monolithic code with a global RAM layout. When using `compile-object`, it references undefined variables like `VAR_ARG0`:

```
Error: SYMBOL:VAR_ARG0+1
```

This is why we generate test objects programmatically using Rust.

### Future Work
To support modular VPy compilation:
1. Implement `import` statement syntax
2. Add per-module symbol tables
3. Modify compiler to generate external symbol references
4. Track imports/exports during compilation
5. Generate relocations for cross-module calls

## Verification

You can inspect the linked binary with:
```bash
hexdump -C linked.bin
```

Expected output:
```
00000000  86 7f bd f2 ab 39 86 7f  bd f2 ab bd c8 80 39     |.....9........9|
```

All 7 phases of the linker are tested and working! 🎉
