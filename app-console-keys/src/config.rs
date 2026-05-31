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
    #[serde(default = "default_totebox_host")]
    pub totebox_host: String,
    #[serde(default = "default_totebox_ssh_port")]
    pub totebox_ssh_port: u16,
    #[serde(default = "default_ssh_key_path")]
    pub ssh_key_path: String,
    #[serde(default = "default_totebox_endpoint")]
    pub totebox_endpoint: String,
    #[serde(default = "default_slm_endpoint")]
    pub slm_endpoint: String,
    #[serde(default = "default_pair_endpoint")]
    pub pair_endpoint: String,
    #[serde(default = "default_drafts_outbound_path")]
    pub drafts_outbound_path: String,
    #[serde(default = "default_email_endpoint")]
    pub email_endpoint: String,
    #[serde(default = "default_orchestration_host")]
    pub orchestration_host: String,
    #[serde(default)]
    pub plain_mode: bool,
}

fn default_username() -> String {
    "operator".into()
}
fn default_tenant() -> String {
    "local".into()
}
fn default_proof_endpoint() -> String {
    "http://127.0.0.1:9092".into()
}
fn default_ingest_endpoint() -> String {
    "http://127.0.0.1:9100".into()
}
fn default_totebox_host() -> String {
    "127.0.0.1".into()
}
fn default_totebox_ssh_port() -> u16 {
    2222
}
fn default_ssh_key_path() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    format!("{}/.ssh/id_ed25519", home)
}
fn default_totebox_endpoint() -> String {
    "http://localhost:9000".into()
}
fn default_slm_endpoint() -> String {
    "http://localhost:9080".into()
}
fn default_pair_endpoint() -> String {
    "http://127.0.0.1:9201".into()
}
fn default_drafts_outbound_path() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    format!("{}/.local/share/os-console/drafts-outbound", home)
}
fn default_email_endpoint() -> String {
    "http://127.0.0.1:9093".into()
}
fn default_orchestration_host() -> String {
    "127.0.0.1".into()
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            username: default_username(),
            tenant: default_tenant(),
            proof_endpoint: default_proof_endpoint(),
            ingest_endpoint: default_ingest_endpoint(),
            totebox_host: default_totebox_host(),
            totebox_ssh_port: default_totebox_ssh_port(),
            ssh_key_path: default_ssh_key_path(),
            totebox_endpoint: default_totebox_endpoint(),
            slm_endpoint: default_slm_endpoint(),
            pair_endpoint: default_pair_endpoint(),
            drafts_outbound_path: default_drafts_outbound_path(),
            email_endpoint: default_email_endpoint(),
            orchestration_host: default_orchestration_host(),
            plain_mode: false,
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
