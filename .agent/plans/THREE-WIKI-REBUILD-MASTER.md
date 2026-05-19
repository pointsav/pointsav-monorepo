# Three-Wiki Rebuild Master Todo — 2026-05-17

> **Scope:** Comprehensive rebuild of all three live wiki instances:
> Documentation (port 9090), Projects (port 9093), Corporate (port 9095).
> Findings from full UI/UX + content + link audit run 2026-05-17.
>
> **Pre-condition for most content work:** Stage 6 + binary rebuild
> must land first (outbox message sent) so the binary serves
> Sprints R→AC. The running binary is Sprint Q — missing several
> routes and UI improvements.

---

## Priority 0 — Engine Bugs (Sprint AD/AE candidates)

These affect all three wikis and make the content rebuild less meaningful
if shipped against the old binary.

### ~~P0-A: `.git` directory not filtered in `collect_topic_files`~~ SHIPPED Sprint AD
`dc0d3af3` — `if name_str.starts_with('.') { continue; }` added to directory branch
of `collect_topic_files`. Hidden dirs (`.git`, `.github`) no longer walk.

### ~~P0-B: `AGENT` not in `SYSTEM_FILE_STEMS`~~ SHIPPED Sprint AD
`dc0d3af3` — `"AGENT"` added to `SYSTEM_FILE_STEMS` const.

### ~~P0-C: Wikilink bare-slug 301 resolver~~ SHIPPED Sprint AD
`3514904e` — `resolve_bare_slug` async helper searches category subdirs for unique
stem match; 301-redirect to path-qualified slug. Fixes ~280+ broken wikilinks from
Wave-1 migration without content edits. Test `wiki_page_bare_slug_redirects_to_qualified` added.

### ~~P0-D: `<title>` tag shows only site title~~ SHIPPED Sprint AD
`dc0d3af3` — `wiki_chrome` now emits `{Article Title} — {Site Title}`.

### ~~P0-E: Tagline hardcoded~~ SHIPPED (Sprint AE — `ecd6b74a`)
`site_title.trim_end_matches(" Wiki")` at server.rs:2106 and 3015.
"Woodfine Projects Wiki" → "From Woodfine Projects"; "Woodfine Corporate Wiki" → "From Woodfine Corporate".

### ~~P0-F: SYSTEM_FILE_STEMS not in search index~~ SHIPPED (Sprint AE — `ecd6b74a`)
`SEARCH_EXCLUDED_STEMS` const + `is_excluded_stem()` in search.rs; hidden-dir skip in dir branch.
New test: `system_files_excluded_from_index`. 206+ tests passing.

### ~~P0-G: No skip-to-content link~~ ALREADY SHIPPED (Sprint AA)
Skip-to-content links present at lines 1017, 1877, 3359 in server.rs.
Search `<input>` aria-label still pending if needed — low priority.

### ~~P0-H: 404 page has no HTML chrome~~ ALREADY PRESENT
`src/error.rs` already has full HTML response: `<!doctype html>`, stylesheet,
header with home link, main with H1 and contextual message. The audit was
done against an older binary version — current error.rs is complete.

---

## Priority 1 — Stage 6 + Binary Rebuild (Master scope, already in outbox)

- [ ] **P1-1** `echo "y" | ~/Foundry/bin/promote.sh` — 15 commits ahead (Sprints R→AD)
- [ ] **P1-2** `cargo build --release` in `~/Foundry/vendor/pointsav-monorepo/app-mediakit-knowledge`
- [ ] **P1-3** `sudo cp + systemctl restart` for all 3 services
- [ ] **P1-4** Verify healthz: ports 9090, 9093, 9095
- [ ] **P1-5** Re-run broken-route check: confirm `/feed.atom`, `/feed.json`, `/llms.txt` on 9090 return 200 after rebuild (these were "Empty reply from server" against the old Sprint Q binary)

---

## Priority 2 — Corporate Wiki Content Fixes (5 quick YAML edits + 1 new file)

**Content dir:** `/srv/foundry/clones/project-knowledge/content-wiki-corporate/`

