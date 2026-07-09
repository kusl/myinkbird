# myinkbird

A small, low-power collector that captures temperature, humidity, and battery
readings from an **INKBIRD ITH-13-B** Bluetooth thermo-hygrometer by *listening*
to the advertisements it already broadcasts — never connecting to it — and
stores those readings as NDJSON in a **local git repository**.

Everything runs in containers via **Podman**. Nothing is installed on the host
except the container engine.

> **Primary design goal: add zero extra battery drain to the sensor.** The
> ITH-13-B runs on two AAA cells and broadcasts its readings over Bluetooth Low
> Energy whether or not anyone is listening. This project only ever listens; it
> never opens a connection. See [the battery section](#why-this-doesnt-drain-the-sensors-battery).

## Contents

- [How it's built with AI assistance (please read)](#how-its-built-with-ai-assistance-please-read)
- [Hardware](#hardware)
- [How it works](#how-it-works)
- [Quick start](#quick-start)
- [Configuration](#configuration)
- [The data](#the-data)
- [Why this doesn't drain the sensor's battery](#why-this-doesnt-drain-the-sensors-battery)
- [Development](#development)
- [Documentation](#documentation)
- [License](#license)
- [Roadmap](#roadmap)

## How it's built with AI assistance (please read)

**This project was developed with substantial help from a large language model
(an AI assistant).** The AI proposed the architecture, wrote most of the Rust
code, the container and Compose files, the helper scripts, and this
documentation. A human directed the work, set the requirements, and is
responsible for reviewing and running it — but you should be aware that a large
fraction of the content here was machine-generated.

Practical implications for you as a reader or user:

- **Review before you trust it.** Read the code and the decode logic yourself
  before relying on it, especially anything touching your Bluetooth stack or
  running with `sudo`.
- **The decoder was validated against a reference implementation and unit
  tests, but not, in this repository's history, end-to-end against physical
  hardware in the same environment that produced it.** Use the `discover`
  subcommand to confirm the byte layout matches *your* unit (see
  [docs/bluetooth.md](docs/bluetooth.md)).
- **Bugs are possible.** Treat it as a starting point and a learning example,
  not a turnkey appliance.

This disclosure is deliberate and errs on the side of over-communicating. If you
build on this project, please carry a similar disclosure forward.

## Hardware

- **Sensor:** INKBIRD ITH-13-B wireless thermo-hygrometer (temperature +
  relative humidity + battery), powered by 2×AAA, communicating over Bluetooth
  Low Energy.
- **Host:** developed against Fedora on an Acer Swift Go 14 (AMD Ryzen 8845HS),
  but nothing is machine-specific: any Linux host with a Bluetooth adapter,
  a running `bluetoothd`, and Podman should work.

The collector identifies the sensor by its Bluetooth address (recommended) or by
a substring of its advertised name (`ith-13-b` by default).

## How it works

```text
   ┌────────────────────┐
   │  INKBIRD ITH-13-B   │  broadcasts BLE advertisements
   │  (2×AAA battery)    │  (temperature, humidity, battery %)
   └─────────┬──────────┘
             │  radio  (listen-only; the sensor is never connected to)
             ▼
   ┌────────────────────┐
   │  Host BlueZ         │  bluetoothd owns the adapter
   │  (bluetoothd)       │
   └─────────┬──────────┘
             │  D-Bus system socket (bind-mounted into the container)
             ▼
┌──────────────────────────────────────────────┐
│  collector container (Rust, btleplug)         │
│    scanner → decode (inkbird-core)            │
│           → throttle → append NDJSON           │
└───────────────────┬───────────────────────────┘
                    │  /data/readings/YYYY-MM-DD.ndjson
                    ▼
        ┌───────────────────────────┐
        │  inkbird-data volume       │
        └───────────┬───────────────┘
                    │  git add / commit on an interval (never pushed)
                    ▼
        ┌───────────────────────────┐
        │  committer container       │
        │  local git repo in /data   │
        └───────────────────────────┘
```

Two crates and two containers:

- **`inkbird-core`** — a pure, I/O-free Rust library that decodes an ITH-13-B
  advertisement into a validated reading. Thoroughly unit-tested, no hardware
  required.
- **`inkbird-collector`** — the binary that listens via `btleplug`, decodes,
  de-duplicates, and writes NDJSON.
- **collector container** — runs the binary in `collect` mode.
- **committer container** — periodically commits the data to a local git repo.

See [docs/architecture.md](docs/architecture.md) for the full picture.

## Quick start

You need Podman and a working host Bluetooth stack (`systemctl status bluetooth`).

```bash
# 1. Build both container images.
./scripts/container-build.sh

# 2. Find your sensor's Bluetooth address (scans for 30s and prints devices).
sudo podman run --rm \
  --security-opt label=disable \
  -v /run/dbus/system_bus_socket:/run/dbus/system_bus_socket \
  localhost/myinkbird-collector:latest discover --seconds 30

# 3. Configure. Copy the example env and set INKBIRD_ADDRESS to the address
#    you found in step 2.
cp .env.example .env
$EDITOR .env

# 4. Start the stack (rootful — see the Bluetooth note below).
./scripts/run.sh

# Follow the logs:
sudo podman compose logs -f
```

Stop it with `./scripts/stop.sh`.

**Why `sudo`?** The container reaches Bluetooth by talking to the host's
`bluetoothd` over the D-Bus system socket, and BlueZ's default policy only
accepts calls from `root`. Running the stack rootful satisfies that policy. The
full explanation, the SELinux note (`label=disable`), and troubleshooting are in
[docs/bluetooth.md](docs/bluetooth.md).

## Configuration

Configuration is via environment variables (read from `.env` by the stack) or
equivalent CLI flags. Copy [`.env.example`](.env.example) to `.env` to start.

| Variable                    | Default             | Meaning                                                                 |
| --------------------------- | ------------------- | ----------------------------------------------------------------------- |
| `RUST_LOG`                  | `info`              | Log verbosity (`error`/`warn`/`info`/`debug`/`trace`).                  |
| `INKBIRD_ADDRESS`           | *(empty)*           | Sensor Bluetooth address, e.g. `AA:BB:CC:DD:EE:FF`. **Recommended.** Empty → match by name. |
| `INKBIRD_NAME_MATCH`        | `ith-13-b`          | Case-insensitive name substring used when no address is set.            |
| `INKBIRD_MIN_INTERVAL_SECS` | `60`                | Minimum seconds between recorded readings for an *unchanged* sensor. Any change is always recorded. |
| `INKBIRD_DATA_DIR`          | `/data` (container) | Directory readings are written under (`<dir>/readings/*.ndjson`).       |
| `GIT_AUTHOR_NAME`           | `myinkbird`         | Author name for the committer's local commits.                          |
| `GIT_AUTHOR_EMAIL`          | `myinkbird@localhost` | Author email for the committer's local commits.                       |
| `COMMIT_INTERVAL_SECS`      | `300`               | How often the committer commits new readings (local only, never pushed).|

## The data

Readings are appended, one JSON object per line, to per-day files under the
`inkbird-data` volume at `/data/readings/<YYYY-MM-DD>.ndjson`. Example line:

```json
{"ts":"2026-07-08T21:03:44Z","address":"AA:BB:CC:DD:EE:FF","name":"ITH-13-B","model":"ITH-13-B","temperature_c":28.9,"humidity_pct":45.5,"battery_pct":100,"rssi_dbm":-61}
```

The committer keeps these files under version control in a **local** git repo in
the same volume and never pushes them anywhere. The full schema, decoding byte
layout, and tips for inspecting the data (with `jq` and `git log -p`) are in
[docs/data-format.md](docs/data-format.md).

## Why this doesn't drain the sensor's battery

Two ideas are easy to conflate; separating them is the whole point:

1. **We never connect.** Opening a Bluetooth (GATT) connection is what forces a
   coin-/AAA-powered sensor to spend real energy maintaining a link. This
   collector never connects — it only reads the advertisement packets the
   sensor already broadcasts on its own. Because the ITH-13-B puts temperature,
   humidity, **and** battery right in the advertisement, listening is all we
   need. This guarantee is unconditional.

2. **Active vs. passive *scanning* is a separate, minor matter.** The `btleplug`
   library drives BlueZ's classic active discovery, so *your computer's* radio
   may send scan-request probes. That is a cost on the scanning side, not on the
   sensor, and it is negligible for a broadcast-only device. Truly passive
   controller-level scanning (via BlueZ's `AdvertisementMonitor`) is possible
   and noted as future work, but it is not required to meet the battery goal.

The short version: **the battery is protected because we never connect.** See
[ADR 0003](docs/adr/0003-listen-only-never-connect.md) and
[docs/bluetooth.md](docs/bluetooth.md).

## Development

All build, test, lint, and audit logic lives in `scripts/`, so what you run
locally is exactly what CI runs.

```bash
./scripts/test.sh    # cargo test --workspace
./scripts/lint.sh    # fmt --check + clippy -D warnings
./scripts/ci.sh      # the full pipeline: deps → lint → build → test
./scripts/deny.sh    # supply-chain audit (licences, advisories)
```

You can also run the collector directly (outside a container) once built —
handy for `discover` during development. Building `btleplug` needs a C compiler,
`pkg-config`, and the D-Bus dev headers; `scripts/install-system-deps.sh`
installs them. See [CONTRIBUTING.md](CONTRIBUTING.md) for the full setup and
conventions.

## Documentation

- [docs/architecture.md](docs/architecture.md) — components and data flow.
- [docs/bluetooth.md](docs/bluetooth.md) — how the container reaches Bluetooth,
  rootful vs. rootless, SELinux, finding your sensor, and troubleshooting.
- [docs/data-format.md](docs/data-format.md) — NDJSON schema, byte layout, and
  how to inspect the data.
- [docs/adr/](docs/adr/README.md) — Architecture Decision Records (the *why*).

## License

Licensed under the **GNU Affero General Public License v3.0 or later**
(AGPL-3.0-or-later). See [LICENSE](LICENSE). This is a strong copyleft licence
with obligations that extend to network use of derivative works; that is
intentional. Dependencies are restricted to permissive, commercial-use-OK,
AGPL-compatible licences, enforced by `cargo-deny`
(see [ADR 0007](docs/adr/0007-agpl-and-dependency-policy.md)).

## Roadmap

Ideas, explicitly not yet built:

- Controller-level **passive scanning** via BlueZ `AdvertisementMonitor`.
- Optional export paths (push the data repo, or stream to a time-series
  database / dashboard) — kept out by default so data stays local.
- Support for additional INKBIRD models in `inkbird-core`.
- A rootless path documented via a custom BlueZ D-Bus policy.

Contributions welcome — see [CONTRIBUTING.md](CONTRIBUTING.md).
