# Setup Completo - vectrex-pseudo-python

**Guía completa para configurar el entorno de desarrollo desde cero**  
*Actualizado: Noviembre 15, 2025*

---

## Tabla de Contenidos

1. [Requisitos del Sistema](#requisitos-del-sistema)
2. [Instalación de Herramientas](#instalación-de-herramientas)
3. [Configuración del Proyecto](#configuración-del-proyecto)
4. [Compilación de Componentes](#compilación-de-componentes)
5. [Ejecución de la IDE](#ejecución-de-la-ide)
6. [Verificación del Setup](#verificación-del-setup)
7. [Troubleshooting](#troubleshooting)
8. [Estructura del Proyecto](#estructura-del-proyecto)

---

## Requisitos del Sistema

### Sistema Operativo
- **Windows 10/11** (PowerShell 5.1+)
- **Linux** (Ubuntu 20.04+ o equivalente)
- **macOS** (10.15+ Catalina o superior)

### Hardware Mínimo
- **RAM**: 8 GB (16 GB recomendado)
- **Disco**: 5 GB espacio libre
- **CPU**: x64 con soporte SSE2

---

## Instalación de Herramientas

### 1. Rust (Obligatorio)

**Windows:**
```powershell
# Descargar instalador desde https://rustup.rs/
# O usar scoop/chocolatey:
scoop install rustup
# O:
choco install rustup
```

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Verificar instalación:**
```bash
rustc --version   # Debe mostrar >= 1.70.0
cargo --version
```

**Configurar toolchain:**
```bash
rustup default stable
rustup update
```

### 2. Node.js (Obligatorio para IDE)

**Versión requerida:** 18.x o superior

**Windows:**
```powershell
# Usando scoop:
scoop install nodejs

# O descarga desde: https://nodejs.org/
```

**Linux:**
```bash
# Ubuntu/Debian:
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Arch:
sudo pacman -S nodejs npm
```

**macOS:**
```bash
brew install node@18
```

**Verificar:**
```bash
node --version    # >= v18.0.0
npm --version
```

### 3. WASM Target (Para emulador web)

```bash
rustup target add wasm32-unknown-unknown
```

### 4. wasm-bindgen-cli (Para emulador web)

```bash
cargo install wasm-bindgen-cli
```

### 5. Git (Si clonando el repositorio)

**Windows:**
```powershell
scoop install git
# O descarga desde: https://git-scm.com/
```

**Linux:**
```bash
sudo apt-get install git  # Ubuntu/Debian
sudo pacman -S git         # Arch
```

**macOS:**
```bash
brew install git
```

---

## Configuración del Proyecto

### 1. Clonar el Repositorio

```bash
git clone https://github.com/tullulah/vectrex-pseudo-python.git
cd vectrex-pseudo-python
```

O si ya tienes el código:
```bash
cd /ruta/al/proyecto/vectrex-pseudo-python
```

### 2. Estructura de Archivos Críticos

Verificar que existen estos directorios:
```
vectrex-pseudo-python/
├── core/                  # Compilador VPy y emulador
│   ├── src/
│   ├── Cargo.toml
├── emulator_v2/          # Emulador refactorizado (WIP)
├── ide/
│   ├── frontend/         # React + Vite
│   │   ├── package.json
│   │   └── src/
│   └── electron/         # Electron shell
│       ├── package.json
│       └── src/
├── runtime/              # Runtime del lenguaje VPy
├── tests/                # Tests del emulador
├── Cargo.toml            # Workspace Rust
└── run-ide.ps1          # Script de inicio IDE
```

### 3. Archivo BIOS (CRÍTICO)

**El emulador requiere la BIOS real de Vectrex.**

**Ubicación esperada:**
```
ide/frontend/dist/bios.bin      # Primaria (8192 bytes)
ide/frontend/src/assets/bios.bin # Alternativa
```

**Obtener BIOS:**
- Buscar "vectrex bios.bin" (archivo liberado, 8KB)
- Hash MD5 esperado: `(verificar con tu archivo)`
- **Tamaños válidos:** 4096 bytes (4KB) o 8192 bytes (8KB)

**Verificar BIOS:**
```bash
# Windows PowerShell:
(Get-Item ide\frontend\dist\bios.bin).Length  # Debe ser 4096 u 8192

# Linux/macOS:
ls -l ide/frontend/dist/bios.bin
```

Si falta, crear directorios:
```bash
mkdir -p ide/frontend/dist
# Copiar tu bios.bin aquí
```

---

## Compilación de Componentes

### 1. Compilar el Workspace Rust (Core + Emulador)

**Build Debug (desarrollo):**
```bash
# Desde raíz del proyecto:
cargo build
```

**Build Release (optimizado):**
```bash
cargo build --release
```

**Compilar solo el compilador VPy:**
```bash
cargo build --bin vectrexc
# O release:
cargo build --bin vectrexc --release
```

**Ubicaciones de binarios:**
- Debug: `target/debug/vectrexc` (o `.exe` en Windows)
- Release: `target/release/vectrexc`

**Verificar compilación:**
```bash
# Windows:
.\target\debug\vectrexc.exe --help

# Linux/macOS:
./target/debug/vectrexc --help
```

Salida esperada:
```
Pseudo-Python multi-target assembler compiler (prototype)

Usage: vectrexc.exe <COMMAND>

Commands:
  build  
  lex    
  ast    
  help   Print this message or the help of the given subcommand(s)
```

### 2. Compilar Frontend (React + Vite)

```bash
cd ide/frontend
npm install     # Instalar dependencias (primera vez)
npm run build   # Build producción
# O para desarrollo:
npm run dev     # Servidor desarrollo (no necesario si usas run-ide.ps1)
cd ../..        # Volver a raíz
```

### 3. Compilar Electron Shell

```bash
cd ide/electron
npm install     # Instalar dependencias (primera vez)
npm run build   # TypeScript → JavaScript
cd ../..
```

### 4. (Opcional) Compilar Emulador WASM

**Solo si necesitas emulador en navegador:**

```bash
# 1. Compilar a WASM:
cargo build --target wasm32-unknown-unknown --release

# 2. Generar bindings:
wasm-bindgen \
  --target web \
  --out-dir ide/frontend/dist-wasm \
  target/wasm32-unknown-unknown/release/vectrex_lang.wasm

# Para Node/Electron:
wasm-bindgen \
  --target nodejs \
  --out-dir ide/electron/dist-wasm \
  target/wasm32-unknown-unknown/release/vectrex_lang.wasm
```

### 5. Compilar Tests (Opcional)

```bash
# Tests del emulador:
cargo test --package vectrex_emulator

# Tests del compilador:
cargo test --package vectrex_lang

# Todos los tests:
cargo test --workspace
```

---

## Ejecución de la IDE

### Método 1: Script PowerShell (Recomendado - Windows)

```powershell
.\run-ide.ps1
```

Este script:
1. Inicia Vite dev server (frontend) en puerto 5173
2. Espera que Vite esté listo
3. Lanza Electron apuntando al dev server
4. Hot reload habilitado para desarrollo

### Método 2: Manual (Todas las plataformas)

**Terminal 1 - Frontend:**
```bash
cd ide/frontend
npm run dev
# Esperar "Local: http://localhost:5173/"
```

**Terminal 2 - Electron:**
```bash
cd ide/electron
npm start
# O si ya compilaste:
npm run electron
```

### Método 3: Build Empaquetado (Producción)

```bash
cd ide/electron
npm run build         # Compilar TypeScript
npm run package       # Empaquetar con electron-builder
# Binarios en: dist/ (según tu SO)
```

---

## Verificación del Setup

### 1. Verificar Compilador VPy

**Crear archivo de prueba:**
```python
# test_setup.vpy
def setup():
    Intensity_a(127)
    Reset0Ref()

def loop():
    Moveto_d(-50, -50)
    Draw_Line_d(50, 50)
```

**Compilar:**
```bash
# Windows:
.\target\debug\vectrexc.exe build test_setup.vpy

# Linux/macOS:
./target/debug/vectrexc build test_setup.vpy
```

**Salida esperada:**
```
=== COMPILATION PIPELINE START ===
...
✓ Phase 5 SUCCESS: Written to .\test_setup.asm
...
=== COMPILATION PIPELINE COMPLETE ===
```

**Compilar con binario:**
```bash
.\target\debug\vectrexc.exe build --bin test_setup.vpy
```

Debe generar:
- `test_setup.asm` (assembly)
- `test_setup.bin` (binario ejecutable, 8192 bytes)
- `test_setup.pdb` (símbolos debug)

### 2. Verificar IDE

1. Ejecutar `.\run-ide.ps1`
2. Debe abrir ventana Electron
3. Panel izquierdo: explorador de archivos
4. Panel central: editor Monaco
5. Panel derecho: emulador Vectrex
6. Abrir `test_setup.vpy`
7. Click botón **"Run"** (▶️)
8. Debe ver línea diagonal en pantalla del emulador

### 3. Verificar Emulador Standalone

```bash
# Test de opcode específico:
cargo test --package vectrex_emulator test_lda_immediate

# Test con BIOS real:
cargo test --package vectrex_emulator test_bios_boot_sequence
```

### 4. Verificar Integración Completa

**Compilar archivo de ejemplo complejo:**
```bash
.\target\debug\vectrexc.exe build --bin examples\demo.vpy
# O si existe rotating_line_correct.vpy:
.\target\debug\vectrexc.exe build --bin rotating_line_correct.vpy
```

Verificar que genera `.bin` sin caer en lwasm fallback:
```
⚠ Native assembler failed: ...
  Falling back to lwasm...
```

Si ves esto, revisa qué instrucciones faltan (ver sección de progreso).

---

## Troubleshooting

### Problema: "cargo: command not found"

**Solución:**
```bash
# Reiniciar terminal después de instalar Rust
# O cargar manualmente:
source $HOME/.cargo/env  # Linux/macOS
```

### Problema: "vectrexc not found"

**Solución:**
```bash
# Recompilar:
cargo build --bin vectrexc

# Verificar ruta:
ls -la target/debug/vectrexc*
```

### Problema: "Cannot find BIOS"

**Solución:**
```bash
# Verificar ubicación:
ls ide/frontend/dist/bios.bin

# Si falta, copiar:
cp /ruta/a/tu/bios.bin ide/frontend/dist/bios.bin

# Verificar tamaño:
# Windows:
(Get-Item ide\frontend\dist\bios.bin).Length
# Linux/macOS:
wc -c ide/frontend/dist/bios.bin  # Debe ser 4096 u 8192
```

### Problema: "Native assembler failed"

**Síntomas:**
```
⚠ Native assembler failed: Error en línea X: Instrucción no soportada: XXX
  Falling back to lwasm...
```

**Solución:**
- Verifica qué instrucción falta (ej: `CMPX`, `LEAX`, etc.)
- Revisa COMPILER_STATUS.md para estado de implementación
- Usa `--verbose` para más detalles:
```bash
.\target\debug\vectrexc.exe build --bin archivo.vpy --verbose
```

### Problema: "Port 5173 already in use"

**Solución:**
```bash
# Matar proceso en puerto 5173:
# Windows:
netstat -ano | findstr :5173
taskkill /PID <PID> /F

# Linux/macOS:
lsof -ti:5173 | xargs kill -9

# O cambiar puerto en ide/frontend/vite.config.ts
```

### Problema: Electron no abre ventana

**Solución:**
```bash
# Verificar logs:
cd ide/electron
npm start 2>&1 | tee electron.log

# Verificar dependencias:
npm install

# Reinstalar Electron:
npm uninstall electron
npm install electron --save-dev
```

### Problema: Hot Reload no funciona

**Solución:**
```bash
# Verificar que Vite está corriendo:
curl http://localhost:5173

# Reiniciar ambos procesos:
# Ctrl+C en ambas terminales, luego:
.\run-ide.ps1
```

### Problema: Tests fallan

**Solución:**
```bash
# Compilar primero:
cargo build --workspace

# Ejecutar test específico:
cargo test --package vectrex_emulator <nombre_test> -- --nocapture

# Ver output completo:
cargo test -- --show-output
```

---

## Estructura del Proyecto

### Crates Rust (Cargo Workspace)

```
Cargo.toml (workspace root)
├── core/                  # vectrex_lang crate
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── lexer/        # Tokenización VPy
│   │   ├── parser/       # AST parsing
│   │   ├── backend/      # Codegen M6809
│   │   │   ├── asm_to_binary.rs  # Ensamblador nativo
│   │   │   ├── m6809_binary_emitter.rs  # Emisión opcodes
│   │   │   └── m6809.rs  # Backend principal
│   │   └── wasm_api.rs   # Exportaciones WASM
│   └── tests/            # Tests unitarios compilador
├── emulator_v2/          # Refactor emulador (WIP)
├── vectrex_emulator/     # Emulador 6809 actual
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── cpu6809.rs    # CPU core
│   │   ├── memory_map.rs # Bus de memoria
│   │   ├── via.rs        # VIA 6522 (parcial)
│   │   └── wasm_api.rs   # Exports WASM
│   └── tests/            # Tests opcodes
└── runtime/              # Runtime VPy (futuro)
```

### Frontend (Node.js)

```
ide/
├── frontend/             # React + Vite
│   ├── package.json
│   ├── vite.config.ts
│   ├── src/
│   │   ├── main.tsx      # Entry point React
│   │   ├── App.tsx       # Componente principal
│   │   ├── components/   # Componentes UI
│   │   ├── services/     # Lógica negocio
│   │   └── assets/       # Recursos estáticos
│   └── dist/             # Build output
│       └── bios.bin      # ⚠️ BIOS aquí
└── electron/             # Electron shell
    ├── package.json
    ├── src/
    │   ├── main.ts       # Proceso principal
    │   ├── preload.ts    # Preload script
    │   └── ipc/          # IPC handlers
    └── dist/             # Binarios empaquetados
```

### Archivos de Configuración

```
.
├── Cargo.toml            # Workspace Rust
├── rust-toolchain.toml   # Versión Rust fijada (si existe)
├── .gitignore
├── README.md             # Documentación general
├── SETUP.md              # Este archivo
├── COMPILER_STATUS.md    # Estado del compilador
├── SUPER_SUMMARY.md      # Documentación técnica detallada
└── run-ide.ps1          # Script inicio IDE (Windows)
```

---

## Comandos Rápidos de Referencia

### Compilación
```bash
# Compilador VPy (debug):
cargo build --bin vectrexc

# Compilador VPy (release):
cargo build --bin vectrexc --release

# Todo el workspace:
cargo build --workspace

# Solo emulador:
cargo build --package vectrex_emulator
```

### Tests
```bash
# Todos los tests:
cargo test --workspace

# Tests de emulador:
cargo test --package vectrex_emulator

# Test específico:
cargo test test_adda_immediate

# Ver output:
cargo test -- --nocapture
```

### IDE
```bash
# Inicio rápido (Windows):
.\run-ide.ps1

# Manual:
cd ide/frontend && npm run dev
# En otra terminal:
cd ide/electron && npm start
```

### Compilar Programa VPy
```bash
# Solo ASM:
.\target\debug\vectrexc.exe build programa.vpy

# ASM + Binario:
.\target\debug\vectrexc.exe build --bin programa.vpy

# Ver AST:
.\target\debug\vectrexc.exe ast programa.vpy

# Ver tokens:
.\target\debug\vectrexc.exe lex programa.vpy
```

---

## Próximos Pasos

Una vez verificado el setup:

1. **Explorar ejemplos:** `examples/*.vpy`
2. **Leer documentación técnica:** `SUPER_SUMMARY.md`
3. **Revisar estado compilador:** `COMPILER_STATUS.md`
4. **Estudiar arquitectura emulador:** `docs/TIMING.md`, `docs/VECTOR_MODEL.md`
5. **Contribuir:** Ver issues en GitHub

---

## Soporte

**Issues:** https://github.com/tullulah/vectrex-pseudo-python/issues  
**Documentación adicional:** `docs/` en el repositorio  

**Versión de este documento:** 1.0 (Nov 15, 2025)
