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

```
## YYYY-MM-DD
- What changed (files touched, counts, rationale)
- What was left pending and why
- New open questions surfaced
```

---

## 2026-05-06 — Doorman DataGraph proxy endpoints + Stage-6 promote

### What landed this session

**Inbox housekeeping (commit `f8e21d0`, Jennifer Woodfine):**
- 3 Master inbox messages archived (DataGraph broadcast, proxy task, project-design request)
- Outbox reply to task@project-design: git-documentation-wiki.zip outside Foundry authority; DataGraph interim at port 9081; canonical Doorman path landing this session

**Doorman graph proxy endpoints (commit `5a6d3f0`, Peter Woodfine):**
- `POST /v1/graph/query` — proxies to `GET {SERVICE_CONTENT_ENDPOINT}/v1/graph/context`
- `POST /v1/graph/mutate` — proxies to `POST {SERVICE_CONTENT_ENDPOINT}/v1/graph/mutate`
- Both require `X-Foundry-Module-ID` header (400 if absent); 503 if endpoint unconfigured
- Audit-log every call via `AuditLedger::append_capture_entry()` with `event_type: "graph-query"` / `"graph-mutation"` (non-fatal)
- New error variants: `GraphProxyMissingModuleId` (400), `GraphProxyServiceUnavailable` (503)
- New `app_state_with_service_content()` test helper
- 5 new wiremock tests; 162 → 167 tests total

**State file updates (commit `bd19107`, Jennifer Woodfine):**
- `service-slm/CLAUDE.md`: test count 162→167 + DataGraph proxy section
- `.agent/outbox.md`: canonical path confirmed live (commit hash, no longer "landing this session")

**Stage-6 promote (this commit):**
- Archived Master's `2026-05-06T02:00Z` "hands off" message; code was already committed
- `service-slm/NEXT.md` updated: DataGraph proxy added to completed software items
- `service-slm/docs/audit-endpoints-contract.md` §2.3 updated: `graph-query` noted as Doorman-internal
- Rebased local main on canonical origin/main; resolved conflicts
- Pushed to staging mirrors + ran `promote.sh`

### Tests

167/167 passing (14 slm-core + 96 slm-doorman + 5 queue + 4 audit + 48 http). Pre-existing flaky `concurrent_workers_dont_double_lease` (flock timing race) passes on retry; unrelated to this session.

### Pending

- All software tasks complete. Remaining work is operator-presence infra (Tier C auth, Yo-Yo deploy, cmake, D4 image pipeline).
- `service-slm/docs/audit-endpoints-contract.md` is at v0.2.0; the graph proxy handlers write `graph-query` entries internally (not via `/v1/audit/capture`). Contract §2.3 now carries a note.

---

## 2026-05-05 — Email template ingest — V5 catalog JSON DATA format (420 communication-template entities)

Session fixed `load_email_templates()` in `service-content/scripts/ingest-jennifer.py` to handle the V5 catalog format.

### Problem

The V5 ZIP catalog (`catalog_base.html`) is a JavaScript SPA — template records are embedded as `const DATA = [{...}, ...];` in a `<script>` block. The original parser used `HTMLParser` to walk `.card` HTML divs, which only exist in the older v1 static catalog. V5 produced 0 entities.

### Fix — commit `c140c3c` (Peter Woodfine)

`load_email_templates()` updated with two-path logic:
1. **V5 path (primary):** `re.search(r'const DATA = (\[.*?\]);', html, re.DOTALL)` + `json.loads()`. Entity mapping: `entity_name` ← `subject`, `role_vector` ← `cat | subName | desc`, `location_vector` ← `code` (e.g., `WMCTCOLFLUP001`), `contact_vector` ← `body[:300]`.
2. **v1 HTML fallback:** `_EmailCatalogParser` HTML card-div parser retained for older static catalogs.

### Outcome

420 `communication-template` entities loaded into the graph at `module_id=woodfine`. Graph is queryable at `http://127.0.0.1:9081`. Entity names are email subject lines; `location_vector` carries the `WMCT{CAT}{TYPE}{NNN}` code for exact-match lookup.

### Pending

- Catalog extracted to `/tmp/email-template-v5/` — temporary; will not survive reboots. Re-run ingest from that path if service-content is restarted on a fresh VM.
- Phase 5 deferred: `.eml` MIME parser for `git-documentation-wiki.zip` (246 self-notes emails → `notes-document` classification).

---

## 2026-05-05 — Taxonomy config layer — Archetypes, COA, Domains, Glossary, Themes, Topics as HTTP-editable CSV config

Session built the full taxonomy config layer for service-content in response to operator's request: *"these are the .config files for service-content."*

