// SPDX-License-Identifier: Apache-2.0

//! `/v1/jobs` HTTP surface: credential gating, header-forgery resistance,
//! id validation, and an end-to-end job against a mock llama endpoint.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use serde_json::{json, Value};
use slm_doorman::jobs::credentials::{Credential, PriorityClass};
use slm_doorman::jobs::engines::{
    JobEngine, LlmLocalEngine, OcrOpenaiVisionEngine, OpenGate, SpanGlinerEngine,
};
use slm_doorman::jobs::registry::{EngineDeclared, EngineKind, EngineSpec};
use slm_doorman::jobs::service::AuditSink;
use slm_doorman::jobs::{CredentialSet, JobStore, JobsConfig, JobsService, Registry};
use slm_doorman::JobAuditEntry;
use slm_doorman_server::jobs_http::router;
use tower::ServiceExt;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn sha(b: &[u8]) -> String {
    slm_doorman::jobs::sha256_hex(b)
}

fn cred(id: &str, token: &str, module: &str, engines: &[&str]) -> Credential {
    serde_json::from_value(json!({
        "credential_id": id,
        "token_sha256": sha(token.as_bytes()),
        "module_id": module,
        "priority_class": "idle",
        "engines": engines,
    }))
    .inspect(|c: &Credential| {
        assert_eq!(c.priority_class, PriorityClass::Idle);
    })
    .unwrap()
}

struct Rig {
    app: Router,
    audit: Arc<Mutex<Vec<JobAuditEntry>>>,
    _llama: MockServer,
    // Held for the whole test: a dropped wiremock server goes back to a shared
    // pool and another test can pick it up, which answers /v1/extract with 404.
    _gliner: MockServer,
    ocr: MockServer,
}

const OLMO_JSON: &str = r#"{"entities":[{"class":"Person","value":"Alice","witness":"Alice"},{"class":"Person","value":"Paris","witness":"Paris"}]}"#;

