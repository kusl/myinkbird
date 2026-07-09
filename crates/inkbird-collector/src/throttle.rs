//! Per-device throttling so an unchanged sensor does not flood the log.
//!
//! A device that keeps re-broadcasting the same values only produces a new
//! record once `min_interval` has elapsed. Any *change* in temperature,
//! humidity, or battery is recorded immediately, regardless of the interval,
//! so real transitions are never lost.
//!
//! The logic is pure (time is passed in as an [`Instant`]) so it is fully
//! unit-testable without sleeping.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// The decoded values used to decide whether a reading is "the same".
///
/// Floats are compared by their raw bits after scaling, but since the decoder
/// produces exact tenths we compare the integer tenths to avoid float-equality
/// pitfalls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Snapshot {
    temp_tenths: Option<i32>,
    hum_tenths: Option<i32>,
    battery: Option<u8>,
}

impl Snapshot {
    fn new(temp_c: Option<f64>, hum_pct: Option<f64>, battery: Option<u8>) -> Self {
        // Round to the nearest tenth; the decoder already yields tenths so this
        // is exact in practice and merely defensive. Values are tiny (tenths of
        // a degree/percent), so the f64 -> i32 cast cannot realistically truncate.
        #[allow(clippy::cast_possible_truncation)]
        let to_tenths = |v: f64| (v * 10.0).round() as i32;
        Self {
            temp_tenths: temp_c.map(to_tenths),
            hum_tenths: hum_pct.map(to_tenths),
            battery,
        }
    }
}

struct Last {
    snapshot: Snapshot,
    at: Instant,
}

/// Tracks the last recorded snapshot per device address.
pub struct Throttle {
    min_interval: Duration,
    last: HashMap<String, Last>,
}

impl Throttle {
    /// Create a throttle that suppresses unchanged readings seen within
    /// `min_interval` of the previous recorded one.
    #[must_use]
    pub fn new(min_interval: Duration) -> Self {
        Self {
            min_interval,
            last: HashMap::new(),
        }
    }

    /// Decide whether a freshly decoded reading for `address` should be
    /// recorded, updating internal state when it should.
    ///
    /// Returns `true` (and remembers this reading) when either:
    /// * the device has not been seen before, or
    /// * any value changed since the last recorded reading, or
    /// * at least `min_interval` has elapsed since the last recorded reading.
    pub fn should_record(
        &mut self,
        address: &str,
        temp_c: Option<f64>,
        hum_pct: Option<f64>,
        battery: Option<u8>,
        now: Instant,
    ) -> bool {
        let snapshot = Snapshot::new(temp_c, hum_pct, battery);
        let decision = match self.last.get(address) {
            None => true,
            Some(prev) => {
                prev.snapshot != snapshot || now.duration_since(prev.at) >= self.min_interval
            }
        };
        if decision {
            self.last
                .insert(address.to_string(), Last { snapshot, at: now });
        }
        decision
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADDR: &str = "AA:BB:CC:DD:EE:FF";

    #[test]
    fn first_sighting_is_always_recorded() {
        let mut t = Throttle::new(Duration::from_secs(60));
        let now = Instant::now();
        assert!(t.should_record(ADDR, Some(20.0), Some(40.0), Some(90), now));
    }

    #[test]
    fn unchanged_within_interval_is_suppressed() {
        let mut t = Throttle::new(Duration::from_secs(60));
        let now = Instant::now();
        assert!(t.should_record(ADDR, Some(20.0), Some(40.0), Some(90), now));
        let soon = now + Duration::from_secs(10);
        assert!(!t.should_record(ADDR, Some(20.0), Some(40.0), Some(90), soon));
    }

    #[test]
    fn changed_value_is_recorded_immediately() {
        let mut t = Throttle::new(Duration::from_secs(60));
        let now = Instant::now();
        assert!(t.should_record(ADDR, Some(20.0), Some(40.0), Some(90), now));
        let soon = now + Duration::from_secs(1);
        // Temperature changed by a tenth -> record now.
        assert!(t.should_record(ADDR, Some(20.1), Some(40.0), Some(90), soon));
    }

    #[test]
    fn unchanged_after_interval_is_recorded() {
        let mut t = Throttle::new(Duration::from_secs(60));
        let now = Instant::now();
        assert!(t.should_record(ADDR, Some(20.0), Some(40.0), Some(90), now));
        let later = now + Duration::from_secs(61);
        assert!(t.should_record(ADDR, Some(20.0), Some(40.0), Some(90), later));
    }

    #[test]
    fn humidity_change_is_recorded() {
        let mut t = Throttle::new(Duration::from_secs(60));
        let now = Instant::now();
        assert!(t.should_record(ADDR, Some(20.0), Some(40.0), Some(90), now));
        assert!(t.should_record(
            ADDR,
            Some(20.0),
            Some(41.0),
            Some(90),
            now + Duration::from_secs(2)
        ));
    }

    #[test]
    fn battery_change_is_recorded() {
        let mut t = Throttle::new(Duration::from_secs(60));
        let now = Instant::now();
        assert!(t.should_record(ADDR, Some(20.0), Some(40.0), Some(90), now));
        assert!(t.should_record(
            ADDR,
            Some(20.0),
            Some(40.0),
            Some(89),
            now + Duration::from_secs(2)
        ));
    }

    #[test]
    fn humidity_appearing_counts_as_change() {
        let mut t = Throttle::new(Duration::from_secs(60));
        let now = Instant::now();
        assert!(t.should_record(ADDR, Some(20.0), None, Some(90), now));
        // Humidity went from absent to present -> change.
        assert!(t.should_record(
            ADDR,
            Some(20.0),
            Some(40.0),
            Some(90),
            now + Duration::from_secs(2)
        ));
    }

    #[test]
    fn separate_addresses_are_tracked_independently() {
        let mut t = Throttle::new(Duration::from_secs(60));
        let now = Instant::now();
        assert!(t.should_record("AA:AA:AA:AA:AA:AA", Some(20.0), Some(40.0), Some(90), now));
        // A different device's first sighting is still recorded even though it
        // is "the same" values within the interval.
        assert!(t.should_record(
            "BB:BB:BB:BB:BB:BB",
            Some(20.0),
            Some(40.0),
            Some(90),
            now + Duration::from_secs(1)
        ));
    }

    #[test]
    fn zero_interval_records_everything() {
        let mut t = Throttle::new(Duration::from_secs(0));
        let now = Instant::now();
        assert!(t.should_record(ADDR, Some(20.0), Some(40.0), Some(90), now));
        assert!(t.should_record(
            ADDR,
            Some(20.0),
            Some(40.0),
            Some(90),
            now + Duration::from_nanos(1)
        ));
    }
}
