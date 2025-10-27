//! Facilities for verifying repository proofs over gRPC.

use contracts::repo_proof::repo_proof_service_client::RepoProofServiceClient;
use contracts::repo_proof::HealthCheckRequest;
use tonic::transport::Channel;
use tracing::instrument;

/// Client wrapper around a gRPC channel that talks to the repository proof service.
#[derive(Clone, Debug)]
pub struct RepoProofClient {
    inner: RepoProofServiceClient<Channel>,
}

impl RepoProofClient {
    /// Wrap an existing gRPC channel.
    pub fn new(channel: Channel) -> Self {
        Self {
            inner: RepoProofServiceClient::new(channel),
        }
    }

    /// Execute a lightweight health check against the remote service.
    #[instrument(name = "repo_proof.health_check")]
    pub async fn health_check(&self) -> anyhow::Result<()> {
        let mut client = self.inner.clone();
        client.health_check(HealthCheckRequest {}).await?;
        Ok(())
    }
}
