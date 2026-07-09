# Bluetooth setup and troubleshooting

This is the part most likely to need host-specific attention, so it gets its own
document. It covers how the container reaches the host's Bluetooth stack, why
the stack runs rootful, how to find your sensor, how to confirm the byte layout,
and what to do when nothing shows up.

## How the container reaches Bluetooth

The collector does **not** get direct access to the Bluetooth adapter
(`/dev/hci0` is never passed through). Instead it talks to the host's
`bluetoothd` daemon over the **D-Bus system bus**. The compose file bind-mounts
the host socket into the container:

```yaml
volumes:
  - /run/dbus/system_bus_socket:/run/dbus/system_bus_socket
```

`btleplug`'s Linux backend speaks the BlueZ D-Bus API, so from the collector's
point of view it is just making D-Bus calls; `bluetoothd` on the host does the
actual radio work. This keeps the container from needing raw `CAP_NET_ADMIN`
adapter access and means the host's normal Bluetooth stack stays in charge.

The host must therefore have:

- a working Bluetooth adapter, and
- `bluetoothd` running (on Fedora: `systemctl status bluetooth`; enable with
  `sudo systemctl enable --now bluetooth`).

## Why the stack runs rootful (and is built rootful)

BlueZ's default D-Bus policy only authorises the `root` user (uid 0) - and
sometimes members of a `bluetooth` group - to call the `org.bluez` service.
D-Bus authenticates the peer via `SO_PEERCRED`, i.e. the real uid of the
process on the other end of the socket.

Under **rootless** Podman the container process is mapped to an unprivileged
subuid on the host, so BlueZ rejects it with an `Rejected send message` /
`org.freedesktop.DBus.Error.AccessDenied` error (often surfaced as an
`EXTERNAL` authentication rejection). Running the stack **rootful**
(`sudo podman ...`) makes the container process present as uid 0 to the system
bus, which satisfies the policy.

Because the stack runs rootful, the images are also **built** rootful: rootless
and rootful Podman keep separate image stores, so an image built rootless is
invisible to the rootful stack (which then tries, and fails, to pull it from a
registry). `scripts/container-build.sh` builds under `sudo`, and `compose.yaml`
sets `pull_policy: never`. See
[ADR 0005](adr/0005-bluetooth-via-host-bluez-dbus.md) and
[ADR 0010](adr/0010-build-images-rootful.md).

> If you prefer not to run rootful, the alternative is to install a permissive
> BlueZ D-Bus policy file on the host that grants your user (or a `bluetooth`
> group) access to `org.bluez`. That is a host configuration change outside the
> scope of this project, so the supported, out-of-the-box path is rootful.

## SELinux (Fedora)

