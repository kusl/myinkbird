# Architecture

This document describes how `myinkbird` is put together and how a temperature /
humidity reading travels from the sensor to a committed line in a local git
repository. For the *why* behind each choice, see the Architecture Decision
Records under [`docs/adr/`](adr/README.md).

## Goal recap

Capture readings from an INKBIRD **ITH-13-B** thermo-hygrometer while adding
**no extra battery drain** to the sensor, and store those readings in a local
git repository. The sensor already broadcasts its measurements as Bluetooth Low
Energy (BLE) advertisements, so the collector only ever *listens*; it never
connects (see [ADR 0003](adr/0003-listen-only-never-connect.md)).

## Components

The project is a Rust Cargo workspace with two crates plus two small container
images wired together with Podman Compose.

### `inkbird-core` (library crate)

Pure, I/O-free decoding of ITH-13-B advertisement bytes. It has a single
non-test dependency (`thiserror`) and knows nothing about Bluetooth, async,
files, or JSON. Given the reconstructed manufacturer *message* it returns a
validated `SensorReading` or a `ParseError`. Keeping it pure means every branch
is covered by fast unit tests with no hardware involved. The byte layout it
decodes is documented in [`docs/data-format.md`](data-format.md) and in the
crate's own module docs.

### `inkbird-collector` (binary crate)

The runnable program. Responsibilities are split across small modules:

- `scanner` - subscribes to BLE adapter events via `btleplug`, reads the
  *advertised* properties of matching devices, decodes their manufacturer data
  with `inkbird-core`, and hands finished records to a sink. It never opens a
  GATT connection.
- `config` - resolves CLI flags and environment variables into a validated
  `Config` (data directory, optional address filter, name-match substring,
  throttle interval).
- `throttle` - suppresses duplicate readings from an unchanged sensor while
  always recording any real change. Pure and time-injected, so it is fully
  unit-testable.
- `sink` - the `ReadingSink` trait (dependency inversion): the scanner depends
  on the abstraction, tests substitute an in-memory `VecSink`.
- `ndjson_sink` - the production `ReadingSink`; appends one JSON object per line
  to per-day files.
- `record` - the on-disk record shape (`StoredReading`) and its serialization.
- `shutdown` - resolves on Ctrl-C or SIGTERM so the scan stops cleanly when
  `podman stop` is issued.

The binary exposes two subcommands: `collect` (the default) and `discover`
(scan for a fixed time and print every device seen, to find your sensor's
address and confirm its byte layout).

### `collector` container

An OCI image built from the `Containerfile`. Multi-stage: it compiles with the
full Fedora toolchain, then ships only the stripped binary on
`fedora-minimal`. Its one runtime dependency is `libdbus`, because `btleplug`
talks to the host's `bluetoothd` over the D-Bus system bus.

### `committer` container

An OCI image built from `Containerfile.committer`. It runs `commit-loop.sh`,
which initialises the shared data volume as a *local* git repository (if needed)
and commits any new readings on an interval. It **never pushes**
(see [ADR 0006](adr/0006-local-git-ndjson-storage.md)).

### Shared data volume

A named Podman volume, `inkbird-data`, mounted at `/data` in both containers.
The collector writes NDJSON into it; the committer commits it. This is the only
thing the two containers share.

## Data flow

```text
   ┌────────────────────┐
   │  INKBIRD ITH-13-B   │   broadcasts BLE advertisements
   │  (2×AAA battery)    │   (temperature, humidity, battery %)
   └─────────┬──────────┘
             │  radio (listen-only; sensor is never connected to)
             ▼
   ┌────────────────────┐
   │  Host Bluetooth     │   bluetoothd owns the adapter
   │  adapter + BlueZ    │
   └─────────┬──────────┘
             │  D-Bus system bus
             │  (/run/dbus/system_bus_socket, bind-mounted)
             ▼
┌──────────────────────────────────────────────┐
│  collector container                          │
│                                               │
│  btleplug ── adapter events ──► scanner       │
│                                   │           │
│              inkbird-core ◄───────┤ decode    │
│                                   │           │
│              throttle  ◄──────────┤ dedupe    │
│                                   ▼           │
│                          NdjsonSink (append)  │
└───────────────────┬───────────────────────────┘
                    │  writes <date>.ndjson
                    ▼
        ┌───────────────────────────┐
        │  inkbird-data volume       │
        │  /data/readings/*.ndjson   │
        └───────────┬───────────────┘
                    │  git add / commit on an interval (no push)
                    ▼
        ┌───────────────────────────┐
        │  committer container       │
        │  local git repo in /data   │
        └───────────────────────────┘
```

Step by step:

1. The sensor broadcasts an advertisement containing manufacturer-specific data.
2. The host Bluetooth adapter, driven by BlueZ's `bluetoothd`, receives it.
3. The collector (inside the container) reaches `bluetoothd` over the bind-
   mounted D-Bus system socket and receives a `DeviceDiscovered` /
   `DeviceUpdated` event.
4. `scanner` resolves the peripheral's advertised properties and checks whether
   it matches the configured address (preferred) or name substring.
5. Matching manufacturer data is decoded by `inkbird-core` into a
   `SensorReading`. Implausible packets (humidity > 100 %, battery > 100 %) are
   rejected whole.
6. `throttle` decides whether this reading is worth recording (new device, any
   changed value, or enough time elapsed).
7. `NdjsonSink` appends the record as one line to
   `/data/readings/<YYYY-MM-DD>.ndjson`.
8. On its own schedule, the committer stages and commits new lines to the local
   git repository in `/data`. Nothing is ever pushed.

## Boundaries and dependency direction

- `inkbird-collector` depends on `inkbird-core`, never the reverse.
- Within the collector, `scanner` depends on the `ReadingSink` *trait*, not on
  `NdjsonSink` directly. The concrete sink is injected in `main`. This keeps the
  hardware-facing code and the storage code independently testable.
- The two containers communicate only through files in the shared volume; there
  is no network link between them.

## What is intentionally *not* here

- **No outbound network.** Readings are stored locally and never pushed. Adding
  a remote or a dashboard is future work.
- **No GATT connection to the sensor.** This is the core battery guarantee.
- **No controller-level passive scanning yet.** `btleplug` uses BlueZ's classic
  active discovery. That is a property of the *scanner's* radio, not the sensor,
  and costs the sensor nothing; see [`docs/bluetooth.md`](bluetooth.md) and
  [ADR 0003](adr/0003-listen-only-never-connect.md).
