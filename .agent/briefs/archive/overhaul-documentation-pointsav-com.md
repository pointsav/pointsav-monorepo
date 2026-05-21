---
schema: foundry-plan-v1
slug: overhaul-documentation-pointsav-com
status: in-progress
phase: 1
owner: project-editorial
created: 2026-05-14
last_edited: 2026-05-14
engines: [gemini-cli, claude-code]
supersedes:
  - prior session 2026-05-14 batches A/B/D — content live, now overhauling for Wikipedia parity
---

# Overhaul of documentation.pointsav.com — Two-Phase Plan

> **Read this entire file before executing any task.** It is the single source of
> truth for the overhaul. Phase 1 is for Gemini CLI; Phase 2 is for Claude Code.
> Do not start Phase 2 until Phase 1 has produced
> `.agent/plans/overhaul-gemini-analysis.md` and all light-work commits have landed.

---

## 1. Context and intent

`documentation.pointsav.com` is the public institutional knowledge wiki for the
PointSav platform. As of 2026-05-14 the site is live with 257 committed TOPICs
across ten categories, 27 staged ZIP draft TOPICs, and roughly 80 GUIDEs across
the two fleet-deployment repositories.

The intended outcome is a documentation site that **leapfrogs hyperscaler
documentation** (AWS, GCP, Azure) in readability, navigability, and reference
quality — and is structurally set up to get better with every future editorial
pass. Specifically:

- **Wikipedia-calibre articles.** Lede that orients in one paragraph. Headers that
  describe, not label. Inline citations at point of claim. One link per 20 words
  minimum; 12–25 wikilinks in the lead for technical articles.
- **Bloomberg/Google Docs register.** Active voice, present tense for facts,
  concrete short sentences. Every GUIDE: numbered steps, imperative verbs,
  fenced code blocks with language tags, prerequisite and outcome declarations.
- **Zero broken links.** Every `[[wikilink]]` resolves to an existing file. Every
  rename and removal triggers an immediate link-repair sweep. No reader lands on
  a blank page or 404.
- **A compounding quality structure.** Readability rules are encoded as TOPICs in
  the wiki itself (`reference/style-guide-topic.md`, `reference/style-guide-guide.md`).
  Gemini's analysis produces a re-runnable checklist. Future sessions improve the
  margin, not the baseline.

**Quality metrics** (measurable at Phase 2 completion):
- Dead internal wikilinks: 0
- TOPICs with < 2 external references: 0
- TOPICs with < 3 `[[wikilink]]` instances in body: 0
- `bcsc_class: internal` entries: 0
- Personal names in body text or filenames: 0
- Governance vocabulary in body text: 0
- GUIDEs with prose-embedded commands not in fenced code blocks: 0
- TOPIC lede > 90 words or < 60 words: flagged (not hard-blocked)
- Label-only headers (Overview, Background, Details, How it works): 0

---

## 2. Current state snapshot (2026-05-14)

### 2.1 Committed TOPICs in `content-wiki-documentation/` (257 articles)

| Category | Count | Notes |
|---|---|---|
| architecture | 37 | Largest topical category; well-formed |
| substrate | 36 | Includes mis-named `jennifer-datagraph-rebuild` |
| patterns | 10 | Two duplicates with applications/ |
| systems | 11 | Mixed filename conventions (`topic-` prefix drift) |
| services | 19 | Well-formed |
| applications | 7 | Two files that are canonical to patterns/ |
| governance | 20 | Includes 11 ADRs (sys-adr-06 through sys-adr-19) |
| infrastructure | 10 | Contains one misrouted GUIDE |
| reference | 55 | Overloaded; needs sub-routing audit |
| design-system | 35 | Well-formed |

### 2.2 Staged ZIP drafts (27 articles)

Location: `.agent/drafts-outbound/zip-topic-*.md`. All `bcsc_class: internal`,
~550–700 words each. Full list:

| Slug | Title |
|---|---|
| zip-topic-app-console-input-f12 | The F12 Input Machine |
| zip-topic-archetypes-and-coa | Chart of Accounts and Eleven Archetypes |
| zip-topic-bim-product-family | BIM and Real-Property Surfaces |
| zip-topic-competitive-positioning | Structural Positioning |
| zip-topic-compliance-disclosure | Compliance and Continuous Disclosure |
| zip-topic-deployment-patterns | Deployment Patterns — Universal Companion |
| zip-topic-design-system | Typography and Brand |
| zip-topic-hardware-research | Hardware Reference |
| zip-topic-leapfrog-2030 | The Leapfrog 2030 Thesis |
| zip-topic-legal-ip-structure | Three Orgs, Squash-and-Merge, Double-Blind Air-Gap |
| zip-topic-machine-based-authorization | Machine-Based Authorization — Pairing as Permission |
| zip-topic-microkernel-substrate | The seL4 Microkernel Substrate |
| zip-topic-os-console | os-console — The Command Ledger |
| zip-topic-os-family-overview | The OS Family — Eight Operating Systems, One Substrate |
| zip-topic-os-infrastructure-network | os-infrastructure and os-network-admin |
| zip-topic-os-mediakit | os-mediakit — The Sovereign Compliance Appliance |
| zip-topic-os-orchestration | os-orchestration — The Fleet Aggregator |
| zip-topic-os-totebox | os-totebox — The Sovereign Vault and Service Host |
| zip-topic-os-workplace | os-workplace — The Sovereign Desktop |
| zip-topic-pointsav-overview | PointSav — Company Overview and Three-Org Structure |
| zip-topic-service-content | service-content — The Gravity Engine |
| zip-topic-service-email-people | service-email and service-people — Ingest and Identity |
| zip-topic-service-slm | service-slm — The Institutional Small Language Model |
| zip-topic-six-tier-sovereignty-matrix | The Six-Tier Sovereignty Matrix |
| zip-topic-supply-chain-governance | The Five-Stage Sovereign Supply Chain |
| zip-topic-the-diode-standard | The Diode Standard — Unidirectional Command Flow |
| zip-topic-three-layer-architecture | The Three-Layer Architecture |

### 2.3 GUIDEs

- `woodfine-fleet-deployment/`: ~68 files across 18 project directories
- `pointsav-fleet-deployment/`: ~12 files

GUIDEs are English-only. Primary defect: prose-embedded commands instead of
fenced code blocks.

---

## 3. Editorial constraints (non-negotiable)

1. **Register.** Bloomberg / Oxford Business + Google Documentation. Active voice,
   present tense for facts. Banned: "harness", "unlock", "empower", "leverage",
   "synergy", "robust", "seamless", "simply", "easy", "utilize".
2. **BCSC posture.** Forward-looking claims: "planned / intended / may / target".
   Sovereign Data Foundation equity stake: planned/intended only.
3. **No personal names.** No "Jennifer", "Mathew", "Peter" in any body text or
   filename. Use role nouns: "the operator", "the editor", "the chief architect".
4. **Bilingual mandate.** Every TOPIC commit carries EN + ES pair. GUIDEs: English-only.
5. **No governance vocabulary.** "Doctrine claim #N", "Conventions/", internal
   Foundry filenames — none appear in public wiki text.
