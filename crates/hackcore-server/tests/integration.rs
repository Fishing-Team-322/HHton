use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Error, Result};
use async_trait::async_trait;
use contracts::events::events_service_client::EventsServiceClient;
use contracts::events::GetEventRequest;
use contracts::participants::participants_service_client::ParticipantsServiceClient;
use contracts::participants::{GetParticipantRequest, ListParticipantsRequest};
use contracts::rating::rating_service_client::RatingServiceClient;
use contracts::rating::{EventOutcome, EventScale, GetRatingRequest, UpdateRatingRequest};
use contracts::repo_proof::{
    verification_verdict::Outcome as RepoProofOutcome, VerificationVerdict, VerifyBindingResponse,
};
use contracts::submits::submits_service_client::SubmitsServiceClient;
use contracts::submits::{GetSubmissionRequest, RepositoryBinding, SubmitSolutionRequest};
use core_domain::hackathon::{Hackathon, HackathonName, TeamSizeLimit};
use core_domain::model::{AggregateRoot, EntityId};
use core_domain::submission::Submission;
use core_domain::team::{Team, TeamName};
use core_domain::user::{DisplayName, EmailAddress, User, UserProfile};
use hackcore_server::persistence::PersistenceGateway;
use hackcore_server::serve_unix;
use hackcore_server::services::{RepoProofAdapter, SubmissionArtifacts};
use repo_proof::RepoProofClient;
use tempfile::tempdir;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::{Channel, Endpoint};
use tower::service_fn;

#[derive(Default)]
struct InMemoryPersistence {
    participants: RwLock<HashMap<String, User>>,
    hackathons: RwLock<HashMap<String, Hackathon>>,
    teams: RwLock<HashMap<String, Team>>,
    submissions: RwLock<HashMap<String, Submission>>,
}

#[async_trait]
impl PersistenceGateway for InMemoryPersistence {
    async fn get_user(&self, id: &str) -> Result<Option<User>> {
        Ok(self.participants.read().await.get(id).cloned())
    }

    async fn list_users(&self, limit: usize, offset: usize) -> Result<Vec<User>> {
        let mut users: Vec<_> = self.participants.read().await.values().cloned().collect();
        users.sort_by(|a, b| a.id().cmp(b.id()));
        Ok(users.into_iter().skip(offset).take(limit).collect())
    }

    async fn get_hackathon(&self, id: &str) -> Result<Option<Hackathon>> {
        Ok(self.hackathons.read().await.get(id).cloned())
    }

    async fn list_hackathons(&self, limit: usize, offset: usize) -> Result<Vec<Hackathon>> {
        let mut events: Vec<_> = self.hackathons.read().await.values().cloned().collect();
        events.sort_by(|a, b| a.id().cmp(b.id()));
        Ok(events.into_iter().skip(offset).take(limit).collect())
    }

    async fn insert_submission(&self, submission: Submission) -> Result<Submission> {
        self.submissions
            .write()
            .await
            .insert(submission.id().to_string(), submission.clone());
        Ok(submission)
    }

    async fn get_submission(&self, id: &str) -> Result<Option<Submission>> {
        Ok(self.submissions.read().await.get(id).cloned())
    }

    async fn get_team(&self, id: &str) -> Result<Option<Team>> {
        Ok(self.teams.read().await.get(id).cloned())
    }
}

#[derive(Default)]
struct MemoryStorage {
    calls: Mutex<Vec<(String, String, Vec<u8>)>>,
}

#[async_trait]
impl SubmissionArtifacts for MemoryStorage {
    async fn store_placeholder(
        &self,
        event_id: &str,
        submission_id: &str,
        bytes: &[u8],
    ) -> Result<()> {
        self.calls.lock().await.push((
            event_id.to_string(),
            submission_id.to_string(),
            bytes.to_vec(),
        ));
        Ok(())
    }
}

struct MockRepoProof {
    health_checks: Mutex<u32>,
    verifications: Mutex<u32>,
    response: VerifyBindingResponse,
}

impl Default for MockRepoProof {
    fn default() -> Self {
        Self::accepting()
    }
}

impl MockRepoProof {
    fn accepting() -> Self {
        Self::with_outcome(RepoProofOutcome::Accepted, "repository binding verified")
    }

    fn rejecting(reason: &str) -> Self {
        Self::with_outcome(RepoProofOutcome::Rejected, reason)
    }

    fn with_outcome(outcome: RepoProofOutcome, reason: &str) -> Self {
        let verdict = VerificationVerdict {
            outcome: outcome as i32,
            reason: reason.to_string(),
            proof: None,
            checked_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_secs() as i64)
                .unwrap_or_default(),
        };
        Self {
            health_checks: Mutex::new(0),
            verifications: Mutex::new(0),
            response: VerifyBindingResponse {
                verdict: Some(verdict),
            },
        }
    }
}

#[async_trait]
impl RepoProofAdapter for MockRepoProof {
    async fn health_check(&self) -> Result<()> {
        *self.health_checks.lock().await += 1;
        Ok(())
    }

