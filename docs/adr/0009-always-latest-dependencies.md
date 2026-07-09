# 0009. Track the latest toolchain and dependencies

- Status: Accepted
- Date: 2026-07-08

## Context

This is a learning-oriented project, and the maintainer's preference is to stay
current: latest stable Rust, latest crate versions, latest GitHub Actions, and
current base container images, rather than pinning to old versions for their own
sake. Staying current means fewer known bugs, up-to-date security fixes, and
exposure to current idioms.

## Decision

- Target the **latest stable Rust** via `rust-toolchain.toml`
  (`channel = "stable"`), using edition 2024 (which requires Rust >= 1.85, set
  as `rust-version`).
- Depend on current crate versions, expressed as caret requirements in the
  workspace `[workspace.dependencies]` (e.g. `tokio = "1"`, `btleplug = "0.12"`)
  so compatible updates are picked up.
- Pin **GitHub Actions** to explicit current versions
  (`actions/checkout@v7.0.0`, `actions/cache@v6.1.0`) for supply-chain clarity,
  and bump them as new versions ship.
- Track the **current Fedora** base images
  (`registry.fedoraproject.org/fedora:latest` and `fedora-minimal:latest`).

## Consequences

- The project benefits from the newest fixes and idioms, and stays a faithful
  example of current practice.
- **Reproducibility trade-off:** `latest`-style references mean an image built
  today and one built months from now may differ. `Cargo.lock` is committed, so
  the Rust dependency graph is reproducible for the binary; but base-image
  contents and freshly resolved caret updates can move. For this project's goals
  (a personal, always-current tool) that trade-off is acceptable. A consumer who
  needs bit-for-bit reproducibility can pin base-image digests and use
  `--locked` builds.
- Keeping current is **ongoing work**: dependencies, actions, and images should
  be refreshed periodically, and `cargo-deny` (see
  [ADR 0007](0007-agpl-and-dependency-policy.md)) guards against a refresh
  silently introducing a bad licence or a known-vulnerable crate.
