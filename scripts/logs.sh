#!/usr/bin/env bash
# Follow the running stack's logs (rootful, to match scripts/run.sh).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib.sh
. "$HERE/lib.sh"
cd "$(repo_root)"
require_podman
ensure_root
if podman compose version >/dev/null 2>&1; then
  as_root podman compose logs -f
elif command -v podman-compose >/dev/null 2>&1; then
  as_root podman-compose logs -f
else
  die "no compose provider found"
fi
