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
    use super::events::Event;
    use super::participants::participants_service_client::ParticipantsServiceClient;
    use super::rating::rating_service_client::RatingServiceClient;
    use super::repo_proof::repo_proof_service_client::RepoProofServiceClient;
    use super::submits::submits_service_client::SubmitsServiceClient;
    use super::submits::{
        RepositoryBinding, Submission, SubmitSolutionRequest, VerificationStatus,
    };
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

    #[test]
    fn submissions_contract_exposes_new_fields() {
        let binding = RepositoryBinding {
            provider: "github".to_string(),
            repository: "org/repo".to_string(),
            commit: "abc123".to_string(),
            is_private: true,
        };

        let request = SubmitSolutionRequest {
            team_id: "team-1".to_string(),
            hackathon_id: "hack-1".to_string(),
            summary: "Build something great".to_string(),
            repository_binding: Some(binding.clone()),
        };

        let status = VerificationStatus {
            check: "repo_proof".to_string(),
            status: "passed".to_string(),
            details: "all good".to_string(),
            checked_at: 1_700_000_000,
        };

        let submission = Submission {
            id: "submission-1".to_string(),
            team_id: request.team_id.clone(),
            hackathon_id: request.hackathon_id.clone(),
            summary: request.summary.clone(),
            repository_binding: request.repository_binding.clone(),
            created_at: 1_699_999_999,
            verification_statuses: vec![status],
            submission_deadline: 1_700_000_100,
        };

        assert_eq!(submission.team_id, request.team_id);
        assert_eq!(submission.repository_binding.unwrap(), binding);
        assert!(submission
            .verification_statuses
            .iter()
            .any(|status| status.check == "repo_proof"));
    }

    #[test]
    fn events_contract_includes_hackathon_schedule() {
        let event = Event {
            id: "hack-42".to_string(),
            name: "Hackathon".to_string(),
            description: "Build the future".to_string(),
            start_time: 1_700_000_000,
            end_time: 1_700_086_400,
            registration_deadline: 1_699_999_900,
            submission_deadline: 1_700_086_300,
            team_size_limit: 5,
        };

        assert!(event.registration_deadline < event.submission_deadline);
        assert_eq!(event.team_size_limit, 5);
    }
}
