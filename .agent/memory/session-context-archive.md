# Session Context Archive — project-infrastructure

Entries aged out of the rolling 3-session summary in `session-context.md`. Newest first.

---

## 2026-05-28 session 6 | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- Wrote two new TOPIC bilingual pairs (EN+ES) and committed to drafts-outbound:
  - `topic-totebox-archive` — sovereign WORM data vault; disk image IS the archive;
    freely transferable; JSONL/GeoParquet/Markdown; Diode + PSP access only; MBA keypair;
    cluster naming convention; what it is NOT.
  - `topic-ppn-architecture-overview` — four-layer architecture overview (operator / PPN /
    hypervisor / Totebox Orchestration); three key properties (isolation invariant, freely
    transferable archives, zero crypto authority at network plane); what PPN is NOT; links
    to all 8 detailed TOPICs.
- Wrote three GUIDE drafts and committed to drafts-outbound:
  - `guide-ppn-first-deployment` — 5-step first-deployment sequence from BRIEF §7, all steps
    unblocked, with exact commands and troubleshooting table.
  - `guide-node-join-ceremony` — approval workflow (node side: Crockford base32 short code;
    operator side: poll + approve via curl); CPace PAKE + SAS; nodes.jsonl; 600s TTL.
  - `guide-vm-prove-balloon-demo` — vm-prove.sh walkthrough; virtio_balloon demo from QEMU
    monitor; pool formula; GCP nested virt enablement; proves-vs-not-yet table.
- Updated manifest.md: wiki leg leg-pending → leg-active; 7 TOPICs + 3 GUIDEs in
  staged_for_pickup.
- VM proof on GCP TCG: Alpine Linux 3.20 (kernel 6.6.31-0-virt) booted in 114s; full
  virtio_balloon cycle confirmed: `balloon 128` → `actual=128`; `balloon 256` → `actual=256`.
- Added `infrastructure/virt/.gitignore` — excludes Alpine ISO + QCOW2 work artifacts.
- Sent outbox session 6 pickup notice to project-editorial (9 TOPIC pairs + 3 GUIDEs total).

**Commits this session:**
- `5029e0fd` — docs(ppn): totebox-archive + ppn-architecture-overview TOPICs; 3 GUIDE drafts; manifest leg-active
- `04388865` — chore(vm-prove): mark GCP TCG balloon proof complete
- `d608f18b` — chore: gitignore virt/work/ — Alpine ISO + QCOW2 are build artifacts

**Pending / carry-forward:**
- Q2: Ratify `10.50.0.0/24` as canonical PPN subnet (de facto confirmed in guide-lxc-network-admin)
- Q3: GCP static IP for cloud relay
- Q4: Laptop B local IP + `network.woodfinegroup.com` DNS status
- Q5: Is service-slm Doorman deployed at `localhost:9080`? (app-network-admin F8 still uses subprocess)
- Q6: Flag stale editorial pickup to Command Session?
- All 7 Genesis Protocol code steps in BRIEF §9.2 gated on Q2–Q6
- 9 TOPIC pairs + 3 GUIDEs in drafts-outbound awaiting project-editorial pickup
- 12 commits ahead of origin/main — Stage 6 from Command Session when ready

**Operator preferences surfaced:**
- "keep going" / "what can we do next" workflow: plans work items in NEXT.md, approves plan,
  then runs phases sequentially; no need to pause between phases once plan is approved.

---

## 2026-05-28 session 5 | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- Wrote os-network-admin TOPIC bilingual pair (EN+ES) — corrects published wiki article that
  conflates os-network-admin (Foundation OS) with app-network-admin (F8 Terminal on top).
  Staged draft is the corrected replacement; project-editorial applies it on pickup.
- Wrote ppn-hypervisor-resource-pool TOPIC bilingual pair (EN+ES) — per-node CPU/RAM pool
  management; virtio_balloon formula; cgroups v2 cpu.weight; orthogonality with os-orchestration.
- Updated BRIEF-PPN-DEV-BOOTSTRAP.md §3 (four-layer diagram) and §6 (virtio_balloon proof plan).
- Updated BRIEF-PPN-ARCHITECTURE.md §9.4 (Resource Pool Management added).
- Added `-device "virtio-balloon"` to both QEMU invocations in `infrastructure/virt/vm-prove.sh`.
- Updated NEXT.md: dev-environment bootstrap tasks, future milestones.
- Sent outbox session 5 pickup notice to project-editorial.

