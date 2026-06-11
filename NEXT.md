# NEXT.md — project-orgcharts

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-09

---

## Resolved 2026-06-09 — DESIGN-TOKEN-CHANGE cosigns + outbox cleanup

- [x] **A3 DESIGN-TOKEN-CHANGE-orgchart-primitives** — master_cosign added
      `"2026-06-09T16:36:52Z jwoodfine"`. project-design notified (msg-id:
      project-orgcharts-20260609-cosign-done-a3-a4). [2026-06-09 totebox@claude-code]
- [x] **A4 DESIGN-TOKEN-CHANGE-orgchart-layout-type** — same cosign. Ready for commit
      to pointsav-design-system/tokens/dtcg-bundle.json. [2026-06-09 totebox@claude-code]
- [x] **Outbox cleanup** — 13 superseded Stage 6 accumulation signals archived to
      outbox-archive.md (status: stale); contaminated project-marketing content in
      outbox-archive.md cleared. Active outbox: 5 messages. [2026-06-09 totebox@claude-code]

---

## Stage 6 pending

- [ ] Promote all project-orgcharts commits to canonical.
      HIGH outbox signal sent (2026-06-08, msg-id: project-orgcharts-20260608-stage6-clean-76-commits).
      76+ commits total — includes color-sample.html cleanup, WCP JW3 green patch (#198038→#54924E),
      CSV V3+V4, 10 design artifact drafts, and all 2026-06-05 chart JW commits.
      Command Session runs `FOUNDRY_PROMOTE_YES=1 bin/promote.sh`.
      [2026-06-09 totebox@claude-code]

---

## Customer leg — awaiting Command + project-editorial

- [ ] Command Session to commit `MANIFEST.md` to `woodfine-fleet-deployment/cluster-totebox-corporate/`.
      Outbox message sent (2026-06-05, msg-id: project-orgcharts-20260605-customer-leg-manifest).
      [2026-06-05 totebox@claude-code]
- [ ] project-editorial to deliver `GUIDE-orgchart-authoring.md` from drafts-outbound.
      Draft staged: `.agent/drafts-outbound/GUIDE-orgchart-authoring.draft.md` (2026-06-05).
      [2026-06-05 totebox@claude-code]

---

## Wiki leg — milestone-gated

- [ ] `topic-corporate-chart-design-system.md` + `topic-pre-canon-vs-post-canon-drift.md`
      substance pending JW7+JW9 REVIEW milestones. [2026-05-01]

---

## archive-2026-06-01/ — deletion review 2026-07-01

- [ ] Directory gitignored (2026-06-05, commit `fe99d71b`). Contains misplaced repo
      clones — no live references. Per README: safe to `rm -rf` after 2026-07-01.
      [2026-06-04 totebox@claude-code]

---

## Registry CSV — follow-up items

- [ ] Nodes 8, 10–14 (Ireland fund service providers: Issuer, AIFM, Depositary, Administrator,
      Auditor, Transfer Agent): TOKEN_SHAPE left empty — these entities do not appear as t-node
      elements in any current chart HTML. Add when a chart is created for the ETN/ICAV structure.
- [ ] Nodes 51 (Global Management), 52 (Realty Solutions Common Shares), 53 (Holdings 1 Inc.),
      54 (Holdings 1 LP): TOKEN_SHAPE left empty — not found in any current chart. Add when charts
      are created for these entities.
- [ ] `token-olive` class (management chart) — may not yet exist in `pointsav-design-system` token
      bundle. Flag for project-design backfill.
- [x] Node 28 (Woodfine Management Corp.) — updated to `token-base token-green` in JW10 (V4 change, 2026-06-05 commit `739e15e5`). TOKEN_SHAPE in V4 CSV also updated.

---

## Resolved this session (2026-06-05, JW10)

- [x] **Bencal Organization JW10** — commit `cbc26742` (jwoodfine, 2026-06-05).
      Boxes 105/106/107 populated with full three-zone structure.
      Box 105 (SPV1 Shares): zone-top "SPV1 Shares" / zone-mid t-alias "Friends, Family and Business Associates (50)" + t-title "Accredited Investors (300)" / zone-bottom "(Global)".
      Box 106 (WCP Shares): zone-top "WCP Shares" / zone-mid t-alias "Accredited Investors (300)" + t-alias "max CAD 100,000 per subscriber" / zone-bottom "(Global)".
      Box 107 (SPV2 Investment Units): zone-top "SPV2 Investment Units" / zone-mid t-alias "Friends, Family and Business Associates (50)" + t-title "Accredited Investors (300)" / zone-bottom "(Global)".

---

## Resolved this session (2026-06-05, JW6 series)

- [x] **Transaction-1 JW31** — commit `d9052752` (jwoodfine). Lines 36→17, 17→16 purple.
- [x] **Transaction-3 JW17** — commit `a34d5361` (jwoodfine). Lines from Box 36 to 16/17/32/33/34 purple.
- [x] **Transaction-2 JW20** — commit `b53fbf8d` (pwoodfine). All lines from Box 36 purple.
- [x] **Cross-Border-2 JW21** — commit `a955732f` (jwoodfine). Box 45 → purple (match Box 36 Transaction-3 JW17).
- [x] **Cross-Border-2 JW22** — commit `5b1c069d` (pwoodfine). Lines 50↔45 → orange (#E65100).
- [x] **Mexico JW12** — commit `7b42fa1b` (jwoodfine). Line 40↔39 → blue.
- [x] **Bencal Organization JW6** — commit `53978aac` (pwoodfine). Grey pill → dashed; orange pill → dashed+orange; Boxes 95/97/104 → purple; Boxes 96/103 → orange; Box 98 → orange dashed pill.

---

## Resolved this session (2026-06-05, continuation)

- [x] **Bencal Organization JW3** — commit `c68593d4` (jwoodfine, 2026-06-05).
      All 9 nodes (95–104, excluding 99 absent) updated to V4 registry token classes.
      New CSS added: `token-green`, `token-blue` (210px × 110px, 10px padding, WCP palette).
      Nodes 100/101 (Kiel Capital, Elzen Holdings) → `token-grey-solid` (already defined).
      Nodes 95/96/97/102/103/104 → `token-base token-green`. Node 98 → `token-base token-blue`.
      Node 104 had inline `border-style: dotted;` — matched exactly.
- [x] **V4 TOKEN_SHAPE changes applied to 6 charts** — commit `739e15e5` (jwoodfine, 2026-06-05).
      Nodes 28/36/40/50 updated. New JW versions: JW10 (Woodfine-Group), JW11 (Mexico),
      JW20 (Cross-Border-2), JW30 (Transaction-1), JW19 (Transaction-2), JW16 (Transaction-3).
      New CSS added to charts: `token-green` (JW10), `token-orange-ellipse-dashed` (JW20),
      `token-purple` (JW30/JW19/JW16).
- [x] **WCP-MASTER-ENTITY-REGISTRY_V4.csv committed** — same commit `739e15e5`.
      New nodes 36, 95–104 added. Working-tree V3 accidentally reverted to old labels during
      Jennifer's V4 edit — restored from git (`git checkout -- inputs/V3.csv`); V4 built correctly
      from committed V3 CSS classes.

---

## Resolved this session (2026-06-05)

- [x] **Bencal naming conflict** — Operator confirmed: canonical is **BPC / Bencal Private Capital Inc.**
      (2026-06-05). JW2 files already correct. No BCL files found in deployment instance
      (grep confirmed). Decision recorded in memory + session-context. [2026-06-05 totebox@claude-code]
- [x] **archive-2026-06-01/ gitignored** — commit `fe99d71b` (pwoodfine, 2026-06-05).
      Deletion review 2026-07-01.
- [x] **Stage 6 outbox sent** — commit `9c422878` (jwoodfine, 2026-06-05).
- [x] **GUIDE-orgchart-authoring staged** — commit `fc7c720d` (pwoodfine, 2026-06-05).
- [x] **Customer leg MANIFEST outbox sent** — commit `fc7c720d` (pwoodfine, 2026-06-05).
12 TOPIC pairs + 4 GUIDEs in `.agent/drafts-outbound/`. Pickup notice sent to project-editorial.

**PROSE-RESEARCH:** v0.2 operator_approved 2026-06-11. "robust" → "reliable" fix applied (line 173).
Editorial pickup message sent (msg-id: command-20260611-prose-research-ppn-architecture-phd-thes).

---

## Code — Genesis Protocol (all Q2–Q6 resolved 2026-06-11)

- [x] Rewrite `os-infrastructure/src/main.rs` — Genesis Protocol boot sequence (Step 1) ✓ 2026-06-11
- [ ] Implement `system-substrate-broadcom/src/lib.rs` — silicon_ping() (Step 2)
- [ ] Implement `system-network-interface/src/lib.rs` — WireGuard/mDNS substrate (Step 3)
- [ ] Short-code pairing ceremony for node join (Step 4)
- [ ] Replace F8 Gateway subprocess with HTTP to localhost:9080 (Step 5, gated on Q5)
- [ ] Replace JSON mesh payloads with 16-byte binary protocol (Step 6)
- [ ] Add focus crates to root `Cargo.toml` workspace members (Step 8)

---

## Blocking — all resolved 2026-06-11

- [x] **Q2: Canonical PPN subnet — `10.8.0.0/24`** (ratified 2026-05-30; replaces 10.50.0.0/24)
- [x] **Q3: GCP cloud relay static IP — `34.53.65.203`** (reserved 2026-05-30)
- [x] **Q4: Laptop B LAN IP — `10.0.0.224`; DNS stable** (no config change; Jennifer VPN stays stable)
- [x] **Q5: Doorman at `localhost:9080` — active** (local-doorman systemctl verified 2026-06-11)
- [x] **Q6: Stage 6 confirmed; editorial pickup confirmed** (6 TOPIC pairs relayed 2026-05-30)

---

## GUIDE leg — cross-repo fix (Command Session scope)

- [ ] `fleet-infrastructure-leased/guide-deploy-vpn.md` — fix hardcoded path
  `$HOME/Foundry/pointsav-monorepo/` → `/srv/foundry/vendor/pointsav-monorepo/`
  Lives in `customer/woodfine-fleet-deployment` — Command Session admin-tier.

---

## Completed this cluster (archived for reference)

- [x] Sweep project-intelligence contamination from archive (session 1)
- [x] Fix session-start.md, manifest.md, NEXT.md, memory init (session 1)
- [x] Stage sovereign-mesh.md + .es.md drafts (session 2)
- [x] Fix os-infrastructure/Makefile + forge_iso.sh paths (session 2)
- [x] Gitignore build artifacts in os-infrastructure/ and os-network-admin/ (session 2)
- [x] Create app-infrastructure-onprem/-leased/-cloud/ Reserved-folder scaffolds (session 2)
- [x] PPN architecture: BRIEF-PPN-ARCHITECTURE.md (385 lines, 57 citations) (session 7)
- [x] vm-prove.sh Alpine TCG proof: virtio_balloon confirmed (session 7)
- [x] service-ppn-pairing deployed :9205 (session 13-14)
- [x] service-vm-fleet + service-vm-host deployed on GCP (session 13-14)
- [x] vm_spawn module + QEMU monitor Phase 2 (session 13-14)
- [x] PROSE-RESEARCH v0.2 editorial revision (session 15)
- [x] service-ppn-pairing normalize bug fix + 4 integration tests (session 15)
- [x] service-ppn-pairing fixed binary deployed to :9205 (session 16)
- [x] Commit carried-over cleanup: CLAUDE.md title fix, reliable fix, stale outbox, memory decontamination (session 17)
- [x] Action all inbox messages — 6 messages archived (session 17)
- [x] PROSE-RESEARCH PPN thesis v0.2 operator approved; editorial pickup message sent (session 17)
- [x] Q2–Q6 all resolved; confirmed values recorded; subnet corrected 10.50→10.8.0.0/24 (session 17)
- [x] Genesis Protocol Step 1: os-infrastructure/src/main.rs rewritten; build-std config added (session 17)
