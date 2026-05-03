# CLAUDE.md — app-console-bookkeeper

> **State:** Active  —  **Last updated:** 2026-04-22
> **Registry row:** `../.claude/rules/project-registry.md`
>
> When state changes, update this header AND the registry row in the
> same commit. Drift between the two is a documentation defect.

---

## What this project is

An `app-console-*` surface application for bookkeeping operations on
ConsoleOS. Runs on the `os-console` host.

Implementation style — **HTML-cartridge plugin**. ConsoleOS renders
the outer view (`view-bookkeeper.html`) and mounts an inner cartridge
(`src/cartridge.html`) that carries the data-bound UI. This is the
HTML-plugin pattern used by ConsoleOS surface apps, distinct from the
Rust-crate pattern used by siblings like `app-console-content` and
`app-console-people`.

Scope — recording, posting, reviewing, and reconciling financial
transactions against the CorporateArchive. Intended as the operator
interface for the Chart-of-Accounts ledger, maintained under
SYS-ADR-10 (F12 human-checkpoint discipline).

## Current state

**Scaffold-coded → Active (activation pilot, 2026-04-22).** Corrects
a registry mis-classification: this row was initially marked
`Reserved-folder` during today's registry bootstrap, but the directory
actually contains HTML-plugin content. Activated and reclassified in
the same commit.

Current content:

- `view-bookkeeper.html` — outer plugin view. 8 lines. Displays
  "Financial Ledger" header, subtitle referencing the 1.2 Interest
  Coverage Ratio, Capital Deployment card with "Awaiting
  service-bookkeeper sync..." placeholder.
- `src/cartridge.html` — inner cartridge. 889 bytes. Renders a
  Q1 2026 Capital Deployment card (`$0.00`, "Awaiting Execution").

Both files use the design-system's CSS custom properties
(`--wf-slate`, `--wf-border`, `--wf-accent`, `--wf-muted`,
`--font-sans`, `--font-mono`). No data binding yet; placeholder
values throughout.

No build step. No tests. No `Cargo.toml` — this is not a Rust crate.

## Forward reference — `service-bookkeeper`

The view reads "Awaiting `service-bookkeeper` sync..." but that
service is NOT in the monorepo registry as of 2026-04-22. Either:

- It is planned and needs to be added as a Reserved-folder row
  before any real integration work; OR
- The real data source is `service-fs/data/service-bookkeeper/`
  (archive-only, no service crate); OR
- The reference is wrong and the view text needs correcting.

Resolve before wiring data binding. Tracked as first Queue item.

## Build and test

No build step — HTML files are served as-is by the ConsoleOS
rendering layer. Testing today is visual verification in the
ConsoleOS plugin host.

Future validation: design-system token coverage (every `var(--*)`
resolves), placeholder removal once data binding lands, accessibility
checks on rendered output.

## File layout

```
app-console-bookkeeper/
├── CLAUDE.md                 — this file
├── NEXT.md                   — current work queue
├── view-bookkeeper.html      — outer view
└── src/
    └── cartridge.html        — inner data cartridge
```

No `README.md` / `README.es.md` yet. Bilingual READMEs pending —
first Queue item.

## Hard constraints — do not violate

1. **Operator-initiated only.** Every write to the CorporateArchive
   ledger goes through F12 per SYS-ADR-10. No batch import; no AI
   auto-posting.
2. **Chart-of-Accounts canonicalisation.** Account codes conform to
   the canonical Chart maintained in the CorporateArchive.
3. **Structured-data path.** Per SYS-ADR-07, ledger data never routes
   through AI. Deterministic parsing only.
4. **Audit trail.** Every posting produces a cryptographically sealed
   record in the Totebox audit ledger.
5. **Design-system tokens only.** Every CSS value references a
   design-system custom property. No hardcoded colours, fonts, or
   spacing.
6. **No inline data in HTML.** All values are data-bound from the
   ledger feed at render time. Do not hardcode dollar amounts, dates,
   or account codes.

## Dependencies on other projects

- **Consumes:** the ConsoleOS plugin host (mounting mechanism),
  whichever service provides the bookkeeping ledger feed
  (`service-bookkeeper` once resolved, or direct `service-fs` reads).
- **Consumed by:** `app-console-minutebook` (board-minutes reference
  ledger postings), any `media-marketing-*` that aggregates portfolio
  summaries.
- **Design-system:** `pointsav-design-system/tokens/` provides the
  `--wf-*` custom properties.

## Commit convention

`app-console-bookkeeper: <what changed>` — short, focused. One
concern per commit.

## What not to do

- Do not add AI processing to the posting path. SYS-ADR-07 violation.
- Do not bypass F12. Every posting is an operator action.
- Do not hardcode dollar amounts or dates in the HTML.
- Do not introduce a Rust crate here. This project is HTML-plugin;
  if Rust logic is needed, it lives in a companion
  `service-bookkeeper` crate, not inside `app-console-bookkeeper`.

---

## Inherited rules — do not duplicate, do not silently override

This project inherits rules from two parent scopes. Do NOT copy their
content into this file; reference them.

- **Repo-level:** `../CLAUDE.md` — prefix taxonomy, canonical names,
  ADR hard rules (SYS-ADR-07, -10, -19), Do-Not-Use vocabulary,
  bilingual README rule, BCSC / Sovereign Data Foundation disclosure.
- **Workspace-level:** `~/Foundry/CLAUDE.md` — identity store, commit
  flow (`tool-commit-as-next.sh`), promotion flow
  (`tool-promote.sh`), authoritative-document priority, rules of
  engagement.

If a rule at this level conflicts with an inherited rule, **stop and
surface the conflict** — do not silently override.
