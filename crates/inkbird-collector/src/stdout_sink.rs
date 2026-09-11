//! A [`ReadingSink`](crate::sink::ReadingSink) of last resort: print each
//! reading to standard output.
//!
//! This is used only when no data directory could be created (neither an
//! XDG-style per-user directory nor the executable's own directory was
//! writable), so readings stay visible instead of being silently lost. It emits
//! one JSON object per line - the same shape the files carry - so the output can
//! still be piped into `jq` or redirected to a file by the user.

use std::io::{self, Write};

use anyhow::Context;

use crate::record::StoredReading;
use crate::sink::ReadingSink;

/// Writes each reading as one NDJSON line to standard output.
#[derive(Debug, Default)]
pub struct StdoutSink;

impl StdoutSink {
    /// Create a stdout sink.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl ReadingSink for StdoutSink {
    fn record(&mut self, reading: &StoredReading) -> anyhow::Result<()> {
        let line = serde_json::to_string(reading).context("serializing reading")?;
        let mut out = io::stdout().lock();
        writeln!(out, "{line}").context("writing reading to stdout")?;
        out.flush().context("flushing stdout")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkbird_core::SensorReading;

    #[test]
    fn record_serializes_without_error() {
        // The sink writes to the process stdout; here we only assert that
        // serialization and the write path succeed for a representative record.
        let reading = StoredReading::new(
            "2026-07-08T21:03:44Z".to_string(),
            "AA:BB:CC:DD:EE:FF".to_string(),
            Some("ITH-13-B".to_string()),
            Some(-60),
            &SensorReading {
                temperature_c: Some(21.0),
                humidity_pct: Some(40.0),
                battery_pct: Some(90),
            },
        );
        let mut sink = StdoutSink::new();
        sink.record(&reading).unwrap();
    }
}