    async fn verify_binding(
        &self,
        _provider: &str,
        _repository: &str,
        _commit: &str,
        _is_private: bool,
    ) -> Result<VerifyBindingResponse> {
        *self.verifications.lock().await += 1;
        Ok(self.response.clone())
    }
}

async fn seed_persistence(persistence: &Arc<InMemoryPersistence>) -> Result<()> {
    let profile = UserProfile::new(
        DisplayName::new("Alice")?,
        EmailAddress::new("alice@example.com")?,
    );
    let user = User::new(EntityId("user-1".into()), profile)?;
    persistence
        .participants
        .write()
        .await
        .insert(user.id().to_string(), user.clone());

    let team = Team::new(
        EntityId("team-1".into()),
        TeamName::new("Alpha")?,
        vec![EntityId("user-1".into())],
    )?;
    persistence
        .teams
        .write()
        .await
        .insert(team.id().to_string(), team);

    let now = SystemTime::now();
    let hackathon = Hackathon::new(
        EntityId("hack-1".into()),
        HackathonName::new("HackOne")?,
        now - Duration::from_secs(60),
        now + Duration::from_secs(3600),
        TeamSizeLimit::new(5)?,
    )?;
    persistence
        .hackathons
        .write()
        .await
        .insert(hackathon.id().to_string(), hackathon);

    Ok(())
}

async fn start_server<R>(
    persistence: Arc<InMemoryPersistence>,
    storage: Arc<MemoryStorage>,
    repo_proof: Arc<R>,
    socket_path: PathBuf,
) -> (JoinHandle<Result<()>>, tokio::sync::oneshot::Sender<()>)
where
    R: RepoProofAdapter + 'static,
{
    let (tx, rx) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(serve_unix(
        socket_path.clone(),
        persistence,
        storage,
        repo_proof,
        async move {
            let _ = rx.await;
        },
    ));
    (server, tx)
}

async fn connect(socket: &PathBuf) -> Result<Channel> {
    let path = socket.clone();
    let endpoint = Endpoint::try_from("http://[::]:50051")?;
    let channel = endpoint
        .connect_with_connector(service_fn(move |_| {
            let path = path.clone();
            async move { tokio::net::UnixStream::connect(path).await }
        }))
        .await?;
    Ok(channel)
}

#[tokio::test]
async fn server_serves_primary_rpcs() -> Result<()> {
    let persistence = Arc::new(InMemoryPersistence::default());
    seed_persistence(&persistence).await?;
    let storage = Arc::new(MemoryStorage::default());
    let repo_proof = Arc::new(MockRepoProof::default());
    let dir = tempdir()?;
    let socket_path = dir.path().join("hackcore.sock");

    let (server, shutdown) = start_server(
        persistence.clone(),
        storage.clone(),
        repo_proof.clone(),
        socket_path.clone(),
    )
    .await;

    tokio::time::sleep(Duration::from_millis(100)).await;

    let channel = connect(&socket_path).await?;

    let mut participants = ParticipantsServiceClient::new(channel.clone());
    let participant = participants
        .get_participant(GetParticipantRequest {
            participant_id: "user-1".into(),
        })
        .await?
        .into_inner()
        .participant
        .expect("participant present");
    assert_eq!(participant.display_name, "Alice");

    let listed = participants
        .list_participants(ListParticipantsRequest {
            page_size: 10,
            page_token: String::new(),
        })
        .await?
        .into_inner();
    assert_eq!(listed.participants.len(), 1);

    let mut events = EventsServiceClient::new(channel.clone());
    let event = events
        .get_event(GetEventRequest {
            event_id: "hack-1".into(),
        })
        .await?
        .into_inner()
        .event
        .expect("event exists");
    assert_eq!(event.name, "HackOne");

    let mut submits = SubmitsServiceClient::new(channel.clone());
    let submission = submits
        .submit_solution(SubmitSolutionRequest {
            team_id: "team-1".into(),
            hackathon_id: "hack-1".into(),
            summary: "Great project".into(),
            repository_binding: Some(RepositoryBinding {
                provider: "github".into(),
                repository: "org/repo".into(),
                commit: "abc123".into(),
                is_private: false,
            }),
        })
        .await?
        .into_inner()
        .submission
        .expect("submission created");
    assert_eq!(submission.team_id, "team-1");
    assert_eq!(submission.verification_statuses.len(), 1);
    assert_eq!(submission.verification_statuses.len(), 1);

    let health_checks = repo_proof.health_checks.lock().await;
    assert_eq!(*health_checks, 1);
    drop(health_checks);

    let verifications = repo_proof.verifications.lock().await;
    assert_eq!(*verifications, 1);
    drop(verifications);

    let stored = storage.calls.lock().await;
    assert_eq!(stored.len(), 1);
    let (event_id, submission_id, bytes) = &stored[0];
    assert_eq!(event_id, "hack-1");
    assert_eq!(submission_id, &submission.id);
    assert_eq!(bytes, b"Great project");
    drop(stored);

    let fetched = submits
        .get_submission(GetSubmissionRequest {
            submission_id: submission.id.clone(),
        })
        .await?
        .into_inner()
        .submission
        .expect("submission fetched");
    assert_eq!(fetched.id, submission.id);
    assert_eq!(fetched.verification_statuses.len(), 1);
    let status = &fetched.verification_statuses[0];
    assert_eq!(status.check, "repo_proof");
    assert_eq!(status.status, "accepted");

    let mut ratings = RatingServiceClient::new(channel.clone());
    ratings
        .update_rating(UpdateRatingRequest {
            participant_id: "user-1".into(),
            outcome: Some(EventOutcome {
                position: 1,
                total_participants: 10,
                scale: EventScale::Global as i32,
                finished_at: (SystemTime::now() - Duration::from_secs(60))
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
                team_size: 1,
            }),
        })
        .await?;

    let rating = ratings
        .get_rating(GetRatingRequest {
            participant_id: "user-1".into(),
        })
        .await?
        .into_inner()
        .rating
        .expect("rating exists");
    assert!(rating.score > 0);

    let _ = shutdown.send(());
    let _ = server.await?;

    Ok(())
}

