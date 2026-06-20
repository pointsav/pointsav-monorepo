---
artifact: brief
schema: foundry-brief-v1
brief-id: project-data-slm-tier-split-architecture
title: "SLM Tier Split: Tier 0 Router on os-totebox + Tier A Aggregator on os-orchestration"
status: active
owner: project-data
created: 2026-06-20
updated: 2026-06-20
parent: BRIEF-os-totebox-build-out
---

## §1 — Context

The current SLM tier model has a single Doorman (service-slm) on the workspace VM
handling all tiers locally:

```
Tier A: local llama-server (OLMo 2 1B) on workspace VM
Tier B: Yo-Yo fleet (brokered by app-orchestration-slm on os-orchestration VM)
Tier C: Anthropic API (circuit fallback)
```

Phase 2 of app-orchestration-slm (committed 2026-06-19, batch fb24a853 + 4e6e5cf6,
Stage 6 pending) added:
- `MembershipKey` — Ed25519 keypair; `issue()` generates 1-hour per-archive tokens
- `POST /v1/graph/federated` — fans out to all registered fleet Doormen; aggregates DataGraph results
- `POST /v1/training/schedule` — federation of training jobs across fleet
- `GET /v1/adapters` — adapters registry; advertises available adapter endpoints
- `FleetRegistry.list_full()` — full member list with `doorman_endpoint` for fanout

These primitives enable a new tier topology that was not previously possible.

## §2 — The proposed architecture

```
os-totebox VM (one per deployment):
  service-slm (Doorman, Tier 0 mode)
    ├── DataGraph: SOVEREIGN (service-content, local only)
    ├── Training data: SOVEREIGN (accumulated locally)
    └── Inference: ROUTED → os-orchestration (Tier A)

os-orchestration VM (shared, one per region/cluster):
  app-orchestration-slm (Tier A aggregator)
    ├── llama-server + model (local; e.g. OLMo 2 7B or Llama 3.3 70B)
    ├── Yo-Yo fleet proxy (Tier B escalation)
    └── Anthropic API fallback (Tier C escalation)
```

### What "Tier 0" means

Tier 0 is a new Doorman operating mode: pure router with sovereign data, no local
inference backend. In Tier 0 mode:

- The Doorman does NOT start or require llama-server on `:8080`
- Doorman responds to DataGraph queries from its local service-content (sovereign)
- For inference requests: authenticates via membership token and POSTs to
  `SLM_ORCHESTRATION_ENDPOINT` (os-orchestration app-orchestration-slm)
- Circuit breaker still applies: if os-orchestration is unavailable, escalates
  to Tier B (Yo-Yo) then Tier C (Anthropic), same as today
- `doorman_health()` reports: `tier_a: "orchestration" (remote)` not `"local"`

### Why sovereign DataGraph matters

Each os-totebox instance operates a different archive (project-gis, project-editorial,
etc.). Their DataGraphs are archive-specific — entity graphs for their own domain.
These are NOT federated upward automatically. The federated graph endpoint on
os-orchestration is operator-invoked only (not automatic background sync).

Training data is also sovereign: each os-totebox accumulates its own corpus from
service-extraction. When ready, it posts a training schedule to os-orchestration
(`POST /v1/training/schedule`) which coordinates the distillation run against the
Yo-Yo trainer node.

## §3 — Research from Phase 2 (commit fb24a853 + 4e6e5cf6)

### MembershipKey as the auth bridge

`membership.rs` in `orchestration-slm/src/` implements:
```rust
pub struct MembershipKey { signing_key: SigningKey, verifying_key: VerifyingKey }
impl MembershipKey {
    pub fn generate() -> io::Result<Self>   // reads /dev/urandom, no rand_core dep
    pub fn issue(&self, module_id: &str, archive_id: &str) -> String  // 1-hour token
    pub fn verify(&self, token: &str) -> Result<MembershipClaims, ChassisError>
}
```
Token format: `<base64url(payload_json)>.<base64url(ed25519_sig)>` where payload
carries `module_id`, `archive_id`, `issued_at`, `expires_at` (unix seconds).

