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

## 2026-05-28 — app-mediakit-knowledge Phase 6A+6B+6C — AJAX nav fix, home page caps, topnav header

- **Phase 6A (wiki.js):** Root cause of "articles not loading" confirmed — `navigateTo()` was
  targeting 3 stale selectors from Phase 2 DOM renames: `#vector-toc` → `aside.toc`,
  `h1.page-title` → `h1.article__title`, `.wiki-breadcrumb` → `nav.crumb`. All 6 occurrences
  fixed. Also fixed `initToc()`, `initTocPin()` getElementById → querySelector, and
  `initActiveTocTracking()` heading selector `.mw-body h2[id]` → `.prose h2[id], .prose h3[id]`.
  server.rs: added `id="toc-list"` to TOC `<ol>`.

- **Phase 6B (server.rs home page):** Removed `div.wiki-home-uncategorised` block (orphan files
  should not surface on the front page). Guides capped at 6 (`.iter().take(6)`). Data fetch
  aligned: `recent_topics_by_last_edited(&buckets, 10)` → `(&buckets, 8)`.

- **Phase 6C (server.rs + style.css):** Replaced 3-row `header.shell-header` (152px) with
  single-row `header.topnav` (80px) in all three chrome functions. `WORDMARK_SVG_POINTSAV`
  constant added: verbatim PointSav SVG from `home.pointsav.com` (320×80px). `.topnav` CSS:
  `1fr auto 1fr` grid, Oswald 11px uppercase navy links, dark-mode SVG invert.
  `--header-h` updated 152px → 80px.

- **Commit:** `afa67bfa` (Jennifer). 106/106 tests pass. Stage 6 + binary rebuild pending.

---

## 2026-05-27 — User feedback sprint: Issues 1–3, 5; AEC Night 3 recovery

### GIS map UI fixes (index.html — commit 16d3d975)
- **Issue 1 (AEC bubble overlay):** Added `applyBubbleAecOverlayStyle()` function. When AEC layers are active,
  cluster bubbles become hollow tier-coloured rings (circle-opacity=0, stroke=tier colour) so Köppen/ASHRAE
  fills show through. Toggle off restores solid bubbles. Wired into `toggleAecMasterGlobal()`, `showOverview()`,
  `clearChainFilter()`.
- **Issue 2 (sel-el pinned):** Moved `#sel-el` Selected Location div from bottom of `showClusterDetail()` HTML
  to second position (after Regional Market, before tier badge). Added `.sel-el-pinned` CSS
  (`position:sticky; top:0; backdrop-filter`) so it stays visible while scrolling the panel.
- **Issue 3 (Retail View inspector):** Added `renderRetailLevelInspector()` + `toggleCatchmentMasterFromRetail()`
  functions; wired call to end of `setRetailLevel()`. Retail Zoom now shows a proper inspector panel with
  Catchment toggle (disabled-flash if no ring selected). Fixed silent 404 in `loadCatchmentCentroids()` to
  emit `console.warn` with cluster ID and HTTP status.
- **Issue 5 (electronics tier descriptor):** Added `"electronics"` to `retail` set and `("electronics", "Electronics")`
  to label tuple in `build-clusters.py:tier_descriptor()`. Overnight rebuild scheduled for 2026-05-29 05:00 UTC
  (crontab, `nightly-rebuild.log`).

### AEC Night 3 recovery (build-aec-koppen-ecozones.sh — commit 16d3d975)
- **Root cause 1 — wrong TIF filename:** Script expected `Beck_KG_V1_present.tif` inside the zip; actual
  filename in `Beck_KG_V1.zip` is `Beck_KG_V1_present_0p0083.tif` (1km = 0.0083° resolution). Fallback
  grabbed `Beck_KG_V1_future_0p5.tif` (32K, coarse 0.5° version) — wrong scale entirely (2,284 polygons
  from a global dataset that should produce tens of millions).
- **Root cause 2 — GPKG geometry column:** Step 3 SQL queried `geometry` column; `gdal_polygonize` writes
  GPKG with column named `geom`. Fixed both in the script.
