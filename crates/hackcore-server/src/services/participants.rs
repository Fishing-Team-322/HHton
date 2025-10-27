use std::sync::Arc;

use tonic::{Request, Response, Status};

use contracts::participants::{
    participants_service_server::ParticipantsService, GetParticipantRequest,
    GetParticipantResponse, ListParticipantsRequest, ListParticipantsResponse, Participant,
};
use core_domain::{model::AggregateRoot, user::User};

use crate::persistence::PersistenceGateway;

#[derive(Clone)]
pub struct ParticipantsHandler<P> {
    persistence: Arc<P>,
}

impl<P> ParticipantsHandler<P> {
    pub fn new(persistence: Arc<P>) -> Self {
        Self { persistence }
    }
}

#[tonic::async_trait]
impl<P> ParticipantsService for ParticipantsHandler<P>
where
    P: PersistenceGateway + 'static,
{
    async fn get_participant(
        &self,
        request: Request<GetParticipantRequest>,
    ) -> Result<Response<GetParticipantResponse>, Status> {
        let id = request.into_inner().participant_id;
        let maybe_user = self
            .persistence
            .get_user(&id)
            .await
            .map_err(internal_error)?;

        match maybe_user {
            Some(user) => Ok(Response::new(GetParticipantResponse {
                participant: Some(to_proto_participant(&user)),
            })),
            None => Err(Status::not_found(format!("participant '{id}' not found"))),
        }
    }

    async fn list_participants(
        &self,
        request: Request<ListParticipantsRequest>,
    ) -> Result<Response<ListParticipantsResponse>, Status> {
        let req = request.into_inner();
        let limit = (req.page_size.max(1)) as usize;
        let offset = req.page_token.parse::<usize>().unwrap_or_default();

        let users = self
            .persistence
            .list_users(limit, offset)
            .await
            .map_err(internal_error)?;

        let next_page_token = if users.len() == limit {
            Some((offset + limit).to_string())
        } else {
            None
        };

        let participants = users
            .into_iter()
            .map(|user| to_proto_participant(&user))
            .collect();

        Ok(Response::new(ListParticipantsResponse {
            participants,
            next_page_token: next_page_token.unwrap_or_default(),
        }))
    }
}

fn to_proto_participant(user: &User) -> Participant {
    let profile = user.profile();
    Participant {
        id: user.id().to_owned(),
        display_name: profile.display_name().value().to_string(),
        email: profile.email().value().to_string(),
    }
}

fn internal_error(error: anyhow::Error) -> Status {
    Status::internal(error.to_string())
}
