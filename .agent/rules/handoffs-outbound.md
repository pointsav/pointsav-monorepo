# Handoffs Outbound — project-language cluster

Pending cross-repo moves originating from this cluster's refined output.
Each entry follows the passive-outbox pattern per CLAUDE.md §11.

State values: `pending-destination-commit` | `destination-committed` | `closed`

---

## from-project-gis batch (processed 2026-05-19)

All 9 drafts in `.agent/drafts-outbound/from-project-gis/` reviewed and cleared.

| Draft | Destination | State |
|---|---|---|
| guide-gis-adding-a-chain.md | woodfine-fleet-deployment/gateway-orchestration-gis/ | destination-committed (d10c971) |
| text-gis-nordic-coverage-release.md + .es | content-wiki-projects/comms/ | destination-committed (34f9500) |
| topic-gis-nordic-uk-coverage-2026-05-06.md | content-wiki-projects/ (as topic-gis-nordic-uk-coverage.md) | destination-committed (a9d5325) |
| topic-app-orchestration-gis.md | content-wiki-documentation/applications/ | destination-committed (prior session) |
| topic-service-fs-data-lake.md | content-wiki-documentation/services/ | destination-committed (prior session) |
| topic-service-business-clustering.md | content-wiki-documentation/services/ | destination-committed (prior session) |
| topic-service-places-filtering.md | content-wiki-documentation/services/ | destination-committed (prior session) |
| topic-co-location-index-canada.md | content-wiki-projects/ | destination-committed (prior session) |
| topic-co-location-index-italy.md | content-wiki-projects/ (stub) | pending-destination-commit |

**Italy note:** `topic-co-location-index-italy.md` exists in content-wiki-projects as an empty stub (data table unpopulated). project-gis must supply actual cluster data before the stub can be published. Bounce to project-gis outbox.

---

## Leapfrog-2030 batch (workspace v0.1.96 / refined 2026-04-30)

Refined output staging area: `clones/project-language/.claude/drafts-outbound/refined/leapfrog-2030/`

### 1. Doctrine amendment

