use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use aws_credential_types::Credentials;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::Client;
use storage::ObjectStorage;
use wiremock::matchers::method;
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

fn extract_key(request: &Request) -> String {
    let path = request.url.path();
    let stripped = path.trim_start_matches('/');
    stripped
        .splitn(2, '/')
        .nth(1)
        .unwrap_or_default()
        .to_string()
}

fn shared_client(endpoint: &str) -> Client {
    let config = aws_sdk_s3::Config::builder()
        .region(Region::new("us-east-1"))
        .credentials_provider(Credentials::new("test", "test", None, None, "unit"))
        .force_path_style(true)
        .behavior_version_latest()
        .endpoint_url(endpoint)
        .build();
    Client::from_conf(config)
}

#[tokio::test]
async fn upload_get_and_delete_snapshot() {
    let server = MockServer::start().await;
    let state: Arc<Mutex<HashMap<String, Vec<u8>>>> = Arc::new(Mutex::new(HashMap::new()));

    let put_state = Arc::clone(&state);
    Mock::given(method("PUT"))
        .respond_with(move |request: &Request| {
            let key = extract_key(request);
            let mut store = put_state.lock().expect("state poisoned");
            store.insert(key, request.body.clone());
            ResponseTemplate::new(200)
        })
        .mount(&server)
        .await;

    let get_state = Arc::clone(&state);
    Mock::given(method("GET"))
        .respond_with(move |request: &Request| {
            let key = extract_key(request);
            let store = get_state.lock().expect("state poisoned");
            match store.get(&key) {
                Some(bytes) => ResponseTemplate::new(200).set_body_bytes(bytes.clone()),
                None => ResponseTemplate::new(404),
            }
        })
        .mount(&server)
        .await;

    let delete_state = Arc::clone(&state);
    Mock::given(method("DELETE"))
        .respond_with(move |request: &Request| {
            let key = extract_key(request);
            let mut store = delete_state.lock().expect("state poisoned");
            store.remove(&key);
            ResponseTemplate::new(204)
        })
        .mount(&server)
        .await;

    let client = shared_client(&server.uri());
    let storage = ObjectStorage::with_prefix(client, "test-bucket", Some("dev"));

    let archive = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let metadata = storage
        .upload_snapshot("hackathon-42", "submission-7", &archive)
        .await
        .expect("upload succeeded");

    assert_eq!(metadata.event_id, "hackathon-42");
    assert_eq!(metadata.submission_id, "submission-7");
    assert_eq!(metadata.archive_size, archive.len() as u64);

    let snapshot = storage
        .get_snapshot("hackathon-42", "submission-7")
        .await
        .expect("snapshot retrieved");

    assert_eq!(snapshot.archive, archive);
    assert_eq!(snapshot.metadata, metadata);

    storage
        .delete_snapshot("hackathon-42", "submission-7")
        .await
        .expect("deletion succeeded");

    let missing = storage.get_snapshot("hackathon-42", "submission-7").await;

    assert!(
        missing.is_err(),
        "snapshot should be missing after deletion"
    );
}