async fn rig() -> Rig {
    let llama = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "choices": [{"message": {"role": "assistant", "content": OLMO_JSON}}]
        })))
        .mount(&llama)
        .await;
    let gliner = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/extract"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "entities": [
                {"entity_name": "Alice", "classification": "Person"},
                {"entity_name": "Zed", "classification": "Person"},
                {"entity_name": "Paris", "classification": "Location"}
            ]
        })))
        .mount(&gliner)
        .await;
    let ocr = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "choices": [{"message": {"role": "assistant", "content": "STATEMENT PAGE TEXT"}}]
        })))
        .mount(&ocr)
        .await;
    let spec_span = |endpoint: String| EngineSpec {
        engine_id: "spans".into(),
        kind: EngineKind::SpanGliner,
        endpoint,
        model: "gliner-multi".into(),
        weights_sha256: "w".into(),
        licence: "unverified".into(),
        permits: 1,
        cpu_set: None,
        nice: None,
        max_job_secs: 30,
        max_input_bytes: 1 << 20,
        chunk_bytes: Some(2000),
        chunk_overlap: Some(100),
        domain_id: Some("projects".into()),
        domains: None,
        fallback: None,
        declared: Some(EngineDeclared {
            capability: Some("entity-spans".into()),
            owner: Some("extraction".into()),
            ..Default::default()
        }),
    };
    let ocr_spec = |id: &str, endpoint: String| EngineSpec {
        engine_id: id.into(),
        kind: EngineKind::OcrOpenaiVision,
        endpoint,
        model: "lighton-ocr-2-1b-q8".into(),
        weights_sha256: "w".into(),
        licence: "unverified".into(),
        permits: 1,
        cpu_set: Some("0-4".into()),
        nice: Some(19),
        max_job_secs: 30,
        max_input_bytes: 1 << 20,
        chunk_bytes: None,
        chunk_overlap: None,
        domain_id: None,
        domains: None,
        fallback: None,
        declared: None,
    };
    let spec = EngineSpec {
        engine_id: "olmo".into(),
        kind: EngineKind::LlmLocal,
        endpoint: llama.uri(),
        model: "OLMo-3-7B-Instruct".into(),
        weights_sha256: "w".into(),
        licence: "apache-2.0".into(),
        permits: 1,
        cpu_set: None,
        nice: None,
        max_job_secs: 30,
        max_input_bytes: 1 << 20,
        chunk_bytes: None,
        chunk_overlap: None,
        domain_id: None,
        domains: None,
        fallback: None,
        declared: None,
    };
    let mut engines: HashMap<String, Arc<dyn JobEngine>> = HashMap::new();
    engines.insert(
        "spans".into(),
        Arc::new(SpanGlinerEngine::new(
            gliner.uri(),
            None,
            Duration::from_secs(5),
        )),
    );
    engines.insert(
        "olmo".into(),
        Arc::new(LlmLocalEngine::new(
            llama.uri(),
            "OLMo-3-7B-Instruct".into(),
            Arc::new(OpenGate),
        )),
    );
    for id in ["ocr", "ocr-down"] {
        let ep = if id == "ocr" {
            ocr.uri()
        } else {
            "http://127.0.0.1:1".to_string()
        };
        engines.insert(
            id.into(),
            Arc::new(OcrOpenaiVisionEngine::new(
                ep,
                "lighton-ocr-2-1b-q8".into(),
                Duration::from_secs(5),
            )),
        );
    }
    let dir = std::env::temp_dir().join(format!(
        "slm-jobs-http-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let audit = Arc::new(Mutex::new(Vec::new()));
    let a2 = audit.clone();
    let sink: AuditSink = Arc::new(move |e| a2.lock().unwrap().push(e));
    let svc = JobsService::new(
        Registry::from_specs(vec![
            spec_span(gliner.uri()),
            spec,
            ocr_spec("ocr", ocr.uri()),
            ocr_spec("ocr-down", "http://127.0.0.1:1".into()),
        ])
        .unwrap(),
        JobStore::new(dir).unwrap(),
        engines,
        sink,
        JobsConfig::default(),
    )
    .unwrap();
    svc.start();
    let creds = CredentialSet::from_credentials(vec![
        cred(
            "a",
            "token-a",
            "mod-a",
            &["olmo", "ocr", "ocr-down", "spans"],
        ),
        cred("b", "token-b", "mod-b", &["olmo"]),
    ])
    .unwrap();
    Rig {
        app: router(svc, Arc::new(creds)),
        audit,
        _llama: llama,
        _gliner: gliner,
        ocr,
    }
}

async fn call(
    app: &Router,
    method: &str,
    uri: &str,
    token: Option<&str>,
    headers: &[(&str, &str)],
    body: Vec<u8>,
) -> (StatusCode, Value) {
    let mut b = Request::builder().method(method).uri(uri);
    if let Some(t) = token {
        b = b.header("authorization", format!("Bearer {t}"));
    }
    for (k, v) in headers {
        b = b.header(*k, *v);
    }
    let resp = app
        .clone()
        .oneshot(b.body(Body::from(body)).unwrap())
        .await
        .unwrap();
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), 4 << 20)
        .await
        .unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

async fn upload(app: &Router, token: &str, text: &str) -> (String, String) {
    let (s, v) = call(
        app,
        "POST",
        "/v1/jobs/inputs",
        Some(token),
        &[("content-type", "text/plain")],
        text.as_bytes().to_vec(),
    )
    .await;
    assert_eq!(s, StatusCode::CREATED, "{v}");
    (
        v["staged_ref"].as_str().unwrap().into(),
        v["sha256"].as_str().unwrap().into(),
    )
}

fn submit_body(sref: &str, sha: &str) -> Vec<u8> {
    json!({
        "engine_id": "olmo", "kind": "extract-llm",
        "input": {"sha256": sha, "media_type": "text/plain", "staged_ref": sref},
        "params": {"seed": 7}
    })
    .to_string()
    .into_bytes()
}

