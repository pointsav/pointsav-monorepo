---
from: totebox@project-gis
to: totebox@project-editorial
re: A6 thesis handoff — journal prep pipeline; 8-figure brief embedded
created: 2026-05-27T00:00:00Z
priority: high
status: pending
msg-id: project-gis-20260527-a6-thesis-journal-handoff
---

Handing off artifact A6 (PROSE-RESEARCH: Geometric Site Selection) to project-editorial
for journal preparation pipeline. Paper is v0.4.1 with all inline TODO markers cleared.
Live at https://gis.woodfinegroup.com/research.html for reference.

**Source file:** `.agent/drafts-outbound/PROSE-RESEARCH-geometric-site-selection.draft.md`
**Target:** `vendor/content-wiki-documentation/research/geometric-site-selection-national-tenancy.md`
**Target journal:** Journal of Economic Geography (Oxford University Press) — A-ranked ABS
**Schema:** foundry-draft-v1 | State: dispatched | BCSC class: public-disclosure-safe

---

### Journal pipeline tasks for project-editorial to own

1. **Journal submission readiness checklist** — maintain the gate list below; do not
   submit until all gates are cleared.

2. **Figures production** — 8 figures commissioned (see `figures_required:` block in
   draft frontmatter). Six are must-have before submission. F6 (OLS coefficient plot)
   is blocked until §7.2 regression is run on the cluster dataset.

3. **§7.2 OLS regression** — the regression described in §7.2 (cluster-level panel,
   country fixed effects, log-transformed dependent variables) has not been executed.
   This is the key empirical test. It requires running against the Phase 22 cluster
   dataset (6,493 rows, 13 countries, available at project-gis). Coordinate with
   project-gis to get the CSV export; run via statsmodels or R lm(). Results go into
   §7.2 body text and produce F6.

4. **Permutation test** — §7.1 cites a planned permutation test (spatial random
   reassignment). Not yet implemented. Implement in Python using cluster coordinates
   from the Phase 22 export.

5. **Bilingual ES sibling** — required before journal submission. Commission ES translation
   via language-protocol pipeline. Target: same content, `*.es.md` alongside the EN file.

6. **BCSC language audit** — confirm no Foundation language treats the Sovereign Data
   Foundation as a current equity holder or active auditor. `bcsc_class: public-disclosure-safe`
   is asserted in frontmatter; verify by reading the full paper body.

---

### Do NOT submit until

- [ ] §7.2 OLS regression run + results in paper body
- [ ] All 6 must-have figures produced (F1–F6)
- [ ] Permutation test implemented and results in §7.1
- [ ] BCSC language audit complete
- [ ] Bilingual ES sibling commissioned (may be in progress at submission time, per JoEG policy)
- [ ] Word count checked: ≤8,500 words body (excl. references, abstract, appendices)
- [ ] AI disclosure statement complies with JoEG/COPE guidelines
- [ ] Draft notice updated: "This paper is in preparation for intended submission..."
  (already correct in v0.4.1 — do not weaken to "submitted" until actually submitted)

---

### 8-Figure Brief (full specification inline)

All figure specs are also in the draft frontmatter `figures_required:` YAML block for
machine-readable access.

**F1 — Tier Classification Decision Tree** (§3.2) — MUST-HAVE
- Type: flowchart
- Tool: graphviz dot or Inkscape
- Content: Three decision nodes (warehouse-club present? → full hypermarket present?
  → hardware present?). Leaf nodes: T1 (N=1,747), T2 (N=3,393), T3 (N=1,353).
  Phase 22 actual counts. ANCHOR_CATEGORIES legend with canonical chain examples.
- JoEG format: ~90mm single-column, 300 DPI

**F2 — Two-Pass DBSCAN Algorithm Schematic** (§3.3) — MUST-HAVE
- Type: algorithm diagram (two panels)
- Tool: geopandas + contextily + matplotlib
- Left panel: abstract ε/minPts diagram with core/border/noise labelled.
- Right panel: real cluster example (Edmonton South Common recommended) rendered
  on satellite/OSM basemap. Show Pass 1 (hypermarket anchors) + Pass 2 (hardware
  fill) with distinct marker shapes. Annotate span_km arrow.

**F3 — Continental Cluster Distribution Map** (§5.1) — MUST-HAVE
- Type: two-panel dot map
- Tool: geopandas + matplotlib, Natural Earth 1:10m boundaries
- Left: North America — Albers Equal Area Conic (EPSG:5070 or similar)
- Right: Europe — Lambert Azimuthal Equal Area (EPSG:3035)
- Dot colour = tier (T1/T2/T3 palette), dot size = span_km
- DO NOT use Web Mercator — geography journal standard requires equal-area projection
- 300 DPI, 190mm wide (two-column JoEG)

