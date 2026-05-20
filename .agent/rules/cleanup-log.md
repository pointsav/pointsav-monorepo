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

## 2026-05-29 — app-mediakit-knowledge Phase 8 + content-wiki-documentation updates

- **Phase 8 (`src/server.rs` + `static/style.css` + `tests/history_test.rs`):**
  - `wiki_chrome()` extended with `body_blake3: &str`; renders `div.article-integrity`
    with first 16 hex chars of `blake3::hash(body.as_bytes())`.
  - `history_page()`: pagination via `HistoryPageParams { page: Option<u32> }`, `PER_PAGE = 25`,
    older→/←newer nav links at bottom.
  - `diff_page()`: counts `diff-row-ins`/`del`/`chg` rows; `div.diff-stats` "+N / −M lines" header.
  - `hash_lookup_page()`: `/special/hash-lookup/{hash}` route; 200 on slug hit, 404 on miss.
  - CSS: `.article-integrity`, `.integrity-hash`, `.diff-stats`, `.history-pagination` added.
  - 3 new tests in `history_test.rs`; all 7 pass. Commit `0e5fd685` (Peter).
  - Pre-existing `collab_test`/`doorman_test` failures (stale `AppState.collab`) unchanged.

- **content-wiki-documentation (commit `13b8caa`, Jennifer):**
  - ES governance stubs: `about.es.md`, `contact.es.md`, `disclaimers.es.md`, `contribute.es.md`.
  - A6 PROSE-RESEARCH committed: `research/geometric-site-selection-national-tenancy.md` with
    preprint WIP block + Forward-Looking Statements block per `journal-artifact-discipline.md`.
  - `research/_index.md` + `research/_index.es.md` category landing pages.
  - `applications/app-privategit-workbench.md` draft fields removed; quality → pre-build.
  - Inbox cleared; 3 messages archived.

- **Stage 6 pending:** 16+ monorepo commits + `13b8caa` need `bin/promote.sh` from Command.
  Release binary build running; deploy to 9090/9093/9095 pending.

---

## 2026-05-28 — app-mediakit-knowledge Phase 6A+6B+6C — AJAX nav fix, home page caps, topnav header

- **Root cause diagnosis:** Phase 6C (commit `afa67bfa`) replaced the 3-row `header.shell-header`
  with a single-row `header.topnav` but (a) dropped the TOC toggle/pin buttons from `aside.toc`,
  causing `initToc()` and `initTocPin()` in wiki.js to early-return (IDs not found), and (b)
  removed search entirely — no `#header-search-q` emitted, so `initSearchAutocomplete()` no-ops.
  The AJAX navigation link fix (Phase 6A) was already committed but binary rebuild still pending.

- **`src/server.rs` — TOC buttons restored** in `div.toc__header` inside `aside.toc`. Added
  `button.toc-toggle #toc-toggle` and `button.toc-pin-btn #toc-pin-btn` alongside
  `span.toc__title`. These IDs now exist in the DOM so `initToc()` and `initTocPin()` execute.

- **`src/server.rs` — search form added to all three topnav locations** (`home_chrome`,
  `wiki_chrome`, `chrome`). `div.topnav-search-wrap` wraps `form.topnav-search` (with
  `input #header-search-q`) and `div.ac-dropdown #search-autocomplete-dropdown {}`. Both IDs
  are required by `initSearchAutocomplete()` which calls `getElementById` and early-returns if
  either is null.

- **`static/style.css` — two CSS blocks added:**
  - `.topnav-search-wrap` / `.topnav-search` / `#search-autocomplete-dropdown` / `.ac-item`
    (topnav search + autocomplete dropdown; mobile override narrows input to 100px at ≤768px).
    Mobile search stays visible because the existing `@media (max-width: 768px)` rule only hides
    `a:not(.lang-toggle)` and `.wiki-appearance-wrap` — a `<form>` element is not affected.
  - `.toc__header { display: flex }` + `#toc-toggle`, `#toc-pin-btn` button styles.
    `.toc__title { flex: 1 }` updated (border-bottom/padding moved to `.toc__header` container).

- **`static/wiki.js` — no changes needed.** Post-Phase-6A selectors (`aside.toc`,
  `h1.article__title`, `nav.crumb`) are already correct. Search and TOC init functions already
  reference the correct IDs — they were silently no-opping because the DOM elements were absent.

- **Tests:** `cargo check` clean. `cargo test` exits 0. Only pre-existing integration test
  failures in `collab_test.rs` / `doorman_test.rs` (stale `AppState.collab` / `enable_collab`
  fields from a removed module — unrelated to this change).