This token is what a Tier 0 Doorman would present in `Authorization: Bearer <token>`
when forwarding an inference request to os-orchestration.

### RegistrationResponseV2

The discovery registration response now returns `membership_token: Option<String>` —
a token issued by os-orchestration to the registering Doorman at fleet join time.
The Tier 0 Doorman would use this token for all subsequent inference calls.

### Federated graph as pull-on-demand

`POST /v1/graph/federated` on os-orchestration calls `state.fleet.list_full().await`
to get all registered members with their `doorman_endpoint`, then fans out a query
to each member's DataGraph. The Tier 0 Doorman's service-content DataGraph would
be one such endpoint. This preserves sovereignty: data stays on os-totebox; queries
come from os-orchestration only when explicitly invoked.

### Training schedule coordination

`POST /v1/training/schedule` on os-orchestration accepts a `TrainingScheduleRequest`
with `archive_id` and training parameters. This is how a Tier 0 Doorman would signal
readiness for a distillation run without needing to talk to the Yo-Yo trainer directly.

## §4 — What needs to be built

### Tier 0 Doorman mode (service-slm)

New env var: `SLM_TIER=0` (default unset = existing behavior, Tier A local).

When `SLM_TIER=0`:
1. Skip llama-server health check on startup (currently `local_doorman.service`
   waits for `:8080` to be live — this step is removed)
2. Read `SLM_ORCHESTRATION_ENDPOINT` (required; no default)
3. On registration with os-orchestration: present `SLM_MODULE_ID` + archive ID;
   store returned `membership_token` in memory
4. For inference requests: forward to `SLM_ORCHESTRATION_ENDPOINT/v1/inference`
   with `Authorization: Bearer <membership_token>`; handle 401 (re-register) and
   503 (escalate to Tier B)
5. `doorman_health()` tier_a field reports: `{mode: "orchestration", endpoint: "...",
   latency_ms: N}` instead of `{mode: "local", model: "..."}`

### Inference proxy endpoint (app-orchestration-slm)

New endpoint: `POST /v1/inference`

- Validates `Authorization: Bearer <token>` via `state.membership.verify()`
- Accepts an inference request payload (prompt + parameters)
- Routes to local llama-server (Tier A) or Yo-Yo fleet (Tier B) per existing logic
- Returns the inference response
- Rate limits per `module_id` (extracted from token claims)

New type in `orchestration-slm-core`:
```rust
pub struct InferenceRequest { pub prompt: String, pub max_tokens: u32, pub temperature: f32 }
pub struct InferenceResponse { pub text: String, pub model: String, pub tokens_used: u32 }
```

### os-orchestration system spec (moonshot-toolkit)

Analogous to `moonshot-toolkit/examples/os-totebox.toml` — a Microkit system spec
for the os-orchestration VM topology. PDs:
- `orchestration-pd` (priority 200): runs app-orchestration-slm broker logic
- `yoyo-proxy-pd` (priority 180): Yo-Yo fleet proxy
- `training-scheduler-pd` (priority 160): training schedule coordination

This is Phase H2 work (after Phase H1 QEMU boot passes). See BRIEF-os-orchestration-build-out.

## §5 — Deployment sequence (phased)

### Naming

| Role | Deployment catalog name | GCP VM name | Notes |
|---|---|---|---|
| First os-totebox cluster | `cluster-totebox-data-1` | `totebox-data-1` | Hosts project-data archive |
| Interim os-orchestration | `gateway-orchestration-command-1` | `foundry-workspace` (existing) | foundry-workspace acting as gateway |
| Future dedicated os-orchestration | `gateway-orchestration-command-2` | `orchestration-command-1` | Phase 3; sized for larger model |

The Totebox Archive on `cluster-totebox-data-1` keeps the archive name `project-data` —
same project, different host VM. foundry-workspace's role alias
`gateway-orchestration-command-1` is a catalog entry only; the physical VM does not change.

### Phase 1 — Provision cluster-totebox-data-1 (NO new code required)

