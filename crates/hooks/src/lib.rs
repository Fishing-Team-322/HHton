//! Integration hooks that broadcast lifecycle events.

use tracing::info;

/// Emit a lifecycle hook and log it for observability.
pub fn emit(event: &str) {
    info!(%event, "hook emitted");
}

/// Placeholder for registering external subscribers.
pub fn register(subscriber: &str) {
    info!(%subscriber, "registering hook subscriber");
}
