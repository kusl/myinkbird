//! Graceful-shutdown signal handling.
//!
//! Resolves when the process receives either Ctrl-C (SIGINT) or SIGTERM (the
//! signal `podman stop` sends), so the collector can stop its scan cleanly.

use tracing::warn;

/// Completes on the first shutdown signal (Ctrl-C or, on Unix, SIGTERM).
#[cfg(unix)]
pub async fn shutdown_signal() {
    use tokio::signal::unix::{signal, SignalKind};

    let mut sigterm = match signal(SignalKind::terminate()) {
        Ok(s) => s,
        Err(error) => {
            warn!(%error, "could not install SIGTERM handler; relying on Ctrl-C only");
            // Fall back to Ctrl-C only.
            let _ = tokio::signal::ctrl_c().await;
            return;
        }
    };

    tokio::select! {
        result = tokio::signal::ctrl_c() => {
            if let Err(error) = result {
                warn!(%error, "error listening for Ctrl-C");
            }
        }
        _ = sigterm.recv() => {}
    }
}

/// Completes on Ctrl-C (non-Unix platforms have no SIGTERM).
#[cfg(not(unix))]
pub async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
