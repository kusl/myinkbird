//! The Bluetooth listener.
//!
//! This module **never connects** to a peripheral. It subscribes to adapter
//! events, reads the advertised properties of matching devices, decodes their
//! manufacturer data with `inkbird-core`, and hands finished records to a
//! [`ReadingSink`]. Never opening a GATT connection is what protects the
//! sensor's battery (see docs/adr/0003).
//!
//! A note on "passive": `btleplug` drives BlueZ's classic `StartDiscovery`,
//! which is an *active* scan at the controller level (the adapter may emit
//! `SCAN_REQ`). That is a property of the scanner's radio, not the sensor, and
//! costs the sensor effectively nothing. True controller-level passive
//! scanning would require BlueZ's `AdvertisementMonitor` API, which `btleplug`
//! does not use today; see docs/bluetooth.md and docs/adr/0003.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use btleplug::api::{Central as _, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, PeripheralId};
use futures::StreamExt;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use tokio::time::sleep;
use tracing::{debug, info, warn};

use inkbird_core::{build_message, parse_ith_13_b, SensorReading, ITH_13_B_MESSAGE_LEN};

use crate::config::Config;
use crate::record::StoredReading;
use crate::shutdown::shutdown_signal;
use crate::sink::ReadingSink;
use crate::throttle::Throttle;

/// Acquire the first available Bluetooth adapter via the host BlueZ stack.
///
/// # Errors
///
/// Fails if the BLE manager cannot be created (commonly: `bluetoothd` is not
/// running or the D-Bus system socket is not reachable from inside the
/// container) or if the host has no Bluetooth adapter.
pub async fn get_central() -> Result<Adapter> {
    let manager = Manager::new()
        .await
        .context("creating BLE manager (is bluetoothd running and the D-Bus system socket mounted?)")?;
    let adapters = manager
        .adapters()
        .await
        .context("listing Bluetooth adapters")?;
    adapters
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("no Bluetooth adapter found"))
}

/// Run the collection loop until a shutdown signal arrives.
///
/// # Errors
///
/// Fails if the adapter's event stream or scan cannot be started.
pub async fn run_collect(
    central: &Adapter,
    config: &Config,
    sink: &mut dyn ReadingSink,
) -> Result<()> {
    let mut events = central
        .events()
        .await
        .context("subscribing to adapter events")?;
    central
        .start_scan(ScanFilter::default())
        .await
        .context("starting BLE scan")?;

    if config.has_address_filter() {
        info!(address = ?config.address, "listening for ITH-13-B advertisements (listen-only)");
    } else {
        info!(
            name_match = %config.name_match,
            "listening for ITH-13-B advertisements by name (listen-only); set an address for reliability"
        );
    }

    let mut throttle = Throttle::new(config.min_interval);
    let shutdown = shutdown_signal();
    tokio::pin!(shutdown);

    loop {
        tokio::select! {
            maybe_event = events.next() => {
                let Some(event) = maybe_event else {
                    warn!("adapter event stream ended");
                    break;
                };
                if let CentralEvent::DeviceDiscovered(id) | CentralEvent::DeviceUpdated(id) = event {
                    if let Err(error) = handle_peripheral(central, config, &mut throttle, sink, &id).await {
                        warn!(%error, "failed to process an advertisement");
                    }
                }
            }
            _ = &mut shutdown => {
                info!("shutdown requested; stopping scan");
                break;
            }
        }
    }

    // Best-effort: stopping the scan is not worth failing shutdown over.
    if let Err(error) = central.stop_scan().await {
        warn!(%error, "error stopping scan");
    }
    Ok(())
}

/// Handle one discovered/updated peripheral: identify, decode, throttle, store.
async fn handle_peripheral(
    central: &Adapter,
    config: &Config,
    throttle: &mut Throttle,
    sink: &mut dyn ReadingSink,
    id: &PeripheralId,
) -> Result<()> {
    let peripheral = central
        .peripheral(id)
        .await
        .context("resolving peripheral from id")?;
    let Some(props) = peripheral
        .properties()
        .await
        .context("reading peripheral properties")?
    else {
        return Ok(());
    };

    let address = props.address.to_string();
    let name = props.local_name.clone();

    // Identify: prefer an explicit address filter; otherwise match by name.
    let matched = if config.has_address_filter() {
        config.address_matches(&address)
    } else {
        name.as_deref()
            .is_some_and(|n| n.to_ascii_lowercase().contains(&config.name_match))
    };
    if !matched {
        return Ok(());
    }

    let Some(reading) = decode_first(&props.manufacturer_data) else {
        debug!(%address, "matched device but no decodable ITH-13-B message yet");
        return Ok(());
    };
    if reading.is_empty() {
        return Ok(());
    }

    if !throttle.should_record(
        &address,
        reading.temperature_c,
        reading.humidity_pct,
        reading.battery_pct,
        Instant::now(),
    ) {
        return Ok(());
    }

    let record = StoredReading::new(now_rfc3339(), address, name, props.rssi, &reading);
    sink.record(&record).context("recording reading")?;
    info!(
        address = %record.address,
        temperature_c = ?record.temperature_c,
        humidity_pct = ?record.humidity_pct,
        battery_pct = ?record.battery_pct,
        rssi_dbm = ?record.rssi_dbm,
        "recorded reading"
    );
    Ok(())
}

