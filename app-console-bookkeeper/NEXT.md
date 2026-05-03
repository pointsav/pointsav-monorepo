# NEXT.md — app-console-bookkeeper

> Last updated: 2026-04-22
> Read at session start. Update before session end so the next
> session knows where to pick up.

---

## Right now

Nothing in progress — this is the activation-pilot commit. Real work
begins once the `service-bookkeeper` forward reference is resolved
and the initial data-binding target is confirmed.

## Queue

- **Resolve `service-bookkeeper` forward reference** — the view
  references a service not in the monorepo registry. Decide: add it
  as Reserved-folder, OR redirect the view text to
  `service-fs/data/`, OR correct the reference.
- **Add `README.md` and `README.es.md`** — bilingual compliance per
  repo-level rule.
- **Confirm the HTML-plugin pattern** — document it in the
  repo-level CLAUDE.md as the canonical pattern for `app-console-*`
  apps that are not Rust crates. Consider registry Type-column
  refinement to distinguish Rust-crate vs HTML-plugin `app-console`
  projects.
- **Wire data binding** — replace placeholder values (`$0.00`,
  "Awaiting Execution") with real values from the ledger feed.
  Depends on the forward-reference resolution.
- **Confirm initial deliverable** — minimum viable data display is
  the Q1 2026 Capital Deployment card. After that: posting form,
  reconciliation view, or Chart-of-Accounts browse/search.

## Blocked

- Data binding — Blocked on: `service-bookkeeper` forward-reference
  resolution.
- Initial deliverable prioritisation — Blocked on: operator
  discussion.

## Recently done

- 2026-04-22: Framework activation via `~/Foundry/CLAUDE.md` §8.
  State Reserved-folder (mis-classified) → Active, via a
  Scaffold-coded acknowledgement. Pilot of the activation procedure.
