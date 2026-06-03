# Cleanup Log — pointsav-monorepo

Living record of in-flight cleanup work, open questions, and decisions made during active development. This file is read at session start and updated at session end when meaningful cleanup occurs. Maintained in-repo so the history travels with the code.

---

## How this file is maintained

- **Read at session start.** Claude Code reads this file at the start of every session (per the instruction in `CLAUDE.md`). The tables below reflect the current state of in-flight work. Apply the guidance before touching any related files.
- **Update at session end.** When a session includes meaningful cleanup — renames across multiple files, deprecated code removal, resolving an open question, surfacing a new one — append a dated entry to the top of the **Session entries** section at the bottom of this file.
- **Do not log trivial edits.** Single-file typo fixes, comment tweaks, or routine formatting changes do not belong here. This log is a record of decisions, not of every keystroke.
- **Commit each update with the code changes it describes.** The log and the work it documents travel together through git history.

---

## Interpreting build signals during cleanup

Until the workspace `Cargo.toml` is unified (see Layer 1 audit findings), `cargo build --workspace` and `cargo check` at the repo root only exercise the 8 declared members. The other ~70 crates are not covered by workspace-level commands. When making changes to any crate outside the declared members, run `cargo check` inside that crate's directory specifically. Do not rely on workspace-root build signals to confirm correctness across the full repo. This caveat lifts when the workspace is unified.

---

## Active legacy-to-canonical renames

These substitutions are known and in progress. Canonical names are from the Nomenclature Matrix. When the last occurrence of a legacy name is removed from the repo, move the row to the **Completed migrations** section with the date of completion.

| Legacy | Canonical | Status | Notes |
|---|---|---|---|
| `cluster-totebox-real-property` | `cluster-totebox-property` | In flight | Appears in older deployment manifests and doc references. |
| `os-interface`, `os-integration` | `os-orchestration` | In flight | Legacy names predate the current three-layer stack nomenclature. |
| `RealPropertyArchive` | `PropertyArchive` | In flight | Appears in older archive-type documentation and possibly in legacy code comments. |

---

## Deprecations — flag and remove

Names no longer in use. Any occurrence in the repo should be flagged and removed. If a removal blocks something active, surface it — do not leave the legacy name in place silently.

| Name | Status | Notes |
|---|---|---|
| `fleet-command-authority` | Deprecated — remove | Node no longer in use. Should not appear in any current deployment manifest, build script, or documentation. |

---

## Intentional exceptions — do not migrate

Items that may look like candidates for cleanup but are intentionally preserved as-is. Do not "fix" these without confirmation.

| Item | Rationale |
|---|---|
| `cluster-totebox-personnel-1` and other numbered personnel instances | Exist locally but intentionally absent from GitHub and the MEMO. Not a naming error. Do not flag as legacy. |
| Two ConsoleOS operating patterns (multi-service `node-console-operator` and single-service nodes) | Both patterns are valid. The MEMO documents `node-console-operator` only, by design, to keep official documentation clean. Do not flag the single-service pattern as an inconsistency. |
| `service-llm` references in legacy docs | Legacy documentation predates `service-slm` naming; read as `service-slm`. Code is correct (code references are already `service-slm`). No migration action needed — this is a permanent documentation-reading convention, not an in-flight rename. Reclassified from Active renames to here per Brief 8 audit 2026-04-28. |

---

## Open questions

Pending confirmations that affect how Claude should describe or reason about parts of the system. Do not invent values for these. If a task requires an answer, stop and surface the question.

