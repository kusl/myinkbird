//! Configuration for the collector, sourced from CLI flags with environment
//! variable fallbacks (via clap's `env` feature).

use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;

/// Arguments for the `collect` subcommand (the default mode).
#[derive(Debug, Clone, Parser)]
pub struct CollectArgs {
    /// Directory the readings are written under (`<dir>/readings/*.ndjson`).
    #[arg(long, env = "INKBIRD_DATA_DIR", default_value = "./data")]
    pub data_dir: PathBuf,

    /// Only record advertisements from this Bluetooth address
    /// (e.g. `AA:BB:CC:DD:EE:FF`). Recommended - it is the most reliable way
    /// to pick your sensor, especially under passive scanning where the name
    /// may be absent. Find it with the `discover` subcommand. An empty value
    /// (the default) means "no address filter": match by name instead.
    #[arg(long, env = "INKBIRD_ADDRESS")]
    pub address: Option<String>,

    /// When no address is set, match devices whose advertised local name
    /// contains this (case-insensitive) substring.
    #[arg(long, env = "INKBIRD_NAME_MATCH", default_value = "ith-13-b")]
    pub name_match: String,

    /// Minimum seconds between recorded readings for an unchanged sensor.
    /// Any change in temperature, humidity, or battery is always recorded.
    #[arg(long, env = "INKBIRD_MIN_INTERVAL_SECS", default_value_t = 60)]
    pub min_interval_secs: u64,
}

/// Resolved, validated configuration for a collection run.
#[derive(Debug, Clone)]
pub struct Config {
    /// Directory readings are written under.
    pub data_dir: PathBuf,
    /// Optional exact Bluetooth-address filter.
    pub address: Option<String>,
    /// Case-insensitive local-name substring used when no address is set.
    pub name_match: String,
    /// Minimum interval between unchanged recorded readings.
    pub min_interval: Duration,
}

impl From<CollectArgs> for Config {
    fn from(args: CollectArgs) -> Self {
        Self {
            data_dir: args.data_dir,
            // Normalise the address once so comparisons are case-insensitive.
            //
            // An empty or whitespace-only value means "no address filter", so
            // we fall back to matching by name. This matters because Compose
            // passes an unset `INKBIRD_ADDRESS` through as the empty string
            // (`INKBIRD_ADDRESS=""`); without this, the collector would filter
            // on an empty address, match nothing, and silently record nothing.
            address: args
                .address
                .map(|a| a.trim().to_ascii_uppercase())
                .filter(|a| !a.is_empty()),
            name_match: args.name_match.to_ascii_lowercase(),
            min_interval: Duration::from_secs(args.min_interval_secs),
        }
    }
}

impl Config {
    /// Does `address` (as reported by the stack) match the configured filter?
    ///
    /// When no address filter is set this always returns `true` and the caller
    /// falls back to name matching.
    #[must_use]
    pub fn address_matches(&self, address: &str) -> bool {
        match &self.address {
            Some(want) => address.trim().eq_ignore_ascii_case(want),
            None => true,
        }
    }

    /// Is an address filter configured?
    #[must_use]
    pub fn has_address_filter(&self) -> bool {
        self.address.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args() -> CollectArgs {
        CollectArgs {
            data_dir: PathBuf::from("/data"),
            address: Some("aa:bb:cc:dd:ee:ff".to_string()),
            name_match: "ITH-13-B".to_string(),
            min_interval_secs: 120,
        }
    }

    #[test]
    fn address_is_normalised_to_uppercase() {
        let cfg = Config::from(args());
        assert_eq!(cfg.address.as_deref(), Some("AA:BB:CC:DD:EE:FF"));
    }

    #[test]
    fn name_match_is_lowercased() {
        let cfg = Config::from(args());
        assert_eq!(cfg.name_match, "ith-13-b");
    }

    #[test]
    fn interval_is_seconds() {
        let cfg = Config::from(args());
        assert_eq!(cfg.min_interval, Duration::from_secs(120));
    }

    #[test]
    fn address_matching_is_case_insensitive() {
        let cfg = Config::from(args());
        assert!(cfg.address_matches("AA:BB:CC:DD:EE:FF"));
        assert!(cfg.address_matches("aa:bb:cc:dd:ee:ff"));
        assert!(!cfg.address_matches("11:22:33:44:55:66"));
        assert!(cfg.has_address_filter());
    }

    #[test]
    fn without_address_filter_everything_matches() {
        let mut a = args();
        a.address = None;
        let cfg = Config::from(a);
        assert!(cfg.address_matches("anything"));
        assert!(!cfg.has_address_filter());
    }

    #[test]
    fn empty_address_is_treated_as_no_filter() {
        // Compose passes an unset INKBIRD_ADDRESS as "".
        let mut a = args();
        a.address = Some(String::new());
        let cfg = Config::from(a);
        assert!(!cfg.has_address_filter());
        assert!(cfg.address_matches("anything"));
    }

    #[test]
    fn whitespace_only_address_is_treated_as_no_filter() {
        let mut a = args();
        a.address = Some("   ".to_string());
        let cfg = Config::from(a);
        assert!(!cfg.has_address_filter());
        assert!(cfg.address_matches("anything"));
    }
}
