#!/usr/bin/env bash
# Check formatting and run clippy with warnings denied.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib.sh
. "$HERE/lib.sh"
cd "$(repo_root)"
ensure_rust
rustup component add rustfmt clippy >/dev/null 2>&1 || true
log "checking formatting (cargo fmt --check)"
cargo fmt --all --check
log "running clippy (warnings as errors)"
cargo clippy --workspace --all-targets -- -D warnings
log "lint complete"
