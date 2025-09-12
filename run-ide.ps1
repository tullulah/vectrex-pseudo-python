<#!
.SYNOPSIS
  Construye y lanza el entorno de desarrollo (LSP + Tauri + frontend React) para la IDE VPy.
.DESCRIPTION
  1. Verifica dependencias básicas (cargo, npm)
  2. Compila todos los crates (workspace)
  3. Inicia el servidor LSP (vpy_lsp.exe) en una ventana (job) separada
  4. Inicia el frontend React (Vite) en otra tarea
  5. Lanza la app Tauri (vpy_ide) al final
  El script muestra un panel resumen con pids y soporta limpieza con Ctrl+C.
.PARAMETER NoBuild
  Omite la compilación del workspace (usa binarios ya construidos)
.PARAMETER NoLsp
  No inicia el proceso LSP manual (para usar el spawn interno de Tauri)
.PARAMETER Release
  Compila en modo release (cargo build --release)
.EXAMPLE
  ./run-ide.ps1
.EXAMPLE
  ./run-ide.ps1 -NoBuild -NoLsp
#>
param(
  [switch]$NoBuild,
  [switch]$NoLsp,
  [switch]$Release,
  [switch]$UseTauriDev
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $MyInvocation.MyCommand.Path
Push-Location $root

function Write-Info($msg){ Write-Host "[INFO] $msg" -ForegroundColor Cyan }
function Write-Warn($msg){ Write-Host "[WARN] $msg" -ForegroundColor Yellow }
function Write-Err($msg){ Write-Host "[ERR ] $msg" -ForegroundColor Red }

# 1. Verificar herramientas
$tools = @('cargo','npm')
foreach($t in $tools){
  if(-not (Get-Command $t -ErrorAction SilentlyContinue)) { Write-Err "Herramienta requerida no encontrada: $t"; exit 1 }
}

# 2. Compilar (si procede)
if(-not $NoBuild){
  Write-Info "Compilando workspace (modo: $(if($Release){'release'}else{'dev'}))..."
  $buildCmd = 'cargo build --workspace'
  if($Release){ $buildCmd += ' --release' }
  Write-Info $buildCmd
  & powershell -NoLogo -NoProfile -Command $buildCmd
  if($LASTEXITCODE -ne 0){ Write-Err "Fallo compilación cargo"; exit 1 }
  Write-Info "Compilación cargo ok"
  Write-Info "Instalando dependencias frontend (si faltan)"
  if(-not (Test-Path "$root/ide/frontend/node_modules")){
    & npm install --prefix ide/frontend
    if($LASTEXITCODE -ne 0){ Write-Err "npm install falló"; exit 1 }
  }
}

# 3. Iniciar LSP (opcional) manual
$lspJob = $null
if(-not $NoLsp){
  # PowerShell 5.1 no soporta el operador ternario estilo C, usamos if/else
  $buildProfile = if($Release){ 'release' } else { 'debug' }
  $lspExe = Join-Path $root "target/$buildProfile/vpy_lsp.exe"
  if(-not (Test-Path $lspExe)){
    Write-Warn "No se encontró $lspExe; el spawn interno de Tauri fallará también"
  } else {
    Write-Info "Lanzando LSP manual: $lspExe"
    $lspJob = Start-Job -ScriptBlock { param($exe,$wd) Set-Location $wd; & $exe } -ArgumentList $lspExe,$root
    Start-Sleep -Milliseconds 300
    Write-Info "LSP JobId=$($lspJob.Id) State=$($lspJob.State)"
  }
}

# 4. Iniciar frontend (Vite)
Write-Info "Lanzando frontend (Vite)"
$viteJob = Start-Job -ScriptBlock { param($rootPath) & npm run dev --prefix "$rootPath/ide/frontend" } -ArgumentList $root
Start-Sleep -Milliseconds 500
Write-Info "Vite JobId=$($viteJob.Id) State=$($viteJob.State)"

# 5. Lanzar Tauri (usa spawn interno del LSP si no se lanzó manual)
# Ejecutar desde la carpeta src-tauri para que tauri.conf.json y contexto se resuelvan correctamente
$tauriPath = Join-Path $root "ide/ide-app/src-tauri"
if(-not (Test-Path $tauriPath)) { Write-Err "Ruta Tauri no encontrada: $tauriPath"; exit 1 }

if($UseTauriDev){
  Write-Info "Lanzando app Tauri (cargo tauri dev) en $tauriPath"
  $tauriJob = Start-Job -ScriptBlock { param($p) Set-Location $p; cargo tauri dev } -ArgumentList $tauriPath
} else {
  Write-Info "Lanzando app Tauri (cargo run --manifest-path src-tauri/Cargo.toml) desde ide/ide-app"
  $appWorkspaceRoot = Join-Path $root "ide/ide-app"
  $tauriJob = Start-Job -ScriptBlock { param($p) Set-Location $p; cargo run --manifest-path src-tauri/Cargo.toml } -ArgumentList $appWorkspaceRoot
}
Start-Sleep -Milliseconds 1200
Write-Info "Tauri JobId=$($tauriJob.Id) State=$($tauriJob.State)"

Write-Host "`n================ RESUMEN ================" -ForegroundColor Green
if($lspJob){
  Write-Host "LSP   : JobId=$($lspJob.Id) ($($lspJob.State))"
} else {
  Write-Host "LSP   : (no lanzado)"
}
Write-Host "Frontend (Vite): JobId=$($viteJob.Id) ($($viteJob.State))"
Write-Host "Tauri : JobId=$($tauriJob.Id) ($($tauriJob.State))"
Write-Host "=========================================`n" -ForegroundColor Green
Write-Host "Ctrl+C para terminar todos los jobs..." -ForegroundColor Yellow

# Limpieza al Ctrl+C
$cleanup = {
  Write-Warn "Deteniendo jobs..."
  foreach($j in @($lspJob,$viteJob,$tauriJob)){
    if($j){
      try { Stop-Job $j -ErrorAction SilentlyContinue } catch {}
      try { Receive-Job $j -ErrorAction SilentlyContinue | Out-Null } catch {}
      try { Remove-Job $j -ErrorAction SilentlyContinue } catch {}
    }
  }
  Write-Info "Jobs terminados"
  Pop-Location | Out-Null
  exit 0
}

# Registrar manejador Ctrl+C
$null = Register-EngineEvent PowerShell.Exiting -Action { & $cleanup } | Out-Null
try {
  while($true){
    Start-Sleep -Seconds 3
    # Monitor rápido de estados
    foreach($j in @($lspJob,$viteJob,$tauriJob)){
      if($j){
        if($j.State -eq 'Failed') { Write-Warn "Job $($j.Id) falló" }
        elseif($j.State -eq 'Completed' -and -not $j.HasMoreData){
          # If Tauri completed early, surface its output once
          if($j -eq $tauriJob){
            Write-Warn "Tauri terminó antes de lo esperado. Mostrando salida:" 
            try { Receive-Job $j -Keep -ErrorAction SilentlyContinue | Write-Host } catch {}
          }
        }
      }
    }
  }
} finally {
  & $cleanup
}
