// SPDX-License-Identifier: Apache-2.0

//! Tier A — local OpenAI-compatible HTTP endpoint.
//!
//! Backed by mistral.rs (long-term Phase-2 runtime per SLM-STACK.md) or
//! llama-server (the Phase-1 prototype runtime per Master's v0.0.9
//! progress note — the runtime that A3 used). Both expose the same
//! OpenAI-compatible wire format, so the client does not branch on which
//! is running.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use slm_core::{ChatMessage, ComputeRequest, ComputeResponse, GrammarConstraint, InferenceRoute};
use tokio::sync::{watch, OwnedSemaphorePermit, Semaphore};
use tracing::debug;

use crate::error::{DoormanError, Result};
use crate::scheduler::{InfightPriority, RequestScheduler, SchedulerPermit};

/// In-process Tier A admission-control snapshot — see `LocalTierClient::admission_status`.
#[derive(Clone, Debug, Serialize)]
pub struct LocalAdmissionStatus {
    pub total_permits_available: Option<usize>,
    pub background_permits_available: Option<usize>,
    pub active_interactive: usize,
}

#[derive(Clone, Debug)]
pub struct LocalTierConfig {
    /// Base URL of the local OpenAI-compatible server, e.g.
    /// `http://127.0.0.1:8080`.
    pub endpoint: String,
    /// Default model identifier. Local Tier A runs OLMo 3 7B Q4
    /// (Apache-2.0 + Open Data Commons; see substrate decision).
    pub default_model: String,
}

/// RAII guard that decrements `active_interactive` and clears the preemption
/// signal when the interactive future completes or is cancelled (dropped).
/// Without this, a dropped HTTP handler would leave the counter stuck > 0,
/// causing all subsequent background callers to return LocalSaturated forever.
struct InteractiveGuard {
    active_interactive: Arc<AtomicUsize>,
    interactive_tx: Arc<watch::Sender<bool>>,
    // Production-readiness roadmap Tier 0.3: carried purely for the acquire/
    // release log pair below, so a stuck-permit incident is diagnosable from
    // the journal (request_id + available_permits at each transition)
    // instead of requiring a restart to even confirm what happened.
    request_id: String,
    scheduler: Option<Arc<RequestScheduler>>,
    acquired_at: Instant,
}

impl Drop for InteractiveGuard {
    fn drop(&mut self) {
        let remaining = self.active_interactive.fetch_sub(1, Ordering::SeqCst);
        if remaining == 1 {
            let _ = self.interactive_tx.send(false);
        }
        debug!(
            request_id = %self.request_id,
            held_ms = self.acquired_at.elapsed().as_millis() as u64,
            total_permits_available = self.scheduler.as_ref().map(|s| s.available_permits()),
            "interactive Tier A permit released"
        );
    }
}

pub struct LocalTierClient {
    config: LocalTierConfig,
    http: reqwest::Client,
    /// Total concurrent OLMo slots (SLM_LOCAL_CONCURRENT, default 2) — the
    /// single source of truth for "is Tier A busy," shared by all three
    /// priority classes (P0 interactive, P1 background, P2 training). When
    /// full, the caller receives LocalSaturated immediately instead of
    /// queuing inside llama-server for up to 1 800 s.
    ///
    /// This used to be a bare `Arc<Semaphore>` with no priority awareness.
    /// Replaced 2026-09-21 with `RequestScheduler` (same underlying
    /// semaphore/capacity, so P0/P1 admission is behaviorally identical —
    /// "admit if any slot free") so `complete_p2()` can ask "are ALL slots
    /// free" using the *same* semaphore P0/P1 already acquire, rather than a
    /// second, independent counter that could drift from the real state.
    scheduler: Option<Arc<RequestScheduler>>,
    /// Background-only semaphore (SLM_BACKGROUND_CONCURRENT, default 1).
    /// Acquired BEFORE the scheduler slot by extraction fallback and drain
    /// dispatch. Ensures at least one total slot remains free for interactive
    /// callers even when background work is in flight.
    background_sem: Option<Arc<Semaphore>>,
    /// Count of in-flight interactive requests. Background callers watch this
    /// via `interactive_rx`; when it goes non-zero they abort their in-progress
    /// OLMo request (via tokio::select!) and return LocalSaturated so
    /// service-content backs off. This prevents interactive chat from sharing
    /// OLMo batch cycles with long extraction jobs.
    active_interactive: Arc<AtomicUsize>,
    /// Sends `true` when the first interactive request starts, `false` when
    /// the last one completes. Background callers watch the paired receiver.
    interactive_tx: Arc<watch::Sender<bool>>,
    /// Cloned per `complete_background()` call for the preemption select!.
    interactive_rx: watch::Receiver<bool>,
}

