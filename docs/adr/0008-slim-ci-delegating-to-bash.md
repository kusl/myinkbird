# 0008. Slim CI that delegates to bash scripts

- Status: Accepted
- Date: 2026-07-08

## Context

Continuous integration should verify the same things a contributor verifies
locally, and it should not become a pile of provider-specific YAML that only
runs on one CI service and cannot be reproduced on a laptop. The project wants
CI kept **as slim as possible**, with the real work living in scripts.

## Decision

- Keep the GitHub Actions workflow (`.github/workflows/ci.yml`) minimal: check
  out the code, restore a cache, and **call a bash script**. All real logic
  lives in `scripts/`.
- The single entry point CI calls is **`scripts/ci.sh`**, which installs system
  dependencies and then runs `scripts/lint.sh`, `scripts/build.sh`, and
  `scripts/test.sh`. A separate `supply-chain` job calls `scripts/deny.sh`.
- Grant the workflow least privilege (`permissions: contents: read`) and use
  concurrency cancellation so superseded runs stop early.
- Pin the actions used to explicit versions (see
  [ADR 0009](0009-always-latest-dependencies.md)).

## Consequences

- A contributor can run the *exact* CI pipeline locally with `./scripts/ci.sh`
  (and `./scripts/deny.sh`), so "works on my machine" and "passes CI" converge.
- Moving to (or adding) another CI provider is mostly a matter of calling the
  same scripts; little logic is trapped in the workflow file.
- The scripts are the source of truth for how the project is built, tested, and
  audited; they are documented in `CONTRIBUTING.md`.
