# NEXT.md — project-design

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-08 [totebox@claude-code]

---

## Pending Command Session (Stage 6 / push / infra)

- [ ] **Stage 6 — pointsav-design-system** — main at `5209e46` (2 commits: `57de61a` org-chart
      components + `5209e46` token-magenta/teal/red removal). Outbox:
      `project-design-20260608-stage6-design-system-57de61a`.
      Procedure: push `--force-with-lease` to both staging mirrors → `promote.sh`.
      [2026-06-08 totebox@claude-code]

- [ ] **Push — woodfine-media-assets** — main at `a752b21` (2 commits: `3336d8f` org-chart palette
      + `a752b21` magenta/teal removal). Outbox:
      `project-design-20260608-push-woodfine-media-assets-3336d8f`.
      Procedure: `git push origin main` (woodfine-administrator SSH alias).
      [2026-06-08 totebox@claude-code]

- [ ] **Commit nginx-design.conf** — sub_filter org-chart-tokens sidebar injection already live in
      `/etc/nginx/sites-available/design.pointsav.com`. Outbox:
      `project-design-20260608-nginx-sub-filter-commit`.
      [2026-06-08 totebox@claude-code]

- [ ] **Vendor elements/ sync** — `vendor/pointsav-design-system/elements/org-chart-tokens/overview.md`
      still has token-magenta/teal/red entries. Deployment already correct + live.
      Outbox: `project-design-20260608-vendor-elements-orgchart-token-removal`.
      Also investigate: elements/ absent from design-system sub-clone (flow anomaly).
      [2026-06-08 totebox@claude-code]

---

## Operator action required

- [ ] **master_cosign — two blocked DESIGN-TOKEN-CHANGE drafts** — both in
      `clones/project-orgcharts/.agent/drafts-outbound/`:
      - `DESIGN-TOKEN-CHANGE-orgchart-primitives.draft.md` — `primitive.color.orgchart` namespace
      - `DESIGN-TOKEN-CHANGE-orgchart-layout-type.draft.md` — `component.orgchart` namespace
      Set `master_cosign: "2026-06-08T<time> jwoodfine"` in both to unblock.
      [2026-06-08 totebox@claude-code]

---

## Pending inbox items (next session)

- [ ] **wp-tokens DESIGN-TOKEN-CHANGE** — 27 DTCG tokens from project-workplace (`.wp-*` namespace);
      master_cosign already populated. Inbox: `project-design-20260607-relay-design-token-change-wp-tokens-2026`.
      Draft at: `clones/project-workplace/.agent/drafts-outbound/DESIGN-TOKEN-CHANGE-wp-tokens-20260602.draft.md`
      [2026-06-07]

- [ ] **DESIGN-wiki-institutional-redesign** — 3 token changes (--color-interactive, body 18px, nav 14px);
      master_cosign: command@claude-code 2026-06-03 (already populated).
      Inbox: `project-design-20260607-relay-wiki-institutional-redesign-master`.
      Draft at: `clones/project-knowledge/.agent/drafts-outbound/DESIGN-wiki-institutional-redesign.draft.md`
      [2026-06-07]

- [ ] **manifest.md cluster: field** — add `cluster: project-design` to `.agent/manifest.md`.
      Two inbox messages pending (command + proforma), low priority.
      [2026-06-08]

---

## Design token open questions

- [ ] **`--wf-teal` / `--wf-red` Woodfine-institutional variants** — current values are
      IBM Carbon-native (#005D5D / #A2191F). Tokens removed from org-chart system 2026-06-08.
      Institutional variants deferred — revisit when Bencal chart authoring surfaces a need.
      [2026-06-08 totebox@claude-code]

- [ ] **`--wf-gold` discrepancy** — `theme-woodfine.css` has `#C89211`; `MEMO-Woodfine-Color-Matrix.md`
      documents `#F57F17` as 9-chart majority canon. Semantic YAML uses `#C89211`. Reconcile.
      Noted in `tokens/charts/token-chart-semantic.yaml`. [2026-06-02 totebox@claude-code]
