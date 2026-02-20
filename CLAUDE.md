# VPy — Vectrex Pseudo Python

A complete development environment for the Vectrex retro gaming console: Python-like language, 9-phase Rust compiler, JavaScript emulator, and Electron IDE.

## Project Layout

```
buildtools/   — 9-phase modular Rust compiler (ACTIVE)
core/         — Legacy monolithic compiler (being retired)
ide/          — Electron IDE with React+Monaco frontend
  electron/   — Main process, IPC, bundled binaries
  frontend/   — React + Vite UI
  mcp-server/ — MCP bridge for AI tools (port 9123)
examples/     — VPy game examples (pang, animations, etc.)
docs/         — 187 docs; see docs/INDEX.md
```

## Compiler Pipeline (buildtools/)

```
Phase 1: vpy_loader         → ProjectInfo
Phase 2: vpy_parser         → Vec<Module> (AST)
Phase 3: vpy_unifier        → UnifiedModule
Phase 4: vpy_bank_allocator → BankLayout
Phase 5: vpy_codegen        → GeneratedIR (ASM)
Phase 6: vpy_assembler      → Vec<ObjectFile>
Phase 7: vpy_linker         → LinkedBinary + SymbolTable  ← ADDRESS SOURCE OF TRUTH
Phase 8: vpy_binary_writer  → .bin file
Phase 9: vpy_debug_gen      → .pdb file
```

Status: Phases 1–6 complete, Phase 7 in progress, Phases 8–9 planned.

## Build & Test

```bash
cd buildtools && cargo build --all    # Build all phases
cd buildtools && cargo test --all     # Run all phase tests
cd buildtools && cargo test -p <crate>  # Test one phase
```

## Key Rules

- **Only the linker (Phase 7) computes final addresses.** Never derive addresses elsewhere.
- **No interrupt vectors in cartridge ROM.** They live in BIOS ($E000-$FFFF).
- **Symbol naming**: `MODULE_symbol` (uppercase module prefix, original-case symbol).
- **Multibank**: Every bank uses `ORG $0000`. Helpers bank = `(total / bank_size) - 1`.
- **Tests required per phase**: single-bank case, multibank case, error case.

## VPy Language

Python-like syntax, compiles to MC6809 assembly. Key builtins: `WAIT_RECAL()`, `SET_INTENSITY()`, `DRAW_LINE()`, `J1_X/Y()`, `PLAY_MUSIC()`. See `docs/COMPILER_STATUS.md` for full reference.

## IDE

- Electron + React + Monaco. State managed with Zustand.
- LSP server: `ide/electron/resources/vpy_lsp` (Rust binary).
- Emulator: JSVecX running in renderer process.
- MCP server on port 9123 (IDE must be running first).

## Important Docs

- `docs/INDEX.md` — navigation guide
- `docs/COMPILER_STATUS.md` — opcodes, backlog, known issues
- `docs/SUPER_SUMMARY.md` — emulator architecture (32 sections)
- `docs/TIMING.md` — deterministic cycle model
- `buildtools/README.md` — phase-by-phase status
