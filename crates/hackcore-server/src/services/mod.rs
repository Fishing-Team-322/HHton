pub mod events;
pub mod participants;
pub mod rating;
pub mod submissions;

use async_trait::async_trait;
use contracts::repo_proof::VerifyBindingResponse;

#[async_trait]
pub trait SubmissionArtifacts: Send + Sync {
    async fn store_placeholder(
        &self,
        event_id: &str,
        submission_id: &str,
        bytes: &[u8],
    ) -> anyhow::Result<()>;
}

#[async_trait]
pub trait RepoProofAdapter: Send + Sync {
    async fn health_check(&self) -> anyhow::Result<()>;
    async fn verify_binding(
        &self,
        provider: &str,
        repository: &str,
        commit: &str,
        is_private: bool,
    ) -> anyhow::Result<VerifyBindingResponse>;
}

#[async_trait]
impl SubmissionArtifacts for storage::ObjectStorage {
    async fn store_placeholder(
        &self,
        event_id: &str,
        submission_id: &str,
        bytes: &[u8],
    ) -> anyhow::Result<()> {
        const SUMMARY_ARTIFACT: &str = "summary.txt";
        self.put_placeholder(event_id, submission_id, SUMMARY_ARTIFACT, bytes, None)
            .await
    }
}

#[async_trait]
impl RepoProofAdapter for repo_proof::RepoProofClient {
    async fn health_check(&self) -> anyhow::Result<()> {
        self.health_check().await
    }

    async fn verify_binding(
        &self,
        provider: &str,
        repository: &str,
        commit: &str,
        is_private: bool,
    ) -> anyhow::Result<VerifyBindingResponse> {
        self.verify_binding(provider, repository, commit, is_private)
            .await
    }
}