- **Stage 6 pending:** this commit (Phase 7A) + prior `afa67bfa` (Phase 6A/6B/6C) need
  `bin/promote.sh` from Command Session + nightly binary rebuild to reach live service.

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

## 2026-05-20 — Group 2 mechanical hygiene — system-core, system-ledger, moonshot-toolkit

Closed all 6 sub-groups of the Group 2 plan (project-system-todo.md). Six commits.

**Group 2A — system-core rustdoc + doc updates (`dcb2700`, Peter Woodfine)**
- Added per-variant rustdoc to `CapabilityType` (5 variants) and `Right` (5 variants)
  in `src/lib.rs`; field docs on `Capability`, `WitnessRecord`; `/// # Examples`
  block on `Capability::hash()`.
- 4 new tests: `capability_hash_expiry_none_vs_some`,
  `capability_hash_changes_with_witness_pubkey`, `right_variants_round_trip`,
  `capability_type_variants_round_trip`. Total: 62 tests (was 51+1 doctest = 52).
- `system-core/ARCHITECTURE.md` §3 resolved to IMPLEMENTED; §5 updated to 62 tests.
- `system-core/NEXT.md` fully rewritten to reflect v0.2.0 structurally complete state.
- `system-core/CLAUDE.md` updated: current state, test count, file layout.
- `system-core/master-relay.rs` deleted (`git rm`; legacy stub with hardcoded
  nonexistent `/bin/service-*` paths, never a `[[bin]]` target).

**Group 2B — 11 new negative-path tests in system-core (`334462b`, Peter Woodfine)**
- `checkpoint.rs`: 7 tests covering `ParseError` variants (NotUtf8, Truncated,
  MissingNewline, BadRootHashLength, MissingSignatureSeparator), `VerifyError::BadPublicKey`
  (y=2 is a quadratic non-residue on Ed25519; smallest non-curve point per Legendre
  symbol computation), `consistency_proof_new_signature_invalid_rejects`.
- `lib.rs`: 4 tests covering `capability_hash_expiry_none_vs_some`,
  `capability_hash_changes_with_witness_pubkey`, round-trip serialisation variants.
- Ed25519 non-curve point: `[0u8; 32]` (y=0) IS accepted by ed25519-dalek v2.2.0
  `from_bytes` (4-torsion, not rejected). Used `bad_pubkey[0] = 2` (y=2, QNR mod p).

**Group 2C — system-ledger doc updates + BENCHMARKS.md (`0881091`, Jennifer Woodfine)**
- `system-ledger/CLAUDE.md`, `NEXT.md`, `ARCHITECTURE.md` all updated from skeleton
  language to v0.2.1 fully-implemented state.
- `system-ledger/BENCHMARKS.md` created: 10 criterion benchmark results from
  `BENCH-v0.2.0.md`, run conditions, architectural observations.

**Group 2D — 3 new gap tests in system-ledger (`cb935f9`, Peter Woodfine)**
- `consult_with_bad_apex_pubkey_returns_inconsistent_state`: bad_pk[0]=2 (non-curve)
  → `ConsultError::InconsistentState`.
- `apply_witness_record_no_apex_returns_no_apex_for_checkpoint`: no genesis →
  `LedgerError::NoApexForCheckpoint`.
- `apply_witness_record_at_handover_height_succeeds`: 2-leaf Merkle tree, tree_size
  matches proof (must match for verify_inclusion_proof), handover path → Ok.
  Bug during development: original test used tree_size=50 but proof covered 2 leaves
  → `TreeSizeMismatch`. Fixed by setting checkpoint tree_size=2.
- Total: 47 tests (was 44).

**Group 2E — moonshot-toolkit ARCHITECTURE.md drift audit (`no-commit` — already applied)**
- Read `AUDIT-moonshot-toolkit-arch-vs-cli.md` (9 proposed edits) against current
  `ARCHITECTURE.md`. All 9 edits already applied in a prior session.
  No code changes needed; confirmed complete.

**Group 2F — clippy/fmt/rustdoc CI pass (`54fb7e7`, Jennifer Woodfine)**
- `cargo fmt`: fixed 5 diffs system-core, 9 diffs system-ledger, 4 diffs moonshot-toolkit.
- `cargo clippy -D warnings`: fixed `push_str("…")` → `push('…')` in moonshot-toolkit.
- `cargo doc --no-deps`: fixed 7 broken intra-doc links (3 system-core, 3 system-ledger,
  1 moonshot-toolkit) and 1 bare URL in system-core/checkpoint.rs.
- Final state: 0 warnings across all three crates for clippy, fmt, doc.
  139 tests passing (62 + 47 + 30).

> **Archived entries:** session logs before this point are in `cleanup-log-archive.md`.
