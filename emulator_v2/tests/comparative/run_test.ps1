param(
    [string]$TestCase = "irq_timer1",
    [int]$Cycles = 500
)

Write-Host "`n===== COMPARATIVE TEST FRAMEWORK =====" -ForegroundColor Cyan

$ComparativeRoot = "$PSScriptRoot"
$RustRunner = "$ComparativeRoot\rust_runner"
$TestCaseDir = "$ComparativeRoot\test_cases\$TestCase"
$RustOutput = "$TestCaseDir\rust_output.json"
$ExpectedFile = "$TestCaseDir\expected.json"

Write-Host "[1] Assembling..." -ForegroundColor Yellow
$LwasmExe = "C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\tools\lwasm.exe"
& $LwasmExe -9 --raw -o "$TestCaseDir\test.bin" "$TestCaseDir\test.asm" 2>&1 | Out-Null
Write-Host "   [OK]`n" -ForegroundColor Green

Write-Host "[2] Building Rust..." -ForegroundColor Yellow
Push-Location $RustRunner
& cargo build --release --quiet
Pop-Location
Write-Host "   [OK]`n" -ForegroundColor Green

Write-Host "[3] Running Rust..." -ForegroundColor Yellow
& "$RustRunner\target\release\rust_runner.exe" "$TestCaseDir\test.bin" $Cycles | Out-File -FilePath $RustOutput -Encoding utf8
Write-Host "   [OK]`n" -ForegroundColor Green

Write-Host "[4] Comparing..." -ForegroundColor Yellow
& python "$ComparativeRoot\compare.py" $ExpectedFile $RustOutput
$ExitCode = $LASTEXITCODE

Write-Host "`n===== SUMMARY =====" -ForegroundColor Cyan
if ($ExitCode -eq 0) {
    Write-Host "[] ALL TESTS PASSED`n" -ForegroundColor Green
    exit 0
} else {
    Write-Host "[] DIFFERENCES FOUND`n" -ForegroundColor Red
    exit 1
}
