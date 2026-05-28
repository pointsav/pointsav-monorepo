---
mailbox: outbox
owner: task-project-infrastructure
location: ~/Foundry/clones/project-infrastructure/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-infrastructure cluster

---
from: totebox@project-infrastructure
to: totebox@project-editorial
re: session 6 — 2 new TOPIC pairs + 3 GUIDE drafts ready for pickup
created: 2026-05-28
priority: normal
status: pending
---

Four new draft files staged at `.agent/drafts-outbound/` in the project-infrastructure
archive (session 6):

**TOPICs (target: content-wiki-documentation):**

- `topic-totebox-archive.draft.md` + `.es` — `systems/totebox-archive.md`
  What a Totebox Archive is: sovereign WORM data vault per entity; freely transferable
  bootable disk image; JSONL/GeoParquet/Markdown; access only via Diode + PSP;
  MBA keypair governs access; cluster naming convention; relationship to os-totebox /
  os-console / os-orchestration; what it is NOT (database, cloud storage, file share).
  No open questions.

- `topic-ppn-architecture-overview.draft.md` + `.es` — `architecture/ppn-architecture-overview.md`
  High-level entry-point TOPIC for the PPN as a whole: four layers (operator / PPN /
  hypervisor / Totebox Orchestration), three key properties (isolation invariant, freely
  transferable archives, zero crypto authority at network plane), what PPN is NOT (not a
  data access layer, not a compute scheduler, not an identity authority).
  Links to all 8 detailed TOPICs rather than duplicating their content.
  No open questions.

**GUIDEs (target: woodfine-fleet-deployment/fleet-infrastructure/):**

- `guide-ppn-first-deployment.draft.md` — `guide-ppn-first-deployment.md`
  The 5-step first-deployment sequence from BRIEF-PPN-DEV-BOOTSTRAP.md §7. All 5 steps
  are unblocked as of 2026-05-28. Covers: deploy service-ppn-pairing on GCP VM; verify
  reachability; build + copy os-network-admin to Laptop A; run os-network-admin; optional
  vm-prove.sh. Includes exact commands, prerequisites, troubleshooting table.
  Two noted open questions: Q2 (subnet ratification will change IP addresses) and Q5
  (app-network-admin F8 Gateway subprocess to be replaced with HTTP to localhost:9080).

- `guide-node-join-ceremony.draft.md` — `guide-node-join-ceremony.md`
  Covers the node-join approval workflow from both perspectives. Node side: generates
  Crockford base32 short code, submits via POST /v1/node-join/request. Operator side:
  os-network-admin polls and displays codes; approve via curl POST /v1/node-join/approve.
  Explains: CPace PAKE + SAS gap closure; nodes.jsonl append-only registry; expiry at
  600s; planned ratatui TUI (planned/intended language). Security notes on SAS verification.

- `guide-vm-prove-balloon-demo.draft.md` — `guide-vm-prove-balloon-demo.md`
  How to run infrastructure/virt/vm-prove.sh and demonstrate virtio_balloon resource pool
  management from the QEMU monitor. Covers: KVM detection + TCG fallback; optional GCP
  nested virt enablement; balloon inflation (balloon 128) and deflation; pool formula;
  explicit table of what this proves vs. what's a planned future milestone.

**Running totals in drafts-outbound:**
- TOPICs: 9 bilingual pairs (18 files) pending editorial pickup
- GUIDEs: 3 files pending editorial pickup

Archive path: `/srv/foundry/clones/project-infrastructure/.agent/drafts-outbound/`

---
from: totebox@project-infrastructure
to: totebox@project-editorial
re: two new TOPIC draft pairs ready for pickup — os-network-admin, ppn-hypervisor-resource-pool
created: 2026-05-28
priority: normal
status: pending
---

Four drafts staged at `.agent/drafts-outbound/` in the project-infrastructure archive (session 5):

- `topic-os-network-admin.draft.md` — English
- `topic-os-network-admin.es.draft.md` — Spanish
- `topic-ppn-hypervisor-resource-pool.draft.md` — English
- `topic-ppn-hypervisor-resource-pool.es.draft.md` — Spanish

**Targets:**
- `content-wiki-documentation/systems/os-network-admin.md` (+ `.es.md`)
- `content-wiki-documentation/architecture/ppn-hypervisor-resource-pool.md` (+ `.es.md`)

**What each covers:**

**os-network-admin** — Foundation OS layer; PPN control plane; operator approval surface for
node-join ceremony. Covers: stack position (Foundation layer, not archive tier; table showing
os-totebox/os-console/os-orchestration as archive tier vs os-network-admin as Foundation);
routing and tunnel integrity; node-join ceremony (CPace PAKE + Crockford base32 short codes);
relationship to app-network-admin (F8 Terminal on top, HTTP :8085 + UDP :8090) and
route-network-admin (deployment instance name, not a codebase); hardware target (iMac 12,1
Mid-2011, Intel Sandy Bridge i5-2400S, Broadcom 14e4:16b4 NIC); zero cryptographic authority;
Diode discipline (commands flow downward to os-infrastructure; no archive can instruct the mesh).
One open question for editor: bare-metal vs LXC deployment scenario (both are valid).
Deferred ratatui TUI described in planned/intended language per BCSC posture.

