---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-knowledge
target_repo: pointsav-design-system
target_path: research/
target_filename: DESIGN-RESEARCH-ux-writing.md
audience: internal
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-RESEARCH
authored: 2026-05-23
authored_by: totebox@project-knowledge
authored_with: claude-opus-4-7
---

# DESIGN-RESEARCH — UX Writing Audit, app-mediakit-knowledge

> Scope: every visible string emitted by `home_chrome()` and `wiki_chrome()` in
> `pointsav-monorepo/app-mediakit-knowledge/src/server.rs`, plus the lede panels
> served from `content-wiki-documentation/index.{md,es.md}`. Three instances are
> in scope: documentation.pointsav.com, projects.woodfinegroup.com,
> corporate.woodfinegroup.com.
>
> Method: read all rendered string literals; classify each against the three
> declared audiences (auditors, engineers, procurement evaluators); compare
> against the linguistic protocol tokens (`protocol-text.yaml`,
> `wf-protocol-lexicon.yaml`) and the Bloomberg-standard rule.

---

## 1. Executive summary

The platform's code-side chrome currently sits at a high C / low B. Structurally
it is correct — Wikipedia muscle memory, clean tab grammar, a sober footer, a
documented research-trail block — but the strings themselves are the work of an
engineer copying MediaWiki affordances verbatim. They are not yet writing for an
auditor, a procurement evaluator, or a senior engineer browsing a regulated
record.

The four highest-leverage moves are (a) replace the auto-generated welcome
banner "Welcome to PointSav Knowledge, the corporate knowledge wiki for the
PointSav Digital Systems platform." with a single declarative line that names
what the reader is looking at and what disclosure regime governs it; (b)
rewrite the IVC band placeholder so it is not a leak of internal phase
vocabulary onto a public surface; (c) reconcile the EN lede (5 sentences,
references air-gap and ownership) with the ES lede (4 sentences, does not);
(d) rename `Aa`, `▾`, `[hide]`, `[pin]` to labelled controls — the platform
asks an auditor to guess what those mean.

Below the executive summary, every recommendation is paired with the current
string and an approximate line number in `server.rs`.

---

## 2. Home page lede audit

### 2.1 Current EN lede (rendered between welcome banner and search box)

> *PointSav builds operating systems and services for regulated businesses that
> need to own their data, their AI, and their record-keeping outright. The
> platform runs on customer hardware, produces continuous-disclosure-grade
> records by structure, and operates fully without AI for buyers that require
> an air-gap.*
>
> *This wiki is the engineering library for that platform, maintained against
> the published editorial standard. It documents the architecture, the services,
> the operating systems, the governance commitments, and the design rationale
> that binds future development. It is written for institutional readers —
> auditors, technical due-diligence reviewers, and procurement evaluators — and
> for the engineers who build on or extend the platform. Where the engineering
> monorepo holds the code, this wiki holds the reasoning.*
>
> *Forward-looking statements in this wiki carry planned, intended, or target
> language per the editorial standard.*

### 2.2 Per-audience score

| Audience | What they need on first paint | Lede delivers? | Gap |
|---|---|---|---|
| **Auditor** | Disclosure regime (NI 51-102 / OSC SN 51-721), record-keeping model, change-control claim, retention assertion | Partially | Forward-looking clause is present but unnamed (no NI 51-102 citation in body); record-keeping is asserted but no link to the WORM ledger or commit-signing claim from this paragraph |
| **Engineer** | Where the code lives, where the doctrine lives, how the two relate | Yes (last sentence: "monorepo holds the code, this wiki holds the reasoning") | The reverse direction is missing — there is no link out to the monorepo, the design system, or the editorial standard from the lede itself |
| **Procurement evaluator** | Vendor identity, scope of supply, ownership posture | Partially | "Operating systems and services for regulated businesses" is the only product-scope statement; missing: jurisdiction, customer count or class, supported regulators |

### 2.3 EN before/after