| Question | Current handling |
|---|---|
| Verification Surveyor daily throttle number | Under operational review. Do not cite a specific number. Refer to it as "a system-enforced daily limit" until confirmed in a future MEMO version. **Code reference (2026-04-23):** `app-console-content/scripts/surveyor.py` hard-codes `MAX_DAILY_VERIFICATIONS = 10`; whether this value is authoritative or drift is the pending decision. |
| User Guide language on Sovereign Data Foundation | The User Guide contains language treating the Foundation as a current equity holder and active auditor. Requires a language review pass before any User Guide content is reused in public-facing materials. Flag any passage that describes the Foundation as current or active. |
| Is the per-crate independent workspace pattern intentional (some crates meant to be extractable and published separately) or accidental drift? | Pending decision — do not act on related findings until answered. |
| Are `app-console-*` and `app-network-*` directories without `Cargo.toml` intentional scaffolding for planned work, or abandoned attempts? | Pending decision — do not act on related findings until answered. |
| ~~Should the doubly-nested `service-email-egress-{ews,imap}` structure be flattened, or does the nesting reflect a real protocol-implementation hierarchy?~~ | **Answered 2026-04-23:** wrappers flattened; two crates kept separate (distinct protocol adapters, not duplicates). 13 Cargo.toml name mismatches remain as separate structural audit finding (not an open question — a known defect). Reclassified per Brief 8 audit 2026-04-28. |
| What is `discovery-queue` — runtime data that should be gitignored, reference data that belongs elsewhere, or a misplaced crate? | Pending decision — do not act on related findings until answered. |
| ~~Does `vendors-maxmind` (containing a GeoLite2 database, not code) belong as a `vendor-*` crate at all, or should it move to a non-workspace data directory?~~ | **Answered 2026-04-23:** non-workspace data directory. Moved to `app-mediakit-telemetry/assets/` (matching the authoritative target path already documented in the vendor's README). `vendor-*` crate framing rejected: the directory contained only data, no code. |

---

## Completed migrations

Migrations fully resolved in the repo. Moved here from **Active legacy-to-canonical renames** when the last occurrence of the legacy name is removed. Empty for now.

| Legacy | Canonical | Closed | Notes |
|---|---|---|---|
| `service-parser` | `service-extraction` | 2026-04-23 | Legacy-era scaffold containing only a README that described an AI-routing architecture since superseded by `service-extraction`'s deterministic Parser-Combinators approach. Zero runtime references, never a workspace member, one commit in history. No code or data to recycle into `service-extraction`; README deleted without migration. |
| `pointsav-pty-bridge` | `service-pty-bridge` | 2026-04-23 | Prefix-violation defect flagged in 2026-04-18 audit (brand prefix `pointsav-` not one of the seven canonical prefixes). Canonical target `service-pty-bridge` fits the daemon runtime role. Working Rust crate with one source file; directory renamed via `git mv`, `Cargo.toml` `name` field updated in the same commit. Not a workspace member, zero external import references, no callers needed updating. |
| `tool-cognitive-forge` + `service-slm/cognitive-forge` | `service-slm/router-trainer/` + `service-slm/router/` | 2026-04-23 | Closes the last rename-series item and removes the "Cognitive Forge" Do-Not-Use term in one commit. The Rust runtime sub-crate at `service-slm/cognitive-forge/` renamed to `service-slm/router/` (Cargo.toml `name` field + `main.rs` usage string updated). The Python distillation workflow at `tool-cognitive-forge/` moved in to `service-slm/router-trainer/`, joining the runtime as producer/consumer pair. Rationale for split naming: the runtime is a router (of messages to service handlers); the trainer distils knowledge to produce the routing model. Inside `router-trainer/`, `distill_knowledge.py` moved from a non-canonical `src/` into `scripts/` alongside `ignite_teacher.sh`. Three binary/log files untracked from Git and covered by new `.gitignore` patterns (still physically present at new paths for the Python workflow): 35 MB `engine/llamafile`, 22 KB `engine/engine.log`, 89 B `llama.log`. The 15 MB `engine/weights/qwen2.5-coder-1.5b.gguf` was already covered by the existing `**/weights/*` + `*.gguf` patterns — no new ignore needed. Git history retains all blobs; shrinking history is separate `git-filter-repo` work. Registry: `tool-cognitive-forge` row removed; Scaffold-coded 54 → 53, Total 98 → 97. `llama.log` surfaced earlier in this session is closed by this commit. |
| `vendors-maxmind` | `app-mediakit-telemetry/assets/` | 2026-04-23 | Not a rename but a reclassification: the `vendors-maxmind` directory was a data container holding `GeoLite2-City.mmdb` + READMEs, no code. The vendor's own README already named `app-mediakit-telemetry/assets/` as the intended location — the monorepo had never realised that path. Moved the `.mmdb` + READMEs into their documented target; deleted the empty `vendors-maxmind/` directory. Monorepo `README.md` line 151 and `USER_GUIDE_2026-03-30_V2.md` line 902 updated to the new path. `repo-layout.md` extended to name `assets/` as a conventional project subfolder. Python script reference in `app-mediakit-telemetry/scripts/generic-omni-matrix-engine.py` left unchanged — it reads a deployment-side path relative to CWD, not the monorepo-side path. Separate `.mmdb` → build-time-fetch task remains open under Structural defects. |

---

## Session entries

Newest on top. Append a dated block when a session includes meaningful cleanup work. Format:

---

## 2026-06-03 — Archetype model rework + artifact refresh

- **Commuter (PKS) redefined as geometric airport-led park-and-ride** (`build-pks-clusters.py`).
  Replaced the metadata/co-location tiering with a purely geometric model: candidate = sized
  regional airport (park-and-fly, ≤600 km from a metro ref) OR outer commuter-rail-belt station
  (15–110 km ring, connected toward core, ≤4 stops from line end). Airports lead because they are
  the geographic-spread lever (rail-only covered 96 NA map cells; airport-led 957). `tier_pks_geo()`
  scores ring/connectivity/isolation/terminus. **5,977 features** deployed.
- **Urban Fringe (VWH) → Retail-density** (`build-vwh-clusters.py`). `qualify_vwh()` admits
  ≥2-category co-locations OR lone STRONG/BROAD trade stores; composition-score `tier_vwh(cats,n)`.
  **7,028 features** deployed. Both archetypes ≈ Retail bubble density.
- **New scripts:** `tools/sim_spread.py` (simulation harness), `ingest-osm-parking.py`,
  `ingest-osm-parcel-depot.py`, `run-overnight-ingests.sh` (crontab June 4 05:00 UTC). 20 new VWH
  brand-chain YAMLs authored in the local-only deployment data dir (not committed).
- **index.html:** mobile BentoBox footbar hardening (visualViewport detent heights + resize
  re-snap, `overscroll-behavior: contain`, modal `dvh`); cache-busting `?v=` token on archetype
  data URLs (fixed a stale-cache "not updating" report).
- **Commits:** code `aec2187e` (7 source files); docs commit (BRIEF, NEXT.md, artifact-registry,
  DATA-MANIFEST, session-context, archetypes memory, this log).
- **Stale-label fixes:** `session-context.md` + `MEMORY.md` frontmatter/header `project-orgcharts`
  / `project-infrastructure` → `project-data`. (`CLAUDE.md` mislabel left for operator — separate item.)

---

## 2026-06-01 — Contaminated session entries removed

All prior session entries in this file were copy-pasted from other archives during a past
`.agent/` bulk-copy. None described work performed in this archive's sessions. Entries removed:

- 2026-05-30 — P0 subnet fixes (project-system: system-udp, app-network-admin, system-gateway-mba, system-core, system-ledger, moonshot-toolkit)
- 2026-05-29 — app-mediakit-knowledge reindex_topic spawn_blocking fix (project-knowledge)
- 2026-05-29 — app-mediakit-knowledge Phase 8 + content-wiki-documentation (project-knowledge)
- 2026-05-28 — app-mediakit-knowledge Phase 6A+6B+6C (project-knowledge)
- 2026-05-27 — GIS map UI fixes + AEC Night 3 (project-gis; commit 16d3d975 not in this archive)
- 2026-05-27 — Phase 1C.a moonshot-toolkit + seL4 kernel (project-system)
- 2026-05-27 — v1.0.0 version bumps + PhD thesis BRIEF (project-system)
- 2026-05-21 — Groups 6+7 Stage-6 prep + WFD housekeeping (project-system)
- 2026-05-20 — Group 2 mechanical hygiene (project-system)

Action: command-20260601-cleanup-log-review-project-data (HIGH inbox item, actioned this session).

Actual session history for this archive begins below. Future session entries should describe
work committed to THIS archive's git branch only.

---