#[tokio::test]
async fn every_route_requires_a_valid_token() {
    let r = rig().await;
    for (m, u) in [
        ("GET", "/v1/engines"),
        ("POST", "/v1/jobs"),
        ("POST", "/v1/jobs/inputs"),
        ("GET", "/v1/jobs/0190b2c8-0000-7000-8000-000000000000"),
        (
            "GET",
            "/v1/jobs/0190b2c8-0000-7000-8000-000000000000/result",
        ),
        ("POST", "/v1/jobs/0190b2c8-0000-7000-8000-000000000000/ack"),
        (
            "POST",
            "/v1/jobs/0190b2c8-0000-7000-8000-000000000000/replay",
        ),
    ] {
        for t in [None, Some("wrong"), Some("")] {
            let (s, _) = call(&r.app, m, u, t, &[], vec![]).await;
            assert_eq!(s, StatusCode::UNAUTHORIZED, "{m} {u} {t:?}");
        }
    }
}

#[tokio::test]
async fn forged_headers_do_not_change_module_or_priority() {
    let r = rig().await;
    let (sref, sha) = upload(&r.app, "token-a", "hello world").await;
    let (s, v) = call(
        &r.app,
        "POST",
        "/v1/jobs",
        Some("token-a"),
        &[
            ("x-foundry-module-id", "mod-b"),
            ("x-foundry-priority", "P0"),
        ],
        submit_body(&sref, &sha),
    )
    .await;
    assert_eq!(s, StatusCode::ACCEPTED, "{v}");
    let id = v["job_id"].as_str().unwrap();
    let uri = format!("/v1/jobs/{id}");
    assert_eq!(
        call(&r.app, "GET", &uri, Some("token-a"), &[], vec![])
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        call(&r.app, "GET", &uri, Some("token-b"), &[], vec![])
            .await
            .0,
        StatusCode::NOT_FOUND
    );
}

