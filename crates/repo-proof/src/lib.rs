//! Facilities for verifying repository proofs over gRPC.

use std::future::Future;
use std::net::SocketAddr;

use contracts::repo_proof::repo_proof_service_client::RepoProofServiceClient;
use contracts::repo_proof::repo_proof_service_server::{RepoProofService, RepoProofServiceServer};
use contracts::repo_proof::{HealthCheckRequest, HealthCheckResponse};
use tonic::transport::{Channel, Server};
use tonic::{Request, Response, Status};
use tracing::{info, instrument};

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

/// Minimal server implementation that always reports a healthy status.
#[derive(Default)]
pub struct RepoProofServiceHandler;

#[tonic::async_trait]
impl RepoProofService for RepoProofServiceHandler {
    async fn health_check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        Ok(Response::new(HealthCheckResponse {
            status: "ok".to_string(),
        }))
    }
}

/// Construct a gRPC service that answers repository proof requests.
pub fn service() -> RepoProofServiceServer<RepoProofServiceHandler> {
    RepoProofServiceServer::new(RepoProofServiceHandler::default())
}

/// Run the repository proof gRPC server until the provided shutdown signal resolves.
#[instrument(skip(shutdown))]
pub async fn serve<S>(addr: SocketAddr, shutdown: S) -> anyhow::Result<()>
where
    S: Future<Output = ()> + Send + 'static,
{
    info!(%addr, "starting repo-proof service");
    Server::builder()
        .add_service(service())
        .serve_with_shutdown(addr, shutdown)
        .await?;
    info!("repo-proof service stopped");
    Ok(())
}
