# 0005. Bluetooth via the host BlueZ D-Bus, run rootful

- Status: Accepted
- Date: 2026-07-08

## Context

The collector runs inside a container but needs to receive BLE advertisements
via the host's Bluetooth adapter. There are two ways to give a container
Bluetooth access:

1. Pass the raw HCI device (`/dev/hci0`) into the container and drive the
   controller directly. This needs elevated capabilities and takes the adapter
   away from the host's own Bluetooth stack.
2. Let the host's `bluetoothd` own the adapter, and have the container talk to
   it over the **D-Bus system bus**. `btleplug`'s Linux backend is a BlueZ
   D-Bus client, so this is its native mode.

BlueZ authenticates D-Bus callers by their real uid (`SO_PEERCRED`) and its
default policy only authorises `root` (uid 0), and sometimes a `bluetooth`
group, to call `org.bluez`. Under rootless Podman the container process is
mapped to an unprivileged host subuid, which BlueZ rejects. On SELinux hosts
(Fedora's default) a container also may not use a bind-mounted host socket
unless its label is relaxed.

## Decision

- Reach Bluetooth by **bind-mounting the host D-Bus system socket**
  (`/run/dbus/system_bus_socket`) into the collector container. Do **not** pass
  `/dev/hci0`.
- Run the stack **rootful** (`sudo podman ...`), so the container process
  presents as uid 0 to the system bus and satisfies BlueZ's default policy.
  `scripts/run.sh` does this.
- Relax SELinux confinement for the container with
  `security_opt: [label=disable]` in `compose.yaml` so it can use the socket.

## Consequences

- The host's normal Bluetooth stack stays in charge of the adapter; the
  container is just another D-Bus client.
- No raw radio capabilities are handed to the container.
- The stack requires `sudo`. This is an accepted trade-off; the documented
  alternative (installing a permissive BlueZ D-Bus policy granting a non-root
  user or a `bluetooth` group access to `org.bluez`) is a host configuration
  change deliberately left out of scope.
- `label=disable` reduces SELinux confinement for that one container. A tighter
  custom policy is possible but not the out-of-the-box default.

Operational details and troubleshooting are in `docs/bluetooth.md`. The battery
implications of scanning are covered in
[ADR 0003](0003-listen-only-never-connect.md).
