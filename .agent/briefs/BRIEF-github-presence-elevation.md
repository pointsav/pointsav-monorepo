---
artifact: brief
status: archived
archived: 2026-05-23
superseded_by: BRIEF-active-work.md
---

# GitHub Presence Elevation — PointSav & Woodfine

**Created:** 2026-05-16
**Owner:** project-editorial (research); Command Session (admin-tier writes)
**Status:** Research complete — implementation queued

---

## Governing principle

> **Content-forward deletion.** Nothing is deleted until its content has been
> worked into the surviving documents. Every removal must iterate the corpus
> forward — the information must live somewhere better, not disappear.
>
> Concretely: before `git rm` or `rm` any file, identify every piece of
> information it contains and confirm each piece is represented (or
> deliberately retired) in the document(s) it routes to. This applies to
> READMEs, guides, token files, draft files, and versioned duplicates alike.

---

## Research basis

Three OPUS agents ran 2026-05-16 against the full `~/Foundry/` workspace:

1. **File tree audit** — misplaced, versioned, and outdated files
2. **GitHub README coherence audit** — all public-facing READMEs, cross-link
   analysis, institutional reader experience
3. **Institutional style research** — Goldman Sachs / Google / Bloomberg /
   IBM Carbon patterns; per-repo writing recommendations; org profile drafts

Supplemented by a direct Bash scan for missing required files across
`vendor/` and `customer/`.

---

## Discovery: repos in the package

| Repo | Org | License | Status |
|---|---|---|---|
| `pointsav-monorepo` | pointsav | Mixed per-component | Active |
| `pointsav-design-system` | pointsav | Apache 2.0 | Active |
| `pointsav-media-assets` | pointsav | PointSav-ARR (proprietary) | Active |
| `factory-release-engineering` | pointsav | Governance | Active |
| `pointsav-fleet-deployment` | pointsav | — | Active |
| `content-wiki-documentation` | pointsav | CC BY 4.0 | Has website |
| `content-wiki-corporate` | woodfine | CC BY 4.0 | Has website |
| `content-wiki-projects` | woodfine | CC BY 4.0 | Has website |
| `woodfine-fleet-deployment` | woodfine | PointSav-ARR | Active |
| `woodfine-media-assets` | woodfine | PointSav-ARR (proprietary) | Active |
| `woodfine-design-bim` | woodfine | — | **Confirmed exists** — BIM tokens repo |
| `pointsav.github.io` | pointsav | — | GitHub Pages site |
| `woodfine.github.io` | woodfine | — | GitHub Pages site |

**Four repos to lead the GitHub presence** (institutional readers, bankers, developers):
1. `pointsav-monorepo`
2. `pointsav-design-system` (open — Apache 2.0, IBM Carbon equivalent)
3. `woodfine-fleet-deployment`
4. `woodfine-design-bim` (BIM tokens — confirm license; likely should be open)

Wikis deprioritised in READMEs — they have live websites.

---

## File tree audit findings

### Versioned duplicate (delete after content check)

| File | Rule violated | Content-forward action before delete |
|---|---|---|
| `vendor/pointsav-monorepo/USER_GUIDE_2026-03-30_V2.md` | Edit-in-place rule — no `_V2` files | Read it; confirm any unique content is in the current guide or wiki; then `git rm` |

### Missing README.es.md (bilingual mandate)

| Repo | Priority |
|---|---|
| `vendor/factory-release-engineering` | Medium (not heavily developer-facing) |
| `vendor/pointsav-fleet-deployment` | Medium |
| `vendor/pointsav-media-assets` | High (institutional readers land here) |
| `vendor/pointsav.github.io` | Medium |
| `customer/woodfine-media-assets` | High |
| `customer/woodfine.github.io` | Medium |

### Missing CLAUDE.md / AGENT.md

| Repo | Note |
|---|---|
| `vendor/factory-release-engineering` | Admin-tier — Command Session adds |
| `vendor/pointsav-fleet-deployment` | Staging-tier — can add |
| `vendor/pointsav-media-assets` | Admin-tier — Command Session adds |
| `vendor/pointsav-monorepo` | Needs session guide — Command Session |
| `vendor/pointsav.github.io` | Admin-tier |

