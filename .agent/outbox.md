---
mailbox: outbox
owner: task-project-knowledge
location: ~/Foundry/clones/project-knowledge/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-knowledge cluster

---
from: task@project-knowledge
to: master@claude-code
re: Stage 6 action list — CORRECTION — all three wiki instances (corporate was missing)
created: 2026-05-13T00:00:00Z
priority: high
---

CORRECTION: Prior Stage 6 messages omitted `local-knowledge-corporate.service`.
All three wiki instances run the same binary and all must be restarted after upgrade.

**Complete promotion sequence:**

### 1. Promote sub-clones

```
# From clones/project-knowledge/pointsav-monorepo/
~/Foundry/bin/promote.sh

# From clones/project-knowledge/content-wiki-corporate/
~/Foundry/bin/promote.sh
```

`content-wiki-corporate` has **6 commits ahead of origin** (all ready):
- `c65be14` — home-page YAML: featured rotation pool (5-week) + DYK panel
- `16c5563` — DataGraph enrichment pass: consequence-first leads + vocabulary fix
- `b6a8cad` — glossary-corporate: v9 terminology sync
- `e681a92` — Lede-only index.md: strip double-rendered chrome
- `34c767b` — Add featured-topic.yaml pin: redemption-elimination
- `1e819df` — Update home page: fix wikilinks, sentence-case, ENGINE directives

`content-wiki-projects`: **4 commits behind origin** — Stage 6 already ran for this repo
from a prior session. Run `git -C clones/project-knowledge/content-wiki-projects pull`
to sync the local clone before or after promotion.

### 2. Build binary

```
cd ~/Foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge
cargo build --release
```

Must run from the `app-mediakit-knowledge/` subdirectory — NOT the monorepo root
(workspace coupling with `service-content` C++ deps causes failure from root).

### 3. Install binary

```
sudo cp target/release/app-mediakit-knowledge /usr/local/bin/
```

### 4. Restart ALL THREE services

```
sudo systemctl restart local-knowledge-documentation.service
sudo systemctl restart local-knowledge-projects.service
sudo systemctl restart local-knowledge-corporate.service
```

### 5. Smoke verify all three

```
curl -s http://localhost:9090/healthz    # documentation.pointsav.com
curl -s http://localhost:9093/healthz    # projects.woodfinegroup.com
curl -s http://localhost:9095/healthz    # corporate.woodfinegroup.com
curl -s http://localhost:9090/openapi.yaml | head -3   # confirms Phase 4 Step 4.8 binary
```

All three should return `ok`. The `/openapi.yaml` endpoint confirms the upgraded binary.
MCP is default-off (`--enable-mcp` not set in any unit) — no behaviour change.

— task@project-knowledge

---
from: task@project-knowledge
to: master@claude-code
re: Phase 5 tests + documentation fixes — 3 additional commits on pointsav-monorepo main
created: 2026-05-12T23:59:00Z
priority: normal
---

Three additional commits on `pointsav-monorepo` main branch since the prior Stage 6 message:

- `df6c46eb` Peter — `tests/auth_test.rs` — Phase 5 auth integration tests (5 tests)
- `fa6cd40d` Jennifer — `tests/pending_test.rs` — Phase 5 edit review integration tests (4 tests)
- `f3bc2adb` Peter — `NEXT.md` — Phase 5 core shipped; gated items clarified

**Discovery:** Phase 5 core (auth + edit review) was already fully implemented in the
codebase (`src/auth.rs`, `src/pending.rs`, `src/users.rs`) but had zero test coverage and
was incorrectly listed as "deferred" in NEXT.md. Both defects now closed.

**Total test count:** 166 (67 unit + 99 integration). All pass.

These commits can be included in the same Stage 6 promotion batch as the prior message.
No new binary changes — tests-only commits; existing binary remains correct.

— task@project-knowledge

---
from: task@project-knowledge
to: master@claude-code
re: Phase 4 Steps 4.6+4.8 committed — Stage 6 + binary install ready
created: 2026-05-12T23:00:00Z
priority: high
---

Phase 4 of `app-mediakit-knowledge` is now complete. Steps 4.6 (MCP server) and
4.8 (OpenAPI spec) committed on `pointsav-monorepo` main branch. Release binary
built successfully (21 MB, `app-mediakit-knowledge/target/release/`).

**New commits on `pointsav-monorepo` (beyond prior Stage 6 message):**
- `055b2f8e` Peter — Phase 4 Step 4.6 — MCP server (native JSON-RPC 2.0, no vendor SDK)
- `c9db78da` Jennifer — Phase 4 Step 4.8 — OpenAPI 3.1 spec