impl LocalTierClient {
    pub fn new(config: LocalTierConfig) -> Self {
        // 1800 s (30 min) covers OLMo 7B Q4_K_M CPU inference up to max_tokens=2048
        // at the observed rate of ~2 tok/s (1024 s theoretical max) plus prefill
        // overhead. Observed real-world runs on this hardware: 17–60 minutes.
        // The prior 120 s value caused an infinite retry loop: the drain worker
        // timed out before llama-server finished, Doorman re-queued the brief,
        // and the next attempt immediately timed out again.
        // Without this timeout the drain worker blocks indefinitely.
        let (interactive_tx, interactive_rx) = watch::channel(false);
        Self {
            config,
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(1800))
                // Catch TCP-level hangs independently of the response timeout.
                // Tier A is always localhost, so 10 s is generous.
                .connect_timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("failed to build Tier A HTTP client"),
            scheduler: None,
            background_sem: None,
            active_interactive: Arc::new(AtomicUsize::new(0)),
            interactive_tx: Arc::new(interactive_tx),
            interactive_rx,
        }
    }

    /// Attach priority admission control (production only). `scheduler`'s
    /// capacity should equal `SLM_LOCAL_CONCURRENT`; `background` should
    /// equal `SLM_BACKGROUND_CONCURRENT`. Tests call `new()` alone and pass
    /// `None` implicitly (no cap applied).
    pub fn with_semaphores(
        mut self,
        scheduler: Arc<RequestScheduler>,
        background: Arc<Semaphore>,
    ) -> Self {
        self.scheduler = Some(scheduler);
        self.background_sem = Some(background);
        self
    }

    /// Production-readiness roadmap Tier 0.5: expose in-process admission-
    /// control state so `/v1/status/tier-a` can report it alongside
    /// llama-server's own `/metrics`, and a caller can detect the divergence
    /// (permits exhausted while llama-server itself is idle) that previously
    /// required a `systemctl restart` to even confirm, let alone clear.
    pub fn admission_status(&self) -> LocalAdmissionStatus {
        LocalAdmissionStatus {
            total_permits_available: self.scheduler.as_ref().map(|s| s.available_permits()),
            background_permits_available: self
                .background_sem
                .as_ref()
                .map(|s| s.available_permits()),
            active_interactive: self.active_interactive.load(Ordering::SeqCst),
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.config.endpoint
    }

    /// Non-blocking P2 (idle-only) admission against the shared scheduler,
    /// for callers that make their own request to the llama-server (the
    /// `/v1/jobs` engines). `None` when saturated or no scheduler configured
    /// (fail closed).
    pub fn try_admit_p2(&self) -> Option<SchedulerPermit> {
        self.scheduler.as_ref()?.try_admit(InfightPriority::P2)
    }

    /// Interactive path (P0): admit if any scheduler slot is free.
    /// Returns LocalSaturated immediately when none are.
    fn try_acquire_interactive(&self) -> Result<Option<SchedulerPermit>> {
        match &self.scheduler {
            None => Ok(None),
            Some(s) => s
                .try_admit(InfightPriority::P0)
                .ok_or(())
                .map(Some)
                .map_err(|_| DoormanError::LocalSaturated),
        }
    }

    /// Background path (P1): acquires `background_sem` first (caps
    /// background concurrency), then a scheduler slot (caps total OLMo
    /// load, shared with P0/P2). If either is full, returns LocalSaturated
    /// immediately. When `background_sem` succeeds but the scheduler
    /// doesn't, the background permit is dropped automatically before
    /// returning the error.
    fn try_acquire_background(
        &self,
    ) -> Result<(Option<OwnedSemaphorePermit>, Option<SchedulerPermit>)> {
        let bg = match &self.background_sem {
            None => None,
            Some(sem) => match sem.clone().try_acquire_owned() {
                Ok(p) => Some(p),
                Err(_) => return Err(DoormanError::LocalSaturated),
            },
        };
        let total = match &self.scheduler {
            None => None,
            Some(s) => match s.try_admit(InfightPriority::P1) {
                Some(p) => Some(p),
                None => return Err(DoormanError::LocalSaturated),
            },
        };
        Ok((bg, total))
    }

    /// Training/proof-run path (P2): admit ONLY when every scheduler slot is
    /// free — no interactive, no background work in flight. Fails closed:
    /// `None` scheduler (not configured) is refused, never treated as
    /// "always idle." Unlike `complete_background()`, this does not race
    /// against a live preemption signal — by construction P2 only starts
    /// once nothing else is running, so there is nothing to preempt it
    /// mid-flight beyond the scheduler's own admission check for the *next*
    /// P0/P1 caller (which correctly returns LocalSaturated to them, not to
    /// this already-admitted P2 call).
    pub async fn complete_p2(&self, req: &ComputeRequest) -> Result<ComputeResponse> {
        let scheduler = self
            .scheduler
            .as_ref()
            .ok_or(DoormanError::LocalSaturated)?;
        let _permit = scheduler
            .try_admit(InfightPriority::P2)
            .ok_or(DoormanError::LocalSaturated)?;
        debug!(
            request_id = %req.request_id,
            "P2 (training/proof-run) Tier A permit acquired — backend was idle"
        );
        self.complete_inner(req).await
    }

    /// Poll `GET /slots` on the local llama-server until all slots report
    /// `is_processing: false`, or until `timeout` elapses.
    ///
    /// Called after firing the preemption signal to ensure background's OLMo
    /// slot is actually freed before we send our request.  Without this, a
    /// mid-prefill background request and our interactive request end up in the
    /// same continuous-batch iteration, which adds 2–15 s of combined prefill
    /// latency.  The slot is freed when llama-server detects the dropped TCP
    /// connection, which only happens between batch iterations (≤ 5 s on this
    /// hardware).
    async fn wait_for_olmo_slots_free(&self, timeout: Duration) {
        let start = Instant::now();
        // Production-readiness roadmap Tier 0.2: this whole poll used to check
        // its deadline only *between* iterations, while each `self.http` call
        // reused the 1800s inference-request timeout — so a single slow GET
        // (e.g. llama-server at ~100% CPU under contention) could block far
        // past `timeout` while the caller still holds a total_sem permit and
        // active_interactive is already incremented. Wrap the whole loop in a
        // real, enforced timeout so this can never exceed the stated budget.
        let _ = tokio::time::timeout(timeout, self.poll_olmo_slots_free_inner()).await;
        debug!(
            elapsed_ms = start.elapsed().as_millis() as u64,
            budget_ms = timeout.as_millis() as u64,
            "wait_for_olmo_slots_free finished"
        );
    }

    async fn poll_olmo_slots_free_inner(&self) {
        let url = format!("{}/slots", self.config.endpoint.trim_end_matches('/'));
        loop {
            tokio::time::sleep(Duration::from_millis(75)).await;
            match self.http.get(&url).send().await {
                Ok(resp) => match resp.json::<Vec<serde_json::Value>>().await {
                    Ok(slots) => {
                        let all_free = slots.iter().all(|s| {
                            !s.get("is_processing")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false)
                        });
                        if all_free {
                            return;
                        }
                    }
                    Err(_) => return, // parsing error; proceed
                },
                Err(_) => return, // OLMo unreachable; proceed
            }
        }
    }

    /// Interactive caller (e.g. `/v1/chat/completions`).
    /// Acquires one total slot; returns LocalSaturated when saturated.
    /// Increments active_interactive while in flight so background callers
    /// can detect contention and abort their OLMo requests promptly.
    /// The InteractiveGuard ensures cleanup runs even when the future is
    /// cancelled (HTTP connection dropped mid-flight).
    ///
    /// When we are the FIRST interactive request (prev == 0), we fire the
    /// preemption signal and then wait for the background's OLMo slot to be
    /// freed before sending our own request.  This prevents combined prefill
    /// batching, which would add 2–15 s of latency even after preemption.
    pub async fn complete(&self, req: &ComputeRequest) -> Result<ComputeResponse> {
        let _permit = self.try_acquire_interactive()?;
        debug!(
            request_id = %req.request_id,
            total_permits_available = self.scheduler.as_ref().map(|s| s.available_permits()),
            "interactive Tier A permit acquired"
        );
        let prev = self.active_interactive.fetch_add(1, Ordering::SeqCst);
        let _guard = InteractiveGuard {
            active_interactive: Arc::clone(&self.active_interactive),
            interactive_tx: Arc::clone(&self.interactive_tx),
            request_id: req.request_id.to_string(),
            scheduler: self.scheduler.clone(),
            acquired_at: Instant::now(),
        };
        if prev == 0 {
            let _ = self.interactive_tx.send(true);
            // Only wait for slot clearance when background semaphores are
            // configured (production).  Tests omit semaphores, so background
            // callers cannot exist — polling /slots would be a spurious request.
            if self.background_sem.is_some() {
                // Wait up to 8 s for the background slot to be freed.
                // llama-server detects a dropped TCP connection between batch
                // iterations; on this hardware each iteration is ≤ 5 s.
                self.wait_for_olmo_slots_free(Duration::from_secs(8)).await;
            }
        }
        self.complete_inner(req).await
    }

    /// Background caller (extraction fallback, drain dispatch).
    /// Acquires background_sem then total_sem; returns LocalSaturated when
    /// either is full so the caller can back off without queuing in llama-server.
    /// Preempts itself when an interactive request arrives mid-flight:
    /// the in-progress OLMo HTTP request is cancelled (connection dropped),
    /// freeing the OLMo slot for the interactive caller within ~1 second.
    pub async fn complete_background(&self, req: &ComputeRequest) -> Result<ComputeResponse> {
        // Fast reject if interactive is already in flight.
        if self.active_interactive.load(Ordering::SeqCst) > 0 {
            return Err(DoormanError::LocalSaturated);
        }
        let (_bg, _total) = self.try_acquire_background()?;
        // Narrow the TOCTOU window: re-check after semaphore acquisition.
        if self.active_interactive.load(Ordering::SeqCst) > 0 {
            return Err(DoormanError::LocalSaturated);
        }
        // Race OLMo completion against the interactive-active signal. `biased`
        // ensures the signal branch is evaluated first — if interactive was
        // signalled between the check above and this select!, we bail immediately
        // without dispatching to OLMo.
        let mut sig = self.interactive_rx.clone();
        tokio::select! {
            biased;
            res = sig.wait_for(|&v| v) => {
                // `wait_for` only errors if the sender is dropped (server shutdown).
                let _ = res;
                Err(DoormanError::LocalSaturated)
            }
            // Use streaming so llama-server detects the dropped connection within
            // one token (~0.5 s) when this future is cancelled at the select! above.
            // Non-streaming only writes at the end of generation; the slot would
            // stay occupied for minutes even after preemption fires.
            result = self.complete_inner_streaming(req) => result,
        }
    }

    /// Background path: same as `complete_inner` but sends `stream: true`.
    ///
    /// With streaming, llama-server writes one chunk per token.  When the TCP
    /// connection is dropped (preemption signal fires → future dropped → TCP FIN
    /// sent), llama-server detects the broken pipe on the NEXT token write,
    /// within ~0.5 s.  With the default non-streaming mode, llama-server only
    /// tries to write once — after full generation — so it may hold the slot for
    /// minutes before detecting the disconnect.  Using streaming here makes
    /// preemption effective even when the background caller is mid-prefill.
    async fn complete_inner_streaming(&self, req: &ComputeRequest) -> Result<ComputeResponse> {
        let model = req
            .model
            .clone()
            .unwrap_or_else(|| self.config.default_model.clone());

        let (grammar_field, json_schema_field) = match req.grammar.as_ref() {
            None => (None, None),
            Some(GrammarConstraint::Gbnf(s)) => (Some(s.clone()), None),
            Some(GrammarConstraint::JsonSchema(v)) => (None, Some(v.clone())),
            Some(GrammarConstraint::Lark(_)) => {
                return Err(DoormanError::TierAGrammarUnsupported {
                    dialect: "Lark",
                    advice: "escalate to Tier B (Yo-Yo) which supports Lark via llguidance, \
                             or provide a GBNF equivalent for Tier A",
                });
            }
        };

        let body = OpenAiChatRequest {
            model: model.clone(),
            messages: req.messages.clone(),
            stream: true, // streaming → broken-pipe detected per-token, not at end
            max_tokens: req.max_tokens,
            temperature: req.temperature,
            grammar: grammar_field,
            json_schema: json_schema_field,
            stop: req.stop_sequences.clone(),
            tools: req.tools.as_ref().map(super::anthropic_tools_to_openai),
            cache_prompt: false,
        };

        let url = format!(
            "{}/v1/chat/completions",
            self.config.endpoint.trim_end_matches('/')
        );
        debug!(target: "slm_doorman::tier::local", %url, %model, "tier-A background streaming request");

        let started = Instant::now();
        let mut resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let mut content = String::new();
        let mut line_buf = String::new();

        while let Some(chunk) = resp.chunk().await? {
            line_buf.push_str(&String::from_utf8_lossy(&chunk));
            // Consume all complete newline-terminated lines in the buffer.
            while let Some(nl) = line_buf.find('\n') {
                let line = line_buf[..nl].to_string();
                line_buf = line_buf[nl + 1..].to_string();
                let line = line.trim();
                if let Some(data) = line.strip_prefix("data: ") {
                    if data == "[DONE]" {
                        break;
                    }
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(delta) = v["choices"][0]["delta"]["content"].as_str() {
                            content.push_str(delta);
                        }
                    }
                }
            }
        }

        let inference_ms = started.elapsed().as_millis() as u64;

        Ok(ComputeResponse {
            request_id: req.request_id,
            tier_used: InferenceRoute::Local,
            model,
            content,
            reasoning_content: None,
            inference_ms,
            cost_usd: 0.0,
            upstream_version: None,
            tool_calls: None,
        })
    }

    async fn complete_inner(&self, req: &ComputeRequest) -> Result<ComputeResponse> {
        let model = req
            .model
            .clone()
            .unwrap_or_else(|| self.config.default_model.clone());

        // Translate GrammarConstraint → llama-server wire fields.
        // llama-server (llama.cpp HTTP API) accepts:
        //   `grammar`     — GBNF string at the top level of the request body
        //   `json_schema` — JSON Schema object at the top level
        // It does NOT accept Lark grammars (llama-server does not ship
        // llguidance). Lark is rejected here before any network call so the
        // caller can escalate to Tier B (vLLM ≥0.12, which supports Lark via
        // llguidance) or supply a GBNF equivalent. Per v0.1.33 Q1 ratification.
        let (grammar_field, json_schema_field) = match req.grammar.as_ref() {
            None => (None, None),
            Some(GrammarConstraint::Gbnf(s)) => (Some(s.clone()), None),
            Some(GrammarConstraint::JsonSchema(v)) => (None, Some(v.clone())),
            Some(GrammarConstraint::Lark(_)) => {
                return Err(DoormanError::TierAGrammarUnsupported {
                    dialect: "Lark",
                    advice: "escalate to Tier B (Yo-Yo) which supports Lark via llguidance, \
                             or provide a GBNF equivalent for Tier A",
                });
            }
        };

        let body = OpenAiChatRequest {
            model: model.clone(),
            messages: req.messages.clone(),
            stream: req.stream,
            max_tokens: req.max_tokens,
            temperature: req.temperature,
            grammar: grammar_field,
            json_schema: json_schema_field,
            stop: req.stop_sequences.clone(),
            tools: req.tools.as_ref().map(super::anthropic_tools_to_openai),
            // Production-readiness roadmap Tier 1.7: re-enabled for the
            // interactive path now that `local-slm.service` runs
            // `--swa-full` (cures the 26x-reprocess-on-cache-miss
            // pathology this field's own doc comment describes -- that
            // was the reason it was false, not a general objection to
            // caching) plus q8_0 K/V quantization to keep the larger
            // full-size SWA cache's memory cost down. Left `false` in
            // `complete_inner_streaming` (the background/extraction
            // path) -- that traffic varies too widely turn-to-turn for
            // prefix reuse to help, per Fable's research pass. Live
            // evidence this was worth doing: a production host's own logs
            // already showed llama-server finding a reusable cached
            // prefix (`sim_best = 0.999`) on a same-session follow-up
            // turn, then discarding it because caching was off.
            cache_prompt: true,
        };
        let url = format!(
            "{}/v1/chat/completions",
            self.config.endpoint.trim_end_matches('/')
        );
        debug!(target: "slm_doorman::tier::local", %url, %model, "tier-A request");

        let started = Instant::now();
        let resp: OpenAiChatResponse = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let inference_ms = started.elapsed().as_millis() as u64;

        let msg = resp
            .choices
            .into_iter()
            .next()
            .map(|c| c.message)
            .ok_or_else(|| DoormanError::UpstreamShape("no choices in response".into()))?;

        Ok(ComputeResponse {
            request_id: req.request_id,
            tier_used: InferenceRoute::Local,
            model,
            content: msg.content.unwrap_or_default(),
            reasoning_content: None,
            inference_ms,
            // Tier A runs on already-paid-for VM compute; per substrate
            // decision the marginal cost is sunk in the VM cost.
            cost_usd: 0.0,
            upstream_version: None,
            tool_calls: msg.tool_calls,
        })
    }
}

