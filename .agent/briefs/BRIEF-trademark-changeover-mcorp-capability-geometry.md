---
artifact: brief
schema: foundry-brief-v1
brief-id: project-editorial-trademark-changeover-mcorp-capability-geometry
title: "Trademark Changeover — MCorp + Capability Geometry™"
status: active
owner: project-editorial
created: 2026-06-19
updated: 2026-06-19
---

> ## 🔵 STATUS: PLAN READY — AWAITING STARTUP
> Scope confirmed. Two changes, brand shortform only (not legal entity rename).
> Execute in Phase order. Phase 1 requires Command Session (admin-tier).

---

## Context

Two trademark changes approved by operator (2026-06-19):

1. **"Woodfine Management Corp" → "MCorp"** — brand shortform change only.
   - Legal entity stays "Woodfine Management Corp." in BC incorporation records.
   - License texts (PointSav-ARR.txt, PointSav-Commercial.txt) stay unchanged.
   - All public-facing trademark notices, footers, and brand references switch to `MCorp™`.
   - "MCorp" already used as shorthand in fleet-deployment guide (line 147) and BIM tiles registry — surfacing as primary display name.

2. **Add "Capability Geometry™"** — net-new trademark; zero existing occurrences.
   - Scope: everywhere all other marks are listed (same surface as Totebox Archive™, Totebox Orchestration™, etc.).
   - Replaces the earlier "Geometric Protection™" proposal (Command inbox correction already noted).

**Command inbox pre-awareness:** `/srv/foundry/.agent/inbox.md` has two messages:
- Older (stale): "Geometric Protection™" — superseded
- Current (authoritative): "Capability Geometry™" — three changes scoped; this BRIEF is the execution plan

**Survey scope:** ~1,600+ files across the workspace. Canonical source files drive propagation — change those first.

---

## Canonical trademark footer string

**Current (find this string):**
```
Woodfine Capital Projects™, Woodfine Management Corp™, PointSav Digital Systems™, Totebox Orchestration™, and Totebox Archive™ are trademarks of Woodfine Capital Projects Inc.
```

**Replacement:**
```
Woodfine Capital Projects™, MCorp™, PointSav Digital Systems™, Totebox Orchestration™, Totebox Archive™, and Capability Geometry™ are trademarks of Woodfine Capital Projects Inc.
```

The Spanish equivalent changes analogously in all `*-es.md` footer templates.

---

## Phase 1 — Canonical sources (Command Session, admin-tier)

**Prerequisite for all later phases. Must complete first.**

### 1a — factory-release-engineering (~8 files; ps-administrator)

| File | Change |
|---|---|
| `TRADEMARK.md` | Mark list §1: `Woodfine Management Corp™` → `MCorp™`; add `Capability Geometry™` entry |
| `policies/TRADEMARK.md` | Same |
| `policies/DISCLAIMER.md` | Trademark attribution line |
| `tokens/legal-tokens-woodfine.yaml` | `trademarks.owned` array + `trademarks.statement` prose |
| `tokens/legal-tokens-pointsav.yaml` | `trademarks.owned` array (add Capability Geometry) |
| `readmes/footer-readme-en.md` | Canonical footer string (see above) |
| `readmes/footer-readme-es.md` | ES equivalent |
| `readmes/footer-topic-en.md` | Canonical footer string |
| `readmes/footer-topic-es.md` | ES equivalent |
| `readmes/footer-guide-en.md` | Canonical footer string |
| `README.md` | Corporate structure prose: `Woodfine Management Corp` → `MCorp (Woodfine Management Corp.)` |
| `PLAYBOOK.md` | Same |

Commit: `~/Foundry/bin/commit-as-next.sh --admin pointsav "brand(trademark): MCorp shortform + Capability Geometry™ mark — update TRADEMARK, tokens, footer templates"`

### 1b — pointsav-design-system linguistic tokens (~7 files; staging-tier → Stage 6)

| File | Change |
|---|---|
| `tokens/linguistic/ps-protocol-trademark.yaml` | Wordmark list + boilerplate paragraph |
| `tokens/linguistic/ps-protocol-trademark-web.yaml` | Web footer boilerplate |
| `tokens/linguistic/legal-disclaimers.yaml` | HTML disclaimer block |
| `tokens/linguistic/ps-protocal-nomenclature.yaml` | Nomenclature matrix entry |
| `tokens/linguistic/corporate-authority.yaml` | Postal address block (if "Woodfine Management Corp." appears) — note: this is the legal entity; check whether to leave as-is (it IS the legal entity name in an address context) |
| `tokens/linguistic/ds-protocol-memo.yaml` | "Prepared by" field — brand shortform OK here |
| `tokens/linguistic/ps-protocol-contact.yaml` | Contact entry |

