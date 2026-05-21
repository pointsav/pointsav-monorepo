use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Default)]
pub struct ConsoleConfig {
    #[serde(default)]
    pub profile: ProfileConfig,
}

#[derive(Debug, Deserialize)]
pub struct ProfileConfig {
    #[serde(default = "default_username")]
    pub username: String,
    #[serde(default = "default_tenant")]
    pub tenant: String,
    #[serde(default = "default_proof_endpoint")]
    pub proof_endpoint: String,
    #[serde(default = "default_ingest_endpoint")]
    pub ingest_endpoint: String,
    #[serde(default = "default_totebox_endpoint")]
    pub totebox_endpoint: String,
    #[serde(default = "default_slm_endpoint")]
    pub slm_endpoint: String,
}

fn default_username() -> String { "operator".into() }
fn default_tenant() -> String { "local".into() }
fn default_proof_endpoint() -> String { "http://127.0.0.1:9092".into() }
fn default_ingest_endpoint() -> String { "http://127.0.0.1:9100".into() }
fn default_totebox_endpoint() -> String { "http://localhost:9000".into() }
fn default_slm_endpoint() -> String { "http://localhost:8011".into() }

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            username: default_username(),
            tenant: default_tenant(),
            proof_endpoint: default_proof_endpoint(),
            ingest_endpoint: default_ingest_endpoint(),
            totebox_endpoint: default_totebox_endpoint(),
            slm_endpoint: default_slm_endpoint(),
        }
    }
}

impl ConsoleConfig {
    pub fn load() -> Self {
        let path = config_path();
        std::fs::read_to_string(path)
            .ok()
            .and_then(|text| toml::from_str(&text).ok())
            .unwrap_or_default()
    }
}

fn config_path() -> PathBuf {
    let mut p = home_dir();
    p.push(".config/os-console/config.toml");
    p
}

fn home_dir() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}