### ARCHITECTURE.md — missing from every repo

Systemic gap. The workspace rule (CLAUDE.md §5) names per-project ARCHITECTURE.md
as authoritative. None exist in vendor/ or customer/. Add to each repo as a
separate work item — do not pad READMEs with architecture content.

Priority order for first ARCHITECTURE.md files:
1. `pointsav-monorepo` — most complex; most institutional value
2. `pointsav-design-system` — explains token architecture
3. `woodfine-fleet-deployment` — explains fleet topology

---

## GitHub README coherence audit — findings

### Individual README scorecards

| Repo | Audience fit | Register | Cross-links | Overall |
|---|---|---|---|---|
| `pointsav-monorepo` | Both | Institutional | Complete (core quad) | Ready |
| `pointsav-design-system` | Unclear | Institutional | Complete (core quad) | Needs work |
| `pointsav-media-assets` | Unclear | Mixed/internal | None | **Needs rewrite** |
| `factory-release-engineering` | Developer/legal | Institutional | None | Ready for audience |
| `content-wiki-documentation` | Both | Institutional | Partial | Ready |
| `content-wiki-corporate` | Banker | Institutional | Partial | Ready |
| `content-wiki-projects` | Banker | Institutional | Partial | Ready |
| `woodfine-fleet-deployment` | Both | **Best in package** | Partial | Ready |
| `woodfine-media-assets` | Unclear | Mixed | Partial | Needs work |
| `woodfine-design-bim` | — | — | — | Unaudited — read first |

**Org profile READMEs: missing for both orgs.** Highest-impact gap.

### Coherence gaps

1. **No org profile pages** — a Goldman Sachs analyst at `github.com/pointsav`
   sees a bare repo list. 60-second comprehension depends on which repo they
   click first. That is not coherent.

2. **`pointsav-design-system` README has massive content drift** — still
   describes a 6-protocol / 5-token surface. The 2026-05-16 batch moved 48 files
   from `content-wiki-documentation/design-system/` into this repo. README must
   be updated to reflect what the repo now actually contains.

3. **`pointsav-media-assets` reads as an internal operational note** — vocabulary
   like "Sovereign Structural Anchor", "Tier-3-Platform", "Operational Mandate."
   Needs a complete rewrite.

4. **BIM/real-estate angle invisible from the PointSav side.** The "$30 trillion
   tokenised real estate" positioning lives only in `woodfine-fleet-deployment`.
   The PointSav org README needs to surface this.

5. **Cross-links: one tight quad + five loose satellites.**
   `monorepo` ↔ `design-system` ↔ `content-wiki-documentation` ↔
   `woodfine-fleet-deployment` cross-link well. Everything else is a dead-end or
   links only one way.

6. **Asymmetric wiki cross-links:** `content-wiki-projects` → `content-wiki-corporate`
   ✓, but `content-wiki-corporate` → `content-wiki-projects` ✗.

---

## Institutional style — key principles

From Goldman Sachs, Google, Bloomberg, IBM Carbon analysis:

**The institutional README rubric** — first 200 words must answer:
1. What it is (noun-phrase definition — no adjectives)
2. Who owns it (firm name in sentence one)
3. What it relates to (platform context, sibling repos)
4. Who it's for (audience signal)
5. What the license is (open/proprietary, link)
6. Where the canonical docs live (link out, do not duplicate)

**Do not include:** emoji in headings, "made with [stack]" badge rows, roadmap
promises with dates, hero gradient banners, "Built with love", "inspired by Carbon",
superlatives ("powerful", "modern", "seamless", "innovative", "robust"), "ecosystem"
(use "platform" or "suite"), forward-looking claims without
"planned / intended / may / target" qualifiers.

**IBM Carbon pattern for design system:** The packages table is the most-used part
of Carbon's README. Alphabetical table of every published package, version badge,
npm link, one-line purpose. Converts a positioning document into a developer entry
point. `pointsav-design-system` needs this.

