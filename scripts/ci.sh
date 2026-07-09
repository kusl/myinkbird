#!/usr/bin/env bash
# The single entry point CI calls. Installs system deps, then lints, builds and
# tests. Keeping the logic here (not in the workflow YAML) lets contributors run
# the exact same pipeline locally. See docs/adr/0008.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib.sh
. "$HERE/lib.sh"

"$HERE/install-system-deps.sh"
ensure_rust
rustup component add rustfmt clippy >/dev/null 2>&1 || true
"$HERE/lint.sh"
"$HERE/build.sh"
"$HERE/test.sh"
log "CI pipeline finished"
