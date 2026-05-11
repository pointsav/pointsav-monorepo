---
schema: foundry-draft-v1
state: ready-for-sweep
originating_cluster: project-bim
target_repo: vendor/pointsav-design-system
target_path: components/<various>
audience: design-system
bcsc_class: vendor-internal
language_protocol: DESIGN-COMPONENT
authored: 2026-04-29T00:40:00Z
authored_by: task-cluster-project-bim
authored_with: claude-opus-4-7
research_done_count: 5
research_suggested_count: 0
open_questions_count: 1
research_provenance:
  - cluster sub-agent BB.13 — `~/Foundry/clones/project-bim/.claude/sub-agent-results/BB.13-design-system-showcase-survey-2026-04-28.md`
  - cluster sub-agent BB.14 — `~/Foundry/clones/project-bim/.claude/sub-agent-results/BB.14-design-system-website-pick-2026-04-28.md`
  - workspace-tier sub-agent A — `~/Foundry/.claude/sub-agent-results/A-bim-design-system-prior-art-2026-04-28.md`
  - cluster CSS implementation — `~/Foundry/clones/project-bim/pointsav-monorepo/app-orchestration-bim/src/style.css`
  - cluster JS implementation — `~/Foundry/clones/project-bim/pointsav-monorepo/app-orchestration-bim/src/script.js`
research_inline: true
notes_for_editor: |
  Per operator framing 2026-04-28: "we should be sending back all the
  COMPONENTS used to project-design with research for future development."
  This is the components-flowback handoff for the v0.0.2 BIM showcase.

  Filter applied — flow back ONLY domain-agnostic patterns. BIM-specific
  components (BimSpatialTree, BimPropertiesPanel, BimViewport3D,
  BimClassificationChip, BimCodeOverlayPanel, IsometricBuildingMassHero,
  BimGuidLabel) stay in pointsav-design-system/components/bim-* on the
  cluster/project-bim branch — they only make sense for AEC contexts.

  Nine generic patterns flow to project-design here. project-design Task
  picks up via cluster-design-draft-pipeline.md sweep — each may become
  its own DESIGN-COMPONENT-<name>.draft.md if project-design prefers
  per-component granularity. The reference implementations live in the
  cluster's CSS + JS files (see research_provenance).
---

# Generic component flowback — 9 patterns from BIM showcase v0.0.2

The cluster v0.0.2 visual upgrade authored nine domain-agnostic
component patterns inside the BIM showcase. Each is potentially
useful at any future PointSav design-system surface (the META
substrate at design.pointsav.com, downstream SMB customer instances,
future verticals). Per the operator's components-flowback policy,
these are surfaced to project-design META-substrate Task here.

The reference implementations (HTML structure + CSS + ARIA + JS
behavior where applicable) live at the cluster path

```
~/Foundry/clones/project-bim/pointsav-monorepo/app-orchestration-bim/src/{style.css,script.js,render.rs}
```

at commit (will be) `<sha>` on `cluster/project-bim` once this draft is
swept and the cluster baseline lands.

## The nine patterns

### 1. SidebarAccordion

Collapsible categorised left-rail navigation. Sections labelled with
small monospace caps; items as horizontally-padded links with an
active state that adds a 2px left border in the brand accent and a
soft accent-tint background.

| Element | Class | Source |
|---|---|---|
| Container | `.bim-sidebar` / `.bim-sidenav` | style.css §4 |
| Section heading | `.bim-sidenav__heading` | style.css §4 |
| Item | `.bim-sidenav__link` (+ `--active`) | style.css §4 |
| Empty-state caption | `.bim-sidenav__empty` | style.css §4 |

Universal applicability — any showcase / docs / dashboard surface
with categorised navigation benefits.

### 2. CodeBlockWithCopy

Pre-formatted code block with an inset copy-to-clipboard button that
fades in on hover. JS handler binds to `data-bim-copy` attribute.
Copy uses `navigator.clipboard.writeText` with a textarea fallback for
restricted contexts. "copied" indicator shows for 1.4s then reverts.

| Element | Class | Source |
|---|---|---|
| Container | `.bim-code-block` | style.css §9 |
| Copy button | `.bim-code-block__copy` (+ `--copied`) | style.css §9 |
| JS binding | `bindCopy()` in script.js §1 | script.js |

Universal applicability — every documentation surface needs this.

### 3. ChipRow

Inline horizontal chip group with semantic variants. Each chip carries
a small monospace label + value, rendered as a single inline-flex
element with constrained height and consistent border-radius.

| Variant | CSS class | Use case |
|---|---|---|
| Default | `.bim-chip` | Generic categorisation chip |
| Brand-accent | `.bim-chip--ifc` | Primary semantic anchor |
| Cyan | `.bim-chip--uniclass` | Classification-domain chip |
| Neutral | `.bim-chip--mode` | Mode-prop / state chip |
| Amber-active | `.bim-chip--codes-active` | Warning / regulation-attached |
| Success | `.bim-chip--success` | Verified / pass state |

The label/value split (small monospace label tag prefix + value) is
the distinctive pattern. Project-design at META-substrate level may
want a generic `.ps-chip` that takes BIM-specific variants in via
naming conventions.

### 4. TabBarDisclosure

Tabs implemented as `<details>` elements grouped in a container,
progressively enhanced by JS to behave as a tabset (opening one
closes others, deep-link via `#fragment`). Zero-JS baseline still
works — all panels are accessible via expand/collapse.

| Element | Class | Source |
|---|---|---|
| Bar container | `.bim-tab-bar` | style.css §7 |
| Tab | `details.bim-tab` (+ `[open]`) | style.css §7 |
| Summary | `.bim-tab__summary` | style.css §7 |
| Panel | `.bim-tab__panel` | style.css §7 |
| JS coordination | `bindTabBar()` in script.js §3 | script.js |