### What landed this session — commit `18f37c2` (Jennifer Woodfine)

**Phase 0 — ontology CSVs replaced/created (12 files):**
- `ontology/archetypes.csv`: 11 archetypes with `signature`, `healing_trigger`, `gravity_keywords` columns
- `ontology/chart_of_accounts.csv`: 50+ reference-numbered entries from Excel CRM tab (1000s Personal, 2000s Compliance, 3000s Real Estate, 4000s Collaborators, 5000s Finance, 6000s IT Support) — replaces old 13-row stub
- `ontology/themes.csv`: 11 themes with `scope` column — 4 tactical + 7 strategic merged from two separate locations; each strategic theme carries a `thesis` field (one-sentence narrative)
- `ontology/domains/domain_{corporate,documentation,projects}.csv`: thesis + gravity_keywords added to all three
- `ontology/glossary/glossary_{corporate,documentation,projects}.csv`: 1,029 terms promoted from jennifer deployment `/srv/foundry/deployments/cluster-totebox-jennifer/service-content/domains/`; `domain` column added; Python csv library used to handle quoted fields (awk comma-split would break on embedded commas)
- `ontology/topics/topics_{corporate,documentation,projects}.csv`: stub topic maps, 5 rows each

**Phase 1 — `service-content/src/graph.rs`:** Two new methods on `GraphStore` trait + `LbugGraphStore` impl:
- `delete_by_classification(module_id, classification) -> Result<usize>`
- `delete_by_classification_and_location(module_id, classification, location) -> Result<usize>`

**Phase 2 — `service-content/src/taxonomy.rs` (new, ~420 lines):** 6 row types (`ArchetypeRow`, `CoaRow`, `DomainRow`, `GlossaryRow`, `ThemeRow`, `TopicRow`), `TaxonomyBundle`, `parse_*` / `serialize_*` / `*_to_entities` converters per type, `load_taxonomy_from_dir()`, `bundle_to_entities()`, `skip_header_owned()`. All taxonomy entities use `module_id = "__taxonomy__"` and `confidence = 1.0`.

**Phase 3 — `service-content/src/config_http.rs` (new, ~200 lines):** 12 GET/POST `/v1/config/*` endpoints:
- GET returns raw CSV from disk (text/csv response, round-trip start)
- POST parses CSV body, calls `delete_by_classification`, upserts entities → `{"loaded": N, "classification": "..."}`
- Routes: `/v1/config/archetypes`, `/v1/config/coa`, `/v1/config/domains`, `/v1/config/themes`, `/v1/config/glossary/:domain`, `/v1/config/topics/:domain`

**Phase 4 — wiring:** `main.rs` gains `SERVICE_CONTENT_ONTOLOGY_DIR` env var + startup taxonomy load into `__taxonomy__` namespace; `http.rs` HttpState gains `ontology_dir: String`; `config_routes()` merged into axum Router; `Cargo.toml` adds `csv = "1.3"`.

**Build:** `cargo check` clean (1 pre-existing dead_code warning on `list_entities`); `cargo build --release` succeeded.

### Design decisions recorded

- **`__taxonomy__` isolation**: taxonomy entities live under `module_id = "__taxonomy__"` separate from tenant queries (`woodfine`). Open question: should taxonomy also appear in tenant context queries? Not resolved — separate decision before wiring GraphContextClient.
- **Thesis as first-class field**: "Thesis" was not previously defined anywhere in the architecture. Defined as a one-sentence declarative claim per domain/theme, now stored in the domain CSVs and themes.csv.
- **Two-theme system resolved**: tactical themes (4, from `seeds/Themes.json`) + strategic themes (7, from jennifer `themes.md`) merged into single `themes.csv` with `scope` column.
- **Glossary source of truth promoted**: authoritative glossary moved from jennifer deployment path to `service-content/ontology/glossary/` in the source repo. `ingest-jennifer.py` still loads from the old path — needs update to load from ontology/.

### What is pending

- `ingest-jennifer.py` glossary load path: should load from `service-content/ontology/glossary/` instead of jennifer deployment `domains/` — the two are now in sync but the old path will diverge on next edit
- Phase 5 (operator-presence gated): `.eml` MIME parser for git-documentation-wiki.zip (246 self-notes emails → `notes-document` classification); template loader for service-email-template_V5.zip (420 templates → `communication-template` classification)
- Service restart with `SERVICE_CONTENT_ONTOLOGY_DIR` set to verify startup taxonomy load and test `/v1/config/*` endpoints
- Stage-6 promote: 38 commits ahead of origin/main

---

## 2026-05-05 — Ontological Data Graph full corpus expansion — all jennifer markdown sources loaded