6. **No competitive comparisons by name.** Structural positioning only.
7. **Wikilink format.** `[[slug]]` — no `topic-` prefix in slugs.
8. **Ten active categories** (exact): architecture, substrate, patterns, systems,
   services, applications, governance, infrastructure, reference, design-system.

---

## 4. Flag resolution table (operator-decided — apply without asking)

| Trigger | Resolution |
|---|---|
| "PointSav Digital Systems AG" | Replace with `PointSav Digital Systems` |
| `[SUPERSEDED BY: ...]` marker | Apply replacement, remove marker |
| BIM content in ZIP drafts | Route to `woodfine-design-bim`; here: summarise + cross-reference only |
| SDF equity stake language | Always "planned" or "intended"; never present-tense |
| Doctrine-conflict flags in ZIP drafts | Drop the flag; content is human-reviewed and cleared |
| `bcsc_class: internal` in ZIP draft | Downgrade to `public-disclosure-safe` after language pass |
| `notes_for_editor:` block in ZIP draft | Remove entirely before commit |
| Governance vocabulary in body | Re-prose into institutional plain English |
| Personal-name mention in body | Replace with role noun |

---

## 5. Known issues checklist

### 5.1 Duplicates — pick canonical, remove the other

- `applications/article-shell-leapfrog.md` vs `patterns/article-shell-leapfrog.md`
  → **Canonical: `patterns/`**. Remove `applications/` copy + `.es.md`.
- `applications/knowledge-wiki-home-page-design.md` vs `patterns/knowledge-wiki-home-page-design.md`
  → **Canonical: `patterns/`**. Remove `applications/` copy + `.es.md`.

### 5.2 Misrouted file

- `infrastructure/guide-totebox-orchestration-gis.md`
  → Move to `woodfine-fleet-deployment/gateway-orchestration-gis-1/`
  (verify cluster dir; use closest matching gateway). Remove from `infrastructure/`.
  Two commits: one per repo.

### 5.3 `systems/` filename normalisation

Rename to drop `topic-` prefix (+ `.es.md` pairs). After rename: update all
wikilinks referencing old slugs across the full content tree and both fleet repos.

| Old filename | New filename |
|---|---|
| `topic-console-os.md` | `console-os.md` |
| `topic-infrastructure-os.md` | `infrastructure-os.md` |
| `topic-mediakit-os.md` | `mediakit-os.md` |
| `topic-totebox-archive.md` | `totebox-archive.md` |
| `topic-totebox-orchestration.md` | `totebox-orchestration.md` |
| `topic-totebox-os.md` | `totebox-os.md` |

### 5.4 Personal name in filename

- `substrate/jennifer-datagraph-rebuild.md` → `substrate/nightly-datagraph-rebuild.md`
  (+ `.es.md`). Update wikilinks site-wide.

### 5.5 `reference/` overload (Phase 2 executes)

Phase 1 proposes sub-buckets; Phase 2 moves files. Candidates:
`reference/style/`, `reference/bim/`, `reference/editorial-process/`,
`reference/glossary/`.

---

## 6. Readability standard — checkable rules

Every line written in Phase 2 must meet these rules. They override session
instinct. They are also encoded in-wiki as TOPICs (`reference/style-guide-topic.md`,
`reference/style-guide-guide.md`) so future sessions can reference them directly.

### 6.1 TOPIC lede (first paragraph — no header above it)

**Target: 60–90 words. Four sentences, in this order:**

1. **Definition.** `<Subject> is <noun phrase that names the kind of thing>.`
2. **Function.** What it does or what role it plays in the platform.
3. **Distinguishing trait.** What separates it from the adjacent concept
   a reader might confuse it with.
4. **Reader position.** `By the end of this article, a reader will understand
   <X, Y, Z>.` Three noun phrases; no verbs.

Example (zero-container-inference):
> "Zero-container inference is the deployment pattern used for Tier B GPU compute:
> native binaries under systemd, with no container runtime. It runs short scheduled
> inference jobs at the unit cost of the underlying GPU-hour, then shuts idle. It
> differs from managed container inference because the registry, image build chain,
> and orchestrator are absent — the binary is the artefact. By the end of this
> article, a reader will understand the pattern, its cost structure, and the one
> trade-off it accepts."

### 6.2 TOPIC headers — descriptive, not labels

Every `##` heading completes the sentence *"This section explains …"*.

**Banned headers:** Overview, Background, Details, How it works, Introduction,
Summary, Notes.

**Good:** `Why the Doorman is the boundary, not the model`
**Bad:** `Architecture`

### 6.3 Link density

Per Wikipedia MOS:LINK and published guidelines for technical wikis:
- Body: approximately one `[[wikilink]]` per 20 words (minimum floor).
- Lede: 12–25 wikilinks for technical articles; higher density than body is
  correct because the lede introduces the most concepts in the least space.
- Do not link the same slug more than once per section.

### 6.4 Article length

Per KCS guidance and information architecture best practice:
- Target: 800–2,000 words per TOPIC.
- Split threshold: >2,500 words triggers a split proposal.
- Consolidate threshold: <400 words (stub) triggers a merge-or-expand proposal.
- TOC anchors (`## `) are mandatory above 1,200 words.

### 6.5 Paragraph length discipline

- Maximum eight prose lines before a header, bullet list, or table.
- If a section does two jobs, give it two headers.
- Bullets for parallel items; prose for narrative flow. Never mix.

### 6.6 Reference integration

External citations appear **inline at the point of claim**, not only in a
trailing bibliography. Format: `per [Publisher, Year][^N]` or `per the
[Standard Name][^N]`, with the expansion in `## References`. Bare URL
dumps at end-of-article are prohibited. Minimum two external references
per TOPIC; minimum one of those must be inline.

### 6.7 GUIDE prerequisite + outcome block

Immediately after the H1, before any prose:

```markdown
## Before you start

You will need:
- **<Noun>**: <one line, with [[wikilink]] to the TOPIC or GUIDE that
  explains it>
- **<Noun>**: …

You should be able to verify: `<one command whose success confirms readiness>`.

## When you finish

The following will be true:
- <State, verifiable in one command>
- <State, verifiable in one command>
```

### 6.8 GUIDE step structure

Every step is one numbered block with four parts, in this order:

1. **Imperative sentence.** One verb. (`Submit the intent. Verify the
   translation. Roll back the deployment.`)
2. **Command.** Fenced code block with language tag.
3. **Expected output.** Fenced block, labelled `text` if not parseable.
4. **If it fails.** Named failure mode → recovery command or link to
   recovery GUIDE.

### 6.9 Register rules

- Active voice. If the subject is unknown, restructure — do not reach for
  passive.
- Present tense for current facts. "planned / intended / target" for
  forward-looking statements.
- Maximum sentence length: 30 words. Prefer 15–20.
- Concrete nouns. Avoid "the system", "it", "this". Name the component.

---

## 7. Broken-link audit procedure

**Zero broken links is a hard requirement.** A broken wikilink signals
incomplete content to every reader and undermines trust in the entire corpus.
Run audits at three points: before Phase 1 begins (baseline), after each
rename/removal commit (incremental), and after Phase 2 completes (final).