Commit from correct Totebox archive (project-design or project-source, whichever owns design-system).
Stage 6 promotion via Command after commit.

**Note on `corporate-authority.yaml`:** Postal/legal entity uses must retain "Woodfine Management Corp." — only brand/trademark notice positions change to MCorp. Confirm each field before replacing.

---

## Phase 2 — Rust source / live website (Totebox archives, then rebuild)

**Run after Phase 1 so Rust source matches canonical tokens.**

### 2a — app-mediakit-knowledge (~2 files; project-knowledge Totebox)

| File | Line | Change |
|---|---|---|
| `src/chrome/mod.rs` | 244 | Trademark string in HTML footer |
| `src/server/home_handlers.rs` | 219 | `copyright_entity = "Woodfine Management Corp."` → `"MCorp"` |

After edit: `cargo build --release` + binary redeploy for all 3 wiki instances.
Stage 6 for monorepo from project-knowledge, or route via outbox to Command for promote.

### 2b — app-orchestration-gis HTML files (~6 files; project-gis Totebox)

Files: `www/research.html`, `www/index.html`, `www/research-*.html`
Lines contain: page title author affiliation "Woodfine Management Corp.", methodology attribution paragraph, page footer.
Change: brand reference → MCorp; footer trademark string → full replacement.

### 2c — Other Rust source (~8 files; various archives)

| File | Archive |
|---|---|
| `service-email-template/src/main.rs` + templates | project-knowledge or project-intelligence |
| `app-orchestration-slm/crates/orchestration-slm/src/license.rs` | project-source |
| `service-content/src/entity_filter.rs` + `main.rs` + `content-compiler/src/main.rs` | project-intelligence |
| `service-slm/crates/slm-doorman-server/src/http.rs` | project-intelligence |
| `tool-proforma-engine/src/report/` (2 files) | project-proforma |

For each: edit trademark string; rebuild binary; deploy; update binary ledger.

---

## Phase 3 — Content wiki markdown (high volume; scripted approach)

**Run after Phase 1 (templates canonical) and Phase 2 (live sites updated).**

### Strategy

For the trademark footer boilerplate (appears at end of most articles), use a scripted sed replacement rather than per-file editing. The string is stable and mechanically identifiable.

Script approach (per sub-clone, from its own Totebox session):
```bash
# Dry run first — count matches
grep -rl "Woodfine Management Corp™" . --include="*.md" | wc -l

# Replace trademark notice string
find . -name "*.md" -exec sed -i \
  's/Woodfine Capital Projects™, Woodfine Management Corp™, PointSav Digital Systems™, Totebox Orchestration™, and Totebox Archive™ are trademarks of Woodfine Capital Projects Inc\./Woodfine Capital Projects™, MCorp™, PointSav Digital Systems™, Totebox Orchestration™, Totebox Archive™, and Capability Geometry™ are trademarks of Woodfine Capital Projects Inc./g' {} \;

# Verify no stragglers
grep -rl "Woodfine Management Corp™" . --include="*.md"

# Commit (staged by archive)
git add <files> && ~/Foundry/bin/commit-as-next.sh "brand(trademark): MCorp™ + Capability Geometry™ — update footer notices"
```

**Important**: Body text uses of "Woodfine Management Corp" (not "Woodfine Management Corp™") are in article prose — change these to "MCorp" only where they're public-facing brand references, NOT in legal/corporate structure context.

### 3a — content-wiki-documentation (~35 files; project-editorial Totebox)

Owner: project-editorial (has sub-clone in CWD).
Script + commit.

### 3b — content-wiki-projects (~15 files; project-editorial Totebox)

Owner: project-editorial (has sub-clone in CWD).
Script + commit.

### 3c — content-wiki-corporate (~25 files; project-editorial Totebox)

Owner: project-editorial (has sub-clone in CWD).
Special care: `about.md`, `contact.md`, disclaimers contain body text references to "Woodfine Management Corp." in legal context — review these individually; do NOT bulk-replace all occurrences.

### 3d — woodfine-fleet-deployment GUIDEs (~80 files; Command admin-tier)

Admin-tier commit (mcorp-administrator). Scripted sed + `commit-as-next.sh --admin woodfine`.
One GUIDE already has correct shorthand (guide-orgchart-authoring.md line 147: `MCorp = Woodfine Management Corp.`) — verify this stays or updates gracefully.

