# Knowledge Platform — Vision & Architecture (2026)

> **Created:** 2026-05-21 · Totebox@claude-code · project-knowledge cluster
> **Last updated:** 2026-05-21 (rev 4 — project-editorial cross-check)
> **Status:** foundation document — the vision layer the rebuild plan sits on.
>
> Synthesized from a 4-agent OPUS research sweep (build-vs-adopt, leapfrog-2030
> vision, AI-native contribution model, machine-readable/flat architecture),
> then extended through an operator design conversation that settled the
> deployment shape and the source-of-truth inversion.
>
> **Method caveat:** the four research agents had web search/fetch denied; they
> worked from the cluster's own docs + training knowledge (Jan 2026 cutoff). The
> *architecture and design* arguments stand on their own logic. *External trend*
> claims (AI-native consumption, `llms.txt` adoption) are unverified — re-verify
> before they enter a public TOPIC.

---

## 1. The build decision — SETTLED

**`app-mediakit-knowledge` continues as a custom Rust engine. Not MediaWiki. Not a "Rust MediaWiki".**

The history — "use MediaWiki" → "build a Rust MediaWiki" → "make it machine-readable and flat" — was not indecision. Each step was the team discovering that the *requirements* are structurally incompatible with what MediaWiki *is*:

- MediaWiki is **database-canonical**: the `page`/`revision`/`text` SQL tables *are* the wiki. This platform is the **exact inverse** — markdown-in-git is canonical, every database/index is throwaway derived state. Both cannot be true in one system. Flat-architecture is not a preference layered onto a wiki engine; it *is* the engine's architecture.
- "We Own It" is not satisfied by self-hosting someone else's PHP+MySQL stack — that is *custody*, not ownership. The custom engine is the only option where every byte that renders a page is auditable Rust or a small named set of crates.
- "A Rust MediaWiki" is a category error: MediaWiki's value is its 20-year ecosystem, not its code.

The reinvented-wheels cost is real but bounded — the wheels actually skipped (Lua templating, Wikibase, VisualEditor, 300-language i18n) are wheels this platform does not drive on. Phases 1–5 are shipped; the expensive part is paid.

**"We Own It" bounds scope — it does not expand it.** Own the render/edit/store/search/API path. Do *not* sprawl into federation/DID/blockchain. Own the architecture and the interfaces; vendor solved low-level problems (tantivy, comrak, axum, git2) without apology.

---

## 2. The vision — Leapfrog 2030

**Stop scoring "97% Wikipedia parity" as the win. Parity chrome is the cost of admission; the leapfrog is the product.**

The one-sentence thesis:

> Wikipedia is a corpus of human-readable **articles** with machine structure bolted on (Wikidata, infoboxes). The leapfrog platform is a corpus of machine-verifiable **claims**; the human article is one *projection* of them, the other being a structured claim API an AI agent consumes directly. **The article is a render target, not the artifact.**

Five load-bearing ideas:

1. **The claim is the atomic unit, not the article.** A claim is a first-class object — ID, content hash, citation set, `valid_at` time, confidence grade, dependency backlinks. An article becomes an ordered composition of claims plus connective prose. Wikipedia structurally *cannot* retrofit this; this platform is claim-native from the first commit, when the corpus is ~130 TOPICs and the discipline costs nothing.
2. **Verifiability is computed and continuous.** Citations are content-addressed (hash of the source at cite time) and continuously re-verified — the platform reports drift. Wikipedia tells you a citation *exists*; the leapfrog tells you whether it still *says what the claim claims*. Calibration: neutral by default, loud only on exceptions.
3. **Dual-surface from one store.** The same claim store serves a human reading view *and* a structured machine API, neither degraded. By 2030 a growing share of "reads" are agents grounding answers — they should get a fact with provenance and freshness attached, not a prose blob to re-parse.
4. **Knowledge is explicitly temporal.** Two clocks per claim — `valid_at` (when the fact applies) vs `published_at` (when written). The platform answers `state(topic, t)`.
5. **Authority is structural and git-governed.** Replace Wikipedia's opaque social-consensus governance (noticeboards, AfD, admin hierarchy) with signed commits, branch review, computed authority. Git *is* the governance layer.

