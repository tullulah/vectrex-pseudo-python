# Checklist de Migración a Nueva Máquina

**Guía rápida para transferir el proyecto vectrex-pseudo-python a un nuevo equipo**

---

## 📋 Pre-Migración (Máquina Antigua)

### 1. Verificar Estado del Repositorio
```bash
cd /ruta/al/proyecto
git status
git log --oneline -10  # Verificar últimos commits
```

### 2. Asegurar Commits Pendientes
```bash
# Si hay cambios sin commit:
git add .
git commit -m "Pre-migration checkpoint"
git push origin master  # ⚠️ RAMA ES master, NO main
```

### 3. Verificar Archivos Versionados

**✅ TODO está en Git - No necesitas backup manual:**
```
✅ bios.bin                          (YA en git: ide/frontend/src/assets/bios.bin)
✅ Código fuente                     (todo en git)
✅ Configuraciones del proyecto      (package.json, Cargo.toml, etc.)
```

**❌ NO copiar (se regeneran automáticamente):**
```
⚠️ target/                           (builds de Rust - recompilar)
⚠️ ide/frontend/node_modules/        (dependencias npm - reinstalar)
⚠️ ide/electron/node_modules/        (dependencias npm - reinstalar)
⚠️ ide/frontend/dist/                (build frontend - regenerar)
⚠️ *.bin, *.asm generados            (outputs del compilador)
```

**El proyecto es 100% autocontenido - solo necesitas git clone.**

### 4. Documentar Configuración Personal (Opcional)
```bash
# Si tienes configuraciones personales:
code .vscode/settings.json  # Exportar si existe
code ide/frontend/.env      # Variables de entorno personalizadas
```

---

## 🔄 Transferencia

### Opción A: Clonar desde GitHub (Recomendado)
```bash
# En máquina nueva:
git clone https://github.com/tullulah/vectrex-pseudo-python.git
cd vectrex-pseudo-python
```

### Opción B: Copiar Directorio Completo
```bash
# Comprimir en máquina antigua:
tar -czf vectrex-project.tar.gz vectrex-pseudo-python/

# O en Windows:
Compress-Archive -Path vectrex-pseudo-python -DestinationPath vectrex-project.zip

# Transferir archivo .tar.gz o .zip a nueva máquina
# Descomprimir en nueva máquina
```

---

## 🛠️ Setup en Nueva Máquina

### 1. Instalar Herramientas Base
```bash
# Ver SETUP.md sección "Instalación de Herramientas" para detalles completos

# Resumen rápido:
# 1. Rust (https://rustup.rs/)
# 2. Node.js 18+ (https://nodejs.org/)
# 3. Git (si no está instalado)
```

**Verificar instalaciones:**
```bash
rustc --version  # >= 1.70.0
node --version   # >= 18.0.0
npm --version
git --version
```

### 2. Configurar Rust
```bash
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown
```

### 3. Verificar BIOS (Ya está en Git)
```bash
# ✅ BIOS ya está versionado en git - no necesitas restaurar nada
# Verificar que existe y tiene el tamaño correcto:

# Windows:
(Get-Item ide\frontend\src\assets\bios.bin).Length  # Debe ser 8192

# Linux/macOS:
ls -lh ide/frontend/src/assets/bios.bin  # Debe mostrar 8.0K
```

**Si falta el archivo:** El build del frontend lo copia automáticamente a `dist/`.

### 4. Compilar Proyecto
```bash
# Desde raíz del proyecto:

# 1. Compilar Rust (esto tarda 5-10 minutos la primera vez)
cargo build --workspace
cargo build --bin vectrexc --release  # Opcional: versión optimizada

# 2. Frontend
cd ide/frontend
npm install      # Descargar dependencias (2-5 minutos)
npm run build    # Build producción
cd ../..

# 3. Electron
cd ide/electron
npm install      # Descargar dependencias (1-2 minutos)
npm run build    # Compilar TypeScript
cd ../..
```

### 5. Verificar Compilación
```bash
# Test rápido del compilador:
# Windows:
.\target\debug\vectrexc.exe --help

# Linux/macOS:
./target/debug/vectrexc --help

# Debe mostrar:
# Pseudo-Python multi-target assembler compiler (prototype)
# Usage: vectrexc <COMMAND>
# ...
```

### 6. Ejecutar Tests
```bash
# Tests del emulador:
cargo test --package vectrex_emulator

# Tests del compilador:
cargo test --package vectrex_lang

# Todos (puede tardar varios minutos):
cargo test --workspace
```

### 7. Iniciar IDE
```bash
# Windows:
.\run-ide.ps1

# Linux/macOS (crear script equivalente o manual):
# Terminal 1:
cd ide/frontend && npm run dev

# Terminal 2:
cd ide/electron && npm start
```

---

## ✅ Verificación Post-Migración

### Checklist de Funcionalidad

- [ ] **Compilador funciona**
  ```bash
  .\target\debug\vectrexc.exe build test_simple.vpy
  # Debe generar test_simple.asm sin errores
  ```

- [ ] **Binario se genera**
  ```bash
  .\target\debug\vectrexc.exe build --bin test_simple.vpy
  # Debe generar test_simple.bin (8192 bytes)
  ```

- [ ] **BIOS carga correctamente**
  - Verificar que bios.bin existe en ide/frontend/dist/
  - Tamaño exacto: 8192 bytes

- [ ] **IDE abre**
  - Ventana Electron se abre
  - Panel de archivos visible (izquierda)
  - Editor Monaco funcional (centro)
  - Emulador visible (derecha)

- [ ] **Emulador funciona**
  - Abrir test_simple.vpy en IDE
  - Click botón "Run" (▶️)
  - Debe ver output en emulador (líneas/vectores)