**VM spec (minimum viable):**
- Type: `e2-small` (2 vCPU, 2GB RAM) — ~$13/month
- Disk: 50GB SSD boot disk (~$4.25/month) — cargo target dir fills fast
- Region: same zone as foundry-workspace (low-latency internal routing)
- OS: Debian 12
- External IP: yes (Claude Code needs Anthropic API egress)
- Total: ~$17/month

2GB RAM is sufficient for Tier 0 (no llama-server). Resize to e2-medium (4GB,
~$30/month) if service-content DataGraph grows or cargo builds are too slow.

**foundry-workspace config change (Command Session, one-time):**
Rebind llama-server from `127.0.0.1:8080` to internal GCP IP (or `0.0.0.0:8080`):
```
# /etc/systemd/system/local-slm.service override:
# Add --host <internal-ip> to the llama-server ExecStart flags
# Open GCP firewall: allow tcp:8080 from totebox-data-1 internal IP only
sudo systemctl edit local-slm.service
sudo systemctl restart local-slm.service
```
This is the ONLY foundry-workspace change needed for Phase 1. No code changes anywhere.

**cluster-totebox-data-1 service stack:**
```
SLM_LOCAL_ENDPOINT=http://<foundry-workspace-internal-ip>:8080   # routes inference to fw llama-server
SLM_BIND_ADDR=0.0.0.0:9080                                        # expose Doorman on internal network
SERVICE_CONTENT_GRAPH_BACKEND=sqlite                               # start lightweight; migrate later
```
The os-totebox-1 Doorman treats foundry-workspace's llama-server as its "Tier A" —
no Tier 0 mode code needed. foundry-workspace handles concurrent requests (llama-server
queues them). DataGraph on os-totebox-1 (service-content) starts empty and populates
as service-extraction processes documents — sovereign from day one.

**project-data archive migration:**
- project-data Totebox Session MOVES to cluster-totebox-data-1
- Current foundry-workspace project-data session closes before cluster VM takes over
- One session per git repo — no concurrent sessions on the same archive

**6-service deploy order on cluster-totebox-data-1:**
```
1. service-fs        (port 9100 — must be first; others depend on it)
2. service-content   (port 9081 — DataGraph; start empty, sqlite backend)
3. service-people    (port 9091)
4. service-input     (port 9106 — required by start-stack.sh; no port server)
5. service-extraction (no HTTP port; fs watcher; depends on service-fs + service-content)
6. service-slm       (port 9080 — Doorman last; all dependencies must be up)
```

### Phase 2 — Validate

With cluster-totebox-data-1 live:
- Confirm DataGraph sovereignty: cluster-totebox-data-1 service-content is independent
- Confirm inference routing: `doorman_health()` shows Tier A alive (foundry-workspace:8080)
- Confirm circuit breaker: if foundry-workspace llama-server stops, Doorman escalates to Tier C
- Confirm federated graph: operator-invoked `POST /v1/graph/federated` on foundry-workspace
  returns entities from its own DataGraph; cluster-totebox-data-1 DataGraph starts empty
  (will grow as extraction runs)

### Phase 3 — Implement proper Tier 0 mode + dedicated os-orchestration VM

After Phase 2 validates the routing concept:
1. Implement `SLM_TIER=0` mode in service-slm Doorman (new code — see §4)
2. Implement `POST /v1/inference` on app-orchestration-slm (new code — see §4)
3. Provision `gateway-orchestration-command-2` GCP VM (dedicated; sized for larger model)
4. Deploy app-orchestration-slm + llama-server on it
5. Switch all os-totebox VMs to proper Tier 0 mode (membership token auth)
6. foundry-workspace Doorman switches to Tier 0 (joins the fleet as a peer)
7. foundry-workspace's interim gateway role (`gateway-orchestration-command-1`) is retired

Target topology: one dedicated Command os-orchestration; N peer Totebox Archives
(cluster-totebox-data-1, foundry-workspace, future VMs) all running proper Tier 0.

## §6 — Decisions locked

1. **DataGraph is sovereign.** os-totebox's service-content DataGraph does NOT
   automatically sync to os-orchestration. Federated queries are pull-on-demand,
   operator-invoked only.
2. **Training data is sovereign.** Corpus accumulates on os-totebox. Only the
   training schedule (metadata) goes to os-orchestration for coordination.
