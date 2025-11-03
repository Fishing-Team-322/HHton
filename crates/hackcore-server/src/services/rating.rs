use std::sync::Arc;

use chrono::Utc;
use tonic::{Request, Response, Status};

use contracts::rating::{
    rating_service_server::RatingService, GetRatingRequest, GetRatingResponse, Rating,
    UpdateRatingRequest, UpdateRatingResponse,
};
use ratings::{
    aggregate_leaderboard, update_participant_profile, EventOutcome as DomainEventOutcome,
    ParticipantProfile, RatingFactors,
};

use crate::persistence::PersistenceGateway;

#[derive(Clone)]
pub struct RatingHandler<P> {
    persistence: Arc<P>,
    factors: Arc<RatingFactors>,
}

impl<P> RatingHandler<P> {
    pub fn new(persistence: Arc<P>) -> Self {
        Self {
            persistence,
            factors: Arc::new(RatingFactors::default()),
        }
    }

    #[cfg(test)]
    pub fn with_factors(persistence: Arc<P>, factors: RatingFactors) -> Self {
        Self {
            persistence,
            factors: Arc::new(factors),
        }
    }
}

#[tonic::async_trait]
impl<P> RatingService for RatingHandler<P>
where
    P: PersistenceGateway + 'static,
{
    async fn get_rating(
        &self,
        request: Request<GetRatingRequest>,
    ) -> Result<Response<GetRatingResponse>, Status> {
        let id = request.into_inner().participant_id;
        let profile = self
            .persistence
            .get_participant_rating_profile(&id)
            .await
            .map_err(internal_error)?
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
        let mut profile = self
            .persistence
            .get_participant_rating_profile(&participant_id)
            .await
            .map_err(internal_error)?
            .unwrap_or_else(|| ParticipantProfile::new(&participant_id, None));

        let persisted_outcome = domain_outcome.clone();
        update_participant_profile(&mut profile, domain_outcome, &factors, today)
            .map_err(internal_error)?;
        let current_rating = profile.rating;

        self.persistence
            .save_participant_rating_outcome(&profile, &persisted_outcome)
            .await
            .map_err(internal_error)?;

        let leaderboard_profiles = self
            .persistence
            .list_participant_rating_profiles()
            .await
            .map_err(internal_error)?;
        let leaderboard = aggregate_leaderboard(
            leaderboard_profiles
                .iter()
                .map(|profile| (profile.participant_id.as_str(), profile.events.as_slice())),
            &factors,
            today,
        )
        .map_err(internal_error)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::PostgresPersistence;
    use contracts::rating::{EventOutcome, EventScale, GetRatingRequest, UpdateRatingRequest};
    use sqlx::PgPool;
    use std::sync::Arc;
    use tonic::Request;

    #[sqlx::test(migrations = "../persistence/migrations")]
    async fn rating_state_survives_handler_restart(pool: PgPool) -> sqlx::Result<()> {
        sqlx::query("INSERT INTO users (id, display_name, email) VALUES ($1, $2, $3)")
            .bind("user-1")
            .bind("Alice")
            .bind("alice@example.com")
            .execute(&pool)
            .await?;

        let persistence = Arc::new(PostgresPersistence::new_with_pool(pool.clone()).await);
        let mut handler = RatingHandler::new(persistence.clone());

        handler
            .update_rating(Request::new(UpdateRatingRequest {
                participant_id: "user-1".into(),
                outcome: Some(EventOutcome {
                    position: 1,
                    total_participants: 10,
                    scale: EventScale::EVENT_SCALE_GLOBAL as i32,
                    finished_at: Utc::now().timestamp(),
                    team_size: 1,
                }),
            }))
            .await
            .expect("first rating update succeeds");

        drop(handler);

        let mut restarted = RatingHandler::new(persistence.clone());
        restarted
            .update_rating(Request::new(UpdateRatingRequest {
                participant_id: "user-1".into(),
                outcome: Some(EventOutcome {
                    position: 2,
                    total_participants: 12,
                    scale: EventScale::EVENT_SCALE_NATIONAL as i32,
                    finished_at: Utc::now().timestamp(),
                    team_size: 2,
                }),
            }))
            .await
            .expect("second rating update succeeds");

        let profile = persistence
            .get_participant_rating_profile("user-1")
            .await
            .expect("query succeeds")
            .expect("profile exists");
        assert_eq!(profile.events.len(), 2, "event history should persist");

        let rating = restarted
            .get_rating(Request::new(GetRatingRequest {
                participant_id: "user-1".into(),
            }))
            .await
            .expect("rating fetch succeeds")
            .into_inner()
            .rating
            .expect("rating present");

        assert!(rating.score > 0);
        assert!(rating.leaderboard_score > 0.0);

        Ok(())
    }
}
