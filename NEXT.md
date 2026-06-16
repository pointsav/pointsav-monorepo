# NEXT.md — project-knowledge (Totebox)

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-16

---

## Blocked — Command Session (route via outbox)

- [ ] **check --strict gate** — F2/F3 dead links at project-editorial must resolve first
- [ ] **Nginx vhost SSE proxy** — add `/_api/edit/events` block to nginx vhost for live reload:
      `proxy_buffering off; proxy_read_timeout 3600s; proxy_set_header Connection ''; proxy_http_version 1.1;`
      (VM sysadmin scope)

## Active (Totebox scope)

- [ ] **Cargo check + test gate** — `cargo check -p app-mediakit-knowledge` + `cargo test -p app-mediakit-knowledge` must pass before Stage 6 promotion; blocked on `~/.cargo/.package-cache-mutate` lock from project-intelligence slm-doorman + system memory pressure (608MB available at time of commit `21212c69`); run as soon as lock releases [2026-06-16 totebox]
- [ ] **Stage 6 READY signal** — send to Command after cargo check + test pass; include E9 commit SHA `21212c69` [2026-06-16 totebox]
- [x] **Design artifacts D1a–D1e** — dispatched to project-design via send_mailbox_message (msg-id: command-20260616-design-artifacts-dispatch-knowledge-plat) [2026-06-16 totebox@claude-code]
- [x] **D2 orgcharts relay** — D2a–D2m relayed to project-design via send_mailbox_message (msg-id: command-20260616-cross-archive-relay-d2a-d2m-orgchart-wor); GUIDE-orgchart-authoring.draft.md flagged for project-editorial [2026-06-16 totebox@claude-code]
- [ ] **F-series tracking** — F1–F7 content repair requests sent to project-editorial 2026-06-14; track responses; update artifact-registry.md Status column when returned [2026-06-16 totebox]

## Completed (this session, 2026-06-16)

- [x] **Sprint D** — home page peer-band (`aside.peer-band`) in `home_handlers.rs`; CSS in `style.css`; peers threaded into `home_chrome()` [2026-06-16 totebox@claude-code]
- [x] **Defect 2** (footnotes CSS) — `sup/sub/footnotes` CSS prophylactically added to `style.css` [2026-06-16 totebox@claude-code]
- [x] **M13 /openapi.json** — `GET /openapi.json` route added → 301 redirect to `/openapi.yaml` [2026-06-16 totebox@claude-code]
- [x] **Sprint E** — `audience` + `aliases` fields added to `render::Frontmatter` (correct) + `walker::Frontmatter`; audience chips in `wiki_chrome`; `resolve_alias_slug()` + 301 alias redirect [2026-06-16 totebox@claude-code]
- [x] **Sprint F** — engine version in `shell_footer()`; `.peer-strip` cross-instance nav in `wiki_chrome`; doc-header CSS gap noted (already in HTML, was missing CSS — `.site-footer__trademark` CSS structure confirmed) [2026-06-16 totebox@claude-code]
- [x] **Sprint G** — `search_complete` returns `{title,slug,lede}`; `search_page` accepts `?category=` + `?status=` filters (post-search) [2026-06-16 totebox@claude-code]

## Completed (Sessions 84+)

- [x] **Stage 6 — sub-clone + archive** — promoted Session 86 (→ d0abd9ad) [2026-06-16 command]
- [x] **Post-Stage-6 TOML + binary rebuild** — instance/canonical_url added; binary rebuilt; all 3 instances healthy [2026-06-16 command]
- [x] Phase 9: WCAG 2.2 focus outline, sitemap/i18n repairs, defects 1/4/8 — committed E6+E7
- [x] Phase 7 ActivityPub scaffold — committed E5 (6d554ec6)
- [x] Sprint C 7-category IA — 9cc1a80c
- [x] GET /images/{*path} route — da07781e
- [x] Sub-clone CLAUDE.md identity fixed (was project-console contamination) — a51f201b
- [x] Archive identity corrected in manifest + CLAUDE.md — 9fb431cb
- [x] .agent/briefs/ gitignore exclusion removed; BRIEFs now tracked — f11197ee
- [x] D1b draft renamed with DESIGN- prefix — 9eda459f
- [x] Master BRIEF updated (Sprint 0 row; 2026-06-15 work log) — 298ba52a
- [x] 12-agent external audit — F1–F7 content repair requests dispatched to project-editorial
- [x] check --strict: F2/F3 dead links identified
# NEXT.md — project-intelligence (Totebox)

## Active (Totebox scope)

- [ ] **CLAUDE.md contamination** — file contains project-console mission text; needs replacement
      with project-intelligence mission (service-slm Doorman + OLMo enrichment + LoRA training);
      Totebox scope fix [2026-06-16 command@claude-code]
