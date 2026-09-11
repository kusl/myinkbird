//! `inkbird-collector` entry point.
//!
//! Subcommands:
//!   * `collect` (default) - listen for ITH-13-B advertisements and append
//!     validated readings to daily NDJSON files.
//!   * `discover` - scan for a fixed time and print every device seen, to help
//!     you find your sensor's Bluetooth address and confirm its byte layout.

mod config;
mod data_dir;
mod ndjson_sink;
mod record;
mod scanner;
mod shutdown;
mod sink;
mod stdout_sink;
mod throttle;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

use crate::config::{CollectArgs, Config};
use crate::data_dir::{DataDirCandidate, default_data_dir_candidates};
use crate::ndjson_sink::NdjsonSink;
use crate::sink::ReadingSink;
use crate::stdout_sink::StdoutSink;

/// Passive BLE collector for the INKBIRD ITH-13-B thermo-hygrometer.
#[derive(Debug, Parser)]
#[command(name = "inkbird-collector", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Listen for advertisements and log readings (default).
    Collect(CollectArgs),
    /// Scan and print nearby devices to identify your sensor.
    Discover(DiscoverArgs),
}

/// Arguments for the `discover` subcommand.
#[derive(Debug, Args)]
struct DiscoverArgs {
    /// How many seconds to scan before exiting.
    #[arg(long, env = "INKBIRD_DISCOVER_SECONDS", default_value_t = 30)]
    seconds: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let cli = Cli::parse();
    match cli.command {
        Some(Command::Discover(args)) => {
            let central = scanner::get_central().await?;
            scanner::run_discover(&central, args.seconds).await
        }
        // `collect` is the default when no subcommand is given. Using
        // CollectArgs::parse() here re-parses the leftover args (there are
        // none once the subcommand is absent), applying env fallbacks and
        // defaults.
        Some(Command::Collect(args)) => run_collect(args).await,
        None => run_collect(CollectArgs::parse_from(std::env::args().take(1))).await,
    }
}

/// Wire up a sink and run the collection loop.
async fn run_collect(args: CollectArgs) -> Result<()> {
    let config = Config::from(args);
    info!("starting collector (listen-only)");
    let central = scanner::get_central().await?;
    let mut sink = open_sink(&config);
    scanner::run_collect(&central, &config, &mut *sink).await
}

/// Choose where readings go, following the resolution order in the `data_dir`
/// module.
///
/// When the data directory was given explicitly (`--data-dir` /
/// `INKBIRD_DATA_DIR`, as the container sets), honour it. Otherwise try an
/// XDG-style per-user directory, then the executable's own directory, and
/// finally fall back to printing readings to standard output so they are at
/// least visible rather than silently lost.
fn open_sink(config: &Config) -> Box<dyn ReadingSink> {
    if let Some(dir) = &config.data_dir {
        // Explicit choice: use it as-is. NdjsonSink creates the directory on the
        // first write and surfaces any error there.
        info!(
            data_dir = %dir.display(),
            "writing readings to the configured data directory"
        );
        return Box::new(NdjsonSink::new(dir));
    }

    for DataDirCandidate { path, source } in default_data_dir_candidates() {
        match NdjsonSink::create_in(&path) {
            Ok(sink) => {
                info!(
                    source = source.label(),
                    "writing readings to {}",
                    path.join("readings").display()
                );
                return Box::new(sink);
            }
            Err(error) => warn!(
                candidate = %path.display(),
                source = source.label(),
                %error,
                "cannot use this data directory; trying the next option"
            ),
        }
    }

    warn!(
        "no writable data directory found (tried an XDG-style per-user directory \
         and the executable's own directory); printing readings to standard \
         output instead. Set INKBIRD_DATA_DIR to choose a writable location."
    );
    Box::new(StdoutSink::new())
}

/// Initialise tracing/logging. Honours `RUST_LOG`; defaults to `info`.
fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}