| Field | Value |
|---|---|
| Source path | `refined/leapfrog-2030/doctrine/doctrine-v0.1.0-leapfrog-major-amendment.md` |
| Destination repo | `~/Foundry/` (workspace root — Master scope) |
| Destination path | `DOCTRINE.md` (MAJOR amendment; replaces current v0.0.14) |
| Destination role | **Master Claude** (direct workspace commit, admin-tier procedure) |
| State | closed |
| Notes | DOCTRINE.md already at v0.1.0 with all 12 claims (#43–#54) applied. Ratified by operator 2026-05-01. No further action needed. |

### 2. Conventions (12 files)

| Field | Value |
|---|---|
| Source path | `refined/leapfrog-2030/conventions/convention-*.md` (12 files) |
| Destination repo | `~/Foundry/conventions/` (workspace root — Master scope) |
| Destination paths | 11 files committed at workspace v0.1.102; `direct-payment-settlement.md` committed 2026-05-01 at workspace v0.1.112 |
| Destination role | **Master Claude** (direct workspace commit, admin-tier procedure) |
| State | closed |
| Notes | All 12 convention files now in ~/Foundry/conventions/. Committed 2026-05-01. |

### 3. INVENTIONS.md addition

| Field | Value |
|---|---|
| Source path | `refined/leapfrog-2030/inventions/inventions-2030-leapfrog.md` |
| Destination repo | `vendor/pointsav-monorepo` |
| Destination path | `INVENTIONS.md` (append to existing file, or replace leapfrog-2030 section) |
| Destination role | **Root Claude** at `vendor/pointsav-monorepo` |
| State | closed |
| Notes | INVENTIONS.md already contains all claims #43–#54 (committed 45d823c). Confirmed 2026-05-01. |

### 4. service-content architecture rebuild

| Field | Value |
|---|---|
| Source path | `refined/leapfrog-2030/architecture/service-content-architecture-rebuild.md` |
| Destination repo | `vendor/pointsav-monorepo` |
| Destination path | `service-content/ARCHITECTURE.md` (new file or rebuild of existing) |
| Destination role | **Root Claude** at `vendor/pointsav-monorepo` (or Task Claude at project-data/project-slm cluster) |
| State | closed |
| Notes | Committed at 88df1c9 (2026-05-01) by Master session. Pushed to canonical + both staging remotes. |

### 5. Leapfrog-2030 public TOPIC (bilingual)

| Field | Value |
|---|---|
| Source paths | `refined/leapfrog-2030/topics/topic-leapfrog-2030-architecture.md` + `topic-leapfrog-2030-architecture.es.md` |
| Destination repo | `vendor/content-wiki-documentation` |
| Destination path | `architecture/topic-leapfrog-2030-architecture.md` + `architecture/topic-leapfrog-2030-architecture.es.md` |
| Destination role | **Root Claude** at `vendor/content-wiki-documentation` |
| State | closed |
| Notes | Committed at `7ee9576` (2026-05-01) by Root Claude in `architecture/` category. Confirmed via remote merge 784b8f8. Source files remain in staged area; source-side git rm pending at project-language Root (or Task) cleanup pass. |

### 6. Tier A sysadmin TUI GUIDE

| Field | Value |
|---|---|
| Source path | `refined/leapfrog-2030/guides/guide-tier-a-sysadmin-tui.md` |
| Destination repo | `customer/woodfine-fleet-deployment` |
| Destination path | `vault-privategit-source/guide-tier-a-sysadmin-tui.md` |
| Destination role | **Root Claude** at `customer/woodfine-fleet-deployment` |
| State | closed |
| Notes | Already committed at 3aed0d7 (bundle with guide-doorman + guide-operating-yoyo). Confirmed 2026-05-01. |

---

## project-knowledge GUIDE batch (refined 2026-04-30)

Refined output staging: `clones/project-language/.claude/drafts-outbound/refined/from-project-knowledge-guides/`

### 7. guide-operate-knowledge-wiki

| Field | Value |
|---|---|
| Source path | `refined/from-project-knowledge-guides/guide-operate-knowledge-wiki.md` |
| Destination repo | `customer/woodfine-fleet-deployment` |
| Destination path | `media-knowledge-documentation/guide-operate-knowledge-wiki.md` |
| Destination role | **Root Claude** at `customer/woodfine-fleet-deployment` |
| State | closed |
| Notes | Committed at 58b2a54 (2026-05-01) with guide-keep-the-home-page-the-gold-standard in same commit. Pushed to all three remotes. |

### 8. guide-keep-the-home-page-the-gold-standard

| Field | Value |
|---|---|
| Source path | `refined/from-project-knowledge-guides/guide-keep-the-home-page-the-gold-standard.md` |
| Destination repo | `customer/woodfine-fleet-deployment` |
| Destination path | `media-knowledge-documentation/guide-keep-the-home-page-the-gold-standard.md` |
| Destination role | **Root Claude** at `customer/woodfine-fleet-deployment` |
| State | closed |
| Notes | Committed at 58b2a54 (2026-05-01). Pushed to all three remotes. |

---

## Woodfine Co-location batch (refined 2026-05-02)

Refined output staging area: `clones/project-language/.claude/drafts-outbound/refined/co-location/woodfine/`

### 10. Co-location Intelligence Topics (Bilingual — 10 files)

| Field | Value |
|---|---|
| Source path | `refined/co-location/woodfine/topic-co-location-*.md` (10 files) |
| Destination repo | `woodfine/content-wiki-projects` |
| Destination path | `./` |
| Destination role | **Master Claude** or **Root Claude** at destination |
| State | `closed` |
| Notes | 5 English topics + 5 Spanish strategic adaptations committed by Gemini CLI session (96c3f26, cfd74f0) and outside voice cleanup commit (3c16de8, 2026-05-04). Closes project-gis gateway pass. |
