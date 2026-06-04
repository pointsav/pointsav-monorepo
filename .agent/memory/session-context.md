---
schema: foundry-session-context-v1
archive: project-design
---

# Session context — rolling 3-session summary

---

## 2026-06-04 | totebox | claude-code

**Done this session:**
- Processed inbox `command-20260603-wiki-institutional-redesign-master-cosig`:
  wiki institutional redesign DESIGN-TOKEN-CHANGE (master_cosign pre-populated by Command).
  Committed to design-system:
  - `--color-interactive: #0E3A66` added to theme-woodfine.css (Woodfine override);
    `#869FB9` default added to theme-generic.css
  - `body-base` 1.0625rem → 1.125rem (18px) in token-global-typography.yaml
  - `nav-minimum: 0.875rem` (14px) added to token-global-typography.yaml
  - `dtcg-vault/research/wiki-institutional-redesign.md` committed (already placed)
  - Commit: `d91ef76` (Peter Woodfine)
- Committed backlogged `elements/org-chart-tokens/overview.md` + `.es.md` — `82c4742` (Jennifer)
- Stage 6 signal sent to Command (msg-id: project-design-20260604-stage6-signal)

**Pending / carry-forward:**
- Stage 6: design-system has 7 commits on `origin-staging-j` awaiting promote.sh from Command.
  Signal in outbox: `project-design-20260604-stage6-signal`.
- `origin-staging-p` (pwoodfine) push fails with publickey error in jennifer environment — Command to push or investigate.
- Binary rebuild for app-privategit-design sidebar nav (org-chart-tokens as top-level entry) still
  operator-pending. Source not in canonical — see outbox `project-design-20260603-design-binary-rebuild-request`.

**Operator preferences surfaced:**
- Responds quickly and directly; prefers short answers with clear next-action state.
- "can we have both colors for now" = include both teal AND red using Carbon-native values; defer institutional variants.

---

## 2026-06-03 | totebox | claude-code

**Done this session:**
- Startup + session lock.
- Processed 3 DESIGN-* drafts from project-orgcharts (`command-20260602-relay-orgcharts-design-drafts`):
  - `0e6f37e`: DESIGN-TOKEN-CHANGE — `--wf-teal: #005D5D`, `--wf-teal-tint: #9EF0F0`,
    `--wf-red: #A2191F`, `--wf-red-tint: #FFB3B8` added to theme-woodfine.css
  - `aca9646`: DESIGN-RESEARCH — bencal green value drift research file committed
  - `252a035`: DESIGN-COMPONENT — org-chart-node-pill teal + grey modifiers in nodes.css
    + component guide + research file
- Green token decision: operator confirmed `--wf-green: #198038` (Carbon Green 70) canonical.
  Command updated token in design-system (`0197d4f`). WCP charts reverted to `#198038`.
- All inbox messages actioned; ACK sent to project-orgcharts outbox.

**Pending / carry-forward:**
- Stage 6 for the 5 design-system commits from this session (covered by later Stage 6 signal).
- `elements/org-chart-tokens/overview.md` + `.es.md` were untracked — committed next session.

**Operator preferences surfaced:**
- Confirmed Carbon Green 70 as canonical Woodfine green by saying "let us use the wf green"
  (meaning the existing CSS variable, not the Carbon value) — then externally reversed to Carbon Green.
  Decision record in `dtcg-vault/research/research-orgchart-green-value-decision.md`.