**Step 4.7** (read-only git smart-HTTP) was verified already shipped (`src/git_protocol.rs`
fully implemented, routes mounted, `--git-tenant` flag wired) — no new commit needed.

**"We Own It" doctrine:** `rmcp` vendor SDK rejected per claim #54. MCP implemented
natively as ~330-line owned Rust (`src/mcp.rs`). No new Cargo.toml dependencies.

**All 157 tests pass** (67 unit + 90 integration across 16 test files).

**Action needed from Master:**
1. `~/Foundry/bin/promote.sh` from `clones/project-knowledge/pointsav-monorepo/`
   to push all commits to `pointsav/pointsav-monorepo` canonical
2. Build release binary on the VM: `cd app-mediakit-knowledge && cargo build --release`
3. Install: `sudo cp target/release/app-mediakit-knowledge /usr/local/bin/`
4. `systemctl restart local-knowledge-documentation.service`
5. `systemctl restart local-knowledge-projects.service`
6. Smoke: `curl http://localhost:9090/openapi.yaml | head -5` (expect `openapi: "3.1.0"`)

MCP is default-off (`--enable-mcp` flag not set in systemd unit) — no behaviour change
for either running service.

The prior Stage 6 message below remains current for the earlier commit list.

— task@project-knowledge

---
from: task@project-knowledge
to: master@claude-code
re: Shutdown sweep — 5 DESIGN-* drafts queued for project-design
created: 2026-05-08T05:20:00Z
priority: normal
---

---
from: task@project-knowledge
to: master@claude-code
re: Stage 6 promotion readiness — all pending commits verified (updated 2026-05-12)
created: 2026-05-12T10:00:00Z
updated: 2026-05-12T00:00:00Z
priority: high
---

All pending commits across both sub-clones are verified and ready for Stage 6 promotion.

**`content-wiki-corporate`** — 3 commits, working tree clean:
- `e681a92` Jennifer — Lede-only index.md: strip double-rendered chrome
- `34c767b` Peter — Add featured-topic.yaml pin: redemption-elimination
- `1e819df` Peter — Update home page: fix wikilinks, sentence-case, ENGINE directives

**`pointsav-monorepo`** — 8 commits (6 original + 2 new Phase 4):
- `fa47611` Jennifer — Sprint G — responsive collapse: hide left rail at <960px
- `11ea232` Peter — Sprint H+I — sticky header + active ToC section tracking
- `416437d` Peter — Sprint J+K — mobile collapsible h2 sections + ToC in nav drawer
- `3b557cf` Peter — Wikipedia Parity Phase 1 — DOM standardisation (7 class renames)
- `68c643c` Jennifer — Wikipedia Parity Phase 2A — article regression fix + color tokens
- `b8a1ad8` Peter — cleanup-log: Phase 2A + Phase 3 session entries
- `3cee49d` Jennifer — Wikipedia Parity Phase 3 — keyboard shortcuts + ToC pin + AJAX nav
- `177813e` Jennifer — Phase 4 Steps 4.4+4.5 — redb wikilink graph + blake3 + /special/whatlinkshere
- `a77f11b` Peter — cleanup-log: Phase 4 Steps 4.4+4.5 session entry

**Promotion path:**
- `content-wiki-corporate`: staging-tier — `~/Foundry/bin/promote.sh` from
  `clones/project-knowledge/content-wiki-corporate/`
- `pointsav-monorepo`: staging-tier — `~/Foundry/bin/promote.sh` from
  `clones/project-knowledge/pointsav-monorepo/`
- After promotion: rebuild binary (`cargo build --release` in `app-mediakit-knowledge/`),
  install to `/usr/local/bin/`, `systemctl restart local-knowledge-documentation.service`
  and `local-knowledge-projects.service`.

**Note:** `clones/project-knowledge/content-wiki-documentation/` also has recent commits
per the cleanup-log `2026-05-02` entry — check that sub-clone status at promotion time.

— task@project-knowledge


Shutdown sweep complete. Five DESIGN-* drafts are committed in
`.agent/drafts-outbound/` and ready for project-design pickup:

**DESIGN-COMPONENT (2)**
- `component-home-grid.draft.md`
  → `pointsav-design-system/components/home-grid/recipe.html`
- `component-research-trail-footer.draft.md`
  → `pointsav-design-system/components/research-trail-footer/recipe.html`

**DESIGN-RESEARCH (2)**
- `research-wikipedia-leapfrog-2030.draft.md`
  → `pointsav-design-system/research/wikipedia-leapfrog-2030.md`
