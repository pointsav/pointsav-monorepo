# Editorial Reference Plan — DataGraph ↔ Content Reconciliation
## project-editorial Task — 2026-05-08

> **Purpose:** Standing reference for all future TOPIC and GUIDE work.
> **This is the canonical resume point** — auto-mode reads this on session
> start to know where to pick up. Read before any editorial session.
>
> Update the "Actual outcomes" lines at the end of each step as work
> completes; update the "Next session pickup" section at session close.
>
> Companion artifact (now in `.agent/archive/`): `datagraph-content-reconciliation-2026-05-07.md`

---

## Next session pickup

**Last session closed:** 2026-05-09T03:30Z. Auto-mode session #6 shipped
the **ship-now subset of Plan #7** (Wikipedia Main Page institutional
adaptation). 3 commits, all on staging mirrors, awaiting Master Stage 6
promote: corporate `c65be14`, projects `1c1e48b`, documentation `c4d1fb1`.

**Plan #7 ship-now subset — what landed:**
- All 3 wikis: refreshed `featured-topic.yaml` with rotation pool comments
  and banker-grade pin notes (5-week corporate rotation; 17-article
  quarterly-cadence projects rotation; 12-article documentation rotation).
- Corporate + projects gain `leapfrog-facts.yaml` for the first time
  (Did You Know panel renders the moment YAML lands canonical).
- Documentation `leapfrog-facts.yaml` refreshed from generic "leapfrog
  inventions" to 7 banker Structural Facts; Doctrine/Convention vocabulary
  scrubbed.
- 3 article titles scrubbed of workspace-internal governance vocabulary
  (`Foundry Doctrine — Architectural Overview` → `Foundry — Architectural
  Overview`; `The Sovereign Airlock Doctrine` → `The Sovereign Airlock`;
  `AEC Muscle Memory and Interface Conventions` → `AEC Muscle Memory and
  Interface Patterns`); slugs retained per content-contract §3.
- 3 PascalCase OS titles normalised to Title Case + space
  (`OrchestrationOS` → `Orchestration OS`, etc.).
- 2 projects tier-index titles reordered for consistency (`European
  Co-location Tier Index` → `Co-location Tier Index: Europe`).
- New cleanup-log Open entry surfacing the body-level Doctrine (72 EN
  files) + Convention (49 EN files) scrub as tracked work, phased over
  multiple sessions.

**Resume in priority order — Plan #7 deferred phases (full plan at
`/home/mathew/.claude/plans/you-are-task-agetn-robust-puddle.md`):**

1. **Phase D — Update `patterns/knowledge-wiki-home-page-design.md` +
   `.es.md`.** Replace the current 5-slot summary with the 10-slot
   Wikipedia structural skeleton + 3 per-wiki slot tables + YAML schemas
   + "10 categories" correction + institutional-banker reader contract +
   operator constraint #3 (no Doctrine/Convention vocabulary). Cheap,
   internal documentation; sets the spec project-knowledge implements.
   ~45 minutes.

2. **Phase E — Outbox to project-knowledge with engine spec.** Single
   message detailing the 5 new chrome slots (In The News, On This Day,
   Featured Spotlight per-wiki variant, Sister Wikis, multi-tenant Other
   Areas). Schema for each new YAML file. Pointers to Phase C drafts.
   Cheap; surfaces engine work. ~15 minutes.

3. **Phase C — Author 15 engine-pending YAML drafts** (5 per wiki × 3,
   except `on-this-day.yaml` only for projects per operator constraint).
   No engine consumer today; ships as content-only drafts that
   project-knowledge consumes when engine slots land. ~45 minutes.

4. **Phase A2 follow-up — Body-level Doctrine/Convention scrub** across
   ~120 documentation-wiki files. Phased: first sweep the 12 banker-relevant
   Featured-rotation articles (highest editorial value, narrow scope);
   subsequent sweeps work outward by category. Per-file editorial decision
   required; not a mechanical find-replace. Multi-session work tracked
   in `content-wiki-documentation/.agent/rules/cleanup-log.md`.

