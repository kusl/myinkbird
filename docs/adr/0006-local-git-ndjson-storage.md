# 0006. Store readings as NDJSON in a local git repository

- Status: Accepted
- Date: 2026-07-08

## Context

We need to persist a stream of small, timestamped readings durably and in a way
that is easy to inspect and reason about, without standing up a database or
depending on any external service. The project also explicitly wants the data
kept in a **git repository**, and kept **local** (this is a personal sensor's
data; it should not leave the machine unless the owner chooses).

## Decision

- Store readings as **NDJSON** (newline-delimited JSON): one JSON object per
  line, partitioned into **per-day files** at
  `/data/readings/<YYYY-MM-DD>.ndjson`.
- Append only. A `throttle` keeps unchanged readings from flooding the files
  while always recording real changes (see `docs/data-format.md`).
- A separate **committer** sidecar `git init`s the data directory as a **local**
  repository and commits new lines on an interval. It **never runs
  `git push`** and configures no remote.
- Keep the *data* repository entirely separate from this *source* repository.
  The data volume is git-ignored here so sensor data is never accidentally
  committed to the code repo.

## Consequences

- NDJSON is append-friendly, human-readable, and yields clean line-oriented
  diffs, so git history is a natural, greppable audit trail of how each day's
  file grew.
- Any standard tool (`grep`, `jq`, `git log -p`) can inspect the data; no schema
  migration machinery is needed.
- Because commits only occur when there is new data, the commit log doubles as a
  coarse record of when the sensor was active.
- The data stays on the machine by default. Exporting it is a deliberate,
  manual act (copy the files, or add a remote yourself). Pushing to a remote,
  building a dashboard, or streaming to a time-series database are possible
  future extensions, intentionally not built in.
