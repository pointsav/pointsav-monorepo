---
schema: foundry-draft-v1
state: draft-pending-language-pass
language_protocol: PROSE-GUIDE
originating_cluster: project-design
target_repo: woodfine/woodfine-fleet-deployment
target_path: <tbd-by-project-editorial>
target_filename: guide-design-system-get-started-designing.md
audience: customer-public
bcsc_class: current-fact
authored: 2026-05-08T00:00:00Z
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 2
research_suggested_count: 1
open_questions_count: 0
research_provenance: tacit
research_inline: true
notes_for_editor: |
  Vault stub is live at https://design.pointsav.com/designing/get-started/.
  Language pass: Bloomberg standard, no AI-product vocabulary, no hyperscaler
  comparisons by name. Add concrete examples where possible (sample token value,
  component tab description). Spanish translation deferred to TRANSLATE-ES pass.
  Cross-links use relative paths to the live vault; verify they resolve when the
  guide lands in its deployment target.
---

## Research trail

### Done — what informed this draft
- [tacit: vault stub designing/get-started.md] — skeleton content as source
- [tacit: vault load sequence in vault.rs + nav.rs] — what the vault actually loads
  (components, elements, tokens, research) vs what the nav exposes

### Suggested — what project-editorial should consult
- [pointsav-design-system/research/] — review current research files to cite
  concrete examples in the "Research" section if any exist (medium priority)

---

# Get Started — Designing with the PointSav Design System

The PointSav Design System gives your team a self-hosted vault of tokens,
components, and research files you own outright. There is no SaaS subscription,
no hosted service dependency, and no gatekeeper between your design decisions and
your Git repository.

This guide is for designers arriving at the system for the first time.

---

## What the vault contains

Your vault is a directory in a Git repository. When the substrate starts, it reads
the vault from disk and serves it at your domain. It does not call home.

The vault contains four types of content:

**Tokens** — named design decisions expressed in W3C DTCG format. Primitive tokens
(raw values: `color.primary-60.$value = "#234ed8"`) and semantic tokens (roles:
`interactive.primary` references `color.primary-60`). Available at `/tokens.json`
on your instance.

**Components** — HTML, CSS, and ARIA recipes for interface elements. Each component
has four tabs: Usage, Style, Code, and Accessibility. The recipe is the source;
your team's chosen framework consumes it.

**Elements** — foundational vocabulary (Color, Typography, Spacing, Motion) that
sits below the component layer. Elements express how the primitives are applied
system-wide, not just in individual components.

**Research** — design decision records explaining why a token value or component
behaviour was chosen. Research files back up the decisions; they are part of the
vault so they travel with the repository.

---

## Foundations — start here before components

The four foundational elements are the building blocks. Work through them in order.

**Color** — the primitive and semantic palette. Primitive colours are raw values
(`--ps-primary-60`, `--ps-neutral-10`). Semantic colours are named roles
(`--ps-interactive-primary`, `--ps-ink-primary`). Components reference semantic
colours only; primitives are below the line. [View Color →](/elements/color/overview/)

**Typography** — the type scale and font stack. Includes size steps, line-height
ratios, weight assignments, and the fallback stack used when a brand font is not
configured. [View Typography →](/elements/typography/overview/)

**Spacing** — a ten-step scale expressed as CSS custom properties
(`--ps-space-1` through `--ps-space-10`). Components and layout use only scale
steps; no arbitrary pixel values. [View Spacing →](/elements/spacing/overview/)

**Motion** — timing and easing conventions. Two speed tokens (`--ps-speed-1`,
`--ps-speed-2`) and two easing tokens (`--ps-ease-standard`,
`--ps-ease-utility`). Reduced-motion respect is built into component specs.
[View Motion →](/elements/motion/overview/)

---

## Importing tokens into your design editor

Tokens are published in W3C Design Tokens Community Group (DTCG) format at
`/tokens.json` on your instance. Most design editors support DTCG import directly
or via a plugin.

**Figma** — install the Tokens Studio plugin, point it at your instance's
`/tokens.json` URL, and sync. Primitive and semantic layers both import; semantic
tokens appear as aliases over primitives in Figma's panel.

**Penpot** — supports native DTCG import. Go to Assets → Design tokens →
Import, paste your `/tokens.json` URL, and confirm.

**Sketch** — install the Tokens Studio plugin and follow the same flow as Figma.

After import, your design tool will display the semantic token names rather than
raw hex values. When a token value changes in the vault, re-syncing updates all
components that reference it.

---

## Reading a component

Each component page has four tabs:

**Usage** — when to use the component, when not to, and how it fits into layouts.
Includes do/don't pairs.

**Style** — the visual anatomy of the component: which tokens drive which
properties, state variations, and responsive behaviour.

**Code** — the HTML recipe and associated CSS class names. Copy this to your
project as a starting point; the framework adapts the markup.

**Accessibility** — the ARIA pattern, keyboard behaviour, focus management
requirements, and WCAG 2.2 AA criteria this component satisfies.

[Browse components →](/components/overview/)

---

## Filing a design issue

If you find an error, a gap, or a decision you want to challenge, file an issue
on GitHub:

1. Go to the design system repository on GitHub.
2. Open a new issue with the label `design`.
3. Include: a screenshot, the component or token name, and the vault version
   (visible at `/healthz` on your instance under `version`).

Issues are the record of design decisions in progress. If you are working from a
forked vault, file issues on your fork first; upstream issues go to the vendor
repository.
