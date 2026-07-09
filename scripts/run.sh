#!/usr/bin/env bash
# Build the images and start the stack.
#
# Rootful, because BlueZ's D-Bus policy only accepts calls from uid 0
# (see docs/adr/0005). The images are built into the rootful store so the
# rootful stack can find them (see docs/adr/0010). Readings are stored on the
# HOST in a directory you can see and git-browse (see docs/adr/0006).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib.sh
. "$HERE/lib.sh"
cd "$(repo_root)"
require_podman

# Ask for sudo once, up front, and keep it alive for the whole run (build + up).
ensure_root

# Create the host data directory before it is bind-mounted, so the readings and
# their local git repo land in a directory you can browse. Defaults to ./data
# next to compose.yaml; override with INKBIRD_HOST_DATA_DIR (in .env).
DATA_DIR="${INKBIRD_HOST_DATA_DIR:-./data}"
mkdir -p "$DATA_DIR"
log "readings will be stored on the host at: $(cd "$DATA_DIR" && pwd)/readings/"

# Build the images (rootful - same store the stack runs from).
"$HERE/container-build.sh"

log "starting stack (rootful for BlueZ D-Bus access)"
if podman compose version >/dev/null 2>&1; then
  as_root podman compose up -d
elif command -v podman-compose >/dev/null 2>&1; then
  as_root podman-compose up -d
else
  die "no compose provider found; install the 'podman compose' plugin or podman-compose"
fi
log "stack started. Follow logs with: ./scripts/logs.sh   (or: sudo podman compose logs -f)"
