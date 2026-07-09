#!/usr/bin/env bash
# Shared helpers for the myinkbird scripts.
# Source it from another script:  . "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
set -euo pipefail

log()  { printf '\033[1;34m[myinkbird]\033[0m %s\n' "$*" >&2; }
warn() { printf '\033[1;33m[myinkbird]\033[0m %s\n' "$*" >&2; }
die()  { printf '\033[1;31m[myinkbird]\033[0m %s\n' "$*" >&2; exit 1; }

# Repository root (parent of the scripts/ directory).
repo_root() { cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd; }

# Run a command as root when not already root; prefer sudo.
as_root() {
  if [ "$(id -u)" -eq 0 ]; then
    "$@"
  elif command -v sudo >/dev/null 2>&1; then
    sudo "$@"
  else
    die "root required for: $* (install sudo or run as root)"
  fi
}

# Echo the detected system package manager, or an empty string.
detect_pkg_mgr() {
  local mgr
  for mgr in dnf microdnf apt-get zypper pacman; do
    if command -v "$mgr" >/dev/null 2>&1; then
      echo "$mgr"
      return 0
    fi
  done
  echo ""
}

# Ensure Podman is available (vendor-neutral: Podman only).
require_podman() {
  command -v podman >/dev/null 2>&1 || die "podman not found; please install Podman"
}

# Ensure cargo/rustc are on PATH, installing the toolchain via rustup if not.
ensure_rust() {
  if command -v cargo >/dev/null 2>&1; then
    return 0
  fi
  if command -v rustup >/dev/null 2>&1; then
    rustup show >/dev/null 2>&1 || rustup toolchain install stable
  else
    warn "cargo not found; attempting to install the Rust toolchain"
    local mgr
    mgr="$(detect_pkg_mgr)"
    case "$mgr" in
      dnf)      as_root dnf -y install rustup ;;
      microdnf) as_root microdnf install -y rustup ;;
      apt-get)  as_root apt-get update && as_root apt-get install -y rustup ;;
      zypper)   as_root zypper --non-interactive install rustup ;;
      pacman)   as_root pacman -Sy --noconfirm rustup ;;
      *)        die "cannot auto-install Rust; install rustup from https://rustup.rs" ;;
    esac
    rustup-init -y --profile minimal --default-toolchain stable --no-modify-path
  fi
  # shellcheck disable=SC1091
  [ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"
  export PATH="$HOME/.cargo/bin:$PATH"
  command -v cargo >/dev/null 2>&1 \
    || die "Rust installed but cargo not on PATH; open a new shell or 'source ~/.cargo/env'"
}