| # | Current | Proposed |
|---|---|---|
| L1 | "PointSav builds operating systems and services for regulated businesses that need to own their data, their AI, and their record-keeping outright." | "PointSav Digital Systems builds operating systems and services for regulated businesses that own their data, their inference, and their books outright." |
| L2 | "The platform runs on customer hardware, produces continuous-disclosure-grade records by structure, and operates fully without AI for buyers that require an air-gap." | "The platform runs on customer-owned hardware, produces records that satisfy continuous-disclosure obligations under NI 51-102 by structure, and operates without AI in air-gapped deployments." |
| L3 | "This wiki is the engineering library for that platform…" | "This wiki is the engineering library for that platform, maintained under the editorial standard published in `style-guide-topic`." |
| L4 | "It documents the architecture, the services, the operating systems, the governance commitments, and the design rationale that binds future development." | "It documents the architecture, the services, the operating systems, the architectural decisions (ADRs), and the design rationale that binds future development." |
| L5 | "It is written for institutional readers — auditors, technical due-diligence reviewers, and procurement evaluators…" | Unchanged — this is the strongest sentence in the lede. |
| L6 | "Where the engineering monorepo holds the code, this wiki holds the reasoning." | "Where the engineering monorepo holds the code, this wiki holds the reasoning. The two are versioned together." |
| L7 | "Forward-looking statements in this wiki carry planned, intended, or target language per the editorial standard." | "Forward-looking statements in this wiki are flagged with planned, intended, or target language and a NI 51-102 hatnote on the article that carries them." |

Rationale for changes:
- L1: replaces *AI* (which `protocol-text.yaml` flags as marketing-prone) with
  *inference* (the engineering object) and *record-keeping* with *books* (the
  accounting-correct noun an auditor expects).
- L2: cites the actual disclosure regime by name; "by structure" stays —
  it is a load-bearing claim about WORM ledgers and signed commits.
- L4: "governance commitments" reads like a corporate website. "ADRs" is the
  artifact an auditor will be looking for and the engineer will already know.
- L7: names the mechanism (hatnote on the article) instead of the policy — the
  reader can look at any forward-looking article and verify the claim.

---

## 3. Navigation label recommendations

### 3.1 Top nav (line 1139–1148, also 2220–2226 on article pages)

Current strings: `Home`, `All pages`, `Categories`, `Recent changes`, `ES` (or
`EN` on the Spanish side).

| Current | Issue | Proposed | Rationale |
|---|---|---|---|
| `Home` | Generic; says nothing about *which* home | `Home` — leave | A reader who has scrolled deep into an article values brevity here; the wordmark and site title already say which wiki this is |
| `All pages` | MediaWiki vocabulary; non-engineers read it as "every URL on the site, including special pages" | `Article index` | Auditor and procurement reader expect a numbered list of records; `Article index` is the term used in regulatory filings |
| `Categories` | Acceptable; matches Wikipedia muscle memory | `Categories` — leave | The category grid on the home page reinforces what this means; no improvement needed |
| `Recent changes` | Wikipedia term; reads as "edit log" rather than "what is new" | `Change log` | Aligns with the regulatory expectation that there is a single canonical changelog; pairs naturally with the `CHANGELOG.md` convention used in every repo |
| `ES` / `EN` | Two ASCII characters; ambiguous as a button (could be a country, a stock ticker, a tag) | `Español` / `English` | The article-page language switcher (line 2417–2426) already uses the language name in its own glyph; consistency wins |

### 3.2 Lower nav row (line 1158–1176)

Current strings vary by brand: `Disclaimer`, `Contact`, `pointsav.com`,
`GitHub` (PointSav theme) or `Corporate`, `Newsroom`, `Projects` (Woodfine
themes).

| Current | Issue | Proposed |
|---|---|---|
| `Disclaimer` | Singular; reads as a checkbox-tick legal page | `Disclosures` |
| `Contact` | Adequate | `Contact` — leave |
| `pointsav.com` | Marketing-site link labelled with a domain — unusual for an institutional reader | `Vendor site` |
| `GitHub` | Service-name; non-engineers may not know what to expect on the other side | `Source repositories` |
| `Corporate` | Already a category name elsewhere — collides | `Corporate site` |
| `Newsroom` | Adequate; matches regulatory press-release convention | `Newsroom` — leave |
| `Projects` | Ambiguous — projects of what? | `Project portfolio` |

### 3.3 Left-rail navigation portlet (line 2340–2351)

Current strings: `Main page`, `Random article`, `Wanted articles`,
`All pages`, `Categories`, `Recent changes`, `Statistics`, `Search`.