**Commits:** `7ec14c86`, `565bc755`

**Pending / carry-forward:**
- Q2–Q6 operator decisions (same as session 4)
- All 7 code implementation steps gated on those decisions

**Operator preferences surfaced:**
- Produces TOPIC and GUIDE drafts proactively alongside code work.
- Wants accuracy audit of existing published topics when new TOPICs are added.

---

## 2026-05-27 session 4 | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- Ran 10 parallel Opus research agents covering: SMB hypervisor market, Type I hypervisor
  survey, seL4 formal verification, NetBSD/bhyve compat bottom, zero-config node federation,
  capability isolation proofs, OS personalities on microkernels, competitive differentiation,
  novel contribution claims, Yale PhD thesis structure.
- Synthesised agent outputs into `BRIEF-PPN-ARCHITECTURE.md` (385 lines, 57 citations) —
  Yale PhD thesis-quality architectural foundation for PPN. Genesis Protocol confirmed as
  canonical bootstrap; CPace PAKE + Crockford base32 SAS pairing; CAmkES OS personality;
  intransitive non-interference as formal isolation invariant.
- Updated NEXT.md: resolved EAPOL vs Genesis Protocol blocking item; restructured into
  BRIEF gate + Q2–Q6 operator decisions + Code implementation sequence.
- Session was disk-blocked mid-session (ENOSPC on home partition); user cleared space.

**Commits this session:**
- `289df71c` — brief: PPN architecture — Yale PhD thesis draft; Genesis Protocol confirmed; session-4 context + NEXT.md

**Pending / carry-forward:**
- Q2: Ratify `10.50.0.0/24` as canonical PPN subnet
- Q3: GCP static IP for cloud relay
- Q4: Laptop B local IP + `network.woodfinegroup.com` DNS status
- Q5: Is service-slm Doorman deployed at `localhost:9080`?
- Q6: Flag stale editorial drafts (5 pairs, 7+ days) to Command Session?
- All 7 code implementation steps in BRIEF §9.2 gated on Q2–Q6

**Operator preferences surfaced:**
- Produce research-first BRIEF before any code work — establish "Yale PhD thesis" quality
  foundation so no tokens wasted building wrong thing.
- 2-question bootstrap UX is the north star: "Is this the first node?" / "What is the
  address of the existing network?" — everything else flows from this simplicity invariant.

---

## 2026-05-20 session 3 | Totebox | claude-code

**Done this session:**
- Wrote three TOPIC draft pairs (EN + ES) and committed to drafts-outbound:
  - `topic-genesis-protocol` — 5-step Genesis Protocol, deferred fleet assembly,
    machine-based-auth relationship. One open question noted (EAPOL vs intended arch).
  - `topic-ppn-command-protocol` — 16-byte binary packet, 4-step dispatch, simultaneous
    broadcast rationale, Diode Standard relationship.
  - `topic-service-pointsav-link` — four properties (default off, hot-plug, clean severance,
    policy in adapter not kernel), Universal Standard.
- Sent outbox pickup notice to project-editorial covering all five staged draft pairs
  (sovereign-mesh from session 2 + three new pairs from this session).
- Updated NEXT.md TOPIC leg to list all four pending draft pairs.

**Commits:** `94290124`, `de899d74`, `4d5b6272`

**Pending / carry-forward:**
- All 4 operator decisions still blocking (EAPOL vs Genesis Protocol; subnet ratification;
  GCP static IP; Laptop A/B IPs + DNS)
- 5 draft pairs in drafts-outbound awaiting project-editorial pickup

---

## 2026-05-20 session 2 | Totebox | claude-code

**Done this session:**
- Stage sovereign-mesh TOPIC: expanded one-sentence stub to full PPN architecture topic.
- Fixed `os-infrastructure/Makefile` and `forge_iso.sh` paths.
- Gitignored build artifacts in `os-infrastructure/` and `os-network-admin/`.
- Created `app-infrastructure-onprem/`, `-leased/`, `-cloud/` Reserved-folder scaffolds.
- Split `system-network-interface` — extracted F8 Terminal Gateway binary to `app-network-admin/`.

