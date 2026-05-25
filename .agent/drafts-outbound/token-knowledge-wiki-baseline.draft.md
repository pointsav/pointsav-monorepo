---
schema: foundry-draft-v1
state: draft-cosigned-pending-design-pass
originating_cluster: project-knowledge
target_repo: pointsav-design-system
target_path: tokens/
target_filename: dtcg-bundle.json (revision)
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-TOKEN-CHANGE
authored: 2026-04-30T01:15:00Z
authored_by: task-project-knowledge (Opus parent synthesis from Sonnet sub-agent D report)
authored_with: claude-opus-4-7
master_cosign: master-claude (workspace ~/Foundry/, session 90701278f84a1323) — 2026-04-30T17:00Z — operator ratified at chat surface
master_cosign_decisions: |
  Three governance decisions resolved at operator chat surface 2026-04-30:

  A. wiki.* namespace — APPROVED as `wiki.*` (not `ps-wiki.*`). Rationale:
     existing token substrate uses no vendor prefix at any tier (primitives,
     semantic, component). `ps-wiki.*` would be the only prefixed semantic
     namespace — an outlier. PointSav authorship is encoded in the design-system
     repo provenance, not in token names.

  B. FLI-banner colour — NEUTRAL (not amber). Rationale: the FLI banner is a
     regulatory-register process-discipline marker, not an error or warning
     condition. Amber communicates caution/danger in standard UI convention,
     which miscommunicates BCSC continuous-disclosure posture to readers. Goal
     is "informed reader," not "alarmed reader." project-design should achieve
     visual discoverability through structural differentiation (left-border
     accent, small disclosure icon) rather than status-colour alarm.

  C. font.family.heading — SYSTEM-STACK (not self-hosted variable font). Rationale:
     CLAUDE.md §6 states sovereignty over external CDN dependency. The design-system
     does not yet have a mature self-hosted asset pipeline. System-stack (Georgia /
     Times New Roman / serif) costs nothing, loads instantly, and is upgradeable to
     a self-hosted variable font when the asset-hosting infrastructure matures.
     Track as a v0.5.0+ NEXT.md item.