- [x] **C1** Fix `featured-topic.yaml`: slug now `topic-interest-coverage-ratio` ✓ (commit `1dee3de`)
- [x] **C2** Fix `leapfrog-facts.yaml`: `topic-` prefix on all 5 `link_slug` values ✓ (commit `1dee3de`)
- [x] **C3** Create `index.es.md` — Spanish home page created ✓ (commit `1dee3de`)
- [x] **C4** Fix `category: root` → ratified categories on all 5 articles ✓ (commit `1dee3de`)
- [x] **C5** Add `short_description` to all 5 articles ✓ (commit `1dee3de`)
- [x] **C6** Fix wikilink anchor text — pipe form applied to all 13 wikilinks ✓ (commit `1dee3de`)
- [x] **C7** Create `about.md`, `contact.md`, `disclaimers.md` stubs ✓ (commit `1dee3de`)

**NOTE — DIVERGENCE WARNING:** `1dee3de` was committed to the cluster-clone
(`/srv/foundry/clones/project-knowledge/content-wiki-corporate/`). The canonical
at `/srv/foundry/customer/content-wiki-corporate/` has an independent commit
`b0c78f6` that stripped `topic-` prefixes and used bare names. These two
conventions are now incompatible. Master must reconcile before Stage 6 for
content-wiki-corporate.

**Phase 2 — New Corporate Content (after C1–C7 stable):**
- [x] **C8** DONE (`cb53200`, 2026-05-19) — 4 company-identity topics: corporate-structure, vendor-customer-model, co-location-investment-thesis, regulatory-posture
- [x] **C9** DONE (`cb53200`, 2026-05-19) — 6 operational topics: continuous-disclosure, property-ledger-technology, investor-access, data-governance, asset-evaluation, technology-services
- [x] **C10** DONE (`cb53200`, 2026-05-19) — ES bilingual pairs for all 10 topics; same commit as C8+C9 (20 files total)

---

## Priority 3 — Projects Wiki Content Fixes

**Content dir:** `/srv/foundry/customer/content-wiki-projects/`

From content audit sub-agent findings:

- [x] **PJ1** DONE (`6765be0`, 2026-05-19) — Not two competing frameworks: same system at two levels. Methodology = conceptual overview; ranking-system = authoritative technical spec (12-rank matrix, site counts). Fixed: methodology's tier table descriptions corrected to accurately cover all 12 ranks; pointer to ranking-system spec added. ES articles were already consistent (no tier table in ES methodology). No changes to ranking-system article.
- [x] **PJ2** DONE (`b138b99`, 2026-05-19) — Canada was already complete; 5 partial stubs expanded (Italy, Mexico, Nordics, Poland, Spain): 2 new sections each (Anchor Network + country-specific convergence pattern), Provenance block added, quality partial→complete, last_edited updated. ES files: `paired_with:` frontmatter bug fixed (stray line → keyed YAML), `language_protocol: PROSE-TOPIC` → `TRANSLATE-ES`, full bilingual expansion to match EN. 10 files (5 EN + 5 ES) in one commit.
- [x] **PJ3** Add `short_description` to articles missing it — DONE (`2ec3a8f`): 26 files (13 EN + 13 ES). All wiki articles now have subtitle text.
- [x] **PJ4** VERIFIED (2026-05-19) — full pass across all projects wiki articles. No H1 in bodies, no skipped heading levels. Clean.
- [x] **PJ5** DONE (`2cb58df`, 2026-05-19) — renamed `comms/text-gis-nordic-coverage-release` → `comms/text-gis-nordic-coverage-2026-05`. Pattern established: `text-{topic}-{YYYY-MM}` for comms releases (date-qualified, no redundant `-release` suffix since `type: release-text` + `comms/` already identifies the artifact class). Slug not yet published (pre-Stage-6), so rename was safe. No wikilinks to update.
- [x] **PJ6** Bilingual parity verified (2026-05-19) — all EN wiki articles have ES pairs. Only non-wiki operational files lack ES (`.agent/rules/`, `CODE_OF_CONDUCT.md`, `TRADEMARK.md` — English-only by rule).
- [x] **PJ7** DONE (`leapfrog-facts.yaml` fix, 2026-05-19) — `featured-topic.yaml` was correct (`topic-` prefix present). `leapfrog-facts.yaml` had all 7 `link_slug` values missing `topic-` prefix; fixed. Both files now use correct slug form.
- [x] **PJ8** Verified (2026-05-19) — no article bodies contain "Michelin". Only occurrence is a YAML comment in `featured-topic.yaml` (not rendered). No BCSC posture issue.

---

## Priority 4 — Documentation Wiki Link-Rot (Systemic)

**Root cause:** Wave 1 migration (Sprint leapfrog-iteration-2) moved all
articles into category subdirs (`architecture/`, `substrate/`, `systems/`,
etc.) but did NOT update the `[[bare-slug]]` wikilinks inside those articles.
Affects approximately 280 of 304 articles.

