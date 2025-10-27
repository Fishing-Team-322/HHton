//! Object storage integration built on top of the official AWS SDK.

use aws_sdk_s3::Client;
use tracing::instrument;

/// Provides high-level interactions with the S3 bucket dedicated to repository assets.
#[derive(Clone, Debug)]
pub struct ObjectStorage {
    client: Client,
    bucket: String,
}

impl ObjectStorage {
    /// Create a new storage facade for the provided bucket name.
    pub fn new(client: Client, bucket: impl Into<String>) -> Self {
        Self {
            client,
            bucket: bucket.into(),
        }
    }

    /// Return the configured bucket name for diagnostic purposes.
    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    /// Stub upload method that will be replaced by a real implementation.
    #[instrument(name = "storage.put_placeholder", skip(self, _bytes))]
    pub async fn put_placeholder(&self, key: &str, _bytes: &[u8]) -> anyhow::Result<()> {
        tracing::debug!(bucket = %self.bucket, %key, "queueing upload to S3");
        let _client = self.client.clone();
        Ok(())
    }
}
