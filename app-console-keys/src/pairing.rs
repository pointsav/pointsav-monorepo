use std::sync::mpsc;
use std::time::Duration;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum PairingState {
    /// MBA probe failed; pairing request not yet sent
    Unpaired { fingerprint: String },
    /// POST pair/request sent; showing code, waiting for operator
    AwaitingApproval {
        code: String,
        request_id: String,
        fingerprint: String,
    },
    /// Operator approved; MBA link transitioning to active
    Approved,
    /// Operator declined
    Denied,
    /// Code expired (10-minute TTL)
    Expired,
    /// Network or server error
    Error(String),
}

impl Default for PairingState {
    fn default() -> Self {
        PairingState::Unpaired {
            fingerprint: String::new(),
        }
    }
}

#[derive(Debug)]
pub enum PairingEvent {
    Approved,
    Denied,
    Expired,
    Error(String),
}

#[derive(Serialize)]
struct PairRequest<'a> {
    username: &'a str,
    tenant: &'a str,
    public_key: &'a str,
    fingerprint: &'a str,
}

#[derive(Deserialize)]
struct PairResponse {
    request_id: String,
    code: String,
}

#[derive(Deserialize)]
struct StatusResponse {
    state: String,
}

/// POST /v1/pair/request — returns (request_id, code) on success.
pub fn post_pair_request(
    base_url: &str,
    username: &str,
    tenant: &str,
    public_key: &str,
    fingerprint: &str,
) -> Result<(String, String)> {
    let url = format!("{}/v1/pair/request", base_url.trim_end_matches('/'));
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    let resp: PairResponse = client
        .post(&url)
        .json(&PairRequest {
            username,
            tenant,
            public_key,
            fingerprint,
        })
        .send()?
        .error_for_status()?
        .json()?;
    Ok((resp.request_id, resp.code))
}

/// Spawns a background thread that polls /v1/pair/status/{request_id}.
/// Returns the receiver end; the thread sends a single terminal PairingEvent then exits.
pub fn spawn_status_poll(base_url: String, request_id: String) -> mpsc::Receiver<PairingEvent> {
    let (tx, rx) = mpsc::channel::<PairingEvent>();
    std::thread::spawn(move || {
        let client = match reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.send(PairingEvent::Error(e.to_string()));
                return;
            }
        };
        loop {
            let url = format!(
                "{}/v1/pair/status/{}",
                base_url.trim_end_matches('/'),
                request_id
            );
            match client
                .get(&url)
                .send()
                .and_then(|r| r.json::<StatusResponse>())
            {
                Ok(s) => match s.state.as_str() {
                    "approved" => {
                        let _ = tx.send(PairingEvent::Approved);
                        break;
                    }
                    "denied" => {
                        let _ = tx.send(PairingEvent::Denied);
                        break;
                    }
                    "expired" => {
                        let _ = tx.send(PairingEvent::Expired);
                        break;
                    }
                    _ => {} // still pending — wait before next poll
                },
                Err(e) => {
                    let _ = tx.send(PairingEvent::Error(e.to_string()));
                    std::thread::sleep(Duration::from_secs(3));
                    continue;
                }
            }
            std::thread::sleep(Duration::from_secs(2));
        }
    });
    rx
}