#[derive(Serialize)]
struct OpenAiChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "is_false")]
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    /// GBNF grammar string. Top-level llama-server field (NOT inside
    /// `extra_body`). Absent when `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    grammar: Option<String>,
    /// JSON Schema for structured output. Top-level llama-server field.
    /// Absent when `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    json_schema: Option<serde_json::Value>,
    /// Stop sequences. Generation halts at the first match. llama-server
    /// accepts this as a top-level `stop` array. Absent when `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    /// OpenAI-format tools array (converted from Anthropic format by
    /// `anthropic_tools_to_openai`). Absent when `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<serde_json::Value>,
    /// Whether llama-server may reuse a slot's cached KV state across
    /// requests. OLMo-3 SWA (Sliding Window Attention) causes 26× slower
    /// prefill when a slot's cached prefix doesn't match the new prompt on
    /// a `--swa-full`-less server, which is why this used to be
    /// unconditionally `false`. Tier 1.7 (production-readiness roadmap)
    /// fixed the actual pathology at the source (`--swa-full` on
    /// `local-slm.service`) instead of avoiding it here, and re-enabled
    /// this for the interactive path (`complete_inner`) — a same-session
    /// follow-up turn's largely-unchanged system prompt + growing history
    /// now gets real prefix reuse. Still `false` in
    /// `complete_inner_streaming` (the background/extraction path):
    /// that traffic varies too widely turn-to-turn for reuse to help.
    cache_prompt: bool,
}