5. **Step 4 — CSV structural cleanup.** **Blocked**, not project-editorial
   scope. Routes to `project-intelligence` or `project-data` Task.

**Open reminders:**
- 13 of 17 inbox-referenced drafts in 2026-05-08 cluster batches were
  already published; sibling Tasks (`project-design`, `project-intelligence`)
  are emitting drafts at session shutdown without checking destination
  state. Courtesy outbox messages sent; pattern flagged for Master at
  workspace level.
- Plan #7 slot labels are first-pass proposals; operator may want to
  adjust before Phase C YAML authoring (labels are data per the
  `slot_label:` field; cheap to change later).

---

## State as of 2026-05-08 — what has been done

### Phase 1 complete — reconciliation report produced

The DataGraph taxonomy (15 CSV seed entries) was cross-referenced against
the real-world wiki output (~252 EN articles). Full findings are in the
reconciliation artifact. Key numbers:

| Dimension | Count |
|---|---|
| Topic CSV entries (all domains) | 15 |
| Wiki articles — documentation | ~213 EN |
| Wiki articles — projects | ~34 EN |
| Wiki articles — corporate | ~5 EN |
| Total EN articles | ~252 |
| CSV entries matched to published articles | 1 (doorman-protocol) |
| CSV entries with no published article ("wanted") | 14 |
| Published articles with no CSV entry ("unclassified") | ~251 |

### Structural bugs found in the CSVs (not yet fixed)

1. **`wiki_repo` bug** — corporate and projects CSV entries point at
   `content-wiki-documentation` instead of their correct wikis.
2. **`wiki_path` format** — all entries use stale `topics/topic-*.md`
   format; correct format is `<category>/<slug>.md`.
3. **`guides_documentation.csv` does not exist** — convention
   `datagraph-guide-entity-class.md` was ratified 2026-05-07 but the
   CSV has not been created. ~72 GUIDEs are unregistered.

### What was published in this session

- `reference/climate-zone-tokens.md` + `.es.md` — converted from
  misrouted GUIDE draft; committed at `15d0942`
- All 6 BIM TOPICs from project-bim batch — pre-published, confirmed
- All GUIDE skeleton work for project-bim committed to cluster

### Stage 6 still pending (woodfine-fleet-deployment + content-wiki-documentation)

7 commits ahead on woodfine-fleet-deployment cluster.
Commits `ab0b709`, `8a4fd6c`, `15d0942` on content-wiki-documentation
need Stage 6 promotion. Flag to Master.

---

## The compounding architecture — why detail level is calibrated the way it is

The DataGraph is a living, improving system. The current state (v1) is a
simple first pass — 15 seed CSV entries, 252 articles unregistered, vocabulary
not clean. Two Yo-Yo compute passes will produce v2 and v3 automatically:

```
DataGraph v1 (now)
  — 15 seed CSV entries, ~252 articles unregistered

Yo-Yo #1 → DataGraph v2 (local GPU, nearly ready, runs daily)
  — refines entity extraction and classification
  — produces training tuples from editorial interactions

Yo-Yo #2 → DataGraph v3 (external API key, weekly/monthly)
  — frontier model quality; higher-quality refinement

Monthly content sweep (every cycle after this pass)
  — Yo-Yo generates new draft per article against updated DataGraph
  — editorial review: accept / refine / reject each draft
  — each verdict is a DPO training tuple → SLM improves
  — language tokens score for register drift
  — repeat; each month is easier and better than the last
```

**Calibration rule:** Do what makes the next Yo-Yo pass better, not what
makes this pass final. The goal for each article is "draft 2 of 10" —
good enough that the Yo-Yo produces a clearly better draft 3. Perfection
in pass 1 is wasted; the compounding mechanism handles improvement.

---

## Wikipedia as the model — learning environment, not documentation

All three wikis share one overarching purpose: they are encyclopedic learning
resources. Wikipedia is the model — not because of its specific content, but
because of what Wikipedia does for a reader:

- **It teaches, not just references.** After reading a Wikipedia article, the
  reader understands something. They came to look up a term; they leave with
  a mental model. Every article in the Foundry wikis should do the same.
- **It builds an interconnected web of understanding.** Articles link to each
  other. A reader who follows the links builds progressively deeper knowledge.
  Red links (wanted articles) are features — they show the reader where the
  encyclopedia is incomplete, and they invite contribution.
- **It is encyclopedic depth at accessible language.** Not shallow bullet
  points. Not impenetrable academic prose. Enough depth to be genuinely
  educational for a sophisticated reader — a banker, an architect, an
  engineer — without requiring prior knowledge of the platform's internal
  vocabulary.
- **It uses a consistent, recognizable structure.** Lead paragraph (most
  important facts, full context, consequence) → body sections (progressively
  deeper) → See also → References. This structure is Wikipedia's muscle
  memory — a reader who knows Wikipedia recognizes it immediately and knows
  how to navigate it.

**Applied to each wiki:**

- **Corporate wiki:** After reading `direct-hold-structures`, a banker
  understands *why* Woodfine uses this structure, *what* it means for
  capital allocation, and *how* it differs from pooled structures. Not just
  what the term means — the full logic, the capital consequence, the
  institutional context.
- **Projects wiki:** After reading `co-location-mandate`, a developer or
  architect understands the logic behind the mandate, the capital framework
  that validates it, the market conditions that make it viable. They could
  explain it to a colleague.
- **Documentation wiki:** After reading `service-slm`, an engineer understands
  the routing logic, why the tier thresholds are set the way they are, and
  what the consequence is if the service is unavailable. A next-generation
  institutional reader scanning the article understands that the platform
  manages AI costs automatically and never sends a request off-premises
  without the operator's knowledge.

**The Wikipedia learning environment is the frame. The three registers are
how you write within that frame for each audience.** Bloomberg/FT writes
encyclopedically for bankers. Stripe/Cloudflare writes encyclopedically for
engineers. The register changes; the encyclopedic purpose does not.

**What this means for the rewrite:** Every article should pass the test —
*does a reader who finishes this article understand the subject, or have they
just found a fact?* If they've only found a fact, the article is incomplete.
The DataGraph-enriched rewrite adds the relationships, context, and consequence
framing that turns a fact into understanding.

---

## Language register framework — corrected assignment (2026-05-08)

### Audience map

| Wiki | Primary audience | Register |
|---|---|---|
| `content-wiki-corporate` | Bankers, family offices, institutional investors | Bloomberg / FT / Economist |
| `content-wiki-projects` | Top-400 development markets, commercial architects — **same institutional readers, different subject** | Bloomberg / FT / Economist |
| `content-wiki-documentation` | Software engineers, designers; **also** next-generation institutional readers evaluating credibility | Stripe / Cloudflare primary + Corporate accessibility layer |
| `bim.woodfinegroup.com` | Architects, engineers, building code officials | RIBA / IFC specification |
| `gis.woodfinegroup.com` | GIS analysts, co-location programme managers | Technical specification |
| `design.pointsav.com` | Design system contributors | Design specification (DTCG) |

**Critical:** RIBA/IFC specification language (shall/shall not, measurements
with units, normative vs informative) belongs only on specialist sites.
It must not appear in the three main wikis.

### Specialist sites as cross-reference destinations

When a main wiki article references a topic requiring prescriptive
specification, link to the specialist site rather than including
specification content inline:

- *"For full climate zone token specifications and IFC mappings, see `bim.woodfinegroup.com`."*
- *"For co-location scoring methodology and zoning criteria, see `gis.woodfinegroup.com`."*
- *"For design token schemas and component specifications, see `design.pointsav.com`."*

### Register rules

