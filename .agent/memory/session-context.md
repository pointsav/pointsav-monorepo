# Session Context — project-infrastructure

Rolling 3-session summary. Newest on top. Push oldest to `session-context-archive.md` when
a fourth entry is added.

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

**Commits this session:**
- `7ec14c86` — docs(ppn): document resource pool, os-network-admin; add virtio_balloon to vm-prove.sh
- (and `565bc755` from earlier in same conversation — dev-environment bootstrap)

**Pending / carry-forward:**
- Q2–Q6 operator decisions (same as session 4)
- All 7 code implementation steps gated on those decisions
- 7 TOPIC pairs in drafts-outbound awaiting project-editorial pickup

**Operator preferences surfaced:**
- Produces TOPIC and GUIDE drafts proactively alongside code work — "what can we do next,
  also need to make TOPIC and GUIDE and send them to project-editorial."
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