- **Recovery:** Correct 23MB TIF extracted from freshly-downloaded zip at /tmp, wrong intermediates removed,
  Night 3 restarted (PID 1805638 at 03:17 UTC). Expected completion ~04:00–04:30 UTC, before Night 4
  seismic at 05:00 UTC.
- **Night 4 (seismic) at 05:00 UTC today, Night 5 (flood) at 05:00 UTC May 28 — crontabbed, unchanged.**
- **Disk: 37G free** — adequate for Night 3 + Night 4 running concurrently.

### Pending (this session)
- **Issue 6 (research.html):** Deferred to next session. Content review required: 7 TODO markers in thesis
  draft (Appendix B country table fillable from Phase 22 data, §5.3 LODES note, Appendix C diagram placeholder).
  Confirmed paper NOT submitted to any journal — draft notice must use "in preparation for intended submission"
  language only.
- **Issue 4 (mobility catchments):** Multi-session roadmap (LODES S1–S4, ~4 sessions total). Not sequenced.

```
## YYYY-MM-DD
- What changed (files touched, counts, rationale)
- What was left pending and why
- New open questions surfaced
```

---

## 2026-05-27 — app-orchestration-slm Steps 3+4 — chassis self-registration + adapter-hub

- **Step 3:** `slm-doorman-server/src/main.rs` — added non-blocking chassis self-registration.
  When `SLM_ORCHESTRATION_ENDPOINT` is set, a `tokio::spawn` POSTs
  `{module_id, archive_id, doorman_endpoint, tier_b_subscribed}` to the chassis
  `/v1/discovery/register` endpoint at startup. Four new env vars documented in the module
  doc comment alongside existing `SLM_*` vars. Zero impact on existing deployments (guarded
  by env var presence check). `reqwest` and `serde_json` were already in `Cargo.toml`.

- **Step 4:** extracted `slm-doorman/src/adapter_registry.rs` (dead code — not exported
  from lib.rs, zero callers) → new crate `service-slm/crates/adapter-hub/`. Added:
  `fuse_adapters(base, overlays) -> String` stub (returns symbolic composed ID; real GGUF
  merge deferred until llama.cpp LoRA hot-swap PR upstream). Added `serde_yaml = "0.9"` to
  workspace deps (was missing; adapter_registry.rs used it but could never compile as-is).
  `slm-doorman/src/lib.rs` re-exports `AdapterEntry`, `AdapterRegistry`, `fuse_adapters`.
  5 tests migrated + 1 new `fuse_adapters_stub` test — all 5 pass.

- **Commit:** `99e2f06a` (Jennifer). `cargo check --workspace` clean. Pre-existing
  `micro_node` integration test failure (stale `AppState` fields) unchanged.

- **Stage 6 pending:** `99e2f06a` (and prior `49a802a2` MVP scaffold) need `bin/promote.sh`
  from Command Session.

---

## 2026-05-12 — Phase 4 Steps 4.4+4.5 — redb wikilink graph + blake3 content hashes

- **`src/links.rs`** (new, 230 lines): `LinkGraph` struct backed by redb. Two tables in
  `<state_dir>/links.redb`: `outlinks` (composite key `"from_slug\x00to_slug"` → u8 sentinel;
  supports prefix scan for outlinks and full-scan filter for backlinks) and `hashes`
  (`"slug\x00revision_sha"` → 32-byte blake3 digest; federation-seam baseline for Phase 7).
  Public API: `open_or_create`, `rebuild_for_slug`, `backlinks`, `record_hash`,
  `lookup_by_hash`, `for_testing`. Wikilink parser: regex `r"\[\[([^\]|#\[]+)"`, output
  slugified (lowercased, spaces → hyphens, anchors/aliases stripped). `for_testing()` uses
  tempfile + atomic counter for isolated parallel test databases.

- **`tests/links_test.rs`** (new, 133 lines): 7 integration tests — 6 unit-level graph
  tests (backlink add/clear, multiple sources, self-links, blake3 round-trip, unknown hash)
  + 1 route-level test (`whatlinkshere_returns_backlinks_from_graph` via tempfile fixture +
  oneshot router pattern matching `tests/feeds_test.rs`).