#[derive(Deserialize)]
struct OpenAiChatResponse {
    choices: Vec<OpenAiChatChoice>,
}

#[derive(Deserialize)]
struct OpenAiChatChoice {
    message: LocalAssistantMessage,
}

/// Assistant turn from llama-server. Content may be null when the model
/// chose to emit tool_calls instead of text.
#[derive(Deserialize)]
struct LocalAssistantMessage {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<serde_json::Value>,
}

fn is_false(b: &bool) -> bool {
    !*b
}

#[cfg(test)]
mod tests {
    use super::*;
    use slm_core::{
        ChatMessage, Complexity, ComputeRequest, GrammarConstraint, InferenceRoute, ModuleId,
        RequestId,
    };
    use std::str::FromStr;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn req() -> ComputeRequest {
        ComputeRequest {
            request_id: RequestId::new(),
            module_id: ModuleId::from_str("foundry").unwrap(),
            model: Some("OLMo-3-7B-Q4_K_M.gguf".into()),
            messages: vec![ChatMessage {
                role: "user".into(),
                content: "ping".into(),
            }],
            complexity: Complexity::Low,
            tier_hint: Some(InferenceRoute::Local),
            stream: false,
            max_tokens: Some(20),
            temperature: Some(0.0),
            sanitised_outbound: true,
            tier_c_label: None,
            yoyo_label: None,
            grammar: None,
            speculation: None,
            graph_context_enabled: None,
            tools: None,
            stop_sequences: None,
            session_context: None,
        }
    }

