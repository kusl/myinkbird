# Prebuilt binaries and releases

Every push to `main` publishes a **full** GitHub release with a self-contained
`inkbird-collector` binary for Linux, macOS, and Windows. This is the
no-toolchain, no-container way to run the collector. For the *why*, see
[ADR 0011](adr/0011-prebuilt-release-binaries.md).

> These are the same listen-only program as the container build - they never
> connect to the sensor, so the battery guarantee still holds (see
> [ADR 0003](adr/0003-listen-only-never-connect.md)).

## Download

Grab the archive for your platform from the
[latest release](https://github.com/kusl/myinkbird/releases/latest). Assets are
named `inkbird-collector-<version>-<target-triple>.tar.gz`, for example:

- Linux (x86-64): `...-x86_64-unknown-linux-gnu.tar.gz`
- macOS (Apple Silicon): `...-aarch64-apple-darwin.tar.gz`
- Windows (x86-64): `...-x86_64-pc-windows-msvc.tar.gz`

Each archive has a matching `.sha256` file and contains the binary, this
project's `LICENSE`, and the `README.md`.

## Verify the download

```bash
# From the directory holding the archive and its .sha256:
sha256sum -c inkbird-collector-<version>-<triple>.tar.gz.sha256   # Linux
shasum -a 256 -c inkbird-collector-<version>-<triple>.tar.gz.sha256   # macOS
```

Then unpack:

```bash
tar -xzf inkbird-collector-<version>-<triple>.tar.gz
```

On Windows, use the built-in `tar` (available in modern Windows and Git Bash),
or any archiver that understands `.tar.gz`.

## Runtime requirements

The binary is self-contained (all Rust dependencies are linked in), with one
per-platform caveat:

- **Linux** needs a running `bluetoothd` and the system `libdbus` (present on
  any desktop with Bluetooth). It talks to BlueZ over D-Bus, and BlueZ's default
  policy means you will typically need to run it with `sudo` (see
  [docs/bluetooth.md](bluetooth.md)).
- **macOS** uses CoreBluetooth; grant the terminal Bluetooth permission when
  prompted.
- **Windows** uses WinRT; no extra runtime libraries are required.

## Run it

Find your sensor first (scan and print nearby devices):

```bash
./inkbird-collector discover --seconds 30
```

Then collect (the default subcommand). Passing your sensor's address is the most
reliable option:

```bash
./inkbird-collector collect --address AA:BB:CC:DD:EE:FF
```

Everything is also configurable by environment variable (the same names the
container uses); see [Configuration](../README.md#configuration).

## Where a standalone binary stores readings

Unlike the container (which writes to `/data`), a standalone binary that is
**not** told where to write - no `--data-dir` flag and no `INKBIRD_DATA_DIR` -
resolves a default location, in this order:

1. an **XDG-style per-user data directory**:
   - honours `XDG_DATA_HOME` if set (used as `$XDG_DATA_HOME/myinkbird`),
   - otherwise the platform default:
     - Linux: `~/.local/share/myinkbird`
     - macOS: `~/Library/Application Support/myinkbird`
     - Windows: `%APPDATA%\myinkbird`
2. otherwise, the **directory containing the executable** (i.e. next to the
   binary you unpacked);
3. otherwise, if neither is writable, it **prints readings to standard output**
   so they are still visible (you can redirect them to a file yourself).

Readings land under `<that directory>/readings/<YYYY-MM-DD>.ndjson`, the same
per-day NDJSON layout described in [docs/data-format.md](data-format.md). The
startup log prints the directory it chose. To pin a location explicitly:

```bash
./inkbird-collector collect --data-dir /path/you/choose
#   or
INKBIRD_DATA_DIR=/path/you/choose ./inkbird-collector collect
```

Note that a standalone binary only *writes* the NDJSON files - it does not run
the git committer. If you want the readings kept under version control, either
run `git init`/`git commit` in the data directory yourself, or use the container
stack, which includes the committer sidecar (see
[docs/architecture.md](architecture.md)).

## Building a release locally

The release build is just a script, so you can produce your platform's artifact
without CI:

```bash
./scripts/release-build.sh          # writes dist/inkbird-collector-<ver>-<triple>.tar.gz(+ .sha256)
```

Publishing is a second script (used by CI, or locally if you have authenticated
the GitHub CLI with `gh auth login`):

```bash
./scripts/release-publish.sh        # creates/updates the release and uploads dist/*
```