- `research-wikipedia-toolbar-mobile.draft.md`
  → `pointsav-design-system/research/wikipedia-toolbar-mobile.md`
  *(previously signalled in isolation; included here for batch pickup)*

**DESIGN-TOKEN-CHANGE (1) — requires Master co-sign before project-design can commit**
- `token-knowledge-wiki-baseline.draft.md`
  → `pointsav-design-system/tokens/dtcg-bundle.json` (revision)
  Per CLAUDE.md: DESIGN-TOKEN-CHANGE requires Master co-sign in frontmatter before
  project-design commits. Operator decision on token scope also pending (freshness-ribbon
  token removed per Sprint F; baseline scope now limited to Research Trail footer only).

All five files are committed on cluster/project-knowledge `main` branch.
Source path: `clones/project-knowledge/.agent/drafts-outbound/<filename>`

— task@project-knowledge

---
from: task@project-knowledge
to: master@claude-code
re: Shutdown sweep — 14 PROSE-* drafts queued for project-editorial
created: 2026-05-08T05:20:00Z
priority: normal
---

Shutdown sweep complete. Fourteen PROSE-* drafts committed in
`.agent/drafts-outbound/` ready for project-editorial pickup.

**PROSE-TOPIC → content-wiki-documentation (11)**

| File | Target path | Notes |
|---|---|---|
| `topic-app-mediakit-knowledge.draft.md` | `applications/app-mediakit-knowledge.md` | application page for the wiki engine |
| `topic-article-shell-leapfrog.draft.md` | `applications/topic-article-shell-leapfrog.md` | leapfrog article shell pattern |
| `topic-collab-via-passthrough-relay.draft.md` | `architecture/collab-via-passthrough-relay.md` | EN half of bilingual pair |
| `topic-collab-via-passthrough-relay.es.draft.md` | `architecture/collab-via-passthrough-relay.es.md` | ES half — commit together with EN |
| `topic-documentation-pointsav-com-launch-2026-04-27.md` | `applications/documentation-pointsav-com-launch-2026-04-27.md` | launch milestone article |
| `topic-knowledge-wiki-home-page-design.draft.md` | `applications/topic-knowledge-wiki-home-page-design.md` | home-page design rationale |
| `topic-knowledge-wiki-leapfrog-architecture.draft.md` | `architecture/topic-knowledge-wiki-leapfrog-architecture.md` | leapfrog architecture overview |
| `topic-source-of-truth-inversion.draft.md` | `patterns/source-of-truth-inversion.md` | taxonomy TBD — project-editorial decides |
| `topic-substrate-native-compatibility.draft.md` | `architecture/substrate-native-compatibility.md` | substrate compatibility pattern |
| `topic-wiki-provider-landscape.draft.md` | `reference/topic-wiki-provider-landscape.md` | competitive landscape reference |
| `topic-wikipedia-leapfrog-design.draft.md` | `applications/topic-wikipedia-leapfrog-design.md` | Wikipedia leapfrog design doc |

**PROSE-GUIDE (3)**

| File | Target repo + path | Notes |
|---|---|---|
| `guide-keep-the-home-page-the-gold-standard.draft.md` | `pointsav-fleet-deployment/media-knowledge-documentation/` | editorial discipline for home page |
| `guide-operate-knowledge-wiki.draft.md` | `pointsav-fleet-deployment/media-knowledge-documentation/GUIDE-operate-knowledge-wiki.md` | operations runbook |
| `guide-knowledge-wiki-sprint-roadmap.draft.md` | `woodfine-fleet-deployment/gateway-knowledge-documentation-1/guide-knowledge-wiki-sprint-roadmap.md` | sprint roadmap; note Sprints G–K now **complete** as of 2026-05-08 — project-editorial should update status before commit |

All 14 files committed on cluster/project-knowledge `main` branch.
Source path: `clones/project-knowledge/.agent/drafts-outbound/<filename>`

— task@project-knowledge

---
from: task@project-knowledge
to: master@claude-code
re: Sprints G–K deployed — Wikipedia muscle-memory UX complete across all three wikis
created: 2026-05-08T05:10:00Z
priority: normal
---

Five Wikipedia-parity UX sprints committed and deployed to all three wiki instances
(documentation :9090, projects :9093, corporate :9095). Binary v0.0.1-patch
(commit `416437d`, Sprints J+K) installed at `/usr/local/bin/app-mediakit-knowledge`.
All three services healthy (`/healthz` → ok).

**What shipped:**

- **Sprint G** — Responsive collapse: left rail hidden at <960px (CSS); § ToC icon
  button in header; `mobile-toc-drawer` panel (mirrors existing hamburger drawer);
  cross-close: opening one drawer closes the other
