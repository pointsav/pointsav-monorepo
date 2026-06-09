// SPDX-License-Identifier: Apache-2.0 OR MIT

//! GCP Compute Engine client — start, stop, and poll a burst GPU VM.
//!
//! The Doorman drives a Yo-Yo node's power state through the Compute Engine
//! REST API. Authentication uses the instance metadata server: on a GCE VM,
//! `GET http://metadata.google.internal/.../token` returns a short-lived OAuth
//! access token for the attached service account, with no credentials file on
//! disk. This is the same Application Default Credentials path the rest of the
//! workspace uses.
//!
//! Three operations matter:
//!
//! - [`GcpComputeClient::start_instance`] — POST `.../instances/{i}/start`.
//! - [`GcpComputeClient::stop_instance`] — POST `.../instances/{i}/stop`.
//! - [`GcpComputeClient::instance_status`] — GET `.../instances/{i}`, read the
//!   `status` field (PROVISIONING / STAGING / RUNNING / STOPPING / TERMINATED).
//!
//! Start and stop return an `Operation` resource that completes
//! asynchronously; [`GcpComputeClient::wait_operation`] polls the zone
//! operations endpoint until `status == "DONE"`.
//!
//! ## Testability
//!
//! Both the API base URL and the metadata URL are injectable, so the unit
//! tests point them at a `wiremock` server. In production they default to the
//! real Google endpoints.

use std::time::Duration;

use serde::Deserialize;

use crate::error::{DoormanError, Result};

/// Default Compute Engine REST base (v1).
pub const DEFAULT_COMPUTE_BASE: &str = "https://compute.googleapis.com/compute/v1";
/// Default metadata server base.
pub const DEFAULT_METADATA_BASE: &str = "http://metadata.google.internal";

/// Lifecycle status of a Compute Engine instance, parsed from the `status`
/// field of the instance resource.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstanceStatus {
    /// PROVISIONING or STAGING — booting.
    Staging,
    /// RUNNING.
    Running,
    /// STOPPING / SUSPENDING — winding down.
    Stopping,
    /// TERMINATED / SUSPENDED — off.
    Terminated,
    /// Any other / unrecognized status string (carried verbatim).
    Other(String),
}

impl InstanceStatus {
    fn from_api(s: &str) -> Self {
        match s {
            "PROVISIONING" | "STAGING" | "REPAIRING" => InstanceStatus::Staging,
            "RUNNING" => InstanceStatus::Running,
            "STOPPING" | "SUSPENDING" => InstanceStatus::Stopping,
            "TERMINATED" | "SUSPENDED" => InstanceStatus::Terminated,
            other => InstanceStatus::Other(other.to_string()),
        }
    }
}

/// A Compute Engine client bound to one project + zone + instance.
#[derive(Clone, Debug)]
pub struct GcpComputeClient {
    http: reqwest::Client,
    project: String,
    zone: String,
    instance: String,
    compute_base: String,
    metadata_base: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct InstanceResource {
    #[serde(default)]
    status: String,
}

#[derive(Deserialize)]
struct OperationResource {
    #[serde(default)]
    status: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    error: Option<serde_json::Value>,
}

impl GcpComputeClient {
    /// Construct a client for an instance using the default Google endpoints.
    pub fn new(
        project: impl Into<String>,
        zone: impl Into<String>,
        instance: impl Into<String>,
    ) -> Self {
        Self::with_endpoints(
            project,
            zone,
            instance,
            DEFAULT_COMPUTE_BASE,
            DEFAULT_METADATA_BASE,
        )
    }

    /// Construct a client with explicit endpoints (used by tests to point at a
    /// mock server).
    pub fn with_endpoints(
        project: impl Into<String>,
        zone: impl Into<String>,
        instance: impl Into<String>,
        compute_base: impl Into<String>,
        metadata_base: impl Into<String>,
    ) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_default();
        Self {
            http,
            project: project.into(),
            zone: zone.into(),
            instance: instance.into(),
            compute_base: compute_base.into().trim_end_matches('/').to_string(),
            metadata_base: metadata_base.into().trim_end_matches('/').to_string(),
        }
    }

    /// The instance label this client controls.
    pub fn instance(&self) -> &str {
        &self.instance
    }