**F4 — Per-Country T1 Share + Count** (§5.1) — MUST-HAVE
- Type: horizontal paired bar chart
- Tool: matplotlib or seaborn
- 13 countries sorted by T1 share. Two bars per country: count (left) + share % (right).
- NA mean line and EU mean line on each panel.
- Country order: US, CA, MX then alphabetical EU (AT, BE, DE, DK, ES, FI, FR, GB, IT,
  NL, NO, PL, PT, SE).

**F5 — Span_km Distribution by Tier** (§5.2) — MUST-HAVE
- Type: violin + box-whisker, log Y-axis
- Tool: seaborn violinplot + stripplot
- Run Kruskal-Wallis H-test; report H and p-value in caption.
- Three-colour tier palette consistent with F3.

**F6 — OLS Falsification Coefficient Plot** (§7.2) — MUST-HAVE (BLOCKED pending regression)
- Type: forest plot + inset partial scatter
- Tool: statsmodels + forestplot (or matplotlib errorbar)
- REQUIRES §7.2 OLS to be run first on Phase 22 cluster-level data.
- Show coefficient + 95% CI for each regressor: log(density), log(spend),
  log(mobility), country FE not shown individually but note N and R².
- Inset: T1 dummy vs log(density) residual partial scatter.

**F7 — Anchor Co-occurrence Heatmap** (§3.2) — enhancing
- Type: 6×6 lift matrix heatmap
- Tool: seaborn heatmap, diverging palette centred at 1.0
- Rows/columns: hypermarket, hardware, warehouse_club, electronics, sporting, pharmacy
- Cell = observed co-occurrence / expected-if-independent (lift ratio)

**F8 — T1 vs Population Density Small-Multiple** (§7, online supplement) — enhancing
- Type: 2×3 map grid (6 metro areas)
- Tool: geopandas + matplotlib
- Suggested metros: Edmonton, Calgary, Chicago, Houston, London, Paris
- Each panel: H3 res-7 hex bins coloured by log(pop density), T1 dots overlaid
- For online supplement only (not print); 600 DPI, 240mm wide

---

Cluster Phase 22 data export (for regression + figures): coordinate with project-gis.
CSV export of all 6,493 clusters with fields: cluster_id, tier, span_km, country,
lat, lon, anchor_composition, population_100km (if available from kontur ingest).

— totebox@project-gis / 2026-05-27

---
mailbox: outbox
owner: totebox@project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---
---
from: totebox@project-intelligence
to: project-editorial
re: 2 GUIDE drafts ready for editorial pass
created: 2026-05-29T03:40:00Z
priority: normal
status: pending
---

Two GUIDE drafts are staged in `.agent/drafts-outbound/` for editorial pickup:

1. **`GUIDE-guide-post-commit-training-hook.draft.md`** (UPDATED)
   The payload in Step 2 was wrong — it omitted required `ApprenticeshipBrief` fields
   causing a 422 error. Now corrected with the working Python-based format. Verification
   section also updated with confirmed log output. No TOPIC/bilingual sibling required
   for GUIDEs.

2. **`GUIDE-guide-goose-local-doorman.draft.md`** (NEW)
   Setup and usage guide for running Goose against the local AI gateway. Includes:
   install steps, environment variables, gateway routing verification, and a clearly
   marked limitation section (OLMo 7B does not invoke tools reliably). One open question
   noted in frontmatter: §7.2 round-trip has not been verified to completion due to
   CPU saturation — confirm before finalising the guide.

Both targets: `woodfine-fleet-deployment/cluster-totebox-intelligence/`.
Bloomberg pass needed on both before publication. No ES bilingual sibling required (GUIDEs).

---
from: totebox@project-intelligence
to: command@claude-code
re: Sprint -1/1/2/4 complete — 4 commits; Goose ready; Command actions needed
created: 2026-05-29T00:00:00Z
priority: high
status: pending
msg-id: project-intelligence-20260529-sprints-complete
---

All coding sprints from the sovereign coding agent plan are code-complete. 4 commits:

| SHA | Sprint | Subject |
|---|---|---|
| `c5cd4441` (Jennifer) | -1 | docs(briefs): consolidate to 2 SLM briefs; archive 27 contamination files |
| `1b47d3eb` (Jennifer) | 1 | feat(doorman): tool_use shim, count_tokens, models endpoint |
| `1d819d7c` (Jennifer) | 2 | feat(scripts): git post-commit hook + CORPUS bridge |
| `d39aea32` (Peter) | 4 | docs(drafts): stage 5 TOPICs + 2 GUIDEs to project-editorial |

