use std::net::SocketAddr;

use anyhow::Context;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let addr =
        std::env::var("REPO_PROOF_LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:50060".to_string());
    let addr: SocketAddr = addr.parse().context("invalid REPO_PROOF_LISTEN_ADDR")?;

    repo_proof::serve(addr, async {
        let _ = tokio::signal::ctrl_c().await;
    })
    .await
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}