- [ ] **down_for_secs in TierBInfo** — expose seconds-since-last-healthy-check in /readyz;
      circuits currently report "closed" with health_up=false causing false routing to Tier B;
      Bug 4 drain-hold fix is live but this TierBInfo extension is outstanding
      [2026-06-15 command@claude-code]
- [ ] **Phase 4b reconciliation pass** — 1,281 sweep-ledger entries written before Tier B online;
      DOC_sweep quarantine gate now in place (4a9c81b9) — next replay will skip DPO pair gen
      for sweep docs and mark SHAs complete correctly; Totebox sprint when Tier B restores
      [2026-06-15 command@claude-code]

## Completed (2026-06-16, Session 18)

- [x] **State sync actioned** — command-20260616-project-intelligence-state-sync; Stage 6
      (088b8e21→4886129d) + doorman deployment (52ead171) + quarantine re-drive confirmed;
      NEXT.md reconciled [2026-06-16 totebox@project-intelligence]
- [x] **Bug 1: SHA-on-202-ACK** — repair-ledger.py (52746a3c) ran; stale SHA entries cleared;
      ledger at 0; ~400 files will re-enrich automatically when Tier B restores; sweep ledger
      also fixed (4a9c81b9 — mark_sweep_sha_complete now unconditional for DOC_sweep-* docs)
      [2026-06-16 totebox@project-intelligence]

## Completed (2026-06-16, Session 84 extended)

- [x] **Stage 6 complete** — 8 commits (088b8e21→4886129d) promoted to canonical; includes
      Q4 nightly gate bypass, Q1 repair-ledger.py, Q11 BRIEF fix, drain-hold fix, Q6/Q8
      batch-extract + redrive-quarantine.py [2026-06-16 command@claude-code]
- [x] **slm-doorman-server deployed** — rebuilt from 4886129d (service-slm sub-workspace);
      sha256: 52ead171; local-doorman restarted; drain-hold fix live in production
      [2026-06-16 command@claude-code]
- [x] **redrive-quarantine.py** — 737 quarantined briefs → queue; queue_pending=785;
      queue_quarantine=0 confirmed [2026-06-16 command@claude-code]
- [x] **SLM_DRAIN_PAUSED** — drain-paused.conf active; OLMo slot freed for entity extraction
      [2026-06-16 totebox@project-intelligence]
- [x] **Bug 4: drain-hold fix** — commit 28231f6f: !tier_a_first guard removed; hold fires on
      health_up=false [2026-06-15 totebox@project-intelligence]
- [x] **Bug 2: LoRA target_modules (real fix)** — commit 23b012a1: q_proj/k_proj/v_proj/
      o_proj/gate_proj/up_proj/down_proj (LLaMA-arch names); commit 401827c7 introduced wrong
      OLMo-1 names; 23b012a1 supersedes it; training has never produced an adapter until this fix
      [2026-06-16 totebox@project-intelligence]
- [x] **DataGraph noise filter expansion** — commit 23b012a1: entity_filter.rs numeric-prefix,
      FRAGMENT_STARTERS 4→14, ABSTRACT_NOUNS blocklist (18 terms); 39/39 tests
      [2026-06-16 totebox@project-intelligence]
- [x] **DOC_sweep quarantine gate + sweep ledger fix** — commit 4a9c81b9: DPO pair gen skips
      DOC_sweep-* worm IDs; mark_sweep_sha_complete now unconditional for sweep docs; sweep
      ledger was permanently stuck at 0 (SHAs re-submitted every nightly cycle); 40/40 tests
      [2026-06-16 totebox@project-intelligence]
- [x] **Doorman batch extract endpoint** — commit e5c0ee4f (in Stage 6'd range); POST
      /v1/batch/extract + Semaphore(4) Tier A / Semaphore(1) Tier B; CONTENT_BATCH_SIZE env var
      [2026-06-16 command@claude-code]
- [x] **yoyo-batch stopped** — operator requested immediate stop; VM TERMINATED 2026-06-16;
      do not restart until operator approves + prerequisites met (see Blocked section)
      [2026-06-16 operator]

## Blocked — requires Command Session (route via outbox)

- [ ] **yoyo-batch VM start** — do not restart until operator approves next cycle; if
      reprovisioning: us-central1-b ONLY (NOT us-central1-a, NOT zone-fallback), image
      slm-yoyo-20260512-111846 + startup-script-url from GCS [via outbox]
- [ ] **ML lib install on yoyo-batch** (Bug 3) — pip install trl peft transformers accelerate
      bitsandbytes in ~/training-venv; VM currently TERMINATED; flag to Command when ready
      [via outbox]
