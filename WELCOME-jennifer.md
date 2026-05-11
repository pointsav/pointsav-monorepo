# Welcome to project-bookkeeping — Jennifer

This memo is for you to read when you first open the
project-bookkeeping cluster. It's three pages. Read it once;
keep it open if you want to refer back.

---

## What this cluster is

You are bringing real bookkeeping work to Foundry. A series of
Excel files. The procedural knowledge of what you do with them.
The classifications, the journal entries, the reconciliations,
the month-end close, the year-end work.

You are NOT here to use a finished bookkeeping product. You are
here to help **build** the bookkeeping logic that PointSav will
eventually ship as software (`service-bookkeeper`,
`app-console-bookkeeper`, `app-workplace-accounting`). The
software comes later. Your work comes first. Your work IS the
spec.

This is unusual. Most software is built by product managers
asking "what do bookkeepers want?", then engineers building it,
then bookkeepers being trained on it. We're inverting that.
You do real bookkeeping. We capture what you do. The eventual
software is written to do what you actually do — not what
someone guessed you might want.

The strategic name for this is in the technical docs: "operator-
driven development with domain-expert apprenticeship." In
plain language: **you are the master craftsperson; Task Claude
is the apprentice.** The apprentice watches, asks questions,
remembers, and gradually takes on more of the work as the
operations become known.

---

## How it works in practice

When you open a Claude Code session inside
`/srv/foundry/clones/project-bookkeeping/`, Task Claude greets
you and asks something like: "What's the first Excel file
you'd like to work on, and what would you normally do with it?"

You open the file. You start working — exactly as you would
without anyone watching. Task Claude observes:

- Which file you opened
- Which sheet you're looking at
- Which rows you're classifying
- What category you assign
- What journal entry you write
- What reconciliation check you run

When something isn't obvious from observation, Task Claude
asks. For example:

> "I noticed you classified row 14 as `accounts-receivable`
> rather than `prepaid-revenue` — what tells you it's that?"

You answer in your own words. Task Claude captures your answer
verbatim — not paraphrased, not summarized — into a structured
log file. Over time, those captured answers become the
classification rules the eventual software will use.

This means your sessions look more like **conversations** than
**software use**. Task Claude won't push buttons for you. Task
Claude will ask, observe, suggest, and ask again. You are still
doing the bookkeeping.

---

## What goes where

Three places hold your work:

### Excel files

You bring them. They live first in `inputs/` of the deployment
instance:

```
/srv/foundry/deployments/cluster-totebox-corporate-3/inputs/
```

You upload an Excel file there (or Task Claude can fetch one
you point to). Once captured, it moves to:

```
/srv/foundry/deployments/cluster-totebox-corporate-3/vault/source/<sha256>.xlsx
```

The vault is **immutable**. Once a file lands there, it never
changes. If you need to edit a sheet, you create a new version
+ new SHA-256 + new vault entry. The original stays. This is
how auditors get an unbroken chain of custody on your books.

### Journal entries (your postings)

When you decide a row in an Excel sheet should produce a
journal entry, that entry lands in:

```
/srv/foundry/deployments/cluster-totebox-corporate-3/vault/ledger/<YYYY>/<MM>.jsonl
```

One file per month. Append-only. Each entry references which
Excel source row it came from + your reasoning. This becomes
the canonical bookkeeping ledger for the Woodfine tenant.

### Operations log (for training the future software)

Every operation you do — classify, post, reconcile, close,
correct — emits a structured event to:

```
/srv/foundry/data/training-corpus/apprenticeship/operations/woodfine/<id>.jsonl
```

This is where the **trajectory of your work** lives. The
software we eventually ship will be trained on these
operations. So when you're working, you're not just doing
this month's books — you're also writing the spec for next
year's bookkeeping software.

---

## Why this matters strategically

Hyperscaler bookkeeping software (QuickBooks, Xero, Sage)
captures 62% of the US SMB market. They built their products
from product hypotheses. Then they sold those products to SMBs.
Then bookkeepers learned to fit their work into the products'
shapes. The result: software shaped by what executives thought
bookkeepers should do, not by what bookkeepers actually do.

PointSav is doing the opposite. Real bookkeeper. Real Excel
files. Real operations. The captured trajectory becomes the
spec. Software written from that spec matches the work it
serves — because it was written FROM the work it serves.

The result: SMB customers get accounting software that does
what bookkeepers actually do. Auditors get a customer-rooted
attestation chain (the immutable vault + cryptographic
signing + Sigstore Rekor anchoring) that no hyperscaler SOC 2
report can match. PointSav builds a competitive moat in a
market hyperscalers cannot structurally replicate.

This pattern is in the technical docs as Doctrine claim #36
(The Data Vault Bookkeeping Substrate). The cluster you're
working in is its first operational instantiation.

---

## What goes through service-slm — and what doesn't