**The open / proprietary boundary sentence** — must appear in both
`pointsav-design-system` and `pointsav-media-assets`, bidirectionally:
> "PointSav-branded assets (logo, brand color values, identity assets) are held
> separately in `pointsav-media-assets` and are not Apache-licensed."

---

## Draft content from research agents

### `github.com/pointsav` org profile README (draft)

> **PointSav Digital Systems**
>
> PointSav Digital Systems builds the platform infrastructure for co-location
> intelligence, building information modeling (BIM) data services, and sovereign
> data operations used in the proptech and fintech sectors. The platform supports
> customers operating real-estate portfolios and financial services that depend on
> accurate, current, and jurisdictionally-controlled data about physical locations
> and the buildings that occupy them. PointSav is headquartered in Canada and
> operates under a continuous-disclosure posture appropriate to Canadian securities
> oversight.
>
> **Platform repositories**
>
> - **pointsav-monorepo** — the canonical source of record for PointSav platform
>   services, applications, and infrastructure.
> - **pointsav-design-system** — the open source design language and component
>   library. Apache License, Version 2.0.
> - **pointsav-media-assets** — proprietary brand assets. Held separately from
>   the open source design system to keep the licensing boundary explicit.
> - **factory-release-engineering** — governance, licensing, and release process
>   documentation across the platform.
>
> **Documentation:** documentation.pointsav.com — canonical reference for
> architecture, service interfaces, and integration guidance.
>
> **Licensing:** The design system is open source (Apache 2.0). Platform services
> are source-available under the licenses declared in each repository; commercial
> operation requires a commercial agreement. Brand assets are proprietary.
>
> **For developers:** Start with `pointsav-design-system`. The packages table lists
> every published component.
>
> **For institutional readers:** Governance and licensing are in
> `factory-release-engineering`. The reference customer deployment is
> `woodfine/woodfine-fleet-deployment`.
>
> Contact: open.source@pointsav.com

*(~480 words full version — see research agent output for complete draft.)*

### `github.com/woodfine` org profile README (draft)

> **Woodfine Management Corp**
>
> Woodfine Management Corp is a real estate and investment management company
> headquartered in Canada. Woodfine is the reference customer for the PointSav
> Digital Systems platform. The repositories in this organization document
> Woodfine's deployment of that platform and the operational practices that
> surround it.
>
> - **woodfine-fleet-deployment** — deployment configuration, operational
>   runbooks, and fleet documentation for Woodfine's PointSav platform instances.
> - **woodfine-design-bim** — BIM design tokens and building information modeling
>   design resources used in Woodfine's portfolio operations.
> - **woodfine-media-assets** — proprietary brand assets for Woodfine-branded
>   deployments. Permission required for use.
>
> The Woodfine fleet is the reference customer deployment for the PointSav
> platform. Asset managers and enterprise deployment teams reviewing the platform
> may find the structure of this repository useful as an example of how a PointSav
> customer organizes deployments at portfolio scale.

*(~290 words full version — see research agent output for complete draft.)*

### Per-repo opening paragraph templates

**`pointsav-monorepo`:**
> The PointSav Monorepo contains the platform services, applications, and
> infrastructure code developed by PointSav Digital Systems. The platform provides
> co-location intelligence, building information modeling (BIM) data services, and
> sovereign data infrastructure for proptech and fintech operators. This repository
> is the canonical source of record for PointSav platform engineering; published
> binaries and deployment artifacts are produced from tagged commits in this tree.

**`pointsav-design-system`:**
> The PointSav Design System is the open source design language and component
> library that PointSav Digital Systems uses to build its platform applications.
> It comprises design tokens, React and web component implementations, and the
> supporting documentation needed to construct interfaces consistent with the
> PointSav visual standard. The design system is released under the Apache License,
> Version 2.0, and is intended for use by developers building applications on or
> alongside the PointSav platform, as well as by independent projects that wish to
> adopt its conventions.

**`pointsav-media-assets`:**
> This repository holds the brand assets of PointSav Digital Systems: the PointSav
> logo, brand color specifications, typography selections, and supporting visual
> identity material. These assets are proprietary to PointSav Digital Systems and
> are not covered by the open source license that governs `pointsav-design-system`.

