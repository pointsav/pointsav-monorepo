---
schema: foundry-doc-v1
document_version: 1.1.0
research_provenance: tacit
research_inline: false
cites: []
---

# DEVELOPMENT.md — service-slm

**Scope.** Build commands, CI policy, licence enforcement, current
shipped state, phase roadmap, and blocking items. The architectural
*shape* of the system is in `ARCHITECTURE.md`; strategic reasoning for
the choices here lives in
`content-wiki-documentation/topic-service-slm.md`.

---

## 1. Build and test

The `service-slm/` directory is a standalone Rust cargo workspace
(resolved 2026-04-25) with three workspace members:
`slm-core`, `slm-doorman`, `slm-doorman-server`.

### 1.1 Standard commands

Run all of the following from inside `service-slm/`:

```
cargo build  --workspace                  # debug build (~30–40s cold)
cargo build  --workspace --release        # release build (opt-level 3, LTO thin)
cargo test   --workspace                  # 157 tests; all pass
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt    --all -- --check
```

Current test distribution (as of 2026-05-04):

| Suite | Count | Notes |
|---|---|---|
| `slm_core` unit tests | 14 | Serde round-trips for `ComputeRequest`, `GrammarConstraint` variants, mesh types |
| `slm_doorman` unit tests | 92 | Tier clients (local, yoyo, external), ledger, audit_proxy, grammar_validation, apprenticeship, verdict, brief queue, citations, redact |
| `slm_doorman_server/tests/audit_endpoints_integration.rs` | 4 | Entry-type discriminator verification for all four ledger entry kinds |
| `slm_doorman_server/tests/queue_tests` | 5 | Brief queue §7C: enqueue/dequeue round-trip, lease expiry, concurrent workers, poison bucket, drain-after-restart |
| `slm_doorman_server/tests/http_test.rs` | 42 | Axum integration tests: smoke (4), error-mapping (5), apprenticeship-disabled 404 (3), audit_proxy (14), audit_capture (14), shadow 202 (2) |
| **Total** | **157** | All passing |

### 1.2 End-to-end against a live Tier A endpoint

The `local-doorman.service` + `local-slm.service` systemd units on
the workspace VM (`foundry-workspace`) provide a live Tier A
environment (B5 verified 2026-04-26):

```bash
# Start the Doorman manually (if not using systemd):
SLM_LOCAL_ENDPOINT=http://127.0.0.1:8080 \
SLM_BIND_ADDR=127.0.0.1:9080 \
SLM_AUDIT_DIR=/var/lib/slm-doorman/audit \
    cargo run -p slm-doorman-server
```

`SLM_YOYO_ENDPOINT` is intentionally unset by default — community-tier
mode (Tier A only). Setting it activates Tier B. The `local-doorman.env`
output from `infrastructure/slm-yoyo/tofu/` provides the correct Yo-Yo
config block.

### 1.3 Cross-compilation

Appliance targets (see `ARCHITECTURE.md` §12):

```
cargo build --target aarch64-unknown-linux-gnu --release
cargo build --target x86_64-unknown-linux-gnu  --release
```

---

## 2. Licence policy

Every direct and transitive dependency must be one of:
MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Unicode-DFS,
MPL-2.0 (file-level), or Zlib.

Forbidden:

- **AGPL-3.0** — network copyleft; modifying and running as a
  service would require open-sourcing the modifications
- **GPL-2.0 / GPL-3.0** — strong copyleft; taints the binary
- **LGPL-3.0** — weak copyleft; only safe for dynamic linking,
  which Rust makes painful
- **BSL / Business Source License** — time-delayed open source,
  uncertain commercial terms
- Custom "community" licences (Llama Community, Gemma Terms of
  Use for weights — model-weight licences are a separate question,
  distinct from code)
- **CC-BY-NC** — non-commercial, incompatible with commercial DKA

### 2.1 `deny.toml`

Lives at `service-slm/deny.toml`. CI runs `cargo deny check
licenses` on every commit; build fails on any new transitive dep
with a disallowed licence.

```toml
[licenses]
confidence-threshold = 0.93
allow = [
    "MIT",
    "Apache-2.0",
    "Apache-2.0 WITH LLVM-exception",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-DFS-2016",
    "MPL-2.0",
    "Zlib",
]

[bans]
multiple-versions = "warn"

[advisories]
vulnerability = "deny"
unmaintained = "warn"
yanked = "deny"
```

### 2.2 CI invocations

Run on every commit:

```
cargo audit                        # CVE scan via RustSec advisory DB
cargo deny check licenses advisories bans sources
cargo sbom > sbom/service-slm.spdx.json
```

Release builds additionally sign the binary via the `sigstore`
crate invoked from release automation in `xtask/`, on top of the
SSH commit + tag signing that workspace `CLAUDE.md` §3 mandates
for every Foundry commit. No container images are produced —
distribution is the GCE custom image plus `.deb` per
`~/Foundry/conventions/zero-container-runtime.md`. Verification
uses the same `sigstore` crate at runtime for adapter signatures
(Ring 3b, GCS-stored adapters).