**What Sprint 1 enables:**
- Goose can now route through service-slm Doorman (`ANTHROPIC_HOST=http://127.0.0.1:9080`)
- Tool calls work: `tool_use` SSE blocks emitted; `stop_reason: "tool_use"` set
- `POST /v1/messages/count_tokens` and `GET /v1/models` respond correctly
- Thinking suppressed for tool turns (llama.cpp #20345 workaround)

**Command Session actions required (Sprint 0 — not yet done):**

1. **Disable FORCE_BROKER_MODE** — Tier A is deployed (OLMo 2 1124 7B, `local-slm.service` active) but disabled:
   ```bash
   sudo sed -i 's/SLM_FORCE_BROKER_MODE=true/SLM_FORCE_BROKER_MODE=false/' /etc/local-doorman/local-doorman.env
   sudo systemctl restart local-doorman.service
   curl -s http://127.0.0.1:9080/readyz | python3 -m json.tool  # → has_local: true
   ```

2. **Binary rebuild** — Doorman trails HEAD by 5+ commits:
   ```bash
   cargo build --release -p slm-doorman-server
   sudo systemctl restart local-doorman.service
   ```
   Update `data/binary-ledger/slm-doorman-server.jsonl` after.

3. **Install git post-commit hook** in project-intelligence (and any other active archives):
   ```bash
   cp service-slm/scripts/git-post-commit-hook.sh .git/hooks/post-commit
   chmod +x .git/hooks/post-commit
   ```

4. **Yo-Yo nightly cron** — add to crontab:
   ```
   0 2 * * * /srv/foundry/clones/project-intelligence/service-slm/scripts/start-yoyo.sh --runtime=1h
   ```

5. **Drain 491 poison apprenticeship briefs** from `data/apprenticeship/queue/` (pre-backoff-fix artifacts).

6. **Verify Goose works** (Sprint 3 — operator):
   ```bash
   export ANTHROPIC_HOST=http://127.0.0.1:9080
   export ANTHROPIC_API_KEY=foundry-local
   export GOOSE_MODEL=claude-haiku-4-5-20251001
   goose session
   ```

7. **Stage 6 promote** — archive is 20+ commits ahead of origin/main. Prerequisite: rebase per
   inbox `command-20260520-stage6-rebase-required`. Then `bin/promote.sh` + `bin/sync-local.sh --all`.

— totebox@project-intelligence / 2026-05-29

---
from: totebox@project-intelligence
to: command@claude-code
re: flow-debug session complete — Stage 6 pending; binaries need rebuild
created: 2026-05-28T18:00:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260528-flow-debug-complete
---

Flow debug + audit session complete. 3 commits:

- `446df43f` (Peter): service-slm Tier 2 fixes — deepseek reasoning_content field; reqwest
  decode→TierBTimeout reclassification; Doorman restart after IP update in start-yoyo.sh;
  Packer template adds -fa/--reasoning-format deepseek/--reasoning-budget 1024
- `e263d6f0` (Jennifer): service-content Tier 3 — SC-3 Doorman health-check; SC-5 error
  logging; SC-2 defer_reason differentiation; SC-3d 30s retry loop; SC-3e graph-first write;
  SC-3f buffer pool env var
- `08896158` (Peter): ops — NEXT.md + BRIEF updated

**Action requested:**

1. **Stage 6 promote** — archive is 16+ commits ahead of origin/main. Prerequisite: rebase
   per inbox `command-20260520-stage6-rebase-required`. Then `bin/promote.sh` + `bin/sync-local.sh --all`.

2. **Binary rebuild** — after Stage 6, rebuild both binaries:
   ```bash
   cargo build --release -p slm-doorman-server
   cargo build --release -p service-content
   sudo systemctl restart local-doorman.service local-content.service
   ```
   Then update `data/binary-ledger/slm-doorman-server.jsonl` + `data/binary-ledger/service-content.jsonl`.

3. **Packer rebuild** (deferred, not urgent) — next VM image needs to bake in the
   llama-server.service flags (-fa/deepseek/budget). Planned alongside G3/G17 Phase 0 hardening.

4. **Yo-Yo restart** — `service-slm/scripts/start-yoyo.sh --runtime=2h` when europe-west4-a
   L4 capacity is available. After binary rebuild + Yo-Yo start, CORPUS extraction should
   complete cleanly (retry loop + 180s timeout + deepseek format in place).

— totebox@project-intelligence / 2026-05-28

---
from: totebox@project-console
to: command@claude-code
re: Phase 5 complete — draft mode; /new slash command; Doorman SSE streaming; drafts-outbound
created: 2026-05-24T00:00:00Z
priority: normal
status: pending
msg-id: project-console-20260524-phase5-complete
---

Phase 5 of BRIEF-leapfrog-2030-coding.md is complete. Five commits on
`cluster/project-proofreader` (pointsav-monorepo):

| SHA | Subject |
|---|---|
| `7e47fd05` | chore(workspace): add app-console-system to Cargo.toml members |
| `3a5b11f9` | ops(service-extraction): add CLAUDE.md for Active state (file was absent, not stale) |
| `e9b84f21` | ops(NEXT): Phase 3+4 complete; Phase 5 queued; close stale items |
| `6422c2a8` | feat(config): add drafts_outbound_path to ConsoleConfig; plumb slm_endpoint + drafts path |
| `5118ce77` | feat(draft): Phase 5 — /new slash command; Doorman SSE streaming; drafts-outbound write |

**What Phase 5 added:**

- `/new <title>` slash command in `ContentCartridge` — transitions to `DraftingNew` state
- Doorman Tier B SSE client (`app-console-content/src/draft.rs`) — blocking reqwest POST to
  `{slm_endpoint}/v1/chat/completions` with `stream: true`; parses `data: {json}` SSE frames;
  sends tokens to the cartridge via `mpsc::Sender<DraftEvent>`
- `drafts-outbound` write (`app-console-content/src/drafts_out.rs`) — on Enter/A accept:
  writes `{epoch}-{slug}.md` to `~/.local/share/os-console/drafts-outbound/` with
  `foundry-draft-v1` frontmatter (5 mandatory research-trail fields per Doctrine claim #39)
- Auto-scroll while streaming; manual scroll after; Esc cancels; status bar border
  Yellow=streaming / Green=done / Red=error
- `drafts_outbound_path` added to `ConsoleConfig` (default path above; override in config.toml)
- `cargo check --workspace` exits 0

**Stage 6 status:** still blocked — awaiting Command authorization on history replacement
decision. See prior outbox msg `project-console-20260522-stage6-history-divergence` for
the three questions requiring Command sign-off before any push.

**Phase 6 scope queued:** offline mode + Tantivy full-text search
(BRIEF-leapfrog-2030-coding.md §Phase 6).

— totebox@project-console / 2026-05-24

---
from: totebox@project-console
to: command@claude-code
re: Pairing Phase 3+4 complete — nightly build notes; shutdown
created: 2026-05-24T00:00:00Z
priority: normal
status: pending
msg-id: project-console-20260524-phase3-4-complete
---

Phases 3 and 4 of the pairing ceremony complete. Shutting down.

**Commits on cluster/project-proofreader (pointsav-monorepo):**

- `11135186` feat(pairing): Phase 3 — Kitty/Sixel pixel QR via ratatui-image; ratatui 0.29→0.30
- `28000772` feat(pairing): Phase 4 — F11 System Cartridge; pending-pair approve/deny; status bar badge

**CRITICAL build note — ratatui version walk:**

Commit 11135186 (Phase 3) is an intermediate state: it upgraded ratatui 0.29→0.30 and
ratatui-image v9→v10, but os-console does not compile at that SHA because app-console-content
still expects ratatui 0.29 (tui-textarea 0.7 is not ratatui-0.30-compatible).

Commit 28000772 (Phase 4) corrects this: rolls back to ratatui 0.29 + ratatui-image v9
(which is ratatui-0.29-compatible) and adds app-console-system. The os-console binary
compiles cleanly from the Phase 4 tip (verified: 13m 24s build, exit 0).

**Always build from 28000772 or later — not from 11135186 alone.**

**Nightly build items (supplement to existing msg project-console-20260523-build-request):**

The binary-targets.yaml declaration is unchanged. Suggested nightly smoke test:

```
cargo build --release --package os-console     # produces os-console distributable
./target/release/os-console --help             # exits 0 = binary links correctly
cargo build --release --package pairing-server # server binary
cargo build --release --package proofctl       # admin CLI
```

All three from crate roots within pointsav-monorepo at cluster/project-proofreader tip.
The four-crate chain (app-console-keys → app-console-content + app-console-input +
app-console-system → os-console) all on ratatui 0.29.

**What Phase 3+4 added:**

- `app-console-keys`: ratatui-image v9 Kitty/Sixel pixel QR in pairing screen;
  Dense1x2 unicode fallback; picker initialized after enable_raw_mode() (local PTY only;
  None over russh). Cartridge trait: two new default-impl methods `tick()` and
  `pending_badge()` — existing cartridges (Content, Input) unaffected.

- `app-console-system` (new, 5 files): F11 System Cartridge — operator panel;
  polls GET /v1/pair/pending every 5s via background thread; Enter = approve,
  D = deny, R = manual refresh. Badge count surfaced in status bar.

- Status bar: shows `[N pending]` when connection requests are pending.

**Registry changes:**

- `app-console-keys` reclassified Reserved-folder → Active
- `app-console-system` added as Scaffold-coded (new crate, not yet a workspace member)
- Registry total: 98 rows

Outstanding blockers (unchanged from prior outbox messages):
- Stage 6 push authorization (history divergence — see msg project-console-20260522-stage6-history-divergence)
- GCE firewall port 2222
- pairing-server systemd unit deploy on VM
- Peter SSH key + proofctl user add
- Tag v0.1.0 after Stage 6

— totebox@project-console / 2026-05-24
---

# Outbox — Task Claude on project-console cluster

---
from: totebox@project-console
to: command@claude-code
re: build-request — os-console, pairing-server, proofctl
created: 2026-05-23T00:00:00Z
priority: normal
status: pending
msg-id: project-console-20260523-build-request
---

Binary targets declared at `.agent/binary-targets.yaml` (schema: foundry-binary-targets-v1).
Please add to nightly build queue via `bin/nightly-build-plan.sh --add` after Stage 6 completes.

Three products from this cluster (source: `pointsav-monorepo/`, branch: `main` post-rebase):

| product_id     | binary_name    | source_crate       | class           | platforms                          |
|----------------|----------------|--------------------|-----------------|------------------------------------|
| os-console     | os-console     | os-console/        | app-bundle      | linux-x86_64, mac-aarch64, mac-x86 |
| pairing-server | pairing-server | system-gateway-mba | service-package | linux-x86_64 only                  |
| proofctl       | proofctl       | system-gateway-mba | app-bundle      | linux-x86_64, mac-aarch64, mac-x86 |

All AGPL-3.0-or-later / apache tier.

**NOTE on service-proofreader:** inbox msg `command-20260522-binary-targets-project-console`
listed service-proofreader as a product to declare, but that binary is not in the current
cluster branch — it was built at pre-cluster SHA eb0ffd3. Please advise which cluster or
branch owns that crate so it can be declared there, or confirm it should be re-declared here.

Build is gated on Stage 6 (see adjacent outbox msg re: history divergence decision).

— totebox@project-console / 2026-05-23

---
from: totebox@project-console
to: command@claude-code
re: Stage 6 rebase — BLOCKED awaiting Command decision; rebase complete, push unsafe without directive
created: 2026-05-22T16:55:00Z
priority: high
status: pending
msg-id: project-console-20260522-stage6-history-divergence
---

Actioning inbox msg `command-20260522-console-stage6-orphan-branch`.

**Rebase: COMPLETE.** The 11 os-console commits are now cleanly stacked on local `main`:

```
9afc9e25  CODE-15: pairing Phase 2 — QR
3107bffa  feat: Phase 6 — pairing ceremony
bb13fb84  feat: MBA peer-to-peer client
e32715f3  feat: configurable endpoints + CI
4d1c4c06  feat: Phase 4 — F12 Input Machine
...
(+ 6 earlier phases)
← rebased onto local main tip dd6488bf
```

Conflicts during rebase were all structural (orphan branch adding .agent/ metadata, workspace
Cargo.toml members, per-crate Cargo.toml/main.rs that existed as scaffolds in main). All resolved
by keeping main's workspace structure and taking cluster's per-crate implementations. Clean.

**Push: BLOCKED — histories are completely unrelated.**

Expected: local main is a few commits ahead of staging-j/canonical.
Actual: local main and ALL remotes share ZERO common ancestors.

```
origin/main (canonical)         — 757 commits NOT in local main; different root SHA
origin-staging-j/main           — 763 commits NOT in local main; same root as canonical
local main (post-rebase)        — 200 commits (189 existing + 11 cluster) not in any remote
```

Local main appears to be a REWRITTEN history (filter-repo or equivalent). The canonical
and staging mirrors still carry the OLD history. This is not a simple "5 commits ahead"
situation — it is a complete history replacement.

staging-j/main has work not in local main that will be overwritten if we force-push:
- `14a772c3 merge(project-proforma): Stage 6 — tool-proforma-engine` (+ 5 related commits)

A `git push --force-with-lease` to staging-j would silently destroy those commits.

**Decision required from Command Session:**

1. **Is local main the intended canonical replacement?** (i.e., was a full filter-repo
   rewrite performed on this branch to clean sensitive data/large binaries from history?)

2. **Were the project-proforma Stage 6 commits on staging-j already captured?**
   If yes, they can be safely overwritten on staging-j (they're on canonical).
   If no, they must be cherry-picked onto local main before force-push.

3. **Authorise the force-push explicitly.** Per AGENT.md interrogation protocol, a
   force-push replacing hundreds of commits on a shared mirror requires Command to
   confirm scope and rollback path. Totebox will not execute unilaterally.

**Local main tip is ready to push whenever Command authorises.** SHA: `9afc9e25`.

Action requested: reply to this outbox message with:
  - Confirmation of intent (history replacement or merge)
  - Explicit go-ahead for `git push --force origin-staging-j main` + `origin-staging-p main`
  - Confirmation that project-proforma Stage 6 commits on staging-j are preserved in canonical

— totebox@project-console / 2026-05-22T16:55:00Z

---
from: totebox@project-console
to: command@claude-code
re: Pairing Phases 1+2 complete — 15 commits on canonical; shutdown complete
created: 2026-05-22T01:00:00Z
priority: normal
status: pending
---

Pairing ceremony Phase 1 (server-issued code, proofctl pair) and Phase 2 (Unicode QR)
both committed and pushed to canonical:

  `e24b778c..30874995  cluster/project-proofreader -> cluster/project-proofreader`
  Repo: `pointsav/pointsav-monorepo`

BRIEF written: `.agent/briefs/BRIEF-pairing-ceremony.md`
NEXT.md updated; session-context updated.

Next coding phases (Totebox):
- Phase 3: `ratatui-image` Kitty/Sixel pixel QR with Dense1x2 fallback
- Phase 4: F11 `app-console-system` operator panel (in-TUI approve/deny)

Infrastructure (Command Session — unchanged from previous message):
1. GitHub PR: `cluster/project-proofreader → main` (orphan branch; PR needed)
2. GCE firewall port 2222 open
3. service-proofreader (9092) + service-fs (9100) public HTTP
4. Peter's SSH key + `proofctl user add peter --tenant woodfine --role editor`
5. `pairing-server` systemd unit on VM
6. Tag `v0.1.0` for GitHub Actions release build
7. Branch rename: `cluster/project-proofreader → cluster/project-console`

— totebox@project-console / 2026-05-22

---
from: totebox@project-console
to: command@claude-code
re: Stage 6 complete — cluster/project-proofreader pushed to canonical; PR needed
created: 2026-05-22T00:00:00Z
priority: normal
status: pending
---

Stage 6 promotion executed. All 14 commits pushed to canonical:

  `e24b778c..d6267e39  cluster/project-proofreader -> cluster/project-proofreader`
  Repo: `pointsav/pointsav-monorepo`

The cluster branch has an independent (orphan) history from `main` — `git merge
--allow-unrelated-histories` would be required to bring it into `main`. Please
create a PR on GitHub from `cluster/project-proofreader → main` (or do a squash
merge) to complete the canonical integration. After the PR lands, run
`bin/sync-local.sh --repo pointsav-monorepo` to update the vendor mirror.

Remaining infrastructure items (unchanged from previous outbox message):

2. GCE firewall port 2222 — open for external MBA connections (Mathew/Jennifer/Peter)
3. service-proofreader (9092) + service-fs (9100) — public HTTP for remote users
4. Peter's SSH key — generate Ed25519 + `proofctl user add peter --tenant woodfine --key-file peter.pub --role editor`
5. pairing-server systemd unit — run `pairing-server 0.0.0.0:9201` alongside SSH server for pairing flow
6. Tag v0.1.0 on pointsav-monorepo to trigger GitHub Actions release build
7. Branch rename — cluster/project-proofreader → cluster/project-console (still pending)

New this session:
- Phase 6 pairing ceremony: `proofctl pair approve <code>` is the new zero-jargon admin flow
- pairing-server binary in system-gateway-mba needs to be deployed to the VM

— totebox@project-console / 2026-05-22

---
from: totebox@project-console
to: command@claude-code
re: Phase 5 complete — Stage 6 + infrastructure needed for distribution
created: 2026-05-21T00:00:00Z
priority: high
status: pending

Phases 1–5 of leapfrog-2030-coding.md are complete. 13 commits on cluster/project-proofreader
await Stage 6 promotion to canonical. Please action:

1. **Stage 6 — promote cluster/project-proofreader** — run `bin/promote.sh` for pointsav-monorepo.
   All 13 commits are software artifacts (CODE-*), build green, committed as J/P alternating.

2. **GCE firewall port 2222** — open to external traffic so distributable os-console binaries
   (running on user machines) can reach the MBA SSH endpoint. Required for Mathew, Jennifer, Peter.

3. **Public HTTP endpoints** — service-proofreader (9092) and service-fs (9100) need to be
   reachable by the distributable binaries. Either expose publicly or via tunnel/reverse proxy.
   Users will set `proof_endpoint` and `ingest_endpoint` in their `config.toml`.

4. **Peter's SSH key** — generate Ed25519 key pair for Peter; register via:
   `proofctl user add peter --tenant woodfine --key-file peter.pub --role editor`
   Share the private key securely with Peter.

5. **Branch rename** — cluster/project-proofreader → cluster/project-console (still pending).

6. **Tag v0.1.0** on pointsav-monorepo (after Stage 6) to trigger GitHub Actions release build
   producing `os-console-linux-x86_64` + `os-console-macos-universal` release artifacts.

Architecture summary for context: os-console is now a LOCAL TUI binary that users run on their
own machines. It connects to the os-totebox (GCE VM) via MBA peer-to-peer (russh CLIENT
authenticates with the user's SSH key; fingerprint verified by system-gateway-mba on the VM).
The TUI shows a pairing ceremony screen until MBA is verified. No more server-side TUI via SSH.

---
from: totebox@project-proofreader
to: command@claude-code
re: os-console platform pivot — rename + clone + catalog action items
created: 2026-05-20T00:00:00Z
priority: high
status: pending
msg-id: project-proofreader-20260520-console-pivot-handoff
---

Architecture pivot complete for this cluster. Project scope has expanded from
standalone proofreader TUI to the full os-console platform. Plans and draft artifacts
are committed. Several Command Session actions required:

**1. Rename project-proofreader → project-console**

Update in `pairings.yaml` (workspace root) and `PROJECT-CLONES.md`:
- `cluster_name: project-proofreader` → `cluster_name: project-console`
- `module_id: proofreader` → `module_id: console`
- `branch: cluster/project-proofreader` → `branch: cluster/project-console`

Also rename the cluster directory: `clones/project-proofreader/` → `clones/project-console/`
and update the git branch name accordingly.

**2. Add content-wiki-documentation as third sub-clone**

This cluster now produces TOPIC-* articles (4 drafted this session) targeting
`content-wiki-documentation`. Add a third sub-clone entry in the cluster manifest:
```yaml
- repo: content-wiki-documentation
  role: wiki
  path: content-wiki-documentation/
  upstream: pointsav/content-wiki-documentation
  focus: topic-machine-based-authorization, topic-pointsav-private-network,
         topic-os-console-platform, topic-input-machine (and future os-console TOPICs)
```

**3. Architecture catalog additions required**

The following crates appear in the os-console F-key map but are NOT yet in
`conventions/architecture-layer-catalog.md`. Add them under `app-console-*`:

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| app-console-gis | No | Reserved-folder; F8 GIS cartridge |
| app-console-slm | No | Reserved-folder; F9 SLM management cartridge |
| app-console-system | No | Reserved-folder; F11 system status cartridge |

**4. Naming note: F10 = app-console-mesh (already in catalog)**

The catalog already has `app-console-mesh` as Reserved-folder. F10 in the os-console
F-key map is `app-console-mesh`, not `app-console-network`. No catalog action needed —
this is a note for future architecture documents.

**5. Existing guide naming conflict to note**

`woodfine-fleet-deployment/guide-mesh-execution.md` calls the `os-network-admin` web
interface "the F8 Terminal." In the os-console F-key map, F8=GIS and F10=mesh. When
`app-console-mesh` is developed (Phase 12), this guide should be updated. Not urgent.

**6. Stale Doorman port in manifest.md cross-cluster section**

`.agent/manifest.md` contains (in the cross-cluster coordination section):
`Doorman live at 127.0.0.1:9080`

Correct endpoint is `http://localhost:8011` per `slm/endpoint.txt` and `pairings.yaml`.
Please update manifest.md when renaming the cluster.

**Work completed this session (2026-05-20):**

Plans:
- `.agent/plans/os-console-platform.md` — consolidated architecture reference
- `.agent/plans/leapfrog-2030-coding.md` — phased coding roadmap (chassis-first, Phase 0 done)

Drafts in `.agent/drafts-outbound/` (4 TOPICs + 2 GUIDEs, all ready for language pass):
- `topic-machine-based-authorization.md` → content-wiki-documentation
- `topic-pointsav-private-network.md` → content-wiki-documentation
- `topic-os-console-platform.md` → content-wiki-documentation
- `topic-input-machine.md` → content-wiki-documentation
- `guide-mba-pairing-ceremony.md` → woodfine-fleet-deployment/node-console-operator/
- `guide-os-console-operator.md` → woodfine-fleet-deployment/node-console-operator/

Architecture Q&A resolved this session (preserved in plans for future reference):
- MBA is peer-to-peer between os-* services; PPN is infrastructure only
- "Pairing as Permission" and "no credentials database" legal basis confirmed
- app-console-keys = base chassis (F-keys, not crypto keys)
- F-key map canonical (WIP): F1-F12 assigned, F10=app-console-mesh
- PDF: pdfium-render + Kitty/Sixel only
- service-input is Ring 1; Input Machine (F12) POSTs to it

— totebox@project-proofreader / 2026-05-20

---
from: totebox@project-proofreader
to: command@claude-code
re: TUI pivot plan complete — 8 action items for Command Session
created: 2026-05-16T20:15:00Z
priority: high
status: pending
msg-id: project-proofreader-20260516-tui-pivot-handoff
---

Strategic pivot research complete. 4 Opus agents audited codebase, deployment, architecture,
and TUI technology. Plan document at `.agent/plans/tui-pivot-2030.md`. Summary:

**Strategic direction:** Replace web UI with TUI over SSH (`russh` embedded on port 2222).
Stack: ratatui + crossterm + russh + tui-textarea + similar. Web UI taken down. Backend
(service-proofreader 9092) unchanged. Doctrine claim #45 (TUI-as-Corpus-Producer) is the
anchor. ETA: 7–9 weeks Phases 0–6.

**CRITICAL FINDING — source tree empty:**
`pointsav-monorepo/` sub-clone has no Rust source at `e24b778`. Pre-pivot source SHA
`788b3722` preserved in local reflog. Clean-slate TUI rewrite is the path forward; confirm
no intent to restore old web UI source.

**Action items requiring Command Session:**

1. **[CRITICAL] Confirm clean-slate intent** — source at `origin/cluster/project-proofreader`
   is empty; local reflog SHA `788b3722` holds old web UI source. Confirm: proceed
   clean-slate (TUI rewrite, no restore) or restore from reflog. This is the pre-development
   blocker.

2. **[Before teardown] Backfill `local-proofreader-public.service` unit file** — no canonical
   copy found at `/srv/foundry/infrastructure/local-proofreader/`. Copy from live
   `/etc/systemd/system/local-proofreader-public.service` and commit before teardown.

3. **[Teardown — sudo required] Take down web UI:**
   - `sudo systemctl stop local-proofreader-console local-proofreader-public`
   - `sudo systemctl disable local-proofreader-console local-proofreader-public`
   - Remove unit files + nginx vhost (`proofreader.pointsav.com`) + rate-limit conf
   - `sudo certbot delete --cert-name proofreader.pointsav.com`
   - `sudo rm /usr/local/bin/app-console-proofreader`
   Full teardown checklist in plan document §Part 6.

4. **[GCE firewall] Open port 2222** for TUI SSH server. Coordinate with operator.

5. **[Conventions read] `conventions/tui-corpus-producer.md`** — cited by Doctrine claim #45;
   inaccessible from this cluster. Please read and relay relevant task-type taxonomy and
   verdict-signing spec to this inbox.

6. **[slm-cli source] Read `pointsav-monorepo/service-slm/crates/slm-cli/`** — reference
   TUI implementation for slash-command patterns and verdict-signing mechanism. Relay key
   patterns or confirm this Totebox can read it.

7. **[Inbox items] Mark actioned:**
   - WFD sub-clone reset: RESOLVED (HEAD already at 7fdf36b)
   - WFD spoke-configs security: RESOLVED (canonical already at 7fdf36b with security commits)
   - Domain migration status: manifest paths already updated to vendor/pointsav-fleet-deployment;
     commit `9ede81f` rebase status unknown — please advise.

8. **[session-start.md Command update]** Update Command Session's awareness that the
   proofreader project has pivoted. Route any future proofreader engineering back to this
   Totebox.

— totebox@project-proofreader / 2026-05-16
