#!/usr/bin/env bash
# Stop and remove the stack.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib.sh
. "$HERE/lib.sh"
cd "$(repo_root)"
require_podman
if podman compose version >/dev/null 2>&1; then
  as_root podman compose down
elif command -v podman-compose >/dev/null 2>&1; then
  as_root podman-compose down
else
  die "no compose provider found"
fi
log "stack stopped"
