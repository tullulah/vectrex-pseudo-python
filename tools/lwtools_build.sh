#!/bin/bash
set -e
DEFAULT_URLS=(
  "https://github.com/lwtools/lwtools.git"
  "https://gitlab.com/retro-tools/lwtools.git"
  "https://github.com/tenox7/lwtools.git"
)
if [ -n "${REPO_URL:-}" ]; then
  DEFAULT_URLS=("$REPO_URL")
fi
printf '[lwtools] Updating apt package index\n'
sudo apt-get update -y
printf '[lwtools] Installing build prerequisites\n'
sudo apt-get install -y git build-essential flex bison || sudo apt-get install -y git build-essential flex

# Ensure previous partial clone is removed if invalid
if [ -d "$HOME/lwtools" ]; then
  if [ ! -d "$HOME/lwtools/.git" ]; then
    printf '[lwtools] Removing non-git directory at ~/lwtools (partial/invalid)\n'
    rm -rf "$HOME/lwtools"
  fi
fi

if [ -d "$HOME/lwtools/.git" ]; then
  current_remote=$(git -C "$HOME/lwtools" config --get remote.origin.url 2>/dev/null || echo '')
  if [ "$current_remote" != "$REPO_URL" ]; then
    printf '[lwtools] Remote mismatch (%s), recloning...\n' "$current_remote"
    rm -rf "$HOME/lwtools"
  fi
fi

try_clone() {
  url="$1"
  printf '[lwtools] Attempting clone from %s\n' "$url"
  git -c credential.helper= -c core.askpass= -c credential.interactive=0 clone --depth=1 "$url" "$HOME/lwtools" 2>&1
}

if [ ! -d "$HOME/lwtools" ]; then
  for u in "${DEFAULT_URLS[@]}"; do
    out=$(try_clone "$u") || true
    if [ -d "$HOME/lwtools/.git" ]; then
      printf '%s' "$out" | head -n 5
      REPO_USED="$u"
      break
    else
      printf '[lwtools] Clone failed from %s (will try next)\n' "$u"
    fi
  done
  if [ ! -d "$HOME/lwtools/.git" ]; then
    printf '[lwtools] ERROR: All clone attempts failed.\n'
    exit 1
  fi
else
  printf '[lwtools] Updating existing lwtools repo\n'
  if ! git -C "$HOME/lwtools" fetch --depth=1 origin; then
    printf '[lwtools] Fetch failed; cleaning and retrying clones.\n'
    rm -rf "$HOME/lwtools"
    for u in "${DEFAULT_URLS[@]}"; do
      out=$(try_clone "$u") || true
      if [ -d "$HOME/lwtools/.git" ]; then
        REPO_USED="$u"
        break
      fi
    done
    if [ ! -d "$HOME/lwtools/.git" ]; then
      printf '[lwtools] ERROR: Re-clone attempts failed.\n'
      exit 1
    fi
  else
    git -C "$HOME/lwtools" reset --hard origin/HEAD 2>/dev/null || git -C "$HOME/lwtools" pull --ff-only
    REPO_USED=$(git -C "$HOME/lwtools" config --get remote.origin.url || echo 'unknown')
  fi
fi
printf '[lwtools] Using repository %s\n' "${REPO_USED:-unknown}"

cd "$HOME/lwtools"
printf '[lwtools] Building\n'
make -j"$(nproc)"
printf '[lwtools] Installing (sudo)\n'
sudo make install
printf '[lwtools] Installed lwasm: %s\n' "$(command -v lwasm)"
lwasm --version || true