### 7.1 Baseline + incremental audit script

Run once before any Phase 1 rename. Outputs `/tmp/broken-links.tsv` with
columns: `source_file`, `line`, `slug`, `display`, `category_hint`,
`suggested_replacement`.

```bash
python3 - <<'PY'
import os, re, sys, pathlib

ROOTS = [
  "/srv/foundry/clones/project-editorial/content-wiki-documentation",
  "/srv/foundry/clones/project-editorial/woodfine-fleet-deployment",
  "/srv/foundry/clones/project-editorial/pointsav-fleet-deployment",
]
CONTENT = ROOTS[0]

# Pre-computed redirect table: old slug -> new slug.
# Update this table before Phase 1 runs if additional renames are planned.
REDIRECTS = {
    "topic-console-os":          "console-os",
    "topic-infrastructure-os":   "infrastructure-os",
    "topic-mediakit-os":         "mediakit-os",
    "topic-totebox-os":          "totebox-os",
    "topic-totebox-archive":     "totebox-archive",
    "topic-totebox-orchestration":"totebox-orchestration",
    "jennifer-datagraph-rebuild": "nightly-datagraph-rebuild",
    # applications/ duplicate removals — point to canonical patterns/ slug
    "applications/article-shell-leapfrog": "article-shell-leapfrog",
    "applications/knowledge-wiki-home-page-design": "knowledge-wiki-home-page-design",
}

# Build slug index from filenames (EN only, excludes _index and .es.md)
slugs = {}
for d in pathlib.Path(CONTENT).iterdir():
    if not d.is_dir() or d.name.startswith('.') or d.name == 'archive':
        continue
    for f in d.glob("*.md"):
        if f.name.endswith(".es.md") or f.name.startswith("_index"):
            continue
        slugs[f.stem] = f"{d.name}/{f.name}"
slugs["index"] = "index.md"

WIKILINK = re.compile(r'\[\[([^\]\|\n]+?)(?:\|([^\]\n]+))?\]\]')

def iter_md_lines(root):
    for path in pathlib.Path(root).rglob("*.md"):
        if path.name.endswith(".es.md"):
            continue
        in_fence = False
        for n, line in enumerate(path.read_text(encoding="utf-8", errors="replace").splitlines(), 1):
            if line.lstrip().startswith("```"):
                in_fence = not in_fence
                continue
            if in_fence:
                continue
            yield path, n, line

broken = []
for root in ROOTS:
    for path, n, line in iter_md_lines(root):
        for m in WIKILINK.finditer(line):
            target = m.group(1).strip()
            display = (m.group(2) or "").strip()
            cat_hint = ""
            if "/" in target:
                cat_hint, target = target.split("/", 1)
            # Require slug-shaped target (guards against bash [[ -e ]] false positives)
            if not re.fullmatch(r'[a-z0-9][a-z0-9\-]*', target):
                continue
            if target in slugs:
                continue
            suggestion = REDIRECTS.get(target, "")
            broken.append((str(path), n, target, display, cat_hint, suggestion))

out = pathlib.Path("/tmp/broken-links.tsv")
with out.open("w") as fh:
    fh.write("source_file\tline\tslug\tdisplay\tcategory_hint\tsuggested_replacement\n")
    for row in broken:
        fh.write("\t".join(str(x) for x in row) + "\n")
print(f"broken={len(broken)}  index_size={len(slugs)}  report={out}")
PY
```

### 7.2 Per-commit targeted re-audit

After each `git mv` or `git rm`, immediately run:

```bash
OLD=jennifer-datagraph-rebuild  # substitute the renamed/removed slug
grep -rnE "\[\[${OLD}(\||\]\])" \
  /srv/foundry/clones/project-editorial/content-wiki-documentation \
  /srv/foundry/clones/project-editorial/woodfine-fleet-deployment \
  /srv/foundry/clones/project-editorial/pointsav-fleet-deployment \
  --include='*.md' --exclude='*.es.md'
```

**Output must be empty before staging the commit.** Non-empty output means stale
callers exist — repair them in the same commit, not a follow-up.

The `(\||\]\])` lookahead prevents partial-slug false positives.

### 7.3 External URL audit (lowest priority — run after Phase 2 promotes)

```bash
python3 - <<'PY'
import re, urllib.request, pathlib, concurrent.futures as cf

URL = re.compile(r'https?://[^\s\)\]<>"]+')
urls = set()
in_refs = False

for p in pathlib.Path(
    "/srv/foundry/clones/project-editorial/content-wiki-documentation"
).rglob("*.md"):
    if p.name.endswith(".es.md"):
        continue
    for line in p.read_text(errors="replace").splitlines():
        if line.startswith("## References"):
            in_refs = True; continue
        if line.startswith("## ") and in_refs:
            in_refs = False
        if in_refs:
            for u in URL.findall(line):
                urls.add((str(p), u.rstrip('.,);')))

def head(item):
    p, u = item
    try:
        req = urllib.request.Request(
            u, method="HEAD", headers={"User-Agent": "link-audit/1"}
        )
        with urllib.request.urlopen(req, timeout=8) as r:
            return (p, u, r.status)
    except Exception as e:
        return (p, u, f"ERR:{type(e).__name__}")

with cf.ThreadPoolExecutor(max_workers=16) as ex:
    for p, u, s in ex.map(head, urls):
        if str(s).startswith("ERR") or (isinstance(s, int) and s >= 400):
            print(f"{p}\t{u}\t{s}")