**`woodfine-fleet-deployment`:**
> This repository contains the deployment configuration, operational runbooks, and
> fleet documentation for Woodfine Management Corp's instances of the PointSav
> platform. Woodfine Management Corp operates a portfolio of PointSav deployments
> supporting its real estate and investment activities; this repository is the
> canonical record of that fleet's configuration and operational procedures.

---

## Priority action list

Ordered by institutional reader impact. Each item specifies owner and the
content-forward rule where deletion is involved.

| # | Action | Owner | Content-forward rule |
|---|---|---|---|
| 1 | Write `pointsav/.github/profile/README.md` from draft above | Command Session (admin-tier) | New file — no deletion |
| 2 | Write `woodfine/.github/profile/README.md` from draft above | Command Session (admin-tier) | New file — no deletion |
| 3 | Rewrite `pointsav-design-system` README: packages table + developer onboarding + updated content inventory + open/proprietary boundary sentence | project-design session | No deletion — expansion |
| 4 | Rewrite `pointsav-media-assets` README: institutional register + asset inventory + crosslink to design-system | Command Session (admin-tier) | Read current README first; confirm "Operational Mandate" content has a home in CLAUDE.md or internal docs |
| 5 | Audit and read `woodfine-design-bim` README — decide license + confirm it should be in the four featured repos | Any session (read-only first) | No deletion |
| 6 | Delete `USER_GUIDE_2026-03-30_V2.md` from `pointsav-monorepo` | Command Session | **Read it first.** Confirm any unique content is in `woodfine-fleet-deployment` guides or wiki. Then `git rm`. |
| 7 | Add ARCHITECTURE.md to `pointsav-monorepo`, `pointsav-design-system`, `woodfine-fleet-deployment` | Respective sessions | New files — no deletion |
| 8 | Fix asymmetric cross-link: add `content-wiki-projects` link to `content-wiki-corporate` README | project-editorial session | No deletion |
| 9 | Add missing `README.es.md` to `pointsav-media-assets`, `woodfine-media-assets` | Command Session (admin-tier for media repos) | New files |
| 10 | Standardize badge row (build status, license, latest release) across all four lead repos | Per-repo sessions | Remove decorative badges — their text content is nil; no content-forward needed |
| 11 | Audit BCSC forward-looking language in every README — apply "planned / intended / may / target" where needed | Per-repo sessions | In-place edits — no deletion |
| 12 | Add `factory-release-engineering` link from Governance section of every repo | Per-repo sessions | No deletion |

---

## woodfine-design-bim — naming analysis (OPUS agent, 2026-05-16)

### What the repo actually is

A DTCG design-token bundle for Building Information Modeling, consumed at runtime
by `app-orchestration-bim` (serving `bim.woodfinegroup.com`) via the
`BIM_DESIGN_SYSTEM_DIR` environment variable. Contents:

- 9 DTCG token files (`tokens/bim/*.dtcg.json`) anchored to IFC 4.3 primitive
  categories (spatial, elements, systems, materials, assemblies, performance,
  identity-codes, relationships) + climate-zone file
- `regulation/` — IDS 1.0 + IFC fragments (BC RS-1 v0.0.3)
- `climate/` — climate-zone specifications
- `components/` — BIM component recipes
- `research/` — AI-readable research files
- License: **EUPL-1.2** (EU copyleft — stronger than Apache 2.0, weaker than AGPL)

Standards floor: IFC 4.3 (ISO 16739-1:2024), Uniclass 2015, IDS 1.0, bSDD.

Structurally it is a **design-system substrate for the BIM/AEC semantic layer**
(same architecture as `pointsav-design-system` but different audience: architects,
structural/MEP engineers, construction managers rather than UI/UX developers).

### Why the current name does not work

1. **Only three-part name in either org.** Every other repo is `<org>-<compound-noun>`
   (`woodfine-fleet-deployment`, `woodfine-media-assets`). `woodfine-design-bim` is
   `<org>-<noun>-<qualifier>` — a unique and inconsistent shape.