**Commits:** `88831f63`, `d3c6a7c8`, `a958b217`, `b2eb755c`

**Pending / carry-forward:**
- 4 operator decisions blocking code + guides; sovereign-mesh drafts need editorial pickup.

---

## 2026-05-20 session 1 | Totebox | claude-code

**Done this session:**
- Startup sequence completed (role confirmed, lock written, manifest + inbox + NOTAM read)
- Full archive read-through: focus crates (`os-infrastructure`, `os-network-admin`,
  `system-network-interface`, `system-substrate-broadcom`), all infrastructure TOPICs in
  `content-wiki-documentation`, and all infrastructure GUIDEs in `woodfine-fleet-deployment`
- Comprehensive TODO written to `.agent/plans/project-infrastructure-todo.md` (339 lines,
  5 sections: housekeeping, TOPICs, GUIDEs, code, operator decisions)
- Section 1 housekeeping completed:
  - `session-start.md` corrected (was `archive: project-intelligence`)
  - `manifest.md` planned_topics slugs corrected to match published wiki filenames
  - `NEXT.md` replaced with infrastructure-scoped items
  - `.agent/memory/` created and seeded
  - 6 project-intelligence plan files removed from `.agent/plans/`
  - 11 project-intelligence drafts removed from `.agent/drafts-outbound/`
  - `CLAUDE.md` + `manifest.md` committed together

**Pending / carry-forward:**
- Section 2: expand `sovereign-mesh.md` stub → full PPN architecture topic
- Section 4a: fix broken build in `os-infrastructure` — blocked on EAPOL vs Genesis Protocol decision
- Section 4b: split `system-network-interface` crate
- Section 5: four operator decisions needed (EAPOL vs Genesis Protocol; subnet; GCP IP; Laptop IPs)

**Operator preferences surfaced:**
- Wants comprehensive TODO plans saved to `.agent/plans/` as markdown before starting work
- Working on PPN / network OS layer (os-infrastructure + os-network-admin as primary focus)

---

## NOTE: entries below this line are from project-editorial (archive header contamination — separate cleanup item)

---

## 2026-05-22 | totebox@claude-code | Opus 4.7 (1M)

**Done this session (large session — ~40 commits across cluster repo + 3 content sub-clones):**
- **Briefs migration** — `.agent/plans/` → `.agent/briefs/`, `BRIEF-` prefix + `artifact: brief` frontmatter + new README; 2 relocated workspace briefs brought in (`e5bd2514`).
- **Editorial-plan AUTO block (10 items)** — E1, E4, A0, D1, D2, A1, D4, D3, D6, E-ruleset. Editorial-QA substrate built at `.agent/editorial-qa/`.
- **A2 — all 12 flagship TOPIC rewrites** (EN+ES = 24 files): Bloomberg 4-paragraph Crisis-first lede + Gate-0 + claim markup. All lint clean.
- **A4 close-out** — `wikilink-audit.py`; 0 broken links across all 3 wikis; plan §12 + top status banner; Stage 6 publish request to Command.

**Pending / carry-forward:**
- Editorial plan: project-editorial's autonomous execution COMPLETE. Stage 6 promotion (Command); D5 apprenticeship loop (operator signing identity); E2/E3/E5/E-claim/E-rename (cross-cluster / operator GitHub rename); plan archival + §9 old-plan deletion (operator go-ahead, post-ship).

**Operator preferences surfaced:**
- AUTO working mode; At-a-glance status banner at top of briefs.

---

## 2026-05-21 | totebox@claude-code | Opus 4.7

**Done this session:**
- Built `award-winning-wiki-overhaul.md`; Gate 0 ratified; adopted project-knowledge's editorial plan; committed as `KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md` (`b8c19dfd`).
- Outbox: cross-check reply + plan-adopted reply to project-knowledge; SITUATION message to Command/Master.

**Pending / carry-forward:**
- Active plan: `KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md`. A2 HELD; 4 inbox messages pending; old-plan deletions parked.

**Operator preferences surfaced:**
- Superseded plans deleted only on operator go-ahead. Peer clusters cannot direct deletions. Editorial work must never block on `service-content` / `service-slm`.

