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
  [string]$Distro = "Ubuntu"
)

Write-Host "[lwtools] Using WSL distro: $Distro"

# Quick WSL presence check
$wslList = wsl -l -q 2>$null
if (-not $?) { Write-Error "WSL not available. Enable WSL and install Ubuntu first."; exit 1 }
if (-not ($wslList -match $Distro)) { Write-Error "Distro '$Distro' not found. Existing: $wslList"; exit 1 }

$bashCmd = @'
set -e
echo "[lwtools] Updating apt package index"
sudo apt-get update -y
echo "[lwtools] Installing build prerequisites"
sudo apt-get install -y git build-essential flex bison
if [ ! -d "$HOME/lwtools" ]; then
  echo "[lwtools] Cloning lwtools"
  git clone https://github.com/lwtools/lwtools.git "$HOME/lwtools"
else
  echo "[lwtools] Updating existing lwtools repo"
  cd "$HOME/lwtools" && git pull --ff-only
fi
cd "$HOME/lwtools"
echo "[lwtools] Building"
make -j"$(nproc)"
echo "[lwtools] Installing (sudo)"
sudo make install
echo "[lwtools] Installed lwasm:" $(command -v lwasm)
lwasm --version || true
'@

Write-Host "[lwtools] Executing build inside WSL..."
wsl -d $Distro bash -lc $bashCmd

if ($LASTEXITCODE -ne 0) { Write-Error "WSL lwtools build failed."; exit 1 }
Write-Host "[lwtools] Done. Test with: wsl lwasm --version"
