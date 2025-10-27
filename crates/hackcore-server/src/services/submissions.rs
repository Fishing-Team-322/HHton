use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use tonic::{Request, Response, Status};
use uuid::Uuid;

use contracts::submits::{
    submits_service_server::SubmitsService, GetSubmissionRequest, GetSubmissionResponse,
    RepositoryBinding as ProtoRepositoryBinding, Submission as ProtoSubmission,
    SubmitSolutionRequest, SubmitSolutionResponse,
};
use core_domain::{
    model::{AggregateRoot, EntityId},
    services::HackathonRules,
    submission::{RepositoryBinding, Submission, SubmissionSummary},
};

use crate::{
    persistence::PersistenceGateway,
    services::{RepoProofAdapter, SubmissionArtifacts},
};

#[derive(Clone)]
pub struct SubmissionsService<P, S, R> {
    persistence: Arc<P>,
    storage: Arc<S>,
    repo_proof: Arc<R>,
}

impl<P, S, R> SubmissionsService<P, S, R> {
    pub fn new(persistence: Arc<P>, storage: Arc<S>, repo_proof: Arc<R>) -> Self {
        Self {
            persistence,
            storage,
            repo_proof,
        }
    }
}

#[tonic::async_trait]
impl<P, S, R> SubmitsService for SubmissionsService<P, S, R>
where
    P: PersistenceGateway + 'static,
    S: SubmissionArtifacts + 'static,
    R: RepoProofAdapter + 'static,
{
    async fn submit_solution(
        &self,
        request: Request<SubmitSolutionRequest>,
    ) -> Result<Response<SubmitSolutionResponse>, Status> {
        let req = request.into_inner();
        let binding = req
            .repository_binding
            .ok_or_else(|| Status::invalid_argument("repository_binding must be provided"))?;

        let hackathon = self
            .persistence
            .get_hackathon(&req.hackathon_id)
            .await
            .map_err(internal_error)?
            .ok_or_else(|| {
                Status::not_found(format!("hackathon '{}' not found", req.hackathon_id))
            })?;

        HackathonRules::ensure_submission_open(&hackathon, SystemTime::now())
            .map_err(|err| Status::failed_precondition(err.to_string()))?;

        let team = self
            .persistence
            .get_team(&req.team_id)
            .await
            .map_err(internal_error)?
            .ok_or_else(|| Status::not_found(format!("team '{}' not found", req.team_id)))?;

        HackathonRules::ensure_team_size_limit(hackathon.team_size_limit(), team.member_count())
            .map_err(|err| Status::failed_precondition(err.to_string()))?;

        let repository_binding = RepositoryBinding::new(
            binding.provider,
            binding.repository,
            binding.commit,
            binding.is_private,
        )
        .map_err(domain_error)?;
        let summary = SubmissionSummary::new(req.summary).map_err(domain_error)?;
        let submission_id = format!("submission-{}", Uuid::new_v4());
        let submission = Submission::new(
            EntityId(submission_id.clone()),
            EntityId(req.team_id.clone()),
            EntityId(req.hackathon_id.clone()),
            summary,
            repository_binding,
            SystemTime::now(),
        )
        .map_err(domain_error)?;

        self.repo_proof
            .health_check()
            .await
            .map_err(internal_error)?;

        let stored = self
            .persistence
            .insert_submission(submission.clone())
            .await
            .map_err(internal_error)?;

        let placeholder_key = format!("submissions/{}/summary", stored.id());
        self.storage
            .store_placeholder(&placeholder_key, stored.summary().value().as_bytes())
            .await
            .map_err(internal_error)?;

        let response_submission =
            to_proto_submission(&stored, Some(hackathon.submission_deadline()));

        Ok(Response::new(SubmitSolutionResponse {
            submission: Some(response_submission),
        }))
    }

    async fn get_submission(
        &self,
        request: Request<GetSubmissionRequest>,
    ) -> Result<Response<GetSubmissionResponse>, Status> {
        let id = request.into_inner().submission_id;
        let submission = self
            .persistence
            .get_submission(&id)
            .await
            .map_err(internal_error)?
            .ok_or_else(|| Status::not_found(format!("submission '{id}' not found")))?;

        let hackathon_id = submission.hackathon_id().0.clone();
        let hackathon = self
            .persistence
            .get_hackathon(&hackathon_id)
            .await
            .map_err(internal_error)?;
        let deadline = hackathon.as_ref().map(|hack| hack.submission_deadline());

        let submission = to_proto_submission(&submission, deadline);
        Ok(Response::new(GetSubmissionResponse {
            submission: Some(submission),
        }))
    }
}

fn to_proto_submission(submission: &Submission, deadline: Option<SystemTime>) -> ProtoSubmission {
    let binding = submission.repository_binding();
    ProtoSubmission {
        id: submission.id().to_owned(),
        team_id: submission.team_id().0.clone(),
        hackathon_id: submission.hackathon_id().0.clone(),
        summary: submission.summary().value().to_string(),
        repository_binding: Some(ProtoRepositoryBinding {
            provider: binding.provider().to_string(),
            repository: binding.repository().to_string(),
            commit: binding.reference().to_string(),
            is_private: binding.is_private(),
        }),
        created_at: system_time_to_epoch(submission.created_at()),
        verification_statuses: Vec::new(),
        submission_deadline: deadline.map(system_time_to_epoch).unwrap_or_default(),
    }
}

fn system_time_to_epoch(time: SystemTime) -> i64 {
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as i64,
        Err(err) => -(err.duration().as_secs() as i64),
    }
}

fn internal_error(error: anyhow::Error) -> Status {
    Status::internal(error.to_string())
}

fn domain_error(error: anyhow::Error) -> Status {
    Status::invalid_argument(error.to_string())
}