3. **Membership tokens are the auth layer.** No separate API key scheme; the
   Ed25519 MembershipKey issued at fleet registration covers all subsequent calls.
4. **No new Yo-Yo VMs.** Existing fleet (trainer + graph + proxy nodes) is
   unchanged by this split. os-orchestration becomes their new front-end for
   Tier 0 clients.
5. **os-orchestration has NO DataGraph and NO LoRA of its own.** It is a
   gateway, not a data store. Rationale:
   - DataGraph: `POST /v1/graph/federated` already fans out to Totebox DataGraphs
     in real time. A meta-graph at os-orchestration creates a stale-cache/sync
     problem with no benefit over real-time federation.
   - LoRA: each Totebox trains a domain-specific adapter (project-gis ≠ project-editorial).
     os-orchestration selects the right adapter from `GET /v1/adapters` via `module_id`.
     A cross-archive LoRA would be a semantic mush — it defeats domain tuning.
   - Stateless rule: existing app-orchestration-slm CLAUDE.md decision stands.
     "No persistent data" means no sync problem, no corruption risk, clean restarts.
   - Summary: os-orchestration provides base model + adapter routing + circuit
     brokering. All knowledge stays in the Totebox that generated it.

## §7 — Decisions open

1. **Which model does os-orchestration run locally (Tier A)?** Current workspace
   Tier A is OLMo 2 1B (fast, small). os-orchestration could run a larger model
   (e.g. OLMo 2 7B or Llama 3.3 70B) given it's a dedicated VM. Blocked on GCP
   VM sizing decision.
2. **Tier 0 circuit breaker fallback when os-orchestration is unreachable.** Options:
   a. Escalate to Tier B (Yo-Yo direct) — requires Tier 0 Doorman to hold Yo-Yo
      credentials, which complicates the "thin router" premise.
   b. Escalate to Tier C (Anthropic) only — simpler but more expensive.
   c. Queue locally and retry — acceptable for batch tasks, not interactive.
3. **Registration on startup vs. lazy.** Does the Tier 0 Doorman register with
   os-orchestration at `service-slm` startup, or on first inference request?
   Startup registration means early failure detection; lazy means the service starts
   even if os-orchestration is down.
4. **DataGraph pull scope for federated queries.** When os-orchestration calls
   `POST /v1/graph/federated`, what entities does a Tier 0 Doorman expose? Full
   archive DataGraph? A curated projection? Needs a DataGraph ACL design.
5. **os-totebox deployment identity.** Each os-totebox VM would have a unique
   `FOUNDRY_ARCHIVE_NAME` + `SLM_MODULE_ID`. The membership token embeds both.
   Does each VM get a static provisioned ID, or is there a discovery/allocation
   protocol?

## §8 — Work log

### Session 12 — 2026-06-20

- BRIEF created from operator direction during os-totebox GCP deployment assessment
- Phase 2 research (commits fb24a853 + 4e6e5cf6) incorporated as §3
- Tier 0 Doorman mode spec written (§4)
- Inference proxy endpoint spec written (§4)
- Phased deployment sequence written (§5) — foundry-workspace as interim os-orchestration
- Decision locked: os-orchestration has NO DataGraph and NO LoRA of its own (§6.5)
  Rationale: gateway not store; federated graph covers cross-Totebox queries on-demand;
  stateless rule preserved; LoRA stays domain-specific per Totebox

## §9 — Carry-forward

- [ ] Stage 6 promote (Command Session) — prerequisite for all downstream work
- [ ] Design `InferenceRequest` / `InferenceResponse` types in orchestration-slm-core
- [ ] Implement `SLM_TIER=0` mode in service-slm Doorman (env var `SLM_TIER=0`)
- [ ] Implement `POST /v1/inference` on app-orchestration-slm
- [ ] Resolve open decisions §7.1 (VM sizing) and §7.2 (circuit breaker fallback)
- [ ] Provision os-totebox-1 GCP VM (Phase 1 of §5 sequence)
- [ ] Write os-orchestration.toml Microkit system spec (Phase H2 dependency)
