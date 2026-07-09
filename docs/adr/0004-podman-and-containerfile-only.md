# 0004. Podman and Containerfiles; vendor-neutral tooling

- Status: Accepted
- Date: 2026-07-08

## Context

A requirement of this project is that **nothing is installed on the host** other
than the container engine itself: all build tooling and all runtime code live
inside containers. The project should also be vendor-neutral about container
tooling and forward-looking (ready for current Podman releases as they land).

## Decision

- Use **Podman** (and Podman's Compose support) as the container engine. The
  helper scripts require `podman` and try `podman compose` first, falling back
  to `podman-compose`.
- Name build recipes **`Containerfile`** (and `Containerfile.committer`), the
  vendor-neutral OCI name, rather than any engine-specific build-file name.
- The Compose file (`compose.yaml`) contains **no `build:` section**. Images are
  built ahead of time by `scripts/container-build.sh` with `podman build`, and
  the Compose file only references the prebuilt `localhost/...` images. This
  keeps the Compose file free of any build-tool-specific keys.
- Avoid tying image references to a single registry vendor: base images are
  pulled from `registry.fedoraproject.org` rather than a vendor default.

## Consequences

- The host stays clean; everything reproducible lives in the repo and in images.
- The stack is not coupled to any one container CLI's proprietary features.
- Building images is an explicit step (`scripts/container-build.sh`, invoked by
  `scripts/run.sh`) rather than an implicit side effect of `compose up`. This is
  a small amount of extra ceremony in exchange for a clean, portable Compose
  file.
- Base images track the current Fedora release; see
  [ADR 0009](0009-always-latest-dependencies.md).
