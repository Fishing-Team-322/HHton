pub mod config;
pub mod persistence;
pub mod services;
pub mod telemetry;

use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;

use services::{
    events::EventsHandler, participants::ParticipantsHandler, rating::RatingHandler,
    submissions::SubmissionsService, RepoProofAdapter, SubmissionArtifacts,
};

use crate::persistence::PersistenceGateway;

/// Launch the gRPC services bound to the provided Unix domain socket.
pub async fn serve_unix<P, S, R, F>(
    socket_path: impl AsRef<Path>,
    persistence: Arc<P>,
    storage: Arc<S>,
    repo_proof: Arc<R>,
    shutdown: F,
) -> Result<()>
where
    P: PersistenceGateway + 'static,
    S: SubmissionArtifacts + 'static,
    R: RepoProofAdapter + 'static,
    F: std::future::Future<Output = ()> + Send + 'static,
{
    let path = socket_path.as_ref();
    if path.exists() {
        tokio::fs::remove_file(path).await?;
    }

    let listener = UnixListener::bind(path)?;
    let incoming = UnixListenerStream::new(listener);

    let participant_service = ParticipantsHandler::new(persistence.clone());
    let event_service = EventsHandler::new(persistence.clone());
    let submissions_service = SubmissionsService::new(persistence.clone(), storage, repo_proof);
    let rating_service = RatingHandler::new(persistence.clone());

    Server::builder()
        .add_service(
            contracts::participants::participants_service_server::ParticipantsServiceServer::new(
                participant_service,
            ),
        )
        .add_service(
            contracts::events::events_service_server::EventsServiceServer::new(event_service),
        )
        .add_service(
            contracts::submits::submits_service_server::SubmitsServiceServer::new(
                submissions_service,
            ),
        )
        .add_service(
            contracts::rating::rating_service_server::RatingServiceServer::new(rating_service),
        )
        .serve_with_incoming_shutdown(incoming, shutdown)
        .await?;

    Ok(())
}
