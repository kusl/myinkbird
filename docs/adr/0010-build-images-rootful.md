# 0010. Build the container images rootful

- Status: Accepted
- Date: 2026-07-09

## Context

The stack runs **rootful** because BlueZ's D-Bus policy only authorises uid 0
to call `org.bluez` (see [ADR 0005](0005-bluetooth-via-host-bluez-dbus.md)), so
`scripts/run.sh` starts it with `sudo podman compose up -d`.

Rootless and rootful Podman keep **separate image stores** (the rootless store
lives under the user's `~/.local/share/containers`; the rootful store lives
under `/var/lib/containers`). If the images are built rootless (a plain
`podman build`) but the stack is started rootful, the rootful engine cannot see
them. Compose then treats `localhost/myinkbird-collector:latest` as something to
**pull**, contacts a registry literally named `localhost`, and fails:

```
Error: initializing source ...localhost/myinkbird-collector:latest:
pinging container registry localhost: Get "https://localhost/v2/":
dial tcp [::1]:443: connect: connection refused
```

This was the first-run failure that made the project look broken.

## Decision

- Build both images **rootful**: `scripts/container-build.sh` runs
  `podman build` under root (via `as_root`), so the images land in the same
  store the rootful stack runs from.
- Belt and braces: set **`pull_policy: never`** on both services in
  `compose.yaml`, so a missing image fails fast and clearly instead of trying to
  pull `localhost/...` from a non-existent registry.
- Acquire root **once, up front** and keep the sudo timestamp alive for the
  whole build+run (see `ensure_root` in `scripts/lib.sh`), so the build does not
  stall waiting for a password prompt partway through.

## Consequences

- `./scripts/run.sh` and `./scripts/container-build.sh` prompt for `sudo` and
  then build and run without further interaction.
- The images are owned by the rootful store; remove them with
  `sudo podman rmi localhost/myinkbird-collector:latest` (and the committer).
- The build cache is under root's storage, so inspecting or pruning it needs
  `sudo`. For a personal, single-host tool this is an acceptable trade-off.
- A future rootless path (a permissive BlueZ D-Bus policy, per ADR 0005) would
  also let the images be built and run rootless; that remains out of scope.
