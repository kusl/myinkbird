//! The on-disk record shape written to the NDJSON log.

use inkbird_core::SensorReading;
use serde::Serialize;

/// The model string stamped onto every record this collector writes.
pub const MODEL: &str = "ITH-13-B";

/// One line of the NDJSON log: a decoded reading plus capture metadata.
///
/// Timestamps are RFC 3339 in UTC (e.g. `2026-07-08T21:03:44Z`). The first ten
/// characters are the calendar date, which the NDJSON sink uses to choose the
/// daily file.
#[derive(Debug, Clone, Serialize)]
pub struct StoredReading {
    /// Capture time, RFC 3339 / ISO 8601, UTC.
    pub ts: String,
    /// Bluetooth device address, e.g. `AA:BB:CC:DD:EE:FF`.
    pub address: String,
    /// Advertised local name, if the advertisement carried one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Device model (always [`MODEL`] for this collector).
    pub model: &'static str,
    /// Temperature in degrees Celsius.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_c: Option<f64>,
    /// Relative humidity as a percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub humidity_pct: Option<f64>,
    /// Battery charge as a whole percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_pct: Option<u8>,
    /// Received signal strength in dBm, if the stack reported it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rssi_dbm: Option<i16>,
}

impl StoredReading {
    /// Build a record from a decoded [`SensorReading`] and capture metadata.
    #[must_use]
    pub fn new(
        ts: String,
        address: String,
        name: Option<String>,
        rssi_dbm: Option<i16>,
        reading: &SensorReading,
    ) -> Self {
        Self {
            ts,
            address,
            name,
            model: MODEL,
            temperature_c: reading.temperature_c,
            humidity_pct: reading.humidity_pct,
            battery_pct: reading.battery_pct,
            rssi_dbm,
        }
    }

    /// The `YYYY-MM-DD` calendar-date prefix of the timestamp.
    ///
    /// Falls back to the whole string if it is somehow shorter than 10 chars,
    /// so a malformed timestamp can never panic the writer.
    #[must_use]
    pub fn date_key(&self) -> &str {
        self.ts.get(..10).unwrap_or(&self.ts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_reading() -> SensorReading {
        SensorReading {
            temperature_c: Some(28.9),
            humidity_pct: Some(45.5),
            battery_pct: Some(100),
        }
    }

    #[test]
    fn date_key_is_first_ten_chars() {
        let r = StoredReading::new(
            "2026-07-08T21:03:44Z".to_string(),
            "AA:BB:CC:DD:EE:FF".to_string(),
            None,
            Some(-60),
            &sample_reading(),
        );
        assert_eq!(r.date_key(), "2026-07-08");
    }

    #[test]
    fn serializes_to_compact_json_line() {
        let r = StoredReading::new(
            "2026-07-08T21:03:44Z".to_string(),
            "AA:BB:CC:DD:EE:FF".to_string(),
            Some("ITH-13-B".to_string()),
            Some(-60),
            &sample_reading(),
        );
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("\"model\":\"ITH-13-B\""));
        assert!(json.contains("\"temperature_c\":28.9"));
        assert!(json.contains("\"humidity_pct\":45.5"));
        assert!(json.contains("\"battery_pct\":100"));
        assert!(json.contains("\"rssi_dbm\":-60"));
        // Compact: no newlines inside a single record.
        assert!(!json.contains('\n'));
    }

    #[test]
    fn absent_optionals_are_skipped() {
        let reading = SensorReading {
            temperature_c: Some(20.0),
            humidity_pct: None,
            battery_pct: Some(88),
        };
        let r = StoredReading::new(
            "2026-07-08T21:03:44Z".to_string(),
            "AA:BB:CC:DD:EE:FF".to_string(),
            None,
            None,
            &reading,
        );
        let json = serde_json::to_string(&r).unwrap();
        assert!(!json.contains("humidity_pct"));
        assert!(!json.contains("rssi_dbm"));
        assert!(!json.contains("\"name\""));
    }
}