| Current | Issue | Proposed |
|---|---|---|
| `Main page` | Same control as `Home` in the top nav; uses different wording | `Home` (match top nav) |
| `Random article` | Wikipedia-stock; in an institutional library, randomness reads as unserious | Remove from default rail; relegate to a "Browse" submenu, or rename `Random topic` |
| `Wanted articles` | Wiki-jargon; an auditor reading `Wanted articles` will picture a 19th-century post-office bulletin | `Open requests` or `Topics requested` |
| `All pages` | See §3.1 — same fix | `Article index` |
| `Recent changes` | See §3.1 — same fix | `Change log` |
| `Statistics` | Adequate, but ambiguous | `Wiki statistics` |
| `Search` | Redundant — search box is in the header | Remove from rail; the header search box has higher visual weight |

---

## 4. Category grid copy audit

### 4.1 Current category set

From `RATIFIED_CATEGORIES` (line 596–609), rendered through `humanize_category()`
which simply title-cases the slug after splitting on hyphens. Twelve categories
ship in the constant array; the wiki taxonomy currently has 10 active
(`company/` and `help/` were retired per naming-convention §13 decision #6 but
remain in `RATIFIED_CATEGORIES` and render as `In preparation.` — see §6).

| Slug | Rendered as | Score for non-engineer | Proposed label |
|---|---|---|---|
| `architecture` | "Architecture" | Acceptable | "Architecture" — leave |
| `substrate` | "Substrate" | **Opaque** — substrate is workspace-internal vocabulary; an auditor will not parse this | "Foundations" or "Platform foundations" |
| `patterns` | "Patterns" | **Opaque** — patterns of what? A reader expects design patterns, project patterns, or something else | "Design patterns" |
| `services` | "Services" | Acceptable in software context | "Platform services" |
| `systems` | "Systems" | Adequate, but plural is ambiguous in an OS-per-article structure | "Operating systems" |
| `applications` | "Applications" | Acceptable | "Applications" — leave |
| `governance` | "Governance" | Acceptable; matches regulatory vocabulary | "Governance" — leave |
| `infrastructure` | "Infrastructure" | Acceptable | "Infrastructure" — leave |
| `company` | "Company" | **Currently empty** (retired) — see §6 | Remove from `RATIFIED_CATEGORIES` |
| `reference` | "Reference" | Acceptable; matches library conventions | "Reference" — leave |
| `help` | "Help" | **Currently empty** (retired) — see §6 | Remove from `RATIFIED_CATEGORIES` |
| `design-system` | "Design System" | Acceptable to designers; opaque to procurement | "Design system" — leave; the wikilink to the design system surface elsewhere disambiguates |

### 4.2 Section heading above the grid (line 1310)

| Current | Proposed |
|---|---|
| "Browse by area" | "Browse the library" — *area* is unspecific; *library* matches the lede's "engineering library" framing |

### 4.3 "All N →" link (line 1326–1330)

| Current | Proposed |
|---|---|
| `"All " (count) " →"` (e.g. "All 35 →") | `"View all " (count) " articles →"` — the verb and the noun make the action unambiguous |

### 4.4 "In preparation" marker (line 1333)

| Current | Proposed |
|---|---|
| "In preparation." | Either hide the empty category entirely (preferred — see §6) or surface "No published articles yet." Empty rooms are a procurement red flag; an explicit zero is honest, "In preparation." reads as a status flag for a draft document |

---

## 5. Article page microcopy

### 5.1 IVC band placeholder (line 2470–2485)

This is the single weakest copy on the article page. The current rendered string
**leaks the internal phase number onto a public surface** and tells the reader
nothing about what IVC is or what it will do.

| Current | Issue | Proposed |
|---|---|---|
| "Verification not yet available — Phase 7" | Leaks internal roadmap vocabulary; reader has no idea what *verification* means in this context | "Citation verification is not yet enabled on this surface." (Phase reference removed; document it in `NEXT.md` instead.) |
| `aria-label="Verification status"` | OK; matches the band's function | Unchanged |
| `Citation marks:` (density label) | Reader sees the toggle before they have seen a citation mark | "Citation marks in body text:" — the qualifier tells the reader *where* the toggle takes effect |
| `Off` / `Exceptions only` / `All` | `Exceptions only` is opaque — exceptions to what? | `Hide` / `Only where contested` / `Show all` — the active state should read like a sentence: "Show only citations where contested" |

### 5.2 Quality badge (line 2408–2410)

The current implementation renders the raw enum value (e.g. `featured`,
`good`, `stub`) as the badge label. For an institutional reader the badge
needs a controlled vocabulary.

