# Vectrex Pseudo Python (VPy)

**Programming language and complete development environment for Vectrex**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Node](https://img.shields.io/badge/node-22.x-green.svg)](https://nodejs.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> Complete development system for Vectrex with modular compiler, native M6809 assembler, integrated JSVecX emulator and visual editors for graphics and levels.

## 🎯 Key Features

- **VPy Language**: Python-like syntax optimized for Vectrex
- **Modular Compiler**: 9-phase pipeline with native M6809 assembler (buildtools)
- **No external dependencies**: No lwasm or external tools required
- **JSVecX Emulator**: JavaScript port of VecX integrated in the IDE
- **Visual Editors**: Graphical tools for vectors (.vec), animations (.vanim) and levels (.vplay)
- **Module System**: Imports, automatic tree shaking, unified symbols
- **Multibank ROM**: Support for cartridges up to 4MB (256 banks × 16KB)

## 🏗️ Compiler Architecture

The new modular compiler (`buildtools/`) replaces the old monolithic one (`core/`):

```
9-Phase Pipeline:
1. vpy_loader       → Reads .vpyproj, discovers files and assets
2. vpy_parser       → Lexer + Parser → AST per module
3. vpy_unifier      → Resolves imports, merges modules, tree shaking
4. vpy_bank_allocator → Assigns functions to banks (multibank)
5. vpy_codegen      → Generates M6809 ASM per bank
6. vpy_assembler    → Assembles to object files (.vo) with relocations
7. vpy_linker       → Real linker (source of truth for addresses)
8. vpy_binary_writer → Writes final .bin
9. vpy_debug_gen    → Generates .pdb for debugging
```

**Advantages over the old compiler**:
- ✅ Native M6809 assembler (no lwasm)
- ✅ Real linker with relocations and symbol table
- ✅ Single source of truth for addresses
- ✅ PDB generated correctly from linker
- ✅ Comprehensive tests per phase
- ✅ Easy to extend and maintain

## 🚀 Quick Start

### Requirements
- **Rust** 1.70+ ([install](https://rustup.rs/))
- **Node.js** 22+ ([install](https://nodejs.org/))
- **Vectrex BIOS**: 8KB (`bios.bin`)

### Installation

```bash
# 1. Clone repository
git clone https://github.com/tullulah/vectrex-pseudo-python.git
cd vectrex-pseudo-python

# 2. Build the compiler (buildtools)
cd buildtools
cargo build --release --bin vpy_cli
cd ..

# 3. Install IDE dependencies
cd ide/frontend && npm install
cd ../electron && npm install
cd ../..

# 4. Start the IDE
./run-ide.sh          # macOS/Linux
# or
run-ide.ps1           # Windows (PowerShell)
```

### Your First Program

```python
# game.vpy
META TITLE = "My First Game"

player_x = 0
player_y = 0

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Read joystick
    player_x = player_x + J1_X()
    player_y = player_y + J1_Y()
    
    # Draw player
    DRAW_LINE(player_x-10, player_y, player_x+10, player_y, 127)
    DRAW_LINE(player_x, player_y-10, player_x, player_y+10, 127)
```

**Compile from terminal:**
```bash
# With the new modular compiler (recommended)
cd buildtools
cargo run --release --bin vpy_cli -- build ../game.vpy -o game.bin

# Or from the IDE: "Run" button (compile + load in emulator)
```

## 📚 Documentation

### Compiler (Buildtools)
- **[buildtools/README.md](buildtools/README.md)** - Modular pipeline architecture
- **[buildtools/STATUS.md](buildtools/STATUS.md)** - Current status of each phase
- **Status per phase**: Loader ✅, Parser ✅, Unifier ✅, Allocator ✅, Codegen ✅, Assembler ✅

### VPy Language
- **[docs/COMPILER_STATUS.md](docs/COMPILER_STATUS.md)** - Syntax and builtins reference
- **[docs/PHASE6_SUMMARY.md](docs/PHASE6_SUMMARY.md)** - Module system and imports
- **Examples**: See `examples/` folder (pang, animations, multi-module)

### Multibank and Assets
- **[docs/MULTIBANK_DEBUG_GUIDE.md](docs/MULTIBANK_DEBUG_GUIDE.md)** - Multibank ROMs (up to 4MB)
- **Assets**: Vectors (.vec), music (.vmus), sounds (.vsfx), levels (.vplay)

### IDE
- **Emulator**: Integrated JSVecX (JavaScript port of VecX)
- **Vector Editor**: Drawing tools for .vec graphics
- **Animation Editor**: Frame sequences with .vec
- **Level Editor (Playground)**: Visual composition of objects and animations
- **Debugging**: Breakpoints, step-by-step, memory inspection

## 🏗️ Arquitectura del Proyecto

```
vectrex-pseudo-python/
├── buildtools/            # 🆕 Compilador modular (9 crates)
│   ├── vpy_loader/       # Fase 1: Carga .vpyproj y descubre archivos
│   ├── vpy_parser/       # Fase 2: Lexer + Parser → AST
│   ├── vpy_unifier/      # Fase 3: Resuelve imports, tree shaking
│   ├── vpy_bank_allocator/ # Fase 4: Asigna funciones a banks
│   ├── vpy_codegen/      # Fase 5: Genera ASM M6809
│   ├── vpy_assembler/    # Fase 6: Ensamblador nativo M6809
│   ├── vpy_linker/       # Fase 7: Linker real con relocaciones
│   ├── vpy_binary_writer/# Fase 8: Escribe .bin final
│   ├── vpy_debug_gen/    # Fase 9: Genera .pdb
│   └── vpy_cli/          # CLI unificado
├── core/                  # [LEGACY] Compilador antiguo monolítico
├── ide/
│   ├── frontend/         # React + Monaco + Vite
│   │   └── public/jsvecx/ # Emulador JSVecX (JavaScript)
│   └── electron/         # Electron shell + IPC
├── examples/             # Proyectos de ejemplo (pang, etc.)
└── docs/   
│   ├── frontend/      # React + Monaco + Vite
│   └── electron/      # Electron shell
├── examples/          # Proyectos de ejemplo
└── docs/              # Documentación técnica
```

## 🎮 Language Features

### Data Types
```python
# Variables
x = 10
name = "VECTREX"
colors = [255, 200, 150]

# Constants (ROM-only)
const ENEMIES = 5
const LEVEL_DATA = [1, 2, 3, 4]
```

### Builtin Functions
```python
# Graphics
SET_INTENSITY(brightness)
DRAW_LINE(x0, y0, x1, y1, intensity)
DRAW_VECTOR("sprite_name")
PRINT_TEXT(x, y, "HELLO")

# Input
joy_x = J1_X()              # -1, 0, 1
joy_y = J1_Y()
btn = J1_BUTTON_1()         # 0 or 1

# Audio
PLAY_MUSIC("theme")
PLAY_SFX("explosion", 0)    # channel 0-2
```

### Asset System
```python
# Assets are automatically discovered:
# - assets/vectors/*.vec
# - assets/music/*.vmus

def loop():
    DRAW_VECTOR("player")      # Uses player.vec
    PLAY_MUSIC("theme")        # Uses theme.vmus
```

### Modules
```python
# input.vpy
def get_input():
    return J1_X(), J1_Y()

# main.vpy
import input

def loop():
    x, y = input.get_input()
```

## 🔧 Development

### Build the New Compiler (Buildtools)
```bash
cd buildtools
cargo build --release --bin vpy_cli
```

### Run the Compiler
```bash
# Compile VPy file
cd buildtools
cargo run --release --bin vpy_cli -- build ../examples/pang/src/main.vpy -o pang.bin

# See help
cargo run --release --bin vpy_cli -- --help
```

### Compiler Tests
```bash
cd buildtools

# Tests per crate
cargo test -p vpy_parser
cargo test -p vpy_unifier
cargo test -p vpy_codegen
# ... etc

# All workspace tests
cargo test --all
```

### IDE Build
```bash
cd ide/frontend
npm run build        # Build frontend (React + Vite)

cd ../electron
npm run build        # Build Electron app
```

### IDE Development
```bash
# Terminal 1: Frontend dev server
cd ide/frontend
npm run dev          # Vite dev server on port 5173

# Terminal 2: Electron
cd ide/electron
npm start            # Electron pointing to localhost:5173
```

## 📦 Formato de Archivos

### .vec - Vector Graphics
```json
{
  "name": "player",
  "canvas": {"width": 256, "height": 256, "origin": "center"},
  "layers": [{
    "paths": [{
      "intensity": 127,
      "closed": true,
      "points": [
        {"x": 0, "y": 20},
        {"x": -15, "y": -10},
        {"x": 15, "y": -10}
      ]
    }]
  }]
}
```

### .vanim - Animations
```json
{
  "name": "explosion",
  "frames": [
    {"vector": "explosion_01", "duration": 2},
    {"vector": "explosion_02", "duration": 2},
    {"vector": "explosion_03", "duration": 2}
  ]
}
```

### .vplay - Levels
```json
{
  "name": "level_1",
  "background": {"r": 0, "g": 0, "b": 0},
  "objects": [
    {
      "type": "player",
      "animation": "player_idle",
      "x": 0,
      "y": -50
    },
    {
      "type": "enemy",
      "animation": "enemy_walk",
      "x": 50,
      "y": 30
    }
  ]
}
```

## 🎯 Project Status (February 2026)

### ✅ Buildtools (Modular Compiler)
- ✅ **9 phases completed**: Loader → Parser → Unifier → Allocator → Codegen → Assembler → Linker → Writer → Debug
- ✅ **Native M6809 assembler**: No lwasm or external tools required
- ✅ **Real linker**: Relocations, symbol table, single source of truth
- ✅ **Tree shaking**: Automatically eliminates unused code
- ✅ **Multibank**: Support for ROMs up to 4MB (256 banks × 16KB)
- ✅ **Comprehensive tests**: 100+ tests covering all phases

### ✅ IDE and Tools
- ✅ **JSVecX Emulator**: Integrated JavaScript port of VecX
- ✅ **Vector Editor**: Drawing tools with real-time preview
- ✅ **Animation Editor**: Visual timeline for sequences
- ✅ **Level Editor (Playground)**: Visual composition of objects and animations
- ✅ **Debugging**: Breakpoints, step execution, memory inspector
- ✅ **Monaco Editor**: Syntax highlighting for VPy
- ✅ **Project system**: .vpyproj with metadata and configuration

### ✅ VPy Language
- ✅ **Module system**: Imports with automatic resolution
- ✅ **Structs and arrays**: Composite types with automatic layout
- ✅ **Const arrays**: ROM-only data with efficient indexing
- ✅ **Integrated assets**: Vectors, music, sounds, levels
- ✅ **Builtins**: 40+ functions (graphics, input, audio, collisions)

### 🚧 In Development
- 🚧 **core → buildtools migration**: Integrate new CLI into IDE
- 🚧 **Updated LSP**: Use new compiler parser
- 🚧 **Optimizations**: Dead code elimination, constant propagation

### 📋 Roadmap
- [ ] Sprite generator from PNG images
- [ ] 2D physics system (collisions, gravity)
- [ ] Profiling tools (CPU, memory)
- [ ] Export to physical cartridges (.vec format)
- [ ] Networking for multi-cart

## 🤝 Contributing

Contributions are welcome:

1. Fork the project
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is under MIT license. See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- **JSVecX** by raz0red - JavaScript port of VecX used as emulator
- **VecX** by Valavan Manohararajah - Original reference emulator
- **Malban** - For extensive documentation and Vide (Vectrex Integrated Development Environment)
- **Technobly** - For his help and the Discord community
- **Jason Kopp** - For being an inspiration to the community
- **Vectrex Fans Unite!** community - For continued support and enthusiasm
- **Vectrex Community** for hardware and BIOS documentation
- **Vectrex BIOS** (publicly released) for development

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/tuusuario/vectrex-pseudo-python/issues)
- **Documentation**: [docs/](docs/) folder
- **Examples**: [examples/](examples/) folder

---

**Made with ❤️ for the Vectrex community**