Session continued from prior context. Prior session had loaded 9,999 entities (people.csv + corporate.csv). This session expanded to full corpus.

### What landed this session

**ingest-jennifer.py expanded** (`service-content/scripts/ingest-jennifer.py`, `89d2813`):
- Added `MARKDOWN_SOURCES` — 9 directory/classification/confidence tuples covering every document type in cluster-totebox-jennifer
- `_title_from_filename` — comprehensive regex rewrite handling 8+ Bloomberg filename patterns (RESEARCH_, PUBLISHED_, date variants, space- and underscore-delimited, `.pdf copy N` suffixes, ` - Bloomberg` suffix, `COLOUR_PORTRAIT_` prefix)
- `_extract_md_role_vector` — extracts first meaningful paragraph from markdown, stripping frontmatter, URLs, timestamps, copyright footers
- `load_markdown_dir` / `load_documents` — batch-load all markdown sources; enriches existing entities via MERGE
- `--skip-documents` flag to skip document loading for faster people-only reloads
- Total entities loaded: **10,414** across 7 classifications
  - person: 4,680 | company: 4,833 | organization: 62
  - domain-term: 424 | research-document: 455+ | corporate-document: 43
  - regulatory-document: 7 | architecture-reference: 19 | technical-reference: 10

**Outbox to project-editorial updated** (`.agent/outbox.md`):
- Full entity inventory with all 7 classifications
- Expanded query examples organized by TOPIC area (corporate architecture, flow-through, co-location, Broadcom, compliance)
- Corrected query syntax note: `q` is a continuous substring match — use single keywords or short exact phrases

**Confirmed graph is live and queryable:**
- `?q=woodfine` → Woodfine people/companies
- `?q=flow-through` → domain-term and research-document entities
- `?q=co-location` → mandate-related entities
- `?q=broadcom` → digital infrastructure entities
- `?q=exempt+market` → regulatory/compliance entities

### What is pending

- TOPIC authoring: project-editorial's scope (Doctrine claim #35). 5 suggested topics staged in outbox.
- service-slm Yo-Yo #2 and Doorman Tier C auth: operator-presence gated (unchanged).
- Stage-6 promote: 36 commits ahead of origin/main.
- graph is live only while service-content process is running; restart command documented in outbox.

### Tests

162/162 passing (unchanged — no Rust code modified this session).

---

## 2026-05-05 — Ontological Data Graph light run — cluster-totebox-jennifer

Session opened with housekeeping (cluster rename project-slm → project-intelligence committed in `9de72da`; manifest + .gitignore + outbox-archive updated).

### What landed this session

**ingest-jennifer.py** (`service-content/scripts/ingest-jennifer.py`, new):
- Reads `cluster-totebox-jennifer/service-people/people.csv` (9,575 entities: 4,680 person, 4,833 company, 62 organization) + `service-content/domains/corporate.csv` (424 domain-term entities)
- Batches in chunks of 100; POSTs to `POST /v1/graph/mutate`
- 100/100 batches HTTP 200; 9,999 entities loaded, all `module_id=woodfine`
- Graph persists at `service-content/data/jennifer-graph/entities.lbug` (gitignored)
- `--dry-run` flag for safe preview without a running server

**service-content graph verified live:**
- `GET /v1/graph/context?q=woodfine&module_id=woodfine&limit=5` → Mathew Woodfine, Peter M. Woodfine, Jennifer M. Woodfine, Woodfine Management Corp., Woodfine Capital Projects Inc.
- cmake + g++ (build-essential) confirmed present on workspace VM — NEXT.md "operator-presence carry" for cmake was already resolved; updated accordingly

**Guide path fix** (`woodfine-fleet-deployment/vault-privategit-source/guide-doorman-deployment.md`):
- Two stale `/srv/foundry/clones/project-slm/` path references → `/srv/foundry/clones/project-intelligence/`
- Preserved `"module_id": "project-slm"` in sample audit-ledger entry (historical runtime data, not a path)

**.gitignore** updated: added `service-content/data/` exclusion with comment.

**Outbox to project-editorial**: message prepended to `.agent/outbox.md` with entity inventory, binary start command, full query syntax + 5 example curls, suggested TOPIC list (5 topics, destination repos), BCSC posture reminder, and note that querying is unrestricted while Yo-Yo #2 continues.

### What is pending

- TOPIC authoring: project-editorial's scope (Doctrine claim #35). 5 suggested topics staged in outbox.
- service-slm Yo-Yo #2 and Doorman Tier C auth: operator-presence gated (unchanged from previous session).
- cmake availability confirmed — remove that item from NEXT.md operator-presence list on next NEXT.md update pass.

### Tests

