# Build script for Emulator V2 WASM
# Compila el emulador a WASM y genera bindings JavaScript

Write-Host "[BUILD] Building Vectrex Emulator V2 for WASM..." -ForegroundColor Cyan

# Change to emulator_v2 directory
Set-Location $PSScriptRoot

# Step 1: Build WASM
Write-Host ""
Write-Host "[1/3] Compiling Rust to WASM..." -ForegroundColor Yellow
cargo build --features wasm --target wasm32-unknown-unknown --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] WASM compilation failed!" -ForegroundColor Red
    exit 1
}

Write-Host "[OK] WASM compilation successful" -ForegroundColor Green

# Step 2: Generate JavaScript bindings
Write-Host ""
Write-Host "[2/3] Generating JavaScript bindings..." -ForegroundColor Yellow
wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/vectrex_emulator_v2.wasm

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] wasm-bindgen failed!" -ForegroundColor Red
    exit 1
}

Write-Host "[OK] Bindings generated successfully" -ForegroundColor Green

# Step 3: Copy to IDE frontend (optional)
$frontendWasmDir = "..\ide\frontend\public\wasm"

if (Test-Path $frontendWasmDir) {
    Write-Host ""
    Write-Host "[3/3] Copying to IDE frontend..." -ForegroundColor Yellow
    
    # Create directory if it doesn't exist
    if (!(Test-Path $frontendWasmDir)) {
        New-Item -ItemType Directory -Path $frontendWasmDir | Out-Null
    }
    
    Copy-Item "pkg\*" $frontendWasmDir -Force
    Write-Host "[OK] Files copied to $frontendWasmDir" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "[SKIP] Step 3/3: IDE frontend not found" -ForegroundColor Yellow
}

# Summary
Write-Host ""
Write-Host "[DONE] Build complete!" -ForegroundColor Cyan
Write-Host ""
Write-Host "Generated files in emulator_v2/pkg/:" -ForegroundColor White
Get-ChildItem pkg | ForEach-Object { Write-Host "  - $($_.Name)" -ForegroundColor Gray }

Write-Host ""
Write-Host "See WASM_API.md for usage instructions" -ForegroundColor Cyan

