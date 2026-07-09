#!/usr/bin/env bash
# Build the images and start the stack. Rootful, because BlueZ's D-Bus policy
# only accepts calls from uid 0 (see docs/adr/0005).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib.sh
. "$HERE/lib.sh"
cd "$(repo_root)"
require_podman

"$HERE/container-build.sh"

log "starting stack (rootful for BlueZ D-Bus access)"
if podman compose version >/dev/null 2>&1; then
  as_root podman compose up -d
elif command -v podman-compose >/dev/null 2>&1; then
  as_root podman-compose up -d
else
  die "no compose provider found; install the 'podman compose' plugin or podman-compose"
fi
log "stack started. Follow logs with: sudo podman compose logs -f"
