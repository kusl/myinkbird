#!/usr/bin/env bash
# Install the system packages needed to *build* the workspace: a C compiler,
# pkg-config, and the D-Bus development headers (btleplug's Linux backend links
# libdbus). Uses whichever package manager is available.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib.sh
. "$HERE/lib.sh"

mgr="$(detect_pkg_mgr)"
log "installing build dependencies (C compiler, pkg-config, D-Bus headers) via ${mgr:-unknown}"
case "$mgr" in
  dnf)      as_root dnf -y install gcc pkgconf-pkg-config dbus-devel ;;
  microdnf) as_root microdnf install -y gcc pkgconf-pkg-config dbus-devel ;;
  apt-get)  as_root apt-get update && as_root apt-get install -y gcc pkg-config libdbus-1-dev ;;
  zypper)   as_root zypper --non-interactive install gcc pkg-config dbus-1-devel ;;
  pacman)   as_root pacman -Sy --noconfirm gcc pkgconf dbus ;;
  *)        die "undetected package manager; install a C compiler, pkg-config and the D-Bus dev headers manually" ;;
esac
log "system dependencies installed"
