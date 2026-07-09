#!/usr/bin/env bash
# Committer sidecar entry point. Initialises /data as a *local* git repo (if
# needed) and commits any new readings on an interval. It never pushes - the
# data stays local (see docs/adr/0006).
set -euo pipefail

DATA_DIR="${INKBIRD_DATA_DIR:-/data}"
INTERVAL="${COMMIT_INTERVAL_SECS:-300}"
AUTHOR_NAME="${GIT_AUTHOR_NAME:-myinkbird}"
AUTHOR_EMAIL="${GIT_AUTHOR_EMAIL:-myinkbird@localhost}"

# The volume may be owned by a different uid than this process; mark it safe.
git config --global --add safe.directory "$DATA_DIR"

cd "$DATA_DIR"
if [ ! -d .git ]; then
  echo "committer: initialising git repository in $DATA_DIR"
  git init -q
fi
git config user.name "$AUTHOR_NAME"
git config user.email "$AUTHOR_EMAIL"

echo "committer: watching $DATA_DIR, committing every ${INTERVAL}s (no push)"
while true; do
  sleep "$INTERVAL"
  git add -A
  if ! git diff --cached --quiet; then
    ts="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    git commit -q -m "readings: $ts"
    echo "committer: committed at $ts"
  fi
done
