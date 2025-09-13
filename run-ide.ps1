param(
  [switch]$DevTools,
  [switch]$RelaxCSP
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
if($RelaxCSP){ $env:VPY_IDE_RELAX_CSP = '1' } else { Remove-Item Env:VPY_IDE_RELAX_CSP -ErrorAction SilentlyContinue | Out-Null }

# Instalación condicional
function Install-NodeModulesIfMissing($dir){
  if(-not (Test-Path (Join-Path $dir 'package.json'))){ return }
  $nm = Join-Path $dir 'node_modules'
  if(-not (Test-Path $nm)){
    Write-Host "[INFO] npm install -> $dir" -ForegroundColor Cyan
  Push-Location $dir
    npm install | Out-Null
  Pop-Location
  }
}

Install-NodeModulesIfMissing (Join-Path $root 'ide/frontend')
Install-NodeModulesIfMissing (Join-Path $root 'ide/electron')

Write-Host '[INFO] Lanzando entorno Electron (npm run dev)' -ForegroundColor Cyan
& powershell -NoLogo -NoProfile -Command "Set-Location ide/electron; npm run dev"
exit $LASTEXITCODE

try {
  while($true){
    Start-Sleep -Seconds 2
    if(-not $Persist){
      if($electron.HasExited){
        Write-Info "Electron terminó (ExitCode=$($electron.ExitCode)). Cerrando entorno..."
        break
      }
    }
    # Señal mínima si Vite muere antes que Electron
    if($vite.HasExited){ Write-Warn "Vite terminó (ExitCode=$($vite.ExitCode))" }
  }
} finally {
  & $cleanup
}