| Enum value (likely) | Current rendered label | Proposed label |
|---|---|---|
| `featured` | "featured" | "Featured" (capitalised, no semantic change) |
| `good` | "good" | "Reviewed" — *good* is Wikipedia jargon; *reviewed* maps to the editorial review queue the platform already implements |
| `stub` | "stub" | "Stub" (capitalised; pairs with the stub hatnote below) |
| `draft` | "draft" | "Draft" |
| `pre-build` | "pre-build" | "Pre-publication" |

Implementation note: the rendered string is the raw `q` value
(`span class={ "quality-badge quality-" (q) } { (q) }`). A `humanize_quality()`
helper analogous to `humanize_category()` is the right fix.

### 5.3 Tagline (line 2438)

| Current | Issue | Proposed |
|---|---|---|
| `"From " (site_title.trim_end_matches(" Wiki"))` — typically renders as "From PointSav Knowledge" | Matches Wikipedia's "From Wikipedia, the free encyclopedia" tagline; the trim strips "Wiki" but there is no "Wiki" suffix on the current site titles, so the trim is a no-op | "From the PointSav engineering library" — names the document class instead of the brand |

### 5.4 Research trail summary (line 2540)

| Current | Issue | Proposed |
|---|---|---|
| `summary { "Research trail" }` | Adequate; matches the linguistic protocol's "audit trail" sensibility | "Research trail (sources, prompts, review)" — tells the reader what is inside before they expand it |

The DL inside the `<details>` renders the raw frontmatter key (`(key)` at line
2543). Keys like `prompt`, `model`, `reviewed_by` will render as those literal
strings unless a `humanize_research_trail_key()` helper is added. Sample
proposed mappings:

| Raw key | Proposed label |
|---|---|
| `prompt` | "Originating prompt" |
| `model` | "Model" |
| `reviewed_by` | "Reviewed by" |
| `sources` | "Sources consulted" |
| `language_protocol` | "Editorial protocol" |

### 5.5 Action menu (line 2456–2467)

The "More actions" dropdown is opened by a `▾` glyph with no label. Auditor
behaviour: they hover, see nothing, do not click.

| Current | Issue | Proposed |
|---|---|---|
| `summary.wiki-cactions-toggle title="More actions" { "▾" }` | Glyph-only control with a `title` attribute that requires hovering | `summary` content becomes `"More ▾"`; keep `title="More actions"` for screen readers but stop hiding the affordance |
| `"Print / Export"` | Adequate | "Print or PDF" — *export* is engineer-vocabulary; *PDF* is what a procurement evaluator wants |
| `"Page information"` | Wiki-stock; OK | "Article metadata" |
| `"Cite this page"` | Adequate | "Cite this article" — the body uses *article* throughout; *page* is the underlying URL |
| `"Download as Markdown"` | Engineer-targeted; fine as-is on the documentation surface, but on corporate/projects this reads as oddly technical | "Download source (Markdown)" — *source* signals the regulatory artifact, the parenthetical names the format |

### 5.6 Tab labels (line 2390–2400, 2445–2455)

| Current | Issue | Proposed |
|---|---|---|
| `Article` / `Talk` | `Talk` is Wikipedia muscle memory; an institutional reader reads it as a video-call function | `Article` / `Discussion` (already what the `title="Discussion page"` attribute says — match the visible label to the tooltip) |
| `Read` / `View history` | `Read` is the active tab when reading; redundant. `View history` is acceptable. | Drop the `Read` tab entirely when it is the active state, OR rename to `Current` / `Revisions` — the verb→noun shift matches the auditor's mental model |

### 5.7 TOC controls (line 2358–2367)

| Current | Issue | Proposed |
|---|---|---|
| `[hide]` | Wikipedia-stock; the brackets are 1990s convention | "Hide" (no brackets); state changes to "Show" when collapsed |
| `[pin]` | Same; *pin* as a verb is engineer-vocabulary for "keep visible" | "Keep visible" / "Float" — the aria-label is already "Pin table of contents" which is the wrong direction |

### 5.8 Appearance menu button (line 2116–2120 and 2197–2201)

| Current | Issue | Proposed |
|---|---|---|
| `"Aa"` | Two-character glyph; reader does not know what it does until they click | Keep `Aa` glyph (it is established print-design convention for typography menus) but pair with a labelled chevron: `Aa ▾` — and ensure the `title="Appearance"` is also a visible label on tablet/desktop widths |

