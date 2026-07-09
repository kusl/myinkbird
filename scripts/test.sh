#!/usr/bin/env bash
# Run all workspace unit tests.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib.sh
. "$HERE/lib.sh"
cd "$(repo_root)"
ensure_rust
log "running workspace tests"
cargo test --workspace
log "tests complete"