- **Wiring across 20 files:**
  - `src/error.rs`: new `WikiError::LinkGraph(String)` variant; mapped to HTTP 500.
  - `src/lib.rs`: `pub mod links;` added.
  - `src/main.rs`: `LinkGraph::open_or_create(&state_dir.join("links.redb"))` at startup
    (after git repo and glossary); stored in `Arc<LinkGraph>`; passed as `AppState.links`.
  - `src/server.rs`: `AppState.links: Arc<LinkGraph>` field; `GET /special/whatlinkshere/{slug}`
    route + `what_links_here` handler (reads `backlinks()`, renders HTML list); "What links here"
    link in article footer chrome. All `AppState` test constructors updated.
  - `src/edit.rs`: `post_edit` and `post_create` both call `record_hash(slug, oid_sha, body)`
    after git commit succeeds, and `rebuild_for_slug(slug, body)` unconditionally. Failures
    logged non-fatally (link graph is derived state, rebuildable).
  - All 11 pre-existing integration test files: `links: LinkGraph::for_testing()` added to
    `AppState` construction (1–2 lines each).

- **Cargo**: `redb = "4.1"` + `blake3 = "1.8"` added to `[dependencies]`.

- **Test results**: 7/7 `links_test` pass (`cargo test --test links_test`). `cargo check`
  clean. Pre-existing `doorman_stubs_return_correct_json_shape` failure unchanged (unrelated).

- **Stage 6 needed**: Wikipedia Parity Phases 1+2A+3 commits (`3b557cf`, `68c643c`, `b8a1ad8`,
  `3cee49d`) + this Phase 4 commit (`177813e`) + cleanup-log entry (this commit) need
  `bin/promote.sh` from Command Session to reach canonical `pointsav/pointsav-monorepo` main.

- **Pending**: Step 4.6 (MCP server via rmcp) and Step 4.7 (git smart-HTTP remote) per
  `docs/PHASE-4-PLAN.md`. Deploy: `systemctl restart local-knowledge-documentation.service`
  after Stage 6 binary rebuild.

---


## 2026-05-12 — Wikipedia Parity Phase 3 — keyboard shortcuts + TOC pin + AJAX page navigation

- **wiki.js rewritten** (~619 lines → ~530 lines). Module-level state vars added for idempotent
  re-init on AJAX navigation: `_sectionObserver`, `_hoverCard`, `_hoverTimer`, `_hoverTarget`,
  `_hoverCache`, `_glossaryTip`, `_fnTip`.

- **5 content-dependent init functions extracted/renamed** to support AJAX page swap:
  `initHoverCards()`, `initGlossaryTooltips()`, `initFootnoteTooltips()`,
  `initNavboxes()`, `initCollapsibleSections()`, `initActiveTocTracking()` (stores observer
  ref in `_sectionObserver`; disconnects before content swap). Called at boot and in
  `reinitContentInteractions()` after every AJAX navigation.

- **Keyboard shortcuts (Part 1)**: `?` key toggles shortcut help overlay; `Esc` closes it.
  AccessKey attributes added to server.rs — `accesskey="r"` (Read), `accesskey="e"` (Edit),
  `accesskey="s"` (View source), `accesskey="h"` (View history), `accesskey="t"` (Talk).
  Browsers trigger via Alt+Shift+key (Firefox/Linux), Alt+key (Chrome), Ctrl+Option (macOS).

- **TOC pin button (Part 2)**: `button.toc-pin-btn #toc-pin-btn` added to `div.toc-header` in
  server.rs (after the existing `[hide]` toggle). `initTocPin()` in wiki.js — pin state
  persisted to `localStorage['wiki-toc-pinned']`; pinned TOC cannot be collapsed by the hide
  button; `applyPinState()` toggles `toc-pinned` class + `aria-pressed` + button text.