### 5.9 Hatnotes and notices (line 2496–2527)

| Notice | Current | Proposed |
|---|---|---|
| Forward-looking | "**Forward-looking information.** Statements herein are subject to material assumptions and risks. Per NI 51-102 / OSC SN 51-721 disclosure posture." | "**Forward-looking statements.** Subject to material assumptions and risks. Per NI 51-102 (CSA) and OSC SN 51-721." — *Information* is a noun; *statements* is what the regulation calls them. Bracketing CSA after NI 51-102 names the regulator for the procurement reader. |
| Stub | "*This article is a stub. You can expand it.*" | "*This article is a stub. The platform team is drafting a fuller version.*" — *You can expand it* invites public editing, which on a closed-CMS wiki is wrong |
| Disambiguation | "*This disambiguation page lists articles associated with the same title. If an internal link led you here, you may wish to change the link to point directly to the intended article.*" | "*This page lists articles that share a title. If a link directed you here in error, please report it via the Contact page.*" — *change the link* is a Wikipedia-editor instruction inappropriate to a regulated wiki |
| Redirected from | `"(Redirected from " <slug> ")"` (line 2488–2493) | "(Redirected from the page titled *`<slug>`*)" — italicise the slug; add the word *page* so the reader knows what the slug refers to |

### 5.10 Footer affordances (line 2566–2599)

| Current | Issue | Proposed |
|---|---|---|
| `"Categories:"` / `"Category:"` | Stock Wikipedia plural/singular toggle; OK | Unchanged |
| `"Last edited: "` | OK | "Last revised: " — *edited* implies casual change; *revised* implies a tracked revision (matches the change-log framing) |

### 5.11 Sister surfaces (line 1386–1459)

The current strings are reasonable but inconsistent across themes.

| Theme | Slot | Current name + desc | Proposed |
|---|---|---|---|
| woodfine-projects | 1 | "Corporate Reference" / "Woodfine Management Corp." | "Corporate knowledge" / "Woodfine Management Corp. records" — *Reference* clashes with the category; *records* is the auditor's noun |
| woodfine-projects | 2 | "Live Platform" / "GIS co-location intelligence" | "Live platform" / "Co-location intelligence (GIS)" — *Live* lower-cased; jargon goes in the parenthetical |
| woodfine-projects | 3 | "Engineering Documentation" / "PointSav platform reference" | "Engineering documentation" / "PointSav platform reference" |
| woodfine-projects | 4 | "Newsroom" / "Announcements and updates" | "Newsroom" / "Press releases and announcements" — *updates* is too soft; *press releases* names the regulatory artifact |
| pointsav | "Projects Platform" / "Woodfine co-location intelligence" | "Project portfolio" / "Woodfine co-location intelligence" |
| pointsav | "PointSav on GitHub" / "Canonical vendor-tier source" | "Source repositories (GitHub mirror)" / "Read-only mirror of canonical source" — names that GitHub is the mirror, not the canon; aligns with `principle-we-own-it` |

---

## 6. Empty / stub state messages

| Location | Current string | Audience read | Proposed |
|---|---|---|---|
| Empty category card (line 1333) | "In preparation." | Reads as a draft-document watermark; suggests work-in-progress on a public surface | Hide the card entirely. Empty categories should not render. `company/` and `help/` are retired per naming-convention §13 #6; remove from `RATIFIED_CATEGORIES` rather than render them as `In preparation.` |
| Uncategorised section heading (line 1346) | "All articles" | Misleading — these are the articles *not* in a ratified category | "Other articles" + the existing note ("Articles not yet sorted into a category.") below |
| Uncategorised section note (line 1347–1348) | "Articles not yet sorted into a category." | Adequate, but reads as an apology | "Articles outside the ratified category set. These are scheduled for taxonomy review." |
| Stub article banner (line 2508) | "*This article is a stub. You can expand it.*" | Invites public editing | "*This article is a stub. A full version is on the platform team's editorial backlog.*" |
| Missing search results (not in this view, but worth noting) | (engine-side) | — | If/when added: "No articles match this query. Try the [article index] or [request a topic]." |
| Wanted articles label | "Wanted articles" | Reads as 19th-century post-office vocabulary | "Open topic requests" |

