#!/usr/bin/env bash
# Build a self-contained release binary of inkbird-collector for the HOST
# platform, then package it (tar.gz) with a SHA-256 checksum into dist/.
#
# The same script runs locally and in CI. CI runs it on a Linux/macOS/Windows
# matrix, so each run produces that platform's native binary. We build natively
# rather than cross-compiling because btleplug uses a different Bluetooth backend
# per platform (BlueZ/D-Bus on Linux, CoreBluetooth on macOS, WinRT on Windows),
# which is impractical to cross-compile. See docs/adr/0011 and docs/releases.md.
#
# The resulting binary is self-contained (all Rust crates are linked in). On
# Linux it still dynamically links the system C library and libdbus, both of
# which are present on any desktop with Bluetooth; macOS and Windows need no
# extra runtime libraries.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib.sh
. "$HERE/lib.sh"
cd "$(repo_root)"

os="$(uname -s)"

# On Linux, btleplug's backend links libdbus; install the build headers.
# macOS (CoreBluetooth) and Windows (WinRT) need no extra system packages.
case "$os" in
  Linux) "$HERE/install-system-deps.sh" ;;
  *)     : ;;
esac

ensure_rust

version="${INKBIRD_VERSION:-${1:-$(workspace_version)}}"
[ -n "$version" ] || die "could not determine the workspace version"

triple="$(rustc -vV | sed -n 's/^host: //p')"
[ -n "$triple" ] || die "could not determine the host target triple from rustc"

log "building inkbird-collector ${version} for ${triple} (release)"
cargo build --release -p inkbird-collector

bin="inkbird-collector"
ext=""
case "$os" in
  MINGW* | MSYS* | CYGWIN* | Windows_NT) ext=".exe" ;;
esac
built="target/release/${bin}${ext}"
[ -f "$built" ] || die "expected build output not found: $built"

dist="dist"
mkdir -p "$dist"
stem="${bin}-${version}-${triple}"
archive="${dist}/${stem}.tar.gz"

# Stage the binary (under its plain name) plus the licence and readme, so the
# download is usable and carries the AGPL text.
stage="$(mktemp -d)"
trap 'rm -rf "$stage"' EXIT
cp "$built" "${stage}/${bin}${ext}"
cp LICENSE README.md "$stage/" 2>/dev/null || true

log "packaging ${archive}"
tar -czf "$archive" -C "$stage" .

# Checksum sidecar next to the archive (path-relative so `sha256sum -c` works
# from inside dist/).
( cd "$dist" && checksum_file "$(basename "$archive")" > "$(basename "$archive").sha256" )

log "release artifact ready:"
log "  ${archive}"
log "  ${archive}.sha256"