**Keep from Wikipedia:** all reading chrome and muscle memory, anonymous read, NPOV as a human editorial value, verifiability epistemics, boring legible design, open license.
**Leapfrog past:** the opaque article-blob, frozen citations, the single prose surface, timeless presentation, social-process governance, Wikidata-as-a-divorced-project.

---

## 3. The deployment shape — three sovereign instances — SETTLED

Today the platform is a shortcut: **one** engine binary multiplexed across three content dirs via `WIKI_GUIDE_DIR_2` env vars. That ends. The model is **three genuine, isolated deployment instances** — because that is how a real Customer or Community Member deploys the wiki, and "We Own It" means dogfooding the real shape.

```
deployments/
├── media-knowledge-documentation/   ← PointSav (vendor) → documentation.pointsav.com
├── media-knowledge-projects/        ← Woodfine (customer) → projects.woodfinegroup.com
└── media-knowledge-corporate/       ← Woodfine (customer) → corporate.woodfinegroup.com
```

Each deployment instance contains: a git clone of its own content repo, a running `app-mediakit-knowledge` engine instance, per-instance config, and derived runtime state (search index etc. — gitignored).

Key facts:
- The **engine source** (`app-mediakit-knowledge`) stays *one* crate in `pointsav-monorepo`, built and deployed the normal vendor way. "Three instances" = the same engine *running* three times, each pointed at its own content — not three copies of the code.
- Each instance is genuinely isolated: the documentation server only ever holds documentation content, corporate only corporate, etc. Real separation, like three different customers.
- **Ownership maps to the corporate topology:** documentation = PointSav-owned; projects + corporate = Woodfine-owned. PointSav (vendor) ships the engine; Woodfine (customer) runs its instances. That *is* "how customers do it for real."

---

## 4. The architecture — source-of-truth inversion — SETTLED (requires Doctrine amendment)

There are **two** inversions. The first is internal to an instance and uncontroversial. The second is the publication-chain inversion the operator decided in this conversation, and it **conflicts with current Foundry Doctrine** — see §11.

### 4a. Inversion within an instance (uncontroversial)

| Tier | What | Where | Authority |
|---|---|---|---|
| **Canonical** | TOPIC/GUIDE markdown + frontmatter; sibling typed slots; `.es.md` pairs | the instance's git tree | **Source of truth.** |
| **Canonical history** | every revision, author, timestamp | `git log` | Source of truth for provenance — git *is* the revisions table. |
| **Derived projection** | tantivy index, redb claim+link graph, blake3 hashes, rendered HTML, JSON-LD | `<state_dir>` (not git-tracked) | Cache. Rebuildable by `git checkout && reindex`. Loses every conflict. |
| **Ephemeral (non-truth)** | CRDT draft state during a live editing session | RAM | Discardable. Collapses to a markdown commit at the F12 gate. Never authoritative. |

### 4b. Inversion of the publication chain (the Doctrine change)

**The instance's content repo IS canonical. GitHub is a downstream mirror that pulls from it.**

Old model (rejected): a separate Foundry-tier canonical repo → the live instance pulls a clone → GitHub mirrors the Foundry tier. This created the "propagate in-wiki edits back upstream" seam — the #1 risk the research flagged: a silent failure there forks the source of truth.

New model (adopted):

```
project-editorial / paired contributor
        │  commits / promotes (F12)
        ▼
media-knowledge-<wiki>  =  the live instance's git repo  =  CANONICAL
        │  push (downstream, one-directional)
        ▼
GitHub  github.com/{pointsav|woodfine}/media-knowledge-<wiki>  =  public mirror + integrity oracle
```

