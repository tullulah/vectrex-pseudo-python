<#
Wrapper to invoke lwasm inside WSL so Windows paths can be passed.
Usage:
  ./tools/lwasm.ps1 --6809 --format=raw --output=game.bin tests\all_tests.asm
Notes:
  - Translates the Windows path of final argument (if it exists as a file) to WSL /mnt/... form.
  - All other args passed through unchanged.
#>
param([Parameter(ValueFromRemainingArguments=$true)] $Args)

if (-not $Args) { Write-Host "Usage: lwasm.ps1 [args] file.asm"; exit 1 }

# Attempt to translate last arg if it is a file
$last = $Args[-1]
if (Test-Path $last) {
  $full = (Resolve-Path $last).Path
  $mnt = $full -replace '^([A-Za-z]):','/mnt/$1' -replace '\\','/'
  $Args[-1] = $mnt.ToLower()
}

wsl lwasm @Args
exit $LASTEXITCODE
