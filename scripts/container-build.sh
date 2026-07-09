#!/usr/bin/env bash
# Build both OCI images with Podman.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib.sh
. "$HERE/lib.sh"
cd "$(repo_root)"
require_podman
log "building collector image (localhost/myinkbird-collector:latest)"
podman build -f Containerfile -t localhost/myinkbird-collector:latest .
log "building committer image (localhost/myinkbird-committer:latest)"
podman build -f Containerfile.committer -t localhost/myinkbird-committer:latest .
log "images built"
