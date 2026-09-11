#!/usr/bin/env bash
# Create (or update) a FULL GitHub release for the current commit and upload the
# packaged binaries in dist/. Uses the GitHub CLI (`gh`), which is available on
# GitHub runners and can also be used locally by a maintainer who has run
# `gh auth login`.
#
# The release is a full release (NOT a pre-release) and is marked as "latest".
# The tag is derived from the workspace version plus the CI run number, so each
# push to main produces a unique release (e.g. v0.1.0-r42). See docs/releases.md
# and docs/adr/0011.
#
# Environment:
#   GH_TOKEN / GITHUB_TOKEN  auth for gh (CI sets this to the workflow token)
#   RUN_NUMBER               unique per push; used in the tag (defaults to 0)
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/lib.sh
. "$HERE/lib.sh"
cd "$(repo_root)"

command -v gh >/dev/null 2>&1 || die "GitHub CLI (gh) not found; install it or run from CI"

version="$(workspace_version)"
[ -n "$version" ] || die "could not determine the workspace version"
run="${RUN_NUMBER:-${GITHUB_RUN_NUMBER:-0}}"
tag="v${version}-r${run}"
title="myinkbird ${version} (build ${run})"

[ -d dist ] || die "no dist/ directory; run scripts/release-build.sh first"
artifacts=()
for f in dist/*; do
  [ -e "$f" ] && artifacts+=("$f")
done
[ "${#artifacts[@]}" -gt 0 ] || die "dist/ is empty; nothing to upload"

commit="${GITHUB_SHA:-$(git rev-parse HEAD 2>/dev/null || echo unknown)}"
notes="$(cat <<EOF
Automated release built from commit \`${commit}\` on push to \`main\`.

Self-contained \`inkbird-collector\` binaries for Linux, macOS, and Windows,
each with a SHA-256 checksum. See [docs/releases.md](docs/releases.md) for
verification and usage, including where the binary stores its readings.
EOF
)"

log "publishing full release ${tag} with ${#artifacts[@]} artifact(s)"
if gh release view "$tag" >/dev/null 2>&1; then
  log "release ${tag} exists; updating its assets"
  gh release upload "$tag" "${artifacts[@]}" --clobber
else
  gh release create "$tag" "${artifacts[@]}" \
    --title "$title" \
    --notes "$notes" \
    --latest
fi
log "release ${tag} published"
