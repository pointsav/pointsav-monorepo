## Session context — rolling 3-session summary

---

### 2026-05-28 | totebox@project-console | claude-sonnet-4-6

**Done this session:**
- Startup only. No code changes.
- NOTAM hazard resolved: 17 health alerts (doorman-unreachable, services-down) cascaded from 2026-05-27T00:34Z. Root cause: `slm-doorman-server` killed by SIGTERM after spin-loop on shadow brief `84DEA8VZHK0XNXW0JD1FERH3WX`. Apprenticeship queue was empty; restarted `local-doorman` — service now healthy.
- Doorman port discrepancy discovered: service binds `127.0.0.1:9080` per systemd logs; manifest + Phase 3 code note says `localhost:8011`. `app-console-content` code references 8011. Must resolve before Phase 6 work (offline mode polls doorman healthz). Added to monorepo NEXT.md.
- Archive-level NEXT.md (`/srv/foundry/clones/project-console/NEXT.md`) contains project-infrastructure content ("NEXT.md — project-infrastructure (cluster/project-infrastructure branch)") — contamination noted in outbox for Command.
- Updated monorepo `NEXT.md`: Phase 5 → Complete; Phase 6 → Current (was still showing Phase 5 as Current since `894452c1` shutdown update didn't land in monorepo on rebased main).

**Pending / carry-forward:**
- Stage 6 push: waiting Command authorization for force-push. See outbox `project-console-20260522-stage6-history-divergence`.
- **Pre-Phase 6 blocker:** doorman port — verify which is authoritative (9080 from service log vs 8011 from manifest). Check `slm/endpoint.txt` and `pairings.yaml`. If 9080 is correct, update code references in `app-console-content/src/draft.rs` and `ContentCartridge`.
- Phase 6: offline mode + Tantivy full-text search.
- Pairing-server systemd unit, GCE firewall port 2222, Peter's SSH key — Command/operator.
- Archive-level NEXT.md replacement (currently has project-infrastructure content) — Command decision needed.

**Operator preferences surfaced:** (none new this session)

---

### 2026-05-24 | totebox@project-console | claude-sonnet-4-6

**Done this session:**

- **Brief consolidation (session 13):** Archived 3 contaminated project-infrastructure briefs + 1 project-editorial file. Staged Gemini AI-AUDIT with corrections. Updated BRIEF-slm-substrate-master.md (P0/P1/P2/P3 open items). Created BRIEF-project-intelligence-active-work.md. Commit `1b6c8df8`.
- **Poison queue resolved (session 13):** 78 entries investigated — 68 pre-Fix-A quarantined, 10 post-Fix-A (llama-server outage artifact) recovered to queue/. queue-poison: 0.
- **P1 — /readyz reason+zone (session 14):** Added `reason` + `zone` fields to `TierBInfo` in `slm-doorman/src/router.rs`. Zone read from `SLM_YOYO_GCP_ZONE` env var. 3 new tests (`tier_b_status_reason_health_probe_failures`, `_request_failures`, `_no_reason_when_closed`). All 105 lib tests pass. Commits `6347d41e`, `eb9a2f75`.
- **P2 — service-content base_dir (session 14):** Replaced stale `/home/mathew/deployments/...` default with `${INFRASTRUCTURE_ROOT}/data`. Commit `6347d41e`.
- **Sprint 4a — app-console-slm status command (session 14):** Full implementation — Doorman /healthz+/readyz, Tier A/B health, chassis health, corpus counts. 6 unit tests pass. Smoke test confirmed (`Doorman UP, Tier A UP`). Commits `df802ff3`, `5077d92d`.
- **Corpus audit (session 14):** Discovered all 1,410 existing `edit` tuples have empty `actual_diff` (pre-Fix-A). 548 shadow-capture tuples have empty OLMo diffs (generated before Fix B). Only the 77+ post-Fix-A `queue/` entries have real diffs. These are the ONLY useful training signal.
- **Research + architecture revision (session 14):** Web research (5 papers) confirms: (1) empty DPO rejected samples are HARMFUL not neutral; (2) SFT alone outperforms SFT+DPO at <5K samples; (3) CodeDPO with execution-based validation is the right GPU path. Revised architecture: SFT-first → CodeDPO-on-GPU only. BRIEF-slm-learning-loop.md §9 written. Commit `9311da5c`.
- **All tests pass:** slm-doorman (all), app-console-slm (6/6), service-content (10/10).

**Pending / carry-forward:**

- **OPERATOR ACTION REQUIRED — drain pause:** Run `sudo sed -i 's/SLM_HOLD_THRESHOLD_SECS=3600/SLM_HOLD_THRESHOLD_SECS=1/' /etc/local-doorman/local-doorman.env && sudo systemctl restart local-doorman.service` — pauses CPU drain, keeps SFT capture, auto-resumes when Yo-Yo starts. (Blocked by sudo classifier from Totebox session.)
- **Stage 6:** 6 commits ahead of origin/main — Command Session needs `bin/promote.sh`. Commits: `1b6c8df8`, `6347d41e`, `df802ff3`, `5077d92d`, `eb9a2f75`, `9311da5c`.
- **SFT extraction script:** `scripts/extract-sft-pairs.py` — read `queue/*.jsonl`, filter `actual_diff != ""`, output clean SFT JSONL for LoRA training on 77 post-Fix-A entries.
- **CodeDPO scaffold (Yo-Yo gated):** Generate candidate diffs with OLMo 3 32B-Think, validate with `cargo check`, output execution-validated DPO pairs.
- **Quarantine corrupt DPO tuples:** The 548 `training-corpus/apprenticeship/shadow-capture/` tuples must be quarantined before any training run.
- **LoRA fine-tuning first run:** After SFT extraction + CodeDPO pairs. Rank=16, alpha=32, 5–10 epochs. Checklist in BRIEF-slm-learning-loop.md §9.
- **Fix C (deferred indefinitely):** GBNF grammar no longer urgent — CPU drain paused; GPU OLMo 3 handles format natively.
- **orchestration-slm deploy:** Operator actions from outbox `project-intelligence-20260530-stage6-orchestration-deploy` still pending.
- **app-console-content port fix (project-console):** `src/draft.rs` 8011 → 9080 — message in outbox to project-console.
- **stale shim test fields:** `anthropic_shim_test.rs` `tier_a_reason`/`idle_monitor`.
- **drain-timer systemd cleanup:** Disabled units still in `/etc/systemd/system/`.

**Operator preferences surfaced:**
- Deep think before coding — questioned whether CPU DPO was realistic (it wasn't). Research before committing to architecture.
- Comprehensive BRIEF updates after major findings — don't lose valuable learning between sessions.

---

### 2026-05-31 | totebox@project-intelligence | claude-sonnet-4-6 (session 12 — apprenticeship prompt audit; Fix A + Fix B deployed)

**Done:** Full audit of shadow capture chain. Fix A: pass diff via `HOOK_DIFF` env var (heredoc was consuming stdin). Fix B: rewrote `APPRENTICE_SYSTEM_PROMPT` to remove Claude-specific jargon; OLMo was preambling before `---` causing 100% escalation. New binary deployed 00:41 UTC. Stage 6 complete by Command (`a0649002`, `aef13fd9`, `b57f9d22`).

**Pending carried forward:** Fix C (GBNF grammar) deferred — now indefinitely deferred per session 14 architecture revision. Yo-Yo 1h test when L4 capacity returns. Stale shim test fields.

---

