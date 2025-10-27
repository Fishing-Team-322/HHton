use std::collections::HashMap;
use std::sync::Arc;

use chrono::Utc;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};

use contracts::rating::{
    rating_service_server::RatingService, GetRatingRequest, GetRatingResponse, Rating,
    UpdateRatingRequest, UpdateRatingResponse,
};
use ratings::{
    aggregate_leaderboard, update_participant_profile, EventOutcome as DomainEventOutcome,
    ParticipantProfile, RatingFactors,
};

#[derive(Clone)]
pub struct RatingHandler {
    profiles: Arc<RwLock<HashMap<String, ParticipantProfile>>>,
    factors: Arc<RatingFactors>,
}

impl Default for RatingHandler {
    fn default() -> Self {
        Self {
            profiles: Arc::new(RwLock::new(HashMap::new())),
            factors: Arc::new(RatingFactors::default()),
        }
    }
}

#[tonic::async_trait]
impl RatingService for RatingHandler {
    async fn get_rating(
        &self,
        request: Request<GetRatingRequest>,
    ) -> Result<Response<GetRatingResponse>, Status> {
        let id = request.into_inner().participant_id;
        let profiles = self.profiles.read().await;
        let profile = profiles
            .get(&id)
            .cloned()
            .unwrap_or_else(|| ParticipantProfile::new(&id, None));

        Ok(Response::new(GetRatingResponse {
            rating: Some(to_proto_rating(&profile)),
        }))
    }

    async fn update_rating(
        &self,
        request: Request<UpdateRatingRequest>,
    ) -> Result<Response<UpdateRatingResponse>, Status> {
        let req = request.into_inner();
        let outcome = req
            .outcome
            .ok_or_else(|| Status::invalid_argument("outcome must be provided"))?;
        let domain_outcome = DomainEventOutcome::try_from(outcome).map_err(internal_error)?;

        let participant_id = req.participant_id;
        let today = Utc::now().date_naive();

        let factors = self.factors.clone();
        let mut profiles_guard = self.profiles.write().await;
        let profile = profiles_guard
            .entry(participant_id.clone())
            .or_insert_with(|| ParticipantProfile::new(participant_id.clone(), None));

        update_participant_profile(profile, domain_outcome, &factors, today)
            .map_err(internal_error)?;
        let current_rating = profile.rating;
        drop(profiles_guard);

        let leaderboard = {
            let profiles = self.profiles.read().await;
            aggregate_leaderboard(
                profiles
                    .iter()
                    .map(|(id, profile)| (id.as_str(), profile.events.as_slice())),
                &factors,
                today,
            )
            .map_err(internal_error)?
        };

        let leaderboard_score = leaderboard
            .into_iter()
            .find(|entry| entry.subject_id == participant_id)
            .map(|entry| entry.score)
            .unwrap_or(current_rating);

        let rating = Rating {
            participant_id: participant_id.clone(),
            score: current_rating.round() as i32,
            updated_at: Utc::now().timestamp(),
            leaderboard_score,
        };

        Ok(Response::new(UpdateRatingResponse {
            rating: Some(rating),
        }))
    }
}

fn to_proto_rating(profile: &ParticipantProfile) -> Rating {
    Rating {
        participant_id: profile.participant_id.clone(),
        score: profile.rating.round() as i32,
        updated_at: Utc::now().timestamp(),
        leaderboard_score: profile.rating,
    }
}

fn internal_error(error: anyhow::Error) -> Status {
    Status::internal(error.to_string())
}
