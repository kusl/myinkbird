# myinkbird

A small, low-power collector that captures temperature, humidity, and battery
readings from an **INKBIRD ITH-13-B** Bluetooth thermo-hygrometer by *listening*
to the advertisements it already broadcasts — never connecting to it — and
stores those readings as NDJSON in a **local git repository** on your machine.

You can run it from a **prebuilt binary** (nothing to install), as a
containerised stack via **Podman**, or straight from source with the Rust
toolchain.

> **Primary design goal: add zero extra battery drain to the sensor.** The
> ITH-13-B runs on two AAA cells and broadcasts its readings over Bluetooth Low
> Energy whether or not anyone is listening. This project only ever listens; it
> never opens a connection. See [the battery section](#why-this-doesnt-drain-the-sensors-battery).

## Contents

- [How it's built with AI assistance (please read)](#how-its-built-with-ai-assistance-please-read)
- [Hardware](#hardware)
- [How it works](#how-it-works)
- [Prebuilt binaries](#prebuilt-binaries)
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
  a running `bluetoothd`, and Podman should work. The prebuilt binaries also run
  on macOS and Windows.

The collector identifies the sensor by its Bluetooth address (recommended) or by
a substring of its advertised name (`ith-13-b` by default).

## How it works

```text
   ┌────────────────────┐
   │  INKBIRD ITH-13-B  │  broadcasts BLE advertisements
   │  (2×AAA battery)   │  (temperature, humidity, battery %)
   └─────────┬──────────┘
             │  radio  (listen-only; the sensor is never connected to)
             ▼
   ┌────────────────────┐
   │  Host BlueZ        │  bluetoothd owns the adapter
   │  (bluetoothd)      │
   └─────────┬──────────┘
             │  D-Bus system socket (bind-mounted into the container)
             ▼
┌──────────────────────────────────────────────┐
│  collector container (Rust, btleplug)        │
│    scanner → decode (inkbird-core)           │
│           → throttle → append NDJSON         │
└───────────────────┬──────────────────────────┘
                    │  ./data/readings/YYYY-MM-DD.ndjson   (on the host)
                    ▼
        ┌───────────────────────────┐
        │  host dir  ./data         │  bind-mounted into both containers
        └───────────┬───────────────┘
                    │  git add / commit on an interval (never pushed)
                    ▼
        ┌───────────────────────────┐
        │  committer container      │
        │  local git repo in ./data │
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

## Prebuilt binaries

Every push to `main` publishes a **full** GitHub release with a self-contained
`inkbird-collector` binary for **Linux, macOS, and Windows** — the
no-toolchain, no-container way to run the collector. Download the archive for
your platform from the
[latest release](https://github.com/kusl/myinkbird/releases/latest), verify its
checksum, unpack it, and run:

```bash
./inkbird-collector discover --seconds 30            # find your sensor
./inkbird-collector collect --address AA:BB:CC:DD:EE:FF
```

These binaries are the same listen-only program as the container build, so the
battery guarantee is unchanged. On Linux they still need a running `bluetoothd`
and `libdbus`, and usually `sudo` (see [docs/bluetooth.md](docs/bluetooth.md));
macOS uses CoreBluetooth and Windows uses WinRT, needing no extra libraries.

**Where a standalone binary stores readings.** With no `--data-dir` /
`INKBIRD_DATA_DIR` set, it writes to an XDG-style per-user directory
(`~/.local/share/myinkbird` on Linux, `~/Library/Application Support/myinkbird`
on macOS, `%APPDATA%\myinkbird` on Windows), falling back to the executable's
own directory, and finally to printing readings on screen if neither is
writable. A standalone binary writes the NDJSON files but does **not** run the
git committer. Checksum verification, the exact resolution order, and how to pin
a location are in [docs/releases.md](docs/releases.md).

## Quick start

You need a working host Bluetooth stack (`systemctl status bluetooth`).

### Option A — see it work fast, no containers

The quickest way to confirm your sensor is seen and readings are landing in a
file you can look at. Needs the Rust toolchain and the D-Bus build headers
(`./scripts/install-system-deps.sh` installs them):

```bash
./scripts/collect-local.sh
```

It builds the collector, asks for `sudo` **once** up front (BlueZ needs root),
and writes readings to `./data/readings/<date>.ndjson` — right in the repo,
where you can `tail -f` or `jq` them. Press Ctrl-C to stop.

(Prefer no build at all? Grab a [prebuilt binary](#prebuilt-binaries) instead.)

### Option B — the full containerised stack (collector + git committer)

You need Podman.

```bash
# 1. Find your sensor's Bluetooth address (scans for 30s and prints devices).
#    (Build the image first; it is built rootful — see the Bluetooth note below.)
./scripts/container-build.sh
sudo podman run --rm \
  --security-opt label=disable \
  -v /run/dbus/system_bus_socket:/run/dbus/system_bus_socket \
  localhost/myinkbird-collector:latest discover --seconds 30

# 2. Configure. Copy the example env and set INKBIRD_ADDRESS to the address
#    you found in step 1 (leave it blank to match by name instead).
cp .env.example .env
$EDITOR .env

# 3. Start the stack (asks for sudo once, up front — see the Bluetooth note).
./scripts/run.sh

# Follow the logs:
./scripts/logs.sh        # or: sudo podman compose logs -f
```

Stop it with `./scripts/stop.sh`.

**Why `sudo`?** The collector reaches Bluetooth by talking to the host's
`bluetoothd` over the D-Bus system socket, and BlueZ's default policy only
accepts calls from uid 0, so the collector runs as root; the images are built
rootful too so the stack can find them. The scripts request `sudo` **once at the
start** and keep the credential alive for the whole run, so you are not prompted
partway through. The full explanation, the SELinux note (`label=disable`), and
troubleshooting are in [docs/bluetooth.md](docs/bluetooth.md) and
[ADR 0010](docs/adr/0010-build-images-rootful.md).

## Configuration

Configuration is via environment variables (read from `.env` by the stack) or
equivalent CLI flags. Copy [`.env.example`](.env.example) to `.env` to start.

| Variable                    | Default             | Meaning                                                                 |
| --------------------------- | ------------------- | ----------------------------------------------------------------------- |
| `RUST_LOG`                  | `info`              | Log verbosity (`error`/`warn`/`info`/`debug`/`trace`).                  |
| `INKBIRD_HOST_DATA_DIR`     | `./data`            | **Host** directory the readings + local git repo live in (bind-mounted into the containers at `/data`). Relative to `compose.yaml`. |
| `INKBIRD_ADDRESS`           | *(empty)*           | Sensor Bluetooth address, e.g. `AA:BB:CC:DD:EE:FF`. **Recommended.** Empty → match by name (empty is *not* treated as an empty address filter). |
| `INKBIRD_NAME_MATCH`        | `ith-13-b`          | Case-insensitive name substring used when no address is set.            |
| `INKBIRD_MIN_INTERVAL_SECS` | `60`                | Minimum seconds between recorded readings for an *unchanged* sensor. Any change is always recorded. |
| `INKBIRD_DATA_DIR`          | *(container: `/data`)* | Directory readings are written under, *inside* the process (`<dir>/readings/*.ndjson`). The stack sets this to `/data`. **A standalone binary that leaves this unset resolves a default location** (see [Prebuilt binaries](#prebuilt-binaries)). |
| `GIT_AUTHOR_NAME`           | `myinkbird`         | Author name for the committer's local commits.                          |
| `GIT_AUTHOR_EMAIL`          | `myinkbird@localhost` | Author email for the committer's local commits.                       |
| `COMMIT_INTERVAL_SECS`      | `300`               | How often the committer commits new readings (local only, never pushed).|

## The data

Readings are appended, one JSON object per line, to per-day files. In the
container (and via `./scripts/collect-local.sh`) they land on the **host** at
`${INKBIRD_HOST_DATA_DIR:-./data}/readings/<YYYY-MM-DD>.ndjson` — a real
directory you can browse. (A standalone binary run outside the container writes
to a per-user data directory instead of `./data`; see
[Prebuilt binaries](#prebuilt-binaries) and
[docs/releases.md](docs/releases.md).) Example line:

```json
{"ts":"2026-07-08T21:03:44Z","address":"AA:BB:CC:DD:EE:FF","name":"ITH-13-B","model":"ITH-13-B","temperature_c":28.9,"humidity_pct":45.5,"battery_pct":100,"rssi_dbm":-61}
```

The committer keeps these files under version control in a **local** git repo in
the same directory and never pushes them anywhere:

```bash
cd ./data
git log --oneline
```

Because the collector runs as root, the files are root-owned; to `git` the data
repo as your user, run `git config --global --add safe.directory "$PWD/data"`
once (or use `sudo git -C ./data ...`). The full schema, decoding byte layout,
and inspection tips (`jq`, `git log -p`) are in
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
./scripts/test.sh          # cargo test --workspace
./scripts/lint.sh          # fmt --check + clippy -D warnings
./scripts/ci.sh            # the full pipeline: deps → lint → build → test
./scripts/deny.sh          # supply-chain audit (licences, advisories)
./scripts/collect-local.sh # run the collector natively → ./data (no containers)
./scripts/logs.sh          # follow the running stack's logs
./scripts/release-build.sh # build + package a downloadable binary → dist/
```

You can also run the collector directly (outside a container) once built —
handy for `discover` during development. Building `btleplug` needs a C compiler,
`pkg-config`, and the D-Bus dev headers; `scripts/install-system-deps.sh`
installs them. See [CONTRIBUTING.md](CONTRIBUTING.md) for the full setup and
conventions, and [docs/releases.md](docs/releases.md) for how releases are cut.

`Cargo.lock` is committed, so the dependency graph (and the `cargo-deny`
advisory check) is reproducible; keep it current with `cargo update`.

## Documentation

- [docs/architecture.md](docs/architecture.md) — components and data flow.
- [docs/bluetooth.md](docs/bluetooth.md) — how the container reaches Bluetooth,
  rootful vs. rootless, SELinux, finding your sensor, and troubleshooting.
- [docs/data-format.md](docs/data-format.md) — NDJSON schema, byte layout, and
  how to inspect the data.
- [docs/releases.md](docs/releases.md) — prebuilt binaries: download, verify,
  run, and where a standalone binary stores its data.
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
- Prebuilt binaries for more targets (e.g. ARM Linux) beyond the three CI
  platforms.

Contributions welcome — see [CONTRIBUTING.md](CONTRIBUTING.md).