    fn ok_body() -> serde_json::Value {
        serde_json::json!({
            "choices": [
                { "message": { "role": "assistant", "content": "PONG" } }
            ]
        })
    }

    fn client(server_uri: String) -> LocalTierClient {
        LocalTierClient::new(LocalTierConfig {
            endpoint: server_uri,
            default_model: "OLMo-3-7B-Q4_K_M.gguf".into(),
        })
    }

    /// Happy path — 200 with well-formed choices array. Verify:
    /// - content extracted correctly
    /// - tier_used is Local
    /// - model is echoed from the request
    /// - cost_usd is 0.0 (VM compute is sunk cost per architecture)
    /// - request shape includes model and messages (POST to /v1/chat/completions)
    #[tokio::test]
    async fn happy_path_200_returns_content_and_local_tier() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let resp = client.complete(&req()).await.expect("happy path 200");
        assert_eq!(resp.tier_used, InferenceRoute::Local);
        assert_eq!(resp.content, "PONG");
        assert_eq!(resp.model, "OLMo-3-7B-Q4_K_M.gguf");
        assert_eq!(
            resp.cost_usd, 0.0,
            "Tier A cost is always 0.0 (sunk VM compute)"
        );
        assert!(
            resp.inference_ms < 10_000,
            "sanity: inference_ms should be wall-clock ms"
        );
    }

    /// Default model is used when request carries no model field.
    /// Tier A's default is the configured `LocalTierConfig::default_model`.
    #[tokio::test]
    async fn default_model_used_when_request_omits_model() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let mut r = req();
        r.model = None; // no model in request — should fall back to config default
        let resp = client.complete(&r).await.expect("default model happy path");
        assert_eq!(resp.model, "OLMo-3-7B-Q4_K_M.gguf");
    }

    /// HTTP 5xx error via `error_for_status()` must surface as
    /// `DoormanError::Upstream` (the `#[from] reqwest::Error` variant).
    /// The Doorman does NOT retry on Tier A — a 500 from the local
    /// llama-server is an operator problem; the router is responsible for
    /// any fallback to Tier B.
    #[tokio::test]
    async fn http_5xx_surfaces_as_upstream_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let err = client
            .complete(&req())
            .await
            .expect_err("500 must surface as error");
        assert!(
            matches!(err, DoormanError::Upstream(_)),
            "expected DoormanError::Upstream for 5xx, got {err:?}"
        );
    }

    /// Empty `choices` array — the server returned 200 but with no
    /// candidates. Must surface `DoormanError::UpstreamShape` naming
    /// the empty-choices case, with no content extracted.
    #[tokio::test]
    async fn empty_choices_surfaces_upstream_shape() {
        let server = MockServer::start().await;
        let empty_body = serde_json::json!({ "choices": [] });
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(empty_body))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let err = client
            .complete(&req())
            .await
            .expect_err("empty choices must surface as error");
        match err {
            DoormanError::UpstreamShape(msg) => {
                assert!(
                    msg.contains("no choices"),
                    "UpstreamShape message should mention empty choices, got: {msg:?}"
                );
            }
            other => panic!("expected DoormanError::UpstreamShape, got {other:?}"),
        }
    }

    /// Malformed JSON response body — server returns 200 with an
    /// invalid JSON body. The `resp.json().await?` call returns a
    /// `reqwest::Error` which maps to `DoormanError::Upstream`.
    #[tokio::test]
    async fn malformed_json_body_surfaces_upstream_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(b"not json {".to_vec(), "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let err = client
            .complete(&req())
            .await
            .expect_err("malformed JSON must surface as error");
        assert!(
            matches!(err, DoormanError::Upstream(_)),
            "expected DoormanError::Upstream for JSON parse failure, got {err:?}"
        );
    }

    // ── Grammar serialisation tests ────────────────────────────────────────

    /// When `grammar` is `None` the upstream body must contain neither
    /// `"grammar"` nor `"json_schema"` keys. Absence is verified both by
    /// parsing the captured body and by checking that the mock received
    /// exactly one request (sanity).
    #[tokio::test]
    async fn grammar_none_omits_all_grammar_fields() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let mut r = req();
        r.grammar = None;
        client.complete(&r).await.expect("none grammar happy path");

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests.len(), 1, "expected exactly one upstream request");
        let body: serde_json::Value =
            serde_json::from_slice(&requests[0].body).expect("request body must be valid JSON");
        assert!(
            body.get("grammar").is_none(),
            "body must not contain 'grammar' key when grammar is None"
        );
        assert!(
            body.get("json_schema").is_none(),
            "body must not contain 'json_schema' key when grammar is None"
        );
    }

    /// When `grammar` is `Some(Gbnf(...))` the upstream body must contain
    /// `"grammar": "<gbnf string>"` at the top level (NOT inside
    /// `extra_body`), and must NOT contain `"json_schema"`.
    #[tokio::test]
    async fn grammar_gbnf_serialises_into_top_level_grammar_field() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let mut r = req();
        let gbnf = r#"root ::= "yes" | "no""#;
        r.grammar = Some(GrammarConstraint::Gbnf(gbnf.to_string()));
        client.complete(&r).await.expect("gbnf grammar happy path");

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests.len(), 1);
        let body: serde_json::Value =
            serde_json::from_slice(&requests[0].body).expect("request body must be valid JSON");
        assert_eq!(
            body.get("grammar").and_then(|v| v.as_str()),
            Some(gbnf),
            "body must contain top-level 'grammar' with the GBNF string"
        );
        assert!(
            body.get("json_schema").is_none(),
            "body must not contain 'json_schema' when grammar is Gbnf"
        );
        // Must NOT be nested inside extra_body (llama-server native field)
        assert!(
            body.get("extra_body").is_none(),
            "Tier A must NOT use extra_body; grammar goes at top level"
        );
    }

    /// When `grammar` is `Some(JsonSchema(...))` the upstream body must
    /// contain `"json_schema": <value>` at the top level and must NOT
    /// contain a `"grammar"` key.
    #[tokio::test]
    async fn grammar_json_schema_serialises_into_top_level_json_schema_field() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = client(server.uri());
        let mut r = req();
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "answer": {"type": "string"}
            },
            "required": ["answer"]
        });
        r.grammar = Some(GrammarConstraint::JsonSchema(schema.clone()));
        client
            .complete(&r)
            .await
            .expect("json_schema grammar happy path");

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests.len(), 1);
        let body: serde_json::Value =
            serde_json::from_slice(&requests[0].body).expect("request body must be valid JSON");
        assert_eq!(
            body.get("json_schema"),
            Some(&schema),
            "body must contain top-level 'json_schema' with the schema value"
        );
        assert!(
            body.get("grammar").is_none(),
            "body must not contain 'grammar' key when grammar is JsonSchema"
        );
        assert!(
            body.get("extra_body").is_none(),
            "Tier A must NOT use extra_body; json_schema goes at top level"
        );
    }

    /// When `grammar` is `Some(Lark(...))` the call must return a typed
    /// `DoormanError::TierAGrammarUnsupported` error BEFORE making any
    /// network call. The wiremock server must have received zero requests.
    #[tokio::test]
    async fn grammar_lark_rejected_before_any_network_call() {
        let server = MockServer::start().await;
        // No mock registered — any request reaching the server would be
        // an unexpected call and cause the test to fail at server drop.

        let client = client(server.uri());
        let mut r = req();
        r.grammar = Some(GrammarConstraint::Lark("start: /[a-z]+/".to_string()));
        let err = client
            .complete(&r)
            .await
            .expect_err("Lark grammar must be rejected");

        assert!(
            matches!(
                err,
                DoormanError::TierAGrammarUnsupported {
                    dialect: "Lark",
                    ..
                }
            ),
            "expected TierAGrammarUnsupported with dialect=Lark, got {err:?}"
        );

        // Critical: no upstream call must have been made.
        let received = server.received_requests().await.unwrap();
        assert!(
            received.is_empty(),
            "Lark rejection must happen before any network call; \
             wiremock received {} request(s)",
            received.len()
        );
    }

    // ── P2 (training/proof-run) admission tests ────────────────────────────

    /// `complete_p2` must fail closed when no scheduler is attached — "not
    /// configured" must never be silently read as "always idle."
    #[tokio::test]
    async fn complete_p2_refuses_when_scheduler_unconfigured() {
        let server = MockServer::start().await;
        // No mock registered — a network call here would be the bug.
        let client = client(server.uri());
        let err = client
            .complete_p2(&req())
            .await
            .expect_err("must refuse without a scheduler");
        assert!(matches!(err, DoormanError::LocalSaturated));
        assert!(server.received_requests().await.unwrap().is_empty());
    }

    /// `complete_p2` admits when the shared scheduler is genuinely idle, and
    /// is rejected while a P0 (interactive) call holds the same slot —
    /// proving idle-detection is real, not a separate/independent counter.
    #[tokio::test]
    async fn complete_p2_admits_when_idle_rejects_when_interactive_in_flight() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .mount(&server)
            .await;

        let scheduler = Arc::new(RequestScheduler::new(1));
        let background = Arc::new(Semaphore::new(1));
        let client = client(server.uri()).with_semaphores(scheduler.clone(), background);

        // Idle: P2 admitted.
        client
            .complete_p2(&req())
            .await
            .expect("idle backend must admit P2");

        // Occupy the one shared slot directly on the scheduler (simulating an
        // in-flight P0 interactive call without needing a slow real request).
        let _held = scheduler
            .try_admit(InfightPriority::P0)
            .expect("first slot");
        let err = client
            .complete_p2(&req())
            .await
            .expect_err("P2 must be rejected while any slot is held");
        assert!(matches!(err, DoormanError::LocalSaturated));
    }
}