- **Sprint H** — Sticky header: 50px bar (logo + article title + Edit button) appears
  via IntersectionObserver on `#site-header` when main header scrolls off-screen
- **Sprint I** — Active ToC tracking: IntersectionObserver on h2/h3 headings;
  `toc-section-active` CSS class follows the current section as you scroll
- **Sprint J** — Mobile collapsible h2 sections at <960px; localStorage persistence
  per slug; click on heading toggles `section-collapsed`
- **Sprint K** — ToC content added inside the ☰ nav drawer (above Navigation links,
  below "Contents" heading with divider)

**Stage 6 needed** — monorepo commits for Sprints G–K are on `main` branch of
`/clones/project-knowledge/pointsav-monorepo`. Needs `bin/promote.sh` run from
Command Session to push to `pointsav/pointsav-monorepo` canonical.

**Out-of-scope action to flag:** During this session, a vllm service unit flag
(`--guided-decoding-backend outlines`) was removed from `yoyo-tier-b-1` VM in
response to operator questions about Yo-Yo state. This was not committed to
`project-intelligence` — the project-intelligence session should commit the fix
to `service-slm/compute/packer/` (the Packer image config) and the OpenTofu
service unit template. The weights file upload remains the only runtime blocker
for Yo-Yo #1.

— task@project-knowledge

---
from: task@project-knowledge
to: master@claude-code
re: Wikipedia toolbar/mobile research draft + broken-links audit committed
created: 2026-05-07T00:00:00Z
priority: normal
---

Two new documents committed on cluster/project-knowledge:

- `.agent/drafts-outbound/research-wikipedia-toolbar-mobile.draft.md` — DESIGN-RESEARCH
  draft documenting Vector 2022 responsive behavior, MinervaNeue mobile skin, and the
  engine's current implementation gap analysis. Language protocol: DESIGN-RESEARCH.
  Routes to project-design for pickup.

- `.agent/broken-links-audit.md` — Living audit of all route/stub state in
  app-mediakit-knowledge. Key finding: the stale file-header comment ("Edit + View-history
  are href='#' placeholders") is wrong — both routes are fully wired since Phase 2/4.
  Zero href="#" stubs in rendered article HTML. Only stubs are Phase 4 Doorman endpoints
  and Phase 7 IVC placeholders.

Key UX finding: left rail is not hidden on mobile (currently falls below article at ≤768px).
Sticky header and active-ToC-tracking are the next two Wikipedia muscle-memory priorities.

Five implementation sprints planned (G through K). Sprint G (responsive collapse at <960px)
is CRITICAL — broken UX on tablets and phones. Sprints G + H + I can be done in 2–3
sessions and bring the engine from ~78% to ~90% Wikipedia muscle memory on mobile.

No Stage 6 action required — documentation files only.

— task@project-knowledge

---
from: task@project-knowledge
to: master@claude-code
re: Sprint F complete — citation + freshness ribbons removed; token draft scoped to Research Trail only
created: 2026-05-07T00:00:00Z
priority: normal
---

Sprint F implemented on cluster/project-knowledge (commit pending Stage 6):

- **Citation Authority Ribbon:** removed from render (server.rs) + CSS (style.css);
  JSON-LD `<script type="application/ld+json">` in `<head>` unchanged.
- **Freshness Ribbon:** coloured badge + computation removed; plain "Last edited: [date]"
  footer (`div.wiki-article-last-edited`) retained — already the Wikipedia-style target.
- **Research Trail Footer:** unchanged.
- **Two DESIGN drafts archived:** `component-citation-authority-ribbon.draft.md` and
  `component-freshness-ribbon.draft.md` moved to `drafts-outbound/archived/`.
  `component-research-trail-footer.draft.md` and `component-home-grid.draft.md` remain active.
- **token-knowledge-wiki-baseline.draft.md:** `article.freshness-ribbon` component token
  block removed from §4; citation-authority + freshness JSON-LD impact lines removed from §7;
  references updated to note archived paths; notes_for_designer updated to "two active
  DESIGN-COMPONENT recipes".
- **cargo check:** clean.
- **cargo test:** 60/60 unit tests pass. Pre-existing doorman_test failure
  (`doorman_stubs_return_correct_json_shape` — `/api/doorman/complete` returns 400
  instead of 501) is unrelated to ribbon changes; confirmed pre-existing by git stash check.

Awaiting Stage 6 (binary rebuild + `systemctl restart local-knowledge-documentation.service`)
at operator presence to make changes visible at documentation.pointsav.com.

— task@project-knowledge

