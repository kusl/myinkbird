# 0006. Store readings as NDJSON in a local git repository

- Status: Accepted
- Date: 2026-07-08 (updated 2026-07-09: store on the host, not in a named volume)

## Context

We need to persist a stream of small, timestamped readings durably and in a way
that is easy to inspect and reason about, without standing up a database or
depending on any external service. The project also explicitly wants the data
kept in a **git repository**, kept **local** (this is a personal sensor's data;
it should not leave the machine unless the owner chooses), and - importantly -
**visible on the host**: the whole point is to be able to `cd` into the data
directory and browse the files and their git history directly, not to have them
buried inside an opaque container-managed volume.

## Decision

- Store readings as **NDJSON** (newline-delimited JSON): one JSON object per
  line, partitioned into **per-day files** at `<data>/readings/<YYYY-MM-DD>.ndjson`.
- Append only. A `throttle` keeps unchanged readings from flooding the files
  while always recording real changes (see `docs/data-format.md`).
- Keep the data on the **host filesystem** via a **bind mount**, not a named
  Podman volume. `compose.yaml` bind-mounts a host directory into both
  containers at `/data`; the directory defaults to `./data` next to
  `compose.yaml` and is overridable with `INKBIRD_HOST_DATA_DIR`. This is what
  makes the readings and their git history directly visible to you.
- A separate **committer** sidecar `git init`s that directory as a **local**
  repository and commits new lines on an interval. It **never runs
  `git push`** and configures no remote.
- Keep the *data* repository entirely separate from this *source* repository.
  The default `./data` path is git-ignored in the code repo, so sensor data (a
  distinct, nested git repo) is never accidentally committed to the code repo.

## Consequences

- The NDJSON files and the local git repo live at `./data/` (by default) where
  you can `ls`, `grep`, `jq`, and `git log -p` them directly - no throwaway
  container needed to reach the data.
- NDJSON is append-friendly, human-readable, and yields clean line-oriented
  diffs, so git history is a natural, greppable audit trail of how each day's
  file grew. Because commits only occur when there is new data, the commit log
  doubles as a coarse record of when the sensor was active.
- The collector runs as root (rootful, for BlueZ - see ADR 0005), so files it
  writes into `./data` are **root-owned** (world-readable with the default
  umask). To browse the git history as your normal user you may need to mark it
  safe once: `git config --global --add safe.directory "$PWD/data"`, or use
  `sudo git -C ./data log`. To take ownership after stopping the stack:
  `sudo chown -R "$USER:$USER" ./data`.
- The data stays on the machine by default. Exporting it is a deliberate,
  manual act (copy the files, or add a remote yourself). Pushing to a remote,
  building a dashboard, or streaming to a time-series database are possible
  future extensions, intentionally not built in.