Mathew has been clear: across all our project-* clusters,
everything Claude does should transit `service-slm` (the
Doorman). That gives us:

- **Per-tenant audit ledger** — every AI call is recorded with
  who/what/when/why
- **Tier routing** — local OLMo 3 7B for cheap classifications;
  bigger models only when the work needs them
- **Sovereignty** — your bookkeeping data doesn't leak across
  Anthropic conversation contexts; it stays inside your
  tenant's audit boundary

For your cluster specifically:

| Path | Transits Doorman? |
|---|---|
| Trajectory capture (JSONL operations log) | ✅ Yes — already today |
| Post-commit shadow briefs (apprenticeship corpus) | ✅ Yes — already today |
| Live conversation with Task Claude during your session | ⏳ Soon — `service-slm` audit-routing endpoints (`/v1/audit_proxy` + `/v1/audit_capture`) are queued to ship in the project-slm cluster's next milestone (PS.4). Once those endpoints land, every Task Claude LLM call you make transits Doorman by default. Until then, your work is captured in the trajectory log (which is the audit story); the live LLM call to Anthropic happens via Claude Code directly. |

You don't need to do anything to participate in the Doorman
routing — when PS.4 ships, your existing sessions automatically
flow through it. We'll send you a note when it's live.

---

## What to expect for the first few sessions

**First session (15-30 minutes recommended):**
- Task Claude greets you + asks which Excel file to start with
- You pick a small one — maybe a single client's invoice
  tracker, or one month of expense receipts
- You walk through it as if Task Claude weren't there. Open the
  file. Look at the sheet. Decide a classification.
- Task Claude observes + asks 2-4 clarifying questions
- End of session: Task Claude writes a session summary you can
  review

**Sessions 2-10:**
- You bring more Excel files. Different shapes. Different
  conventions.
- Task Claude starts noticing patterns — "this looks like the
  type of thing we classified as X last week"
- You correct Task Claude when it's wrong. Those corrections
  are the most valuable training signal.
- A schema starts to emerge: chart of accounts, classification
  rules, reconciliation patterns

**Sessions 10+:**
- The operational schema is concrete enough that Task Claude
  can suggest classifications + journal entries before you do
- You review + accept or correct
- The acceptance/correction loop becomes the Stage-2 DPO
  training signal (this is the "preference learning" the
  technical docs talk about)

**Months 2-3:**
- The spec is concrete. We start scaffolding
  `service-bookkeeper`. The first version of the software
  AUTOMATES what you've been doing. It doesn't invent new
  behavior.
- Your role gradually shifts from "doing every operation" to
  "reviewing automation suggestions + handling exceptions"

This is a long arc. There's no pressure to get to the end fast.
The more time spent capturing operations carefully, the better
the eventual software fits real work.

---

## Practical notes

**How to start:**

```bash
cd /srv/foundry/clones/project-bookkeeping
claude
```

Task Claude opens. Greet it. It will read its inbox + this
memo and proceed.

**Where things live:**

- Cluster scripts + manifest: `.claude/`
- Sub-clones (engineering source): `pointsav-monorepo/`,
  `woodfine-fleet-deployment/`
- Your Excel uploads: `/srv/foundry/deployments/cluster-totebox-corporate-3/inputs/`
- Your captured ledger: `/srv/foundry/deployments/cluster-totebox-corporate-3/vault/ledger/`
- Operations log:
  `/srv/foundry/data/training-corpus/apprenticeship/operations/woodfine/`

**Who to ask when something's confusing:**

1. Ask Task Claude in the session — it knows the cluster
   context
2. If Task Claude is stuck or surfaces a Master-tier question,
   it sends an outbox message to Mathew automatically. You
   don't have to translate.
3. For anything personal (login issues, Excel uploads not
   appearing, suspicious behavior), text Mathew directly

**Privacy + isolation:**

Your bookkeeping data is **strictly tenant-isolated**. The
PointSav side (engineering) never sees your financial data at
training time. Only the operational schema (the *patterns*
of what bookkeepers do) gets generalized into the eventual
software. The actual amounts, vendor names, customer details
stay inside your tenant's vault on this VM.

---

## Why we're glad you're here

Bookkeepers know things about real-world money flows that
nobody else knows. Engineers can write the world's cleanest
data structures and still ship software that doesn't fit how
books actually get done. Product managers can interview a
hundred bookkeepers and still miss the tacit knowledge that
makes the work go.

The only way to capture that tacit knowledge is to watch
someone do the work. Patiently. With curiosity. Asking real
questions. Capturing the answers as the person says them, not
as the watcher would have phrased them.

That's what this cluster does. You bring the work. We capture
the trajectory. The eventual software is written from what we
captured. SMB customers later get bookkeeping software shaped
by a real bookkeeper's real operations — not by a product
manager's hypothesis.

Welcome.

— Master Claude (on behalf of Mathew + the workspace)
2026-04-28