162/162 passing (14 slm-core + 96 slm-doorman + 5 queue + 4 audit + 43 http). Pre-existing flaky `concurrent_workers_dont_double_lease` failed first run; passed on retry (flock timing race, documented).

---

## 2026-05-02 — system file filter + site_title + guide_dir_2 + service rename

- **System file filter**: `SYSTEM_FILE_STEMS` const added to `server.rs` (13 stems: README, CHANGELOG, MANIFEST, CLAUDE, NEXT, NOTAM, TRADEMARK, CODE_OF_CONDUCT, BUDGET, DOCTRINE, LICENSE, CONTRIBUTING, SECURITY). Filter applied at both root and subdirectory level in `collect_topic_files()`. Closes UX issue where repo-management files appeared in "All articles" catch-all section.

- **`site_title` field**: `AppState.site_title: String` + `--site-title` / `WIKI_SITE_TITLE` CLI/env arg. Same binary now serves multiple wiki instances with distinct branding. All three chrome functions updated. Default preserves existing value ("PointSav Documentation Wiki").

- **`guide_dir_2` field**: `AppState.guide_dir_2: Option<PathBuf>` + `--guide-dir-2` / `WIKI_GUIDE_DIR_2`. `collect_all_topic_files()` signature changed from `(content_dir, guide_dir: Option<&FsPath>)` to `(content_dir, guide_dirs: &[Option<&FsPath>])`. All call sites updated: `index()`, `sitemap_xml()`, `llms_txt()`, `wiki_page()` fallback chain. `bucket_topics_by_category()` takes both guide dirs. 11 inline test fixtures and 12 external test `AppState` constructions updated (Python script for both).

- **Service rename**: `local-knowledge.service` stopped, disabled, and superseded by `local-knowledge-documentation.service`. New unit adds `WIKI_GUIDE_DIR_2=/srv/foundry/customer/woodfine-fleet-deployment` and `WIKI_SITE_TITLE`. New `local-knowledge-projects.service` added for Woodfine Projects Wiki at port 9093.

- **New deployment instance**: `~/Foundry/deployments/media-knowledge-projects-1/` with MANIFEST.md, README.md, README.es.md for `projects.woodfinegroup.com`. nginx vhost written and enabled. DNS + certbot pending.

- **Commit**: `ea4ad77` (Peter Woodfine) on `cluster/project-knowledge`. `cargo check` clean; all 110+ tests pass.

- **Pending**: Stage-6 promotion to canonical main (Master scope). DNS A record for `projects.woodfinegroup.com` + certbot after DNS propagation.

---

## 2026-05-01 — iteration-2 Waves 1+2+3 complete — recursive walk + article chrome + home page

- **Operator direction**: "we need to loop all the waves and get all the way though" — complete
  all three waves of engine changes to unblock the wiki re-launch.

- **Wave 1 (critical blocker):** `collect_topic_files()` recursive walk replaces the flat
  `read_dir()`. Path-qualified slugs (`architecture/compounding-substrate`). Route `/wiki/{*slug}`
  wildcard. `sitemap_xml()` + `llms_txt()` updated. 130+ TOPICs in category subdirectories are
  now visible to the engine.

- **Wave 2 (article chrome):** `short_description` italic subtitle below H1 (Frontmatter field
  added in `render.rs`); breadcrumb navigation; `last_edited` date in article footer;
  `category:` singular tag from the `category:` frontmatter field when `categories:` list absent.

- **Wave 3 (home page):** `LeapfrogFact` + `LeapfrogFactsYaml` types; `read_leapfrog_facts()`;
  `home_chrome()` two-column layout (featured left, leapfrog facts right); `HomeStats` banner
  previously shipped wired to new layout; `index()` calls all three.

- **CSS:** `.topic-short-description`, `.wiki-breadcrumb`, `.wiki-home-two-col`,
  `.wiki-home-leapfrog`, `.wiki-home-bilingual-notice`, `.wiki-article-last-edited`,
  `.wiki-category-single-tag` — all additive, no existing rules changed.

- **`cargo check` clean** (17.78s). All prior tests pass.

- **Pending / deferred:**
  - `feeds.rs` `collect_recent_items()` still uses flat `read_dir()` at line 53 — feeds only
    surface root-level topics. Deferred to Wave 4 (next session).
  - New integration tests for subdirectory slug resolution, leapfrog facts panel,
    `last_edited` footer — not written this session; existing 104 tests all pass.

- **Content side (content-wiki-documentation):**
  - `featured-topic.yaml` slug → `architecture/topic-leapfrog-2030-architecture`
  - `leapfrog-facts.yaml`: 8 facts with path-qualified link_slugs (all resolve)
  - `index.md`: Leapfrog 2030 lede; ENGINE duplicates removed; Provenance section removed

