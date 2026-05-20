---
schema: foundry-session-context-v1
archive: project-design
format: rolling-3-entries  # oldest entry pushed to session-context-archive.md
---

# Session context — project-design

---

## 2026-05-20 | Totebox | claude-code

**Done this session:** Startup-only. Completed all 8 startup steps. Session lock written. Inbox swept (all messages actioned). State survey of all 3 sub-clones.

**State at session end:**
- `pointsav-design-system`: clean on `main`, HEAD `7c1916a` (CITATION.cff — committed since last working session)
- `pointsav-monorepo`: clean
- `pointsav-fleet-deployment`: dirty on `cluster/project-design` — pre-existing unstaged changes:
  - `vault-privategit-design/GUIDE-deploy-design-substrate.md` deleted; `vault-privategit-design/guide-deploy-design-substrate.md` untracked (case rename, uppercase→lowercase per naming conventions)
  - `.claude/rules/project-registry.md` deleted (50 lines — not sure if intentional; check before committing)
  - ~25 other files modified with zero content diff (likely propagation noise — mode/timestamp drift)
  - `.agent/` untracked (new in this sub-clone)

**Pending / carry-forward:**
- [ ] Fleet-deployment: investigate zero-diff modifications, then stage + commit the rename and any real changes
- [ ] `drafts-outbound/design-gis-chain-search-bento-2026-05-06.md` — marked committed at eb51d0f in archive log; remove file from drafts-outbound
- [ ] `drafts-outbound/topic-favicon-matrix.md` — stale; check and remove if superseded
- [ ] GIS map screenshots: `asset-gis-map-screenshots-2026-05-06.md` still at `asset-capture-pending-operator`; remains for operator action
- [ ] 6 GUIDE + 5 TOPIC/PROSE drafts in drafts-outbound — awaiting project-editorial pickup (outbox message pending since 2026-05-17)

**Operator preferences surfaced:** none new.

---

## 2026-05-17 | Totebox | claude-code

**Done this session:** Drafts cleanup sweep. Committed `research-ps-badge-favicon-design` (cbfaad7). Extracted `design-main-page-token-2` to `tokens/main-page/main-page.dtcg.json` (0955b5c). Stage 6 complete (b29b0a9 → 0955b5c on canonical). Routed 12 GUIDE/TOPIC drafts to project-editorial (outbox message). PRODUCT_VISION actioned. Session-start updated. 11 committed drafts removed from drafts-outbound (55d1f9a). BIM sweep ACK to project-bim.

**Pending / carry-forward (as recorded):** fleet-deployment dirty state; GIS screenshots operator-action; project-editorial draft pickup; woodfine-design-bim Stage 6 (Command scope).
