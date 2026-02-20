# Vectrex Studio

A complete development environment for the [Vectrex](https://en.wikipedia.org/wiki/Vectrex) console (Motorola 6809).

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Node](https://img.shields.io/badge/node-18.x-green.svg)](https://nodejs.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## What is this?

Vectrex Studio lets you write games for the Vectrex in **VPy**, a Python-like high-level language, and run them instantly in an embedded emulator.

**Components:**
- **VPy compiler** — 9-phase pipeline (Rust) from `.vpy` source to Vectrex ROM
- **Native MC6809 assembler** — built-in, no external tools required
- **IDE** — Electron + React + Monaco editor with LSP (completions, hover, diagnostics, go-to-def, semantic tokens)
- **Emulator** — custom JSVecX embedded in the IDE panel
- **MCP server** — debugger observability tools for AI assistants
- **Asset pipeline** — `.vec` (vectors), `.vmus` (music), `.vsfx` (sound effects)

---

## Quick Start

**Prerequisites:**
- Rust 1.70+ ([install](https://rustup.rs/))
- Node.js 18+ ([install](https://nodejs.org/))
- Vectrex BIOS (8KB) placed at `ide/frontend/public/bios.bin`

**Build and launch:**

```bash
# 1. Build the VPy compiler
cargo build --bin vectrexc --release

# 2. Install IDE dependencies
cd ide/frontend && npm install
cd ../electron && npm install
cd ../..

# 3. Launch IDE
pwsh ./run-ide.ps1
# Or manually: cd ide/frontend && npm run dev, then cd ../electron && npm start
```

See [docs/SETUP.md](docs/SETUP.md) for full step-by-step instructions.

---

## Documentation

| Doc | Description |
|-----|-------------|
| [docs/START_HERE.md](docs/START_HERE.md) | Where to go depending on your goal |
| [docs/SETUP.md](docs/SETUP.md) | Full installation and build guide |
| [docs/MANUAL.md](docs/MANUAL.md) | VPy language reference |
| [docs/COMPILER_STATUS.md](docs/COMPILER_STATUS.md) | Compiler phase status and roadmap |
| [docs/INDEX.md](docs/INDEX.md) | Full documentation index |

---

## What Works

### Compiler
- Full VPy language: functions, loops, conditionals, arithmetic, bitwise ops
- 100+ MC6809 opcodes with all addressing modes
- Multibank ROM support (up to 4MB)
- Optimizations: constant folding, dead code elimination, peephole
- Debug symbol generation (`.pdb` files)
- Project format: `.vpyproj` (TOML)

### IDE
- Monaco editor with full LSP: completions, hover, go-to-definition, semantic tokens, diagnostics
- Emulator panel (JSVecX) with Play/Pause/Reset
- Run button: compiles and loads ROM into emulator in one click
- File explorer, error panel, output panel, memory viewer, debug panel
- MCP server for AI assistant integration

### Asset Formats
| Format | Description | Status |
|--------|-------------|--------|
| `.vec` | Vector graphics lists | ✅ Working |
| `.vmus` | Music (PSG AY-3-8912) | ✅ Working |
| `.vsfx` | Sound effects | ✅ Working |
| `.vplay` | Level data | ⏳ Partial |

---

## Project Structure

```
buildtools/     # VPy compiler (Rust, one crate per phase)
  vpy_loader/   # Phase 1 — project file loading (.vpyproj)
  vpy_parser/   # Phase 2 — parsing to AST
  vpy_unifier/  # Phase 3 — multi-file AST unification
  vpy_bank_allocator/  # Phase 4 — multibank ROM allocation
  vpy_codegen/  # Phase 5 — AST to MC6809 assembly
  vpy_assembler/ # Phase 6 — assembly to machine code
  vpy_linker/   # Phase 7 — symbol resolution and linking
  vpy_binary_writer/  # Phase 8 — binary output
  vpy_debug_gen/ # Phase 9 — debug symbol generation
  vpy_cli/      # CLI orchestrator (vectrexc)
core/           # Shared VPy language services (LSP, lexer, parser)
ide/            # Electron IDE shell + React frontend
  frontend/     # React/TypeScript UI (Monaco, panels, emulator)
  electron/     # Electron shell
  mcp-server/   # MCP server (Node.js)
emulator/       # Rust emulator crate (not currently used in IDE)
docs/           # All documentation
examples/       # Example VPy programs
```

---

## License

MIT
