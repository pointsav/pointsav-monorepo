# DEVELOPMENT.md — service-slm

**Scope.** Build commands, CI policy, licence enforcement,
migration roadmap, and blocking items. The architectural *shape*
of the system is in `ARCHITECTURE.md`; strategic reasoning for the
choices here lives in
`content-wiki-documentation/topic-service-slm.md`.

---

## 1. Build and test

### Phase 1 — current

The Rust workspace in `ARCHITECTURE.md` §6 is not yet scaffolded.
Only the nested `cognitive-forge/` subcrate exists. Build it in
isolation:

```
cargo build --manifest-path cognitive-forge/Cargo.toml
cargo test  --manifest-path cognitive-forge/Cargo.toml
```

End-to-end execution requires:

- a running SLM endpoint on `http://127.0.0.1:8080/v1/chat/completions`
- a Totebox root directory containing `service-slm/transient-queues/`

Neither has been run end-to-end from this clone.

### Phase 2+ — target

Once the workspace `Cargo.toml` lands (`NEXT.md` Queue):

```
cargo build  --workspace --release
cargo test   --workspace
cargo clippy --workspace -- -D warnings
cargo fmt    --all -- --check
```

Binary output: `target/release/slm-cli`. Single binary, no
additional runtime dependencies beyond the host's TLS roots.

### Cross-compilation

Appliance targets (see `ARCHITECTURE.md` §9):

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

### 2.1 `deny.toml` skeleton

Lives at project root once the workspace exists. CI runs
`cargo deny check licenses` on every commit; build fails on any
new transitive dep with a disallowed licence.

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
// SPDX-License-Identifier: Apache-2.0
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

## 4. Migration roadmap

Four phases. Each phase is independently valuable; the project is
not required to finish the next one.

### Phase 1 — Python trial (current)

- Python, vLLM (multi-LoRA serving primitive per
  `~/Foundry/conventions/adapter-composition.md`), OpenTofu, `dbt`,
  Dagster per the trial spec (no SkyPilot — OpenTofu is the
  provisioning surface per
  `~/Foundry/conventions/zero-container-runtime.md`)
- Goal: validate the architecture, not the language choice
- A Rust migration during Phase 1 would add risk without
  validating anything

### Phase 2 — Rust rewrite (after trial passes)

Order of work:

1. Fresh cargo workspace per `ARCHITECTURE.md` §6
2. Port the doorman protocol (`crates/slm-doorman`) — sanitise /
   send / receive / rehydrate
3. `service-content` is out of scope for this migration; it is a
   different service
4. Port the ledger (`crates/slm-ledger`)
5. Port the GCE compute driver (`crates/slm-compute`,
   `crates/slm-inference-remote`) per `infrastructure/slm-yoyo/tofu/`
6. Replace vLLM with `mistral.rs` on the yo-yo node
   (`crates/slm-inference-local` for local-host paths;
   remote-side native binary delivered via the
   `pointsav-public` GCE image)

**Success criterion:** the Rust `service-slm` passes the same
Phase-1 test suite as the Python version. Parity before cutover.

### Phase 3 — os-totebox integration

- Cross-compile for Totebox targets (x86_64, aarch64)
- Integrate with os-totebox init / systemd
- Sign releases with Sigstore per §2.2
- Ship as appliance component

### Phase 4 — Optional open-source release

- Apply the licence / header / DCO checks in §2, §3
- Publish to GitHub under the `pointsav` org
- Write a launch post

No timeline commitment on Phase 4. The option stays open.

---

## 5. Blockers before Phase 2 build-out

These do not block the Phase-1 trial — they block Ring 2 / Ring 3b
expansion in Phase 2 and beyond. Each must resolve before the
feature it gates can be scaffolded.