Why this is correct:
- **It is the honest customer shape.** A customer running their own instance owns their content *on their instance*. PointSav cannot be the central canonical-holder for every customer's content — that is neither sovereign for the customer nor scalable.
- **It deletes the worst seam.** Editorial commits and paired contributions land *directly* in the one canonical place, which is also the live place. There is no "sync back up." GitHub only pulls *down*; a failed down-pull is just a stale mirror — no data loss.
- **GitHub stays useful and disposable** — a free, independently-hosted, publicly-auditable replica and integrity oracle. Lose it and you have lost a mirror, not the data. "We Own It" is preserved: canonical lives on the instance, on Foundry/customer-controlled infrastructure.

**Critical distinction — only CONTENT inverts, not the engine.** The engine code (`app-mediakit-knowledge`) still follows the normal Foundry `vendor → customer → deployment` flow with Stage 6 promotion. It is the three **content repos** (`media-knowledge-*`) whose canonical role moves onto the live instance. The engine is vendor-canonical; the content is instance-canonical.

### 4c. Naming — SETTLED

There is exactly **one content repo per wiki**. The repos `content-wiki-{documentation,projects,corporate}` are **renamed → `media-knowledge-{documentation,projects,corporate}`** so content repo, engine (`app-mediakit-knowledge`), and instance share one naming family. Same git history, same GitHub mirror; the canonical role moves onto the instance. **The operator is renaming the GitHub repos.** `project-editorial` commits refined TOPICs/GUIDEs into these repos as it does today — they are simply now both canonical and live.

---

## 5. The contribution model — pairing, not login

The web login/form/moderation-queue model is retired — a 2001 server-rendered artifact whose every premise is now false.

**The model: propose-as-branch / review-as-diff / commit-as-promotion** — the Foundry's own `commit-as-next.sh` → `promote.sh` flow turned outward as the product.

