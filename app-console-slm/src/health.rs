use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct DoormanHealth {
    #[serde(default)]
    pub ai_available: bool,
    #[serde(default)]
    pub tier_b_circuit_state: String,
    #[serde(default)]
    pub entity_count: Option<u64>,
    #[serde(default)]
    pub active_tier: Option<String>,
}

pub fn fetch_readyz(endpoint: &str) -> anyhow::Result<DoormanHealth> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()?;
    let url = format!("{}/readyz", endpoint);
    let resp = client.get(&url).send()?;
    if resp.status().is_success() {
        Ok(resp.json::<DoormanHealth>()?)
    } else {
        anyhow::bail!("HTTP {}", resp.status())
    }
}