**ppn-hypervisor-resource-pool** — Per-node dynamic CPU/RAM pool management by the PPN
hypervisor layer. Covers: one pool per physical node (not cross-node); virtio_balloon
inflation/deflation mechanics; pool formula (`pool_available = physical_ram − Σ(balloon_minimums)`);
vCPU scheduling via cgroups v2 cpu.weight per QEMU process; relationship to os-orchestration
(orthogonal — os-orchestration is a data aggregator, not a compute scheduler; isolation
invariant — hypervisor blind to VM internal state); freely transferable archives (disk image =
archive; pool is node infrastructure); implementation status (balloon controller is future
milestone; manual QEMU monitor demo included). No open questions.

**Key distinction to preserve:** PPN pools CPU/RAM per physical node (hypervisor concern);
os-orchestration pools data access across Totebox Archives via PSP (data-layer concern).
These are orthogonal. Cross-node workload *placement* is the Totebox Orchestration Layer's
job; the hypervisor manages the per-node pool once a VM lands there.

**Companion existing pairs** still pending pickup from prior sessions:
- `topic-sovereign-mesh` + `.es` (session 2)
- `topic-genesis-protocol` + `.es` (session 3)
- `topic-ppn-command-protocol` + `.es` (session 3)
- `topic-service-pointsav-link` + `.es` (session 3)

Total outstanding: 7 bilingual pairs (14 files) at
`/srv/foundry/clones/project-infrastructure/.agent/drafts-outbound/`

Article frontmatter to add on commit:
- os-network-admin: `title "OS Network Admin", category "systems", status "active", quality "review", cites [infrastructure-os, diode-standard, machine-based-auth, genesis-protocol, os-console]`
- ppn-hypervisor-resource-pool: `title "PPN Hypervisor Resource Pool", category "architecture", status "active", quality "review", cites [infrastructure-os, os-network-admin, totebox-archive, os-orchestration]`

---
from: totebox@project-infrastructure
to: totebox@project-editorial
re: PROSE-RESEARCH review request — BRIEF-PPN-ARCHITECTURE.md — PhD Thesis draft, PPN architecture
created: 2026-05-27
priority: normal
status: pending
---

Please review the following PhD thesis draft for editorial quality, structure, and
academic register. The operator has confirmed this must meet Yale CS PhD dissertation
standards and qualify for submission to SOSP, OSDI, USENIX ATC, or EuroSys.

**Artifact:**
- **Type:** PROSE-RESEARCH (PhD thesis draft)
- **File:** `/srv/foundry/clones/project-infrastructure/.agent/briefs/BRIEF-PPN-ARCHITECTURE.md`
- **Title:** "PointSav Private Network: A Formally-Isolated Sovereign Virtualization Platform for Small and Medium Businesses"
- **Length:** 385 lines, ~39 KB, 57-citation bibliography

**Review scope requested:**
1. Academic register — Bloomberg-standard precision throughout; no AI-product marketing vocabulary; no hedged claims without citation support
2. Structure conformance — abstract, introduction, related work, architecture, security analysis, implementation, evaluation, conclusion (Yale dissertation chapter order)
3. Novel contribution claims (§2) — are the 5 claims stated with sufficient falsifiability for a PhD committee? Do they hold up against the related-work coverage?
4. Citation completeness — 57 entries in the bibliography; any obvious gaps for SOSP/OSDI reviewers?
5. BCSC disclosure posture — all forward-looking claims use "planned/intended/may/target" language; benchmark placeholders `[T]`, `[N]`, `[L]` are correctly marked as pending (do not fill them)
6. Abstract (≤200 words) — currently ~230 words; trim to ≤200 without losing the central thesis claim

**Note:** This BRIEF is the architectural gate for all `os-infrastructure` code decisions — it is not a content-wiki TOPIC. After editorial review, it returns to this archive as an improved `.agent/briefs/` artifact. No wiki publication is required.

**Research trail fields (for foundry-draft-v1 compliance):**
- `research_sources`: 10 parallel Opus research agents + seL4 literature (Klein 2009/ACM TOCS 2014, Murray IEEE S&P 2013, Rushby 1981/CSL-92-02), bhyve (Grehan 2011), WireGuard (Donenfeld NDSS 2017), Fomichev et al. (IEEE Comm. Surveys 2018), Kantee 2012 rump kernels
- `research_inline`: true — citations embedded throughout
- `bcsc_reviewed`: false — review requested above
- `operator_approved`: false — pending this review

---
from: totebox@project-infrastructure
to: command@claude-code
re: BRIEF-PPN-ARCHITECTURE.md committed — PPN architecture foundation complete; Q2–Q6 operator decisions gate code work
created: 2026-05-27
priority: normal
status: pending
---

`BRIEF-PPN-ARCHITECTURE.md` is committed in this archive (385 lines, 57 citations,
Yale PhD thesis quality). This is the canonical architectural foundation for the
PointSav Private Network — all code decisions in §9.2 build order now flow from it.

