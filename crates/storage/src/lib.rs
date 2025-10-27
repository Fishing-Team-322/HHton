//! Object storage integration built on top of the official AWS SDK.

use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Context;
use aws_sdk_s3::{primitives::ByteStream, Client};
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Metadata stored alongside submission snapshots.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotMetadata {
    pub event_id: String,
    pub submission_id: String,
    pub archive_size: u64,
    pub uploaded_at: u64,
}

impl SnapshotMetadata {
    /// Convenience constructor used when the metadata is derived during upload.
    pub fn new(
        event_id: impl Into<String>,
        submission_id: impl Into<String>,
        archive_size: u64,
        uploaded_at: u64,
    ) -> Self {
        Self {
            event_id: event_id.into(),
            submission_id: submission_id.into(),
            archive_size,
            uploaded_at,
        }
    }
}

/// Represents the full snapshot download (archive bytes + metadata).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub metadata: SnapshotMetadata,
    pub archive: Vec<u8>,
}

/// Provides high-level interactions with the S3 bucket dedicated to repository assets.
#[derive(Clone, Debug)]
pub struct ObjectStorage {
    client: Client,
    bucket: String,
    prefix: Option<String>,
}

impl ObjectStorage {
    /// Create a new storage facade for the provided bucket name.
    pub fn new(client: Client, bucket: impl Into<String>) -> Self {
        Self::with_prefix::<String>(client, bucket, None)
    }

    /// Create a new storage facade with an optional key prefix applied to every object.
    pub fn with_prefix<S>(client: Client, bucket: impl Into<String>, prefix: Option<S>) -> Self
    where
        S: Into<String>,
    {
        let prefix = prefix
            .map(Into::into)
            .map(|value| value.trim_matches('/').to_string())
            .filter(|value| !value.is_empty());

        Self {
            client,
            bucket: bucket.into(),
            prefix,
        }
    }

    /// Return the configured bucket name for diagnostic purposes.
    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    fn snapshot_base(&self, event_id: &str, submission_id: &str) -> String {
        let base = format!("hackathons/{event_id}/submissions/{submission_id}");
        match &self.prefix {
            Some(prefix) => format!("{prefix}/{base}"),
            None => base,
        }
    }

    fn archive_key(&self, event_id: &str, submission_id: &str) -> String {
        format!("{}.tar.gz", self.snapshot_base(event_id, submission_id))
    }

    fn metadata_key(&self, event_id: &str, submission_id: &str) -> String {
        format!("{}.json", self.snapshot_base(event_id, submission_id))
    }

    /// Upload a submission snapshot archive and persist derived metadata alongside it.
    #[instrument(name = "storage.upload_snapshot", skip(self, archive_bytes))]
    pub async fn upload_snapshot(
        &self,
        event_id: &str,
        submission_id: &str,
        archive_bytes: &[u8],
    ) -> anyhow::Result<SnapshotMetadata> {
        let archive_key = self.archive_key(event_id, submission_id);
        let metadata_key = self.metadata_key(event_id, submission_id);
        let uploaded_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let metadata = SnapshotMetadata::new(
            event_id,
            submission_id,
            archive_bytes.len() as u64,
            uploaded_at,
        );

        tracing::debug!(bucket = %self.bucket, %archive_key, %metadata_key, "uploading snapshot archive");

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&archive_key)
            .body(ByteStream::from(archive_bytes.to_vec()))
            .send()
            .await
            .with_context(|| format!("failed to upload archive to '{archive_key}'"))?;

        let metadata_payload =
            serde_json::to_vec(&metadata).context("failed to serialize snapshot metadata")?;

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&metadata_key)
            .content_type("application/json")
            .body(ByteStream::from(metadata_payload))
            .send()
            .await
            .with_context(|| format!("failed to upload metadata to '{metadata_key}'"))?;

        Ok(metadata)
    }

    /// Retrieve a snapshot along with its metadata from the object storage.
    #[instrument(name = "storage.get_snapshot", skip(self))]
    pub async fn get_snapshot(
        &self,
        event_id: &str,
        submission_id: &str,
    ) -> anyhow::Result<Snapshot> {
        let archive_key = self.archive_key(event_id, submission_id);
        let metadata_key = self.metadata_key(event_id, submission_id);

        let metadata_object = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(&metadata_key)
            .send()
            .await
            .with_context(|| format!("failed to download metadata from '{metadata_key}'"))?;
        let metadata_bytes = metadata_object
            .body
            .collect()
            .await
            .context("failed to collect metadata body")?
            .into_bytes()
            .to_vec();
        let metadata: SnapshotMetadata =
            serde_json::from_slice(&metadata_bytes).with_context(|| {
                format!("failed to deserialize metadata stored in '{metadata_key}'")
            })?;

        let archive_object = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(&archive_key)
            .send()
            .await
            .with_context(|| format!("failed to download archive from '{archive_key}'"))?;
        let archive = archive_object
            .body
            .collect()
            .await
            .context("failed to collect archive body")?
            .into_bytes()
            .to_vec();

        Ok(Snapshot { metadata, archive })
    }

    /// Delete both the snapshot archive and its metadata.
    #[instrument(name = "storage.delete_snapshot", skip(self))]
    pub async fn delete_snapshot(&self, event_id: &str, submission_id: &str) -> anyhow::Result<()> {
        let archive_key = self.archive_key(event_id, submission_id);
        let metadata_key = self.metadata_key(event_id, submission_id);

        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(&archive_key)
            .send()
            .await
            .with_context(|| format!("failed to delete archive '{archive_key}'"))?;

        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(&metadata_key)
            .send()
            .await
            .with_context(|| format!("failed to delete metadata '{metadata_key}'"))?;

        Ok(())
    }

    /// Stub upload method that will be replaced by a real implementation.
    #[instrument(name = "storage.put_placeholder", skip(self, _bytes))]
    pub async fn put_placeholder(&self, key: &str, _bytes: &[u8]) -> anyhow::Result<()> {
        tracing::debug!(bucket = %self.bucket, %key, "queueing upload to S3");
        let _client = self.client.clone();
        Ok(())
    }
}