The general principle: an institutional reader should never see a default
empty state that reads like a work-in-progress note from an engineer to
themselves. If a slot is empty, either hide it or explain *why* in a way that
preserves authority.

---

## 7. Bilingual parity notes

### 7.1 EN vs ES lede — sentence-level diff

| EN sentence | ES sentence | Parity |
|---|---|---|
| L1 — "PointSav builds … own their data, their AI, and their record-keeping outright." | L1 — "PointSav desarrolla … poseer sus datos, su inteligencia artificial y su registro contable de forma integral." | **Parity broken.** EN says *outright* (a posture word); ES says *de forma integral* (a completeness word). They are not the same claim. *outright* should translate to *de pleno derecho* or *en su totalidad y bajo su control*. |
| L2 — "runs on customer hardware … without AI … air-gap" | L2 — "funciona en hardware del cliente … sin IA … entorno aislado" | OK in meaning; *air-gap* → *entorno aislado* is acceptable in Spanish engineering vocabulary |
| L3 — "This wiki … maintained against the published [[style-guide-topic]]" | L3 — "Esta wiki es la biblioteca de ingeniería de esa plataforma." | **Parity broken.** ES drops the editorial-standard reference entirely. Spanish reader has no signal that the wiki is governed by a published standard. |
| L4 — "documents the architecture, the services, the operating systems, **the governance commitments**, and the design rationale that binds future development" | L4 — "Documenta la arquitectura, los servicios, los sistemas operativos, **el sistema de diseño** y los compromisos de gobernanza que rigen el desarrollo futuro." | **Parity broken.** EN omits *the design system*; ES omits *the design rationale*. The two ledes catalogue different things. |
| L5 — "It is written for institutional readers …" | L5 — "Está redactada para lectores institucionales …" | Parity OK |
| L6 — "Where the engineering monorepo holds the code, this wiki holds the reasoning." | L6 — "Donde el repositorio de ingeniería contiene el código, esta wiki contiene el razonamiento." | Parity OK |
| L7 — "Forward-looking statements … per the editorial standard." | (no equivalent) | **Parity broken.** ES has no forward-looking-statements paragraph. This is a BCSC discipline failure on the Spanish surface. |

### 7.2 ES frontmatter inconsistency

The ES file carries a `short_description:` field (lines 4–6 of `index.es.md`)
that the EN file does not. Either:

- both files carry a `short_description:` (preferred — it feeds the
  `topic-short-description` paragraph in `wiki_chrome` line 2440), or
- neither does (current EN behaviour).

The asymmetry today means the ES home page may render a `<em>` block that the
EN home page does not, depending on whether the home renderer reads
`short_description` at all.

### 7.3 ES last_edited drift

EN `last_edited: 2026-05-17`; ES `last_edited: 2026-05-15`. ES is two days
behind. The bilingual-symmetry rule in `protocol-text.yaml` requires
*1-to-1 EN/ES structural mirroring*; the EN sentence count is 5 (counting the
forward-looking paragraph), the ES count is 4. Reconcile both versions to the
EN sentence count with localised forward-looking language.

### 7.4 Action items for parity

1. Add the editorial-standard wikilink to the ES lede (sentence 3).
2. Add the design-system reference to the EN lede sentence 4 — or remove it
   from the ES lede sentence 4 — so the two catalogue the same artifacts.
3. Add a forward-looking-statements paragraph to ES; suggested text:
   *"Las afirmaciones prospectivas en esta wiki se identifican con los
   términos planeado, previsto u objetivo, conforme a la norma editorial
   publicada."*
4. Either populate `short_description:` in `index.md` to match `index.es.md`,
   or remove it from `index.es.md`.
5. Reconcile *outright* / *de forma integral* → *de pleno derecho*.

---

## 8. Priority order (highest ROI first)