- **AJAX page navigation (Part 3)**: `initAjaxNavigation()` intercepts `/wiki/*` link clicks
  and `popstate` events. `navigateTo()` uses `fetch()` + `DOMParser` + DOM swap of
  `#mw-content-text`, `#vector-toc`, `h1.page-title`, `nav #p-views`, `.wiki-breadcrumb`,
  `document.title`. Loading bar (`#wiki-loading-bar`) with CSS-driven progress at page top.
  Modifier clicks (Ctrl/Meta/Alt/Shift) and non-`/wiki/` links fall through to full navigation.
  On fetch error → `window.location.href` fallback. `history.pushState` for forward nav;
  `history.replaceState` seeds initial state. Uses `.then/.catch` (not async/await) for
  broad browser compat.

- **CSS additions** (~80 lines appended): `#wiki-loading-bar` (fixed top-of-page progress bar);
  `#toc-pin-btn` + `.toc-pin-active` (pin button next to hide toggle); `#wiki-shortcut-overlay`
  + `#wiki-shortcut-panel` + `#wiki-shortcut-close` + `.wiki-shortcut-note` (keyboard overlay).

- **Commit**: `3cee49d` (Jennifer). 60/60 lib tests pass. `doorman_stubs_return_correct_json_shape`
  pre-existing failure, unrelated.

- **Deployment**: Release build needed; install + `systemctl restart` pending for both services.

---

## 2026-05-27 — app-orchestration-slm MVP scaffold

- **New project directory** `app-orchestration-slm/` created — commercial Yo-Yo broker chassis
  implementing DOCTRINE claim #23 (multi-Totebox paid tier). Follows the `app-orchestration-bim`
  chassis pattern, deploys as `gateway-orchestration-slm-N` on `os-orchestration` host.

- **Three-crate Cargo workspace** (standalone, not a monorepo root member):
  - `orchestration-slm-core` — shared wire types: `FleetMember`, `RegistrationRequest`,
    `ReadyzResponse`, `YoyoLabel`, `CHASSIS_VERSION`
  - `orchestration-slm` — business logic: `FleetRegistry` (in-memory RwLock HashMap),
    `YoyoProxyClient` (reqwest client with 90s timeout), `MeteringLedger` (per-tenant
    in-process cost metering), `ChassisError` (thiserror enum)
  - `orchestration-slm-server` — axum binary on `:9180`, 7 MVP endpoints

- **MVP endpoints live:**
  `GET /healthz`, `GET /readyz` (Yo-Yo probe + fleet count), `GET /v1/fleet`,
  `POST /v1/discovery/register`, `POST /v1/yoyo/proxy`, `POST /v1/yoyo/trainer`,
  `POST /v1/yoyo/graph`

- **Auth model (MVP):** `Authorization: Bearer <module-id>` → fleet lookup (401 if unregistered),
  `X-Foundry-Module-ID` header match (403 if spoofed), `tier_b_subscribed` flag (402 if free tier).
  SSH-signed token validation deferred to Phase 3.

- **No new Yo-Yo VMs.** Chassis front-ends existing Yo-Yo #1 (`"trainer"` L4 24GB) and
  Yo-Yo #2 (`"graph"` H100 80GB) via `ORCHESTRATION_YOYO_TRAINER_ENDPOINT` /
  `ORCHESTRATION_YOYO_GRAPH_ENDPOINT` env vars.

- **Tests:** 13 tests across fleet.rs + metering.rs + http.rs; all pass. `cargo check --workspace`
  clean (0 warnings, 0 errors).

- **Registry:** `app-orchestration-slm` row added as Scaffold-coded; total 105 rows.

- **Stage 6 pending:** commit needs `bin/promote.sh` from Command Session.

- **Pending (post-MVP):**
  - Step 3: update Doorman `slm-doorman-server/src/main.rs` to POST to
    `SLM_ORCHESTRATION_ENDPOINT` on startup (chassis self-registration)
  - Step 4: `adapter-hub` crate split from `slm-doorman/src/adapter_registry.rs`
  - Step 5: `lora-forge` scaffold + `build-corpus`
  - Step 6: rename `compute/packer/scripts/lora-training.sh` → `lora-loom/run.sh`
  - Phase 2: `/v1/graph/federated`, `/v1/training/schedule`, `/v1/adapters`