#[tokio::test]
async fn path_like_and_garbage_ids_are_refused() {
    let r = rig().await;
    for id in [
        "..",
        "..%2F..%2Fetc%2Fpasswd",
        "a%2Fb",
        "not-a-uuid",
        "0190b2c8-0000-7000-8000-00000000000g",
    ] {
        for (m, suffix) in [
            ("GET", ""),
            ("GET", "/result"),
            ("POST", "/ack"),
            ("POST", "/replay"),
        ] {
            let (s, _) = call(
                &r.app,
                m,
                &format!("/v1/jobs/{id}{suffix}"),
                Some("token-a"),
                &[],
                vec![],
            )
            .await;
            assert!(
                s == StatusCode::BAD_REQUEST || s == StatusCode::NOT_FOUND,
                "{id} {suffix} -> {s}"
            );
        }
    }
    let (s, _) = call(
        &r.app,
        "GET",
        "/v1/jobs/not-a-uuid",
        Some("token-a"),
        &[],
        vec![],
    )
    .await;
    assert_eq!(s, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn bad_submissions_are_refused() {
    let r = rig().await;
    let (sref, sha) = upload(&r.app, "token-a", "hello").await;
    let mut body: Value = serde_json::from_slice(&submit_body(&sref, &sha)).unwrap();
    body["bogus"] = json!(1);
    let (s, _) = call(
        &r.app,
        "POST",
        "/v1/jobs",
        Some("token-a"),
        &[],
        body.to_string().into_bytes(),
    )
    .await;
    assert_eq!(s, StatusCode::BAD_REQUEST, "unknown field");
    let mut body: Value = serde_json::from_slice(&submit_body(&sref, &sha)).unwrap();
    body["engine_id"] = json!("other");
    let (s, _) = call(
        &r.app,
        "POST",
        "/v1/jobs",
        Some("token-a"),
        &[],
        body.to_string().into_bytes(),
    )
    .await;
    assert_eq!(s, StatusCode::FORBIDDEN, "engine not on allow-list");
    let mut body: Value = serde_json::from_slice(&submit_body(&sref, &sha)).unwrap();
    body["input"]["staged_ref"] = json!("../../etc/passwd");
    let (s, _) = call(
        &r.app,
        "POST",
        "/v1/jobs",
        Some("token-a"),
        &[],
        body.to_string().into_bytes(),
    )
    .await;
    assert_eq!(s, StatusCode::BAD_REQUEST, "path-like staged_ref");
    // Module B cannot use module A's staged input.
    let (s, _) = call(
        &r.app,
        "POST",
        "/v1/jobs",
        Some("token-b"),
        &[],
        submit_body(&sref, &sha),
    )
    .await;
    assert_eq!(s, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn end_to_end_upload_run_result_ack() {
    let r = rig().await;
    let (s, v) = call(&r.app, "GET", "/v1/engines", Some("token-a"), &[], vec![]).await;
    assert_eq!(s, StatusCode::OK);
    let engines = v["engines"].as_array().unwrap();
    assert!(engines.iter().any(|e| e["engine_id"] == "olmo"));
    let olmo = engines.iter().find(|e| e["engine_id"] == "olmo").unwrap();
    assert!(
        olmo["declared"].is_null(),
        "no declared block, reported null"
    );
    let spans = engines.iter().find(|e| e["engine_id"] == "spans").unwrap();
    assert_eq!(spans["declared"]["capability"], "entity-spans");
    assert_eq!(spans["declared"]["owner"], "extraction");
    let (sref, sha_in) = upload(&r.app, "token-a", "Alice met Bob in Paris").await;
    let (s, v) = call(
        &r.app,
        "POST",
        "/v1/jobs",
        Some("token-a"),
        &[],
        submit_body(&sref, &sha_in),
    )
    .await;
    assert_eq!(s, StatusCode::ACCEPTED, "{v}");
    let id = v["job_id"].as_str().unwrap().to_string();
    let uri = format!("/v1/jobs/{id}");
    let mut status = Value::Null;
    for _ in 0..200 {
        let (_, v) = call(&r.app, "GET", &uri, Some("token-a"), &[], vec![]).await;
        if v["status"] != "queued" && v["status"] != "running" {
            status = v;
            break;
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    assert_eq!(status["status"], "done", "{status}");
    assert_eq!(status["record"]["input_sha256"], sha_in);
    assert_eq!(status["record"]["output_sha256"], sha(OLMO_JSON.as_bytes()));
    assert_eq!(status["record"]["params"]["seed"], 7);
    let (s, v) = call(
        &r.app,
        "GET",
        &format!("{uri}/result"),
        Some("token-a"),
        &[],
        vec![],
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(v["text"], OLMO_JSON);
    let (s, _) = call(
        &r.app,
        "POST",
        &format!("{uri}/ack"),
        Some("token-a"),
        &[],
        vec![],
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    let (s, _) = call(
        &r.app,
        "GET",
        &format!("{uri}/result"),
        Some("token-a"),
        &[],
        vec![],
    )
    .await;
    assert_eq!(s, StatusCode::NOT_FOUND, "result deleted by ack");
    let audit = r.audit.lock().unwrap();
    assert_eq!(audit.len(), 2);
    let all = serde_json::to_string(&*audit).unwrap();
    assert!(
        !all.contains("Alice") && !all.contains("Paris"),
        "no content in audit"
    );
}

const PNG: &[u8] = &[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, 9, 9, 9];

async fn upload_bytes(app: &Router, token: &str, mt: &str, bytes: &[u8]) -> (String, String) {
    let (s, v) = call(
        app,
        "POST",
        "/v1/jobs/inputs",
        Some(token),
        &[("content-type", mt)],
        bytes.to_vec(),
    )
    .await;
    assert_eq!(s, StatusCode::CREATED, "{v}");
    (
        v["staged_ref"].as_str().unwrap().into(),
        v["sha256"].as_str().unwrap().into(),
    )
}

fn ocr_body(engine: &str, kind: &str, mt: &str, sref: &str, sha: &str) -> Vec<u8> {
    json!({
        "engine_id": engine, "kind": kind,
        "input": {"sha256": sha, "media_type": mt, "staged_ref": sref},
        "params": {"page_ref": "stmt-2024-03.p3", "seed": 1}
    })
    .to_string()
    .into_bytes()
}

async fn finish(app: &Router, token: &str, id: &str) -> Value {
    for _ in 0..200 {
        let (_, v) = call(
            app,
            "GET",
            &format!("/v1/jobs/{id}"),
            Some(token),
            &[],
            vec![],
        )
        .await;
        if v["status"] != "queued" && v["status"] != "running" {
            return v;
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    panic!("job never finished");
}

#[tokio::test]
async fn ocr_page_job_returns_one_result_naming_reader_and_page() {
    let r = rig().await;
    let (sref, sha_in) = upload_bytes(&r.app, "token-a", "image/png", PNG).await;
    let (s, v) = call(
        &r.app,
        "POST",
        "/v1/jobs",
        Some("token-a"),
        &[],
        ocr_body("ocr", "ocr-page", "image/png", &sref, &sha_in),
    )
    .await;
    assert_eq!(s, StatusCode::ACCEPTED, "{v}");
    let id = v["job_id"].as_str().unwrap().to_string();
    let st = finish(&r.app, "token-a", &id).await;
    assert_eq!(st["status"], "done", "{st}");
    assert_eq!(st["record"]["engine_id"], "ocr");
    assert_eq!(st["record"]["model_id"], "lighton-ocr-2-1b-q8");
    assert_eq!(st["record"]["params"]["page_ref"], "stmt-2024-03.p3");
    assert_eq!(st["record"]["prompt_sha256"], "");
    let (_, res) = call(
        &r.app,
        "GET",
        &format!("/v1/jobs/{id}/result"),
        Some("token-a"),
        &[],
        vec![],
    )
    .await;
    assert_eq!(res["text"], "STATEMENT PAGE TEXT");
    assert_eq!(r.ocr.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn stopped_ocr_engine_defers_and_no_other_reader_is_used() {
    let r = rig().await;
    let (sref, sha_in) = upload_bytes(&r.app, "token-a", "image/png", PNG).await;
    let (s, v) = call(
        &r.app,
        "POST",
        "/v1/jobs",
        Some("token-a"),
        &[],
        ocr_body("ocr-down", "ocr-page", "image/png", &sref, &sha_in),
    )
    .await;
    assert_eq!(s, StatusCode::ACCEPTED, "{v}");
    let st = finish(&r.app, "token-a", v["job_id"].as_str().unwrap()).await;
    assert_eq!(st["status"], "deferred", "{st}");
    assert_eq!(
        r.ocr.received_requests().await.unwrap().len(),
        0,
        "healthy reader must not pick it up"
    );
}

#[tokio::test]
async fn ocr_kind_validation() {
    let r = rig().await;
    let (sref, sha_in) = upload_bytes(&r.app, "token-a", "image/png", PNG).await;
    let (s, _) = call(
        &r.app,
        "POST",
        "/v1/jobs",
        Some("token-a"),
        &[],
        ocr_body("olmo", "ocr-page", "image/png", &sref, &sha_in),
    )
    .await;
    assert_eq!(s, StatusCode::BAD_REQUEST, "OLMo cannot take ocr-page");
    let (s, _) = call(
        &r.app,
        "POST",
        "/v1/jobs",
        Some("token-a"),
        &[],
        ocr_body("ocr", "ocr-page", "text/plain", &sref, &sha_in),
    )
    .await;
    assert_eq!(s, StatusCode::BAD_REQUEST, "non-image media type");
    let (sref2, sha2) = upload_bytes(&r.app, "token-a", "image/png", b"not an image").await;
    let (s, v) = call(
        &r.app,
        "POST",
        "/v1/jobs",
        Some("token-a"),
        &[],
        ocr_body("ocr", "ocr-page", "image/png", &sref2, &sha2),
    )
    .await;
    assert_eq!(s, StatusCode::ACCEPTED);
    let st = finish(&r.app, "token-a", v["job_id"].as_str().unwrap()).await;
    assert_eq!(st["status"], "unreadable");
    assert_eq!(r.ocr.received_requests().await.unwrap().len(), 0);
}

#[tokio::test]
async fn span_engine_vs_olmo_comparison_flags_disagreement_and_invention() {
    let r = rig().await;
    let text = "Alice flew to Paris.";
    let (sref, sha_in) = upload(&r.app, "token-a", text).await;
    let body = |engine: &str, kind: &str, prompt: &str| {
        json!({
            "engine_id": engine, "kind": kind,
            "input": {"sha256": sha_in, "media_type": "text/plain", "staged_ref": sref},
            "params": {"prompt_id": prompt, "seed": 1}
        })
        .to_string()
        .into_bytes()
    };
    let (s1, v1) = call(
        &r.app,
        "POST",
        "/v1/jobs",
        Some("token-a"),
        &[],
        body("olmo", "extract-llm", "entities-v2"),
    )
    .await;
    let (s2, v2) = call(
        &r.app,
        "POST",
        "/v1/jobs",
        Some("token-a"),
        &[],
        body("spans", "extract-spans", "spans-v1"),
    )
    .await;
    assert_eq!(
        (s1, s2),
        (StatusCode::ACCEPTED, StatusCode::ACCEPTED),
        "{v1} {v2}"
    );
    let (olmo_id, span_id) = (
        v1["job_id"].as_str().unwrap().to_string(),
        v2["job_id"].as_str().unwrap().to_string(),
    );
    let st = finish(&r.app, "token-a", &olmo_id).await;
    assert_eq!(st["status"], "done", "{st}");
    let st = finish(&r.app, "token-a", &span_id).await;
    assert_eq!(st["status"], "done", "{st}");
    // GLiNER's invented "Zed" never appears in the text: rejected, counted.
    let (_, res) = call(
        &r.app,
        "GET",
        &format!("/v1/jobs/{span_id}/result"),
        Some("token-a"),
        &[],
        vec![],
    )
    .await;
    assert_eq!(res["rejected"], 1);
    assert_eq!(res["entities"].as_array().unwrap().len(), 2);
    let (s, c) = call(
        &r.app,
        "GET",
        &format!("/v1/jobs/compare?a={olmo_id}&b={span_id}"),
        Some("token-a"),
        &[],
        vec![],
    )
    .await;
    assert_eq!(s, StatusCode::OK, "{c}");
    assert_eq!(
        c["agreed"].as_array().unwrap().len(),
        1,
        "Alice/Person agrees"
    );
    assert_eq!(
        c["only_a"][0]["value"], "Paris",
        "OLMo said Person, GLiNER said Location"
    );
    assert_eq!(c["only_b"][0]["class"], "Location");
    // Another module cannot compare these jobs; bad ids are refused.
    let (s, _) = call(
        &r.app,
        "GET",
        &format!("/v1/jobs/compare?a={olmo_id}&b={span_id}"),
        Some("token-b"),
        &[],
        vec![],
    )
    .await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    let (s, _) = call(
        &r.app,
        "GET",
        "/v1/jobs/compare?a=..&b=x",
        Some("token-a"),
        &[],
        vec![],
    )
    .await;
    assert_eq!(s, StatusCode::BAD_REQUEST);
    // extract-spans on an llm engine / a foreign prompt id is refused.
    let (s, _) = call(
        &r.app,
        "POST",
        "/v1/jobs",
        Some("token-a"),
        &[],
        body("olmo", "extract-spans", "spans-v1"),
    )
    .await;
    assert_eq!(s, StatusCode::BAD_REQUEST);
    let (s, _) = call(
        &r.app,
        "POST",
        "/v1/jobs",
        Some("token-a"),
        &[],
        body("spans", "extract-spans", "entities-v2"),
    )
    .await;
    assert_eq!(s, StatusCode::BAD_REQUEST);
}