| # | Blocker | Gates | Status |
|---|---|---|---|
| B1 | Mooncake + LMCache licence audit at adoption time | Ring 2 scaffolding in `memory/kv/` | Open — pending operator review |
| B2 | Mooncake master hosting decision (small GCE VM / Totebox co-host / OpenTofu module with `idle_shutdown_minutes=N` per `infrastructure/slm-yoyo/tofu/`) | Ring 2 deployment | Open — working recommendation: small GCE VM for Phase 2; revisit once Totebox stabilises |
| B3 | Adapter training hardware allocation (A100 40 GB, ~4 hrs per adapter, ~$30 via Batch API) | Ring 3b first training run | Open |
| B4 | Adapter evaluation protocol defined | Ring 3b registry populates | Open — depends on a separate operator decision on archetype promotion thresholds |
| B5 | Secret Manager key-management migration (Phase 1 uses SSH env vars) | Phase 2 operational hardening | Open |
| B6 | system-slm connection protocol decision (OpenAI-compatible HTTP vs local CLI binary) | Phase-1 bridge goes live | Open — see `NEXT.md` Blocked |

Each blocker that moves to "Resolved" migrates to `CHANGELOG.md`
or `NEXT.md` Recently done — not kept here.

---

## 6. Build-time risks that affect CI

Operational risks, not strategic. Strategic risk analysis lives
in `content-wiki-documentation/topic-service-slm.md`.

| Risk | Mitigation |
|---|---|
| **`cargo deny` flags unexpected transitive licences.** New transitive deps with AGPL / GPL / BSL enter the tree through upstream updates. | Run `cargo deny` in CI from the first commit of the workspace. Fix licence drift at the merge that introduced it; do not defer to release. |
| **Rust build times long relative to Python dev loop.** CUDA-adjacent crates (`mistralrs`) compile slowly. | `sccache` for compiler cache; separate the inference crate from the doorman crate so doorman rebuilds do not rebuild CUDA kernels. |
| **`mistral.rs` maintenance concentration.** Small-team upstream. | `candle` (Hugging Face, larger team) sits underneath and is the fallback. Pin `mistralrs` to a known-good commit if the maintainer disengages; carry patches. |
| **LadybugDB is a fork of post-acquisition Kuzu.** Maintenance signal unclear. | MIT-licensed; worst case is carrying patches. Monitor for six months; contribute upstream fixes to build relationship. |

Risks that are strategic rather than operational (hiring-pool
width, research parity with Python ecosystem) belong in the wiki
topic doc, not here.

---

## 7. Workspace dependencies (appendix)

Indicative minimums as of April 2026. Pin to latest compatible at
the start of Phase 2. All entries MIT or Apache-2.0.

```toml
[workspace.dependencies]
# HTTP / async
axum = "0.8"
tower = "0.5"
tokio = { version = "1.40", features = ["full"] }
hyper = "1.5"
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }

# Storage
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio-rustls"] }
object_store = { version = "0.11", features = ["gcp", "aws"] }

# LadybugDB bindings
kuzu = "0.11"        # or lbug crate if migrating

# Inference
mistralrs = { version = "0.8", features = ["cuda", "flash-attn"] }
candle-core = "0.9"
candle-nn = "0.9"

# Documents
oxidize-pdf = "2.5"
docx-rust = "0.3"
pulldown-cmark = "0.12"
calamine = "0.26"

# Orchestration
apalis = { version = "0.7", features = ["limit"] }
apalis-sqlite = "0.7"
apalis-workflow = "0.1"
backoff = "0.4"

# Networking
russh = "0.46"
rustls = "0.23"
google-cloud-storage = "0.23"
google-cloud-compute = "*"

# Serde + validation
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
validator = { version = "0.19", features = ["derive"] }
schemars = "0.8"

# Observability
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
opentelemetry = "0.27"

# Signing
sigstore = "0.10"

# Errors
anyhow = "1"
thiserror = "2"
```

---

## 8. Cross-references

- `CLAUDE.md` — state header, hard constraints
- `NEXT.md` — queue, blocked items, deferred
- `ARCHITECTURE.md` — workspace shape, three-ring model, stack by role
- Workspace `CLAUDE.md` — identity store, commit flow, ADR hard rules
- `content-wiki-documentation/topic-service-slm.md` — strategic
  rationale, open-source posture *(destination not yet committed)*
