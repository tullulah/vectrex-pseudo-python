param(
  [switch]$DevTools,
  [switch]$StrictCSP,    # ahora la relajación es por defecto; usar -StrictCSP para política estricta
  [switch]$NoRustBuild,
  [switch]$NoWasmBuild,  # omite wasm build
  [switch]$Fast,         # omite npm install si ya existe node_modules
  [switch]$NoClear,      # evita limpiar pantalla (preserva logs)
  [switch]$VerboseLsp,   # más logging sobre ruta/estado LSP
  [switch]$Production    # ejecuta en modo producción (sin hot reload)
)
<#
Script simplificado (con dependencias automáticas):
  1. Verifica npm
  2. Instala (si faltan) dependencias en ide/frontend y ide/electron
  3. Lanza: powershell -NoLogo -NoProfile -Command "Set-Location ide/electron; npm run dev"
  4. Si se pasa -DevTools exporta VPY_IDE_DEVTOOLS=1

Compilación Rust: manual (cargo build --workspace) si la quieres antes del LSP.
Uso:
  .\run-ide.ps1            # normal
  .\run-ide.ps1 -DevTools  # con devtools permitidos
#>

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $root

if(-not (Get-Command npm -ErrorAction SilentlyContinue)){
  Write-Host '[ERR ] npm no encontrado en PATH' -ForegroundColor Red
  exit 1
}

if($DevTools){ $env:VPY_IDE_DEVTOOLS = '1' } else { Remove-Item Env:VPY_IDE_DEVTOOLS -ErrorAction SilentlyContinue | Out-Null }
# Relajado por defecto (para permitir React Fast Refresh y estilos inline de desarrollo). Si se pasa -StrictCSP se limpia.
if(-not $StrictCSP){ $env:VPY_IDE_RELAX_CSP = '1' } else { Remove-Item Env:VPY_IDE_RELAX_CSP -ErrorAction SilentlyContinue | Out-Null }
if($NoClear){ $env:VPY_IDE_NO_CLEAR = '1' } else { Remove-Item Env:VPY_IDE_NO_CLEAR -ErrorAction SilentlyContinue | Out-Null }
if($VerboseLsp){ $env:VPY_IDE_VERBOSE_LSP = '1' } else { Remove-Item Env:VPY_IDE_VERBOSE_LSP -ErrorAction SilentlyContinue | Out-Null }

# Instalación condicional (opcionalmente saltada con -Fast)
function Install-NodeModulesIfMissing($dir){
  if(-not (Test-Path (Join-Path $dir 'package.json'))){ return }
  $nm = Join-Path $dir 'node_modules'
  if($Fast -and (Test-Path $nm)){ return }
  if(-not (Test-Path $nm)){
    Write-Host "[INFO] npm install -> $dir" -ForegroundColor Cyan
    Push-Location $dir
    npm install
    if($LASTEXITCODE -ne 0){ Write-Host '[ERR ] npm install falló' -ForegroundColor Red; exit 1 }
    Pop-Location
  }
}

Install-NodeModulesIfMissing (Join-Path $root 'ide/frontend')
Install-NodeModulesIfMissing (Join-Path $root 'ide/electron')

# Build Rust (LSP + core) salvo -NoRustBuild
if(-not $NoRustBuild){
  if(-not (Get-Command cargo -ErrorAction SilentlyContinue)){
    Write-Host '[WARN] cargo no encontrado; se omite build Rust' -ForegroundColor Yellow
  } else {
    Write-Host '[INFO] cargo build (LSP + core only, no emulator)' -ForegroundColor Cyan
    cargo build -p vectrex_lang --bin vpy_lsp
    if($LASTEXITCODE -ne 0){ Write-Host '[ERR ] cargo build falló' -ForegroundColor Red; exit 1 }
  }
}

# Comprobar binario LSP esperado (heursítica) y avisar si falta
$lspExe = Join-Path $root 'target/debug/vpy_lsp.exe'
if(-not (Test-Path $lspExe)){
  Write-Host "[WARN] Binario LSP no encontrado en $lspExe (spawn podría fallar)" -ForegroundColor Yellow
}

Write-Host '[INFO] Lanzando entorno Electron' -ForegroundColor Cyan
if($Production){
  Write-Host '[INFO] Modo producción - sin hot reload' -ForegroundColor Green
  # Asegurar que el frontend esté construido
  Write-Host '[INFO] Construyendo frontend...' -ForegroundColor Cyan
  & powershell -NoLogo -NoProfile -Command "Set-Location ide/frontend; npm run build"
  if($LASTEXITCODE -ne 0){ Write-Host '[ERR ] Frontend build falló' -ForegroundColor Red; exit 1 }
  # Ejecutar en modo producción
  & powershell -NoLogo -NoProfile -Command "Set-Location ide/electron; npm run start"
} else {
  Write-Host '[INFO] Modo desarrollo - con hot reload' -ForegroundColor Yellow
  if($NoClear){
    # Vite normalmente limpia consola; forzamos variable para detectar en plugin (si se implementa) o al menos preservamos scroll buffer
    $env:FORCE_COLOR = '1'
  }
  & powershell -NoLogo -NoProfile -Command "Set-Location ide/electron; if($NoClear){ Write-Host '[INFO] (NoClear) Ejecutando npm run dev'; }; npm run dev"
}
exit $LASTEXITCODE