//! Facilities for verifying repository proofs over gRPC.

use tonic::transport::Channel;
use tracing::instrument;

/// Client wrapper around a gRPC channel that talks to the repository proof service.
#[derive(Clone, Debug)]
pub struct RepoProofClient {
    channel: Channel,
}

impl RepoProofClient {
    /// Wrap an existing gRPC channel.
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }

    /// Execute a lightweight health check against the remote service.
    #[instrument(name = "repo_proof.health_check")]
    pub async fn health_check(&self) -> anyhow::Result<()> {
        // The API surface will be filled in by follow-up tasks. For now we only ensure
        // that asynchronous code compiles and tracing is wired in place.
        let _ = self.channel.clone();
        Ok(())
    }
}