**Two-track fix:**

### Track A: Engine fix (P0-C above — Phase 6 wikilink resolver)
Best long-term solution. A slug → path-qualified index built at startup
allows all existing `[[bare-slug]]` links to resolve transparently.
No content edits needed. This is Sprint AD.

### Track B: Content fix (manual, if engine fix is delayed)
Run a script that:
1. Builds a `{stem: path/stem}` map from the content dir
2. Scans every `.md` file for `[[bare-slug]]` or `](/wiki/bare-slug)` links
3. Replaces with `[[path/stem]]` or `](/wiki/path/stem)` where a unique match exists
4. Flags ambiguous cases for manual review

**Known flat-to-path mappings (from audit):**
| Bare slug | Actual path-qualified slug |
|---|---|
| `compounding-substrate` | `substrate/compounding-substrate` |
| `apprenticeship-substrate` | `substrate/apprenticeship-substrate` |
| `compounding-doorman` | `substrate/compounding-doorman` |
| `sovereign-ai-commons` | `substrate/sovereign-ai-commons` |
| `llm-substrate-decision` | `substrate/llm-substrate-decision` |
| `language-protocol-substrate` | `substrate/language-protocol-substrate` |
| `trajectory-substrate` | `substrate/trajectory-substrate` |
| `mcp-substrate-protocol` | `substrate/mcp-substrate-protocol` |
| `citation-substrate` | `substrate/citation-substrate` |
| `seed-taxonomy-as-smb-bootstrap` | `substrate/seed-taxonomy-as-smb-bootstrap` |
| `knowledge-graph-grounded-apprenticeship` | `substrate/knowledge-graph-grounded-apprenticeship` |
| `capability-based-security` | `architecture/capability-based-security` |
| `cryptographic-ledgers` | `architecture/cryptographic-ledgers` |
| `doorman-protocol` | `architecture/doorman-protocol` |
| `machine-based-auth` | `architecture/machine-based-auth` |
| `three-ring-architecture` | `architecture/three-ring-architecture` |
| `worm-ledger-architecture` | `infrastructure/worm-ledger-architecture` |
| `worm-ledger-design` | `infrastructure/worm-ledger-design` |
| `service-fs-architecture` | `services/service-fs-architecture` |
| `service-slm` | `services/service-slm` |
| `service-extraction` | `services/service-extraction` |
| `service-search` | `services/service-search` |
| `os-console` | `systems/os-console` |
| `os-infrastructure` | `systems/os-infrastructure` |
| `os-totebox` | `systems/os-totebox` |
| `edge-deployment` | `infrastructure/edge-deployment` |
| `anti-homogenization-discipline` | `governance/anti-homogenization-discipline` |
| `style-guide-guide` | `reference/style-guide-guide` |
| `style-guide-readme` | `reference/style-guide-readme` |
| `bim-token-three-layers` | `reference/bim-token-three-layers` |
| `bim-token-what-it-is` | `reference/bim-token-what-it-is` |
| `open-bim-regulatory-acceptance` | `reference/open-bim-regulatory-acceptance` |

**Files that appear broken but are genuinely missing** (verify before flagging):
- `single-boundary-compute-discipline` — check if exists under any subdir
- `pairing-as-permission` — check
- `yoyo-compute-substrate` — check
- `service-slm-architecture` — check (different from `services/service-slm`)
- `app-privategit-source-control` — check
- `four-tier-slm-substrate` — check (file may be `substrate/four-tier-slm-substrate`)
- `guide-ingress-operations` → should be `cluster-totebox-personnel/guide-ingress-operations`

