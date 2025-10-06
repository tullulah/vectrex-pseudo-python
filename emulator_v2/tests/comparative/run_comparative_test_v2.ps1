# run_comparative_test.ps1
# Automatiza: ensamblado → Vectrexy (SIEMPRE) → Rust → comparación 3-way

param(
    [Parameter(Mandatory=$true)]
    [string]$TestName,
    
    [int]$Cycles = 100,
    
    [switch]$SkipBuild,  # Skip recompilation de runners
    
    [switch]$BiosOnly    # Execute BIOS from reset vector instead of test.bin at 0xC800
)

$ErrorActionPreference = "Stop"
$ComparativeRoot = $PSScriptRoot
$TestDir = Join-Path $ComparativeRoot "test_cases\$TestName"
$VectrexyRunner = Join-Path $ComparativeRoot "vectrexy_runner\build\Release\vectrexy_runner.exe"
$RustRunner = Join-Path $ComparativeRoot "rust_runner\target\release\rust_runner.exe"

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "  COMPARATIVE TEST: $TestName" -ForegroundColor Cyan
Write-Host "========================================`n" -ForegroundColor Cyan

# 1. Verificar que existe el directorio de test
if (-not (Test-Path $TestDir)) {
    Write-Host "ERROR: Test directory not found: $TestDir" -ForegroundColor Red
    exit 1
}

# 2. Ensamblar con lwasm
Write-Host "[1/5] Assembling test..." -ForegroundColor Yellow
$asmFile = Join-Path $TestDir "test.asm"
$binFile = Join-Path $TestDir "test.bin"
$lwasm = "C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\tools\lwasm.exe"

if (-not (Test-Path $asmFile)) {
    Write-Host "  ERROR: Assembly file not found: $asmFile" -ForegroundColor Red
    exit 1
}

& $lwasm -9 --raw -o $binFile $asmFile 2>&1 | Out-Null
if ($LASTEXITCODE -ne 0) {
    Write-Host "  ERROR: Assembly failed" -ForegroundColor Red
    exit 1
}

$binSize = (Get-Item $binFile).Length
Write-Host "  ✅ Assembled: $binSize bytes → $binFile" -ForegroundColor Green

# 3. Compilar runners si es necesario
if (-not $SkipBuild) {
    Write-Host "`n[2/5] Building runners..." -ForegroundColor Yellow
    
    # Rust runner
    if (-not (Test-Path $RustRunner)) {
        Write-Host "  Building Rust runner..." -ForegroundColor Gray
        Push-Location (Join-Path $ComparativeRoot "rust_runner")
        cargo build --release 2>&1 | Select-String -Pattern "Compiling|Finished" | Out-Host
        Pop-Location
    } else {
        Write-Host "  ✅ Rust runner exists (use without -SkipBuild to rebuild)" -ForegroundColor Green
    }
    
    # Vectrexy runner  
    if (-not (Test-Path $VectrexyRunner)) {
        Write-Host "  Building Vectrexy runner..." -ForegroundColor Gray
        Push-Location (Join-Path $ComparativeRoot "vectrexy_runner")
        cmake --build build --config Release 2>&1 | Select-String -Pattern "Building|Linking" | Out-Host
        Pop-Location
    } else {
        Write-Host "  ✅ Vectrexy runner exists" -ForegroundColor Green
    }
} else {
    Write-Host "`n[2/5] Skipping build (using existing binaries)..." -ForegroundColor Yellow
}

# 4. SIEMPRE ejecutar Vectrexy para generar reference output
Write-Host "`n[3/5] Running Vectrexy (generating reference output)..." -ForegroundColor Yellow
$vectrexyOutput = Join-Path $TestDir "vectrexy_output.json"
$vectrexyDebug = Join-Path $TestDir "vectrexy_debug.log"

# Build argument list
$vectrexyArgs = @($binFile, $Cycles)
if ($BiosOnly) {
    $vectrexyArgs += "--bios-only"
    Write-Host "  Mode: BIOS-only (executing from reset vector 0xF000)" -ForegroundColor Cyan
}

$process = Start-Process -FilePath $VectrexyRunner `
                         -ArgumentList $vectrexyArgs `
                         -NoNewWindow `
                         -Wait `
                         -PassThru `
                         -RedirectStandardOutput $vectrexyOutput `
                         -RedirectStandardError $vectrexyDebug

if ($process.ExitCode -ne 0) {
    Write-Host "  WARNING: Vectrexy exited with code $($process.ExitCode)" -ForegroundColor Yellow
    Write-Host "  Check debug log: $vectrexyDebug" -ForegroundColor Gray
}

# Verificar JSON válido
try {
    $vecJson = Get-Content $vectrexyOutput -Raw | ConvertFrom-Json
    Write-Host "  ✅ Vectrexy output: $vectrexyOutput" -ForegroundColor Green
    Write-Host "     CPU.PC=0x$($vecJson.cpu.pc.ToString('X4')), Cycles=$($vecJson.cycles)" -ForegroundColor Cyan
} catch {
    Write-Host "  ERROR: Invalid JSON from Vectrexy: $_" -ForegroundColor Red
    exit 1
}

# 5. Ejecutar Rust
Write-Host "`n[4/5] Running Rust emulator..." -ForegroundColor Yellow
$rustOutput = Join-Path $TestDir "rust_output.json"

# Build argument list
$rustArgs = @($binFile, $Cycles)
if ($BiosOnly) {
    $rustArgs += "--bios-only"
    Write-Host "  Mode: BIOS-only (executing from reset vector 0xF000)" -ForegroundColor Cyan
}

& $RustRunner $rustArgs | Out-File -Encoding UTF8 $rustOutput

if ($LASTEXITCODE -ne 0) {
    Write-Host "  ERROR: Rust runner failed with exit code $LASTEXITCODE" -ForegroundColor Red
    exit 1
}

# Verificar JSON válido
try {
    $rustJson = Get-Content $rustOutput -Raw | ConvertFrom-Json
    Write-Host "  ✅ Rust output: $rustOutput" -ForegroundColor Green
    Write-Host "     CPU.PC=0x$($rustJson.cpu.pc.ToString('X4')), Cycles=$($rustJson.cycles)" -ForegroundColor Cyan
} catch {
    Write-Host "  ERROR: Invalid JSON from Rust: $_" -ForegroundColor Red
    exit 1
}

# 6. Comparar (3-way: vectrexy vs vectrexy vs rust)
Write-Host "`n[5/5] Comparing outputs..." -ForegroundColor Yellow
Write-Host "  Reference: $vectrexyOutput (Vectrexy C++)" -ForegroundColor Gray
Write-Host "  Test:      $rustOutput (Rust port)" -ForegroundColor Gray

python (Join-Path $ComparativeRoot "compare.py") $vectrexyOutput $vectrexyOutput $rustOutput

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n✅ ✅ ✅  TEST PASSED: $TestName  ✅ ✅ ✅" -ForegroundColor Green
    Write-Host "Rust emulator matches Vectrexy reference implementation.`n" -ForegroundColor Green
} else {
    Write-Host "`n❌ ❌ ❌  TEST FAILED: $TestName  ❌ ❌ ❌" -ForegroundColor Red
    Write-Host "Differences found between Rust and Vectrexy.`n" -ForegroundColor Red
    exit 1
}
