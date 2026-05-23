# Session Context — project-editorial

Rolling 3-session summary. Newest entry first. Push oldest to session-context-archive.md when >3 entries.

---

## 2026-05-23 | totebox@claude-code | Sonnet 4.6

**Done this session (pre-build prep + wiki quality sweep):**
- **Category placement fix** — `capability-ledger-substrate` + `merkle-proofs-as-substrate-primitive` moved from `architecture/topic-*` → `substrate/*`; slug, frontmatter, `_index.md` + `_index.es.md` MOC entries corrected (`69c6030`).
- **Wikilink audit** — 0 broken links confirmed across all 3 wikis after category fix.
- **featured-topic.yaml candidates** — `capability-ledger-substrate` + 11 other rotation-pool articles added (`3f798bf`).
- **Banned vocab sweep** — 19 files across 6 categories: all `utilize*`, `robust`, `seamless`, `leverage` instances removed (`7cd8e3e`, `22face7`).
- **doorman-protocol claim fix** — `tier-a-verified` confidence changed from `established` → `structural` in EN+ES (`7cd8e3e`).
- **D1 linter hardening** — claim-validation §9 (id/confidence/uniqueness/cites/depends_on/projected-language), WARN-vs-ERROR fix for empty cites, ES projected-language skip (NameError fix), skip-dirs filter for non-article files (`d212863c`, `f64e279b`, `b8e1665e`).
- **glossary-documentation.es.md stub** — last missing bilingual pair; linter now reports 0 errors across 366 content-wiki-documentation articles (`583f642`).
- **NEXT.md dedup** — duplicate "Currently open" heading removed (`b8ddee0`).
- **Outbox addendum** — path-correction for capability-ledger-substrate (architecture/topic-* → substrate/*) prepended to project-system message (`cd7ae157`).

**Pending / carry-forward:**
- **Stage 6 promotion** — all three content-wiki-* repos; Command session only. Request in outbox since 2026-05-22.
- **D5** — operator signing identity; not project-editorial-executable.
- **E2/E3/E5/E-claim/E-rename** — cross-cluster / operator GitHub rename gated.
- **Plan archival + §9 old-plan deletion** — operator go-ahead, post-ship.
- **glossary-documentation.es.md** — stub only; full translation deferred.

**Operator preferences surfaced:**
- AUTO mode: execute autonomously on approved runs; surface only real blockers; report at end.
- Build-night goal: 0 lint errors before nightly build; achieved.

---

## 2026-05-22 | totebox@claude-code | Opus 4.7 (1M)

**Done this session (large session — ~40 commits across cluster repo + 3 content sub-clones):**
- **Briefs migration** — `.agent/plans/` → `.agent/briefs/`, `BRIEF-` prefix + `artifact: brief` frontmatter + new README; 2 relocated workspace briefs brought in (`e5bd2514`).
- **Editorial-plan AUTO block (10 items)** — E1 (flagged service-content to project-intelligence), E4 (project-intelligence inbound triage), A0 (Gate-0 standard into the 4 style guides), D1 (`editorial-lint.py` + `banned-vocabulary.txt`), D2 (failure-mode registry), A1 (3 recommended Main Page ledes, EN+ES), D4 (canonical `editorial-standard.md` + `CORPUS-SCHEMA.md`), D3 (16 genre templates), D6 (manifest revision), E-ruleset (routed to project-knowledge). Editorial-QA substrate built at `.agent/editorial-qa/`.
- **A2 — all 12 flagship TOPIC rewrites** (EN+ES = 24 files): Bloomberg 4-paragraph Crisis-first lede + Gate-0 + claim markup per `claim-authoring-convention` #54. 10 in content-wiki-documentation, 1 projects, 1 corporate. All lint clean.
- **A4 close-out** — built `wikilink-audit.py`; 0 broken links across all 3 wikis; plan §12 + top status banner; Stage 6 publish request to Command.

**Pending / carry-forward:**
- **Editorial plan: project-editorial's autonomous execution is COMPLETE.** Status banner + §12 in `BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md`.
- Pending, none project-editorial-executable: Stage 6 promotion (Command — request in outbox); D5 apprenticeship loop (operator signing identity); E2/E3/E5/E-claim/E-rename (cross-cluster / operator GitHub rename); A1 review pass (project-knowledge branches Main Pages); claim-validation linter pass (Track-D follow-up on `editorial-lint.py`); plan archival + §9 old-plan deletion (operator go-ahead, post-ship).
- Inbox: 4 messages actioned this session (briefs-migration, briefs-cleanup-ack, doctrine-ratified, project-knowledge plan handoff) — archive at next startup. 4 still pending: 3 project-system language-pass batches + 1 Command LICENSE-artifacts batch — language-pass backlog, not yet triaged.
- Branch drift (recorded in manifest D6 note): content sub-clones commit editorial work to `main`; sub-clone branches retain the pre-rename `cluster/project-language`.

**Operator preferences surfaced:**
- **AUTO working mode** — when the operator approves an AUTO run, execute autonomously: commit per track/article, surface §10 stop conditions, report at the end. Do not re-ask permission for in-scope work.
- **At-a-glance status** — when asked to update a brief, put a clear status banner at the top, not only a detail section at the end.

---

## 2026-05-21 | totebox@claude-code | Opus 4.7

**Done this session:**
- Built `award-winning-wiki-overhaul.md` — consolidated the 3 blueprint files (INPUT-KNOWLEDGE-PLATFORM-BLUEPRINT, MASTER_STRATEGY, FINAL_EXECUTION) into a 5-track plan; 4 research agents (editorial gap, engine state, design tokens, archived drafts).
- Gate 0 ratified by operator — 5 Lucidity-Protocol-vs-Bloomberg-standard conflict reconciliations.
- 3 more research agents — AI writing craft + AI-writing QA + project-editorial substrate audit → Track D (editorial QA substrate); §2.2 services-optional bypass posture; Track E from a project-intelligence state check.
- Cross-checked project-knowledge's `KNOWLEDGE-PLATFORM-VISION.md` (rev 4): Main Page ownership → project-knowledge, repo rename `content-wiki-* → media-knowledge-*`, source-of-truth inversion, claim-native model.
- **Adopted** project-knowledge's proposed editorial plan; finalized it (re-inserted §3 bypass posture, inlined all referenced detail → self-contained); committed as `KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md` (`b8c19dfd`). It supersedes `award-winning-wiki-overhaul.md` + the 3 blueprints + the overhaul-* plans.
- Outbox: cross-check reply + plan-adopted reply to project-knowledge; SITUATION message to Command/Master.

**Pending / carry-forward:**
- **Active plan: `KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md`.** Next: A0 (encode the standard into style guides) → D1/D2 (editorial linter + failure-mode registry). E1/E4 are cheap, run first.
- **A2 HELD** — Top-12 TOPIC rewrites wait for project-knowledge's claim-authoring convention (their Phase 2.4); they route it to our inbox when specced.
- **4 pending inbox messages NOT actioned** — project-system (capability-ledger + merkle-proofs TOPICs, 6 README drafts), Command (LICENSE artifacts incl. one project-knowledge item). Triage next session.
- **Old-plan deletions parked** — execute after the overhaul ships, on operator go-ahead (delete set in plan §9). The 3 blueprint files + `award-winning-wiki-overhaul.md` remain untracked, intentionally.
- Doctrine amendment for the source-of-truth inversion pending from Master; repo rename pending from operator.
- `service-content` runtime-hung (project-intelligence scope) — E1 handoff staged in outbox.

**Operator preferences surfaced:**
- Superseded plans are deleted only after the overhaul ships, on explicit operator go-ahead — not on a peer cluster's directive. Peer Totebox clusters cannot direct deletions in another cluster's archive.
- When adopting another cluster's proposed plan, inline referenced detail so the plan is self-contained.
- Editorial work must never block on `service-content` / `service-slm` — bypass by default.

