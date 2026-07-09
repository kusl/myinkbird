//! Domain types for a decoded INKBIRD reading.

use thiserror::Error;

/// A single decoded reading from an INKBIRD ITH-13-B.
///
/// Every field is optional so the type can represent partial data and remain
/// forward-compatible with other INKBIRD models. For the ITH-13-B specifically:
///
/// * `temperature_c` is always populated on a successful parse.
/// * `battery_pct` is always populated on a successful parse (the ITH-13-B
///   reports battery inside the advertisement, so passive listening captures
///   it - no connection required).
/// * `humidity_pct` is populated only when the sensor reported a non-zero
///   humidity value.
///
/// The `f64` values are exact tenths (e.g. `28.9`), so serializing them with a
/// shortest-round-trip formatter (such as `serde_json`) produces clean output
/// with no spurious trailing digits.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorReading {
    /// Temperature in degrees Celsius. Signed; may be negative.
    pub temperature_c: Option<f64>,
    /// Relative humidity as a percentage in `0.0..=100.0`.
    pub humidity_pct: Option<f64>,
    /// Battery charge as a whole percentage in `0..=100`.
    pub battery_pct: Option<u8>,
}

impl SensorReading {
    /// An empty reading with every field unset.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            temperature_c: None,
            humidity_pct: None,
            battery_pct: None,
        }
    }

    /// Returns `true` when the reading carries no measurements at all.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.temperature_c.is_none() && self.humidity_pct.is_none() && self.battery_pct.is_none()
    }
}

/// Reasons an advertisement message could not be decoded into a reading.
///
/// A validation failure rejects the **entire** reading rather than publishing
/// a partially-corrupt one, matching the reference INKBIRD parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ParseError {
    /// The message was shorter than the minimum needed to read every field.
    #[error("message too short: need at least {need} bytes, got {got}")]
    TooShort {
        /// Minimum number of bytes required.
        need: usize,
        /// Number of bytes actually supplied.
        got: usize,
    },

    /// Humidity decoded to a physically impossible value (> 100 %RH), which
    /// marks the packet as corrupt (e.g. a garbage `0xFFFF` field -> 6553.5).
    #[error("implausible humidity: {0:.1}% exceeds 100%")]
    ImplausibleHumidity(f64),

    /// Battery decoded to a physically impossible value (> 100 %), which marks
    /// the packet as corrupt (e.g. a garbage `0xFF` byte -> 255).
    #[error("implausible battery: {0}% exceeds 100%")]
    ImplausibleBattery(u8),
}