    /// Fetch a fresh OAuth access token from the metadata server. The token is
    /// short-lived (≈1h) and inexpensive to fetch; callers fetch per request
    /// rather than caching, which keeps the code simple and the metadata
    /// server itself caches and refreshes before expiry.
    pub async fn access_token(&self) -> Result<String> {
        let url = format!(
            "{}/computeMetadata/v1/instance/service-accounts/default/token",
            self.metadata_base
        );
        let resp = self
            .http
            .get(&url)
            .header("Metadata-Flavor", "Google")
            .send()
            .await
            .map_err(|e| DoormanError::GcpApi {
                operation: "token",
                reason: format!("metadata transport: {e}"),
            })?;
        if !resp.status().is_success() {
            return Err(DoormanError::GcpApi {
                operation: "token",
                reason: format!("metadata status {}", resp.status()),
            });
        }
        let body: TokenResponse = resp.json().await.map_err(|e| DoormanError::GcpApi {
            operation: "token",
            reason: format!("metadata decode: {e}"),
        })?;
        Ok(body.access_token)
    }

    /// Issue a start. Returns the operation name to poll with
    /// [`GcpComputeClient::wait_operation`].
    pub async fn start_instance(&self) -> Result<String> {
        self.power_op("start").await
    }

    /// Issue a stop. Returns the operation name.
    pub async fn stop_instance(&self) -> Result<String> {
        self.power_op("stop").await
    }

    async fn power_op(&self, verb: &'static str) -> Result<String> {
        let token = self.access_token().await?;
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}/{}",
            self.compute_base, self.project, self.zone, self.instance, verb
        );
        let resp = self
            .http
            .post(&url)
            .bearer_auth(token)
            .header("Content-Length", "0")
            .send()
            .await
            .map_err(|e| DoormanError::GcpApi {
                operation: leak_verb(verb),
                reason: format!("transport: {e}"),
            })?;
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(DoormanError::GcpApi {
                operation: leak_verb(verb),
                reason: format!("status {status}: {}", truncate(&body, 300)),
            });
        }
        let op: OperationResource =
            serde_json::from_str(&body).map_err(|e| DoormanError::GcpApi {
                operation: leak_verb(verb),
                reason: format!("operation decode: {e}"),
            })?;
        Ok(op.name)
    }

    /// Read the instance's current lifecycle status.
    pub async fn instance_status(&self) -> Result<InstanceStatus> {
        let token = self.access_token().await?;
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}",
            self.compute_base, self.project, self.zone, self.instance
        );
        let resp = self
            .http
            .get(&url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| DoormanError::GcpApi {
                operation: "status",
                reason: format!("transport: {e}"),
            })?;
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(DoormanError::GcpApi {
                operation: "status",
                reason: format!("status {status}: {}", truncate(&body, 300)),
            });
        }
        let inst: InstanceResource =
            serde_json::from_str(&body).map_err(|e| DoormanError::GcpApi {
                operation: "status",
                reason: format!("instance decode: {e}"),
            })?;
        Ok(InstanceStatus::from_api(&inst.status))
    }

    /// Poll a zone operation until it reports `DONE`, or until `max_polls`
    /// polls have elapsed (each spaced by `interval`). Returns `Ok(())` when
    /// the operation completes without an error field; returns
    /// [`DoormanError::GcpApi`] if the operation reports an error or never
    /// completes within the budget.
    pub async fn wait_operation(
        &self,
        operation: &str,
        interval: Duration,
        max_polls: u32,
    ) -> Result<()> {
        for _ in 0..max_polls {
            let token = self.access_token().await?;
            let url = format!(
                "{}/projects/{}/zones/{}/operations/{}",
                self.compute_base, self.project, self.zone, operation
            );
            let resp = self
                .http
                .get(&url)
                .bearer_auth(token)
                .send()
                .await
                .map_err(|e| DoormanError::GcpApi {
                    operation: "wait-operation",
                    reason: format!("transport: {e}"),
                })?;
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            if !status.is_success() {
                return Err(DoormanError::GcpApi {
                    operation: "wait-operation",
                    reason: format!("status {status}: {}", truncate(&body, 300)),
                });
            }
            let op: OperationResource =
                serde_json::from_str(&body).map_err(|e| DoormanError::GcpApi {
                    operation: "wait-operation",
                    reason: format!("operation decode: {e}"),
                })?;
            if op.status == "DONE" {
                if let Some(err) = op.error {
                    return Err(DoormanError::GcpApi {
                        operation: "wait-operation",
                        reason: format!(
                            "operation {} failed: {}",
                            op.name,
                            truncate(&err.to_string(), 300)
                        ),
                    });
                }
                return Ok(());
            }
            tokio::time::sleep(interval).await;
        }
        Err(DoormanError::GcpApi {
            operation: "wait-operation",
            reason: format!("operation {operation} did not complete within budget"),
        })
    }
}