**Register 1 — Bloomberg/FT (Corporate + Projects wikis)**
- Sentence length: 14–18 words target, 25 max
- Lead: consequence first — the most important fact for a capital allocator
- Voice: active. Passive reads as evasion.
- Numbers: always specific. "$7/month" not "low-cost."
- Jargon: translate everything on first use
- Code blocks: never
- Avoid: academic hedging, abstract nouns, Foundry-internal metaphors

**Register 2 — Stripe/Cloudflare + accessibility layer (Documentation wiki)**
- Structure: Concept → Why it matters (one institutional-reader sentence) → How it works → Code → Edge cases
- The "why it matters" sentence must stand alone for an institutional reader scanning headers
- Code blocks: real and runnable
- Foundry-specific terms: define once in plain language on first use, then use the term
- Avoid: over-explaining basics, vague architecture descriptions, incomplete examples

**Accessibility layer pattern (documentation wiki):**
The first sentence of each major section should be legible to an institutional
reader who is scanning — consequence-first, no jargon. The engineering reader
reads the whole article; the institutional reader reads section openers and code
captions. Both must work.

### Vocabulary retirement (applies across all three main wikis)

| Retire | Replace with |
|---|---|
| Substrate | the data layer / the platform code / the security foundation |
| Doctrine | architectural principle / design decision / engineering policy |
| Compounding | training signal aggregation / model improvement over time |
| Leapfrog | designed to exceed / targeted to replace / planned for [date] |
| Doorman | access-control gateway / AI request router |
| Ring 1 / Ring 2 / Ring 3 | archive tier / data tier / inference gateway |
| Totebox | property archive / data vault |
| Yo-Yo pool | on-demand GPU instances / ephemeral inference nodes |
| Sovereign (adjective) | independently verifiable / operator-controlled |
| Compounding Substrate | the platform (first use); link to architecture article |
| Apprenticeship Substrate | the learning pipeline / the training system |
| Mooncake | (retire entirely; use model-family name or "local inference model") |
| LadybugDB | property graph database / the DataGraph store |

**Rule:** Retired terms may appear in architecture/services TOPIC articles
where they are being defined. They must not appear in corporate, projects,
or GUIDE content without a plain-language translation immediately preceding.

---

## The two inputs to every rewrite — critical distinction

The rewrite of TOPICs and GUIDEs draws on two completely separate sources:

| Input | Source | What it provides |
|---|---|---|
| **HOW to write** | RESEARCH corpus → language tokens | Register, sentence structure, vocabulary, consequence-first lead |
| **WHAT to say** | Full DataGraph — entities, relationships, domains, themes | Content the articles were written before the DataGraph existed to supply |

The wiki articles were authored bottom-up from engineering work, before
the DataGraph was populated. The DataGraph now knows things the articles
don't express: what each entity connects to, which domain it belongs to,
what themes span it, and what the consequence is for an institutional
reader. The rewrite makes the articles express that accumulated knowledge.

**Example:**

Before (style-only): *"service-slm routes AI requests across compute tiers."*

After (DataGraph-enriched): *"service-slm routes every AI request to the
cheapest compute tier that meets the deadline — without the caller
specifying which tier. A request that resolves locally never leaves the
customer's infrastructure and never appears on a cloud billing statement.
The routing logic, tier thresholds, and audit log are all operator-controlled."*

The second version draws on DataGraph knowledge: the three-tier architecture,
the consequence for the institutional reader, the sovereignty framing.
That is content, not style.

**Why this matters for Yo-Yo training:** If articles are style-cleaned but
content-thin, Yo-Yo #1 trains on thin examples. If articles express
DataGraph knowledge in the correct register, Yo-Yo #1 trains on
signal-rich examples and generates better v2 drafts. The DataGraph is
the compound interest; the articles are how it pays out.

---

## Forward plan — 5 steps in sequence

### STEP 1 — Read RESEARCH corpus (DO FIRST)

**Owner:** project-editorial or project-intelligence Task
**Output:** annotated vocabulary + pattern list

Read all documents in `vault-privategit-design-1/research/`:
`brand-voice.md`, `design-philosophy.md`, `primitive-vocabulary-rationale.md`,
`wikipedia-leapfrog-2030.md`, and all other files in that folder.