The disclosure-as-tabs pattern is broadly useful for documentation
sites where most users want to land on the default tab but power
users want to deep-link to specific tabs.

### 5. PreviewFrame (with light/dark toggle)

Bordered surface for previewing component recipes. Carries a `data-theme`
attribute on the container (`light` default, `dark` for 3D-context
components). Toolbar with two toggle buttons in the top-right; JS
flips `data-theme` and updates `aria-pressed`. CSS rules under
`.bim-preview[data-theme="dark"]` override the surface palette.

| Element | Class | Source |
|---|---|---|
| Frame | `.bim-preview` (+ `[data-theme]`) | style.css §8 |
| Toolbar | `.bim-preview__toolbar` | style.css §8 |
| Toggle button | `.bim-preview__toggle` (+ `[aria-pressed]`) | style.css §8 |
| JS binding | `bindPreviewTheme()` in script.js §2 | script.js |

The pattern is BIM-flavored in the cluster (dark mode reads as 3D
viewport chrome), but the underlying mechanic — themed preview frame
with light/dark toggle — is universally applicable.

### 6. BreadcrumbNav

Chevron-separated path navigation (` / ` separator generated as `::after`
content, not as an inline glyph — accessible). Each crumb is a link
to the named ancestor; the current page is implied by being the
content header below the breadcrumb (not duplicated in the crumb
list).

| Element | Class | Source |
|---|---|---|
| Container | `.bim-crumbs` | style.css §5 |
| List | `.bim-crumbs__list` | style.css §5 |
| Item | `.bim-crumbs__item` (with `::after`) | style.css §5 |
| Link | `.bim-crumbs__link` | style.css §5 |

Universal applicability.

### 7. EmptyStateCard

Dashed-border card with a heading + body paragraph + optional inline
links. Used for "no data yet" surfaces (the v0.0.2 BIM Code overlays
index uses it). Convention: title in display-serif, body in body-sans,
links separated by visible whitespace (not bullets).

| Element | Class | Source |
|---|---|---|
| Container | `.bim-empty-state` | style.css §14 |
| Title | `.bim-empty-state__title` | style.css §14 |
| Body | `.bim-empty-state__body` | style.css §14 |
| Link row | `.bim-empty-state__links` | style.css §14 |

Universal applicability — every showcase / dashboard with potentially
empty surfaces needs this.

### 8. MachineSurfaceFooter

Three-column footer with brand identity (left), machine surface links
(`/tokens.json`, `/components`, `/research`, `/healthz`, etc), and
substrate provenance (Doctrine claims, standards floor). Convention:
small monospace section headings (`.bim-footer__heading`), 11px caps;
body in `.bim-footer__list` at 14px; canonical-URL base bar
underneath in monospace at 12px muted.

| Element | Class | Source |
|---|---|---|
| Container | `.bim-footer` | style.css §17 |
| Inner grid | `.bim-footer__inner` | style.css §17 |
| Section heading | `.bim-footer__heading` | style.css §17 |
| List | `.bim-footer__list` | style.css §17 |
| Base bar | `.bim-footer__base` | style.css §17 |

Universal applicability — every PointSav-shape showcase has the
"machine surface" pattern (DTCG bundle, MCP endpoint, registry, etc).

### 9. EditOnGitHubLink (NOTE: not yet implemented in v0.0.2)

The cluster's BIM showcase v0.0.2 does not yet ship an Edit-on-GitHub
link the way design.pointsav.com does. The generic pattern is well
understood — link in page footer pointing to the source markdown of
the current page on GitHub. Naming this here as a planned addition
to inform project-design's META-substrate.

| Element | Class | Source |
|---|---|---|
| Link | `.bim-edit-link` (proposed) | not yet authored |

Skip from v0.0.2 implementation in BIM showcase pending operator
direction; project-design META-substrate version may land first.

## Coordination ask to project-design Task

1. Review the nine patterns above. Accept all as-is, or split into
   per-component DESIGN-COMPONENT drafts at your sweep cadence.
2. Where useful, generalise into META-substrate `ps-` prefixed
   components in `pointsav-design-system/components/` (e.g.,
   `components/sidebar-accordion/`, `components/code-block-with-copy/`).
3. Open follow-up cross-cluster outbox messages if patterns need
   refinement before Stage-6 promotion.

The cluster does NOT block on project-design's response; the BIM
showcase v0.0.2 ships with these patterns as cluster-internal
implementations. When project-design lands generalised versions in
the META-substrate, the BIM showcase can refactor to consume the
generalised forms (or stay specific — the patterns are stable).

## Research trail

### Done

- Each pattern was implemented in the cluster's `style.css` + `script.js` and verified live on port 9099.
- Cross-walked against design.pointsav.com's existing patterns (`.ps-sidenav`, `.ps-tabs`, `.ps-prose`, `.ps-footer`) — most have direct project-design analogs, suggesting the generalisation path is well-trodden.
- Cross-walked against Adobe Spectrum (BB.14 reference shape) — the patterns are Spectrum-shape compatible.
- BB.13 + BB.14 sub-agent reports document the design-system-website convention space and the AEC-domain overlay markers.
- Operator confirmed components-flowback policy 2026-04-28 ("we should be sending back all the COMPONENTS used to project-design").

### Open question

- Should project-design's generic versions adopt the `bim-` namespace's class naming conventions (e.g., `.ps-chip__label` mirroring `.bim-chip__label`) or invent a parallel naming? Recommendation: consistent class-naming convention across both substrates makes downstream consumer code (any tenant surface that uses both) cleaner. Project-design Task chooses.