master_cosign_request: |
  Outbox to Master 2026-04-30 requests co-sign on this DESIGN-TOKEN-CHANGE before
  project-design refines and commits to pointsav-design-system. Master review scope:
  - The `wiki.*` semantic-namespace introduction (does brand-identity governance
    permit a tenant-facing public API namespace under PointSav's substrate?)
  - The 6 open questions in §6 — Master may pre-decide some before project-design
    gateway pickup, or defer all to project-design
  - The aspirational FLI-banner colour decision (amber vs neutral) — BCSC-posture
    governance scope
  [RESOLVED — see master_cosign_decisions above]
references:
  - clones/project-knowledge/.claude/drafts-outbound/research-wikipedia-leapfrog-2030.draft.md
  - clones/project-knowledge/.agent/drafts-outbound/archived/component-citation-authority-ribbon.draft.md (archived — Sprint F)
  - clones/project-knowledge/.claude/drafts-outbound/component-research-trail-footer.draft.md
  - clones/project-knowledge/.agent/drafts-outbound/archived/component-freshness-ribbon.draft.md (archived — Sprint F)
  - clones/project-knowledge/.claude/drafts-outbound/component-home-grid.draft.md
  - external:designtokens.org/tr/drafts/format/ (DTCG 2025.10 stable)
  - external:w3.org/community/design-tokens/2025/10/28/design-tokens-specification-reaches-first-stable-version/
  - external:v10.carbondesignsystem.com/guidelines/themes/overview/
  - external:v10.carbondesignsystem.com/guidelines/typography/overview/
  - external:v10.carbondesignsystem.com/guidelines/spacing/overview/
  - external:doc.wikimedia.org/codex/latest/style-guide/typography.html
  - external:doc.wikimedia.org/codex/v1.9.0/design-tokens/definition-and-structure.html
  - external:phabricator.wikimedia.org/T363849
  - external:mediawiki.org/wiki/Skin:Vector/2022/Design_documentation
  - conventions/cluster-design-draft-pipeline.md
notes_for_designer: |
  This token-change is large by design — it proposes a complete three-tier DTCG
  bundle covering primitives, semantic, and component tokens for the encyclopedic-
  knowledge-wiki surface. The substrate may already have partial coverage; project-
  design's gateway pass should DIFF this proposal against the existing dtcg-bundle.json
  rather than overwriting wholesale.

  The bundle is intentionally additive — no proposed token replaces an existing
  Carbon-derived primitive. The wiki-specific tokens (`wiki.*` semantic namespace
  and `home-*`, `article.*` component namespaces) are net-new additions that
  preserve all current substrate primitives unchanged.

  Sprint F update (2026-05-07): citation-authority-ribbon and freshness-ribbon
  DESIGN-COMPONENT drafts have been archived. Only two active DESIGN-COMPONENT
  recipes remain in this pickup batch: research-trail-footer and home-grid.
  The `article.freshness-ribbon` component token block has been removed from §4.

  Master co-sign is complete (2026-04-30T17:00Z). Three governance decisions
  resolved — see `master_cosign_decisions` above. Summary for project-design:

  1. `wiki.*` namespace is approved. Use `wiki.*` throughout; `ps-wiki.*`
     alternative is rejected.

  2. FLI-banner colour is NEUTRAL. Do not use amber (`status.warn.bg`). Use
     a neutral surface with a left-border accent and a small disclosure icon to
     achieve visual discoverability without alarm semantics.

  3. `font.family.heading` uses system-stack (Georgia / Times New Roman / serif).
     No self-hosted variable font at this stage. Remove any CDN font-load from the
     bundle; set `font.family.heading` to the system-stack fallback chain only.

  Project-design refines and commits to pointsav-design-system/tokens/dtcg-bundle.json
  with the three-tier hierarchy intact. The two active DESIGN-COMPONENT recipes in this
  same pickup batch reference these tokens — refine token-change first, then
  components. DIFF against existing dtcg-bundle.json rather than overwriting wholesale.
research_done_count: 8
research_suggested_count: 5
open_questions_count: 6
research_provenance: synthesized-from-research-draft + sub-agent-D-direct-research
research_inline: true
---

# DESIGN-TOKEN-CHANGE — knowledge-wiki baseline three-tier DTCG bundle additions

A DTCG (Design Tokens Community Group, 2025.10 stable specification) token-bundle revision that adds the three-tier vocabulary (primitive / semantic / component) the substrate needs to express the leapfrog-2030 encyclopedic-knowledge-wiki design language. Anchored to Carbon Design System v10 conventions and Wikimedia Codex precedent. Companion to four DESIGN-COMPONENT drafts in the same pickup batch — the token bundle is the foundation those component recipes reference.

This draft requires Master co-sign per `conventions/cluster-design-draft-pipeline.md` §3 (brand identity is governance scope). Outbox notification to Master 2026-04-30 with this draft's URL; project-design may not pick up at gateway until Master co-sign lands in the frontmatter `master_cosign:` field.

## 1. Three-tier taxonomy

The DTCG 2025.10 stable specification formalises the three-tier model the industry has converged on (https://www.designtokens.org/tr/drafts/format/):

- **Tier 1 — Primitives.** Named after their value, not their use. Colour stops are numeric (`color.neutral.700`); dimensions are ordinal (`space.4`); type sizes are positional (`font.size.3`). The raw palette. No semantic meaning encoded here.
- **Tier 2 — Semantic.** Named for *why*, not *what*. `surface.background` not `color.neutral.100`. References primitives via `{primitive.path}` alias syntax. The only tokens the visual design team negotiates. Dark-mode is implemented entirely at this tier.
- **Tier 3 — Component.** Named for the component and the design decision within it. `home-grid.gap` not `space.4`. References semantic tokens, never primitives directly. The only lever a deployment customises without touching the semantic layer.

Carbon Design System v10 uses the equivalent terms *global tokens* (primitives), *alias tokens* (semantic), and *component tokens*. PointSav inherits Carbon's colour stop conventions, spacing token vocabulary, and productive type-set philosophy. PointSav extends with wiki-specific semantic and component tokens that Carbon has no equivalent for.

## 2. Primitive tokens — additions

### `color.*`

Primitive colours not already in Carbon's `gray-*` and `blue-*` ranges. PointSav-specific:

```json
{
  "primitive": {
    "color": {
      "$type": "color",
      "brand": {
        "blue": {
          "50": { "$value": "#edf5ff" },
          "60": { "$value": "#0f62fe", "$description": "PointSav primary blue (Carbon interactive-01 equivalent)" },
          "70": { "$value": "#0043ce" },
          "80": { "$value": "#002d9c" }
        },
        "teal": {
          "50": { "$value": "#d9fbfb" },
          "60": { "$value": "#009d9a", "$description": "PointSav direct-source / featured-pin accent" }
        }
      },
      "link": {
        "default": { "$value": "#3366cc", "$description": "Wikipedia/Codex encyclopedic-standard link blue" },
        "visited": { "$value": "#795cb2" },
        "hover": { "$value": "#447ff5" },
        "active": { "$value": "#0043ce" },
        "redlink": { "$value": "#cc3333", "$description": "Wikipedia red-link convention — article not yet written" }
      }
    }
  }
}
```

The `color.link.default` value `#3366cc` aligns with Wikimedia Codex's `--color-progressive: #36c` (https://phabricator.wikimedia.org/T363849) and Wikipedia Vector 2022's link blue. Encyclopedic-register continuity.

### `font.family.*`

```json
{
  "primitive": {
    "font": {
      "family": {
        "$type": "fontFamily",
        "body": { "$value": "Linux Libertine, Georgia, Times, Source Serif 4, serif" },
        "heading": { "$value": "Linux Libertine Display, Charter, Tinos, system-ui-serif, serif" },
        "sans": { "$value": "IBM Plex Sans, Helvetica Neue, Arial, sans-serif" },
        "mono": { "$value": "IBM Plex Mono, Menlo, Consolas, Liberation Mono, Courier New, monospace" },
        "kbd": { "$value": "IBM Plex Mono, SFMono-Regular, Consolas, monospace" }
      }
    }
  }
}
```

Linux Libertine is Wikipedia's body serif (Apache 2.0 / OFL); strongly associated with encyclopedia-register cognition. IBM Plex (Apache 2.0) is Carbon's substrate — sans/mono inheritance is direct. Variable-font question deferred to §6 open questions.

### `font.size.*` — 14-stop scale

Anchored to 16px root (1rem = 16px). Carbon's mathematical interval; stops 1–10 identical to Carbon in em/rem terms. Wikipedia Vector 2022's body-size convergence at 16px after user research recommending 18px for "text-heavy" reading is honoured at `font.size.5`.

```json
{
  "primitive": {
    "font": {
      "size": {
        "$type": "dimension",
        "1": { "$value": "0.6875rem", "$description": "11px — fine print, legal" },
        "2": { "$value": "0.75rem", "$description": "12px — captions, footnote labels" },
        "3": { "$value": "0.875rem", "$description": "14px — footnote body, kbd, small" },
        "4": { "$value": "1rem", "$description": "16px — body copy default" },
        "5": { "$value": "1.125rem", "$description": "18px — reading-mode body" },
        "6": { "$value": "1.25rem", "$description": "20px — lead sentence / lede" },
        "7": { "$value": "1.5rem", "$description": "24px — H4, sub-section" },
        "8": { "$value": "1.75rem", "$description": "28px — H3" },
        "9": { "$value": "2rem", "$description": "32px — H2" },
        "10": { "$value": "2.25rem", "$description": "36px — H1 article title" },
        "11": { "$value": "2.625rem" },
        "12": { "$value": "3rem" },
        "13": { "$value": "3.75rem" },
        "14": { "$value": "4.25rem" }
      }
    }
  }
}
```

### `font.weight.*` + `line.height.*`

```json
{
  "primitive": {
    "font": {
      "weight": {
        "$type": "fontWeight",
        "light": { "$value": 300 },
        "regular": { "$value": 400 },
        "semibold": { "$value": 600 },
        "bold": { "$value": 700 }
      }
    },
    "line": {
      "height": {
        "$type": "number",
        "body": { "$value": 1.6, "$description": "Codex: 1.6× font size for reading paragraphs" },
        "tight": { "$value": 1.2 },
        "lede": { "$value": 1.45 },
        "kbd": { "$value": 1.4 },
        "caption": { "$value": 1.3 }
      }
    }
  }
}
```

### `space.*` — 8pt baseline grid

```json
{
  "primitive": {
    "space": {
      "$type": "dimension",
      "025": { "$value": "0.125rem", "$description": "2px — micro-spacing" },
      "05": { "$value": "0.25rem", "$description": "4px" },
      "1": { "$value": "0.5rem", "$description": "8px" },
      "2": { "$value": "1rem", "$description": "16px" },
      "4": { "$value": "2rem", "$description": "32px" },
      "8": { "$value": "4rem", "$description": "64px" },
      "16": { "$value": "8rem", "$description": "128px" },
      "32": { "$value": "16rem", "$description": "256px" }
    }
  }
}
```

### `radius.*`, `motion.*`, `density.*`

```json
{
  "primitive": {
    "radius": {
      "$type": "dimension",
      "none": { "$value": "0" },
      "xs": { "$value": "2px", "$description": "encyclopedia register — minimal" },
      "sm": { "$value": "4px" },
      "md": { "$value": "8px" }
    },
    "motion": {
      "duration": {
        "$type": "duration",
        "instant": { "$value": "0ms" },
        "fast": { "$value": "75ms" },
        "base": { "$value": "150ms" },
        "slow": { "$value": "300ms" }
      },
      "easing": {
        "$type": "cubicBezier",
        "standard": { "$value": [0.2, 0, 0.38, 0.9] },
        "accelerate": { "$value": [0.4, 0, 1, 1] },
        "decelerate": { "$value": [0, 0, 0.38, 0.9] }
      }
    },
    "density": {
      "$type": "number",
      "compact": { "$value": 0.75, "$description": "24px row grid (0.75 × 32px)" },
      "comfortable": { "$value": 1, "$description": "32px row grid (baseline)" },
      "spacious": { "$value": 1.5, "$description": "48px row grid (1.5 × 32px)" }
    }
  }
}
```

## 3. Semantic tokens — additions

### `surface.*` and `text.*`

```json
{
  "semantic": {
    "surface": {
      "$type": "color",
      "background": { "$value": "{primitive.color.neutral.10}" },
      "layer": { "$value": "#ffffff" },
      "layer-accent": { "$value": "{primitive.color.neutral.20}" },
      "layer-hover": { "$value": "{primitive.color.neutral.30}" },
      "inverse": { "$value": "{primitive.color.neutral.90}" }
    },
    "text": {
      "$type": "color",
      "primary": { "$value": "{primitive.color.neutral.100}" },
      "secondary": { "$value": "{primitive.color.neutral.70}" },
      "tertiary": { "$value": "{primitive.color.neutral.50}" },
      "placeholder": { "$value": "{primitive.color.neutral.40}" },
      "on-color": { "$value": "#ffffff" },
      "disabled": { "$value": "{primitive.color.neutral.30}" }
    }
  }
}
```

### `interactive.*` and `border.*`

```json
{
  "semantic": {
    "interactive": {
      "$type": "color",
      "link": { "$value": "{primitive.color.link.default}" },
      "link-visited": { "$value": "{primitive.color.link.visited}" },
      "link-hover": { "$value": "{primitive.color.link.hover}" },
      "link-redlink": { "$value": "{primitive.color.link.redlink}" },
      "focus-ring": { "$value": "{primitive.color.brand.blue.60}" },
      "button-primary": { "$value": "{primitive.color.brand.blue.60}" }
    },
    "border": {
      "$type": "color",
      "subtle": { "$value": "{primitive.color.neutral.20}" },
      "strong": { "$value": "{primitive.color.neutral.50}" },
      "interactive": { "$value": "{primitive.color.brand.blue.60}" },
      "disabled": { "$value": "{primitive.color.neutral.30}" }
    }
  }
}
```

### `wiki.*` — wiki-specific semantic namespace

This is the namespace Master and project-design ratify together (§6 question (b)). Proposed under `wiki.*` (tenant-facing public API); alternative is `ps-wiki.*` (PointSav-namespaced):

```json
{
  "semantic": {
    "wiki": {
      "$type": "color",
      "hatnote": {
        "bg": { "$value": "{semantic.surface.layer-accent}" },
        "border": { "$value": "{semantic.border.subtle}" }
      },
      "lead": {
        "bg": { "$value": "{semantic.surface.layer}" }
      },
      "toc": {
        "bg": { "$value": "{semantic.surface.layer-accent}" }
      },
      "references": {
        "bg": { "$value": "{semantic.surface.layer-accent}" }
      },
      "redlink": {
        "color": { "$value": "{semantic.interactive.link-redlink}" }
      },
      "editpencil": {
        "color": { "$value": "{semantic.text.tertiary}" }
      },
      "featured-pin": {
        "bg": { "$value": "{primitive.color.brand.teal.50}" },
        "accent": { "$value": "{primitive.color.brand.teal.60}" }
      }
    }
  }
}
```

## 4. Component tokens — additions

The four DESIGN-COMPONENT recipes in this batch reference these tokens directly. Component-token additions cover home-page composition (`home-grid.*`, `home-featured.*`, `home-recent.*`) and article-shell additions (`article.lead.*`, `article.toc.*`, `article.section.*`, `article.references.*`, `article.research-trail.*`, `article.fli-banner.*`, `article.density-toggle.*`, `article.freshness-ribbon.*`).

```json
{
  "component": {
    "home-grid": {
      "gap": { "$type": "dimension", "$value": "{primitive.space.2}" },
      "card-padding": { "$type": "dimension", "$value": "{primitive.space.2}" },
      "card-radius": { "$type": "dimension", "$value": "{primitive.radius.xs}" },
      "card-border-width": { "$type": "dimension", "$value": "1px" },
      "card-border-color": { "$type": "color", "$value": "{semantic.border.subtle}" },
      "card-surface": { "$type": "color", "$value": "{semantic.surface.layer}" },
      "breakpoint-3-to-2": { "$type": "dimension", "$value": "960px" },
      "breakpoint-2-to-1": { "$type": "dimension", "$value": "640px" },
      "card-min-height": { "$type": "dimension", "$value": "120px" }
    },
    "home-featured": {
      "accent-width": { "$type": "dimension", "$value": "4px" },
      "accent-color": { "$type": "color", "$value": "{semantic.wiki.featured-pin.accent}" },
      "padding": { "$type": "dimension", "$value": "{primitive.space.4}" },
      "lead-paraphrase-max-chars": { "$type": "number", "$value": 280, "$description": "Wikipedia TFA blurb is 909–1009 chars; 280 is shorter for documentation register" }
    },
    "home-recent": {
      "date-column-width": { "$type": "dimension", "$value": "6rem" },
      "date-font-family-mono": { "$type": "fontFamily", "$value": "{primitive.font.family.mono}" },
      "item-padding-y": { "$type": "dimension", "$value": "{primitive.space.1}" }
    },
    "article": {
      "lead": {
        "line-length-max-chars": { "$type": "number", "$value": 75, "$description": "Codex: no longer than 75 characters; literature consensus 45–75ch with 66 ideal" },
        "first-sentence-bold": { "$type": "number", "$value": 1, "$description": "Boolean as number per DTCG" },
        "paragraph-spacing": { "$type": "dimension", "$value": "{primitive.space.2}" }
      },
      "toc": {
        "collapsed-default": { "$type": "number", "$value": 0 },
        "position-left-rail": { "$type": "number", "$value": 1 },
        "max-depth": { "$type": "number", "$value": 3 },
        "indent-step": { "$type": "dimension", "$value": "{primitive.space.2}" },
        "bg": { "$type": "color", "$value": "{semantic.wiki.toc.bg}" }
      },
      "section": {
        "heading-edit-pencil-color": { "$type": "color", "$value": "{semantic.wiki.editpencil.color}" },
        "heading-edit-pencil-opacity-default": { "$type": "number", "$value": 0 },
        "heading-edit-pencil-opacity-hover": { "$type": "number", "$value": 1 }
      },
      "references": {
        "backlink-bracket-color": { "$type": "color", "$value": "{semantic.text.secondary}" },
        "citation-badge-academic-bg": { "$type": "color", "$value": "{primitive.color.brand.blue.60}" },
        "citation-badge-regulator-bg": { "$type": "color", "$value": "{primitive.color.status.success.base}" },
        "citation-badge-industry-bg": { "$type": "color", "$value": "{semantic.surface.layer-accent}" },
        "citation-badge-direct-source-bg": { "$type": "color", "$value": "{primitive.color.brand.teal.60}" },
        "footnote-arrow-rotation": { "$type": "number", "$value": 180 }
      },
      "research-trail": {
        "done-color": { "$type": "color", "$value": "{primitive.color.status.success.base}" },
        "suggested-color": { "$type": "color", "$value": "{primitive.color.brand.blue.60}" },
        "open-question-color": { "$type": "color", "$value": "{primitive.color.status.warn.base}" },
        "trail-bg": { "$type": "color", "$value": "{semantic.surface.layer-accent}" },
        "trail-border": { "$type": "color", "$value": "{semantic.border.subtle}" }
      },
      "fli-banner": {
        "bg": { "$type": "color", "$value": "{primitive.color.status.warn.bg}", "$description": "Open question §6(e) — amber currently; neutral alternative pending Master ratification" },
        "border": { "$type": "color", "$value": "{primitive.color.status.warn.base}" },
        "icon": { "$type": "color", "$value": "{primitive.color.status.warn.base}" },
        "padding": { "$type": "dimension", "$value": "{primitive.space.2}" }
      },
      "density-toggle": {
        "compact-multiplier": { "$type": "number", "$value": "{primitive.density.compact}" },
        "comfortable-multiplier": { "$type": "number", "$value": "{primitive.density.comfortable}" },
        "spacious-multiplier": { "$type": "number", "$value": "{primitive.density.spacious}" }
      },
    }
  }
}
```

## 5. Reference resolution chain — example

For the home-grid card surface:

```
component.home-grid.card-surface
  → semantic.surface.layer
    → "#ffffff" (explicit)

component.home-grid.card-border-color
  → semantic.border.subtle
    → primitive.color.neutral.20
      → "#e0e0e0" (Carbon-derived)

component.home-grid.gap
  → primitive.space.2
    → "1rem"
```

Component code references `--component-home-grid-card-surface`, `--component-home-grid-card-border-color`, `--component-home-grid-gap` only. No primitive token name appears in component CSS. Dark-mode override at the semantic tier substitutes `semantic.surface.layer` to `primitive.color.neutral.90` without touching any component code.

## 6. Open questions for Master + project-design ratification

(a) Light/dark theme switching pattern — Style Dictionary `@value` override / DTCG `$extends` override file / CSS `:root[data-theme="dark"]` selector. Implementation strategy decision; affects runtime bundle size and tenant-theme stacking.

(b) `wiki.*` vs `ps-wiki.*` semantic namespace prefix. Project-design ratifies `wiki.*` choice; **Master ratifies whether the wiki surface earns a tenant-facing public API namespace at all**.

(c) Variable-font loading vs system-stack discipline for `font.family.heading`. Sovereignty-vs-consistency tradeoff. **Master scope** per CLAUDE.md §6.

(d) Density toggle as token vs component state. Token approach is more portable across rendering environments; component-state approach keeps tokens cleaner.

(e) FLI-banner colour register — amber vs neutral. **Master scope** — BCSC posture interpretation. The FLI banner is a *process discipline* signal, not an *error condition*. Amber risks reader miscommunication; neutral may under-signal regulatory attention.

(f) Research-trail visual weight — chrome vs body content. Affects `--article-research-trail-trail-bg` resolution.

## 7. JSON-LD impact

The wiki engine emits JSON-LD per article (`<script type="application/ld+json">` in `<head>`). The token bundle does not modify JSON-LD emission directly, but the visual classes the bundle defines map 1:1 to JSON-LD `additionalType` and `category` annotations on the `TechArticle` schema. Substrate-design refinement of token names should preserve the `additionalType` semantics — those are what AI consumers parse.

Specifically:
- Research-trail subsection classes map to `potentialAction` `category` values — see `component-research-trail-footer.draft.md`

## Research trail

### Done

- [research-wikipedia-leapfrog-2030.draft.md §7] — six open questions for project-design ratification
- [external:designtokens.org/tr/drafts/format/] — DTCG 2025.10 stable specification
- [external:v10.carbondesignsystem.com/guidelines/themes/overview/] — Carbon three-tier model
- [external:v10.carbondesignsystem.com/guidelines/typography/overview/] — Carbon type scale 1–10 inheritance
- [external:doc.wikimedia.org/codex/v1.9.0/design-tokens/definition-and-structure.html] — Wikimedia Codex token taxonomy
- [external:phabricator.wikimedia.org/T363849] — Codex link-blue rationale
- [external:mediawiki.org/wiki/Skin:Vector/2022/Design_documentation] — Wikipedia 16/18px body-size convergence
- [Sub-agent D direct research] — full token inventory in 1500-word structured report

### Suggested

- [external:carbondesignsystem.com/elements/themes/tokens] — Carbon v11 token taxonomy diff (priority: high)
- [external:doc.wikimedia.org/codex/latest/style-guide/typography.html] — verify line-length and baseline conventions match Codex (priority: medium)
- [pointsav-design-system/research/] — verify no existing token-system research conflicts (priority: high)
- [external:styledictionary.com/info/dtcg/] — Style Dictionary's DTCG 2025.10 build-pipeline support (priority: medium)
- [pointsav-design-system/tokens/dtcg-bundle.json] — read current state at gateway pickup time (priority: critical)

### Open questions

The 6 questions in §6 above. All require ratification before refined output commits to the design system.

## Provenance

Synthesizes the four-agent parallel research with direct primary-source consultation of DTCG 2025.10, Carbon Design System v10, and Wikimedia Codex's published token taxonomy. The proposal is additive — every existing Carbon-derived primitive token in the substrate's current bundle is preserved unchanged; the wiki-specific tokens (`wiki.*` semantic; `home-*` and `article.*` component) are net-new additions.

Master co-sign request 2026-04-30 outboxed alongside this draft. Per `cluster-design-draft-pipeline.md` §3, project-design must not pick up at gateway until `master_cosign:` field reflects an active Master session ID.