PY
```

Dead external URLs are filed in `content-wiki-documentation/.agent/rules/cleanup-log.md`
as open items with date. They do not block promotion.

### 7.4 Integration into the two-phase plan

- **Phase 1 (Gemini):** Run §7.1 before any rename. Commit the report as
  `audit/baseline-broken-links.tsv` in the project-editorial cluster root
  (not inside a sub-clone). All rows with a non-empty `suggested_replacement`
  are deterministic rewrites; Gemini adds them to the Phase 1 light-work
  manifest. Rows with empty suggestions are surfaced to the operator before
  Phase 2 begins.
- **Phase 2 (Claude):** After each rename/removal, run §7.2 with the affected
  slug. Empty output required before staging. After all sub-phases complete,
  re-run §7.1 and confirm broken count is zero.
- **Post-promote:** Run §7.3 once against canonical. File findings.

---

## 8. Phase 1 — Gemini CLI: Analysis + light work

### 8.1 Session start sequence

1. Read `.agent/inbox.md` and current NOTAM
2. Read `.agent/rules/design-tokens.md` and `.agent/rules/handoffs-outbound.md`
3. Read `.agent/plans/README.md`
4. Read `.agent/session-start.md`
5. Read this file in full
6. Read `.agent/plans/overhaul-progress.md` (see §14.1 for schema). If absent and this is the first Gemini session, initialise it from the §14.1 template with `phase: 1`, `status: in-progress`, `owner_engine: gemini-cli`.
7. Run the mid-sub-phase recovery procedure (§14.3) end-to-end. No exceptions, even on the first day.
8. Verify `phase: 1` in progress file and `owner_engine` equals `gemini-cli`. Mismatch = STOP and surface to operator.
9. Run §13.1 vocabulary-baseline extraction — commit `vocabulary-baseline.tsv` to `.agent/plans/`.
10. Run §7.1 broken-link baseline — commit the TSV to project-editorial cluster.

Then `git status` clean in all three sub-clones before proceeding.

**Scope fence (mandatory):** Before authoring any commit, re-read §8.4 — only the 6 listed light-work commit types are within Phase 1 scope. Refuse to author any commit outside that list.

### 8.2 Required reads

- `content-wiki-documentation/index.md` (root home page)
- `content-wiki-documentation/<category>/_index.md` for all ten categories
- All 257 EN TOPIC files
- All 27 ZIP drafts at `.agent/drafts-outbound/zip-topic-*.md`
- All GUIDE files in both fleet repos
- `content-wiki-documentation/patterns/knowledge-wiki-home-page-design.md`
- `content-wiki-documentation/patterns/wikipedia-leapfrog-design.md`
- `content-wiki-documentation/reference/wikipedia-structure.md`
- `content-wiki-documentation/reference/style-guide-topic.md`
- `content-wiki-documentation/reference/style-guide-guide.md`

### 8.3 Deliverable: `overhaul-gemini-analysis.md`

Write `.agent/plans/overhaul-gemini-analysis.md` with eight sections:

**§1 — Catalogue proposal.** One row per TOPIC:
`| slug | current category | recommended action | target category | notes |`

Recommended action (exactly one of): `keep` | `expand` | `consolidate-with:<slug>` |
`split-into:<slug-a>,<slug-b>` | `retire` | `move-category:<target>` |
`move-to-guide:<cluster>` | `move-from-guide:<path>`

**§2 — ZIP-to-TOPIC mapping.** One row per ZIP draft:
`| zip slug | disposition | target | notes |`

Disposition: `merge-into:<existing-slug>` OR `new:<proposed-slug>,<category>`

**§3 — Navigation audit.** For each of the 11 landing pages: current lede,
mismatch flags, proposed rewrite outline (outline only — not final prose).

**§4 — Cross-reference gap list.** Two sub-tables:
- Bare-concept references: `| source-file | line | concept-named | target-slug |`
- Orphan TOPICs: `| slug | incoming-link-count | recommended-inbound-sources |`

**§5 — Reference proposals.** For each TOPIC with fewer than two external
references: two suggested sources with one-line justification. Internal
Foundry docs do not count.

**§6 — `reference/` sub-routing proposal.** Proposed split of 55 articles.
For each article: target sub-bucket.

**§7 — Readability audit.** Five tables (Gemini diagnoses; does not rewrite):
- 7.1 Lede defects: `| slug | word count | missing sentence (1–4) | diagnosis |`
- 7.2 Label-only headers: `| slug | header text | proposed descriptive rewrite |`
- 7.3 Passive-voice density: `| slug | passive-clause count | top 3 offending sentences |`
  Flag threshold: > 3 passive clauses per 500 words.
- 7.4 GUIDE structure gaps: `| guide path | missing prereq? | missing outcome? | prose-command count |`
- 7.5 Reference orphans: `| slug | citations only in trailer? | claims needing inline attribution |`

**§8 — Article length audit.** Flag outliers:
- Split candidates: TOPICs > 2,500 words → `| slug | word count | proposed split point |`
- Stub candidates: TOPICs < 400 words → `| slug | word count | expand or merge? |`

**§9 — Domain map.** One row per committed TOPIC and GUIDE:
`| artefact_path | title | type | primary_domain | secondary_domain | wiki |`
(see §15.2 for full specification). Commit as `domain-map.tsv` alongside this file.

### 8.4 Light editorial work (Gemini executes after writing analysis)

Use `~/Foundry/bin/commit-as-next.sh`. Add files by name only.

1. **Broken-link repairs from §7.1 baseline** (rows with non-empty
   `suggested_replacement`). Update all wikilinks in-place.
   Commit: `chore(links): repair pre-overhaul broken wikilinks from baseline audit`

2. **Resolve duplicates** (§5.1). `git rm` both `.es.md` pairs.
   After removal, run §7.2 for each removed slug to verify no callers remain.
   Commit: `chore(catalogue): resolve duplicate file locations — patterns is canonical`

3. **Move misrouted GUIDE** (§5.2). Two commits (one per repo).
   Run §7.2 after removal.
   Commits: `chore(catalogue): remove misrouted guide from infrastructure/`
   and `guide: move totebox-orchestration-gis to gateway-orchestration-gis-1`

4. **Normalise `systems/` filenames** (§5.3). Six renames + six `.es.md` pairs.
   After each rename, run §7.2 for the old slug.
   Commit: `refactor(systems): normalise filenames — drop topic- prefix`

5. **Rename `jennifer-datagraph-rebuild`** (§5.4). Rename + `.es.md`.
   Run §7.2 after. Update all callers in the same commit.
   Commit: `refactor(substrate): remove personal name from article filename`

6. **Wikilink injection — architecture, substrate, services only.**
   Convert bare concept references to `[[slug]]`. Fix existing references only;
   do not add new ones.
   Commit per category: `chore(<category>): convert bare concept references to wikilinks`

### 8.5 Phase 1 verification

- `overhaul-gemini-analysis.md` exists with all nine sections (§1–§9)
- `domain-map.tsv` exists; every TOPIC and GUIDE has a row; no `primary_domain` blank
- `vocabulary-baseline.tsv` exists with `in_documentation`, `in_corporate`, `in_projects` columns
- All six light-work commits landed
- `git status` clean in all sub-clones
- `grep -ri "jennifer" content-wiki-documentation/*/*.md` → 0 hits in body/filenames
- All six `topic-` prefixed systems slugs gone from filenames and wikilinks
- Re-run §7.1 → confirm broken count equals 0 or matches expected stubs only

Gemini then writes a handoff entry to `.agent/inbox.md` confirming Phase 2
is unblocked, listing the commit SHAs for the six light-work items.

The inbox message must use the subject line `re: Phase 1 complete — Phase 2 gate open`
and list the results of each Phase 1 → Phase 2 gate check (§14.2). It sets
`owner_engine` in `overhaul-progress.md` to `<none>` and `status` to `gate-open`
before closing the session.

---

## 9. Phase 2 — Claude Code: Full overhaul

### 9.1 Session start sequence

1. Read `.agent/inbox.md` and current NOTAM
2. Read `.agent/rules/design-tokens.md` and `.agent/rules/handoffs-outbound.md`
3. Read `.agent/plans/README.md`
4. Read `.agent/session-start.md`
5. Read this file in full
6. Read `.agent/plans/overhaul-progress.md` (see §14.1 for schema).
7. Run the mid-sub-phase recovery procedure (§14.3) end-to-end. No exceptions.
8. Verify the Phase 1 → Phase 2 gate (§14.2). If any gate item fails, post `gate-rejected` to outbox and STOP — do not begin any editorial work.
9. Read `.agent/plans/overhaul-gemini-analysis.md` in full.
10. Read `.agent/plans/vocabulary-baseline.tsv` — this is the term-intersection reference for every TOPIC touched in Phase 2.
11. Re-run §7.1 broken-link script; confirm broken count equals 0 (or only named expected stubs).

**Scope fence (mandatory):** Before any commit, check the per-item tracker in `overhaul-progress.md` — refuse to act on any item already in state `committed`. Update `owner_engine` to `claude-code` and flip `status` to `in-progress` before the first commit of each resumed session.

### 9.2 Execution sequence (strict order — do not parallelise sub-phases)

**Sub-phase 2a — ZIP language pass + bilingual production (27 drafts)**

For each ZIP draft:
1. Read the draft in full
2. Consult ZIP-to-TOPIC mapping (analysis §2)
3. Apply language pass (§3 register, §4 flag resolutions)
4. Apply readability rules (§6): lede formula, descriptive headers, inline references
5. Remove `notes_for_editor:` block; downgrade `bcsc_class`
6. Replace personal names with role nouns; remove governance vocabulary
7. If `merge-into:<slug>`: integrate into target TOPIC; update `last_edited`; re-produce `.es.md`
8. If `new:<slug>,<category>`: write to target category; produce `.es.md`
9. Run §7.2 for any slug touched; output must be empty
10. Commit bilingual pair: `topic(<category>): <slug> — <title>`

One ZIP draft = one commit. Never batch. `git status` clean before next.

**Sub-phase 2b — `reference/` sub-routing**

Execute analysis §6. `git mv` EN + ES pairs to sub-buckets. Update wikilinks.
Run §7.2 for each moved slug. Commit per sub-bucket:
`refactor(reference): move <bucket> articles into reference/<sub>/`

**Sub-phase 2c — Catalogue actions (retire / consolidate / split / move)**

Execute every non-`keep` action from analysis §1. One commit per action.
Run §7.2 after each removal or rename. Patterns:
- `retire`: `git rm` EN + ES + wikilink cleanup in same commit
- `consolidate-with:<slug>`: merge prose, `git rm` source, update wikilinks
- `split-into:<slug-a>,<slug-b>`: create two TOPICs, `git rm` original, update wikilinks
- `move-category:<target>`: `git mv`, update frontmatter `category:` + wikilinks

Commit: `refactor(catalogue): <action> <slug> — <reason>`

**Sub-phase 2d — Reference additions**

For every TOPIC with fewer than two external references, add analysis §5
suggestions: inline at point of claim + expansion in `## References`.
Commit per category: `docs(<category>): add external references across topics`

**Sub-phase 2e — Cross-reference gap fill**

Execute analysis §4. Convert bare references to `[[wikilink]]`. Add inbound
links for orphan TOPICs from recommended sources.
Commit per category: `docs(<category>): fill wikilink gaps and link orphan articles`

**Sub-phase 2f — Code-block normalisation in GUIDEs**

Every prose-embedded shell command, config snippet, or file fragment → fenced
code block with language tag (`bash`, `toml`, `yaml`, `json`, `rust`, `sql`,
`nginx`, `systemd`, etc.). Formatting only; no semantic changes.
Commit per cluster: `docs(guide): fenced code blocks across <cluster> guides`

**Sub-phase 2g — Readability pass (§6 applied corpus-wide)**

Process TOPICs ordered by inbound-link count (highest first — maximises reader
impact per article). For each TOPIC:
1. Rewrite lede to four-sentence formula (§6.1), 60–90 words
2. Rewrite label-only headers (§6.2) — use analysis §7.2 worklist
3. Convert passive-voice clauses (analysis §7.3 worklist)
4. Integrate trailing citations inline (analysis §7.5 worklist)
5. Compare result against Gemini's §7 row — if Claude disagrees with a flag,
   log the disagreement (slug + reason) at the end of `overhaul-gemini-analysis.md`
6. Re-produce `.es.md` pair to match
7. Run §7.2 for any slug whose wikilinks changed

Commit: `readability(<category>): <slug> — lede + headers + register`

For GUIDEs (from §7.4 worklist): add prerequisite + outcome block (§6.7) and
convert steps to four-part structure (§6.8).
Commit per GUIDE: `readability(guide): <guide-path> — step structure + prereq/outcome`

**Sub-phase 2h — `reference/` style guide update**

Update `reference/style-guide-topic.md` and `reference/style-guide-guide.md` to
embed the readability rules from §6 with worked examples. These become the
living in-wiki standard for future sessions. Produce `.es.md` pairs.
Commit: `docs(reference): embed readability standard into style-guide topics`

**Sub-phase 2i — Main page and category landing rewrite (last)**

After all content has settled:
1. Rewrite `index.md` per analysis §3 root + `knowledge-wiki-home-page-design` pattern.
   Produce `.es.md` pair.
2. Rewrite each of the ten `<category>/_index.md` files per analysis §3 per-category
   recommendations. Produce `.es.md` pairs.
3. Update `featured-topic.yaml`.

Commits:
- `home: rewrite root index for catalogue overhaul`
- One commit per category landing: `home(<category>): rewrite _index.md`
- `home: update featured-topic.yaml`

After each sub-phase completes: `~/Foundry/bin/promote.sh` from inside the
affected sub-clone.

### 9.3 Commit discipline

- Tool: `~/Foundry/bin/commit-as-next.sh "<message>"` inside sub-clone
- One logical unit per commit; bilingual pairs in one commit
- Never `git add -A` or `git add .`; add files by name only
- `git status` clean after each commit before the next
- After each sub-phase: promote to canonical via `promote.sh`

### 9.4 Phase 2 verification (quality metrics)

Verify all metrics from §1 before declaring Phase 2 complete:
- Re-run §7.1 → broken internal wikilink count = 0
- `grep -r "bcsc_class: internal" content-wiki-documentation/` → 0
- `grep -ri "jennifer\|mathew\|peter" content-wiki-documentation/*/*.md` → 0 body/filename hits
- `grep -r "Doctrine claim\|conventions/" content-wiki-documentation/` → 0 body hits
- Every TOPIC: ≥ 2 external references, ≥ 3 `[[wikilink]]` instances in body
- Every GUIDE with commands: all in fenced code blocks
- Root `index.md` and all ten `_index.md` files rewritten
- `reference/style-guide-topic.md` and `reference/style-guide-guide.md` version matches §6; both have `.es.md` pairs
- `domains-development-proposal.md` staged in `.agent/drafts-outbound/`; `handoffs-outbound.md` entry added for `pointsav-monorepo/service-content/`
- Run §7.3 external URL audit; file any dead URLs in cleanup-log.md

Claude writes final handoff to `.agent/inbox.md`. Sets `status: complete` in this
file. Moves this file to `.agent/plans/archive/overhaul-documentation-pointsav-com.md`.

---

## 10. Session mechanics reference

| Item | Value |
|---|---|
| Working directory | `/srv/foundry/clones/project-editorial/` |
| Content sub-clone | `content-wiki-documentation/` |
| Fleet sub-clone (Woodfine) | `woodfine-fleet-deployment/` |
| Fleet sub-clone (PointSav) | `pointsav-fleet-deployment/` |
| Commit tool | `~/Foundry/bin/commit-as-next.sh "<message>"` |
| Promotion tool | `~/Foundry/bin/promote.sh` (run inside sub-clone) |
| Identity at next commit | Jennifer Woodfine |
| Wiki URL | https://documentation.pointsav.com |
| Phase 0 output | `.agent/plans/vocabulary-baseline.tsv` (three-wiki term index) |
| Phase 1 output | `.agent/plans/overhaul-gemini-analysis.md` (9 sections) + `.agent/plans/domain-map.tsv` |
| Progress tracker | `.agent/plans/overhaul-progress.md` (see §14.1) |
| Broken-link baseline | `audit/baseline-broken-links.tsv` (project-editorial root) |
| ZIP draft location | `.agent/drafts-outbound/zip-topic-*.md` |
| Glossary (wiki) | `content-wiki-documentation/reference/glossary-documentation.md` |
| Glossary (runtime) | `pointsav-monorepo/service-content/seeds/Domains.json` + `ontology/*.csv` |
| Framework reference | `/srv/foundry/.agent/plans/framework-pointsav-products-services.md` |

Session start ritual (both engines): inbox → NOTAM → rules → plans README →
session-start → this file → `overhaul-progress.md` → recovery check (§14.3).

---

## 11. Phase summary

| Phase | Engine | Output (analytical) | Output (commits) |
|---|---|---|---|
| 0 | Gemini CLI | `vocabulary-baseline.tsv` (3-wiki term index, ~300+ rows) | 1 commit (vocabulary baseline) |
| 1 | Gemini CLI | `overhaul-gemini-analysis.md` (9 sections) + `domain-map.tsv` + broken-link baseline | ~6 light-work commits |
| 2 | Claude Code | Inbox handoff + quality-metric verification + `domains-development-proposal.md` | ~90–130 commits across three sub-clones |

---

## 12. Stop conditions

Stop and write to `.agent/inbox.md` before continuing if any of these occur:

- A flag resolution in §4 does not cover an encountered case
- A ZIP draft contains BCSC-sensitive material beyond §4 (revenue figures,
  customer names, dated forward statements without hedging)
- A catalogue action would orphan more than five inbound wikilinks without
  a clean redirect target
- §7.1 baseline broken-link audit returns > 20 unexplained broken links
  (indicates a structural issue requiring operator review)
- A commit hook fails for a reason not resolved by a corrective new commit
- A third duplicate or misrouted file is discovered not listed in §5

Do not invent a resolution. Write the situation; stop.

---

## 13. Vocabulary baseline and glossary self-heal

`service-content` (the Gravity Engine) runs a deterministic Aho-Corasick keyword
scan against every payload. The vocabulary that drives this scan lives in two
machine-readable surfaces:

- `pointsav-monorepo/service-content/seeds/Domains.json` — `gravity_keywords` arrays
- `pointsav-monorepo/service-content/ontology/*.csv` — first column, lowercased, is the term

The public wiki carries a third surface:
- `content-wiki-documentation/reference/glossary-documentation.md` — ~280 A–Z entries

These three surfaces are **not currently synchronised**. The overhaul uses them
together to anchor vocabulary consistency across all 257+ TOPICs and to propose
runtime-side improvements without violating SYS-ADR-07 (no structured data through AI)
or SYS-ADR-19 (no automated publishing to verified ledgers).

### 13.1 Phase 0 — vocabulary baseline extraction (Gemini runs first)

Extract a `vocabulary-baseline.tsv` before any editorial work begins.
Columns: `term | definition | wiki_slug | source | glossary_status | bilingual_status`

`source` values: `runtime` | `wiki` | `both`
`glossary_status` values: `canonical` (in both surfaces) | `wiki-only` | `wanted` (used in TOPIC but undefined)
`bilingual_status` values: `en` | `en+es`

Extraction commands (read-only, no AI authorship of structured data):

```bash
# Runtime terms from Domains.json
jq -r '.domains[].gravity_keywords[]' \
  /srv/foundry/clones/project-editorial/pointsav-monorepo/service-content/seeds/Domains.json

# Runtime terms from ontology CSVs
awk -F, 'NR>1 {print $1}' \
  /srv/foundry/clones/project-editorial/pointsav-monorepo/service-content/ontology/*.csv | sort -u

# Wiki glossary headings (full entries)
grep -E '^### ' \
  /srv/foundry/clones/project-editorial/content-wiki-documentation/reference/glossary-documentation.md

# Wiki glossary stubs
grep -E '^- \*\*' \
  /srv/foundry/clones/project-editorial/content-wiki-documentation/reference/glossary-documentation.md
```

Merge all four lists; deduplicate (case-insensitive); assign `source` and
`glossary_status` per term. Known defects to fix in the same pass:
- Line ~198: truncated `service-search` entry — mark for stub repair
- Lines ~344–352: duplicate `OrchestrationOS` heading — mark for dedup
- ~40 "Standard IT infrastructure term…" boilerplate stubs — mark `status: stub`

Commit: `plans: add vocabulary baseline — Phase 0`
Path: `.agent/plans/vocabulary-baseline.tsv`

### 13.2 Per-TOPIC vocabulary check (Phase 2 — runs once per TOPIC before editing)

Before refining any TOPIC, cross-check its noun phrases against `vocabulary-baseline.tsv`.
For each term found in the TOPIC body:
- `canonical`: no action needed; ensure the term's first occurrence is a `[[wikilink]]`
- `wiki-only`: no action needed for this TOPIC; flag in baseline as "used in corpus"
- `wanted` (term appears but neither surface defines it): write a one-line stub in
  `reference/glossary-documentation.md` with `status: stub`, commit alongside the TOPIC

Do NOT coin a definition inline in the TOPIC body — stubs go in the glossary article.
Do NOT write directly to `seeds/Domains.json` or `ontology/*.csv` — see §13.3.

### 13.3 Runtime-side feedback (gated — not in Phase 2 execution scope)

Any `wiki-only` or `wanted` terms that should propagate to the runtime Domains.json
or ontology CSVs are staged as a proposal only:

1. Append proposed additions to `.agent/drafts-outbound/glossary-runtime-patch.md`
   with `foundry-draft-v1` frontmatter, one row per term.
2. Add a `handoffs-outbound.md` entry (state `pending-destination-commit`,
   destination repo `pointsav-monorepo/service-content/`).
3. The runtime write happens in a separate human-tier Command Session that edits
   `seeds/Domains.json` and `ontology/*.csv` directly, gated by SYS-ADR-07 and SYS-ADR-10.

**This outbox handoff is a Phase 2 deliverable** — it does not block Phase 2 completion,
but it must be written before Claude declares Phase 2 done.

### 13.4 Glossary self-heal (Phase 2 — woven into sub-phase 2g)

During the readability pass, also repair the three known structural defects in
`reference/glossary-documentation.md`:

1. Fix the truncated `service-search` entry (~line 198)
2. Remove the duplicate `OrchestrationOS` heading block (~lines 344–352)
3. Expand ~40 boilerplate stubs: replace "Standard IT infrastructure term,
   re-contextualised…" with a one-sentence definition using the register from §3

Each repair is part of the sub-phase 2g commit for the `reference/` category.
Produce the `.es.md` pair for `glossary-documentation.md` in the same commit.

**Add to Phase 2 quality metrics:**
- `grep -c "Standard IT infrastructure term" reference/glossary-documentation.md` → 0
- Duplicate heading check: `grep -E "^### OrchestrationOS" reference/glossary-documentation.md` → 1 line only

---

## 14. Multi-session handoff discipline

The overhaul spans multiple days and two engines. Any session may be interrupted
mid-work. This section defines the state file, gate protocol, and recovery
procedure that prevent chaos and duplicate work.

### 14.1 `overhaul-progress.md` schema

Location: `.agent/plans/overhaul-progress.md`
Initialised by: the Claude session that writes the master plan (Step 2 of execution).
Updated by: every engine at session start (claim `owner_engine`) and session end
(set `status` + clear `owner_engine`).

```yaml
---
schema: overhaul-progress-v1
plan: overhaul-documentation-pointsav-com.md
phase: 0                      # 0=vocabulary-baseline 1=gemini-light-work 2=claude-execution
sub_phase: 0-vocabulary-baseline  # exact label from master plan
status: in-progress           # in-progress | clean-checkpoint | blocked | gate-open | complete
safe_to_resume: true          # false ONLY if dirty-checkpoint or active blocker
unsafe_reason: ""             # required when safe_to_resume=false
owner_engine: ""              # gemini-cli | claude-code | "" (empty = no active session)
last_updated: 2026-05-14T00:00:00Z
last_session_id: ""           # <boot_id>-<pid> — written at session start
---

## Last completed sub-task
- task: (none)
- commit_sha: ""
- committed_at: ""

## Next pending sub-task
- task: 0-vocabulary-baseline — extract vocabulary-baseline.tsv
- inputs: seeds/Domains.json, ontology/*.csv, reference/glossary-documentation.md
- expected_commit_msg_prefix: "plans: add vocabulary baseline"

## Blockers
- (none)

## Per-item tracker — sub-phase 2a (27 ZIP drafts)
| id | slug                              | state   | sha | notes |
|----|-----------------------------------|---------|-----|-------|
| 01 | zip-topic-app-console-input-f12   | pending | —   | |
| 02 | zip-topic-archetypes-and-coa      | pending | —   | |
| 03 | zip-topic-bim-product-family      | pending | —   | |
| 04 | zip-topic-competitive-positioning | pending | —   | |
| 05 | zip-topic-compliance-disclosure   | pending | —   | |
| 06 | zip-topic-deployment-patterns     | pending | —   | |
| 07 | zip-topic-design-system           | pending | —   | |
| 08 | zip-topic-hardware-research       | pending | —   | |
| 09 | zip-topic-leapfrog-2030           | pending | —   | |
| 10 | zip-topic-legal-ip-structure      | pending | —   | |
| 11 | zip-topic-machine-based-authorization | pending | — | |
| 12 | zip-topic-microkernel-substrate   | pending | —   | |
| 13 | zip-topic-os-console              | pending | —   | |
| 14 | zip-topic-os-family-overview      | pending | —   | |
| 15 | zip-topic-os-infrastructure-network | pending | — | |
| 16 | zip-topic-os-mediakit             | pending | —   | |
| 17 | zip-topic-os-orchestration        | pending | —   | |
| 18 | zip-topic-os-totebox              | pending | —   | |
| 19 | zip-topic-os-workplace            | pending | —   | |
| 20 | zip-topic-pointsav-overview       | pending | —   | |
| 21 | zip-topic-service-content         | pending | —   | |
| 22 | zip-topic-service-email-people    | pending | —   | |
| 23 | zip-topic-service-slm             | pending | —   | |
| 24 | zip-topic-six-tier-sovereignty-matrix | pending | — | |
| 25 | zip-topic-supply-chain-governance | pending | —   | |
| 26 | zip-topic-the-diode-standard      | pending | —   | |
| 27 | zip-topic-three-layer-architecture | pending | —  | |
```

Item states: `pending` | `in-flight` | `committed` | `flagged` | `skipped`.
`in-flight` is forbidden in a clean checkpoint — a session must either commit or
revert before closing.

### 14.2 Phase 1 → Phase 2 gate checklist

Gemini must verify ALL nine items before writing the gate-open inbox message.
Claude must re-verify ALL nine items before beginning any Phase 2 work.

| # | Check | Command / verification |
|---|---|---|
| 1 | `overhaul-gemini-analysis.md` exists with all 8 section headings | `grep -c "^## " .agent/plans/overhaul-gemini-analysis.md` → 8 |
| 2 | `vocabulary-baseline.tsv` exists | `test -f .agent/plans/vocabulary-baseline.tsv` |
| 3 | `git status` clean in project-editorial root | `git status --porcelain` → empty |
| 4 | All 6 Phase-1 light-work commits present on cluster branch | `git log --oneline | head -20` — verify 6 `overhaul-1:` or matching prefix commits |
| 5 | `overhaul-progress.md` shows `status: gate-open` or Gemini wrote a gate-open inbox message | Read progress file AND inbox |
| 6 | No `in-flight` rows in per-item tracker | `grep "in-flight" .agent/plans/overhaul-progress.md` → no output |
| 7 | No active blockers | Blockers section in progress file shows "(none)" |
| 8 | Phase 1 commits promoted (staging mirrors match) | `git ls-remote origin-staging-j cluster/project-editorial` HEAD matches local HEAD |
| 9 | No Gemini session lock active (stale or absent) | `.agent/engines/gemini-cli/session.lock` absent or `boot_id` differs from `cat /proc/sys/kernel/random/boot_id` |

If any check fails: Claude posts `gate-rejected` message to outbox with failing item(s)
and STOPS. Gemini (or operator) resolves the failing check; Claude re-verifies on next session.

### 14.3 Mid-sub-phase recovery procedure

Run by every incoming session (both engines) as step 7 of their start sequence.

1. Read `overhaul-progress.md`. If `safe_to_resume: false`, STOP and surface `unsafe_reason` to operator — do not attempt to resume.
2. Verify `owner_engine` is empty or matches the current engine. Non-empty foreign owner = STOP (concurrent-session guard).
3. Run `git status` in archive root and each sub-clone. Any tracked modifications or staged-but-uncommitted files = **dirty checkpoint**. Recovery: read last committed SHA from tracker; diff working tree against it; either finish-and-commit the in-flight item OR `git restore` to last committed SHA and mark item `pending`.
4. Check `.agent/engines/*/session.lock` for the other engine. If lock is present with a live PID (`kill -0 <pid>` succeeds) and matching `boot_id` (`cat /proc/sys/kernel/random/boot_id`), STOP — concurrent session conflict. If `boot_id` differs (VM rebooted), remove the stale lock automatically and proceed.
5. Verify the "Next pending sub-task" input file(s) exist at the paths named. If missing, set `status: blocked` with `unsafe_reason: input-missing` and STOP.
6. Count tracker rows: `committed + pending + flagged + skipped` must equal the sub-phase total (27 for 2a). Mismatch = corrupt tracker — STOP and surface to operator.
7. All 6 checks pass: write session lock, set `owner_engine` to self, update `last_session_id`, update `last_updated`. Resume from "Next pending sub-task" exactly as named.

**Clean checkpoint** = `git status` clean + no `in-flight` rows + `safe_to_resume: true`.
Sessions MUST exit clean. Set `owner_engine: ""` and `status: clean-checkpoint` before closing.

### 14.4 Failure-mode guard table

| Failure mode | Guard | Detection | Recovery |
|---|---|---|---|
| Gemini starts Phase 2 work | Start sequence step 8 (phase pairing); Phase 1 scope fence lists only 6 commit types | `git log --grep="overhaul-2"` on Gemini-authored commits returns rows | `git revert` offending commit; re-promote; post NOTAM; flag operator |
| Claude starts Phase 2 before gate | Start sequence step 8 requires gate-open status + inbox message; Claude refuses otherwise | `overhaul-gemini-analysis.md` missing or `status: in-progress` when Claude active | Stop; post `gate-rejected` to outbox; await Gemini completion |
| Two sessions on same sub-clone | Recovery step 4 (live PID + boot_id check); `owner_engine` field second guard | Foreign lock with live PID or `owner_engine` mismatch | Newer session aborts; if stale lock (boot_id mismatch) auto-remove and proceed |
| Sub-phase marked complete but promote skipped | Gate item 8 verifies remote matches local before `status: complete` | Gate check fails; tracker shows `committed` but remote SHA lags | Run `bin/promote.sh`; do not flip `status: complete` until remote matches |
| ZIP draft committed twice | Per-item tracker single source of truth; both engines refuse `committed` rows; commit prefix `overhaul-1:` vs `overhaul-2a:` makes duplicates greppable | `git log --oneline \| grep zip-topic-NN` returns > 1 row | Operator-approved `git revert` of duplicate; update tracker row; post NOTAM |

---

## 15. Domains development, TOPIC+GUIDE map, and living style guide

### 15.1 Domains across all three wikis

`service-content`'s Gravity Engine classifies content against Domains drawn from
`seeds/Domains.json` and `ontology/*.csv`. The current runtime vocabulary covers
`documentation.pointsav.com` content in general terms, but the three wiki contexts
have distinct vocabularies that should each be reflected in the Domain substrate:

| Wiki | Context | Canonical repo |
|---|---|---|
| `documentation.pointsav.com` | Technical product documentation | `content-wiki-documentation/` |
| `corporate.woodfinegroup.com` | Financial, governance, statutory | `content-wiki-corporate/` |
| `projects.woodfinegroup.com` | Project-specific, BIM, GIS | `content-wiki-projects/` |

**Phase 0 extension:** the vocabulary-baseline extraction (§13.1) runs against all
three wiki content repos, not just `content-wiki-documentation/`. Add three columns
to `vocabulary-baseline.tsv`: `in_documentation` | `in_corporate` | `in_projects`
(boolean — whether the term appears in each wiki corpus).

The resulting three-way intersection table shows:
- Terms unique to documentation → candidates for a `documentation` Domain
- Terms unique to corporate → candidates for a `corporate` Domain  
- Terms unique to projects → candidates for a `projects` or `bim-gis` Domain
- Terms in all three → core platform vocabulary (highest-priority runtime terms)

**Runtime proposal deliverable (Phase 2, §13.3 extension):**
Add a `domains-development-proposal.md` to `.agent/drafts-outbound/` alongside
the existing `glossary-runtime-patch.md`. Structure:

```
## Proposed new Domains

### documentation-platform (documentation.pointsav.com)
gravity_keywords: [list from three-way intersection, documentation-unique terms]

### corporate-governance (corporate.woodfinegroup.com)
gravity_keywords: [corporate-unique terms]

### project-bim-gis (projects.woodfinegroup.com)
gravity_keywords: [projects-unique terms]
```

This draft is staged and handed off via `handoffs-outbound.md` to the
`pointsav-monorepo/service-content/` maintainer. The actual write to
`seeds/Domains.json` is human-gated per SYS-ADR-07.

### 15.2 TOPIC + GUIDE → Domain map

A navigability artefact: a single flat table mapping every committed TOPIC and
GUIDE to its primary Domain. Produced by Gemini in Phase 1 as an additional
section of `overhaul-gemini-analysis.md` (call it **§9 — Domain map**).

Format:
```
| artefact_path | title | type | primary_domain | secondary_domain | wiki |
```

`type`: `TOPIC` | `GUIDE`
`primary_domain`: the single Domain that best describes this artefact's subject
`secondary_domain`: optional second Domain (blank if not applicable)
`wiki`: `documentation` | `corporate` | `projects`

**This map serves three uses:**
1. Navigation: the main page and category landings use Domain clusters as an
   alternative browse path (future feature — out of scope for this overhaul,
   but the map is the prerequisite)
2. Completeness audit: Domains with fewer than 3 TOPICs flagged as under-served
3. Gemini's catalogue analysis (§8.3 §1) uses Domain assignment as a signal for
   `move-category` and `consolidate-with` recommendations

The map is committed to `.agent/plans/` alongside `overhaul-gemini-analysis.md`
as `domain-map.tsv`. Phase 2 Claude reads it before any catalogue action.

**Add to Gemini Phase 1 deliverables (§8.3):** produce §9 domain map as described.
**Add to Phase 1 verification (§8.5):** `domain-map.tsv` exists; every committed
TOPIC and GUIDE has a row; no `primary_domain` cell is blank.

### 15.3 Living style guide — iterations, not one-offs

The style guide for this wiki is not a single commit (sub-phase 2h). It is a
compounding standard that every future session reads first and improves on finding
gaps. The overhaul establishes v1.0; subsequent sessions increment the minor
version whenever a new rule is added or an existing rule is clarified by a
real-world example encountered in the corpus.

**Two canonical locations — both must stay in sync:**

1. **In-wiki**: `content-wiki-documentation/reference/style-guide-topic.md` and
   `reference/style-guide-guide.md` — public, bilingual (EN + ES). These are the
   reader-facing standard. They are versioned in frontmatter (`version: 1.0`).

2. **In-plan**: §6 of this document (`overhaul-documentation-pointsav-com.md`) —
   engine-facing, English-only. When a rule is added in §6, sub-phase 2h
   must propagate it to both wiki articles in the same session.

**Version discipline:**
- PATCH (1.0 → 1.1): clarify an existing rule with a worked example
- MINOR (1.0 → 2.0): add a new checkable rule
- MAJOR: restructure (unlikely; coordinate with operator)

Each version bump: update `version:` in TOPIC frontmatter, update `last_edited:`,
produce `.es.md` pair. Commit: `docs(reference): style-guide v<N> — <rule added/clarified>`

**Style guide integration into quality metrics (add to §9.4):**
- `grep "version:" content-wiki-documentation/reference/style-guide-topic.md` and
  `style-guide-guide.md` → must match §6 version number exactly
- Both style-guide articles have an `.es.md` pair

**Future sessions:** any session that encounters a TOPIC or GUIDE that cannot be
improved with the existing §6 rules should propose a new rule to the operator
before inventing ad-hoc language. The proposal goes to `.agent/inbox.md` as a
`re: style-guide rule proposal — <topic>` message. Operator-approved rules are
added to §6 and propagated to the wiki articles.

