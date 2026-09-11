# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project aims to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Prebuilt cross-platform binaries.** `scripts/release-build.sh` builds a
  self-contained `inkbird-collector` for the host platform and packages it as a
  `.tar.gz` with a SHA-256 checksum (and the `LICENSE`) into `dist/`;
  `scripts/release-publish.sh` publishes it via the GitHub CLI. A new
  `.github/workflows/release.yml` runs the build on a Linux/macOS/Windows matrix
  and cuts a **full** GitHub release (not a pre-release) on every push to
  `main`, tagged `v<version>-r<run-number>`. The workflow stays slim and defers
  to the scripts (see [ADR 0008](docs/adr/0008-slim-ci-delegating-to-bash.md)).
  See [ADR 0011](docs/adr/0011-prebuilt-release-binaries.md) and
  [docs/releases.md](docs/releases.md).
- **Default data-directory resolution for standalone runs** (a new `data_dir`
  module in the collector). With no `--data-dir` and no `INKBIRD_DATA_DIR`, the
  binary writes to an XDG-style per-user directory (`XDG_DATA_HOME` or the
  platform default: `~/.local/share/myinkbird`,
  `~/Library/Application Support/myinkbird`, or `%APPDATA%\myinkbird`), else the
  directory containing the executable, else it prints readings to standard
  output. The resolution is pure and unit-tested.
- **`StdoutSink`** - a `ReadingSink` that prints readings as NDJSON to standard
  output, used as the last-resort fallback in the resolution above.
- `scripts/collect-local.sh` - run the collector directly on the host (no
  containers) and write readings to the visible `./data` directory; the quickest
  way to confirm the sensor is seen and readings are being written.
- `scripts/logs.sh` - follow the running stack's logs (rootful).

### Changed

- **`INKBIRD_DATA_DIR` / `--data-dir` is now optional.** When unset it triggers
  the default resolution described above; an empty value is treated as "unset"
  (so a stray `INKBIRD_DATA_DIR=""` no longer writes to `./readings` in the
  current directory). The container still sets `INKBIRD_DATA_DIR=/data`
  explicitly, so its behaviour is unchanged.
- **Readings are stored on the host, not in a named volume.** `compose.yaml`
  bind-mounts a host directory (default `./data`, override with
  `INKBIRD_HOST_DATA_DIR`) into both containers at `/data`, so the NDJSON files
  and their local git history are directly visible and browsable on the host.
  See [ADR 0006](docs/adr/0006-local-git-ndjson-storage.md).
- Scripts that need root (`run.sh`, `stop.sh`, `logs.sh`, `container-build.sh`,
  `collect-local.sh`) now acquire `sudo` **once, up front**, via `ensure_root`
  in `scripts/lib.sh`, and keep the credential alive in the background for the
  duration of the run rather than prompting partway through.
- `deny.toml`: silence "unused allowed license" warnings
  (`unused-allowed-license = "allow"`) and set `wildcards = "deny"` with
  `allow-wildcard-paths = true`; the workspace `inkbird-core` dependency now
  carries a concrete `version`, removing the wildcard/unresolved-workspace
  diagnostics.

### Fixed

- **Stack now starts.** The images were built rootless but the stack runs
  rootful (required for BlueZ's D-Bus policy), and the two use separate image
  stores. A rootful `podman compose up` therefore could not find
  `localhost/myinkbird-*:latest` and failed trying to pull it from a
  registry named `localhost` (`connection refused`). `scripts/container-build.sh`
  now builds **rootful**, and `compose.yaml` sets `pull_policy: never` on both
  services. See [ADR 0010](docs/adr/0010-build-images-rootful.md).
- **Empty `INKBIRD_ADDRESS` no longer suppresses all readings.** Compose passes
  an unset address through as the empty string; the collector treated that as a
  real (empty) address filter and matched nothing. `Config` now normalises an
  empty or whitespace-only address to "no filter" and falls back to name
  matching. Added unit tests.
- **cargo-deny advisories pass.** The dependency graph pinned `time` 0.3.45,
  which carries RUSTSEC-2026-0009 (RFC 2822 parsing stack-exhaustion DoS).
  `Cargo.lock` is now committed and pins `time` >= 0.3.47, deterministically
  clearing the advisory (and closing a reproducibility gap - the lock was
  previously untracked). The `Containerfile` copies `Cargo.lock` so the image
  builds the same pinned versions.

## [0.1.0] - 2026-07-08

Initial scaffold.

### Added

- `inkbird-core` crate: pure, I/O-free decoder for INKBIRD ITH-13-B
  advertisements, returning a validated `SensorReading` or a descriptive
  `ParseError`. Exhaustive unit tests cover the byte layout, boundary values,
  corrupt-packet rejection, negative temperatures, zero-humidity omission, and
  name matching.
- `inkbird-collector` binary: a listen-only BLE collector built on `btleplug`.
  - `collect` subcommand (default): decode matching advertisements and append
    validated readings to per-day NDJSON files.
  - `discover` subcommand: scan for a fixed time and print every device seen
    with an attempted decode, to find a sensor's address and confirm its layout.
  - Per-device throttling that always records real changes but suppresses
    unchanged readings within a configurable interval (pure and unit-tested).
  - Graceful shutdown on Ctrl-C / SIGTERM.
  - `ReadingSink` trait with an NDJSON implementation and an in-memory test
    double.
- Containerisation with Podman: `Containerfile` (collector) and
  `Containerfile.committer` (git committer sidecar), plus a `compose.yaml`
  (Compose Spec, no build section) wiring both services and a shared
  `inkbird-data` volume.
- A committer sidecar (`scripts/commit-loop.sh`) that stores readings in a
  **local** git repository and never pushes.
- Helper scripts for every task (`build`, `test`, `lint`, `ci`, `deny`,
  `container-build`, `run`, `stop`, `install-system-deps`) sharing a common
  `lib.sh`.
- Slim GitHub Actions CI that only checks out, caches, and calls the bash
  scripts; a separate `cargo-deny` supply-chain job.
- Supply-chain policy via `deny.toml` (licence allow-list, advisory and yanked
  checks, crates.io-only sources).
- Documentation: `README.md`, `docs/architecture.md`, `docs/bluetooth.md`,
  `docs/data-format.md`, and Architecture Decision Records `0001`–`0009` under
  `docs/adr/`.
- Project licensed under AGPL-3.0-or-later.

[Unreleased]: https://github.com/kusl/myinkbird/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/kusl/myinkbird/releases/tag/v0.1.0
