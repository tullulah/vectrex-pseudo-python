# Run All Comparative Opcode Tests
# Automatically executes all CPU opcode test cases

param(
    [switch]$SkipBuild = $false
)

$scriptDir = $PSScriptRoot
$testCases = @(
    @{Name="cpu_arithmetic"; Cycles=50; Description="ADD operations"},
    @{Name="cpu_subtract"; Cycles=80; Description="SUB operations"},
    @{Name="cpu_logic"; Cycles=100; Description="AND/OR/EOR operations"},
    @{Name="cpu_compare"; Cycles=120; Description="CMP operations"},
    @{Name="cpu_increment"; Cycles=100; Description="INC/DEC operations"},
    @{Name="cpu_shift"; Cycles=120; Description="Shift/Rotate operations"},
    @{Name="cpu_branch"; Cycles=80; Description="Branch operations"},
    @{Name="cpu_load_store"; Cycles=80; Description="Load/Store operations"},
    @{Name="cpu_stack"; Cycles=150; Description="Stack PUSH/PULL"},
    @{Name="cpu_indexed"; Cycles=150; Description="Indexed addressing"},
    @{Name="cpu_transfer"; Cycles=150; Description="TFR/EXG operations"},
    @{Name="cpu_jsr_rts"; Cycles=80; Description="Subroutine JSR/RTS"}
)

$passed = 0
$failed = 0
$failedTests = @()

Write-Host "`n╔═══════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  MC6809 OPCODE COMPARATIVE TEST SUITE                ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

foreach ($test in $testCases) {
    Write-Host "Testing: " -NoNewline
    Write-Host "$($test.Name)" -ForegroundColor Yellow -NoNewline
    Write-Host " - $($test.Description)" -ForegroundColor Gray
    
    if ($SkipBuild) {
        & "$scriptDir\run_comparative_test_v2.ps1" -TestName $test.Name -Cycles $test.Cycles -SkipBuild | Out-Null
    } else {
        & "$scriptDir\run_comparative_test_v2.ps1" -TestName $test.Name -Cycles $test.Cycles | Out-Null
    }
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  ✅ PASSED`n" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "  ❌ FAILED`n" -ForegroundColor Red
        $failed++
        $failedTests += $test.Name
    }
}

Write-Host "`n╔═══════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  TEST SUMMARY                                         ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

Write-Host "Total Tests: " -NoNewline
Write-Host "$($testCases.Count)" -ForegroundColor White

Write-Host "Passed:      " -NoNewline
Write-Host "$passed" -ForegroundColor Green

Write-Host "Failed:      " -NoNewline
Write-Host "$failed" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })

if ($failed -gt 0) {
    Write-Host "`nFailed Tests:" -ForegroundColor Red
    foreach ($testName in $failedTests) {
        Write-Host "  - $testName" -ForegroundColor Red
    }
    Write-Host ""
    exit 1
} else {
    Write-Host "`n🎉 ALL TESTS PASSED! 🎉`n" -ForegroundColor Green
    exit 0
}
