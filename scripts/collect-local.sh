#!/usr/bin/env bash
# Run the collector directly on the host (no containers) and store readings in a
# visible ./data directory. This is the quickest way to confirm your sensor is
# seen and that readings are being written - no image build required.
#
# It still needs root, because BlueZ's D-Bus policy only accepts calls from
# uid 0 (see docs/adr/0005). We acquire sudo once, up front, and keep it alive.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib.sh
. "$HERE/lib.sh"
cd "$(repo_root)"
ensure_rust

# Reuse the same configuration the stack uses, if you have a .env.
if [ -f .env ]; then
  set -a; . ./.env; set +a
fi
# An empty INKBIRD_ADDRESS means "no address filter" (match by name); make sure
# an empty value from .env is not treated as a real (empty) address.
[ -z "${INKBIRD_ADDRESS:-}" ] && unset INKBIRD_ADDRESS || true

DATA_DIR="${INKBIRD_HOST_DATA_DIR:-./data}"
mkdir -p "$DATA_DIR"
ABS_DATA_DIR="$(cd "$DATA_DIR" && pwd)"

log "building the collector (release)"
cargo build --release -p inkbird-collector

ensure_root
export RUST_LOG="${RUST_LOG:-info}"
export INKBIRD_DATA_DIR="$ABS_DATA_DIR"
export INKBIRD_NAME_MATCH="${INKBIRD_NAME_MATCH:-ith-13-b}"
export INKBIRD_MIN_INTERVAL_SECS="${INKBIRD_MIN_INTERVAL_SECS:-60}"
[ -n "${INKBIRD_ADDRESS:-}" ] && export INKBIRD_ADDRESS || true

log "listening for ITH-13-B advertisements; readings -> ${ABS_DATA_DIR}/readings/"
log "press Ctrl-C to stop"
if [ "$(id -u)" -eq 0 ]; then
  ./target/release/inkbird-collector collect
else
  # -E preserves the INKBIRD_*/RUST_LOG we just exported.
  sudo -E ./target/release/inkbird-collector collect
fi
