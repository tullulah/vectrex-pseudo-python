# Quick Test Runner - Execute all opcode tests
# Simple version without fancy UI

Write-Host "`n==== MC6809 OPCODE TESTS ====" -ForegroundColor Cyan

$tests = @(
    # Bloque 1: Operaciones básicas (12 tests) ✅
    "cpu_arithmetic",
    "cpu_subtract", 
    "cpu_logic",
    "cpu_compare",
    "cpu_increment",
    "cpu_shift",
    "cpu_branch",
    "cpu_load_store",
    "cpu_stack",
    "cpu_indexed",
    "cpu_transfer",
    "cpu_jsr_rts",
    
    # Bloque 2: Operaciones avanzadas (8 tests)
    "cpu_multiply",
    "cpu_lea",
    "cpu_branches_extended",
    "cpu_complement",
    "cpu_test",
    "cpu_16bit",
    "cpu_abx",
    "cpu_sex"
)

$passed = 0
$failed = 0

foreach ($test in $tests) {
    Write-Host "`n[$test]" -ForegroundColor Yellow -NoNewline
    
    $output = .\run_comparative_test_v2.ps1 -TestName $test -Cycles 150 -SkipBuild 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host " PASSED" -ForegroundColor Green
        $passed++
    } else {
        Write-Host " FAILED" -ForegroundColor Red
        $failed++
    }
}

Write-Host "`n==== SUMMARY ====" -ForegroundColor Cyan
Write-Host "Passed: $passed" -ForegroundColor Green
Write-Host "Failed: $failed" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host ""