2. **Reads with "BIM" as the head noun.** By English grammar, `woodfine-design-bim`
   parses as "Woodfine's BIM, in the design area." The repo is *design tokens for
   BIM*, not a BIM artifact. The head noun should be the token/design layer, not BIM.

3. **`design-bim` is not a recognized term** in AEC or UI/UX vocabulary. A Goldman
   Sachs analyst and an AEC developer would both need to decode it.

4. **`IT_SUPPORT_Nomenclature_Matrix_V8.md` no longer exists** — CLAUDE.md §5 has
   a dangling reference to it. Canonical replacement is
   `conventions/nomenclature-taxonomy.md`. Flag as a CLAUDE.md cleanup item.

### Deeper structural issue discovered (separate from naming)

**Two sources contradict each other on where BIM tokens belong:**

- **project-bim manifest** says: platform-level Building Design System tokens
  belong in `pointsav-design-system` (vendor tier, sub-clone in project-bim cluster)
- **`.agent/plans/README.md` routing table** says: `BIM-* → project-bim →
  woodfine-design-bim. Never routes to pointsav-design-system`

These cannot both be canonical. Additionally, the project-bim cluster's three
sub-clones are `pointsav-monorepo`, `pointsav-design-system`, and
`woodfine-fleet-deployment` — `woodfine-design-bim` is **not a sub-clone** of
project-bim, meaning BIM-* artifacts are being routed to a repo the project-bim
Task cannot write to. This is a routing defect independent of the name.

**Operator decision needed:** Is `woodfine-design-bim` a *tenant-branded* BIM
token bundle (customer tier, Woodfine-specific → stays in woodfine org) or is it
the *platform-level* BIM design substrate (vendor tier → should live in
`pointsav-design-system` or a new `pointsav-bim-*` repo)?

### SUPERSEDED — see updated analysis below

Prior recommendation was `woodfine-bim` (keep in woodfine org). Operator clarified
2026-05-17 that BIM tokens are open and generic for the AEC community — like Carbon
for UI/UX. That intent resolves the operator-decision flagged above: the repo belongs
in the `pointsav` vendor org. See updated recommendation.

### Content-forward transition: files to update on rename

| File | Change |
|---|---|
| `customer/woodfine-design-bim/README.md` + `.es.md` | Update title heading |
| `.agent/plans/README.md` line 28 | `woodfine-design-bim` → `woodfine-bim` (+ revisit "Never routes to pointsav-design-system" caveat) |
| `PROJECT-CLONES.md` project-bim block | Add explicit "tokens repo: woodfine-bim" line |
| `pairings.yaml` | Update repo name entry |
| `deployments/gateway-orchestration-bim-1/MANIFEST.md` | Update any path references (env var `BIM_DESIGN_SYSTEM_DIR` itself does not need changing) |
| GitHub remote | Rename in `github.com/woodfine` settings; GitHub redirects ~6 months; update `git remote set-url origin` in any clones |
| Any open inbox/outbox/plans referencing old name | Search and update |
| `CLAUDE.md` §5 | Fix dangling `IT_SUPPORT_Nomenclature_Matrix_V8.md` reference → `conventions/nomenclature-taxonomy.md` |

---

### Updated naming recommendation (OPUS agent, 2026-05-17)

**Operator intent clarified:** BIM tokens are open and generic for the AEC community
— "for everyone to use like design tokens." This resolves the org-placement question:
open platform substrate belongs in the `pointsav` vendor org, not the `woodfine`
customer org. Woodfine's role as first-mover is acknowledged in README and NOTICE,
not encoded in the slug — same as IBM/Carbon (`carbon-design-system`, not
`ibm-bim-system`).

#### Three candidates

| Name | Org | Parallel | 5-sec read |
|---|---|---|---|
| **`pointsav-bim-system`** ✓ | pointsav (vendor) | `design-system` ↔ `bim-system` — two substrates, same steward | "PointSav's BIM design substrate" |
| `pointsav-bim-tokens` | pointsav (vendor) | `@carbon/tokens` vocabulary; most technically precise | "DTCG bundle for BIM" — narrow but honest |
| `pointsav-building-design-system` | pointsav (vendor) | Longest; "building design system" is the pitch-deck phrase | "Carbon for buildings" — highest narrative payload, longer slug |