---

## 2026-05-12 — Wikipedia Parity Phase 2A — article typography regression fix + color token port

- **Regression fix**: Phase 1 changed `article.wiki-article` → `div #mw-content-text`, silently
  breaking all `article { }` CSS rules (article typography: Georgia serif, heading borders, link
  colors, code blocks, blockquotes, tables). Fixed by replacing the entire article-body block
  (lines 118–197) with `.page-body { }` equivalents. `.page-body` is the `div.page-body` wrapper
  that the server renders inside `div#mw-content-text`.

- **`--mw-*` tokens wired into article rules**: `.page-body a` → `var(--mw-color-link)`,
  `.page-body a:visited` → `var(--mw-color-link-visited)`, code/pre backgrounds →
  `var(--mw-color-base-10)`, borders → `var(--mw-color-base-50)`.

- **9 hardcoded hex colors in secondary `:root` block ported** to existing CSS variables:
  `--toc-bg`, `--tab-active-border`, `--tab-hover-bg`, `--density-btn-bg`,
  `--density-btn-active-bg`, `--density-btn-active-fg`, `--hatnote-color`, `--cat-bg`.

- **4 body-level hardcoded colors ported**: `.wiki-lang-btn:hover color` → `var(--bg)`;
  `.wiki-home-featured background` → `var(--mw-color-base-10)`; `.wiki-home-dyk background` →
  `var(--bg)`; `a.wiki-redlink color` → `var(--mw-color-link-redlink)`.

- **Left unchanged** (UI-specific palettes with no matching token): `#b58900` FLI notice border;
  `#a55858`/`#d73c3c`/`#b52e2e`/`#ffeef0`/`#f5c2c7` editor/auth error-state palette.

- **Commit**: `68c643c` (Jennifer). 60/60 lib tests pass. `doorman_stubs_return_correct_json_shape`
  pre-existing failure, unrelated to this change.

- **Deployment**: Release build in progress; install + `systemctl restart` pending.

---

## 2026-05-12 — Wikipedia Parity Phase 1 DOM standardisation

- **7 structural class/ID names renamed** to MediaWiki/Vector 2022 equivalents across
  `src/server.rs`, `static/style.css`, `static/wiki.js` (commit `3b557cf`, Peter).
  PointSav-specific classes (`wiki-home-*`, `wiki-cat-*`, `wiki-special-*`, etc.) left unchanged.

  | Old | New | Scope |
  |---|---|---|
  | `.site-header` / `#site-header` | `.mw-header` / `#mw-header` | `<header>` chrome |
  | `div.wiki-left-rail` | `div #mw-panel` | left sidebar |
  | `nav.wiki-nav-portlet` | `nav.vector-main-menu` | nav portlet |
  | `nav.wiki-toc` / `#wiki-toc` | `nav.vector-toc` / `#vector-toc` | TOC |
  | `main.wiki-main` | `main.mw-body` | page body wrapper |
  | `nav.wiki-action-tabs` | `nav #p-views` | Read/Edit/History tabs |
  | `article.wiki-article` | `div #mw-content-text` | article body |

- **CSS custom properties seeded** in `:root` — 9 `--mw-*` aliases referencing the
  existing PointSav variables (Phase 2 token port entrypoints). No existing rules broken.

- **Maud syntax fix applied**: in Rust 2021, `element#id` (no preceding `.class`) is a
  reserved prefixed identifier. Correct form is `element #id` (space before `#`). Affected
  three elements: `div #mw-panel`, `nav #p-views`, `div #mw-content-text`.

- **One test updated**: `server::tests::wiki_page_renders_navigation_portlet` assertion
  changed from `"wiki-nav-portlet"` to `"vector-main-menu"`. 60/60 lib tests pass.
  `doorman_stubs_return_correct_json_shape` failure confirmed pre-existing (unrelated to
  this change).

- **No new open questions** from this session.

---

> **Archived entries:** session logs before this point are in `cleanup-log-archive.md`.
