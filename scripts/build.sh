#!/usr/bin/env bash
# Build the whole workspace in release mode.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib.sh
. "$HERE/lib.sh"
cd "$(repo_root)"
ensure_rust
log "building workspace (release)"
cargo build --workspace --release
log "build complete"
