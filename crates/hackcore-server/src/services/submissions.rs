use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tonic::{Request, Response, Status};
use uuid::Uuid;

use contracts::repo_proof::verification_verdict::Outcome as RepoProofOutcome;
use contracts::submits::{
    submits_service_server::SubmitsService, GetSubmissionRequest, GetSubmissionResponse,
    RepositoryBinding as ProtoRepositoryBinding, Submission as ProtoSubmission,
    SubmitSolutionRequest, SubmitSolutionResponse,
};
use core_domain::{
    model::{AggregateRoot, EntityId},
    services::HackathonRules,
    submission::{
        RepositoryBinding, Submission, SubmissionSummary, VerificationOutcome, VerificationResult,
    },
};
use prost_types::{value::Kind, ListValue, Struct, Value};
use serde_json::Value as JsonValue;

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

        if team.hackathon_id().0 != req.hackathon_id {
            return Err(Status::failed_precondition(format!(
                "team '{}' is not registered for hackathon '{}'",
                req.team_id, req.hackathon_id
            )));
        }

        HackathonRules::ensure_team_size_limit(hackathon.team_size_limit(), team.member_count())
            .map_err(|err| Status::failed_precondition(err.to_string()))?;

        let verification_response = self
            .repo_proof
            .verify_binding(
                &binding.provider,
                &binding.repository,
                &binding.commit,
                binding.is_private,
            )
            .await
            .map_err(internal_error)?;

        let repository_binding = RepositoryBinding::new(
            binding.provider,
            binding.repository,
            binding.commit,
            binding.is_private,
        )
        .map_err(domain_error)?;
        let summary = SubmissionSummary::new(req.summary).map_err(domain_error)?;
        let submission_id = format!("submission-{}", Uuid::new_v4());
        let mut submission = Submission::new(
            EntityId(submission_id.clone()),
            EntityId(req.team_id.clone()),
            EntityId(req.hackathon_id.clone()),
            summary,
            repository_binding,
            SystemTime::now(),
        )
        .map_err(domain_error)?;

        if let Some(verification) = extract_verification(&verification_response)? {
            if verification.outcome() == VerificationOutcome::Rejected {
                return Err(Status::failed_precondition(
                    verification
                        .reason()
                        .unwrap_or("repository binding rejected by verifier"),
                ));
            }
            submission.set_verification(verification);
        }

        self.repo_proof
            .health_check()
            .await
            .map_err(internal_error)?;

        let stored = self
            .persistence
            .insert_submission(submission.clone())
            .await
            .map_err(internal_error)?;

        self.storage
            .store_placeholder(
                stored.hackathon_id().0.as_ref(),
                stored.id(),
                stored.summary().value().as_bytes(),
            )
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
    let verification_statuses = submission
        .verification()
        .map(|verification| contracts::submits::VerificationStatus {
            check: "repo_proof".to_string(),
            status: verification.outcome().as_str().to_lowercase(),
            details: verification
                .reason()
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| "verified".to_string()),
            checked_at: system_time_to_epoch(verification.checked_at()),
        })
        .into_iter()
        .collect();
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
        verification_statuses,
        submission_deadline: deadline.map(system_time_to_epoch).unwrap_or_default(),
    }
}

fn extract_verification(
    response: &contracts::repo_proof::VerifyBindingResponse,
) -> Result<Option<VerificationResult>, Status> {
    let verdict = match response.verdict.as_ref() {
        Some(verdict) => verdict,
        None => return Ok(None),
    };

    let outcome = match RepoProofOutcome::try_from(verdict.outcome)
        .map_err(|_| Status::internal("verification verdict outcome is missing or unknown"))?
    {
        RepoProofOutcome::Accepted => VerificationOutcome::Accepted,
        RepoProofOutcome::Rejected => VerificationOutcome::Rejected,
        RepoProofOutcome::Unspecified => {
            return Err(Status::internal(
                "verification verdict outcome is missing or unknown",
            ))
        }
    };

    let checked_at = epoch_to_system_time(verdict.checked_at);
    let reason = if verdict.reason.is_empty() {
        None
    } else {
        Some(verdict.reason.clone())
    };
    let proof = verdict
        .proof
        .as_ref()
        .map(struct_to_json)
        .transpose()
        .map_err(internal_error)?;

    Ok(Some(VerificationResult::new(
        outcome, reason, checked_at, proof,
    )))
}

fn system_time_to_epoch(time: SystemTime) -> i64 {
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as i64,
        Err(err) => -(err.duration().as_secs() as i64),
    }
}

fn epoch_to_system_time(epoch: i64) -> SystemTime {
    if epoch >= 0 {
        UNIX_EPOCH + Duration::from_secs(epoch as u64)
    } else {
        UNIX_EPOCH - Duration::from_secs(epoch.unsigned_abs())
    }
}

fn struct_to_json(data: &Struct) -> anyhow::Result<JsonValue> {
    let mut map = serde_json::Map::new();
    for (key, value) in &data.fields {
        map.insert(key.clone(), value_to_json(value)?);
    }
    Ok(JsonValue::Object(map))
}

fn list_to_json(list: &ListValue) -> anyhow::Result<JsonValue> {
    let mut values = Vec::with_capacity(list.values.len());
    for value in &list.values {
        values.push(value_to_json(value)?);
    }
    Ok(JsonValue::Array(values))
}

fn value_to_json(value: &Value) -> anyhow::Result<JsonValue> {
    match value.kind.as_ref() {
        Some(Kind::NullValue(_)) | None => Ok(JsonValue::Null),
        Some(Kind::NumberValue(number)) => serde_json::Number::from_f64(*number)
            .map(JsonValue::Number)
            .ok_or_else(|| anyhow::anyhow!("invalid number value")),
        Some(Kind::StringValue(string)) => Ok(JsonValue::String(string.clone())),
        Some(Kind::BoolValue(boolean)) => Ok(JsonValue::Bool(*boolean)),
        Some(Kind::StructValue(struct_value)) => struct_to_json(struct_value),
        Some(Kind::ListValue(list)) => list_to_json(list),
    }
}

fn internal_error(error: anyhow::Error) -> Status {
    Status::internal(error.to_string())
}

fn domain_error(error: anyhow::Error) -> Status {
    Status::invalid_argument(error.to_string())
}