Also read as positive examples:
- All 5 `content-wiki-corporate/` articles (correct Bloomberg/FT register)
- `content-wiki-projects/topic-co-location-methodology.md`
- `content-wiki-projects/topic-zoning-acquisition-rules.md`

Extract: lead sentence patterns, sentence length, active/passive ratio,
jargon handling, consequence-first structure, citation style, how
technical depth is introduced.

Scope: read, annotate, extract — do not rewrite anything.

Why first: Step 2 tokens encode what we actually find here. Tokens
built without reading the corpus are speculation, not evidence.

**Actual outcome:** *(update when complete)*

---

### STEP 2 — Author language tokens

**Owner:** project-design Task
**Output:** `pointsav-design-system/tokens/language/` token files

Grounded in what Step 1 found:
- `register-corporate.json` — Bloomberg/FT rules as token constraints
- `register-documentation.json` — Stripe/Cloudflare + accessibility layer rules
- `register-specialist.json` — RIBA/IFC specification rules (for bim/gis/design sites)
- `vocabulary-banned-corporate.json` — retirement list for corporate + projects wikis
- `vocabulary-banned-documentation.json` — retirement list for documentation wiki
- `crossref-specialist-sites.json` — canonical patterns for linking to bim/gis/design
- `template-topic-lead-corporate.json` — opening sentence patterns, corporate register
- `template-topic-lead-documentation.json` — opening sentence patterns, documentation register

Scope: enough for the SLM to score articles and generate register-correct
first sentences. Not exhaustive templates or full style guides.

Integration pipeline:
```
tokens/language/
    ↓ service-content/schemas/
        ├── banned-vocab.lark
        ├── genre-templates/
        └── register-specs/
    ↓ service-slm Doorman editorial pipeline
        ├── pre-generation: inject register spec into system prompt
        ├── post-generation: score against register tokens
        └── training tuple: (draft, verdict, register_score) → DPO corpus
```

**Actual outcome:** *(update when complete)*

---

### STEP 3 — Read full DataGraph for content enrichment

**Owner:** project-editorial or project-intelligence Task
**Output:** per-entity content brief for each article targeted for rewrite

For each article in the rewrite priority list, look up the corresponding
DataGraph entity using:
- The reconciliation artifact (`.agent/artifacts/datagraph-content-reconciliation-2026-05-07.md`)
- The topic CSVs in `service-content/ontology/topics/`
- The RESEARCH documents in `vault-privategit-design-1/research/`

Extract for each entity:
- What domain does it belong to?
- What other entities connect to it, and how? (edges: operates, deploys, serves)
- What themes span it?
- What does the RESEARCH corpus say about this domain or entity?
- What is the consequence framing for the institutional reader?

Output: "what the DataGraph knows that the article doesn't currently say"
— the substance that the rewrite will add.

Scope: read and extract. Not a full graph query; enough to enrich the rewrite.

**Actual outcome:** *(update when complete)*

---

### STEP 4 — Domain cleanup in service-content CSVs

**Owner:** project-intelligence or project-data Task (not project-editorial —
submit via outbox if working from project-editorial cluster)

Fix structural bugs:
- `wiki_repo` field: correct corporate and projects CSV entries to point
  at `content-wiki-corporate` and `content-wiki-projects` respectively
- `wiki_path` format: update all entries from stale `topics/topic-*.md`
  to `<category>/<slug>.md` format
- Mark `doorman-protocol` as `active` (first confirmed active entry)

Register all existing content:
- Add all ~251 unclassified articles to the correct domain CSVs with
  `active` state and correct `wiki_path`
- Create `guides_documentation.csv` — register all ~72 GUIDEs as
  DataGraph entities per `datagraph-guide-entity-class.md`
- Register RESEARCH corpus documents as `research-document` entities

Patch `domains.csv` to add the `documentation` domain row (required by
the GUIDE entity class convention).

