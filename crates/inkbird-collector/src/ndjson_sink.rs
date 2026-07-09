//! An [`ReadingSink`] that appends readings to per-day NDJSON files.
//!
//! Files live under `<data_dir>/readings/<YYYY-MM-DD>.ndjson`, one JSON object
//! per line. NDJSON is append-friendly, human-readable, and produces clean
//! line-oriented git diffs (see docs/adr/0006).

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::record::StoredReading;
use crate::sink::ReadingSink;

/// Writes readings to daily NDJSON files rooted at a data directory.
#[derive(Debug, Clone)]
pub struct NdjsonSink {
    readings_dir: PathBuf,
}

impl NdjsonSink {
    /// Create a sink rooted at `data_dir`; readings go under
    /// `data_dir/readings/`. The directory is created on first write.
    #[must_use]
    pub fn new(data_dir: impl AsRef<Path>) -> Self {
        Self {
            readings_dir: data_dir.as_ref().join("readings"),
        }
    }

    /// Absolute path of the file a reading with this date key would land in.
    fn file_for(&self, date_key: &str) -> PathBuf {
        self.readings_dir.join(format!("{date_key}.ndjson"))
    }
}

impl ReadingSink for NdjsonSink {
    fn record(&mut self, reading: &StoredReading) -> anyhow::Result<()> {
        fs::create_dir_all(&self.readings_dir).with_context(|| {
            format!("creating readings dir {}", self.readings_dir.display())
        })?;

        let path = self.file_for(reading.date_key());
        let mut line = serde_json::to_string(reading).context("serializing reading")?;
        line.push('\n');

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .with_context(|| format!("opening {}", path.display()))?;
        file.write_all(line.as_bytes())
            .with_context(|| format!("appending to {}", path.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkbird_core::SensorReading;
    use tempfile::tempdir;

    fn reading_on(day: &str, temp: f64) -> StoredReading {
        StoredReading::new(
            format!("{day}T21:03:44Z"),
            "AA:BB:CC:DD:EE:FF".to_string(),
            Some("ITH-13-B".to_string()),
            Some(-55),
            &SensorReading {
                temperature_c: Some(temp),
                humidity_pct: Some(45.5),
                battery_pct: Some(100),
            },
        )
    }

    #[test]
    fn writes_one_line_per_reading_into_dated_file() {
        let dir = tempdir().unwrap();
        let mut sink = NdjsonSink::new(dir.path());
        sink.record(&reading_on("2026-07-08", 28.9)).unwrap();
        sink.record(&reading_on("2026-07-08", 29.0)).unwrap();

        let path = dir.path().join("readings/2026-07-08.ndjson");
        let contents = fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = contents.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("\"temperature_c\":28.9"));
        assert!(lines[1].contains("\"temperature_c\":29"));
        // Each line is independently valid JSON.
        for line in lines {
            let _: serde_json::Value = serde_json::from_str(line).unwrap();
        }
    }

    #[test]
    fn partitions_by_calendar_date() {
        let dir = tempdir().unwrap();
        let mut sink = NdjsonSink::new(dir.path());
        sink.record(&reading_on("2026-07-08", 20.0)).unwrap();
        sink.record(&reading_on("2026-07-09", 21.0)).unwrap();

        assert!(dir.path().join("readings/2026-07-08.ndjson").exists());
        assert!(dir.path().join("readings/2026-07-09.ndjson").exists());
    }

    #[test]
    fn appends_across_sink_instances() {
        let dir = tempdir().unwrap();
        NdjsonSink::new(dir.path())
            .record(&reading_on("2026-07-08", 20.0))
            .unwrap();
        // A fresh sink (as if the process restarted) must append, not truncate.
        NdjsonSink::new(dir.path())
            .record(&reading_on("2026-07-08", 21.0))
            .unwrap();

        let path = dir.path().join("readings/2026-07-08.ndjson");
        let contents = fs::read_to_string(&path).unwrap();
        assert_eq!(contents.lines().count(), 2);
    }
}