**Content-level fix checklist (after engine fix is in or instead of it):**
- [x] **D1** `featured-topic.yaml` slug verified (2026-05-19) — `architecture/economic-model` is path-qualified. No fix needed.
- [x] **D2** `leapfrog-facts.yaml` link_slug values verified (2026-05-19) — all 7 are path-qualified (`substrate/`, `infrastructure/`, `architecture/` prefixes). No fix needed.
- [x] **D3** DONE (`cf72e67`, 2026-05-19) — No stale counts found (no numeric counts in any _index.md). Real gap: substrate/ and patterns/ categories had 7/32 and 3/10 article coverage respectively — stub MOC pages from the 2026-05-09 category split. Expanded both to full curated MOC coverage: substrate (6 thematic sections, 32 articles EN+ES), patterns (4 thematic sections, 10 articles EN+ES).
- [x] **D4** Resync `index.es.md` — verified current (2026-05-18); no staleness found
- [x] **D5** Add `short_description` to all articles missing it — DONE Sprint AE/AF (`c8192fc`): 162 files (60 EN + 100 ES + 2 _index). All wiki articles now have subtitle text. Stub nightly-datagraph-rebuild EN+ES also completed.
- [x] **D6** DONE (`a07bdf5`, 2026-05-19) — Original scope moot: 12 sys-adr stubs were consolidated into `architecture-decisions.md` (commit `62d71fb`, 2026-05-15). Remaining governance work completed: `sovereign-airlock-doctrine` EN+ES rewritten (stale vocabulary, broken frontmatter); `moonshot-initiatives`, `ontological-governance`, `sovereign-replacement-initiative` EN+ES elevated stub→complete with substantive additions; governance `_index.md`+`.es.md` expanded with 3 new sections covering 8 previously-unlisted articles.
- [x] **D7** Moot — `guide-component-*` files moved to `pointsav-design-system/` in design-system/ Batch 2 (closed handoff). No ES bilingual pairs needed here.
- [x] **D8** Schema normalization — governance/_index.md and design-system/_index.md frontmatter fixed (type, quality, short_description, paired_with added). Remaining articles verified conformant.
- [x] **D9** Moot — `AGENT.md` is in `repo-layout.md §1` allowed-files list; Sprint AD already added `"AGENT"` to `SYSTEM_FILE_STEMS`. No action needed.
- [ ] **D10** Engine fix: Once P0-C (wikilink resolver) lands, run validation pass to confirm all 280+ previously-broken links now resolve

---

## Priority 5 — Guide Dir Housekeeping

- [x] **G1** MOOT — file does not exist in `.git/`; P0-A engine fix already prevents hidden-dir walk. No action needed.
- [x] **G2** DONE (`a06f64f`, 2026-05-19) — removed from cluster-clone (`/srv/foundry/clones/project-knowledge/woodfine-fleet-deployment/`). **NOTE:** Canonical at `/srv/foundry/customer/woodfine-fleet-deployment/` still has the file — that path is `guide_dir_2` for `local-knowledge-documentation.service`. Canonical removal requires Command Session admin-tier commit to `woodfine/woodfine-fleet-deployment`. Outbox message sent.
- [ ] **G3** `pointsav-fleet-deployment/.git` is being walked by the engine even if no MD files currently pass — P0-A engine fix prevents this but the content dir walk order is still fragile. Consider checking if INVENTORY.yaml and other root files in `pointsav-fleet-deployment` leak through.

---

## Priority 6 — Ops/Config Fixes

- [ ] **O1** Confirm that Sprint Q binary's `/feed.atom`, `/feed.json`, `/llms.txt` regression on port 9090 resolves after Stage 6 rebuild
- [ ] **O2** After rebuild, re-run full sitemap HTTP check against all 304 doc wiki URLs
- [ ] **O3** After P0-A engine fix: confirm `.git/guide-provision-node` drops from sitemap

---

## Sprint AD Candidates (next engine sprint after Stage 6)

Ranked by impact vs complexity:

| Sprint | What | Impact |
|---|---|---|
| AD-1 | Wikilink slug resolver (P0-C) | Unblocks 280+ broken links; highest user-visible impact |
| AD-2 | `.git` dir filter + AGENT stem fix (P0-A/P0-B) | Fixes sitemap pollution + stray article |
| AD-3 | `<title>` article-first (P0-D) | SEO + muscle memory |
| AD-4 | Tagline from site_title (P0-E) | Correct cross-wiki branding |
| AD-5 | 404 chrome (P0-H) | "Create this article" affordance |
| AD-6 | Search index system-file filter (P0-F) | Cleaner search results |

---

## Content Rebuild Sequencing (recommended)

```
Stage 6 rebuild → C1–C7 (corporate quick fixes, one commit) →
PJ1–PJ8 (projects) → Sprint AD (engine) → D1–D10 (documentation, after engine) →
New corporate content C8–C10 → New projects content
```

---

*Plan authored 2026-05-17 by Totebox Claude on project-knowledge cluster.
Based on: 4 content audit sub-agents, 3 UI/UX audit sub-agents, parent
session direct curl audit of all 3 wikis (304 + 18 + 5 sitemap URLs).*
