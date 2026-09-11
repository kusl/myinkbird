# 0011. Ship prebuilt cross-platform binaries via GitHub Releases

- Status: Accepted
- Date: 2026-09-11

## Context

Until now the only ways to run the collector were to build it from source or to
build and run the container stack. Both need a toolchain (or Podman) and some
setup. For someone who just wants to point the tool at their sensor on a laptop,
a single downloadable executable is far lower friction.

A prebuilt binary also has to answer a question the container never did: **where
does it write its readings** when nobody has bind-mounted `/data` or set
`INKBIRD_DATA_DIR`? It should pick a sensible, conventional location on each OS,
and degrade gracefully if that location is not writable.

## Decision

- **Publish prebuilt `inkbird-collector` binaries for Linux, macOS, and
  Windows** on GitHub Releases.
- **Build natively on a CI matrix** (`ubuntu-latest`, `macos-latest`,
  `windows-latest`) rather than cross-compiling. `btleplug` uses a different
  Bluetooth backend per OS (BlueZ/D-Bus on Linux, CoreBluetooth on macOS, WinRT
  on Windows), so a native build per platform is far simpler and more reliable
  than cross-compiling those backends.
- **Keep all real logic in bash scripts** (consistent with
  [ADR 0008](0008-slim-ci-delegating-to-bash.md)):
  - `scripts/release-build.sh` builds the release binary for the host, then
    packages it as a `.tar.gz` with a `.sha256` sidecar (and the `LICENSE`) into
    `dist/`. The same script runs locally and in each matrix leg.
  - `scripts/release-publish.sh` creates/updates the release and uploads
    `dist/*` via the GitHub CLI (`gh`).
  - `.github/workflows/release.yml` only checks out, caches, and calls those
    scripts.
- **A full release on every push to `main`.** Each push produces one GitHub
  release marked as the latest **full** release (not a pre-release). The tag is
  derived from the workspace version plus the CI run number, e.g.
  `v0.1.0-r42`, so every push yields a unique release.
- **Default data-directory resolution for a standalone run** (in the collector's
  `data_dir` module), used only when neither `--data-dir` nor `INKBIRD_DATA_DIR`
  is set:
  1. an XDG-style per-user data directory - `XDG_DATA_HOME/myinkbird`, else the
     platform default (`~/.local/share/myinkbird` on Linux,
     `~/Library/Application Support/myinkbird` on macOS,
     `%APPDATA%\myinkbird` on Windows);
  2. otherwise the directory containing the executable;
  3. otherwise print readings to standard output so they are still visible.
  The container is unaffected: it sets `INKBIRD_DATA_DIR=/data` explicitly, and
  an empty value is treated as "unset".

## Consequences

- **The binary is self-contained but not fully static.** All Rust crates are
  linked in. On Linux it still dynamically links the system C library and
  `libdbus` - both present on any desktop that has Bluetooth - so it is not a
  from-scratch static binary. macOS and Windows need no extra runtime libraries.
- **The battery guarantee is unchanged.** This is the same listen-only code
  (see [ADR 0003](0003-listen-only-never-connect.md)); packaging it differently
  changes nothing about how it talks to the sensor.
- **The data location differs from the container.** A standalone run writes to a
  per-user directory (or falls back), not `/data`. This is documented in
  [docs/releases.md](../releases.md) and
  [docs/data-format.md](../data-format.md).
- **Many releases accumulate.** Releasing on every push (in the spirit of
  [ADR 0009](0009-always-latest-dependencies.md)'s always-current stance) trades
  a tidy release list for always having downloadable binaries for the latest
  `main`. Old releases can be pruned if desired.
- **Licensing.** The binaries are covered by AGPL-3.0-or-later like the source;
  the archive ships the `LICENSE` text, and the corresponding source is this
  repository at the released commit.
- **No cross-compilation.** Building only for the three runner platforms means
  no binaries for other targets (e.g. ARM Linux) yet; a contributor who needs
  one can run `scripts/release-build.sh` on that platform.
