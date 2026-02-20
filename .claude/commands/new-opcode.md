Add a new MC6809 opcode to the VPy assembler.

Usage: /new-opcode [OPCODE_NAME]

Steps to implement a new opcode (from docs/INDEX.md workflow):

1. Check `docs/COMPILER_STATUS.md` for the list of pending opcodes and verify $ARGUMENTS is there
2. Read `buildtools/vpy_assembler/src/m6809/asm_to_binary.rs` — add dispatch in the opcode match
3. Read `buildtools/vpy_assembler/src/m6809/binary_emitter.rs` — implement the emit_xxx methods for each addressing mode (immediate, direct, extended, indexed, inherent)
4. Add a test in `buildtools/vpy_assembler/src/m6809/asm_to_binary.rs` that assembles the opcode and checks the byte output
5. Run `cargo test -p vpy_assembler` to confirm

Reference the MC6809 opcode table at `docs/6809_opcodes.md` for the correct byte values and addressing modes.
