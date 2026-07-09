# Architecture Decision Records

This directory holds the Architecture Decision Records (ADRs) for `myinkbird`.

An ADR captures a single significant decision: its context, the choice made,
and the consequences. Recording them keeps the reasoning behind the code
discoverable long after the conversation that produced it is gone, and makes it
easy to revisit a decision deliberately rather than by accident.

The format here is intentionally lightweight, in the spirit of Michael Nygard's
original ADR write-up and the MADR template. For background see
<https://adr.github.io/>.

## Conventions

- One decision per file, numbered sequentially: `NNNN-short-title.md`.
- Numbers are never reused. A superseded decision keeps its file; a new ADR
  supersedes it and both link to each other.
- Status is one of: `Proposed`, `Accepted`, `Superseded by ADR-XXXX`,
  `Deprecated`.
- Keep them short. The code and the other `docs/` files hold the detail; an ADR
  holds the *why*.

## Index

| ADR                                                        | Title                                             | Status   |
| ---------------------------------------------------------- | ------------------------------------------------- | -------- |
| [0001](0001-record-architecture-decisions.md)              | Record architecture decisions                     | Accepted |
| [0002](0002-rust-and-btleplug.md)                          | Rust with btleplug for the BLE collector          | Accepted |
| [0003](0003-listen-only-never-connect.md)                  | Listen to advertisements only; never connect      | Accepted |
| [0004](0004-podman-and-containerfile-only.md)              | Podman and Containerfiles; vendor-neutral tooling | Accepted |
| [0005](0005-bluetooth-via-host-bluez-dbus.md)              | Bluetooth via the host BlueZ D-Bus, run rootful   | Accepted |
| [0006](0006-local-git-ndjson-storage.md)                   | Store readings as NDJSON in a local git repo      | Accepted |
| [0007](0007-agpl-and-dependency-policy.md)                 | AGPL-3.0-or-later and a free-dependency policy     | Accepted |
| [0008](0008-slim-ci-delegating-to-bash.md)                 | Slim CI that delegates to bash scripts            | Accepted |
| [0009](0009-always-latest-dependencies.md)                 | Track the latest toolchain and dependencies       | Accepted |
