fn main() {
    let proto_files = [
        "proto/participants.proto",
        "proto/events.proto",
        "proto/submits.proto",
        "proto/rating.proto",
        "proto/repo_proof.proto",
    ];

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(&proto_files, &["proto"])
        .expect("failed to compile gRPC definitions");
}
