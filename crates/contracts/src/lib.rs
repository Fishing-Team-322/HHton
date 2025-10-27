//! Generated gRPC contracts for the Hackathon Hub services.

pub mod participants {
    tonic::include_proto!("hh.contracts.participants.v1");
}

pub mod events {
    tonic::include_proto!("hh.contracts.events.v1");
}

pub mod submits {
    tonic::include_proto!("hh.contracts.submits.v1");
}

pub mod rating {
    tonic::include_proto!("hh.contracts.rating.v1");
}

pub mod repo_proof {
    tonic::include_proto!("hh.contracts.repo_proof.v1");
}

#[cfg(test)]
mod tests {
    use super::events::events_service_client::EventsServiceClient;
    use super::participants::participants_service_client::ParticipantsServiceClient;
    use super::rating::rating_service_client::RatingServiceClient;
    use super::repo_proof::repo_proof_service_client::RepoProofServiceClient;
    use super::submits::submits_service_client::SubmitsServiceClient;
    use tonic::transport::Channel;

    fn assert_client<T: Clone + Send + Sync + 'static>() {}

    #[test]
    fn clients_are_constructible() {
        assert_client::<RepoProofServiceClient<Channel>>();
        assert_client::<ParticipantsServiceClient<Channel>>();
        assert_client::<EventsServiceClient<Channel>>();
        assert_client::<SubmitsServiceClient<Channel>>();
        assert_client::<RatingServiceClient<Channel>>();
    }
}
