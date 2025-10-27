pub mod events;
pub mod participants;
pub mod rating;
pub mod submissions;

use async_trait::async_trait;

#[async_trait]
pub trait SubmissionArtifacts: Send + Sync {
    async fn store_placeholder(&self, key: &str, bytes: &[u8]) -> anyhow::Result<()>;
}

#[async_trait]
pub trait RepoProofAdapter: Send + Sync {
    async fn health_check(&self) -> anyhow::Result<()>;
}

#[async_trait]
impl SubmissionArtifacts for storage::ObjectStorage {
    async fn store_placeholder(&self, key: &str, bytes: &[u8]) -> anyhow::Result<()> {
        self.put_placeholder(key, bytes).await
    }
}

#[async_trait]
impl RepoProofAdapter for repo_proof::RepoProofClient {
    async fn health_check(&self) -> anyhow::Result<()> {
        self.health_check().await
    }
}
