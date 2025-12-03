#!/bin/bash
set -e

# Script de ejecución IDE para macOS/Linux
# Traducción de run-ide.ps1 compatible con Bash/Zsh

# Parse de argumentos
NO_DEV_TOOLS=false
STRICT_CSP=false
NO_RUST_BUILD=false
NO_WASM_BUILD=false
FAST=false
NO_CLEAR=false
VERBOSE_LSP=false
PRODUCTION=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --no-devtools)
      NO_DEV_TOOLS=true
      shift
      ;;
    --strict-csp)
      STRICT_CSP=true
      shift
      ;;
    --no-rust-build)
      NO_RUST_BUILD=true
      shift
      ;;
    --no-wasm-build)
      NO_WASM_BUILD=true
      shift
      ;;
    --fast)
      FAST=true
      shift
      ;;
    --no-clear)
      NO_CLEAR=true
      shift
      ;;
    --verbose-lsp)
      VERBOSE_LSP=true
      shift
      ;;
    --production)
      PRODUCTION=true
      shift
      ;;
    --help)
      echo "Uso: ./run-ide.sh [opciones]"
      echo ""
      echo "Opciones:"
      echo "  --no-devtools      Deshabilita DevTools (habilitados por defecto)"
      echo "  --strict-csp       Política CSP estricta (relajada por defecto)"
      echo "  --no-rust-build    Omite compilación Rust"
      echo "  --no-wasm-build    Omite compilación WASM"
      echo "  --fast             Omite npm install si ya existe node_modules"
      echo "  --no-clear         Preserva logs (no limpia pantalla)"
      echo "  --verbose-lsp      Más logging sobre ruta/estado LSP"
      echo "  --production       Ejecuta en modo producción (sin hot reload)"
      echo "  --help             Muestra esta ayuda"
      exit 0
      ;;
    *)
      echo "Opción desconocida: $1"
      echo "Usa --help para ver opciones disponibles"
      exit 1
      ;;
  esac
done

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

# Verificar npm
if ! command -v npm &> /dev/null; then
  echo '[ERR ] npm no encontrado en PATH'
  exit 1
fi

# Configurar variables de entorno
if [ "$NO_DEV_TOOLS" = false ]; then
  export VPY_IDE_DEVTOOLS=1
else
  unset VPY_IDE_DEVTOOLS
fi

if [ "$STRICT_CSP" = false ]; then
  export VPY_IDE_RELAX_CSP=1
else
  unset VPY_IDE_RELAX_CSP
fi

if [ "$NO_CLEAR" = true ]; then
  export VPY_IDE_NO_CLEAR=1
else
  unset VPY_IDE_NO_CLEAR
fi

if [ "$VERBOSE_LSP" = true ]; then
  export VPY_IDE_VERBOSE_LSP=1
else
  unset VPY_IDE_VERBOSE_LSP
fi

# Función para instalar node_modules si es necesario
install_node_modules_if_missing() {
  local dir="$1"
  
  if [ ! -f "$dir/package.json" ]; then
    return
  fi
  
  if [ "$FAST" = true ] && [ -d "$dir/node_modules" ]; then
    return
  fi
  
  if [ ! -d "$dir/node_modules" ]; then
    echo "[INFO] npm install -> $dir"
    (cd "$dir" && npm install)
    if [ $? -ne 0 ]; then
      echo '[ERR ] npm install falló'
      exit 1
    fi
  fi
}

# Instalar dependencias
install_node_modules_if_missing "$ROOT/ide/frontend"
install_node_modules_if_missing "$ROOT/ide/electron"

# Build Rust (LSP + core) salvo --no-rust-build
if [ "$NO_RUST_BUILD" = false ]; then
  if ! command -v cargo &> /dev/null; then
    echo '[WARN] cargo no encontrado; se omite build Rust'
  else
    echo '[INFO] cargo build (LSP + compiler + core only, no emulator)'
    cargo build -p vectrex_lang --bin vpy_lsp --bin vectrexc
    if [ $? -ne 0 ]; then
      echo '[ERR ] cargo build falló'
      exit 1
    fi
  fi
fi

# Comprobar binario LSP esperado (heurística) y avisar si falta
LSP_BIN="$ROOT/target/debug/vpy_lsp"
if [ ! -f "$LSP_BIN" ]; then
  echo "[WARN] Binario LSP no encontrado en $LSP_BIN (spawn podría fallar)"
fi

echo '[INFO] Lanzando entorno Electron'

if [ "$PRODUCTION" = true ]; then
  echo '[INFO] Modo producción - sin hot reload'
  # Asegurar que el frontend esté construido
  echo '[INFO] Construyendo frontend...'
  (cd "$ROOT/ide/frontend" && npm run build)
  if [ $? -ne 0 ]; then
    echo '[ERR ] Frontend build falló'
    exit 1
  fi
  # Ejecutar en modo producción
  (cd "$ROOT/ide/electron" && npm run start)
else
  echo '[INFO] Modo desarrollo - con hot reload'
  if [ "$NO_CLEAR" = true ]; then
    export FORCE_COLOR=1
  fi
  (cd "$ROOT/ide/electron" && npm run dev)
fi

exit $?
