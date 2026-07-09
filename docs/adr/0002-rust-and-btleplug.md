# 0002. Rust with btleplug for the BLE collector

- Status: Accepted
- Date: 2026-07-08

## Context

The collector is a long-running process that sits next to a Bluetooth radio and
decodes binary advertisement packets. We want:

- a **compiled** language (the collector should be a small, self-contained
  binary with no interpreter or heavy runtime to ship in a container);
- strong correctness guarantees for byte-level parsing;
- a maintained, permissively licensed BLE library that works with Linux/BlueZ;
- good testability for the decode logic.

Rust was the stated first preference for this project, and a scripting language
(e.g. Python) was to be avoided unless nothing else fit.

## Decision

Implement the project in **Rust** as a Cargo workspace, and use **`btleplug`**
for Bluetooth Low Energy.

- `btleplug` is cross-platform and, on Linux, is a client of BlueZ over D-Bus,
  which fits our "talk to the host bluetoothd" architecture
  (see [ADR 0005](0005-bluetooth-via-host-bluez-dbus.md)).
- Its licence (MIT / Apache-2.0 / BSD-3-Clause) is permissive and compatible
  with distributing this project under AGPL-3.0-or-later
  (see [ADR 0007](0007-agpl-and-dependency-policy.md)).
- Async is handled with `tokio`; streams with `futures`; logging with
  `tracing`; serialization with `serde`/`serde_json`; timestamps with `time`;
  CLI with `clap`. All are mainstream, maintained, permissively licensed crates.

The decode logic is isolated in a separate, dependency-light `inkbird-core`
crate so it can be unit-tested without any Bluetooth, async, or I/O.

## Consequences

- We ship a single stripped binary; the runtime image only needs `libdbus`.
- Parsing correctness is enforced by the type system and an exhaustive unit-test
  suite.
- `btleplug` uses BlueZ's classic active discovery rather than the
  `AdvertisementMonitor` passive API; the battery implications are addressed in
  [ADR 0003](0003-listen-only-never-connect.md) and `docs/bluetooth.md`.
- Rust's edition 2024 requires a recent toolchain (>= 1.85); see
  [ADR 0009](0009-always-latest-dependencies.md).
