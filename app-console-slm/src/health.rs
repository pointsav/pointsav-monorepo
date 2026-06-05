use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DoormanHealth {
    #[serde(default)]
    pub ai_available: bool,
    #[serde(default)]
    pub tier_b_circuit_state: String,
    #[serde(default)]
    pub entity_count: Option<u64>,
    #[serde(default)]
    pub active_tier: Option<String>,
    // P2-B: structured status fields
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub tier_a: bool,
    #[serde(default)]
    pub tier_a_reason: Option<String>,
    #[serde(default)]
    pub node_class: Option<String>,
    // Sprint 5B: brief queue snapshot
    #[serde(default)]
    pub queue_pending: Option<u64>,
    #[serde(default)]
    pub queue_done: Option<u64>,
    #[serde(default)]
    pub queue_poison: Option<u64>,
}

pub fn fetch_readyz(endpoint: &str) -> anyhow::Result<DoormanHealth> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()?;
    let url = format!("{}/readyz", endpoint);
    let resp = client.get(&url).send()?;
    let status = resp.status();
    // Accept 200 (ok) and 503 (closed — no tier available); both return structured JSON.
    if status.is_success() || status == reqwest::StatusCode::SERVICE_UNAVAILABLE {
        Ok(resp.json::<DoormanHealth>()?)
    } else {
        anyhow::bail!("HTTP {}", status)
    }
}
