// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! GLiNER entity extraction through the Doorman's `/v1/jobs` family.
//!
//! Operator ruling 2026-11-19: `service-content` may call GLiNER only
//! through the Doorman, so every call is a registered engine, a per-job
//! audit entry and a credential-bound request. This module is the client
//! side of that path. It is off unless `SERVICE_CONTENT_GLINER_VIA_DOORMAN`
//! is truthy; when it is on there is no direct fallback to the GLiNER
//! endpoint, and any failure is reported as unavailable (the caller then
//! falls through to Tier A, as it does for a direct failure).
//!
//! One document is one job: stage the text, submit an `extract-spans` job
//! on the engine registered for the domain, poll until it is terminal,
//! read the validated entities, acknowledge the job so its buffer is freed.
//!
//! Settings (all read per call, so a rotated token file is picked up):
//! - `SERVICE_CONTENT_GLINER_VIA_DOORMAN`: `1`/`true`/`yes` turns the path on.
//! - `SLM_DOORMAN_ENDPOINT`: the Doorman (default `http://127.0.0.1:9080`).
//! - `SERVICE_CONTENT_JOBS_TOKEN_FILE`: file holding the bearer token. On
//!   unix it must not be readable by group or others, the same rule the
//!   Doorman applies to its credentials file.
//! - `SERVICE_CONTENT_GLINER_ENGINE_PREFIX`: engine id is this prefix plus
//!   the domain (default `gliner-`, so `gliner-projects`).
//! - `SERVICE_CONTENT_GLINER_ENGINE`: one engine id for every domain. When
//!   set (non-blank) the prefix is ignored, every job goes to this engine and
//!   names its domain in `params.domain_id`; the registry entry must list
//!   the domains in `domains`. One engine means one permit and one queue in
//!   front of the single-inference GLiNER service. Unset keeps the
//!   per-domain engines.
//! - `SERVICE_CONTENT_GLINER_JOB_DEADLINE_SECS`: whole-job wait (default 60).
//!
//! The registry entry for each engine must set `chunk_bytes` at or below
//! the GLiNER service's own character limit; the Doorman chunks, this
//! module sends the whole document.

use std::time::{Duration, Instant};

use reqwest::blocking::Client;
use serde_json::{json, Value};

const DEFAULT_PREFIX: &str = "gliner-";
const DEFAULT_DEADLINE_SECS: u64 = 60;
const POLL_FIRST: Duration = Duration::from_millis(200);
const POLL_MAX: Duration = Duration::from_secs(2);

/// Resolved from the environment for one call.
pub(crate) enum Mode {
    /// Flag off: the direct GLiNER path is used, unchanged.
    Off,
    /// Flag on and configured.
    On(DoormanGliner),
    /// Flag on but unusable (for example no readable token). The caller
    /// reports unavailable; it must not fall back to the direct path.
    Misconfigured(String),
}

pub(crate) struct DoormanGliner {
    base: String,
    token: String,
    engine_prefix: String,
    /// One engine for every domain (the domain travels as a job param).
    single_engine: Option<String>,
    deadline: Duration,
}

fn truthy(v: &str) -> bool {
    matches!(v, "1" | "true" | "TRUE" | "yes")
}

impl Mode {
    pub(crate) fn from_env() -> Mode {
        let on = std::env::var("SERVICE_CONTENT_GLINER_VIA_DOORMAN")
            .map(|v| truthy(&v))
            .unwrap_or(false);
        if !on {
            return Mode::Off;
        }
        let Ok(path) = std::env::var("SERVICE_CONTENT_JOBS_TOKEN_FILE") else {
            return Mode::Misconfigured("SERVICE_CONTENT_JOBS_TOKEN_FILE is not set".into());
        };
        let token = match read_token(&path) {
            Ok(t) => t,
            Err(e) => return Mode::Misconfigured(e),
        };
        let base = std::env::var("SLM_DOORMAN_ENDPOINT")
            .unwrap_or_else(|_| "http://127.0.0.1:9080".to_string());
        let engine_prefix = std::env::var("SERVICE_CONTENT_GLINER_ENGINE_PREFIX")
            .unwrap_or_else(|_| DEFAULT_PREFIX.to_string());
        let deadline = std::env::var("SERVICE_CONTENT_GLINER_JOB_DEADLINE_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .filter(|s| *s > 0)
            .unwrap_or(DEFAULT_DEADLINE_SECS);
        let single_engine = std::env::var("SERVICE_CONTENT_GLINER_ENGINE")
            .ok()
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());
        let mut g = DoormanGliner::new(&base, token, engine_prefix, Duration::from_secs(deadline));
        if let Some(engine) = single_engine {
            g = g.with_single_engine(engine);
        }
        Mode::On(g)
    }
}

