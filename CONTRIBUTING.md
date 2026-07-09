# Contributing to myinkbird

Thanks for taking a look. This project is public mostly as a learning artifact,
but issues and pull requests are welcome.

## A note on how this project is developed

Parts of this project were written with the assistance of a large language
model (see the disclosure in the [README](README.md)). If you contribute,
please review your changes yourself for correctness and licensing just as you
would any other code, and say so in your PR if you used an AI assistant.

## Prerequisites

- **Rust** (latest stable; edition 2024 needs >= 1.85). Install via
  <https://rustup.rs>. The `rust-toolchain.toml` pins the channel to stable.
- **A C toolchain, pkg-config, and D-Bus development headers** to build
  `btleplug`'s Linux backend. `scripts/install-system-deps.sh` installs these
  for the common package managers, or install them yourself:
  - Fedora: `sudo dnf install gcc pkgconf-pkg-config dbus-devel`
  - Debian/Ubuntu: `sudo apt-get install gcc pkg-config libdbus-1-dev`
- **Podman** to build and run the containers (only needed for the container
  workflow, not for `cargo test`).
- To actually receive readings you need a Bluetooth adapter and a running
  `bluetoothd`; see [docs/bluetooth.md](docs/bluetooth.md).

## The scripts are the source of truth

All build/test/lint/audit logic lives in `scripts/` so that what you run locally
is exactly what CI runs (see [ADR 0008](docs/adr/0008-slim-ci-delegating-to-bash.md)).
Each script is small and sources `scripts/lib.sh`. Scripts that need root
acquire `sudo` once, up front, via `ensure_root` (in `lib.sh`) and keep it alive
for the whole run.

| Script                        | What it does                                              |
| ----------------------------- | --------------------------------------------------------- |
| `scripts/install-system-deps.sh` | Install the C compiler, pkg-config, and D-Bus headers  |
| `scripts/build.sh`            | `cargo build --workspace --release`                       |
| `scripts/test.sh`             | `cargo test --workspace`                                  |
| `scripts/lint.sh`             | `cargo fmt --check` and `cargo clippy -D warnings`        |
| `scripts/deny.sh`             | `cargo deny check` (licences, advisories, sources)        |
| `scripts/ci.sh`               | The full local pipeline: deps → lint → build → test       |
| `scripts/container-build.sh`  | Build both OCI images with `podman build` (**rootful**)   |
| `scripts/run.sh`              | Build images and start the stack (rootful)                |
| `scripts/logs.sh`             | Follow the running stack's logs (rootful)                 |
| `scripts/stop.sh`             | Stop and remove the stack                                 |
| `scripts/collect-local.sh`    | Run the collector natively (no containers) → `./data`     |

Before opening a PR, please run:

```bash
./scripts/lint.sh
./scripts/test.sh
./scripts/deny.sh   # if you touched dependencies
```

or simply `./scripts/ci.sh` to run the same sequence CI does.

## Project layout

```
crates/inkbird-core/       pure decoder library (no I/O, heavily unit-tested)
crates/inkbird-collector/  BLE listener + NDJSON writer binary
scripts/                   all build/test/run logic
docs/                      architecture, bluetooth, data-format, and ADRs
Containerfile*             OCI image recipes (vendor-neutral naming)
compose.yaml               Compose Spec wiring the two services
```

## Coding conventions

- **Keep `inkbird-core` pure.** No async, no I/O, no Bluetooth, no
  serialization. New decoding rules belong here, with unit tests, so they can be
  tested without hardware.
- **Depend on abstractions in the collector.** New storage backends should
  implement the `ReadingSink` trait rather than being wired directly into the
  scanner.
- **Never connect to the sensor.** The battery guarantee depends on
  listen-only operation (see [ADR 0003](docs/adr/0003-listen-only-never-connect.md)).
  Do not add code that opens a GATT connection to the sensor.
- **Formatting and lints.** Code must pass `cargo fmt --check` and
  `cargo clippy -D warnings`.
- **Tests.** Add tests for new logic; prefer pure, hardware-free tests. The
  existing tests inject time (`Instant`) and use `tempfile` directories rather
  than touching real global state.

## Dependencies and licensing

- The project is **AGPL-3.0-or-later**. New dependencies must use a permissive,
  commercial-use-OK, AGPL-compatible licence on the `deny.toml` allow-list.
- Avoid nagware and "non-commercial only" licences entirely (see
  [ADR 0007](docs/adr/0007-agpl-and-dependency-policy.md)).
- If you add a dependency with a licence not yet on the allow-list, update
  `deny.toml` in the same PR and explain why in the description. `cargo-deny`
  will fail CI otherwise.
- `Cargo.lock` is committed (see [ADR 0009](docs/adr/0009-always-latest-dependencies.md));
  when you change dependencies, commit the updated lock, and keep it current
  with `cargo update` so the advisory check stays green.

## Vendor-neutral tooling

Please keep the tooling vendor-neutral: use **Podman**, name image recipes
**`Containerfile`** (the vendor-neutral OCI name), and do not add a `build:`
section to `compose.yaml` (images are built by `scripts/container-build.sh`).
See [ADR 0004](docs/adr/0004-podman-and-containerfile-only.md).

## Documentation and decisions

This project documents as it goes. When you make a change, please update the
relevant docs in the **same PR**:

- user-facing behaviour → `README.md`;
- how something works → the relevant file under `docs/`;
- the on-disk format → `docs/data-format.md`;
- a **significant decision** → add or update an ADR under `docs/adr/`
  (see [docs/adr/README.md](docs/adr/README.md) for the format), and add a
  `CHANGELOG.md` entry under `[Unreleased]`.

## The `export.sh` helper

The repository includes an `export.sh` that dumps the git-tracked source into
`docs/llm/dump.txt` for feeding into an AI assistant. It is a developer
convenience; You do not need it for normal development.
