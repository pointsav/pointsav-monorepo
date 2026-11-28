// SPDX-License-Identifier: Apache-2.0

//! Engine adapters. An engine runs one input and returns text or a typed
//! error; mapping errors to job outcomes is the service's job. No adapter
//! ever calls another engine.

use std::time::Duration;

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use super::types::JobParams;

/// What an engine returns on success.
#[derive(Clone, Debug)]
pub struct EngineOutput {
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineError {
    /// Engine unreachable, saturated, or refused: becomes `deferred`.
    Unavailable(String),
    /// Input cannot be read by this engine: becomes `unreadable`.
    Unreadable(String),
    /// Anything else: becomes `failed`.
    Failed(String),
}

pub struct EngineRequest<'a> {
    pub input: &'a [u8],
    pub media_type: &'a str,
    pub params: &'a JobParams,
    pub system_prompt: &'a str,
    pub schema: Option<serde_json::Value>,
}

#[async_trait]
pub trait JobEngine: Send + Sync {
    async fn run(&self, req: EngineRequest<'_>) -> Result<EngineOutput, EngineError>;
}

/// Non-blocking, idle-only admission to the shared inference backend.
/// Returns a guard released on drop, or `None` when the backend is busy.
pub trait AdmissionGate: Send + Sync {
    fn try_admit(&self) -> Option<Box<dyn Send>>;
}

/// Gate that always admits (tests, or engines with their own capacity).
pub struct OpenGate;
impl AdmissionGate for OpenGate {
    fn try_admit(&self) -> Option<Box<dyn Send>> {
        Some(Box::new(()))
    }
}

/// Local llama.cpp-style chat endpoint (Tier A / OLMo), called directly so
/// seed and cache settings are explicit. Jobs are admitted only while the
/// backend is idle, so interactive traffic always wins.
pub struct LlmLocalEngine {
    pub endpoint: String,
    pub model: String,
    pub gate: std::sync::Arc<dyn AdmissionGate>,
    pub retry_interval: Duration,
    http: reqwest::Client,
}

impl LlmLocalEngine {
    pub fn new(endpoint: String, model: String, gate: std::sync::Arc<dyn AdmissionGate>) -> Self {
        let http = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .build()
            .expect("reqwest client");
        Self {
            endpoint: endpoint.trim_end_matches('/').to_string(),
            model,
            gate,
            retry_interval: Duration::from_secs(2),
            http,
        }
    }
}

#[derive(Deserialize)]
struct ChatResp {
    choices: Vec<ChatChoice>,
}
#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMsg,
}
#[derive(Deserialize)]
struct ChatMsg {
    content: Option<String>,
}

#[async_trait]
impl JobEngine for LlmLocalEngine {
    async fn run(&self, req: EngineRequest<'_>) -> Result<EngineOutput, EngineError> {
        let text = std::str::from_utf8(req.input)
            .map_err(|_| EngineError::Unreadable("input is not UTF-8 text".into()))?;
        // Wait for an idle backend; the service-level job deadline bounds
        // this loop (the future is dropped on timeout).
        let _permit = loop {
            if let Some(p) = self.gate.try_admit() {
                break p;
            }
            tokio::time::sleep(self.retry_interval).await;
        };
        let mut body = json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": req.system_prompt},
                {"role": "user", "content": text},
            ],
            "temperature": req.params.temperature.unwrap_or(0.0),
            "stream": false,
            "cache_prompt": false,
        });
        if let Some(seed) = req.params.seed {
            body["seed"] = json!(seed);
        }
        if let Some(mt) = req.params.max_tokens {
            body["max_tokens"] = json!(mt);
        }
        if let Some(schema) = req.schema {
            body["json_schema"] = schema;
        }
        let url = format!("{}/v1/chat/completions", self.endpoint);
        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| EngineError::Unavailable(format!("engine request failed: {e}")))?;
        let status = resp.status();
        if status.is_server_error() || status.as_u16() == 429 {
            return Err(EngineError::Unavailable(format!(
                "engine returned {status}"
            )));
        }
        if !status.is_success() {
            return Err(EngineError::Failed(format!("engine returned {status}")));
        }
        let parsed: ChatResp = resp
            .json()
            .await
            .map_err(|e| EngineError::Failed(format!("engine response shape: {e}")))?;
        let content = parsed
            .choices
            .into_iter()
            .next()
            .and_then(|c| c.message.content)
            .ok_or_else(|| EngineError::Failed("engine response has no content".into()))?;
        Ok(EngineOutput { text: content })
    }
}

