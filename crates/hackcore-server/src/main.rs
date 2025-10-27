use std::sync::Arc;

use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client as S3Client;
use hackcore_server::{
    config::Settings,
    persistence::{connect_pool, PostgresPersistence},
    serve_unix, telemetry,
};
use persistence::Database;
use repo_proof::RepoProofClient;
use tokio::signal;
use tonic::transport::Channel;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    telemetry::init();
    let settings = Settings::load()?;
    info!(socket = %settings.socket_path, "starting hackcore server");

    let pool = connect_pool(&settings).await?;
    let database = Database::new(pool);
    let persistence = PostgresPersistence::new(database);
    persistence.run_migrations().await?;

    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3_client = S3Client::new(&aws_config);
    let storage = storage::ObjectStorage::new(s3_client, settings.s3_bucket.clone());

    let channel = Channel::from_shared(settings.repo_proof_endpoint.clone())?
        .connect()
        .await?;
    let repo_proof = RepoProofClient::new(channel);

    let shutdown = async {
        let _ = signal::ctrl_c().await;
        info!("shutdown signal received");
    };

    serve_unix(
        &settings.socket_path,
        Arc::new(persistence),
        Arc::new(storage),
        Arc::new(repo_proof),
        shutdown,
    )
    .await?;

    Ok(())
}
