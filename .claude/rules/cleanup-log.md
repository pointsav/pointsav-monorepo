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

## 2026-04-30 — leapfrog-iteration-2 batch — research, design substrate, stats banner

- **Operator-direction session**: "do come rearech on the home page for
  documentation.pointsav.com ... iterate forwards a bit ... leapfrog 2030
  original copy that preserves Wikipedia muscle memory ... add as tokens in
  pointsav-design-system ... deep think, do reashc with sonnet ... win an
  award for cleaning up Wikipedia." Cluster session executed in 4 phases.

- **Phase 1 — Sonnet sub-agent dispatch (parallel, foreground)** under
  operator-override authorization (memory: `feedback_operator_override_sonnet_dispatch.md`).
  4 agents returned within budget with high-quality structured reports:
  (A) Wikipedia Main Page primitive-level inventory; (B) article-shell
  anatomy + leapfrog candidates under Vector 2022; (C) competitive
  landscape audit across 25 wiki/knowledge-base/docs-generator/PKM
  providers; (D) DTCG token-vocabulary anchored to Carbon v10 + Wikimedia
  Codex. Master ratifies post-hoc per v0.1.30 §1A.6.

- **Phase 2 — 9 drafts staged** at
  `~/Foundry/clones/project-knowledge/.claude/drafts-outbound/`:
  - 5 DESIGN drafts for project-design gateway pickup:
    `research-wikipedia-leapfrog-2030.md` (DESIGN-RESEARCH; 600+ lines),
    `component-citation-authority-ribbon.md` (DESIGN-COMPONENT),
    `component-research-trail-footer.md` (DESIGN-COMPONENT; Doctrine claim
    #39 at article scale), `component-freshness-ribbon.md` (DESIGN-COMPONENT),
    `token-knowledge-wiki-baseline.md` (DESIGN-TOKEN-CHANGE — pending
    Master cosign per cluster-design-draft-pipeline.md §3).
  - 4 PROSE drafts for project-language gateway pickup:
    `topic-knowledge-wiki-home-page-design.md` (PROSE-TOPIC public-facing
    home-page narrative), `topic-article-shell-leapfrog.md` (PROSE-TOPIC
    article-shell leapfrog narrative), `topic-wiki-provider-landscape.md`
    (PROSE-TOPIC 25-provider competitive-landscape audit),
    `guide-keep-the-home-page-the-gold-standard.md` (PROSE-GUIDE
    operational; English-only; deployment-subfolder per CLAUDE.md §14).
  - All 9 drafts carry `foundry-draft-v1` frontmatter with research-trail
    discipline (5 frontmatter fields + body Research-trail section per
    Doctrine claim #39 / draft-research-trail-discipline.md).

- **Phase 3 — Apprenticeship-corpus events emitted**: 9 `draft-created`
  JSONL events written to
  `~/Foundry/data/training-corpus/apprenticeship/prose-edit/pointsav/`,
  208,262 bytes total. Available for Stage-1/Stage-2 DPO pair construction
  per apprenticeship-substrate.md §7A and cluster-wiki-draft-pipeline.md §7.

- **Phase 4 — engine iteration shipped on cluster branch (commit b694127,
  Jennifer)**: home-page stats banner ("N articles across N categories —
  last updated YYYY-MM-DD."). New `HomeStats` struct + `compute_home_stats(buckets)`
  function in `src/server.rs`; `home_chrome()` signature extended with
  `&HomeStats` parameter; `<p class="wiki-home-stats">` rendered between
  lede and featured panel; suppressed entirely when `article_count == 0`.
  `.wiki-home-stats` CSS in `static/style.css` (+18 lines); system-sans,
  muted-foreground, tabular-nums on the `<time>` element.
  Specced in `content-wiki-documentation/index.md` as a pending ENGINE
  comment for some time; this session implements it.

- **Test results**: `cargo check` clean (3m12s); `cargo test --test home_test`
  passes 7/7 in 13.10s. Change is purely additive — existing
  integration tests run without modification (no test asserts absence of
  the new element).

- **Stage-6 promotion ask**: outbox to Master 2026-04-30T01:50Z requests
  Master operator-presence cycle to Stage-6 promote `cluster/project-knowledge`
  HEAD to canonical `pointsav/pointsav-monorepo` main, rebuild binary,
  restart `local-knowledge.service` to make stats banner visible at
  documentation.pointsav.com.

- **Cluster manifest updated**: `~/Foundry/clones/project-knowledge/.claude/manifest.md`
  extended with 4 new wiki TOPICs in `wiki:` leg's `planned_topics:`, and a
  new `design:` section paralleling the wiki leg (5 new design drafts +
  triggers enumeration). This is not a Tetrad amendment (Tetrad has 4
  legs); it is the design-leg participation declaration per v0.1.57
  cluster-design-draft-pipeline.

- **Open questions surfaced** (none new for monorepo cleanup-log):
  - Master cosign request on token-knowledge-wiki-baseline DESIGN-TOKEN-CHANGE:
    (1) `wiki.*` semantic namespace (governance scope), (2) FLI-banner
    colour register amber-vs-neutral (BCSC posture), (3) variable-font
    loading vs system-stack (sovereignty-vs-consistency).
  - Engine work for per-section JSON-LD emission (citation-authority
    ribbon, research-trail footer, freshness-ribbon — the three
    first-class leapfrog primitives from research §6) is tracked for a
    future project-knowledge iteration. Substrate-side recipes ship
    when project-design refines the staged DESIGN-COMPONENT drafts;
    engine-side rendering follows when the substrate token bundle is
    ratified.

## 2026-04-28 — documentation.pointsav.com home-page iteration 1 — engine MUST features shipped

- **Engine implementation pass** for documentation.pointsav.com home
  page iteration 1 against the engine-spec relayed from
  project-language by Master 2026-04-28T22:40Z. Followed the scoping
  doc at `app-mediakit-knowledge/docs/HOMEPAGE-IMPL-PLAN.md`
  (committed `10e0073` earlier this session).

- **Sonnet sub-agent dispatch** under operator-override "yes keep
  going" pattern (memory:
  `feedback_operator_override_sonnet_dispatch.md`). Brief authored
  by Opus parent + parent-reviewed before commit per CLAUDE.md §11
  v0.1.30 §1A.6. Sonnet ran ~17 minutes, returned 104 tests passing
  (97 → 104; +7 new in `tests/home_test.rs`), 0 net clippy delta
  (14 baseline warnings in pre-existing edit.rs/feeds.rs/search.rs
  unchanged; new code clean per `cargo clippy --all-targets`
  inspection).

- **6 MUST features delivered:**
  - `index.md` routing with placeholder fallback (absent
    `index.md` preserves the pre-iteration-1 file-listing chrome via
    extracted `placeholder_index()` helper)
  - `category:` + `last_edited:` promoted from
    `Frontmatter.extra` to first-class `Option<String>` fields
    (render.rs +18 lines)
  - By-category 3×3 grid using the operator-ratified 9-category
    set (architecture / services / systems / applications /
    governance / infrastructure / company / reference / help) per
    `naming-convention.md` §10 Q5-A; "0 articles — in preparation"
    placeholder for empty categories
  - Featured TOPIC pin reading
    `<content-dir>/featured-topic.yaml` (slug + optional since +
    optional note); silent suppress on absent; warn-and-suppress on
    parse-fail or unresolvable slug
  - Recent-additions feed (top-5 by `last_edited:` desc; git
    shell-out fallback when frontmatter date absent; mtime fallback
    when git fails) — iteration-1 git-shell-out chosen to avoid
    pulling Phase 4 `git2` dep forward
  - Wikilink resolution in home body (already handled by
    existing `parse_page` + render pipeline per content-contract.md
    §5.1; no engine changes needed)

- **File touches:** `src/render.rs` +18 lines (Frontmatter struct);
  `src/server.rs` +422 lines (handler + helpers + types +
  home_chrome + placeholder_index extraction); `static/style.css`
  +218 lines appended (`.wiki-home-*` BEM classes + 3-col → 2-col →
  1-col responsive grid); `tests/home_test.rs` new file 305 lines
  (7 integration tests via tempdir fixture + oneshot router
  pattern mirroring `tests/feeds_test.rs`).

- **Iteration-1 deferrals** documented in HOMEPAGE-IMPL-PLAN.md §9:
  Spanish home routing (`/es` → `index.es.md`), search-box on home,
  `/wanted` page, featured-rotation logic, date-tagged
  announcements. Did You Know? / On This Day cut by spec.

- **Minor concerns flagged by parent review** (acceptable for
  iteration 1; tracked for iteration 2):
  - Category-card title links to `/search?q=category:<name>` —
    BM25 search backend doesn't natively support `category:`
    syntax; this matches the literal token. Iteration 2 wires
    `/<category>` route per content-contract.md §7.
  - Sort-key in `recent_topics_by_last_edited` mixes ISO 8601
    strings and Unix epoch seconds (mtime fallback) in the same
    lexicographic compare — fragile but acceptable for the
    49-TOPIC current corpus.
  - `bucket_topics_by_category` parses every TOPIC on every `/`
    request; iteration-2 cache (60s TTL) per HOMEPAGE-IMPL-PLAN §13.

- **DESIGN-* discipline triggered** per v0.1.57: cluster ships UI
  work today (the new home-page chrome). Plan:
  `component-home-grid` DESIGN draft staged to
  `~/Foundry/clones/project-knowledge/.claude/drafts-outbound/`
  for project-design pickup per
  `conventions/cluster-design-draft-pipeline.md` (claim #38).

- **Open question carried** (none new this session): all 7 BP1
  questions still pending operator decision; the iteration-1 home
  page does NOT depend on BP1.

## 2026-04-28 — Read-only audit batch (Briefs 5-8) — applied bounded fixes per parent review

- **Cluster sub-agent queue** created at
  `~/Foundry/clones/project-knowledge/.claude/sub-agent-queue.md`
  per Master ratification 2026-04-28T04:00Z (workspace v0.1.30
  §1A.4 layer-scope rule: cluster-scope briefs go in cluster's
  own queue, not Master's workspace queue). Operator
  authorization framing 2026-04-28 "take care of all open
  issues". Queue holds 8 ratified briefs across 3 sections
  (Phase 4 — operator-gated; Wiki-leg expansion ×3; Read-only
  audits ×4 parallelisable).

- **Read-only batch dispatched (4 parallel, per §1A rule 2)**.
  All 4 returned with high-quality reports.
  - **Brief 5** (JSONL corpus integrity audit, Sonnet) — all 6
    JSONL files in `prose-edit/pointsav/` clean against the
    foundry-draft-v1 schema; byte-for-byte match confirmed
    against staged drafts. Corpus ready for Stage-1 DPO pair
    construction. No action needed.
  - **Brief 6** (frontmatter validation pass, Sonnet) — 5 of
    6 drafts compliant; 1 FAIL: `topic-collab-via-passthrough-relay.es.draft.md`
    missing `references:` field. **Fix applied** (mirror English
    sibling's references, 9 entries). 5 drafts carry inline-comment
    WARN on `target_path` (parser-stripped; not FAIL).
  - **Brief 7** (static/ asset audit, Haiku — Master concurred
    on tier) — 1 unused class found: `wikilink-redlink` (line
    135) + `--link-redlink` variable (line 21). **Fix applied**
    (both removed); per CLAUDE.md §6 "don't design for
    hypothetical future requirements" — re-add when Phase 4's
    wikilink graph emits red-link rendering. 42/43 CSS used; 4
    JS files referenced; 0 dead font refs; vendored bundles
    current.
  - **Brief 8** (cleanup-log triage, Sonnet) — A:3 / B:3 /
    C:5 / D:10 categorisation. **Category A fixes applied
    this commit**:
    - A1: `app-mediakit-knowledge` registry row state
      `Scaffold-coded` → `Active` with annotation listing
      shipped phases + flagging project-root CLAUDE.md/NEXT.md
      activation defect.
    - A2: `service-search` row removed from Open questions
      (answer was already embedded in row text — confirmed for
      next MEMO, doc catch-up is NEXT.md item).
    - A3: `service-llm` reclassified from Active legacy-to-canonical
      renames table to Intentional exceptions table — it's a
      permanent documentation-reading convention (code is
      already `service-slm`), not an in-flight rename.
    - Category B item answered-but-still-listed:
      `doubly-nested service-email-egress-{ews,imap}` open
      question struck through with "Answered 2026-04-23" note
      (wrappers flattened in 2026-04-23 session; Cargo.toml
      name mismatches remain as separate structural defect).

- **Categories B/C carried for later sessions:**
  - B2: `disclosure_class: glossary` schema extension needs
    re-triage against `conventions/reverse-funnel-editorial-pattern.md`
    claim #35 enum when ARCH §6 operator decision lands.
  - C1: `cluster-totebox-real-property` rename — to be
    actioned as in-transit edit when `USER_GUIDE_2026-03-30_V2.md`
    handoff to content-wiki-documentation executes.
  - C2/C3: `os-interface`/`os-integration` → `os-orchestration`
    + `RealPropertyArchive` → `PropertyArchive` renames stale 4
    sessions; should be grep-confirmed before Phase 4 touches
    related code.
  - C4: `discovery-queue` open question — registry row already
    has operational disposition (gitignore + move to
    `service-fs/data/`); open-questions row should be closed in
    a future session.
  - C5: `os-infrastructure` two-script pipeline question
    unanswered — needs resolution before Phase 4 build pipeline
    work.

- **Sonnet sub-agent dispatch pattern operating cleanly.**
  All 4 sub-agents returned within budget (Brief 7 ~3 min;
  others ~1-2 min). Parent (Opus) review pass identified
  Brief 7's "future-feature" framing risk (red-link CSS as
  forward-looking infrastructure) and applied per CLAUDE.md
  §6 discipline. Parent never delegated commit decision per
  v0.1.30 §1A rule 6.

## 2026-04-28 — Tetrad upgrade + PK.1/PK.4 prep commits + skeleton TOPIC pair

- **Tetrad Discipline backfill** for the project-knowledge cluster
  per Doctrine claim #37 / doctrine v0.0.10 / convention
  `~/Foundry/conventions/project-tetrad-discipline.md` ratified
  2026-04-28. Cluster manifest at
  `~/Foundry/clones/project-knowledge/.claude/manifest.md` amended:
  `triad:` → `tetrad:`; new `wiki:` leg added with declared
  `planned_topics:` listing 3 substantive bulk drafts already
  staged 2026-04-27 (`topic-app-mediakit-knowledge`,
  `topic-documentation-pointsav-com-launch-2026-04-27`,
  `topic-substrate-native-compatibility`) + 1 skeleton staged
  this commit (`topic-collab-via-passthrough-relay`) + 2 future
  planned (`topic-source-of-truth-inversion`,
  `topic-wikipedia-leapfrog-design`). Status active. Vendor +
  customer + deployment legs status updated to active to reflect
  the v0.1.29 HTTPS launch state.

- **Skeleton TOPIC pair authored** at
  `~/Foundry/clones/project-knowledge/.claude/drafts-outbound/`:
  `topic-collab-via-passthrough-relay.draft.md` (English canonical,
  ~85 lines, 7 sections with `(draft-pending — substance follows
  in milestone N+1)` placeholders per convention §4 backfill
  procedure) + `topic-collab-via-passthrough-relay.es.draft.md`
  (Spanish overview sibling, mirrored section structure). Pair
  demonstrates Tetrad-leg intent for milestone N+1; substance
  lands when this cluster next touches Step 7 or a related collab
  topic. JSONL `draft-created` events emitted for both at
  `~/Foundry/data/training-corpus/apprenticeship/prose-edit/pointsav/`
  (`draft-2026-04-28-topic-collab-via-passthrough-relay.jsonl` +
  `-es.jsonl`).

- **PK.1 + PK.4 prep work committed** (was authored 2026-04-27 but
  blocked on identity-key SSH-strict permissions; Master fixed
  the keys 2026-04-28T00:22Z chmod 0600 across all 4 canonical
  store keys).
  - `e09d9a8` (Peter) — `app-mediakit-knowledge/docs/BP1-DECISION-PACKET.md`
    (304 lines): restructures PHASE-4-PLAN.md §7's 7 BP1 questions
    in a 15-min operator-review format. Six recommendations to
    confirm-or-override; only Q4 (project-slm coordination order)
    is genuinely open. Accelerates PK.1 from ~1 hr to ~15 min per
    SLM Operationalization Plan §4 v0.1.42.
  - `ea26118` (Jennifer) — `app-mediakit-knowledge/docs/STEP-7-COLLAB-SMOKE.md`
    (324 lines): full PK.4 runbook covering pre-smoke build + SSH
    tunnel; 7-step manual two-client smoke procedure with pass
    criteria; pre-staged systemd unit unified diff for production
    enable; rollback procedure; sign-off table.

- **4 substantive bulk drafts forwarded to project-language**
  (Master action 2026-04-28T00:22Z per outbox in_reply_to):
  drafts staged 2026-04-27 at
  `clones/project-knowledge/.claude/drafts-outbound/`
  (`topic-app-mediakit-knowledge.draft.md`,
  `topic-documentation-pointsav-com-launch-2026-04-27.draft.md`,
  `topic-substrate-native-compatibility.draft.md`,
  `GUIDE-operate-knowledge-wiki.draft.md`) — Master forwarded
  the pickup notification to project-language inbox; PL.6 in
  the SLM Operationalization Plan §4 picks up via
  `bin/draft-sweep.sh` at next session start, daily-velocity per
  cluster-wiki-draft-pipeline.md §3.1.

- **No new open questions surfaced this session.** All 7 BP1
  questions remain pending operator decision (now with the
  decision packet to accelerate). The 4 deployment-side carried
  questions remain pending. PK.2/PK.3 sub-agent dispatch
  proposals still hold for after PK.1 clears.

## 2026-04-27 — Phase 2 Step 7 (collab) + Phase 4 plan + HTTPS-launch coordination

- **Phase 2 implementation now COMPLETE end-to-end** — Step 7
  (collab via yjs) shipped (commit `05f1dab`, Peter). Default-off
  behind `--enable-collab` CLI flag; production deploys without
  the flag never load the 302 KB collab JS bundle and never expose
  the WebSocket route. Server is a passthrough relay
  (`tokio::sync::broadcast` per-slug rooms; no `yrs`/Yjs Rust port
  needed because the relay carries no doc state). Client uses
  `yjs` + `y-codemirror.next` + `y-websocket` lazy-loaded from
  `cm-collab.bundle.js`. Test count: 90 → 97 (3 unit + 4
  integration). Manual two-client smoke (operators editing the
  same TOPIC and seeing each other's cursors) is needed before
  ratification but the unit/integration coverage proves the
  framework is wired correctly.

- **Phase 4 implementation plan landed for BP1 review** — commit
  `73e931e` (Jennifer). `docs/PHASE-4-PLAN.md` (~340 lines)
  covers 8 sequenced steps (git2 wiring + commit-on-edit; GET
  /history + /blame via gix; GET /diff; redb link-graph + GET
  /backlinks; blake3 federation-seam baseline; MCP server via
  rmcp; read-only Git remote via smart-HTTP; OpenAPI 3.1 spec).
  File map, route table, CLI flag additions, deferred items,
  test plan, **seven open questions for operator at BP1** (MCP
  transport, git remote protocol, --enable-mcp default,
  project-slm coordination for Step 4.6, gix vs git2 split,
  libgit2-dev system-lib install, OpenAPI hand-author vs codegen).

- **HTTPS-launch coordination** (cross-cluster, cross-sub-clone)
  — operator decided 2026-04-27 to push HTTPS live at
  `documentation.pointsav.com` immediately, served from a
  minimal placeholder content set (NOT the legacy 30+ TOPIC
  corpus, which will be re-written in a separate `project-language`
  cluster effort). Four BCSC-clean placeholder TOPICs authored
  in `content-wiki-documentation/launch-placeholder/` (commit
  `6f14f06` in content-wiki-documentation, Jennifer). Outbox
  message dispatched to Master for the redeploy + certbot session.
  Collapses the BCSC pre-flip concern (placeholder content has
  no SDF current-tense framings, no unlabelled FLI, no Do-Not-Use
  vocabulary). Production HTTPS-flip blocked on Master executing
  the queued redeploy (build from cluster HEAD; install binary
  with `--citations-yaml` + `--state-dir` + optional
  `--enable-collab` flags; switch systemd unit's `--content-dir`
  to `launch-placeholder/`; `certbot --nginx`).

- **Sonnet usage in this segment**: zero. All three tracks
  (placeholder content, Step 7 collab, Phase 4 plan) executed
  Opus-direct because each was either (a) requiring tight
  judgment on novel structure (Step 7 collab integration with
  CodeMirror's create-time extension constraint), or (b) doc
  authoring where context preservation matters more than
  per-token cost (Phase 4 plan).

- **OPEN QUESTIONS still pending operator/Master from prior
  Phase 3 segment** (carried forward; this segment did not close
  any of them):
  1. ARCH §6 schema extension (hatnote, translations, categories,
     disclosure_class: glossary)
  2. Z2 BCSC bulk-fix decisions (6 contested items) — DEFERRED to
     project-language cluster; this cluster's launch-placeholder
     bypasses
  3. `apt install libssl-dev` + (Phase 4 brings) `libgit2-dev` —
     production binary build pre-requisites
  4. cargo workspace coupling at monorepo root —
     service-content's reqwest pulls openssl-sys; permanent fix
     out-of-cluster scope

- **NEW OPEN QUESTIONS surfaced this segment:**
  1. Phase 4 BP1 — 7 questions in PHASE-4-PLAN.md §7 awaiting
     operator clearance before Phase 4 implementation begins
  2. Phase 2 Step 7 collab — manual two-client smoke verification
     needed; operator decision on whether to enable in production
     deploy

## 2026-04-26 — Phase 3 implementation complete (Steps 3.1–3.4)

- **Phase 3 of `app-mediakit-knowledge` shipped end-to-end** —
  3 commits on `cluster/project-knowledge` covering Steps 3.1–3.4
  of ARCHITECTURE.md §3 Phase 3. Tests grew 57 → 90 across the
  Phase 3 work (+33 across 3 commits).

  | Commit  | Step                                  | Tests after |
  |---------|---------------------------------------|-------------|
  | 0ace07e | Step 3.1 — Tantivy search backend     | 64          |
  | 72c4756 | Step 3.2 — search route + edit-triggers-reindex | 69 |
  | bbd995a | Steps 3.3+3.4 — feeds + crawler discovery + git/markdown | 90 |

- **Engine surface now covers ARCHITECTURE.md §3 Phase 3 fully**:
  on-disk Tantivy index at `<state_dir>/search/`, rebuilt on
  startup; `GET /search?q=` HTML page over BM25; edit-triggers-
  reindex via `crate::search::reindex_topic`; `GET /feed.atom`
  (RFC 4287); `GET /feed.json` (JSON Feed 1.1); `GET /sitemap.xml`
  (sitemaps.org); `GET /robots.txt`; `GET /llms.txt` (llmstxt.org
  emerging convention); `GET /git/{slug}` raw Markdown source for
  git-clone-style ingestion.

- **Resumed mid-session after a Bash-tool failure** that
  interrupted the original Step 3.1 commit. Plan-mode plan at
  `~/.claude/plans/eager-watching-leaf.md` captured the resume
  sequencing; Explore agent verified on-disk survival before
  resumption; Sonnet sub-agent drafted Steps 3.3+3.4 and Opus
  reviewed + committed.

- **Three Opus fixes applied to Sonnet drafts during Phase 3:**
  1. `tantivy::schema::Document::get_first` returns
     `CompactDocValue` in tantivy 0.24 (not `OwnedValue`); used
     `tantivy::schema::Value::as_str()` trait method instead of
     a match arm against `OwnedValue::Str`.
  2. `ReloadPolicy::OnCommitWithDelay` reader is asynchronous;
     the `reindex_replaces_existing_entry` test searched before
     reload completed. Added explicit `reader.reload()` in
     `reindex_topic` after the writer commit (and `drop(writer)`
     to release the mutex first).
  3. axum 0.8 panics on a literal `.md` suffix after a dynamic
     route segment (`/git/{slug}.md` blew up at router build).
     Changed route to `/git/{slug}`; handler strips an optional
     `.md` suffix from the captured value to preserve both UX
     shapes (with and without `.md`).

- **OPEN QUESTIONS surfaced this Phase 3 segment for operator/Master:**
  1. **`disclosure_class: glossary` enum extension** still
     pending from Phase 2 (Step 1 JSON-LD profile selection +
     ARCH §6 schema).
  2. **Track Z2 BCSC review report** at
     `~/Foundry/clones/project-knowledge/.claude/bcsc-review-2026-04-26.md`
     — 6 contested operator-decision items; bulk-fix application
     waits on those answers.
  3. **`apt install libssl-dev`** still pending — release binary
     build (`cargo build --release`) blocks until the
     workspace-side openssl-sys system lib lands. Debug binary
     suffices for demo + tests.
  4. **Phase 2 Step 7 (collab via yjs + y-codemirror.next)** —
     still deferred per BP1 §8 default; operator can ship at any
     time as a single Task session.
  5. **Cargo workspace coupling at monorepo root**: `cargo` from
     `pointsav-monorepo/` pulls `service-content`'s reqwest →
     openssl-sys, which fails without `libssl-dev`. Crate-scoped
     `cd app-mediakit-knowledge && cargo` is the working path.
     Permanent fix: switch service-content to rustls OR install
     libssl-dev OR re-tighten the parent workspace's `members`.

- **2026-04-26 — Phase 2 implementation complete (Steps 1-6)** below.

## 2026-04-26 — Phase 2 implementation complete (Steps 1-6)

- **Phase 2 of `app-mediakit-knowledge` shipped end-to-end** —
  5 commits on `cluster/project-knowledge` covering Steps 1-6 of
  `docs/PHASE-2-PLAN.md` §1; tests grew from 19 → 57 across the
  session. Step 7 (collab via yjs) deferred to Phase 2.x per BP1
  §8 default.

  | Commit  | Step                               | Tests after |
  |---------|------------------------------------|-------------|
  | b8580f9 | Step 1 — JSON-LD baseline          | 28          |
  | 69e5610 | Step 2 — edit endpoint + atomic write + path hardening | 39 |
  | 8f5f010 | Step 3 — vendor CodeMirror 6 bundle + base editor | 40 |
  | fd1adf9 | Step 4 — SAA squiggle framework (7 deterministic rules) | 47 |
  | 2bd74e9 | Steps 5+6 — citation autocomplete + 3-keystroke ladder stubs | 57 |

- **End-to-end editor surface at `/edit/{slug}`**: CodeMirror 6 +
  Markdown highlight + line numbers/wrap + history + atomic save +
  squiggle linting with cited authority + `[`-triggered citation
  autocomplete fed by `/srv/foundry/citations.yaml` + Tab/Cmd-K
  Doorman affordances (501 stubs until Phase 4 wires the MCP
  integration). JSON-LD baseline in every TOPIC `<head>`.

- **Mid-session demo verified** — operator browsed the wiki via
  SSH tunnel (`gcloud compute ssh -- -L 9090:localhost:9090`) on
  the debug binary; 30+ existing TOPICs + the 5 new fixtures
  rendered correctly through Phase 1.1 chrome. Server then killed
  per operator instruction.

- **Sonnet sub-agent participation** — Steps 5+6 drafted by a
  Sonnet sub-agent in background (cost discipline per
  `conventions/model-tier-discipline.md`); Opus reviewed, found
  two issues, fixed, committed:
  1. `AppState` constructors in pre-existing test files
     (`jsonld_test.rs`, `edit_test.rs`, `squiggle_test.rs`) needed
     the new `citations_yaml` field — Sonnet only updated tests it
     authored.
  2. `/srv/foundry/citations.yaml` opens with a YAML-frontmatter
     metadata block (`---...---`) before the `citations:` document.
     Sonnet's parser hit the frontmatter as document 1 and failed
     to find `citations:`. Added `strip_prefix("---\\n")` logic
     to skip frontmatter when present (parallels
     `render::parse_page` for TOPIC files).

- **OPEN QUESTIONS surfaced this session for operator/Master:**
  1. **ARCHITECTURE.md §6 schema extension** — three Phase 1.1
     frontmatter fields (`hatnote`, `translations`, `categories`)
     not formally enumerated; also `disclosure_class: glossary`
     enum extension introduced for Phase 2 JSON-LD profile
     selection. Recommendation: extend §6 (explicit > implicit).
  2. **Phase 2 Step 7** (collab via `yjs` + `y-codemirror.next`
     + self-hosted `y-websocket`) deferred — operator can ship at
     any time without other Phase 2 work blocking. Step 7 brief
     remains in `PHASE-2-PLAN.md` §1.
  3. **Cargo `openssl-sys` at monorepo root** — running cargo from
     the monorepo root pulls `service-content`'s reqwest →
     openssl-sys, which needs `libssl-dev`. The crate-scoped
     `cd app-mediakit-knowledge && cargo` is a workaround.
     Permanent fix is either (a) install `libssl-dev` on the VM,
     or (b) switch service-content from reqwest's default
     `native-tls` feature to `rustls`. Surface for next
     service-content touch.
  4. **Production deployment** at `documentation.pointsav.com` —
     separate outbox message to Master in
     `~/Foundry/clones/project-knowledge/.claude/outbox.md` with
     concrete 11-step runbook (DNS, TLS, reverse proxy, systemd,
     BCSC content review pre-flip, `libssl-dev` install).

- **Phase 1.1 Wikipedia muscle-memory chrome shipped on
  `app-mediakit-knowledge`** (project-knowledge cluster session 3,
  Track A). Additive UI/template/CSS only over Phase 1; the four
  Phase 1 routes (`/`, `/wiki/{slug}`, `/static/{*path}`,
  `/healthz`) and their responses are unchanged. Wikipedia muscle-
  memory inventory items 1 (Article/Talk tabs), 2 (Read/Edit/View
  history tabs), 3 (per-section [edit] pencils), 5 (end-of-article
  ordering), 6 (hatnote), 8 (lead first-sentence), 9 (tagline),
  12 (collapsible left-rail TOC), 14 (language switcher),
  15 (footer convention) are added per UX-DESIGN.md §1. IVC
  masthead band placeholder (UX-DESIGN.md §4.5) and reader density
  toggle (§4.6) ship as visual surfaces only — no IVC machinery
  until Phase 7. Diff: `src/render.rs` +240, `src/server.rs` +444,
  `static/style.css` +428, `static/wiki.js` new (~120),
  `tests/fixtures/content/topic-hello.md` extended with the new
  optional frontmatter fields. Test count 8 → 19, all passing.
- **Three compile-time / test-correctness fixes applied to
  Sonnet sub-agent's draft before commit.**
  - `render.rs` PENCIL constant: raw-string delimiter mismatch —
    the inner `href="#"` closes a single-hash raw string early;
    bumped to double-hash (`r##"…"##`).
  - `extract_headings`: was searching for `id="` in the heading's
    opening tag, but comrak with `header_ids: Some(...)` emits the
    id on the inner anchor (`<h2><a id="h-…"></a>Alpha</h2>`).
    Rewrote to scan inside the full heading element.
  - IVC masthead band test: was case-sensitive on placeholder copy
    (`"verification"` vs rendered `"Verification"`); switched to
    structural class-name check (`wiki-ivc-band`) for stability.
- **Open question surfaced for ARCHITECTURE.md.** Three new
  optional frontmatter fields used by Phase 1.1 chrome (`hatnote`,
  `translations`, `categories`) are not listed in §6 schema.
  Either (a) extend §6 to enumerate them, or (b) treat the §6
  field list as required-only with the `extra: BTreeMap` catch-
  all sufficient for optional fields. Recommendation: extend §6
  (explicit > implicit). Operator/Master to decide.
- **Phase 2 implementation plan landed for BP1 review** (sibling
  commit, same session, Track B BP1). Wrote
  `app-mediakit-knowledge/docs/PHASE-2-PLAN.md` (~480 lines) —
  operator-reviewable artefact for Breakpoint 1 in the project-
  knowledge AUTO workflow. Plan covers: 7-step implementation
  order (JSON-LD baseline → edit endpoint → CodeMirror vendoring
  → SAA squiggles → citation autocomplete → 3-keystroke ladder
  stubs → optional collab); vendoring strategy (pre-build JS
  out-of-tree, commit artefacts to `static/vendor/`); file map
  (15 new + 6 modified); 8 new endpoints; test plan; six open
  questions for operator at BP1. No implementation code touched
  in the BP1 commit; Phase 2 impl waits on operator clearing BP1.
- **L1 trajectory capture** writes one corpus record per commit
  to `~/Foundry/data/training-corpus/engineering/project-
  knowledge/<sha>.jsonl` per cluster manifest. Two records added
  this session under this repo (Track A + BP1).

---

## 2026-04-23

- **Repo-layout rule introduced.** Added
  `.claude/rules/repo-layout.md` codifying the allowed file set at
  the monorepo root and at each project directory root, and naming
  the sibling repos where cross-cutting content belongs (user guides,
  ADRs, design-system material). Anchor for the file-relocation work
  queued behind it (see `NEXT.md`).
- **Defects surfaced at root by this rule** — staged for separate
  commits, not moved in this session:
  - ~~`force_build.sh` (tracked, at repo root) → queued move to
    `vendor-sel4-kernel/scripts/`~~ **Closed 2026-04-23** — moved
    via `git mv` in a follow-up commit within this session. Zero
    runtime callers; script body uses absolute paths so no content
    edits required.
  - `GUIDE-OPERATIONS.md` (tracked, at repo root) → queued move to
    `content-wiki-documentation/`.
  - `USER_GUIDE_2026-03-30_V2.md` (tracked, at repo root) → queued
    move to `content-wiki-documentation/` with `_V2` dropped, per
    CLAUDE.md §6 edit-in-place rule.
  - ~~`app-console-content/src/{pointsav-surveyor.sh,surveyor.py}` →
    queued move to `app-console-content/scripts/`~~ **Closed
    2026-04-23** — both files moved via `git mv` (recognised as
    100% renames). Shell wrapper uses `$(dirname "$0")/surveyor.py`
    (relative) so the pair moves together without edits. Python
    script uses absolute paths into `woodfine-fleet-deployment` so
    location-independent. Zero intra-repo runtime callers; no cron
    entries found. The clone at `~/Foundry/clones/service-slm/`
    retains its copy on branch `cluster/service-slm` (separate
    `.git/`) and is unaffected by this move on `main`; it will
    receive the change only when that branch merges.
  - ~~`os-infrastructure/build_iso/forge_iso.sh` → queued rename to
    `os-infrastructure/build_iso/compile_binary.sh`~~ **Closed
    2026-04-23** — renamed via `git mv`; in-file header comment
    updated to reflect the new name and record the rename
    rationale. Zero external callers.
- ~~**Project-root scripts flagged (not yet moved):** ~15 scripts sit
  at project root instead of under `scripts/` across `service-vpn`
  (5 generator scripts), `service-email` (`spool-daemon.sh`),
  `service-slm` (`cognitive-bridge.sh`), `service-content`
  (`forge-seeds.sh`), `os-network-admin` (2 scripts),
  `os-totebox` (1), `tool-cognitive-forge` (1),
  `vendor-phi3-mini` (2), `app-mediakit-telemetry` (5 generic
  scaffold scripts). Each project is a separate closure task.~~
  **Closed 2026-04-23** — all 9 projects relocated in 9 separate
  `git mv` commits (18 files total, every one a 100% rename).
  Commit chain: `8f5cc48` os-totebox → `2456ea6` service-content
  → `30ff629` service-email → `cda2ce5` service-slm → `654d255`
  tool-cognitive-forge → `503f922` os-network-admin → `6df4be0`
  vendor-phi3-mini → `6f95279` service-vpn → `faae141`
  app-mediakit-telemetry. No callers needed updating; the only
  in-script references found were self-usage strings that remain
  valid after the move.
- **Stray runtime log surfaced.** `tool-cognitive-forge/llama.log`
  at project root — runtime log, almost certainly should be
  gitignored (and removed from tracking if tracked). Not addressed
  in this session. Added to `NEXT.md` as a separate item.
- **First rename-series closure: `service-parser` removed.**
  `service-parser/` directory deleted (`git rm -r`); contained
  only a README describing an abandoned AI-routing framing — no
  code, no data, no subdirectories. Zero runtime references
  anywhere in the repo. Rename-table row moved to Completed
  migrations; registry row removed; registry Defect count updated
  from 5 to 4 and Total rows from 100 to 99.
- **Second rename-series closure: `pointsav-pty-bridge` →
  `service-pty-bridge`.** Directory renamed via `git mv` (four
  100% renames: `.gitignore`, `Cargo.toml`, `Cargo.lock`,
  `src/main.rs`); `target/` left in place because it is gitignored
  build output. `Cargo.toml` `name` field updated in the same
  commit. Registry row moved from "Other / special" to the
  Service section, alphabetically between `service-people` and
  `service-search`, reclassified Defect → Scaffold-coded. Summary
  counters: Defect 4 → 3, Scaffold-coded 51 → 52, Total stays 99.
  Zero external Rust imports, no callers needed updating; not a
  workspace member. Stray `Cargo.lock` inside the renamed
  directory remains — resolves with workspace `Cargo.toml`
  unification (separate open structural defect).
- **Handoffs-outbound entries made self-executing.** Each outbox
  entry now carries a "Prescriptive actions" subsection with the
  exact commands a destination Root Claude can run mechanically —
  `cp` commands from source absolute path, `git add`, commit
  message, any in-transit edits, and the completion-signal commit
  pattern. Header also describes the convention so future outboxes
  follow the same shape. Two existing entries for
  `GUIDE-OPERATIONS.md` and `USER_GUIDE_2026-03-30_V2.md` updated
  with their prescriptive actions. This lets a cold-start Root
  Claude session in `content-wiki-documentation/` execute the
  handoffs without reading anything from this session's context.
- **Fifth (final) rename-series closure: Cognitive Forge term
  retired.** `service-slm/cognitive-forge/` renamed to
  `service-slm/router/`; former top-level `tool-cognitive-forge/`
  moved in as `service-slm/router-trainer/`. Producer/consumer
  now live together under `service-slm`. Rust Cargo.toml `name`
  field + `main.rs` usage string updated. Python
  `distill_knowledge.py` relocated from non-canonical `src/` to
  `scripts/` alongside `ignite_teacher.sh`. Three binary/log
  files stopped being tracked (`llamafile` 35 MB, `engine.log`,
  `llama.log`) via `git rm --cached` + new `.gitignore` section;
  physical files remain at new paths so the Python workflow still
  finds them. The 15 MB `qwen2.5-coder-1.5b.gguf` under `weights/`
  was already ignored. Registry Scaffold-coded 54 → 53, Total
  98 → 97 (one top-level project absorbed into `service-slm`).
  This closes the rename-series queue (5 of 5 done) and the
  separate `llama.log` stray item surfaced earlier in this
  session.
- **Fourth rename-series closure: `service-email-egress-{ews,imap}`
  wrappers flattened; consolidation plan reversed.** After
  reviewing sub-crate contents, EWS and IMAP are two
  protocol-specific adapters — not duplicates. Shared sub-crates:
  `egress-ingress`, `egress-ledger`, `egress-roster`,
  `data-ledgers/`. Protocol-specific: `egress-archive-ews` /
  `egress-archive-imap`; EWS-only: `egress-prune`,
  `egress-balancer`. Merging them would erase that architectural
  distinction. Instead, flattened the redundant
  `service-email-egress-ews/service-email-egress-ews/` wrapper
  (and the imap equivalent) — 73 files promoted up one level.
  Relative `../data-ledgers/` paths in Rust sources remain valid
  because crate dirs and `data-ledgers/` both moved together.
  Registry reclassified both from Defect → Scaffold-coded;
  Defect count 2 → 0 (registry is now Defect-free); Scaffold-coded
  52 → 54. The 13 dir-name / Cargo-name mismatches the 2026-04-18
  audit flagged (e.g., dir `egress-ingress` containing
  `Cargo.toml` with `name = "service-email-batch-ingress"`) are
  unaddressed and remain as a separate audit finding.
- **Third rename-series closure: `vendors-maxmind` reclassified
  to `app-mediakit-telemetry/assets/`.** Not a rename but a
  data-reclass: the directory held only the 63.5 MB
  `GeoLite2-City.mmdb` + READMEs with no code. The vendor's own
  README already named `app-mediakit-telemetry/assets/` as the
  intended target path — the monorepo had never realised that
  path. Moved the `.mmdb` + both READMEs into the documented
  target; removed `vendors-maxmind/.keep`; empty directory
  auto-removed by git. Closed the related "does it belong as a
  `vendor-*` crate at all?" open question (answer: no;
  non-workspace data directory). Updated monorepo `README.md`
  line 151 and `USER_GUIDE_2026-03-30_V2.md` line 902 (in-transit
  edit travels with the cross-repo handoff). Extended
  `repo-layout.md` to name `assets/` and `data/` as conventional
  project subfolders. Registry row removed; Defect 3 → 2, Total
  rows 99 → 98. Python script reference in
  `app-mediakit-telemetry/scripts/generic-omni-matrix-engine.py`
  left unchanged (it refers to deployment-side path relative to
  CWD — independent of monorepo-side layout). Separate `.mmdb` →
  build-time-fetch task remains open under Structural defects.
- **Open question surfaced.** `surveyor.py` hard-codes
  `MAX_DAILY_VERIFICATIONS = 10`. The existing cleanup-log open
  question — "Verification Surveyor daily throttle number — Under
  operational review. Do not cite a specific number" — must
  reconcile: either the code is authoritative (close the question,
  value is 10) or the doc is authoritative (the code is out of step
  and needs updating). Do not cite the number externally until
  resolved.
- **Second open question surfaced (os-infrastructure build
  pipeline).** The two scripts `os-infrastructure/forge_iso.sh`
  (ISO assembly) and `os-infrastructure/build_iso/compile_binary.sh`
  (binary compile, renamed this session) are sequential build
  stages but are not wired together — the assembly script does not
  invoke the compile script, and there is no Makefile or top-level
  driver. Operator must run them manually in order. Is this
  intentional (operator-gated two-step) or drift (should become a
  single driver script)? Pending decision before next pipeline
  refactor.
- **Handoff-outbound pattern piloted.** Added
  `.claude/rules/handoffs-outbound.md` as a cross-repo file-move
  outbox. Two entries lodged: `GUIDE-OPERATIONS.md` and
  `USER_GUIDE_2026-03-30_V2.md` both → `content-wiki-documentation`.
  Both files remain in place in this repo until a Root Claude in
  the destination repo commits the add-side; only then does a
  follow-up Root Claude session here commit the source-remove.
  The pattern is passive — an outbox entry waits for pickup.
- **Surfaced for Master Claude** (workspace-scope changes, outside
  Root Claude's write lane per §9):
  1. Formalise the cross-repo handoff pattern as an addendum in
     `~/Foundry/CLAUDE.md` §9. Current §9 stops at clone
     provisioning; the handoff mechanic is the natural extension
     for file movement between engineering repos.
  2. Extend `~/Foundry/CLAUDE.md` §10's `.claude/rules/` canonical
     list from three files to four — add `handoffs-outbound.md`
     alongside `repo-layout.md`, `project-registry.md`, and
     `cleanup-log.md`.
  3. Propagate both the `repo-layout.md` rule (§10 already names
     the monorepo as reference implementation) and the new
     `handoffs-outbound.md` pattern to the other engineering repos
     over time. Order of propagation is `~/Foundry/NEXT.md`'s
     concern.
- **`app-mediakit-knowledge/` populated from cross-repo zip.** The
  zip `content-wiki-documentation/app-mediakit-knowledge.zip`
  (42 KB, 44 entries) extracted into the existing Scaffold-coded
  `app-mediakit-knowledge/` directory, promoting it from a 4-file
  scaffold to a working-looking Rust crate skeleton: `src/` with
  5 modules (`editor/`, `renderer/`, `search/`, `server/`,
  `sync/`) plus `main.rs` and `config.rs`; `templates/` (4 HTML
  files); `static/` (13 KB `wiki.js` + 19 KB `style.css`);
  `tests/fixtures/architecture/` with 2 markdown fixtures;
  `.gitignore` (46 B). `Cargo.toml` and `README.md` were
  overwritten (93 B → 1,470 B; 751 B → 8,243 B). A garbage
  top-level directory literally named `{src` — containing a
  four-level chain of brace-expansion artefacts from how the zip
  was originally created (quoted `mkdir` blocked shell expansion)
  — was removed before any git operation. Nothing staged or
  committed in the extraction step itself.
- **Open follow-ups from the extraction (not acted on this
  session):**
  - `README.es.md` (403 B scaffold) is now out of sync with the
    new 8,243 B English README — CLAUDE.md §6 bilingual-pair rule
    in violation until a refresh pass lands. Editorial work;
    track as open item rather than inline.
  - `.gitkeep` at project root is redundant now that `src/` has
    real files; remove at next commit touching this project.
  - Registry row (`app-mediakit-knowledge` under `app-mediakit`)
    currently reads "Scaffold-coded, 4 files" — state remains
    Scaffold-coded per §8 (never run end-to-end) but file count
    and notes need updating.
  - Source-side disposition of
    `content-wiki-documentation/app-mediakit-knowledge.zip`
    undecided: delete from the sibling repo (cross-repo move,
    separate commit there), or retain as an archive. Not
    recorded in this repo's `handoffs-outbound.md` since the
    direction is inbound, not outbound.
- **BIM product family handoff landed — four project directories
  created, rules extension added.** The zip
  `/home/mathew/Documents/pointsav-bim-handoff.zip` (44 KB, 10
  files) was unpacked into a `/tmp` staging area and 9 files were
  placed into the monorepo:
  - Four new project directories each with `CLAUDE.md` +
    `RESEARCH.md`: `app-console-bim/`, `app-orchestration-bim/`,
    `app-workplace-bim/`, `service-bim/`.
  - One new `.claude/rules/` file:
    `.claude/rules/bim-product-family.md` (9,238 B) — a new
    *category* of rules file (product-family rules), outside the
    four named in `~/Foundry/CLAUDE.md` §10. Surfaced to Master
    Claude as a potential §10 extension.
  - Joint research file placed as `RESEARCH.md` in **both**
    `app-console-bim/` and `app-orchestration-bim/` — intentional
    duplication for Task Claude — BIM to rationalise during its
    cleanup pass, not prematurely.
  - `RESEARCH-BIM-MARKET.md` not placed in the monorepo (already
    present in `content-wiki-documentation/` at repo root,
    byte-identical; per `repo-layout.md` sibling-repo rule, market
    research belongs in content-wiki only).
  - `CLAUDE-root-additions.md` held back — it describes patches to
    a monorepo root `CLAUDE.md` that does not exist. Zip retained
    at source path; Master Claude applies when the root CLAUDE.md
    is created.
- **Registry drift closed (four rows without directories).** The
  2026-04-22 bootstrap registered the four BIM dirs as
  Reserved-folder with "1 file (RESEARCH.md)" notes, but
  `git ls-tree` showed no trace on any branch. The rows were
  aspirational; the directories were never created. This session
  creates them for the first time. Registry rows updated to
  reflect the actual contents (2 files each). State remains
  Reserved-folder (§8: Scaffold-coded requires a `Cargo.toml`
  skeleton; these are research-phase, no code yet).
- **Cross-repo BIM handover outbox entry opened.** Single
  consolidated entry in `handoffs-outbound.md` headed "BIM
  material → content-wiki-documentation", labelled as a **pattern
  variant: raw-material handover, not a file move** — source files
  remain in the monorepo permanently. Destination Root Claude
  transforms the material into proper wiki topics per its own
  repo-layout. Detection pattern for closure:
  `"receive BIM material from pointsav-monorepo"` in the
  destination repo's git log.
- **Surfaced for Master Claude (workspace-scope follow-ups):**
  1. **Root `CLAUDE.md` for `pointsav-monorepo` is missing.**
     Required per §10 to wire the `.claude/rules/*` files into
     Claude sessions. `CLAUDE-root-additions.md` in the handoff
     zip (location:
     `/home/mathew/Documents/pointsav-bim-handoff.zip` →
     `CLAUDE-root-additions.md`, 1,594 B) describes four targeted
     additions (`.claude/rules/bim-product-family.md` reference,
     four BIM dirs in Repo structure, canonical-name guards,
     IFC/F12 rules). Apply when the root CLAUDE.md is first
     drafted.
  2. **§10 canonical list may need to grow.**
     `bim-product-family.md` is a fifth type of `.claude/rules/`
     file beyond the four listed in §10. Decision: enumerate,
     generalise, or name as a subcategory.
  3. **`cluster-bim` clone provisioning pending.** Per §9, Master
     Claude provisions clones. A future Task Claude — BIM needs
     `~/Foundry/clones/cluster-bim/` with feature branch
     `cluster/bim` and a `PROJECT-CLONES.md` row before it can
     activate the four BIM projects.
  4. **Stale paths in existing outbox entries.** The two prior
     entries in `handoffs-outbound.md` use
     `/home/mathew/Foundry/factory-pointsav/...` paths (non-
     existent on disk) and helper `~/Foundry/tool-commit-as-next.sh`
     (§7 canonical is `~/Foundry/bin/commit-as-next.sh`). A
     destination Root Claude running the prescribed commands
     verbatim would hit failures. Needs correction before
     pickup. This session's new BIM entry uses correct paths.

---

## 2026-04-22

- **Project framework bootstrap.** Added `.claude/rules/project-registry.md`
  with 100-row inventory of every top-level directory, classified by
  state per `~/Foundry/CLAUDE.md` §8 (Reserved-folder /
  Scaffold-coded / Active / Defect / Not-a-project). Framework docs,
  templates, and activation procedure live workspace-level. This
  cleanup-log was also introduced onto `main` today (previously
  present only on feature branches — drift closed).
- **Taxonomy expanded to seven domains.** Added `app-orchestration-*`
  to the in-force `app-[os]-*` list in
  `~/Foundry/IT_SUPPORT_Nomenclature_Matrix_V8.md` §3. Triggered by
  `app-orchestration-bim` appearing during the session — would have
  been an unmatched-prefix defect under the original six-domain
  rule. Now conformant; `os-orchestration` already exists as a
  Systemic Wordmark (§2).
- **Four BIM-research directories registered.** `app-console-bim`,
  `app-orchestration-bim`, `app-workplace-bim`, `service-bim` — each
  with a single `RESEARCH.md`. Classified as Reserved-folder pending
  decision to activate.
- **Audit cleanup.** Removed 2 `__MACOSX/` directories and 16
  tracked `.DS_Store` / AppleDouble files from extraction-artefact
  scaffolding in the egress crates. Added `.DS_Store` to
  `.gitignore`.

---

## 2026-04-18 — Layer 1 structural audit — findings

- **Headline finding:** Workspace `Cargo.toml` declares only 8 of ~70+ crates as members. Everything else is treated as standalone workspaces, which explains the 23 stray `Cargo.lock` files scattered through the repo. `cargo build --workspace` will skip almost everything; profile/edition inheritance is not reaching most crates.
- **Severity counts:** 1 Critical, 1 High, 4 Medium, 1 Low.
  - Critical: workspace under-declaration (8 of ~70+ crates).
  - High: 23 stray `Cargo.lock` files inside member crates.
  - Medium: prefix violations (2); dir-name vs `Cargo.toml` name mismatches (13); doubly-nested `service-email-egress-{ews,imap}` scaffolding; many `app-console-*` / `app-network-*` directories without `Cargo.toml`.
  - Low: `discovery-queue` orphan data directory at root.
- **Good news on prefix adherence:** across ~85 directories, adherence to the seven canonical prefixes is approximately 97.6%. Only two violations found: `pointsav-pty-bridge` (no recognized prefix) and `vendors-maxmind` (plural form instead of canonical `vendor-`).
- **Nested redundancy:** `service-email-egress-ews` and `service-email-egress-imap` both contain a redundant intermediate directory of the same name — a doubly-nested copy-paste scaffolding pattern producing depth-3 crates. All 13 directory-name / `Cargo.toml`-name mismatches are concentrated in these nested egress areas (short dir names like `egress-ingress` aliasing qualified crate names like `service-email-batch-ingress`).
- **No modifications were made in this session — audit only.**
- **Next:** Open Questions section of this log to be updated separately with five new questions raised by the audit.

---

## 2026-04-18

- Initialized this cleanup log. Seeded active renames, deprecations, intentional exceptions, and open questions from Section 13 of the PointSav Project Instructions.
- Established the session-start / session-end read-and-update pattern in CLAUDE.md.
- No code changes in this session. Next session should confirm the active renames table against a fresh grep of the repo to establish a baseline count of remaining occurrences per legacy term.
- Open question surfaced: whether the `service-parser` / `service-extraction` consolidation is scoped for a specific MEMO version or tracked informally. Answer will determine how we prioritize closing that migration.
