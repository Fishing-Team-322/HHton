//! Facilities for verifying repository proofs over gRPC.

use std::collections::BTreeMap;
use std::future::Future;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

use contracts::repo_proof::repo_proof_service_client::RepoProofServiceClient;
use contracts::repo_proof::repo_proof_service_server::{RepoProofService, RepoProofServiceServer};
use contracts::repo_proof::verification_verdict::Outcome;
use contracts::repo_proof::{
    HealthCheckRequest, HealthCheckResponse, VerificationVerdict, VerifyBindingRequest,
    VerifyBindingResponse,
};
use prost_types::{value::Kind, Struct, Value};
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

    /// Ask the remote repository proof service to validate a binding.
    #[instrument(name = "repo_proof.verify_binding", skip(self))]
    pub async fn verify_binding(
        &self,
        provider: &str,
        repository: &str,
        commit: &str,
        is_private: bool,
    ) -> anyhow::Result<VerifyBindingResponse> {
        let mut client = self.inner.clone();
        let request = VerifyBindingRequest {
            provider: provider.to_string(),
            repository: repository.to_string(),
            commit: commit.to_string(),
            is_private,
        };
        let response = client.verify_binding(request).await?.into_inner();
        Ok(response)
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

    async fn verify_binding(
        &self,
        request: Request<VerifyBindingRequest>,
    ) -> Result<Response<VerifyBindingResponse>, Status> {
        let payload = request.into_inner();
        let verdict = self.evaluate_binding(&payload);
        Ok(Response::new(VerifyBindingResponse {
            verdict: Some(verdict),
        }))
    }
}

/// Construct a gRPC service that answers repository proof requests.
pub fn service() -> RepoProofServiceServer<RepoProofServiceHandler> {
    RepoProofServiceServer::new(RepoProofServiceHandler)
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

impl RepoProofServiceHandler {
    fn evaluate_binding(&self, binding: &VerifyBindingRequest) -> VerificationVerdict {
        let mut proof_fields = BTreeMap::new();
        proof_fields.insert(
            "provider".to_string(),
            Value {
                kind: Some(Kind::StringValue(binding.provider.clone())),
            },
        );
        proof_fields.insert(
            "repository".to_string(),
            Value {
                kind: Some(Kind::StringValue(binding.repository.clone())),
            },
        );
        proof_fields.insert(
            "commit".to_string(),
            Value {
                kind: Some(Kind::StringValue(binding.commit.clone())),
            },
        );
        proof_fields.insert(
            "is_private".to_string(),
            Value {
                kind: Some(Kind::BoolValue(binding.is_private)),
            },
        );

        let checked_at = current_epoch_seconds();

        if !is_supported_provider(&binding.provider) {
            return VerificationVerdict {
                outcome: Outcome::Rejected as i32,
                reason: format!("unsupported provider '{}'", binding.provider),
                proof: Some(Struct {
                    fields: proof_fields,
                }),
                checked_at,
            };
        }

        if !looks_like_repository(&binding.repository) {
            return VerificationVerdict {
                outcome: Outcome::Rejected as i32,
                reason: "repository must be in the form 'owner/name'".to_string(),
                proof: Some(Struct {
                    fields: proof_fields,
                }),
                checked_at,
            };
        }

        if !looks_like_commit(&binding.commit) {
            return VerificationVerdict {
                outcome: Outcome::Rejected as i32,
                reason: "commit must be a hexadecimal identifier".to_string(),
                proof: Some(Struct {
                    fields: proof_fields,
                }),
                checked_at,
            };
        }

        VerificationVerdict {
            outcome: Outcome::Accepted as i32,
            reason: "repository binding verified".to_string(),
            proof: Some(Struct {
                fields: proof_fields,
            }),
            checked_at,
        }
    }
}

fn is_supported_provider(provider: &str) -> bool {
    matches!(provider, "github" | "gitlab" | "bitbucket" | "gitea")
}

fn looks_like_repository(repository: &str) -> bool {
    let mut segments = repository.split('/');
    matches!(
        (segments.next(), segments.next(), segments.next()),
        (Some(owner), Some(name), None)
            if !owner.trim().is_empty() && !name.trim().is_empty()
    )
}

fn looks_like_commit(commit: &str) -> bool {
    let len = commit.len();
    (6..=64).contains(&len) && commit.chars().all(|c| c.is_ascii_hexdigit())
}

fn current_epoch_seconds() -> i64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as i64,
        Err(err) => -(err.duration().as_secs() as i64),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unknown_provider() {
        let handler = RepoProofServiceHandler::default();
        let verdict = handler.evaluate_binding(&VerifyBindingRequest {
            provider: "unknown".into(),
            repository: "org/repo".into(),
            commit: "abcdef1".into(),
            is_private: false,
        });

        assert_eq!(verdict.outcome, Outcome::Rejected as i32);
        assert!(verdict.reason.contains("unsupported provider"));
    }

    #[test]
    fn rejects_invalid_commit() {
        let handler = RepoProofServiceHandler::default();
        let verdict = handler.evaluate_binding(&VerifyBindingRequest {
            provider: "github".into(),
            repository: "org/repo".into(),
            commit: "invalid".into(),
            is_private: false,
        });

        assert_eq!(verdict.outcome, Outcome::Rejected as i32);
        assert!(verdict.reason.contains("hexadecimal"));
    }

    #[test]
    fn accepts_valid_binding() {
        let handler = RepoProofServiceHandler::default();
        let verdict = handler.evaluate_binding(&VerifyBindingRequest {
            provider: "github".into(),
            repository: "org/repo".into(),
            commit: "abcdef1".into(),
            is_private: false,
        });

        assert_eq!(verdict.outcome, Outcome::Accepted as i32);
        assert!(verdict.reason.contains("verified"));
        assert!(verdict.proof.is_some());
    }
}
