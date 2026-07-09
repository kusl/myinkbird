#!/usr/bin/env bash
# Shared helpers for the myinkbird scripts.
# Source it from another script:  . "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
set -euo pipefail

log()  { printf '\033[1;34m[myinkbird]\033[0m %s\n' "$*" >&2; }
warn() { printf '\033[1;33m[myinkbird]\033[0m %s\n' "$*" >&2; }
die()  { printf '\033[1;31m[myinkbird]\033[0m %s\n' "$*" >&2; exit 1; }

# Repository root (parent of the scripts/ directory).
repo_root() { cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd; }

# Run a single command as root when not already root; prefer sudo.
as_root() {
  if [ "$(id -u)" -eq 0 ]; then
    "$@"
  elif command -v sudo >/dev/null 2>&1; then
    sudo "$@"
  else
    die "root required for: $* (install sudo or run as root)"
  fi
}

# Acquire root ONCE, up front, and keep the sudo timestamp alive for the whole
# life of the calling script. This means a long-running step (image build,
# `compose up`, a native `collect` run) never stalls on a password prompt
# partway through, and the sudo credential does not expire mid-run.
#
# Call this near the top of any script that will need root. Subsequent
# `as_root ...`/`sudo ...` calls then run without prompting. It is a no-op when
# already root, and it is inherited by child scripts (via INKBIRD_SUDO_ACTIVE)
# so nested scripts do not prompt again or start a second keep-alive.
ensure_root() {
  if [ "$(id -u)" -eq 0 ]; then
    return 0
  fi
  if [ -n "${INKBIRD_SUDO_ACTIVE:-}" ]; then
    return 0                       # a parent already acquired + is keeping it alive
  fi
  command -v sudo >/dev/null 2>&1 \
    || die "root is required (BlueZ D-Bus access + rootful Podman) but sudo is not installed; re-run this as root"

  log "root is required (BlueZ D-Bus access + rootful Podman). Requesting sudo now, up front."
  sudo -v || die "could not obtain sudo credentials"
  export INKBIRD_SUDO_ACTIVE=1

  # Background keep-alive: refresh the sudo timestamp every 50s until this
  # script exits. $$ inside the subshell is the PID of this (parent) script, so
  # the loop stops as soon as the script is gone.
  ( while true; do
      sudo -n true 2>/dev/null || exit 0
      sleep 50
      kill -0 "$$" 2>/dev/null || exit 0
    done ) &
  _SUDO_KEEPALIVE_PID=$!
  # Best-effort: stop the keep-alive when this script exits.
  trap 'kill "$_SUDO_KEEPALIVE_PID" 2>/dev/null || true' EXIT
  log "sudo acquired; keeping it alive for the duration of this run"
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