#[tokio::test]
async fn submit_solution_succeeds_with_repo_proof_endpoint() -> Result<()> {
    let persistence = Arc::new(InMemoryPersistence::default());
    seed_persistence(&persistence).await?;
    let storage = Arc::new(MemoryStorage::default());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let repo_addr = listener.local_addr()?;
    let (repo_shutdown_tx, repo_shutdown_rx) = tokio::sync::oneshot::channel();
    let repo_server = tokio::spawn(async move {
        let incoming = TcpListenerStream::new(listener);
        tonic::transport::Server::builder()
            .add_service(repo_proof::service())
            .serve_with_incoming_shutdown(incoming, async move {
                let _ = repo_shutdown_rx.await;
            })
            .await
            .map_err(Error::new)
    });

    let channel = Channel::from_shared(format!("http://{}", repo_addr))?
        .connect()
        .await?;
    let repo_proof = Arc::new(RepoProofClient::new(channel));

    let dir = tempdir()?;
    let socket_path = dir.path().join("hackcore.sock");

    let (server, shutdown) = start_server(
        persistence.clone(),
        storage.clone(),
        repo_proof,
        socket_path.clone(),
    )
    .await;

    tokio::time::sleep(Duration::from_millis(100)).await;
    let channel = connect(&socket_path).await?;
    let mut submits = SubmitsServiceClient::new(channel);
    let submission = submits
        .submit_solution(SubmitSolutionRequest {
            team_id: "team-1".into(),
            hackathon_id: "hack-1".into(),
            summary: "Great project".into(),
            repository_binding: Some(RepositoryBinding {
                provider: "github".into(),
                repository: "org/repo".into(),
                commit: "abc123".into(),
                is_private: false,
            }),
        })
        .await?
        .into_inner()
        .submission
        .expect("submission created");
    assert_eq!(submission.team_id, "team-1");

    let _ = shutdown.send(());
    let _ = server.await?;

    let _ = repo_shutdown_tx.send(());
    repo_server.await??;

    Ok(())
}

#[tokio::test]
async fn submit_solution_fails_when_repo_proof_rejects() -> Result<()> {
    let persistence = Arc::new(InMemoryPersistence::default());
    seed_persistence(&persistence).await?;
    let storage = Arc::new(MemoryStorage::default());
    let repo_proof = Arc::new(MockRepoProof::rejecting("commit not reachable"));

    let dir = tempdir()?;
    let socket_path = dir.path().join("hackcore.sock");

    let (server, shutdown) = start_server(
        persistence.clone(),
        storage.clone(),
        repo_proof.clone(),
        socket_path.clone(),
    )
    .await;

    tokio::time::sleep(Duration::from_millis(100)).await;

    let channel = connect(&socket_path).await?;
    let mut submits = SubmitsServiceClient::new(channel);

    let err = submits
        .submit_solution(SubmitSolutionRequest {
            team_id: "team-1".into(),
            hackathon_id: "hack-1".into(),
            summary: "Great project".into(),
            repository_binding: Some(RepositoryBinding {
                provider: "github".into(),
                repository: "org/repo".into(),
                commit: "abc123".into(),
                is_private: false,
            }),
        })
        .await
        .expect_err("submission should be rejected");
    assert_eq!(err.code(), tonic::Code::FailedPrecondition);
    assert!(err.message().contains("commit not reachable"));

    let stored = persistence.submissions.read().await;
    assert!(stored.is_empty());
    drop(stored);

    let health_checks = repo_proof.health_checks.lock().await;
    assert_eq!(*health_checks, 0);
    drop(health_checks);

    let verifications = repo_proof.verifications.lock().await;
    assert_eq!(*verifications, 1);
    drop(verifications);

    let artifacts = storage.calls.lock().await;
    assert!(artifacts.is_empty());
    drop(artifacts);

    let _ = shutdown.send(());
    let _ = server.await?;

    Ok(())
}