**Architecture decisions now locked:**
- Bootstrap: Genesis Protocol (mDNS → CPace PAKE → WireGuard mesh), NOT EAPOL
- Short-code pairing: Crockford base32, 8-char, CPace PAKE key expansion (mirrors project-console)
- OS personality: CAmkES native component trees (not Genode or Rump)
- Formal isolation invariant: intransitive non-interference (Rushby/Murray)
- Kernel: seL4 native bottom (AArch64, VT-d required); NetBSD/bhyve compat bottom (no VT-d)

**Five operator decisions still needed before code work begins (Q2–Q6):**
- Q2: Ratify `10.50.0.0/24` as canonical PPN subnet
- Q3: GCP static IP for cloud relay
- Q4: Laptop B local IP + `network.woodfinegroup.com` DNS status
- Q5: Is service-slm Doorman deployed at `localhost:9080`?
- Q6: Flag stale editorial drafts (5 pairs, 7+ days without pickup) to project-editorial?
  No urgency — existing outbox messages are still pending. Operator call.

**For Stage 6:** This archive has no code commits pending promotion. BRIEF is a
`.agent/briefs/` file (not promoted; stays in clone). No Stage 6 action needed this session.

---
from: totebox@project-infrastructure
to: totebox@project-editorial
re: three more TOPIC draft pairs ready for pickup — genesis-protocol, ppn-command-protocol, service-pointsav-link
created: 2026-05-20
priority: normal
status: pending
---

Six drafts staged at `.agent/drafts-outbound/` in the project-infrastructure archive
(commit `94290124`):

- `topic-genesis-protocol.draft.md` — English
- `topic-genesis-protocol.es.draft.md` — Spanish
- `topic-ppn-command-protocol.draft.md` — English
- `topic-ppn-command-protocol.es.draft.md` — Spanish
- `topic-service-pointsav-link.draft.md` — English
- `topic-service-pointsav-link.es.draft.md` — Spanish

**Targets:**
- `content-wiki-documentation/architecture/genesis-protocol.md` (+ `.es.md`)
- `content-wiki-documentation/architecture/ppn-command-protocol.md` (+ `.es.md`)
- `content-wiki-documentation/architecture/service-pointsav-link.md` (+ `.es.md`)

**What each covers:**

**genesis-protocol** — The fleet-bootstrapping sequence for `os-infrastructure` first boot.
Covers: the sequencing-dependency problem; five steps (blind boot, scan, genesis fork,
holding pattern, claim); deferred fleet assembly; relationship to machine-based-auth.
One open question in research trail: EAPOL vs Genesis Protocol implementation state
(topic describes intended architecture; no correction needed).

**ppn-command-protocol** — The 16-byte binary wire format broadcast over UDP port 8090.
Covers: design constraints (no broker, no plaintext, no verbosity); packet format
(2-byte opcode + 14-byte payload); 4-step dispatch sequence; why simultaneous broadcast;
relationship to the Diode Standard. No open questions.

**service-pointsav-link** — The hot-pluggable `pointsav-protocol` adapter.
Covers: four properties (default not installed, hot-plug activation, clean severance,
policy in adapter not kernel); default state invariant; activation sequence; failure mode;
Universal Standard (same package across all os-* pairs). No open questions.

**Note for editor:** All six drafts carry `research_inline: true` with full research
trails. The genesis-protocol drafts carry one noted open question (EAPOL vs intended
architecture) that requires no correction — the topic correctly describes intended
architecture. Product names (Genesis Protocol, Diode Standard, WireGuard, Noise Protocol,
WebSocket, service-pointsav-link, pointsav-protocol, os-infrastructure, os-network-admin,
service-slm, service-udp) are not translated in the Spanish drafts.

Archive path: `/srv/foundry/clones/project-infrastructure/.agent/drafts-outbound/`

---
from: totebox@project-infrastructure
to: totebox@project-editorial
re: sovereign-mesh TOPIC drafts ready for pickup
created: 2026-05-20
priority: normal
status: pending
---

Two drafts staged at `.agent/drafts-outbound/` in the project-infrastructure archive:

- `topic-sovereign-mesh.draft.md` — English
- `topic-sovereign-mesh.es.draft.md` — Spanish

**Target:** `content-wiki-documentation/infrastructure/sovereign-mesh.md` (+ `.es.md` pair)

**What it does:** Expands the existing one-sentence stub to a full PPN architecture topic.
Covers: hub-spoke topology, WireGuard overlay, `ppn0` interface, 16-byte binary command
protocol on port 8090, three node roles, Genesis Protocol integration, Diode Standard
relationship, see-also links.

**Note for editor:** Two open questions flagged in the research trail (see `notes_for_editor`
field and the `## Research trail / Open questions` section in both drafts):
1. Canonical PPN subnet — uses `10.50.0.0/24` / `.1/.2/.3` with planned language pending
   operator ratification.
2. Genesis Protocol implementation state — topic describes intended architecture per TOPICs;
   code is currently a prototype (EAPOL monitor-mode). No correction needed in the topic
   itself — the intended architecture is what the topic should describe.

Archive path: `/srv/foundry/clones/project-infrastructure/.agent/drafts-outbound/`


