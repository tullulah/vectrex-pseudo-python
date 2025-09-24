<#
WSL LWTOOLS Installer
Prereqs: WSL with an Ubuntu distribution installed and initialized.
Usage (PowerShell):
  ./tools/install_lwtools_wsl.ps1
This will:
  1. Ensure required Ubuntu packages (git build-essential flex bison) are installed.
  2. Clone or update lwtools under: $HOME/lwtools (inside WSL).
  3. Build & install (sudo make install) placing lwasm in /usr/local/bin.
After completion you can invoke: wsl lwasm --version
Optional wrapper: use ./tools/lwasm.ps1 to call from Windows PATH.
#>

param(
  [string]$Distro = "Ubuntu",
  [string]$RepoUrl = "",
  [switch]$AutoInstall,
  [switch]$ForceRebuild,
  [switch]$UseBrew,
  [switch]$VerboseList
)

Write-Host "[lwtools] Using WSL distro: $Distro"

# Quick WSL presence check
$wslListRaw = wsl -l -q 2>$null
if (-not $?) { Write-Error "WSL not available. Enable WSL (wsl --install) then rerun."; exit 1 }
$wslDistros = $wslListRaw -split "`r?`n" | ForEach-Object { $_.Trim() } | Where-Object { $_ -and $_.Length -gt 0 }
if ($VerboseList) {
  Write-Host "[lwtools] Raw distro list:`n$wslListRaw" -ForegroundColor DarkCyan
  $i = 0
  foreach ($d in $wslDistros) { Write-Host ("[lwtools] Dist[$i]='$d' len=" + $d.Length); $i++ }
}
if (-not ($wslDistros | Where-Object { $_.Equals($Distro, 'InvariantCultureIgnoreCase') })) {
  if ($AutoInstall -and $Distro -eq 'Ubuntu') {
    Write-Host "[lwtools] Distro '$Distro' not found. Attempting automatic install... (requires admin)" -ForegroundColor Yellow
    try {
      wsl --install -d Ubuntu
      Write-Host "[lwtools] Ubuntu install initiated. Launch the Ubuntu app once to finish user setup, then re-run this script." -ForegroundColor Yellow
    } catch {
      Write-Error "Automatic install failed: $_"; exit 1
    }
    exit 0
  }
  Write-Error "Distro '$Distro' not found. Existing: $($wslDistros -join ', ') (run: wsl -l -o to list available). Install it or pass -Distro <name>."; exit 1
}

# If lwasm already installed and not forcing, skip build
$existingLwasm = & wsl -d $Distro bash -lc "command -v lwasm || true"
if (-not $ForceRebuild -and $existingLwasm -and $existingLwasm.Trim() -ne "" -and -not ($existingLwasm -match 'not found')) {
  Write-Host "[lwtools] lwasm already present at $existingLwasm . Use -ForceRebuild to rebuild." -ForegroundColor Green
  & wsl -d $Distro bash -lc "lwasm --version || true"
  exit 0
}

if ($UseBrew) {
  Write-Host "[lwtools] Using Homebrew installation path (-UseBrew)" -ForegroundColor Cyan
  $brewScript = @'
set -e
echo "[lwtools] Ensuring prerequisites (curl git build-essential file)"
sudo apt-get update -y
sudo apt-get install -y build-essential curl file git
if ! command -v brew >/dev/null 2>&1; then
  echo "[lwtools] Installing Homebrew (non-interactive)";
  NONINTERACTIVE=1 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" || { echo "[lwtools] Homebrew install failed"; exit 1; }
fi
if [ -x /home/linuxbrew/.linuxbrew/bin/brew ]; then
  eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"
elif [ -x "$HOME/.linuxbrew/bin/brew" ]; then
  eval "$("$HOME/.linuxbrew/bin/brew" shellenv)"
fi
echo "[lwtools] Brew path: $(command -v brew)"
echo "[lwtools] Searching for lwtools formula";
brew search lwtools || true
if brew list lwtools >/dev/null 2>&1; then
  echo "[lwtools] lwtools already installed"
else
  echo "[lwtools] Installing lwtools via brew";
  brew install lwtools || { echo "[lwtools] brew install lwtools failed"; exit 1; }
fi
if command -v lwasm >/dev/null 2>&1; then
  echo "[lwtools] Installed lwasm at $(command -v lwasm)"; lwasm --version || true
else
  echo "[lwtools] ERROR: lwasm not found after brew install"; exit 1
fi
'@
  $b64 = [Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes($brewScript))
  wsl -d $Distro bash -lc "echo $b64 | base64 -d > /tmp/lwtools_brew.sh && sed -i 's/\r$//' /tmp/lwtools_brew.sh && chmod +x /tmp/lwtools_brew.sh"
  if ($LASTEXITCODE -ne 0) { Write-Error "Failed to create brew script in WSL."; exit 1 }
  wsl -d $Distro bash -lc "/bin/bash /tmp/lwtools_brew.sh"
  if ($LASTEXITCODE -ne 0) { Write-Error "Homebrew lwtools install failed."; exit 1 }
  Write-Host "[lwtools] Done (brew). Test with: wsl lwasm --version" -ForegroundColor Green
  exit 0
}

$scriptPath = Join-Path $PSScriptRoot 'lwtools_build.sh'
if (-not (Test-Path $scriptPath)) { Write-Error "Missing build script: $scriptPath"; exit 1 }
Write-Host "[lwtools] Using build script: $scriptPath"
Write-Host "[lwtools] Executing build inside WSL..."
Write-Host "[lwtools] Transferring build script to WSL..."
$bytes = [IO.File]::ReadAllBytes($scriptPath)
$b64 = [Convert]::ToBase64String($bytes)
wsl -d $Distro bash -lc "echo $b64 | base64 -d > /tmp/lwtools_build.sh && sed -i 's/\r$//' /tmp/lwtools_build.sh && chmod +x /tmp/lwtools_build.sh"
if ($LASTEXITCODE -ne 0) { Write-Error "Failed to create script in WSL."; exit 1 }
Write-Host "[lwtools] Executing build script in WSL..."
wsl -d $Distro bash -lc "REPO_URL='$RepoUrl' /bin/bash /tmp/lwtools_build.sh"
if ($LASTEXITCODE -ne 0) {
  Write-Host '[lwtools] Dumping /tmp/lwtools_build.sh (first 60 lines) for debugging:' -ForegroundColor Yellow
  wsl -d $Distro bash -lc "nl -ba /tmp/lwtools_build.sh | sed -n '1,60p'"
  Write-Error "WSL lwtools build failed."; exit 1
}
Write-Host "[lwtools] Done. Test with: wsl lwasm --version"
