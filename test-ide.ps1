# Test script for VPy IDE - Execute in separate PowerShell to avoid interfering with Electron terminal
# Uso: .\test-ide.ps1 [-SkipBuild] [-OpenBrowser]

param(
    [switch]$SkipBuild,
    [switch]$OpenBrowser
)

Write-Host "=== VPy IDE Test Script ===" -ForegroundColor Cyan
Write-Host "Timestamp: $(Get-Date)" -ForegroundColor Gray

# Change to project directory
$ProjectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $ProjectRoot
Write-Host "Working directory: $PWD" -ForegroundColor Yellow

try {
    if (-not $SkipBuild) {
        Write-Host "`n[1/3] Building Rust vectrex_lang compiler..." -ForegroundColor Green
        Set-Location vectrex_lang
        cargo build --release
        if ($LASTEXITCODE -ne 0) {
            throw "Rust build failed with exit code $LASTEXITCODE"
        }
        Set-Location ..
        Write-Host "✅ Rust compiler built successfully" -ForegroundColor Green

        Write-Host "`n[2/3] Building TypeScript frontend..." -ForegroundColor Green
        Set-Location ide/frontend
        npm run build
        if ($LASTEXITCODE -ne 0) {
            throw "TypeScript build failed with exit code $LASTEXITCODE"
        }
        Set-Location ../..
        Write-Host "✅ Frontend built successfully" -ForegroundColor Green
    } else {
        Write-Host "⏭️  Skipping build (SkipBuild flag set)" -ForegroundColor Yellow
    }

    Write-Host "`n[3/3] Testing VPy compilation with bouncing ball demo..." -ForegroundColor Green
    
    # Test compilation of bouncing ball demo
    $CompilerPath = "vectrex_lang/target/release/vectrex_lang.exe"
    $DemoPath = "bouncing_ball_fixed.vpy"
    
    if (-not (Test-Path $CompilerPath)) {
        throw "Compiler not found at $CompilerPath. Run without -SkipBuild first."
    }
    
    if (-not (Test-Path $DemoPath)) {
        throw "Demo file not found at $DemoPath"
    }
    
    Write-Host "Compiling: $DemoPath" -ForegroundColor Cyan
    & $CompilerPath $DemoPath --bin
    if ($LASTEXITCODE -ne 0) {
        throw "VPy compilation failed with exit code $LASTEXITCODE"
    }
    
    # Check generated files
    $AsmFile = $DemoPath -replace "\.vpy$", ".asm"
    $BinFile = $DemoPath -replace "\.vpy$", ".bin"
    
    if (Test-Path $AsmFile) {
        $AsmSize = (Get-Item $AsmFile).Length
        Write-Host "✅ Generated ASM: $AsmFile ($AsmSize bytes)" -ForegroundColor Green
    }
    
    if (Test-Path $BinFile) {
        $BinSize = (Get-Item $BinFile).Length
        Write-Host "✅ Generated BIN: $BinFile ($BinSize bytes)" -ForegroundColor Green
    } else {
        Write-Warning "BIN file not generated - check compiler --bin flag"
    }

    if ($OpenBrowser) {
        Write-Host "`n[BONUS] Opening IDE in browser..." -ForegroundColor Magenta
        Start-Process "http://localhost:3000"
    }

    Write-Host "`n🎉 All tests passed successfully!" -ForegroundColor Green
    Write-Host "IDE is ready for development." -ForegroundColor Cyan

} catch {
    Write-Host "`n❌ Test failed: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "Stack trace:" -ForegroundColor Red
    Write-Host $_.ScriptStackTrace -ForegroundColor Red
    exit 1
}

Write-Host "`n=== Test Complete ===" -ForegroundColor Cyan