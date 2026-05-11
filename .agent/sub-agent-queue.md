---
schema: foundry-sub-agent-queue-v1
owner: task-project-bim
created: 2026-04-28
last_updated: 2026-04-28T22:00:00Z
---

# Sub-agent queue — project-bim cluster

Cluster-scope sub-agent briefs dispatched under v0.1.30 §1A discipline.
Standing operator-override per `feedback_operator_override_sonnet_dispatch.md`
applies to this cluster (operator green-lit "yes" 2026-04-28 first session
start; standing for cluster scope).

Per-brief outputs land in `.claude/sub-agent-results/`. Parent Task
reviews each result before commit-or-queue-next per §1A.6.

---

## In-flight (foreground+parallel — read-only research)

| ID | Subject | Model | Status | Output file |
|---|---|---|---|---|
| BB.1 | IfcOpenShell 0.8.5 Rust subprocess invocation patterns | sonnet | in-flight | sub-agent-results/BB.1-ifcopenshell-rust-subprocess-2026-04-28.md |
| BB.2 | xeokit (AGPL-3.0) vs @thatopen (MIT) decision for app-workplace-bim | sonnet | in-flight | sub-agent-results/BB.2-xeokit-vs-thatopen-2026-04-28.md |
| BB.3 | Tauri 2.10 best practices for BIM-scale models (memory, IPC, mobile) | sonnet | in-flight | sub-agent-results/BB.3-tauri-bim-scale-2026-04-28.md |
| BB.4 | Bonsai (formerly BlenderBIM) interface conventions deep-dive | sonnet | in-flight | sub-agent-results/BB.4-bonsai-interface-deepdive-2026-04-28.md |

Dispatched 2026-04-28T21:58Z. Parallel because read-only research with
no shared filesystem write surface. 30-min soft cap each; ~600-1100
lines of markdown report each. Returns inform BB.5–BB.10 scaffolding
decisions.

## Pending (foreground+serial — sequential writing dispatches per §1A.2)

| ID | Subject | Model | Blocked-on | Notes |
|---|---|---|---|---|
| BB.5 | Scaffold service-materials Rust crate | sonnet | none (independent) | Reserved-folder → Scaffold-coded; bSDD URI references + IfcMaterial + Pset_Material* |
| BB.6 | Scaffold service-buildings Rust crate | sonnet | none (independent) | IFC GUID-keyed element store; per-element YAML sidecars; Speckle-style hash-addressed object store |
| BB.7 | Scaffold service-codes Rust crate | sonnet | BB.1 (uses ifctester invocation pattern) | The City-Code-as-Composable-Geometry invention; bSDD + IDS + IFC fragments |
| BB.8 | Scaffold app-orchestration-bim (Axum + server-rendered HTML; Building Design System showcase) | sonnet | BB.11 (token contracts), BB.12 (component recipe contracts) | Mirrors app-privategit-design pattern; port 9096 |
| BB.9 | Scaffold app-workplace-bim (Tauri 2.10) | sonnet | BB.1, BB.2, BB.3 | Pattern 1 architecture per BIM_Buildable Architecture.md |
| BB.10 | Scaffold app-console-bim | sonnet | BB.4 (UX conventions to NOT-mirror from Bonsai) | Mode-prop READ surface; BimGuidSearch + BimAuditLog + BimDashboard + BimExportPanel |

Sequential because each writes to the same parent's `.git/index` per §1A.2.
Parent commits each before dispatching the next.

## Pending (parent-direct — drafts staging without sub-agent)

| ID | Subject | Notes |
|---|---|---|
| BB.11 | Stage 8 PROSE-TOPIC drafts in drafts-outbound/ | foundry-draft-v1 + Research-trail per claim #39; project-language sweeps |
| BB.12 | Stage 8 DESIGN drafts in drafts-outbound/ | 1 RESEARCH + 7 COMPONENT (foundry-draft-v1 + component_metadata block) |

Parent-direct rather than dispatched: drafts are short-form templated
content with stable structure; sub-agent dispatch overhead exceeds the
work. Parent writes them in-line, then commits.

---

## Completed briefs

*(none yet — first batch in flight as of 2026-04-28T21:58Z.)*

---

## Notes for next Task session

- BB.1–BB.4 results are decision inputs: BB.2 (xeokit vs @thatopen) gates
  BB.9 (app-workplace-bim) embed choice. BB.4 (Bonsai) gates BB.10
  (app-console-bim) "what NOT to copy" list. BB.1 + BB.3 inform BB.9
  Tauri config + IfcOpenShell sidecar wiring.
- If a sub-agent comes back with confident, evidence-backed pushback on
  the cluster manifest's defaults (e.g., "@thatopen wins on 7 of 8
  axes"), update the manifest before scaffolding. Surface significant
  pushback to Master via outbox before re-architecting.
- Sub-agent results are LOCAL to this cluster (`.claude/sub-agent-results/`).
  Workspace-level Master research from cluster provisioning lives at
  `~/Foundry/.claude/sub-agent-results/A-bim-* B-bim-* C-bim-*`.
