use std::sync::Arc;

use tonic::{Request, Response, Status};

use contracts::events::{
    events_service_server::EventsService, Event, GetEventRequest, GetEventResponse,
    ListEventsRequest, ListEventsResponse,
};
use persistence::repositories::hackathons::hackathon_to_contract_event;

use crate::persistence::PersistenceGateway;

#[derive(Clone)]
pub struct EventsHandler<P> {
    persistence: Arc<P>,
}

impl<P> EventsHandler<P> {
    pub fn new(persistence: Arc<P>) -> Self {
        Self { persistence }
    }
}

#[tonic::async_trait]
impl<P> EventsService for EventsHandler<P>
where
    P: PersistenceGateway + 'static,
{
    async fn get_event(
        &self,
        request: Request<GetEventRequest>,
    ) -> Result<Response<GetEventResponse>, Status> {
        let id = request.into_inner().event_id;
        let maybe_event = self
            .persistence
            .get_hackathon(&id)
            .await
            .map_err(internal_error)?;

        match maybe_event {
            Some(hackathon) => {
                let dto = hackathon_to_contract_event(&hackathon);
                Ok(Response::new(GetEventResponse { event: Some(dto) }))
            }
            None => Err(Status::not_found(format!("event '{id}' not found"))),
        }
    }

    async fn list_events(
        &self,
        request: Request<ListEventsRequest>,
    ) -> Result<Response<ListEventsResponse>, Status> {
        let req = request.into_inner();
        let limit = req.page_size.max(1) as usize;
        let offset = req.page_token.parse::<usize>().unwrap_or_default();

        let events = self
            .persistence
            .list_hackathons(limit, offset)
            .await
            .map_err(internal_error)?
            .into_iter()
            .map(|hackathon| hackathon_to_contract_event(&hackathon))
            .collect::<Vec<Event>>();

        let next_page_token = if events.len() == limit {
            (offset + limit).to_string()
        } else {
            String::new()
        };

        Ok(Response::new(ListEventsResponse {
            events,
            next_page_token,
        }))
    }
}

fn internal_error(error: anyhow::Error) -> Status {
    Status::internal(error.to_string())
}
