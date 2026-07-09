//! The storage abstraction: a [`ReadingSink`] receives finished records.
//!
//! Keeping this a trait (rather than hard-wiring the NDJSON writer into the
//! scanner) follows the dependency-inversion principle: the scanner depends on
//! the abstraction, and tests can substitute an in-memory [`VecSink`] with no
//! filesystem involved.

use crate::record::StoredReading;

/// A destination for decoded readings.
pub trait ReadingSink {
    /// Persist one reading.
    ///
    /// # Errors
    ///
    /// Returns an error if the reading could not be stored (for example, an
    /// I/O failure while appending to a file).
    fn record(&mut self, reading: &StoredReading) -> anyhow::Result<()>;
}

/// An in-memory sink that simply collects every reading. Test-only helper.
#[derive(Debug, Default)]
pub struct VecSink {
    /// Every reading handed to [`ReadingSink::record`], in order.
    pub readings: Vec<StoredReading>,
}

// The inherent helpers below are exercised only by the unit tests; the release
// binary uses `VecSink` solely through the `ReadingSink` trait.
#[cfg_attr(not(test), allow(dead_code))]
impl VecSink {
    /// Create an empty sink.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of readings collected so far.
    #[must_use]
    pub fn len(&self) -> usize {
        self.readings.len()
    }

    /// Whether any readings have been collected.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.readings.is_empty()
    }
}

impl ReadingSink for VecSink {
    fn record(&mut self, reading: &StoredReading) -> anyhow::Result<()> {
        self.readings.push(reading.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkbird_core::SensorReading;

    fn rec() -> StoredReading {
        StoredReading::new(
            "2026-07-08T21:03:44Z".to_string(),
            "AA:BB:CC:DD:EE:FF".to_string(),
            None,
            None,
            &SensorReading {
                temperature_c: Some(21.0),
                humidity_pct: Some(40.0),
                battery_pct: Some(90),
            },
        )
    }

    #[test]
    fn vec_sink_collects_in_order() {
        let mut sink = VecSink::new();
        assert!(sink.is_empty());
        sink.record(&rec()).unwrap();
        sink.record(&rec()).unwrap();
        assert_eq!(sink.len(), 2);
        assert_eq!(sink.readings[0].address, "AA:BB:CC:DD:EE:FF");
    }
}