/// OCR reader behind an OpenAI-style vision chat endpoint. The page image is
/// base64-encoded from the caller's exact bytes: there is no decode, resize
/// or re-encode anywhere in this path. An unreachable or erroring engine is
/// `Unavailable` (job `deferred`); no other engine is ever tried.
/// Default `max_tokens` for OCR pages when the caller sets none.
pub const DEFAULT_OCR_MAX_TOKENS: u32 = 3500;

pub struct OcrOpenaiVisionEngine {
    pub endpoint: String,
    pub model: String,
    http: reqwest::Client,
}

impl OcrOpenaiVisionEngine {
    pub fn new(endpoint: String, model: String, request_timeout: Duration) -> Self {
        let http = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(request_timeout)
            .build()
            .expect("reqwest client");
        Self {
            endpoint: endpoint.trim_end_matches('/').to_string(),
            model,
            http,
        }
    }
}

/// True when `bytes` start with the magic number for `media_type`.
fn image_matches(media_type: &str, bytes: &[u8]) -> bool {
    match media_type {
        "image/png" => bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]),
        "image/jpeg" => bytes.starts_with(&[0xff, 0xd8, 0xff]),
        "image/webp" => bytes.len() > 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP",
        _ => false,
    }
}

#[async_trait]
impl JobEngine for OcrOpenaiVisionEngine {
    async fn run(&self, req: EngineRequest<'_>) -> Result<EngineOutput, EngineError> {
        use base64::Engine as _;
        if !image_matches(req.media_type, req.input) {
            return Err(EngineError::Unreadable(
                "input is not an image of the declared media type".into(),
            ));
        }
        let data_url = format!(
            "data:{};base64,{}",
            req.media_type,
            base64::engine::general_purpose::STANDARD.encode(req.input)
        );
        let mut body = json!({
            "model": self.model,
            "messages": [{
                "role": "user",
                "content": [{"type": "image_url", "image_url": {"url": data_url}}],
            }],
            "temperature": req.params.temperature.unwrap_or(0.0),
            "stream": false,
            "cache_prompt": false,
        });
        if let Some(seed) = req.params.seed {
            body["seed"] = json!(seed);
        }
        // Statement pages truncate at 1500 tokens and need 3500 (measured on
        // real statement pages), so OCR defaults high rather than leaving it to the server.
        body["max_tokens"] = json!(req.params.max_tokens.unwrap_or(DEFAULT_OCR_MAX_TOKENS));
        let url = format!("{}/v1/chat/completions", self.endpoint);
        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| EngineError::Unavailable(format!("ocr engine request failed: {e}")))?;
        let status = resp.status();
        if status.is_server_error() || status.as_u16() == 429 {
            return Err(EngineError::Unavailable(format!(
                "ocr engine returned {status}"
            )));
        }
        if !status.is_success() {
            return Err(EngineError::Failed(format!("ocr engine returned {status}")));
        }
        let parsed: ChatResp = resp
            .json()
            .await
            .map_err(|e| EngineError::Failed(format!("ocr engine response shape: {e}")))?;
        let content = parsed
            .choices
            .into_iter()
            .next()
            .and_then(|c| c.message.content)
            .ok_or_else(|| EngineError::Failed("ocr engine response has no content".into()))?;
        Ok(EngineOutput { text: content })
    }
}

/// GLiNER-class span extractor. It returns entity names and classes but no
/// offsets; the job layer derives spans by exact match (the entity name is
/// the witness), so an entity the model returns that is not literally in the
/// input is rejected like any other invented one. Output is re-expressed as
/// `{entities:[{class,value,witness}]}` for the shared validator.
pub struct SpanGlinerEngine {
    pub endpoint: String,
    pub domain_id: String,
    http: reqwest::Client,
}

impl SpanGlinerEngine {
    pub fn new(endpoint: String, domain_id: Option<String>, request_timeout: Duration) -> Self {
        let http = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(request_timeout)
            .build()
            .expect("reqwest client");
        Self {
            endpoint: endpoint.trim_end_matches('/').to_string(),
            domain_id: domain_id.unwrap_or_else(|| "projects".to_string()),
            http,
        }
    }
}

#[derive(Deserialize)]
struct GlinerResp {
    entities: Vec<GlinerEntity>,
}
#[derive(Deserialize)]
struct GlinerEntity {
    entity_name: String,
    classification: String,
}