/// Map the dynamic verb to a `'static` operation label for the error variant.
fn leak_verb(verb: &str) -> &'static str {
    match verb {
        "start" => "start",
        "stop" => "stop",
        _ => "power-op",
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// Stand up a mock metadata token endpoint on the given server.
    async fn mock_token(server: &MockServer) {
        Mock::given(method("GET"))
            .and(path(
                "/computeMetadata/v1/instance/service-accounts/default/token",
            ))
            .and(header("Metadata-Flavor", "Google"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "ya29.mock",
                "expires_in": 3599,
                "token_type": "Bearer"
            })))
            .mount(server)
            .await;
    }

    fn client_for(server: &MockServer) -> GcpComputeClient {
        GcpComputeClient::with_endpoints(
            "proj",
            "us-central1-a",
            "yoyo-batch",
            server.uri(),
            server.uri(),
        )
    }

    #[tokio::test]
    async fn fetches_access_token() {
        let server = MockServer::start().await;
        mock_token(&server).await;
        let c = client_for(&server);
        let tok = c.access_token().await.unwrap();
        assert_eq!(tok, "ya29.mock");
    }

    #[tokio::test]
    async fn start_returns_operation_name() {
        let server = MockServer::start().await;
        mock_token(&server).await;
        Mock::given(method("POST"))
            .and(path(
                "/projects/proj/zones/us-central1-a/instances/yoyo-batch/start",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "kind": "compute#operation",
                "name": "operation-123",
                "status": "PENDING"
            })))
            .mount(&server)
            .await;
        let c = client_for(&server);
        let op = c.start_instance().await.unwrap();
        assert_eq!(op, "operation-123");
    }

    #[tokio::test]
    async fn stop_returns_operation_name() {
        let server = MockServer::start().await;
        mock_token(&server).await;
        Mock::given(method("POST"))
            .and(path(
                "/projects/proj/zones/us-central1-a/instances/yoyo-batch/stop",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "operation-stop-1",
                "status": "PENDING"
            })))
            .mount(&server)
            .await;
        let c = client_for(&server);
        let op = c.stop_instance().await.unwrap();
        assert_eq!(op, "operation-stop-1");
    }

    #[tokio::test]
    async fn instance_status_maps_running() {
        let server = MockServer::start().await;
        mock_token(&server).await;
        Mock::given(method("GET"))
            .and(path(
                "/projects/proj/zones/us-central1-a/instances/yoyo-batch",
            ))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({ "status": "RUNNING" })),
            )
            .mount(&server)
            .await;
        let c = client_for(&server);
        assert_eq!(c.instance_status().await.unwrap(), InstanceStatus::Running);
    }

    #[tokio::test]
    async fn instance_status_maps_terminated() {
        let server = MockServer::start().await;
        mock_token(&server).await;
        Mock::given(method("GET"))
            .and(path(
                "/projects/proj/zones/us-central1-a/instances/yoyo-batch",
            ))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({ "status": "TERMINATED" })),
            )
            .mount(&server)
            .await;
        let c = client_for(&server);
        assert_eq!(
            c.instance_status().await.unwrap(),
            InstanceStatus::Terminated
        );
    }

    #[tokio::test]
    async fn wait_operation_succeeds_when_done() {
        let server = MockServer::start().await;
        mock_token(&server).await;
        Mock::given(method("GET"))
            .and(path("/projects/proj/zones/us-central1-a/operations/op-1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "op-1",
                "status": "DONE"
            })))
            .mount(&server)
            .await;
        let c = client_for(&server);
        c.wait_operation("op-1", Duration::from_millis(1), 3)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn wait_operation_surfaces_operation_error() {
        let server = MockServer::start().await;
        mock_token(&server).await;
        Mock::given(method("GET"))
            .and(path("/projects/proj/zones/us-central1-a/operations/op-err"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "op-err",
                "status": "DONE",
                "error": { "errors": [{ "code": "QUOTA_EXCEEDED" }] }
            })))
            .mount(&server)
            .await;
        let c = client_for(&server);
        let err = c
            .wait_operation("op-err", Duration::from_millis(1), 3)
            .await
            .unwrap_err();
        assert!(matches!(err, DoormanError::GcpApi { .. }));
    }

    #[tokio::test]
    async fn start_surfaces_http_error() {
        let server = MockServer::start().await;
        mock_token(&server).await;
        Mock::given(method("POST"))
            .and(path(
                "/projects/proj/zones/us-central1-a/instances/yoyo-batch/start",
            ))
            .respond_with(ResponseTemplate::new(403).set_body_string("quota"))
            .mount(&server)
            .await;
        let c = client_for(&server);
        let err = c.start_instance().await.unwrap_err();
        assert!(matches!(
            err,
            DoormanError::GcpApi {
                operation: "start",
                ..
            }
        ));
    }
}