Scope: register all existing content so Yo-Yo #1 has clean taxonomy.
Not perfect classification — "good enough for Yo-Yo #1" is the bar.

**Actual outcome:** *(update when complete)*

---

### STEP 5 — DataGraph-informed rewrite of all TOPICs and GUIDEs

**Owner:** project-editorial Task
**Input:** HOW from tokens (Step 2) + WHAT from DataGraph briefs (Step 3)

One pass across all 252 EN articles + ~72 GUIDEs. For each article:
- Fix the lead sentence: consequence-first, register-correct, no jargon
- Add "why it matters" for the institutional reader (1 sentence per
  major section) where missing
- Pull in DataGraph relationships, domain context, and theme connections
  that the article currently doesn't express
- Retire vocabulary per the retirement list
- Add crossref to specialist site where appropriate

Scope: ~15–30 minutes per article maximum. Light enrichment pass.
Not full rewrites, structural overhauls, or new sections invented
from scratch.

Goal: "draft 2 of 10 that expresses what the DataGraph knows."
Yo-Yo #1 then produces draft 3 informed by what draft 2 taught it.

**Priority order:**

| Priority | Target | Why |
|---|---|---|
| 1 | Corporate wiki (5 articles) | Shortest; highest external visibility; already correct register — verify + enrich |
| 2 | Projects wiki (34 articles) | Same institutional audience on different subject |
| 3 | Architecture + governance | Most register violations; highest external exposure; core value proposition (bank's tech committee reads these) |
| 4 | Services, applications, reference | Lower urgency |
| 5 | GUIDEs | Engineering audience only; enrichment still valuable for context headers |

**Highest urgency for architecture/governance pass:**
- `architecture/compounding-substrate.md` — core value proposition; written for engineers
- `governance/ontological-governance.md` — data governance; impenetrable jargon
- `architecture/doorman-protocol.md` — AI boundary; full of internal service names
- `architecture/leapfrog-2030-architecture.md` — platform vision; needs Corporate register lead

**Actual outcome (cumulative):**

Priority 1 — Corporate wiki (5 articles): COMPLETE 2026-05-08 prior session. Commit `16c5563`. Stage 6 pending.
Priority 2 — Projects wiki (34 articles): COMPLETE 2026-05-08 prior session. Commit `7e634e0`. Stage 6 pending.
Priority 3 — Architecture + governance (4 EN+ES pairs): COMPLETE 2026-05-08 prior session. Commit `96e221d`. Stage 6 pending.
Priority 4a — Services first batch (3 EN+ES pairs: service-slm, service-email, service-fs-architecture): COMPLETE 2026-05-08 prior session. Commit `91b8910`. Stage 6 pending.

**Priority 4b — Services-remaining (12 EN+ES pairs): COMPLETE 2026-05-08 this session.**
- Batch 1 commit `e7b14c3` (6 articles): service-people, service-extraction, service-search, message-courier, service-business-clustering, service-places-filtering — Pattern A redundant blockquote leads removed; Pattern B `short_description` and `paired_with` fields added.
- Batch 2 commit `11d617a` (6 articles): fs-anchor-emitter, service-fs-security-compliance, service-fs-data-lake, service-slm-totebox-sysadmin, template-ledger, pointsav-gis-engine — frontmatter normalised (template-ledger schema upgrade `foundry-topic-v1` → `foundry-doc-v1`; fs-anchor-emitter `type: guide` → `type: topic`; pointsav-gis-engine body H1 removed per content-contract.md §5.2; "Compounding Doorman" → "access-control gateway, also referred to as the Doorman" on first use). Stage 6 pending.

Lead-only refinement per "draft 2 of 10" calibration. Body structure preserved across all 12 articles. Ring 1/2/3 retained per services-topic exemption (architecture being defined). All `last_edited` bumped to 2026-05-08.

**Priority 4c — applications category (3 named app-* + 5 design-spec/overview/launch + 1 retire): COMPLETE 2026-05-08 this session.**
- `500f201` app-mediakit-knowledge EN+ES — schema upgrade `foundry-topic-v1` → `foundry-doc-v1` + lead consequence-first (in-flight pre-merge).
- `5f17aa1` app-mediakit-marketing EN+ES + app-orchestration-gis EN+ES — register-corrected, short_description added.
- `dc9acec` 4 design-intent articles moved `applications/` → `architecture/` (article-shell-leapfrog, knowledge-wiki-home-page-design, wikipedia-leapfrog-design, location-intelligence-ux); location-intelligence-platform refined in place (body H1 removed per content-contract.md §5.2); documentation-pointsav-com-launch-2026-04-27 retired (historical event captured in CHANGELOG and the live URL); 2 inbound wikilinks scrubbed in app-mediakit-knowledge See Also.
- `0a5b96f` frontmatter+lead edits to the 4 moved articles (schema upgrade `foundry-topic-v1` → `foundry-doc-v1` on 3 of them; full frontmatter normalisation across all 4 EN+ES pairs).

After 4c: `applications/` retains 4 canonical app-* topics + `location-intelligence-platform` (platform overview); `architecture/` gains 4 design-intent siblings to compounding-substrate, doorman-protocol, and the 5 Master batch TOPICs from earlier in the session.

**Still pending in Step 5:**
- Priority 5: GUIDEs (~72 files in `woodfine-fleet-deployment`)
- Category migration: root `topic-*.md` files → category subdirectories (unblocked 2026-05-07; verification pass to confirm completion)

Step 4 (CSV structural bugs) remains blocked on project-intelligence / project-data scope.

---

## What NOT to do

- **Do not delete wiki articles because they are not in a CSV.** The wiki
  is richer than the CSVs; the fix is to add to the CSVs, not subtract
  from the wiki.
- **Do not conflate "not in CSV" with "wrong."** The CSVs are stale
  prototypes, not law.
- **Do not run CSV updates and wiki rewrites in the same session.** One
  session per `.git/index`. Use separate cluster clones or sequence.
- **Stream A (CSV updates) is not project-editorial scope.** Submit via
  outbox to project-intelligence or project-data.
- **Do not try to perfect anything in one pass.** The Yo-Yo cycles handle
  improvement. Perfection in pass 1 wastes energy better spent establishing
  the framework.
- **Do not use RIBA/IFC specification language in the main wikis.** That
  register belongs on bim / gis / design specialist sites.

---

## Critical files

| File | Role |
|---|---|
| `.agent/artifacts/datagraph-content-reconciliation-2026-05-07.md` | Full reconciliation report — all findings from Phase 1 |
| `vendor/pointsav-monorepo/service-content/ontology/topics/topics_documentation.csv` | DataGraph TOPIC taxonomy — documentation domain (5 seed entries, all stale) |
| `vendor/pointsav-monorepo/service-content/ontology/topics/topics_corporate.csv` | DataGraph TOPIC taxonomy — corporate domain (wiki_repo bug) |
| `vendor/pointsav-monorepo/service-content/ontology/topics/topics_projects.csv` | DataGraph TOPIC taxonomy — projects domain (wiki_repo bug) |
| `conventions/datagraph-guide-entity-class.md` | GUIDE entity class ratification — 2026-05-07 |
| `deployments/vault-privategit-design-1/research/` | RESEARCH corpus — positive language examples in Bloomberg/FT + Stripe registers |
| `clones/project-editorial/content-wiki-documentation/` | Documentation wiki cluster copy (~213 EN) |
| `customer/content-wiki-projects/` | Projects wiki (~34 EN) |
| `customer/content-wiki-corporate/` | Corporate wiki (~5 EN) |
| `pointsav-design-system/tokens/language/` | Language tokens — target location for Step 2 output (does not exist yet) |

---

## Monthly cycle (after this pass)

```
[source/asset/ledger data accumulates]
→ Yo-Yo #1 runs (daily) → DataGraph v2 entities refined
→ Yo-Yo #2 runs (weekly/monthly) → DataGraph v3 higher quality
→ Monthly content sweep:
    - Yo-Yo generates new draft per article against updated DataGraph
    - Editorial review: accept / refine / reject each draft
    - Each verdict = DPO training tuple → SLM improves for next month
    - Language tokens score for register drift → caught automatically
    - Next month: articles one level better; SLM first drafts already better
→ Repeat
```

The compounding effect: each monthly pass produces better training signal
than the last because (a) the DataGraph is richer, (b) the SLM has been
trained on prior editorial verdicts, and (c) the articles being refined
are themselves already better. Work required per article decreases as
the system matures.

---

## Starting do list — where to begin (next session)

This is the concrete starting batch. Everything here feeds Step 1 and Step 2
of the forward plan. Nothing is written until this reading is done.

### Batch A — Read positive examples (Step 1)

- [ ] Read all files in `deployments/vault-privategit-design-1/research/`
      — brand-voice.md, design-philosophy.md, primitive-vocabulary-rationale.md,
        wikipedia-leapfrog-2030.md, and any others
      — note: what makes each encyclopedic? what patterns recur?
- [ ] Read all 5 `customer/content-wiki-corporate/` articles
      — note: lead sentence structure, sentence length, consequence framing
- [ ] Read `customer/content-wiki-projects/topic-co-location-methodology.md`
- [ ] Read `customer/content-wiki-projects/topic-zoning-acquisition-rules.md`

**Output:** a short annotated list — lead patterns, vocabulary, sentence
structure — that Step 2 tokens will encode.

### Batch B — Read worst-register articles (diagnosis before token authorship)

Before writing the tokens, read the four highest-urgency architecture articles
to understand concretely what is wrong:

- [ ] `content-wiki-documentation/architecture/compounding-substrate.md`
- [ ] `content-wiki-documentation/governance/ontological-governance.md`
- [ ] `content-wiki-documentation/architecture/doorman-protocol.md`
- [ ] `content-wiki-documentation/architecture/leapfrog-2030-architecture.md`

**Output:** for each article, one line on the primary register violation
(e.g. "opens with internal metaphor; no consequence sentence; 4 retired terms
in paragraph 1"). This makes the tokens specific and actionable.

### Batch C — Author language tokens (Step 2)

With Batches A and B in hand:

- [ ] `register-corporate.json` — Bloomberg/FT rules + Wikipedia lead structure
- [ ] `register-documentation.json` — Stripe/Cloudflare + accessibility layer
- [ ] `register-specialist.json` — RIBA/IFC (for bim/gis/design specialist sites)
- [ ] `vocabulary-banned-corporate.json` — retirement list for corporate + projects
- [ ] `vocabulary-banned-documentation.json` — retirement list for documentation
- [ ] `crossref-specialist-sites.json` — canonical link patterns to bim/gis/design
- [ ] `template-topic-lead-corporate.json` — 3–5 lead sentence patterns, corporate
- [ ] `template-topic-lead-documentation.json` — 3–5 lead sentence patterns, documentation

Destination: `vendor/pointsav-design-system/tokens/language/`
Owner: project-design Task

### Batch D — Rewrite the 5 corporate articles (Step 5 preview — highest urgency)

Once tokens are in place:

- [ ] Rewrite each of the 5 `content-wiki-corporate/` articles using the
      Wikipedia encyclopedic structure: Lead → Body sections → See also → References
- [ ] Lead sentence: consequence-first, 14–18 words, no jargon
- [ ] Each article passes the test: does the reader understand the subject,
      or have they only found a fact?
- [ ] Bilingual: update `.es.md` pairs to match

These 5 articles become the seed positive examples for Yo-Yo #1 training.
They are also the most externally visible content — bankers read these first.

### After Batch D — proceed to projects wiki (34 articles), then documentation

Follow the priority order in Step 5 of the forward plan.
Each batch: HOW from tokens, WHAT from DataGraph brief (Step 3).

---

*To update at the end of each step: fill in "Actual outcome" lines in the
forward plan above, and check off items in this do list as they complete.*
