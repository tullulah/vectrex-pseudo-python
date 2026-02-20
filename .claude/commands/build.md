Build the VPy buildtools Rust workspace.

Run `cargo build --all` from the `buildtools/` directory. Report compilation errors clearly, grouped by crate. If there are errors, identify which compiler phase (1-9) is affected and suggest the relevant source files to investigate.

The 9 phases and their crates are:
- Phase 1: vpy_loader
- Phase 2: vpy_parser
- Phase 3: vpy_unifier
- Phase 4: vpy_bank_allocator
- Phase 5: vpy_codegen
- Phase 6: vpy_assembler
- Phase 7: vpy_linker (in progress)
- Phase 8: vpy_binary_writer
- Phase 9: vpy_debug_gen