#[async_trait]
impl JobEngine for SpanGlinerEngine {
    async fn run(&self, req: EngineRequest<'_>) -> Result<EngineOutput, EngineError> {
        let text = std::str::from_utf8(req.input)
            .map_err(|_| EngineError::Unreadable("input is not UTF-8 text".into()))?;
        // The job may name a domain the registry entry serves; submit has
        // already checked it. Without one the engine's own domain applies.
        let domain = req.params.domain_id.as_deref().unwrap_or(&self.domain_id);
        let body = json!({"text": text, "domain_id": domain, "entity_hints": null});
        let resp = self
            .http
            .post(format!("{}/v1/extract", self.endpoint))
            .json(&body)
            .send()
            .await
            .map_err(|e| EngineError::Unavailable(format!("span engine request failed: {e}")))?;
        let status = resp.status();
        if status.is_server_error() || status.as_u16() == 429 {
            return Err(EngineError::Unavailable(format!(
                "span engine returned {status}"
            )));
        }
        if !status.is_success() {
            return Err(EngineError::Failed(format!(
                "span engine returned {status}"
            )));
        }
        let parsed: GlinerResp = resp
            .json()
            .await
            .map_err(|e| EngineError::Failed(format!("span engine response shape: {e}")))?;
        let entities: Vec<_> = parsed
            .entities
            .into_iter()
            .map(|e| json!({"class": e.classification, "value": e.entity_name, "witness": e.entity_name}))
            .collect();
        Ok(EngineOutput {
            text: json!({"entities": entities}).to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    struct BusyThenIdle(AtomicUsize);
    impl AdmissionGate for BusyThenIdle {
        fn try_admit(&self) -> Option<Box<dyn Send>> {
            if self.0.fetch_add(1, Ordering::SeqCst) < 3 {
                None
            } else {
                Some(Box::new(()))
            }
        }
    }

    #[tokio::test]
    async fn waits_for_idle_backend_and_sends_deterministic_params() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "choices": [{"message": {"role": "assistant", "content": "OUT"}}]
            })))
            .mount(&server)
            .await;
        let gate = Arc::new(BusyThenIdle(AtomicUsize::new(0)));
        let mut engine = LlmLocalEngine::new(server.uri(), "m".into(), gate.clone());
        engine.retry_interval = Duration::from_millis(1);
        let params = JobParams {
            seed: Some(42),
            ..Default::default()
        };
        let out = engine
            .run(EngineRequest {
                input: b"text",
                media_type: "text/plain",
                params: &params,
                system_prompt: "sys",
                schema: Some(serde_json::json!({"type": "object"})),
            })
            .await
            .unwrap();
        assert_eq!(out.text, "OUT");
        assert_eq!(
            gate.0.load(Ordering::SeqCst),
            4,
            "retried while busy, admitted when idle"
        );
        let reqs = server.received_requests().await.unwrap();
        assert_eq!(reqs.len(), 1, "no engine call before admission");
        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["seed"], 42);
        assert_eq!(body["temperature"], 0.0);
        assert_eq!(body["cache_prompt"], false);
        assert_eq!(body["json_schema"]["type"], "object");
    }

    #[tokio::test]
    async fn non_utf8_is_unreadable_and_refusal_is_unavailable() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(503))
            .mount(&server)
            .await;
        let engine = LlmLocalEngine::new(server.uri(), "m".into(), Arc::new(OpenGate));
        let params = JobParams::default();
        let req = |input: &'static [u8]| EngineRequest {
            input,
            media_type: "text/plain",
            params: &params,
            system_prompt: "s",
            schema: None,
        };
        assert!(matches!(
            engine.run(req(&[0xff, 0xfe])).await,
            Err(EngineError::Unreadable(_))
        ));
        assert!(matches!(
            engine.run(req(b"ok")).await,
            Err(EngineError::Unavailable(_))
        ));
    }

    const PNG: &[u8] = &[
        0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, 1, 2, 3, 4, 5,
    ];

    #[tokio::test]
    async fn ocr_forwards_exact_image_bytes_and_reads_text() {
        use base64::Engine as _;
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "choices": [{"message": {"role": "assistant", "content": "PAGE TEXT"}}]
            })))
            .mount(&server)
            .await;
        let engine =
            OcrOpenaiVisionEngine::new(server.uri(), "lighton".into(), Duration::from_secs(5));
        let params = JobParams {
            seed: Some(1),
            ..Default::default()
        };
        let out = engine
            .run(EngineRequest {
                input: PNG,
                media_type: "image/png",
                params: &params,
                system_prompt: "",
                schema: None,
            })
            .await
            .unwrap();
        assert_eq!(out.text, "PAGE TEXT");
        let reqs = server.received_requests().await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        let url = body["messages"][0]["content"][0]["image_url"]["url"]
            .as_str()
            .unwrap();
        let b64 = url
            .strip_prefix("data:image/png;base64,")
            .expect("data url");
        assert_eq!(
            base64::engine::general_purpose::STANDARD
                .decode(b64)
                .unwrap(),
            PNG,
            "bytes unchanged (no resize)"
        );
        assert_eq!(body["seed"], 1);
        assert_eq!(
            body["max_tokens"], DEFAULT_OCR_MAX_TOKENS,
            "pages default to 3500 tokens"
        );

        let params = JobParams {
            max_tokens: Some(100),
            ..Default::default()
        };
        engine
            .run(EngineRequest {
                input: PNG,
                media_type: "image/png",
                params: &params,
                system_prompt: "",
                schema: None,
            })
            .await
            .unwrap();
        let reqs = server.received_requests().await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&reqs[1].body).unwrap();
        assert_eq!(body["max_tokens"], 100, "caller override wins");
    }

    #[tokio::test]
    async fn ocr_rejects_non_images_and_defers_when_down() {
        let params = JobParams::default();
        let req = |input: &'static [u8], mt: &'static str| EngineRequest {
            input,
            media_type: mt,
            params: &params,
            system_prompt: "",
            schema: None,
        };
        // Nothing listens on this port: connection refused => Unavailable.
        let down = OcrOpenaiVisionEngine::new(
            "http://127.0.0.1:1".into(),
            "m".into(),
            Duration::from_secs(2),
        );
        assert!(matches!(
            down.run(req(PNG, "image/png")).await,
            Err(EngineError::Unavailable(_))
        ));
        assert!(matches!(
            down.run(req(b"plain text", "image/png")).await,
            Err(EngineError::Unreadable(_))
        ));
        assert!(matches!(
            down.run(req(PNG, "image/jpeg")).await,
            Err(EngineError::Unreadable(_))
        ));
    }

    #[tokio::test]
    async fn span_engine_maps_gliner_output_to_witness_json() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/extract"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "entities": [{"entity_name": "Alice", "classification": "Person"}]
            })))
            .mount(&server)
            .await;
        let engine = SpanGlinerEngine::new(
            server.uri(),
            Some("corporate".into()),
            Duration::from_secs(5),
        );
        let params = JobParams::default();
        let out = engine
            .run(EngineRequest {
                input: b"Alice here",
                media_type: "text/plain",
                params: &params,
                system_prompt: "",
                schema: None,
            })
            .await
            .unwrap();
        let v: serde_json::Value = serde_json::from_str(&out.text).unwrap();
        assert_eq!(v["entities"][0]["witness"], "Alice");
        assert_eq!(v["entities"][0]["class"], "Person");
        let reqs = server.received_requests().await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["domain_id"], "corporate");
        let down = SpanGlinerEngine::new("http://127.0.0.1:1".into(), None, Duration::from_secs(2));
        assert!(matches!(
            down.run(EngineRequest {
                input: b"x",
                media_type: "text/plain",
                params: &params,
                system_prompt: "",
                schema: None
            })
            .await,
            Err(EngineError::Unavailable(_))
        ));
    }
    #[tokio::test]
    async fn span_engine_sends_the_job_domain_when_given_else_its_own() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/extract"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"entities": []})))
            .mount(&server)
            .await;
        let engine = SpanGlinerEngine::new(
            server.uri(),
            Some("projects".into()),
            Duration::from_secs(5),
        );
        for (job_domain, expect) in [(Some("documentation"), "documentation"), (None, "projects")] {
            let params = JobParams {
                domain_id: job_domain.map(str::to_string),
                ..Default::default()
            };
            engine
                .run(EngineRequest {
                    input: b"x",
                    media_type: "text/plain",
                    params: &params,
                    system_prompt: "",
                    schema: None,
                })
                .await
                .unwrap();
            let reqs = server.received_requests().await.unwrap();
            let body: serde_json::Value =
                serde_json::from_slice(&reqs.last().unwrap().body).unwrap();
            assert_eq!(body["domain_id"], expect);
        }
    }
}