---

## 2026-05-20 | totebox@claude-code | Sonnet 4.6

**Done this session:**
- Startup + shutdown only. No editorial work performed.

**Pending / carry-forward:**
- Stage 6: content-wiki-documentation, content-wiki-projects, content-wiki-corporate, woodfine-fleet-deployment — Command Session task.
- pointsav-monorepo `readme-fixes-2026-05-16` → main merge + service restart — Command Session task.
- Route design drafts to project-design — Command Session to forward outbox message.
- Phase E bilingual home routing — deferred, low priority, needs Rust change → project-knowledge.
- Italy co-location stub — needs data from project-gis.

**Operator preferences surfaced:**
- No new preferences this session.

---

## 2026-05-19 (continuation) | totebox@claude-code | Sonnet 4.6

**Done this session:**
- Phase 1c resolved: `topic-` prefix confirmed canonical for content-wiki-projects; all 31 EN+ES pairs already use it; outbox message sent to unblock Command Stage 6 (commit `ea074049`).
- Vendor co-location drafts: location-intelligence-ux TOPIC pair (EN+ES) committed to content-wiki-documentation/applications/ (`a2baf95`); all 6 vendor refined/ drafts removed from drafts-outbound (`01379316`).
- E1 wikilink fixes (13 files): delink reverse-funnel-editorial-pattern + service-minutebook/bookkeeper; fix sel4-foundation→sel4-microkernel-substrate in 7 ES files; remove broken cross-category _index links (`19cd854`).
- E3 status field: brand-family-swatch + brand-typography pairs — added `status: active` (`80de908`).
- E4 title case: foundry-services-slice-model sentence case; slug drift logged in cleanup-log.md (`d8d82cf`).
- Outbox pruned: 12 actioned/stale messages archived to outbox-archive.md; routing message added for 5 DESIGN-RESEARCH + component drafts → project-design (`487d1da3`).
- todo-open-items.md: archived 3 old DONE blocks; marked all new completed items (`6ca2e592`).

**Pending / carry-forward:**
- Stage 6 for content-wiki-documentation, content-wiki-projects, content-wiki-corporate, woodfine-fleet-deployment — Command Session task.
- pointsav-monorepo `readme-fixes-2026-05-16` → main merge + service restart — Command Session task.
- Route design drafts to project-design — Command Session to forward outbox message.
- Phase E bilingual home routing — deferred, low priority, needs Rust change → project-knowledge.
- Italy co-location stub — needs data from project-gis.

**Operator preferences surfaced:**
- No new preferences this session.

---

## 2026-05-19 | totebox@claude-code | Sonnet 4.6

**Done this session:**
- Corporate wiki lede rewritten using DataGraph-aligned language from `cluster-totebox-jennifer/service-content/domains/corporate.csv` — "Principal Manager for a portfolio of direct-hold solutions…" — replacing incorrect outbox draft that used "fractional equity in a named property" framing. Commit `188dabd`.
- index.es.md: status active, BCSC disclosure paragraph added, Spanish strategic adaptation written. Same commit.
- featured-topic.yaml: rotated from topic-interest-coverage-ratio → topic-redemption-elimination (rotation 2). Same commit.
- Command Session outbox message (2026-05-18T02:55:00Z re: corporate wiki lede) marked actioned.
- GIS drafts batch reviewed: topic-gis-nordic-uk-coverage.md + .es.md written and committed to content-wiki-projects (`a9d5325`); all other 8 drafts confirmed already committed in prior sessions.
- from-project-gis/ drafts-outbound cleared (9 files removed, `f82b4ee6`); Italy stub (`topic-co-location-index-italy`) bounced to handoffs-outbound — needs real cluster data from project-gis before publishing.

**Pending / carry-forward:**
- Phase 1c — resolved this continuation session.
- Stage 6 promotions needed for content-wiki-corporate, content-wiki-projects, woodfine-fleet-deployment sub-clones — Command Session task.
- Phase E bilingual home routing — deferred, low priority.

**Operator preferences surfaced:**
- DataGraph at cluster-totebox-jennifer is the authoritative source for corporate wiki vocabulary. Always check it before drafting lede or terminology for content-wiki-corporate. Outbox message language is a starting point, not final word.
