use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use async_trait::async_trait;
use contracts::events::events_service_client::EventsServiceClient;
use contracts::events::GetEventRequest;
use contracts::participants::participants_service_client::ParticipantsServiceClient;
use contracts::participants::{GetParticipantRequest, ListParticipantsRequest};
use contracts::rating::rating_service_client::RatingServiceClient;
use contracts::rating::{EventOutcome, EventScale, GetRatingRequest, UpdateRatingRequest};
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
use tempfile::tempdir;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;
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
    calls: Mutex<Vec<String>>,
}

#[async_trait]
impl SubmissionArtifacts for MemoryStorage {
    async fn store_placeholder(&self, key: &str, _bytes: &[u8]) -> Result<()> {
        self.calls.lock().await.push(key.to_string());
        Ok(())
    }
}

#[derive(Default)]
struct MockRepoProof {
    invocations: Mutex<u32>,
}

#[async_trait]
impl RepoProofAdapter for MockRepoProof {
    async fn health_check(&self) -> Result<()> {
        *self.invocations.lock().await += 1;
        Ok(())
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

async fn start_server(
    persistence: Arc<InMemoryPersistence>,
    storage: Arc<MemoryStorage>,
    repo_proof: Arc<MockRepoProof>,
    socket_path: PathBuf,
) -> (JoinHandle<Result<()>>, tokio::sync::oneshot::Sender<()>) {
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

    let stored = storage.calls.lock().await;
    assert_eq!(stored.len(), 1);
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