fn read_token(path: &str) -> Result<String, String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let meta = std::fs::metadata(path).map_err(|e| format!("token file unreadable: {e}"))?;
        if meta.permissions().mode() & 0o077 != 0 {
            return Err("token file is readable by group or others".into());
        }
    }
    let raw = std::fs::read_to_string(path).map_err(|e| format!("token file unreadable: {e}"))?;
    let token = raw.trim().to_string();
    if token.is_empty() {
        return Err("token file is empty".into());
    }
    Ok(token)
}

impl DoormanGliner {
    pub(crate) fn new(
        base: &str,
        token: String,
        engine_prefix: String,
        deadline: Duration,
    ) -> Self {
        Self {
            base: base.trim_end_matches('/').to_string(),
            token,
            engine_prefix,
            single_engine: None,
            deadline,
        }
    }

    /// Send every domain to one engine, naming the domain in the job params.
    pub(crate) fn with_single_engine(mut self, engine_id: String) -> Self {
        self.single_engine = Some(engine_id);
        self
    }

    /// Runs one document through the engine registered for `domain`. `Ok`
    /// (possibly empty) means the job finished; `Err` carries a reason for
    /// the log and means unavailable.
    pub(crate) fn run_domain(
        &self,
        client: &Client,
        text: &str,
        domain: &str,
    ) -> Result<Vec<Value>, String> {
        let bearer = format!("Bearer {}", self.token);
        let started = Instant::now();

        let staged = client
            .post(format!("{}/v1/jobs/inputs", self.base))
            .header("Authorization", &bearer)
            .header("Content-Type", "text/plain")
            .body(text.to_string())
            .send()
            .map_err(|e| format!("stage request failed: {e}"))?;
        if !staged.status().is_success() {
            return Err(format!("stage returned {}", staged.status()));
        }
        let staged: Value = staged
            .json()
            .map_err(|e| format!("stage response shape: {e}"))?;
        let (Some(staged_ref), Some(sha256)) =
            (staged["staged_ref"].as_str(), staged["sha256"].as_str())
        else {
            return Err("stage response lacks staged_ref or sha256".into());
        };

        let mut body = json!({
            "engine_id": format!("{}{}", self.engine_prefix, domain),
            "kind": "extract-spans",
            "input": {
                "sha256": sha256,
                "media_type": "text/plain",
                "staged_ref": staged_ref,
            },
        });
        if let Some(engine) = &self.single_engine {
            body["engine_id"] = json!(engine);
            body["params"] = json!({ "domain_id": domain });
        }
        let submitted = client
            .post(format!("{}/v1/jobs", self.base))
            .header("Authorization", &bearer)
            .json(&body)
            .send()
            .map_err(|e| format!("submit request failed: {e}"))?;
        if !submitted.status().is_success() {
            return Err(format!("submit returned {}", submitted.status()));
        }
        let submitted: Value = submitted
            .json()
            .map_err(|e| format!("submit response shape: {e}"))?;
        let Some(job_id) = submitted["job_id"].as_str().map(str::to_string) else {
            return Err("submit response lacks job_id".into());
        };

        let outcome = self.wait(client, &bearer, &job_id, started);
        let result = match outcome.as_deref() {
            Ok("done") => self.read_entities(client, &bearer, &job_id),
            Ok("empty") => Ok(Vec::new()),
            Ok(other) => Err(format!("job ended {other}")),
            Err(e) => Err(e.clone()),
        };
        // Free the staged input and result once the job is terminal. A job
        // that never finished is left to the Doorman's own buffer limits.
        if outcome.is_ok() {
            let _ = client
                .post(format!("{}/v1/jobs/{}/ack", self.base, job_id))
                .header("Authorization", &bearer)
                .send();
        }
        result
    }

