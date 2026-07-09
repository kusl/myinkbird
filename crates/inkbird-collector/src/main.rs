//! `inkbird-collector` entry point.
//!
//! Subcommands:
//!   * `collect` (default) - listen for ITH-13-B advertisements and append
//!     validated readings to daily NDJSON files.
//!   * `discover` - scan for a fixed time and print every device seen, to help
//!     you find your sensor's Bluetooth address and confirm its byte layout.

mod config;
mod ndjson_sink;
mod record;
mod scanner;
mod shutdown;
mod sink;
mod throttle;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::config::{CollectArgs, Config};
use crate::ndjson_sink::NdjsonSink;

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

/// Wire up the NDJSON sink and run the collection loop.
async fn run_collect(args: CollectArgs) -> Result<()> {
    let config = Config::from(args);
    info!(data_dir = %config.data_dir.display(), "starting collector");
    let central = scanner::get_central().await?;
    let mut sink = NdjsonSink::new(&config.data_dir);
    scanner::run_collect(&central, &config, &mut sink).await
}

/// Initialise tracing/logging. Honours `RUST_LOG`; defaults to `info`.
fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}
