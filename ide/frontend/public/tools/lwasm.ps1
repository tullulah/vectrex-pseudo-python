<#
LWASM wrapper for Windows using local executables
Uses lwasm executables from tools directory

Usage:
  ./tools/lwasm_simple.ps1 source.asm dest.bin
  ./tools/lwasm_simple.ps1 --format=raw --output=dest.bin source.asm
#>
param([Parameter(ValueFromRemainingArguments=$true)] $lwArgs)

Write-Host "=== LWASM LOCAL EXECUTABLE WRAPPER ==="

if (-not $lwArgs) { 
    Write-Host "Usage: lwasm_simple.ps1 [args] file.asm" 
    Write-Host "   or: lwasm_simple.ps1 source.asm dest.bin"
    exit 1 
}

# Normalize simple form: (source.asm dest.bin) or (source.asm dest.raw)
if ($lwArgs.Count -eq 2 -and $lwArgs[0] -match '\.asm$' -and $lwArgs[1] -match '\.(bin|raw)$') {
    $src = $lwArgs[0]; $dst = $lwArgs[1]
    Write-Host "Simple form detected: $src -> $dst"
    $lwArgs = @('--6809','--format=raw',"--output=$dst", $src)
}

Write-Host "Arguments: $($lwArgs -join ' ')"

# Get script directory to find local lwasm executables
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Write-Host "Script directory: $scriptDir"

# Try local lwasm.exe first
$localLwasm = Join-Path $scriptDir "lwasm.exe"
Write-Host "Checking for local lwasm: $localLwasm"

if (Test-Path $localLwasm) {
    Write-Host "Found local lwasm at: $localLwasm"
    try {
        # Change to project root directory for correct include paths
        $projectRoot = (Split-Path (Split-Path (Split-Path (Split-Path $scriptDir))))
        Write-Host "Changing to project root: $projectRoot"
        Push-Location $projectRoot
        
        Write-Host "Running lwasm with args: $($lwArgs -join ' ')"
        & $localLwasm @lwArgs
        $exitCode = $LASTEXITCODE
        
        Pop-Location
        Write-Host "lwasm exit code: $exitCode"
        exit $exitCode
    }
    catch {
        Pop-Location
        Write-Host "Error running lwasm: $($_.Exception.Message)"
        exit 1
    }
} else {
    Write-Host "Local lwasm.exe not found"
}

# Try system PATH as fallback
Write-Host "Trying system PATH..."
try {
    $pathResult = Get-Command lwasm -ErrorAction SilentlyContinue
    if ($pathResult) {
        Write-Host "Found lwasm in system PATH: $($pathResult.Source)"
        & lwasm @lwArgs
        $exitCode = $LASTEXITCODE
        Write-Host "lwasm exit code: $exitCode"
        exit $exitCode
    } else {
        Write-Host "lwasm not found in system PATH"
    }
}
catch {
    Write-Host "Error checking system PATH: $($_.Exception.Message)"
}

# Installation guidance
Write-Host ""
Write-Host "=== LWASM NOT FOUND ==="
Write-Host "lwasm (6809 assembler) is required to generate binary files."
Write-Host ""
Write-Host "Please place lwasm.exe in the tools directory or add it to PATH."
Write-Host "Download lwtools from: http://lwtools.projects.l-w.ca/"
Write-Host ""
Write-Host "The compiler can still generate .asm files without lwasm."
exit 1