### 3e — Remaining vendor/customer READMEs (~50 files; various)

Repos: pointsav-design-system, pointsav-fleet-deployment, pointsav-media-assets, pointsav.github.io, woodfine-media-assets, woodfine.github.io, SECURITY.md files across repos.
Commit per repo (staging-tier for vendor, admin-tier for customer).

---

## Phase 4 — Workspace governance docs (Command Session)

| File | Change |
|---|---|
| `/srv/foundry/CLAUDE.md` | Org chart + contributor description (2 occurrences) |
| `/srv/foundry/AGENT.md` | Contributor description |
| `/srv/foundry/CHANGELOG.md` | Historical entries may stay; new entries use MCorp |
| `/srv/foundry/NEXT.md` | Any pending references |
| `/srv/foundry/conventions/journal-artifact-discipline.md` | Journal affiliation string |
| `/srv/foundry/conventions/cluster-wiki-draft-pipeline.md` | Pipeline prose |
| `/srv/foundry/conventions/content-wiki-scope-discipline.md` | Scope prose |
| `/srv/foundry/citations.yaml` | Entity record if present |

Workspace commit: `~/Foundry/bin/commit-as-next.sh "brand(trademark): MCorp shortform in workspace docs"`

---

## Phase 5 — Stage 6 + sync (Command Session)

After each phase's commits are promoted (bin/promote.sh), run `bin/sync-local.sh --all` to pull canonical into all registered live-service paths and reload consuming systemd services.

Order: Phase 1 promote → Phase 2 rebuild → Phase 3 promote → Phase 4 promote → final sync.

---

## Decisions locked

- Legal entity "Woodfine Management Corp." unchanged in license texts (PointSav-ARR.txt, PointSav-Commercial.txt)
- `corporate-authority.yaml` postal/legal address fields: verify before replacing (may stay as legal entity name)
- ES footer strings: change the Spanish version analogously wherever EN footer changes
- "Capability Geometry™" scope: everywhere Totebox Archive™ appears (all trademark notices, both woodfine and pointsav sides)
- Body text: brand reference positions → MCorp; legal/corporate structure positions → check individually

## Decisions open

- `corporate-authority.yaml` — does the postal/legal address field use the brand or legal entity name? Check before Phase 1b.
- Historical CHANGELOG entries — update to MCorp for clarity, or leave as written-at-the-time record?
- Does "Capability Geometry" need a description / product association in TRADEMARK.md or is the mark name alone sufficient?

---

## Carry-forward from prior session (fold into startup plan)

These are outstanding items from the 2026-06-19 AUTO run that should be worked in the next session alongside or before the trademark changeover:

- [ ] **Stage 6** — Command: media-knowledge-projects tip 7fa466b (6 commits: M7+M9 parity repairs)
- [ ] **Track 2d / project-console routing** — 13 project-console artifacts pending Command ACK (msg-id: command-20260619-drafts-outbound-pickup-editorial-researc); PROSE-RESEARCH-ppn-architecture-phd-thesis destination pending
- [ ] **media-knowledge-documentation M9** — ES parity sweep not yet run for documentation sub-clone
- [ ] **NEXT.md contamination cleanup** — project-knowledge + project-intelligence content in project-editorial NEXT.md
- [ ] **F2/F3 dead links** — check --strict gate blocked; dead wikilinks need resolution before L4 gate

---

## Session routing summary

| Phase | Session | Identity |
|---|---|---|
| 1a factory-release-engineering | Command | ps-administrator |
| 1b design-system tokens | Totebox (project-design or project-source) | jwoodfine/pwoodfine → Stage 6 |
| 2a app-mediakit-knowledge | Totebox (project-knowledge) | jwoodfine/pwoodfine → Stage 6 + rebuild |
| 2b GIS HTML | Totebox (project-gis) | jwoodfine/pwoodfine → Stage 6 |
| 2c other Rust | Various Totebox archives | per-archive |
| 3a–c content wikis documentation/projects/corporate | Totebox (project-editorial) | jwoodfine/pwoodfine |
| 3d fleet-deployment GUIDEs | Command | mcorp-administrator |
| 3e vendor/customer READMEs | Mixed (Totebox for vendor; Command for customer) | per-repo |
| 4 workspace governance docs | Command | jwoodfine/pwoodfine |
| 5 Stage 6 + sync | Command | bin/promote.sh + bin/sync-local.sh |

## Work log

- 2026-06-19: Plan written from three parallel survey agents (factory-release-engineering, website footers, workspace-wide grep). Operator confirmed brand shortform + full surface for Capability Geometry.
