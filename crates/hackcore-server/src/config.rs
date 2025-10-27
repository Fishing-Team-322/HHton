use anyhow::Result;
use serde::Deserialize;

const DEFAULT_SOCKET_PATH: &str = "/var/run/hackcore.sock";
const DEFAULT_MAX_CONNECTIONS: u32 = 5;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub database_url: String,
    #[serde(default = "default_socket_path")]
    pub socket_path: String,
    #[serde(default = "default_repo_proof_endpoint")]
    pub repo_proof_endpoint: String,
    pub s3_bucket: String,
    #[serde(default = "default_max_connections")]
    pub database_max_connections: u32,
}

impl Settings {
    pub fn load() -> Result<Self> {
        let mut builder = config::Config::builder();
        builder = builder.add_source(config::Environment::default().separator("__"));
        let settings: Settings = builder.build()?.try_deserialize()?;
        Ok(settings)
    }
}

fn default_socket_path() -> String {
    DEFAULT_SOCKET_PATH.to_string()
}

fn default_repo_proof_endpoint() -> String {
    "http://127.0.0.1:50060".to_string()
}

fn default_max_connections() -> u32 {
    DEFAULT_MAX_CONNECTIONS
}