---

## 3. Source licence headers

Every `.rs` file carries an SPDX identifier in its first comment:

```rust
// SPDX-License-Identifier: Apache-2.0 OR MIT
```

The project follows the [REUSE Specification](https://reuse.software/)
for machine-verifiable licence metadata. Files without an SPDX
identifier fail the REUSE check in CI.

### Dual-licensing

Match the Rust community norm: code authored for this project is
licensed under **Apache-2.0 OR MIT at the caller's choice**.
Apache-2.0's explicit patent grant is valuable in institutional
markets; MIT compatibility keeps the crate reusable in
MIT-licensed downstream workspaces.

### Contributing

DCO sign-off (`git commit -s`), not a CLA. Contributors retain
copyright and agree to license under the project licence by the
act of sign-off. This matches LadybugDB's pattern. CLAs depress
contribution and add legal overhead the project does not need.

---

## 4. Shipped state and remaining gates

The Doorman is in production service on the workspace VM. The
B1–B7 task list from the original scaffolding brief is summarised
here with current disposition.

### Landed

| Item | Description | Date |
|---|---|---|
| B1 | Doorman scaffold — three-crate workspace | 2026-04-25 |
| B2 | Tier B (Yo-Yo) HTTP client, bearer-token auth, retry policy, mock tests | 2026-04-26 |
| B3 | `local-slm.service` systemd unit on workspace VM | Master workspace v0.0.11 |
| B4 | Tier C (external API) client, compile-time allowlist, pricing, mock tests | 2026-04-26 |
| B5 | End-to-end Tier A verification against live llama-server | 2026-04-26 |
| B6 | `cognitive-bridge.sh` → `scripts/`; Doorman HTTP wiring | 2026-04-26 |
| PS.3 | Grammar substrate: `GrammarConstraint` type, per-tier serialisation, `LarkValidator` (llguidance), Tier A/C rejection | 2026-04-28 (iters 1–4) |
| PS.4 | Audit substrate: `/v1/audit/proxy` + `/v1/audit/capture`; `entry_type` discriminator (contract v0.2.0); caps; concurrency | 2026-04-28 (iters 5–9) |
| PS.6 | Coverage: http.rs test factory + integration tests; tier/local unit tests; VerdictDispatcher Reject/DeferTierC | 2026-04-28 |
| PS.7 | Zero-container drift cleanup (ARCHITECTURE.md + DEVELOPMENT.md) | 2026-04-28 |
| AS-2..7 | Apprenticeship substrate: brief/verdict/shadow endpoints; corpus capture; verdict promotion; redact filter | 2026-04-28/29 |
| §7C | Brief queue substrate: `queue.rs`, drain worker, lease reaper, 5 tests | 2026-04-29 |
| Phase 2 | service-content Ring 2: LadybugDB graph + HTTP server (port 9081); GraphContextClient in Doorman; Ring 2→3 graph grounding | 2026-04-30 |
| Multi-Yo-Yo | `HashMap<String, YoYoTierClient>`; named nodes `"default"`, `"trainer"`, `"graph"`; `yoyo_label` on `ComputeRequest` | 2026-05-04 |
| Mesh scaffold | `slm-core/src/mesh.rs` + `slm-doorman/src/mesh.rs`; `MeshRegistry`/`DiscoveryProvider`/`DynamicRegistry`; `route_async()` stub | 2026-05-04 |

### Remaining gates

| Item | Description | Gate |
|---|---|---|
| Apprenticeship re-enable | Restart `local-doorman.service` with `SLM_APPRENTICESHIP_ENABLED=true` | Operator-presence (~5 min) |
| cmake on VM | `apt install cmake build-essential` — required for service-content `cargo build` (lbug) | Operator-presence (~2 min) |
| Phase 3 | Training threshold detection (50-tuple trigger, Sunday 02:00 UTC cron, engineering-pointsav adapter, ≥60% acceptance gate) | Operator go-ahead; also needs D4 for live Yo-Yo training |
| Mesh DiscoveryProvider | Concrete `StaticConfigProvider` or `HttpDiscoveryProvider`; wire `route_async()` to actually dispatch to `node.endpoint` | Task scope |
| Grammar injection | service-content sets `yoyo_label="graph"` + `grammar=JsonSchema(schema)` on requests; Yo-Yo #2 enforces ontological strictness | Task scope; needs D4 for Yo-Yo #2 |
| D4 | `pointsav-public` GCP project + image-build pipeline (vLLM ≥0.12, nginx TLS, CUDA, Ubuntu 24.04) | Master + operator; unblocks all Tier B real deploy |
| PS.1 | Yo-Yo deploy readiness (preemptible flag, A100 quota, image verification) | Gated on D4 |
| PS.2 | Multi-LoRA + structured-outputs verification on live Yo-Yo | Gated on D4 |
| PS.5 | Production routing on `version-bump-manifest` task type | Corpus-threshold gate (accept-rate ≥0.6 over rolling 50) |

---

## 5. Workspace dependencies (current)

The actual workspace `Cargo.toml` carries these deps. All entries
MIT or Apache-2.0.

```toml
[workspace.dependencies]
slm-core   = { path = "crates/slm-core" }
slm-doorman = { path = "crates/slm-doorman" }

# Async runtime
tokio = { version = "1.40", features = ["full"] }

# HTTP
axum    = "0.8"
reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }

# Serialisation
serde      = { version = "1", features = ["derive"] }
serde_json = "1"

# Identifiers, time
uuid   = { version = "1.10", features = ["v7", "serde"] }
chrono = { version = "0.4", default-features = false, features = ["clock", "serde"] }

# Errors
anyhow    = "1"
thiserror = "2"

# Async trait support (BearerTokenProvider in slm-doorman)
async-trait = "0.1"

# Regex (redact sanitize-outbound; apprenticeship YAML-frontmatter parsing)
regex = "1"

# File locking for apprenticeship promotion ledger (flock(2) per design-pass Q3)
fs2 = "0.4"

# Base64 for SSH signature blob in ApprenticeshipVerdict wire shape (design-pass Q5)
base64 = "0.22"

# Observability
tracing            = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
```

Additional direct dependencies on individual crates (not declared
at workspace level):

| Crate | Where used | Version | Purpose |
|---|---|---|---|
| `llguidance` | `slm-doorman` | 1.7 | Lark grammar pre-validation (PS.3 step 5) |
| `wiremock` | `slm-doorman` (dev), `slm-doorman-server` (dev) | 0.6 | Mock HTTP server for tier client + http.rs tests |
| `tower` | `slm-doorman-server` (dev) | matching axum | `TestClient` in test_helpers |

---

## 6. Build-time risks that affect CI

Operational risks, not strategic. Strategic risk analysis lives
in `content-wiki-documentation/topic-service-slm.md`.

| Risk | Mitigation |
|---|---|
| **`cargo deny` flags unexpected transitive licences.** New transitive deps with AGPL / GPL / BSL enter the tree through upstream updates. | Run `cargo deny` in CI from the first commit of the workspace. Fix licence drift at the merge that introduced it; do not defer to release. |
| **Rust build times long relative to Python dev loop.** CUDA-adjacent crates compile slowly when added to the workspace. | `sccache` for compiler cache; keep the inference crate (future) separate from the Doorman crate so Doorman rebuilds do not rebuild CUDA kernels. |
| **`llguidance` API surface.** Minor-version updates to `llguidance` may shift the `ParserFactory` / `TopLevelGrammar` API used by `grammar_validation.rs`. | Pinned to `"1.7"` in `slm-doorman/Cargo.toml`; update explicitly and re-run the Lark validation test suite. |
| **LadybugDB is a fork of post-acquisition Kuzu.** Maintenance signal unclear. | MIT-licensed; worst case is carrying patches. Monitor for six months; contribute upstream fixes to build relationship. |

---

## 7. Apprenticeship substrate — enablement

The three apprenticeship endpoints (`/v1/brief`, `/v1/verdict`,
`/v1/shadow`) are disabled by default. The `local-doorman.service`
unit on the workspace VM runs without `SLM_APPRENTICESHIP_ENABLED`,
so the current production deployment returns 404 on all three.

To enable:

```env
SLM_APPRENTICESHIP_ENABLED=true
FOUNDRY_ROOT=/srv/foundry
FOUNDRY_ALLOWED_SIGNERS=/srv/foundry/identity/allowed_signers
FOUNDRY_DOCTRINE_VERSION=0.0.7
FOUNDRY_TENANT=pointsav
SLM_BRIEF_TIER_B_THRESHOLD_CHARS=8000
```

B7 ran (Master v0.1.68, 2026-04-29) and briefly set the flag; a
subsequent service restart left the env var unset. The code is ready.
Re-enable by updating `infrastructure/local-doorman/service.d/env-file.conf`
and restarting the unit — ~5 minutes operator-presence.

---

## 8. Cross-references

- `CLAUDE.md` — state header, hard constraints
- `NEXT.md` — queue, blocked items, deferred
- `ARCHITECTURE.md` — workspace shape, three-ring model, endpoint table, tier routing policy
- `service-slm/docs/audit-endpoints-contract.md` — canonical wire contract for audit endpoints (v0.2.0)
- Workspace `CLAUDE.md` — identity store, commit flow, ADR hard rules
- `content-wiki-documentation/topic-service-slm.md` — strategic
  rationale, open-source posture *(destination not yet committed)*
- `~/Foundry/conventions/zero-container-runtime.md` — deployment model doctrine
- `~/Foundry/conventions/apprenticeship-substrate.md` — full apprenticeship spec