- [ ] **Tests pasan**
  ```bash
  cargo test --package vectrex_emulator -- --test-threads=1
  # Debe mostrar: test result: ok. XXX passed
  ```

### Archivos que DEBEN Existir

```
vectrex-pseudo-python/
├── Cargo.toml                           ✅ Workspace Rust
├── SETUP.md                             ✅ Guía de setup
├── INDEX.md                             ✅ Índice de docs
├── COMPILER_STATUS.md                   ✅ Estado compilador
├── CHANGELOG.md                         ✅ Historial
├── README.md                            ✅ Intro
├── ide/
│   ├── frontend/
│   │   ├── dist/bios.bin               ⚠️ CRÍTICO (8192 bytes)
│   │   ├── package.json                ✅
│   │   └── node_modules/               ✅ (después de npm install)
│   └── electron/
│       ├── package.json                ✅
│       └── node_modules/               ✅ (después de npm install)
├── core/
│   └── src/                            ✅
└── target/
    └── debug/vectrexc(.exe)            ✅ (después de cargo build)
```

---

## 🐛 Troubleshooting Migración

### Error: "Cannot find BIOS"
**Causa:** bios.bin no está en ubicación correcta  
**Solución:**
```bash
# Verificar:
ls -la ide/frontend/dist/bios.bin
# Si falta, copiar desde backup o descargar de nuevo
```

### Error: "cargo: command not found"
**Causa:** Rust no instalado o PATH no configurado  
**Solución:**
```bash
# Reinstalar Rust:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Reiniciar terminal
source $HOME/.cargo/env  # Linux/macOS
```

### Error: "node: command not found"
**Causa:** Node.js no instalado  
**Solución:**
```bash
# Descargar e instalar desde https://nodejs.org/
# Reiniciar terminal
```

### Error: Compilación Rust falla
**Causa:** Versión de Rust desactualizada o dependencias faltantes  
**Solución:**
```bash
rustup update
cargo clean
cargo build --workspace
```

### Error: npm install falla
**Causa:** Cache corrupto o permisos  
**Solución:**
```bash
npm cache clean --force
rm -rf node_modules package-lock.json
npm install
```

### Error: IDE no abre
**Causa:** Puerto 5173 ocupado o frontend no corriendo  
**Solución:**
```bash
# Verificar puerto:
# Windows:
netstat -ano | findstr :5173
# Linux/macOS:
lsof -i :5173

# Matar proceso si existe, luego:
cd ide/frontend && npm run dev
```

### Error: Tests fallan en nueva máquina
**Causa:** Diferencias de timing o BIOS incorrecta  
**Solución:**
```bash
# Verificar BIOS:
# Windows:
(Get-Item ide\frontend\dist\bios.bin).Length  # DEBE ser 8192

# Ejecutar tests individuales:
cargo test --package vectrex_emulator test_lda_immediate -- --nocapture
```

---

## 📊 Diferencias entre Máquinas

### Cosas que PUEDEN Diferir (OK)
- Rutas absolutas de archivos
- Configuración de IDE (VSCode settings)
- Tiempo de compilación (depende de CPU)
- Permisos de archivos (Linux vs Windows)

### Cosas que DEBEN Ser Idénticas
- ✅ bios.bin (8192 bytes, mismo MD5)
- ✅ Versión de Rust (1.70+)
- ✅ Versión de Node (18+)
- ✅ Código fuente (mismo commit git)
- ✅ Binarios generados por vectrexc (mismo .bin para mismo .vpy)

---

## 🔐 Seguridad y Backup

### Recomendaciones
1. **Commit frecuente** antes de migrar
2. **Push a GitHub** para tener backup remoto
3. **Verificar .gitignore** antes de copiar archivos
4. **NO versionar**:
   - `target/` (binarios Rust - recompilar)
   - `node_modules/` (dependencias Node - reinstalar)
   - `*.bin` generados por compilador (regenerar)
5. **SÍ versionar o backup manual**:
   - `bios.bin` (CRÍTICO - difícil de obtener)

### Backup Command (Seguro)
```bash
# Crear backup solo de archivos esenciales:
tar -czf vectrex-backup-$(date +%Y%m%d).tar.gz \
  --exclude=target \
  --exclude=node_modules \
  --exclude=dist \
  --exclude=.git \
  vectrex-pseudo-python/

# Verificar contenido:
tar -tzf vectrex-backup-*.tar.gz | less
```

---

## 📚 Referencias

- **Setup completo**: [SETUP.md](SETUP.md)
- **Índice de docs**: [INDEX.md](INDEX.md)
- **Estado compilador**: [COMPILER_STATUS.md](COMPILER_STATUS.md)
- **Historial cambios**: [CHANGELOG.md](CHANGELOG.md)

---

## 🎯 Resumen Express (TL;DR)

```bash
# En máquina nueva:

# 1. Instalar herramientas base
# - Rust (rustup.rs)
# - Node.js 18+ (nodejs.org)

# 2. Clonar repo (incluye bios.bin automáticamente)
git clone https://github.com/tullulah/vectrex-pseudo-python.git
cd vectrex-pseudo-python

# 3. Compilar todo
cargo build --workspace
cd ide/frontend && npm install && cd ../..
cd ide/electron && npm install && cd ../..

# 4. Verificar
./target/debug/vectrexc --help  # Debe mostrar ayuda
cargo test --workspace            # Tests deben pasar

# 5. Ejecutar IDE
./run-ide.ps1  # Windows
# O manual: cd ide/frontend && npm run dev, luego cd ../electron && npm start

# ✅ LISTO!
```

---

**Versión:** 2.0  
**Fecha:** Noviembre 15, 2025  
**Tiempo estimado de migración:** 20-40 minutos (ya no necesitas backup manual)
