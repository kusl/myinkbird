# Data format

The collector writes one JSON object per line (NDJSON) into per-day files, and
the committer stores those files in a local git repository. This document
describes the record schema and how to inspect the data.

## Where the data lives

Inside the containers the data volume is mounted at `/data`:

```
/data/
├── .git/                       created by the committer (local only, never pushed)
└── readings/
    ├── 2026-07-08.ndjson
    ├── 2026-07-09.ndjson
    └── ...
```

Files are partitioned by UTC calendar date (the first ten characters of the
timestamp). A new file starts at 00:00 UTC.

If you run the collector directly during development (not in a container), the
default data directory is `./data` relative to where you run it, unless you set
`INKBIRD_DATA_DIR` (see [docs/bluetooth.md](bluetooth.md) for direct-run
examples).

## Record schema

Each line is a single JSON object with these fields:

| Field           | Type              | Always present? | Notes                                             |
| --------------- | ----------------- | --------------- | ------------------------------------------------- |
| `ts`            | string            | yes             | Capture time, RFC 3339 / ISO 8601, UTC, whole-second (e.g. `2026-07-08T21:03:44Z`) |
| `address`       | string            | yes             | Bluetooth device address, `AA:BB:CC:DD:EE:FF`     |
| `name`          | string            | no              | Advertised local name, if the advertisement carried one |
| `model`         | string            | yes             | Always `"ITH-13-B"` for this collector            |
| `temperature_c` | number            | no¹             | Degrees Celsius, one decimal place; may be negative |
| `humidity_pct`  | number            | no              | Relative humidity %, one decimal place; omitted when the sensor reported zero |
| `battery_pct`   | integer           | no¹             | Battery charge %, `0..=100`                       |
| `rssi_dbm`      | integer           | no              | Received signal strength in dBm, if the stack reported it |

¹ For a successfully decoded ITH-13-B packet, `temperature_c` and `battery_pct`
are always populated in practice; they are typed optional only so the record
shape stays forward-compatible with other INKBIRD models. Fields that are
absent are omitted entirely rather than written as `null`.

### Example lines

```json
{"ts":"2026-07-08T21:03:44Z","address":"AA:BB:CC:DD:EE:FF","name":"ITH-13-B","model":"ITH-13-B","temperature_c":28.9,"humidity_pct":45.5,"battery_pct":100,"rssi_dbm":-61}
{"ts":"2026-07-08T21:04:45Z","address":"AA:BB:CC:DD:EE:FF","name":"ITH-13-B","model":"ITH-13-B","temperature_c":29.0,"humidity_pct":45.4,"battery_pct":100,"rssi_dbm":-63}
```

## How readings are decoded (byte layout)

The values above come from decoding the advertisement's manufacturer-specific
data. On the wire, BLE manufacturer data is a 16-bit little-endian company
identifier followed by a payload. The decoder reconstructs the full *message*
as `company_id.to_le_bytes() ++ payload` and reads the 18-byte ITH-13-B layout:

```text
offset  bytes  field
------  -----  --------------------------------------------
0..2    2      company id (little-endian)
2..6    4      unknown / device-specific
6..8    2      temperature, i16 little-endian, value / 10 = degrees C
8..10   2      humidity,    u16 little-endian, value / 10 = %RH
10      1      battery percent (0..=100)
11..18  7      unknown / device-specific
```

Temperature is **signed** (a sensor in a fridge can read below zero). Humidity
is published only when its raw value is non-zero. A packet whose humidity
exceeds 100 %RH or whose battery exceeds 100 % is treated as corrupt and the
**whole** reading is dropped. This mirrors the reference behaviour of the
`Bluetooth-Devices/inkbird-ble` project. The authoritative description lives in
the crate's module docs (`crates/inkbird-core/src/lib.rs`), and every rule here
is covered by unit tests in `crates/inkbird-core/src/parser.rs`.

To confirm the layout matches your specific unit, use the `discover` subcommand,
which prints the raw bytes and an attempted decode; see
[docs/bluetooth.md](bluetooth.md).

## De-duplication (why lines are sparse)

The collector throttles unchanged readings. For a given device it records a new
line when:

- it is the first time that device is seen, **or**
- the temperature, humidity, or battery value changed, **or**
- at least `INKBIRD_MIN_INTERVAL_SECS` (default 60) have elapsed since the last
  recorded line.

So a perfectly stable room produces roughly one line per minute, while a
changing environment produces a line per change. This keeps the files (and the
git history) compact without losing real transitions.

## Inspecting the data

NDJSON is line-oriented, so ordinary tools work well. The examples below assume
you are looking at the data volume; to reach it from the host you can either
mount it into a throwaway container or copy it out.

Open a shell in the data volume:

```bash
sudo podman run --rm -it -v inkbird-data:/data registry.fedoraproject.org/fedora-minimal:latest \
  bash -lc 'cd /data && ls -la readings'
```

Pretty-print today's readings with `jq`:

```bash
sudo podman run --rm -v inkbird-data:/data registry.fedoraproject.org/fedora:latest \
  bash -lc 'dnf -y install jq >/dev/null 2>&1; jq . /data/readings/'"$(date -u +%F)"'.ndjson' | less
```

Extract just timestamps and temperature as CSV:

```bash
jq -r '[.ts, (.temperature_c|tostring)] | @csv' readings/2026-07-08.ndjson
```

### Browsing the git history

The committer keeps the data under version control locally. From inside the
volume:

```bash
cd /data
git log --oneline           # commits, one per interval that had new readings
git log -p readings/2026-07-08.ndjson   # how a day's file grew over time
git show HEAD               # the most recent batch of appended readings
```

Because commits only happen when there is something new, the log doubles as a
coarse record of when the sensor was active. Nothing here is ever pushed to a
remote (see [ADR 0006](adr/0006-local-git-ndjson-storage.md)); if you want the
data elsewhere, copy the files or add a remote yourself, deliberately.