**Three roles** (the operator's "os-console pairs with os-mediakit", sharpened):

| Role | Is | Holds |
|---|---|---|
| **`os-console`** | the contributor's client — a secure terminal, not a website | the human identity; embeds the editor; signals F12 |
| **`os-mediakit`** | the pairing broker + promotion gate — the trust boundary | issues *pairings* (scoped, expiring capabilities — not accounts); enforces F12 |
| **`app-mediakit-knowledge`** | an identity-free git-native content substrate — the *resource* | nothing about identity |

**Five stages of a TOPIC change:** Propose (open a pairing → working branch) → Draft (in `os-console`, the editor, or an agent over MCP — writes to a *branch*, never `main`) → Attribute (provenance trailer: who, which AI model if any, the prompt, citations resolved) → Review (a branch diff over the read-only git remote; reviewer ≠ proposer; for AI-authored content the reviewer *must* be human) → Commit (F12 — an explicit human keypress merging to `main`).

Because the instance repo is canonical (§4b), an F12 promotion lands **directly in canonical, on the live server** — there is no upstream to sync to. The seam is gone.

**SYS-ADR-10 / SYS-ADR-19 become capability invariants, not review-time checks.** The broker mints two capability classes: *draft pairings* (branch-write — humans and agents alike) and *promotion capability* (the F12 merge — **human identities only, never an agent**). Humans and agents are equal as *proposers*, unequal as *publishers*.

### Main Page ownership

The **Main Page** (home page) of each wiki is owned by **`project-knowledge`** — not `project-editorial`.

The Main Page is structural and operational, not long-form prose. It decomposes into `index.md` / `index.es.md` (lede + welcome text), `featured-topic.yaml` (the pinned TOPIC), `leapfrog-facts.yaml` / `.es.yaml` (the "Did you know" rotation), and the category grid + slot order rendered by `home_chrome()`. Every one of those is tightly coupled to the engine and to each wiki's structure, and changes on a *maintenance* cadence — `project-knowledge`'s scope, not the editorial-batch cadence. Parking it on `project-editorial` overloads that gateway.

`project-editorial`'s role narrows to a **review pass on the Main Page lede prose** — the home page is the most-read page of each wiki, so its prose still clears the Bloomberg-standard / banned-vocabulary gate, the same as any public TOPIC. Under the contribution model (§5) this is clean: `project-knowledge` *proposes* the Main Page change as a branch; `project-editorial` is a natural *reviewer* of the lede prose (reviewer ≠ proposer). **Ownership here; prose review there.**

This is **build-phase** ownership. Long-term, a Customer-run instance curates its own Main Page — consistent with §3's customer-deployment shape. During the build, `project-knowledge` maintains all three.

---

## 6. Machine-readable layer

A TOPIC is simultaneously human prose and a queryable record. Four representations of the same canonical file:

| Representation | Consumer | Status |
|---|---|---|
| Rendered HTML + chrome | human browser | shipped |
| JSON-LD in `<head>` | crawlers, answer-engines | shipped — **enrich** (`dateModified`, `description`, `citation` from resolved `cites:`, `version`, `keywords`) |
| Raw canonical markdown / git remote | mirrors, auditors, ingest | shipped |
| Structured claim API | AI agents | **rebuild on MCP** — see §8 |

Add `GET /wiki/{slug}` JSON content-negotiation returning `{frontmatter, body_md, blake3, revision_sha, backlinks}` — the highest-value machine-readability win. Never make a machine parse a representation built for a human.

---

## 7. What the engine IS and ISN'T

**IS:** an identity-free, git-native content substrate. Render markdown; search; feeds/sitemap/llms.txt; read-only git smart-HTTP remote; the MCP claim+contribution API; derived projections. Two write affordances (MCP `propose_edit`, git push to a branch); one read affordance (anonymous HTTP).

**ISN'T:** an application platform. No in-engine accounts, no moderation-queue table, no real-time collab, no federation/blockchain/DID. Identity and the publish gate live in the `os-mediakit` broker. The disclosure-grade machinery in `INVENTIONS.md` (OpenTimestamps/Bitcoin anchoring, W3C VC, ActivityPub federation, adapter algebra) is **a different product** — correct for Woodfine corporate disclosure under BCSC, wrong for a public knowledge wiki. It moves to a future `project-disclosure` tenant.

---

## 8. MCP — reconciled (reverses the earlier "delete MCP" call)

Earlier this session the recon judged the MCP server redundant and we agreed to remove it. **That judgment was correct for the MCP server *as it exists* — and the vision reverses it.**

- The *current* MCP tools (`search_topics`, `get_revision`, `list_backlinks`) **are** redundant — they duplicate `/search`, `/git/{slug}`, `/whatlinkshere`. Delete those tools.
- MCP's *transport* is the right home for two things the platform genuinely needs: **(a) the structured claim API** (`query_claims(topic, asof)` → cited, freshness-tagged claim records, not prose); **(b) the contribution seam** (`propose_edit` → a branch with a provenance trailer).
- MCP is **not** a public anonymous endpoint. It is reached through the `os-mediakit` broker — a paired, brokered seam, never proxied publicly.

**Verdict: keep the MCP transport; delete the redundant read tools; rebuild it as the claim-query + contribution API behind the broker.** Not "delete the MCP server" — *re-found* it on its real purpose.

---

## 9. The Minimum Viable Leapfrog

`INVENTIONS.md` is over-engineered for a knowledge wiki. Strip the speculative machinery. The MVL is small deltas on the engine that already exists:

1. **Claim layer in the data model** — a `Claim` struct extracted from normally-authored markdown at render time (light inline convention, *not* a separate sidecar file — authoring friction kills it). Per-claim citation resolution. The redb graph gains a claim graph.
2. **Continuous citation verification** — background re-fetch + re-hash of cited sources; drift surfaced in the citation ribbon.
3. **Claim-record MCP API** — `mcp.rs` returns claim records, not prose (§8).
4. **Two-clock temporality** — `valid_at` + `published_at` per claim; the freshness ribbon reads from it; `?asof=` view from git history.

Everything beyond this is either a different product (disclosure substrate) or premature (the corpus is too small for claim-graph machinery to be visible yet).

---

## 10. What changes vs. the current engine

| Current feature | Disposition | Why |
|---|---|---|
| Markdown render, routing, search, feeds, sitemap, llms.txt, JSON-LD | **Keep** — core | The product. Enrich JSON-LD; add JSON content-negotiation. |
| Read-only git smart-HTTP server | **Keep** — re-justified | The contribution model gives it a job (reviewers inspect branch diffs over it) + sovereign clone path. |
| MCP server | **Keep transport, rebuild tools** | §8 — re-founded as claim API + contribution seam. |
| In-browser editor (`edit.rs`, CodeMirror, squiggles, citation autocomplete) | **Reposition** | Not a public engine route — becomes a component embedded by `os-console`. The SAA squiggle engine survives as a library. |
| Auth / accounts (`auth.rs`, `users.rs`) | **Remove from engine** | Identity moves to the `os-mediakit` pairing broker; engine keeps only pairing-token verification. |
| Pending-changes queue (`pending.rs`, SQLite) | **Remove** | Git branches replace the moderation table; review is a branch diff; accept = promotion. |
| Real-time collab (`collab.rs`, yjs) | **Remove** | Speculative; never deployed. |
| Doorman proxy stubs, dead `templates/*.html` | **Remove** | Dead. |
| OpenAPI spec | **Shrink** to surviving public read routes, or drop. |
| `INVENTIONS.md` Phase 7–9 (Bitcoin anchoring, VC, federation, adapter algebra) | **Move out** → future `project-disclosure` tenant. |
| One-binary-three-content-dirs multiplexing | **Replace** → three separate instances (§3). |
| Claim layer, continuous citation verification, two-clock temporality | **Add** (§9) — the actual leapfrog. |

Net: the engine gets **smaller and sharper** — a clean identity-free git-native substrate — *and* gains the claim layer that is the reason to leave Wikipedia.

---

## 11. Doctrine amendment required — note to MASTER

The §4b inversion conflicts with current Foundry Doctrine, which defines the tier flow `vendor → customer → deployments` with deployments as downstream **instances** ("no reverse writes"; local-only; gitignored — CLAUDE.md §10, AGENT.md). Making a deployment instance's **content repo** canonical inverts that.

**The ask to MASTER (Command Session):** amend the deployment-lifecycle / tier doctrine to permit a documented exception — *a knowledge-platform deployment instance's content repo (`media-knowledge-*`) is canonical for its content; GitHub is its downstream mirror.* This exception applies to **content only** — the engine code (`app-mediakit-knowledge`) and all other repos follow the normal `vendor → customer → deployment` flow with Stage 6 promotion unchanged.

Rationale to convey: this is the only model under which a Customer- or Community-deployed wiki instance genuinely owns its own content (sovereignty for the customer), and it eliminates the high-risk "edits-propagate-back-upstream" seam.

**Action:** stage this as an outbox message to `command@claude-code` once the operator confirms timing. A NEXT.md item should also record the doctrine-amendment dependency.

---

## 12. Decision status

**Settled in this session:**
- ✅ Build: custom Rust engine, continue `app-mediakit-knowledge` (§1).
- ✅ Deployment shape: three separate isolated instances (§3).
- ✅ Source-of-truth inversion: instance content repo is canonical, GitHub downstream (§4b).
- ✅ Naming: `content-wiki-*` → `media-knowledge-*` (§4c) — operator renaming GitHub repos.
- ✅ Doctrine amendment will be requested from MASTER (§11).
- ✅ Main Page ownership: `project-knowledge` owns each wiki's Main Page; `project-editorial` does a lede-prose review pass only (§5).
- ✅ project-editorial cross-check (§14): no conflict; the editorial standard adopted is the **Gate-0-reconciled** version, not the raw blueprint; the editorial linter is one ruleset / two consumers.

**Still open — to walk through one at a time:**
1. Adopt the claim-native data model (the MVL, §9)? The one irreducible new idea.
2. Adopt the pairing contribution model (§5)? Confirms removing `auth.rs` + `pending.rs` and repositioning the editor into `os-console` — real deletion of shipped Phase 2/5 work.
3. MCP (§8) — confirm the reversal: keep + re-found rather than delete.
4. `INVENTIONS.md` descope (§7, §10) — confirm Bitcoin/VC/federation move to a future `project-disclosure` tenant.
5. `os-console` / `os-mediakit` are both still `Hello, world!` scaffolds — the contribution model is real new scope in two other projects. Sequencing.
6. Immediate operational risk (unchanged): the production binary is ~16 commits behind canonical, Stage 6 blocked — clear regardless of any vision decision.

---

## 13. Relationship to existing plans

- This document is the **vision layer** — what we are building and why. It supersedes the Gen-2 blueprint (`MASTER-EXECUTION-BLUEPRINT.md` etc.).
- `KNOWLEDGE-PLATFORM-REBUILD-PLAN.md` (the 6-phase execution todo) is now **downstream of this** and must be reworked: Phase 5 (MCP) is rewritten by §8; a claim-layer phase enters (§9); the descope (§10) becomes a phase; the deployment-split (§3) becomes a phase; the contribution-model work spans `os-console`/`os-mediakit`, not just this engine.
- Next step after operator sign-off on the §12 open decisions: rework the rebuild plan against this vision.

---

## 14. Cross-check with project-editorial (2026-05-21)

This vision (rev 3) was cross-checked against project-editorial's live plan
`award-winning-wiki-overhaul.md` (msg-ids `project-knowledge-20260521-vision-crosscheck`
→ `project-editorial-20260521-vision-crosscheck-reply`).

**Verdict:** no item in this vision contradicts project-editorial's strategy.
The two plans operate at different altitudes — this document is the
*vision / architecture*; `award-winning-wiki-overhaul.md` is project-editorial's
*editorial execution* (its Track A). They complement; they do not compete.

**Accepted by project-editorial:**
- Main Page ownership (§5) — accepted; project-knowledge owns the artifact, project-editorial reviews the lede prose. project-editorial will offer recommended lede drafts as starting material.
- Contribution model (§5) — accepted.
- Source-of-truth inversion + `media-knowledge-*` rename (§4) — accepted as pending dependencies on the rename and the MASTER Doctrine amendment.

**Correction adopted — the editorial standard.** Where this vision and the
rebuild plan build a `validate_editorial_standards` capability, it adopts the
**Gate-0-reconciled** editorial standard ratified by the operator 2026-05-21 —
NOT the raw blueprint "Lucidity Protocol." The five reconciliations:
- expansion sentences ≤ ~45 words (not 60); disclosure prose keeps the 25-word discipline;
- prefer active verbs (present-fact mechanism only) — no absolute `is/are/was` ban, no personification;
- analogy is a ceiling (≤ one per 300 words), not a quota;
- the Bloomberg 4-paragraph lede stays as the nut graf; the Franklin arc governs body-section order only;
- the SaaS-marketing register ("Liquid Glass" etc.) is rejected for public content.
The canonical encoded ruleset is produced by project-editorial's Track A0 and routed to project-knowledge as a deliverable.

**Linter model — one ruleset, two consumers.** project-editorial's Track D
builds the canonical editorial ruleset + linter (`editorial-lint.py`). The
engine-side `validate_editorial_standards` consumes the *same* ruleset — no
second rule set. project-editorial routes the ruleset to project-knowledge.

**Open dependency flagged.** The design tokens moved since the earlier recon:
`wiki.*` was renamed → `knowledge.*` (2026-05-07 co-sign), freshness-ribbon
tokens were removed, and two competing `dtcg-bundle.json` files exist (`tokens/`
vs `dtcg-vault/tokens/`). The rebuild plan's token-wiring phase must use the
current `knowledge.*` bundle; project-design must declare which bundle is
canonical. Recorded here as a dependency.

**Cross-cluster plan synchronization.** Three living documents, layered, never
mirrored: this **vision** is the single upstream (architecture + cross-cluster
decisions — §12 is the decision ledger); `KNOWLEDGE-PLATFORM-REBUILD-PLAN.md`
(project-knowledge) and `award-winning-wiki-overhaul.md` (project-editorial) are
per-cluster **downstream execution plans** that *reference* the vision by
section number and never restate it. Material change that affects another
cluster propagates by **outbox message** — the channel used for this very
cross-check. Each cross-check appends a dated entry to this section.

*Synthesized 2026-05-21 from 4 OPUS research agents + operator design conversation. Agent reports archived in this session's transcript.*
