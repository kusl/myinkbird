#!/usr/bin/env bash
# Supply-chain audit: licences + security advisories via cargo-deny.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib.sh
. "$HERE/lib.sh"
cd "$(repo_root)"
ensure_rust
if ! command -v cargo-deny >/dev/null 2>&1; then
  log "installing cargo-deny"
  cargo install cargo-deny --locked
fi
log "running cargo-deny checks (advisories, bans, licenses, sources)"
cargo deny check
log "supply-chain checks complete"
