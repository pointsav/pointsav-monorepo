# Cleanup Log Archive — project-knowledge/pointsav-monorepo

Archived session entries from `cleanup-log.md`. Newest on top.
Active config sections remain in `cleanup-log.md`.

---

> *Archive compressed — 579 older lines removed. Full history in git log.*

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