    fn wait(
        &self,
        client: &Client,
        bearer: &str,
        job_id: &str,
        started: Instant,
    ) -> Result<String, String> {
        let mut pause = POLL_FIRST;
        loop {
            let resp = client
                .get(format!("{}/v1/jobs/{}", self.base, job_id))
                .header("Authorization", bearer)
                .send()
                .map_err(|e| format!("status request failed: {e}"))?;
            if !resp.status().is_success() {
                return Err(format!("status returned {}", resp.status()));
            }
            let v: Value = resp
                .json()
                .map_err(|e| format!("status response shape: {e}"))?;
            match v["status"].as_str() {
                Some("queued") | Some("running") => {}
                Some(done) => return Ok(done.to_string()),
                None => return Err("status response lacks status".into()),
            }
            if started.elapsed() >= self.deadline {
                return Err(format!("job {job_id} not finished within the deadline"));
            }
            std::thread::sleep(pause);
            pause = (pause * 3 / 2).min(POLL_MAX);
        }
    }

    fn read_entities(
        &self,
        client: &Client,
        bearer: &str,
        job_id: &str,
    ) -> Result<Vec<Value>, String> {
        let resp = client
            .get(format!("{}/v1/jobs/{}/result", self.base, job_id))
            .header("Authorization", bearer)
            .send()
            .map_err(|e| format!("result request failed: {e}"))?;
        if !resp.status().is_success() {
            return Err(format!("result returned {}", resp.status()));
        }
        let v: Value = resp
            .json()
            .map_err(|e| format!("result response shape: {e}"))?;
        let Some(entities) = v["entities"].as_array() else {
            return Err("result lacks an entities array".into());
        };
        if let Some(rejected) = v["rejected"].as_u64().filter(|n| *n > 0) {
            eprintln!(
                "  -> [TIER-0] Doorman rejected {rejected} proposed entit{} not found verbatim in the input",
                if rejected == 1 { "y" } else { "ies" }
            );
        }
        let mut seen = std::collections::HashSet::new();
        let mut out = Vec::new();
        for e in entities {
            let (Some(name), Some(class)) = (e["value"].as_str(), e["class"].as_str()) else {
                continue;
            };
            if seen.insert(format!("{}__{}", name.to_lowercase(), class)) {
                out.push(json!({"entity_name": name, "classification": class}));
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{body_string_contains, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const JOB: &str = "00000000-0000-4000-8000-000000000001";

    /// Tests that set the `SERVICE_CONTENT_*` variables read by
    /// `Mode::from_env` must not interleave.
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn client() -> Client {
        Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap()
    }

    async fn stage_and_submit(server: &MockServer, engine: &str) {
        Mock::given(method("POST"))
            .and(path("/v1/jobs/inputs"))
            .and(header("authorization", "Bearer tok"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "staged_ref": "00000000-0000-4000-8000-0000000000aa",
                "sha256": "a".repeat(64),
                "size": 10
            })))
            .mount(server)
            .await;
        Mock::given(method("POST"))
            .and(path("/v1/jobs"))
            .and(body_string_contains(engine))
            .and(body_string_contains("extract-spans"))
            .respond_with(ResponseTemplate::new(202).set_body_json(json!({"job_id": JOB})))
            .mount(server)
            .await;
    }

    fn status(s: &str) -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(json!({"job_id": JOB, "status": s}))
    }

    fn gl(server: &MockServer) -> DoormanGliner {
        DoormanGliner::new(
            &server.uri(),
            "tok".into(),
            "gliner-".into(),
            Duration::from_secs(5),
        )
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn done_job_returns_entities_in_the_direct_shape_and_acks() {
        let server = MockServer::start().await;
        stage_and_submit(&server, "gliner-corporate").await;
        Mock::given(method("GET"))
            .and(path(format!("/v1/jobs/{JOB}")))
            .respond_with(status("done"))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/v1/jobs/{JOB}/result")))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "job": {},
                "text": "",
                "entities": [
                    {"class": "Person", "value": "Alice", "witness": "Alice", "span": {"start": 0, "end": 5}},
                    {"class": "Person", "value": "alice", "witness": "alice", "span": {"start": 9, "end": 14}},
                    {"class": "Location", "value": "Paris", "witness": "Paris", "span": {"start": 20, "end": 25}}
                ],
                "rejected": 1
            })))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path(format!("/v1/jobs/{JOB}/ack")))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let g = gl(&server);
        let out = tokio::task::spawn_blocking(move || {
            g.run_domain(&client(), "Alice flew to Paris.", "corporate")
        })
        .await
        .unwrap()
        .unwrap();
        // The second Alice differs only by case: deduplicated like the direct path.
        assert_eq!(out.len(), 2);
        assert_eq!(out[0]["entity_name"], "Alice");
        assert_eq!(out[0]["classification"], "Person");
        assert_eq!(out[1]["entity_name"], "Paris");
        server.verify().await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn empty_job_is_an_empty_list_not_an_error() {
        let server = MockServer::start().await;
        stage_and_submit(&server, "gliner-projects").await;
        Mock::given(method("GET"))
            .and(path(format!("/v1/jobs/{JOB}")))
            .respond_with(status("empty"))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path(format!("/v1/jobs/{JOB}/ack")))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;
        let g = gl(&server);
        let out = tokio::task::spawn_blocking(move || g.run_domain(&client(), "x", "projects"))
            .await
            .unwrap();
        assert_eq!(out.unwrap().len(), 0);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn queued_then_running_then_done_is_polled() {
        let server = MockServer::start().await;
        stage_and_submit(&server, "gliner-projects").await;
        Mock::given(method("GET"))
            .and(path(format!("/v1/jobs/{JOB}")))
            .respond_with(status("queued"))
            .up_to_n_times(1)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/v1/jobs/{JOB}")))
            .respond_with(status("running"))
            .up_to_n_times(1)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/v1/jobs/{JOB}")))
            .respond_with(status("done"))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/v1/jobs/{JOB}/result")))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                json!({"job": {}, "text": "", "entities": [], "rejected": 0}),
            ))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path(format!("/v1/jobs/{JOB}/ack")))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(&server)
            .await;
        let g = gl(&server);
        let out = tokio::task::spawn_blocking(move || g.run_domain(&client(), "x", "projects"))
            .await
            .unwrap();
        assert!(out.unwrap().is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn deferred_and_failed_jobs_are_unavailable() {
        for terminal in ["deferred", "failed", "unreadable", "not-applicable"] {
            let server = MockServer::start().await;
            stage_and_submit(&server, "gliner-projects").await;
            Mock::given(method("GET"))
                .and(path(format!("/v1/jobs/{JOB}")))
                .respond_with(status(terminal))
                .mount(&server)
                .await;
            Mock::given(method("POST"))
                .and(path(format!("/v1/jobs/{JOB}/ack")))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
                .mount(&server)
                .await;
            let g = gl(&server);
            let out =
                tokio::task::spawn_blocking(move || g.run_domain(&client(), "x", "projects"))
                    .await
                    .unwrap();
            let err = out.expect_err(terminal);
            assert!(err.contains(terminal), "{err}");
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn a_job_that_never_finishes_hits_the_deadline() {
        let server = MockServer::start().await;
        stage_and_submit(&server, "gliner-projects").await;
        Mock::given(method("GET"))
            .and(path(format!("/v1/jobs/{JOB}")))
            .respond_with(status("running"))
            .mount(&server)
            .await;
        let g = DoormanGliner::new(
            &server.uri(),
            "tok".into(),
            "gliner-".into(),
            Duration::from_millis(600),
        );
        let out = tokio::task::spawn_blocking(move || g.run_domain(&client(), "x", "projects"))
            .await
            .unwrap();
        assert!(out.unwrap_err().contains("deadline"));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn refused_stage_or_submit_is_unavailable() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/jobs/inputs"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;
        let g = gl(&server);
        let out = tokio::task::spawn_blocking(move || g.run_domain(&client(), "x", "projects"))
            .await
            .unwrap();
        assert!(out.unwrap_err().contains("401"));

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/jobs/inputs"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "staged_ref": "r", "sha256": "a".repeat(64), "size": 1
            })))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/v1/jobs"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;
        let g = gl(&server);
        let out = tokio::task::spawn_blocking(move || g.run_domain(&client(), "x", "projects"))
            .await
            .unwrap();
        assert!(out.unwrap_err().contains("403"));
    }

    #[test]
    fn an_unreachable_doorman_is_unavailable() {
        let g = DoormanGliner::new(
            "http://127.0.0.1:1",
            "tok".into(),
            "gliner-".into(),
            Duration::from_secs(1),
        );
        assert!(g.run_domain(&client(), "x", "projects").is_err());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn empty_default_domain_retries_on_the_corporate_engine() {
        const JOB_P: &str = "00000000-0000-4000-8000-0000000000b1";
        const JOB_C: &str = "00000000-0000-4000-8000-0000000000b2";
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/jobs/inputs"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "staged_ref": "r", "sha256": "a".repeat(64), "size": 1
            })))
            .mount(&server)
            .await;
        for (engine, id, st) in [
            ("gliner-projects", JOB_P, "empty"),
            ("gliner-corporate", JOB_C, "done"),
        ] {
            Mock::given(method("POST"))
                .and(path("/v1/jobs"))
                .and(body_string_contains(engine))
                .respond_with(ResponseTemplate::new(202).set_body_json(json!({"job_id": id})))
                .mount(&server)
                .await;
            Mock::given(method("GET"))
                .and(path(format!("/v1/jobs/{id}")))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": st})))
                .mount(&server)
                .await;
            Mock::given(method("POST"))
                .and(path(format!("/v1/jobs/{id}/ack")))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
                .mount(&server)
                .await;
        }
        Mock::given(method("GET"))
            .and(path(format!("/v1/jobs/{JOB_C}/result")))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "job": {}, "text": "", "rejected": 0,
                "entities": [{"class": "Person", "value": "Alice Example",
                              "witness": "Alice Example", "span": {"start": 0, "end": 13}}]
            })))
            .mount(&server)
            .await;

        let g = gl(&server);
        let outcome = tokio::task::spawn_blocking(move || {
            crate::call_tier_0_gliner_via(&g, "Alice Example met with Acme Corp.", None)
        })
        .await
        .unwrap();
        match outcome {
            crate::GlinerOutcome::Found(e) => {
                assert_eq!(e.len(), 1);
                assert_eq!(e[0]["entity_name"], "Alice Example");
            }
            _ => panic!("expected Found after the corporate retry"),
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn a_failed_job_through_the_doorman_is_unavailable_not_a_direct_call() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/jobs/inputs"))
            .respond_with(ResponseTemplate::new(503))
            .mount(&server)
            .await;
        let g = gl(&server);
        let outcome = tokio::task::spawn_blocking(move || {
            crate::call_tier_0_gliner_via(&g, "text", Some("projects"))
        })
        .await
        .unwrap();
        assert!(matches!(outcome, crate::GlinerOutcome::Unavailable));
    }

    #[test]
    fn mode_is_off_by_default_and_misconfigured_without_a_token_file() {
        let _env = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // These variables are read only by `Mode::from_env`; no other test
        // depends on them.
        std::env::remove_var("SERVICE_CONTENT_GLINER_VIA_DOORMAN");
        std::env::remove_var("SERVICE_CONTENT_JOBS_TOKEN_FILE");
        assert!(matches!(Mode::from_env(), Mode::Off));
        std::env::set_var("SERVICE_CONTENT_GLINER_VIA_DOORMAN", "1");
        assert!(matches!(Mode::from_env(), Mode::Misconfigured(_)));
        std::env::set_var("SERVICE_CONTENT_JOBS_TOKEN_FILE", "/nonexistent/token");
        assert!(matches!(Mode::from_env(), Mode::Misconfigured(_)));
        std::env::remove_var("SERVICE_CONTENT_GLINER_VIA_DOORMAN");
        std::env::remove_var("SERVICE_CONTENT_JOBS_TOKEN_FILE");
    }

    #[cfg(unix)]
    #[test]
    fn token_file_must_be_private_and_non_empty() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("tok");
        std::fs::write(&p, "secret\n").unwrap();
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(read_token(p.to_str().unwrap())
            .unwrap_err()
            .contains("group or others"));
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o600)).unwrap();
        assert_eq!(read_token(p.to_str().unwrap()).unwrap(), "secret");
        std::fs::write(&p, "  \n").unwrap();
        assert!(read_token(p.to_str().unwrap())
            .unwrap_err()
            .contains("empty"));
        assert!(read_token("/nonexistent/token").is_err());
    }
    async fn submitted_body(server: &MockServer) -> Value {
        let reqs = server.received_requests().await.unwrap();
        let r = reqs
            .iter()
            .find(|r| r.url.path() == "/v1/jobs")
            .expect("a submit request");
        serde_json::from_slice(&r.body).unwrap()
    }

    async fn run_empty(server: &MockServer, g: DoormanGliner, domain: &'static str) {
        Mock::given(method("GET"))
            .and(path(format!("/v1/jobs/{JOB}")))
            .respond_with(status("empty"))
            .mount(server)
            .await;
        Mock::given(method("POST"))
            .and(path(format!("/v1/jobs/{JOB}/ack")))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .mount(server)
            .await;
        let out = tokio::task::spawn_blocking(move || g.run_domain(&client(), "x", domain))
            .await
            .unwrap();
        assert_eq!(out.unwrap().len(), 0);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn single_engine_mode_submits_one_engine_with_the_domain_as_a_param() {
        let server = MockServer::start().await;
        stage_and_submit(&server, "\"gliner\"").await;
        let g = gl(&server).with_single_engine("gliner".into());
        run_empty(&server, g, "corporate").await;
        let body = submitted_body(&server).await;
        assert_eq!(body["engine_id"], "gliner");
        assert_eq!(body["params"]["domain_id"], "corporate");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn legacy_mode_keeps_prefix_plus_domain_and_sends_no_params() {
        let server = MockServer::start().await;
        stage_and_submit(&server, "gliner-corporate").await;
        let g = gl(&server);
        run_empty(&server, g, "corporate").await;
        let body = submitted_body(&server).await;
        assert_eq!(body["engine_id"], "gliner-corporate");
        assert!(body.get("params").is_none(), "legacy body must not change");
    }

    #[test]
    fn single_engine_env_selects_the_mode_and_blank_is_ignored() {
        let _env = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("tok");
        std::fs::write(&p, "secret\n").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        std::env::set_var("SERVICE_CONTENT_GLINER_VIA_DOORMAN", "1");
        std::env::set_var("SERVICE_CONTENT_JOBS_TOKEN_FILE", &p);
        std::env::set_var("SERVICE_CONTENT_GLINER_ENGINE", "gliner");
        let Mode::On(g) = Mode::from_env() else {
            panic!("expected On")
        };
        assert_eq!(g.single_engine.as_deref(), Some("gliner"));
        std::env::set_var("SERVICE_CONTENT_GLINER_ENGINE", "  ");
        let Mode::On(g) = Mode::from_env() else {
            panic!("expected On")
        };
        assert_eq!(g.single_engine, None);
        std::env::remove_var("SERVICE_CONTENT_GLINER_ENGINE");
        std::env::remove_var("SERVICE_CONTENT_GLINER_VIA_DOORMAN");
        std::env::remove_var("SERVICE_CONTENT_JOBS_TOKEN_FILE");
    }
}