/// Try to decode a reading from any manufacturer-data entry whose
/// reconstructed message is exactly the ITH-13-B length.
fn decode_first(manufacturer_data: &HashMap<u16, Vec<u8>>) -> Option<SensorReading> {
    for (company_id, payload) in manufacturer_data {
        let message = build_message(*company_id, payload);
        if message.len() == ITH_13_B_MESSAGE_LEN {
            if let Ok(reading) = parse_ith_13_b(&message) {
                return Some(reading);
            }
        }
    }
    None
}

/// Scan for `seconds` and print every distinct device once, with its raw
/// manufacturer data and an attempted ITH-13-B decode. Handy for finding your
/// sensor's address and confirming its byte layout.
///
/// # Errors
///
/// Fails if the adapter's event stream or scan cannot be started.
pub async fn run_discover(central: &Adapter, seconds: u64) -> Result<()> {
    let mut events = central
        .events()
        .await
        .context("subscribing to adapter events")?;
    central
        .start_scan(ScanFilter::default())
        .await
        .context("starting BLE scan")?;
    info!(seconds, "discovering BLE devices; press Ctrl-C to stop early");

    let mut seen: HashSet<String> = HashSet::new();
    let deadline = sleep(Duration::from_secs(seconds));
    tokio::pin!(deadline);
    let shutdown = shutdown_signal();
    tokio::pin!(shutdown);

    loop {
        tokio::select! {
            maybe_event = events.next() => {
                let Some(event) = maybe_event else { break; };
                if let CentralEvent::DeviceDiscovered(id) | CentralEvent::DeviceUpdated(id) = event {
                    if let Err(error) = report_device(central, &mut seen, &id).await {
                        warn!(%error, "failed to report a device");
                    }
                }
            }
            _ = &mut deadline => break,
            _ = &mut shutdown => break,
        }
    }

    if let Err(error) = central.stop_scan().await {
        warn!(%error, "error stopping scan");
    }
    info!(devices = seen.len(), "discovery finished");
    Ok(())
}

/// Print one device's details, but only the first time its address is seen.
async fn report_device(
    central: &Adapter,
    seen: &mut HashSet<String>,
    id: &PeripheralId,
) -> Result<()> {
    let peripheral = central
        .peripheral(id)
        .await
        .context("resolving peripheral from id")?;
    let Some(props) = peripheral
        .properties()
        .await
        .context("reading peripheral properties")?
    else {
        return Ok(());
    };

    let address = props.address.to_string();
    if !seen.insert(address.clone()) {
        return Ok(());
    }

    let name = props.local_name.unwrap_or_else(|| "<no name>".to_string());
    println!("device {address}");
    println!("  name:  {name}");
    if let Some(rssi) = props.rssi {
        println!("  rssi:  {rssi} dBm");
    }
    if !props.services.is_empty() {
        let uuids: Vec<String> = props.services.iter().map(ToString::to_string).collect();
        println!("  services: {}", uuids.join(", "));
    }
    for (company_id, payload) in &props.manufacturer_data {
        println!("  mfr 0x{company_id:04x}: {}", to_hex(payload));
        let message = build_message(*company_id, payload);
        match parse_ith_13_b(&message) {
            Ok(r) => println!(
                "    -> decoded as ITH-13-B: temp={:?} C, hum={:?} %, batt={:?} %",
                r.temperature_c, r.humidity_pct, r.battery_pct
            ),
            Err(error) => println!("    -> not a valid ITH-13-B message: {error}"),
        }
    }
    Ok(())
}

/// Current UTC time as a whole-second RFC 3339 string
/// (e.g. `2026-07-08T21:03:44Z`).
fn now_rfc3339() -> String {
    let now = OffsetDateTime::now_utc();
    let now = now.replace_nanosecond(0).unwrap_or(now);
    now.format(&Rfc3339).unwrap_or_else(|_| now.to_string())
}

/// Lower-case, space-separated hex dump of a byte slice.
fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 3);
    for (i, byte) in bytes.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        // Writing to a String is infallible.
        let _ = write!(out, "{byte:02x}");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_first_picks_valid_eighteen_byte_entry() {
        let mut data: HashMap<u16, Vec<u8>> = HashMap::new();
        // A 16-byte payload + 2-byte company id == 18-byte message.
        // temp 289 -> 28.9, hum 455 -> 45.5, battery 100.
        data.insert(
            0xCDAB,
            vec![
                0x11, 0x22, 0x33, 0x44, 0x21, 0x01, 0xC7, 0x01, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
        );
        let reading = decode_first(&data).expect("should decode");
        assert_eq!(reading.battery_pct, Some(100));
    }

    #[test]
    fn decode_first_ignores_wrong_length_entries() {
        let mut data: HashMap<u16, Vec<u8>> = HashMap::new();
        data.insert(0x1234, vec![0x01, 0x02, 0x03]); // far too short
        assert!(decode_first(&data).is_none());
    }

    #[test]
    fn decode_first_on_empty_map_is_none() {
        let data: HashMap<u16, Vec<u8>> = HashMap::new();
        assert!(decode_first(&data).is_none());
    }

    #[test]
    fn to_hex_formats_bytes() {
        assert_eq!(to_hex(&[0x00, 0x0a, 0xff]), "00 0a ff");
        assert_eq!(to_hex(&[]), "");
    }

    #[test]
    fn now_rfc3339_has_no_fractional_seconds() {
        let ts = now_rfc3339();
        assert!(ts.ends_with('Z'), "ts={ts}");
        assert!(!ts.contains('.'), "ts={ts}");
        assert_eq!(ts.len(), 20, "expected YYYY-MM-DDTHH:MM:SSZ, got {ts}");
    }
}
