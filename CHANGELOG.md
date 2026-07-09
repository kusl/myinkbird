# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project aims to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
