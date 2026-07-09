#!/usr/bin/env bash
# Build both OCI images with Podman, ROOTFUL.
#
# Why rootful: the stack runs rootful (BlueZ's D-Bus policy only accepts calls
# from uid 0 - see docs/adr/0005). Rootless and rootful Podman keep SEPARATE
# image stores, so images built rootless are invisible to a rootful
# `podman compose up`, which then tries to *pull* `localhost/...` from a
# registry and fails with a connection-refused error. Building rootful puts the
# images in the same store the stack runs from. See docs/adr/0010.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib.sh
. "$HERE/lib.sh"
cd "$(repo_root)"
require_podman
ensure_root

log "building collector image (localhost/myinkbird-collector:latest) [rootful]"
as_root podman build -f Containerfile -t localhost/myinkbird-collector:latest .
log "building committer image (localhost/myinkbird-committer:latest) [rootful]"
as_root podman build -f Containerfile.committer -t localhost/myinkbird-committer:latest .
log "images built into the rootful store"