#### Recommendation: `pointsav-bim-system`

`<org>-<compound-noun>` shape matches every other Foundry vendor repo. The
`design-system` ↔ `bim-system` parallel makes the relationship to
`pointsav-design-system` legible at a glance — two repos, one for the UI/UX
semantic layer, one for the BIM/AEC semantic layer, same steward, same license.
No incumbent owns the `bim-system` slug on GitHub or npm — the namespace is clear.

The "BIM authoring software" overload risk (AEC industry uses "BIM system" for
software like Revit) is real but resolved in the README's first sentence.

#### License: change EUPL-1.2 → Apache 2.0

EUPL-1.2 is network-copyleft — a commercial AEC vendor (Autodesk, Bentley,
Nemetschek) cannot embed EUPL tokens in proprietary software without exposing
their product to copyleft demands. That kills adoption. IBM Carbon, Google Material,
Adobe Spectrum all use Apache 2.0 specifically to remove the copyleft barrier.
For a token standard whose value depends on becoming the industry default, the
license must be Apache 2.0.

All contributors to date are PointSav/Woodfine employees under CLA — re-licensing
authority is clean. Document the chain in a `NOTICE` file.

#### What changes (full move, not just rename)

This is a **repo transfer + rename + relicense**, not a local rename:

| Change | Detail |
|---|---|
| GitHub transfer | `woodfine/woodfine-design-bim` → `pointsav` org first; then rename to `pointsav-bim-system`. Do transfer before rename — easier to debug. |
| Local path | `customer/woodfine-design-bim/` → `vendor/pointsav-bim-system/` (crosses vendor/customer boundary) |
| Stage 6 path | No longer flows through Woodfine staging mirrors; flows through `origin-staging-j` / `origin-staging-p` like other `pointsav/*` vendor repos |
| Admin identity | `mcorp-administrator` → `ps-administrator` for any admin-tier commits |
| `pairings.yaml` | Move entry from customer section to vendor section |
| `PROJECT-CLONES.md` | Update project-bim block; add explicit "tokens repo: pointsav-bim-system" |
| `.agent/plans/README.md` routing | Rewrite line 28: `BIM-* → project-bim → pointsav-bim-system`; clarify boundary with `pointsav-design-system` (UI/UX tokens vs BIM tokens — neither subsumes the other); drop "Never routes to pointsav-design-system" caveat |
| project-bim cluster manifest | Add `pointsav-bim-system` as fourth sub-clone (fixes routing defect) |
| README / NOTICE | Add: "Initial token set authored by Woodfine Management Corp. for portfolio operations; published under PointSav vendor stewardship for AEC community adoption." |
| LICENSE file | Replace EUPL-1.2 → Apache 2.0; add NOTICE file documenting relicensing |
| CHANGELOG.md | One-line entry recording org transfer + relicense (BCSC discipline) |
| `git remote set-url origin` | Update in any existing clones after GitHub transfer settles |

---

## Sequencing note

Items 1–2 (org profiles) are admin-tier and need Command Session.
Items 3, 7, 8 can be Totebox sessions.
Items 4, 6, 9 are admin-tier or require careful content-forward review first.

**Do not attempt multiple repos in one session** — one session per `.git/` index.

---

## What "iterate forward" means for this project

When we touch any file for this elevation pass:
- If we **rewrite** a README: the old README's unique content (operational notes,
  internal vocabulary, anything not in the new version) must land in CLAUDE.md,
  a GUIDE, or an internal plan file before the old version is overwritten.
- If we **delete** a file: read it, extract, route. Then delete.
- If we **add** a file (ARCHITECTURE.md, org profile): it supersedes scattered
  architecture prose in READMEs — move that prose to the new file, then trim
  the README. No duplication.
- If we **link** repos: the link target must exist and must already say something
  worth linking to. Fix the target first, then add the link.