| Rank | Change | Estimated effort | Why this rank |
|---|---|---|---|
| 1 | IVC band placeholder — strip "Phase 7" leak, rewrite to neutral institutional voice (§5.1) | 10 min | Single most damaging visible string; appears on every article page; leaks internal roadmap vocabulary onto a public surface |
| 2 | Bilingual parity — reconcile EN ↔ ES lede sentence count and add forward-looking paragraph to ES (§7) | 30 min | BCSC posture is currently EN-only; a Spanish-speaking reviewer at OSC or any LATAM regulator would surface this immediately |
| 3 | Category labels — `Substrate` → `Foundations`, `Patterns` → `Design patterns` (§4.1) | 15 min | Two opaque labels on the home grid; the rest of the home page chrome assumes the reader has parsed the categories |
| 4 | Empty category cards — remove `company/`, `help/` from `RATIFIED_CATEGORIES` (§6) | 5 min | Three of every twelve category cards currently render as `In preparation.`; this is procurement-grade visual debt |
| 5 | Stub banner — remove "You can expand it" (§5.9, §6) | 5 min | Single-string fix; invites the wrong reader behaviour |
| 6 | Navigation labels — `Recent changes` → `Change log`; `All pages` → `Article index`; `Wanted articles` → `Open topic requests` (§3) | 20 min | Renames; appears in top nav, left rail, and mobile drawer — three call sites each |
| 7 | Action menu — label the `▾` toggle, rename items (§5.5) | 15 min | The dropdown is the canonical hand-off to "advanced reader" affordances; today it is glyph-only |
| 8 | Quality badge — implement `humanize_quality()`; rename `good` → `Reviewed`, `pre-build` → `Pre-publication` (§5.2) | 30 min (new helper + tests) | The badge is the platform's editorial-confidence signal; raw enum values undermine it |
| 9 | Density toggle labels — `Off`/`Exceptions only`/`All` → `Hide`/`Only where contested`/`Show all` (§5.1) | 10 min | The control reads as a developer's debug toggle today |
| 10 | Tab labels — `Talk` → `Discussion`; consider `Read` removal (§5.6) | 15 min | Wikipedia muscle memory vs institutional voice — the discussion-page rename is the lower-risk half of this pair |
| 11 | EN lede sentence-level rewrites (§2.3) | 45 min | Higher cognitive effort; touches an article-content file (`index.md`) not the engine; lower ROI than chrome fixes that ship to every page |
| 12 | Sister-surfaces names + descriptions (§5.11) | 20 min | Visual polish; matters most on the Woodfine surfaces where the four-slot grid sets the corporate tone |
| 13 | Research-trail key humanization (§5.4) | 20 min (new helper) | Only renders when an article carries a `research_trail:` block — limited surface today, but every article that adds one inherits the fix |
| 14 | TOC controls `[hide]` / `[pin]` (§5.7) | 5 min | Tiny but persistent annoyance; bracketed-verb convention is dated |
| 15 | "Last edited:" → "Last revised:" (§5.10) | 5 min | Renders on every article footer; trivial change |

The first five items account for roughly 80% of the perceived quality jump.
Items 6–10 take the platform from a strong B to a credible A-. Items 11–15
are polish.

---

## 9. Out of scope (flagged for separate review)

- **JSON-LD strings** emitted by `jsonld_for_topic()` (line 2172) — not visible
  to a human reader; review for schema.org-vocabulary correctness is a
  separate audit.
- **OpenAPI 3.1 strings** in `openapi.yaml` — public API surface; institutional
  copy review owes the same discipline but the file is 1,027 lines and
  warrants its own pass.
- **Email egress and notification copy** — lives in
  `service-email-egress-{ews,imap}/`, not this engine; route to a separate
  research note.
- **Authentication / edit-review queue strings** (Phase 5 surfaces) — only
  shown to authenticated editors; lower-priority audience and not in the
  three target classes for this audit.

---

## 10. Sources

- `content-wiki-documentation/index.md` (EN lede, dated 2026-05-17)
- `content-wiki-documentation/index.es.md` (ES lede, dated 2026-05-15)
- `pointsav-design-system/tokens/linguistic/protocol-text.yaml` (Bloomberg
  standard, ISO 24495-1 plain-language rule, bilingual symmetry rule)
- `woodfine-media-assets/tokens/linguistic/wf-protocol-lexicon.yaml` (Five Fs
  thematic anchors; banned-vocabulary list including *transformative*,
  *disruptor*, *but*, *however*, *despite*, *although*)
- `woodfine-media-assets/tokens/linguistic/corporate-authority.yaml`
  (signature block reference)
- `pointsav-design-system/guidelines/MEMO-02-Neurodiversity-Typography.md`
  (left-alignment rule; cognitive-load reduction)
- `pointsav-monorepo/app-mediakit-knowledge/src/server.rs` lines 594–609
  (`RATIFIED_CATEGORIES`), 1031–1499 (`home_chrome`), 1503–1514
  (`humanize_category`), 2102–2674 (`wiki_chrome`)
