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

# Normalize simple form: (source.asm dest.bin) or (source.asm dest.raw)
if ($Args.Count -eq 2 -and $Args[0] -match '\.asm$' -and $Args[1] -match '\.(bin|raw)$') {
  $src = $Args[0]; $dst = $Args[1]
  $Args = @('--6809','--format=raw',"--output=$dst", $src)
}

# Convert a Windows path to a lowercase /mnt/<drive>/ form for WSL
function Convert-ToMntPath($p) {
  if (Test-Path $p) { $full = (Resolve-Path $p).Path } else { $full = [System.IO.Path]::GetFullPath($p) }
  return ($full -replace '^([A-Za-z]):','/mnt/$1' -replace '\\','/').ToLower()
}

# Normalize all path-like args (.asm/.bin/.raw) and --output forms
for ($i=0; $i -lt $Args.Count; $i++) {
  $a = $Args[$i]
  if ($a -match '^--output=(.+)$') {
    $outPath = $Matches[1]
    $Args[$i] = "--output=" + (Convert-ToMntPath $outPath)
    continue
  }
  if ($a -eq '--output' -and ($i + 1) -lt $Args.Count) {
    $next = $Args[$i+1]
    $Args[$i+1] = (Convert-ToMntPath $next)
    continue
  }
  if ($a -match '(?i)\.(asm|bin|raw)$') {
    if (Test-Path $a) { $Args[$i] = Convert-ToMntPath $a } else {
      # Try resolving relative
      $full = Join-Path (Get-Location) $a
      if (Test-Path $full) { $Args[$i] = Convert-ToMntPath $full }
    }
  }
}

# Ensure lwasm exists at expected location
$lwasmPath = '/home/linuxbrew/.linuxbrew/bin/lwasm'
$check = wsl -d Ubuntu bash -lc "test -x $lwasmPath && echo ok"
if (-not $check.Trim().Equals('ok')) { Write-Error 'lwasm not found in Linuxbrew path'; exit 2 }

# cd into current project directory inside WSL so relative includes (future) work
$projMnt = (Convert-ToMntPath (Get-Location))

# Quote each arg safely for bash
$escapedArgs = @()
foreach ($a in $Args) {
  if ($a -notmatch "'") { $escapedArgs += "'$a'" } else { $escapedArgs += ( '"' + ($a -replace '"','\\"') + '"') }
}
$cmdLine = ("cd '$projMnt' && PATH=/home/linuxbrew/.linuxbrew/bin:\$PATH exec $lwasmPath " + ($escapedArgs -join ' '))
Write-Host "[lwasm.ps1] CMD: $cmdLine"
wsl -d Ubuntu bash -lc "$cmdLine"
exit $LASTEXITCODE
