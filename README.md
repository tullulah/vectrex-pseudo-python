# Vectrex Pseudo Python (VPy)

**Lenguaje de programación y entorno de desarrollo completo para Vectrex**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Node](https://img.shields.io/badge/node-22.x-green.svg)](https://nodejs.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> Sistema completo de desarrollo para Vectrex con compilador modular, ensamblador nativo M6809, emulador JSVecX integrado y editores visuales para gráficos y niveles.

## 🎯 Características Principales

- **Lenguaje VPy**: Sintaxis Python-like optimizada para Vectrex
- **Compilador Modular**: Pipeline de 9 fases con ensamblador M6809 nativo (buildtools)
- **Sin dependencias externas**: No requiere lwasm ni herramientas externas
- **Emulador JSVecX**: Port JavaScript de VecX integrado en el IDE
- **Editores Visuales**: Herramientas gráficas para vectores (.vec), animaciones (.vanim) y niveles (.vplay)
- **Sistema de Módulos**: Imports, tree shaking automático, símbolos unificados
- **Multibank ROM**: Soporte para cartuchos de hasta 4MB (256 banks × 16KB)

## 🏗️ Arquitectura del Compilador

El nuevo compilador modular (`buildtools/`) reemplaza al antiguo monolítico (`core/`):

```
Pipeline de 9 Fases:
1. vpy_loader       → Lee .vpyproj, descubre archivos y assets
2. vpy_parser       → Lexer + Parser → AST por módulo
3. vpy_unifier      → Resuelve imports, une módulos, tree shaking
4. vpy_bank_allocator → Asigna funciones a banks (multibank)
5. vpy_codegen      → Genera ASM M6809 por bank
6. vpy_assembler    → Ensambla a object files (.vo) con relocaciones
7. vpy_linker       → Linker real (source of truth para direcciones)
8. vpy_binary_writer → Escribe .bin final
9. vpy_debug_gen    → Genera .pdb para debugging
```

**Ventajas sobre el compilador antiguo**:
- ✅ Ensamblador M6809 nativo (no lwasm)
- ✅ Linker real con relocaciones y symbol table
- ✅ Single source of truth para direcciones
- ✅ PDB generado correctamente desde linker
- ✅ Tests comprehensivos por fase
- ✅ Fácil de extender y mantener

## 🚀 Quick Start

### Requisitos
- **Rust** 1.70+ ([instalar](https://rustup.rs/))
- **Node.js** 22+ ([instalar](https://nodejs.org/))
- **BIOS Vectrex**: 8KB (`bios.bin`)

### Instalación

```bash
# 1. Clonar repositorio
git clone https://github.com/tullulah/vectrex-pseudo-python.git
cd vectrex-pseudo-python

# 2. Compilar el compilador (buildtools)
cd buildtools
cargo build --release --bin vpy_cli
cd ..

# 3. Instalar dependencias del IDE
cd ide/frontend && npm install
cd ../electron && npm install
cd ../..

# 4. Iniciar el IDE
./run-ide.sh          # macOS/Linux
# o
run-ide.ps1           # Windows (PowerShell)
```

### Tu Primer Programa

```python
# game.vpy
META TITLE = "Mi Primer Juego"

player_x = 0
player_y = 0

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Leer joystick
    player_x = player_x + J1_X()
    player_y = player_y + J1_Y()
    
    # Dibujar jugador
    DRAW_LINE(player_x-10, player_y, player_x+10, player_y, 127)
    DRAW_LINE(player_x, player_y-10, player_x, player_y+10, 127)
```

**Compilar desde terminal:**
```bash
# Con el nuevo compilador modular (recomendado)
cd buildtools
cargo run --release --bin vpy_cli -- build ../game.vpy -o game.bin

# O desde el IDE: botón "Run" (compila + carga en emulador)
```

## 📚 Documentación

### Compilador (Buildtools)
- **[buildtools/README.md](buildtools/README.md)** - Arquitectura del pipeline modular
- **[buildtools/STATUS.md](buildtools/STATUS.md)** - Estado actual de cada fase
- **Estado por fase**: Loader ✅, Parser ✅, Unifier ✅, Allocator ✅, Codegen ✅, Assembler ✅

### Lenguaje VPy
- **[docs/COMPILER_STATUS.md](docs/COMPILER_STATUS.md)** - Referencia de sintaxis y builtins
- **[docs/PHASE6_SUMMARY.md](docs/PHASE6_SUMMARY.md)** - Sistema de módulos e imports
- **Ejemplos**: Ver carpeta `examples/` (pang, animations, multi-module)

### Multibank y Assets
- **[docs/MULTIBANK_DEBUG_GUIDE.md](docs/MULTIBANK_DEBUG_GUIDE.md)** - ROMs multibank (hasta 4MB)
- **Assets**: Vectores (.vec), música (.vmus), sonidos (.vsfx), niveles (.vplay)

### IDE
- **Emulador**: JSVecX integrado (puerto JavaScript de VecX)
- **Editor de Vectores**: Herramientas de dibujo para gráficos .vec
- **Editor de Animaciones**: Secuencias de frames con .vec
- **Editor de Niveles (Playground)**: Composición visual de objetos y animaciones
- **Debugging**: Breakpoints, step-by-step, inspección de memoria

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

## 🎮 Características del Lenguaje

### Tipos de Datos
```python
# Variables
x = 10
name = "VECTREX"
colors = [255, 200, 150]

# Constantes (ROM-only)
const ENEMIES = 5
const LEVEL_DATA = [1, 2, 3, 4]
```

### Funciones Builtin
```python
# Gráficos
SET_INTENSITY(brightness)
DRAW_LINE(x0, y0, x1, y1, intensity)
DRAW_VECTOR("sprite_name")
PRINT_TEXT(x, y, "HELLO")

# Input
joy_x = J1_X()              # -1, 0, 1
joy_y = J1_Y()
btn = J1_BUTTON_1()         # 0 o 1

# Audio
PLAY_MUSIC("theme")
PLAY_SFX("explosion", 0)    # channel 0-2
```

### Sistema de Assets
```python
# Los assets se descubren automáticamente:
# - assets/vectors/*.vec
# - assets/music/*.vmus

def loop():
    DRAW_VECTOR("player")      # Usa player.vec
    PLAY_MUSIC("theme")        # Usa theme.vmus
```

### Módulos
```python
# input.vpy
def get_input():
    return J1_X(), J1_Y()

# main.vpy
import input

def loop():el Nuevo Compilador (Buildtools)
```bash
cd buildtools
cargo build --release --bin vpy_cli
```

### Ejecutar el Compilador
```bash
# Compilar archivo VPy
cd buildtools
cargo run --release --bin vpy_cli -- build ../examples/pang/src/main.vpy -o pang.bin

# Ver ayuda
cargo run --release --bin vpy_cli -- --help
```

### Tests del Compilador
```bash
cd buildtools

# Tests por crate
cargo test -p vpy_parser
cargo test -p vpy_unifier
cargo test -p vpy_codegen
# ... etc

# Tests de todo el workspace
cargo test --all
```

### Build del IDE
```bash
cd ide/frontend
npm run build        # Build frontend (React + Vite)

cd ../electron
npm run build        # Build Electron app
```

### Desarrollo del IDE
```bash
# Terminal 1: Frontend dev server
cd ide/frontend
npm run dev          # Vite dev server en puerto 5173

# Terminal 2: Electron
cd ide/electron
npm start            # Electron apuntando a localhost:5173
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

### .vanim - Animaciones
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

### .vplay - Niveles
```json
{
  "namBuildtools (Compilador Modular)
- ✅ **9 fases completadas**: Loader → Parser → Unifier → Allocator → Codegen → Assembler → Linker → Writer → Debug
- ✅ **Ensamblador M6809 nativo**: No requiere lwasm ni herramientas externas
- ✅ **Linker real**: Relocaciones, symbol table, single source of truth
- ✅ **Tree shaking**: Elimina código no usado automáticamente
- ✅ **Multibank**: Soporte para ROMs hasta 4MB (256 banks × 16KB)
- ✅ **Tests comprehensivos**: 100+ tests cubriendo todas las fases

### ✅ IDE y Herramientas
- ✅ **Emulador JSVecX**: Puerto JavaScript de VecX integrado
- ✅ **Editor de Vectores**: Herramientas de dibujo con preview en tiempo real
- ✅ **Editor de Animaciones**: Timeline visual para secuencias
- ✅ **Editor de Niveles (Playground)**: Composición visual de objetos y animaciones
- ✅ **Debugging**: Breakpoints, step execution, memory inspector
- ✅ **Monaco Editor**: Syntax highlighting para VPy
- ✅ **Sistema de proyectos**: .vpyproj con metadata y configuración

### ✅ Lenguaje VPy
- ✅ **Sistema de módulos**: Imports con resolución automática
- ✅ **Structs y arrays**: Tipos compuestos con layout automático
- ✅ **Const arrays**: Datos ROM-only con indexación eficiente
- ✅ **Assets integrados**: Vectores, música, sonidos, niveles
- ✅ **Builtins**: 40+ funciones (gráficos, input, audio, colisiones)

### 🚧 En Desarrollo
- 🚧 **Migración core → buildtools**: Integrar CLI nuevo en IDE
- 🚧 **LSP actualizado**: Usar parser del nuevo compilador
- 🚧 **Optimizaciones**: Dead code elimination, constant propagation

### 📋 Roadmap
- [ ] Generador de sprites desde imágenes PNG
- [ ] Sistema de física 2D (colisiones, gravedad)
- [ ] Herramientas de profiling (CPU, memoria)
- [ ] Export a cartuchos físicos (.vec format)
### 🚧 En Desarrollo
- 🚧 Compilación incremental
- 🚧 Debugger con breakpoints
- 🚧 Optimizaciones del compilador

### 📋 Roadmap
- [ ] LSP mejorado (autocomplete contextual)
- [ ] Generador de sprites desde imágenes
- [ ] Sistema de física 2D
- [ ] Networking para multi-cart

## 🤝 Contribuir

Las contribuciones son bienvenidas:

1. Fork el proyecto
2. Crea una rama feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## 📝 Licencia

Este proyecto está bajo licencia MIT. Ver [LICENSE](LICENSE) para más detalles.

## 🙏 Agradecimientos

- **JSVecX** por raz0red - Puerto JavaScript de VecX usado como emulador
- **VecX** por Valavan Manohararajah - Emulador original de referencia
- **Comunidad Vectrex** por documentación de hardware y BIOS
- **BIOS Vectrex** (liberada públicamente) para desarrollo

## 📞 Soporte

- **Issues**: [GitHub Issues](https://github.com/tuusuario/vectrex-pseudo-python/issues)
- **Documentación**: Carpeta [docs/](docs/)
- **Ejemplos**: Carpeta [examples/](examples/)

---

**Hecho con ❤️ para la comunidad Vectrex**