On SELinux-enforcing hosts (Fedora's default), the container is not allowed to
use the bind-mounted host socket or data directory unless its label is relaxed.
The compose file sets, on both services:

```yaml
security_opt:
  - label=disable
```

which disables SELinux confinement for the containers so they can use the D-Bus
socket and the bind-mounted data directory. If you want tighter confinement, you
can instead craft a custom SELinux policy, but `label=disable` is the simple,
documented default.

## Active vs. passive scanning (and the battery guarantee)

The project's #1 goal is to add **no** battery load to the sensor. Two things
are worth separating:

- **Never connecting.** The collector never opens a GATT connection to the
  sensor. A connection is what would force the sensor to do extra radio work and
  drain its coin-sized power budget. This is the guarantee that matters, and it
  holds unconditionally (see
  [ADR 0003](adr/0003-listen-only-never-connect.md)).
- **Active vs. passive *scanning*.** `btleplug` drives BlueZ's classic
  `StartDiscovery`, which is an *active* scan: the scanning adapter may send
  `SCAN_REQ` packets asking advertisers for a scan response. This is a property
  of **your computer's** radio, not the sensor's, and the incremental cost to a
  broadcast-only sensor is negligible. True controller-level *passive* scanning
  (listening only, never asking) is possible through BlueZ's
  `AdvertisementMonitor` D-Bus API, but `btleplug` does not use it today.
  Adopting it is tracked as future work.

In short: the battery guarantee comes from **never connecting**, which is
already in force. Passive scanning would be a nice-to-have refinement, not a
requirement for the goal.

## Finding your sensor's Bluetooth address

Matching by address is more reliable than matching by name (a passive
advertisement may not even carry the name). Use the `discover` subcommand, which
scans for a fixed time and prints every device it sees, along with an attempted
ITH-13-B decode.

The quickest way (no image build) is the native helper:

```bash
# Builds the workspace, then scans natively (asks for sudo up front):
INKBIRD_DISCOVER_SECONDS=30 sudo -E ./target/release/inkbird-collector discover
# ...or just run it once via cargo:
cargo build --release -p inkbird-collector
```

Or via the container image (build it first - rootful - with
`./scripts/container-build.sh`):

```bash
sudo podman run --rm \
  --security-opt label=disable \
  -v /run/dbus/system_bus_socket:/run/dbus/system_bus_socket \
  localhost/myinkbird-collector:latest discover --seconds 30
```

Look for a device whose name contains `ITH-13-B`, or whose manufacturer data
decodes to a sensible temperature / humidity / battery. Note its address
(`AA:BB:CC:DD:EE:FF`) and put it in your `.env` as `INKBIRD_ADDRESS`.

You can also run the collector directly during development once the workspace is
built:

```bash
./scripts/build.sh
INKBIRD_DISCOVER_SECONDS=30 sudo -E ./target/release/inkbird-collector discover
```

## Confirming the byte layout

The decoder assumes the documented 18-byte ITH-13-B layout (see
[`docs/data-format.md`](data-format.md)). `discover` prints the raw
manufacturer bytes for each device, e.g.:

```text
device AA:BB:CC:DD:EE:FF
  name:  ITH-13-B
  rssi:  -61 dBm
  services: 0000fff0-0000-1000-8000-00805f9b34fb
  mfr 0xcdab: 11 22 33 44 21 01 c7 01 64 00 00 00 00 00 00 00
    -> decoded as ITH-13-B: temp=Some(28.9) C, hum=Some(45.5) %, batt=Some(100) %
```

Compare the decoded values against what the sensor shows on its own display or
in the vendor app. If they line up, the layout is correct for your unit. If they
do not, the byte offsets in `crates/inkbird-core/src/lib.rs` /
`parser.rs` are the place to adjust (and please update the tests and
[`docs/data-format.md`](data-format.md) if you do).

## Troubleshooting

**`pinging container registry localhost ... connect: connection refused`
(port 443) when starting the stack.**
The rootful stack cannot find the images because they were built rootless
(separate image store), so compose tries to pull them from a registry called
`localhost` and fails. Build rootful - `./scripts/container-build.sh` (or just
`./scripts/run.sh`, which does it for you) now builds under `sudo`. See
[ADR 0010](adr/0010-build-images-rootful.md).

**`creating BLE manager ...` / no adapter found.**
`bluetoothd` is probably not running or the D-Bus socket is not mounted. Check
`systemctl status bluetooth` on the host and confirm
`/run/dbus/system_bus_socket` exists and is bind-mounted.

**`AccessDenied` / `Rejected send message` / `EXTERNAL` rejection.**
The container is not running as uid 0 as far as BlueZ is concerned. Make sure
you launched rootful (`sudo podman compose up -d`, or `./scripts/run.sh`). This
is the classic rootless symptom.

**Permission denied touching the socket, or immediate SELinux AVC denials.**
Add `--security-opt label=disable` (already set in `compose.yaml`), or check
`ausearch -m avc -ts recent` on the host.

**The collector runs but records nothing.**
Confirm the sensor is advertising and in range. Run `discover` to see whether it
appears at all. If it appears but under a different name, set `INKBIRD_ADDRESS`
to its address rather than relying on the name match. Remember the collector
throttles unchanged readings (default 60 s), so a perfectly stable environment
produces sparse lines by design. (Note: an *empty* `INKBIRD_ADDRESS` is treated
as "no filter"/match-by-name, so leaving it blank does **not** suppress
readings.)

**`no compose provider found`.**
Install either the `podman compose` plugin or `podman-compose`. The scripts try
`podman compose` first, then fall back to `podman-compose`.
