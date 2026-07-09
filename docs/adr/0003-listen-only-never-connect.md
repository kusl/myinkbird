# 0003. Listen to advertisements only; never connect

- Status: Accepted
- Date: 2026-07-08

## Context

The overriding goal of this project is to add **no extra battery drain** to the
INKBIRD ITH-13-B, which runs on two AAA cells. There are two broad ways to get
data off such a sensor over Bluetooth Low Energy:

1. **Connect** to it (open a GATT connection) and read/subscribe to
   characteristics. A connection obliges the peripheral to keep a connection
   event schedule and respond to the central, which costs it real energy.
2. **Listen** to the advertisement packets it already broadcasts. The sensor
   broadcasts periodically regardless of whether anyone is listening (that is
   how the vendor app gets readings without a stable link), so a passive
   listener costs the sensor nothing beyond what it already spends.

The ITH-13-B conveniently includes temperature, humidity, **and** battery
percentage directly in its advertisement, so listening is sufficient - there is
nothing we need that would require a connection.

## Decision

The collector **only ever listens** to advertisements and decodes their
manufacturer data. It never opens a GATT connection to the sensor. In the code
this is enforced by simply never calling `connect()`; the scanner resolves each
peripheral's *advertised properties* and decodes them, nothing more.

## Consequences

- The battery guarantee holds **unconditionally**: no connection means no
  connection-induced drain, full stop.
- We get temperature, humidity, and battery from the advertisement alone.
- **Active vs. passive scanning is a separate matter.** `btleplug` drives
  BlueZ's classic `StartDiscovery`, which is an *active* scan: our adapter may
  emit `SCAN_REQ` probes. That is a property of *our computer's* radio, not the
  sensor's, and its incremental cost to a broadcast-only sensor is negligible.
  Truly passive controller-level scanning (never probing) is available via
  BlueZ's `AdvertisementMonitor` D-Bus API, which `btleplug` does not currently
  use.
- **Future work:** adopt `AdvertisementMonitor`-based passive scanning if/when
  it is convenient, as a refinement. It is not required to meet the battery
  goal, because the goal is satisfied by never connecting.

See `docs/bluetooth.md` for the operational details and troubleshooting.
