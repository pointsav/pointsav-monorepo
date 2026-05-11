# Woodfine marketing site — IA component map

> Single source of truth for which UI component lives on which page.
> Resolves the wireframe-numbering inconsistency Jennifer flagged in
> the hand sketch ("Disclaimer being 7 instead of 16. Move logo to
> point a instead of point b.")
>
> Status: ratified by operator 2026-05-09 alongside the
> website-congruence-plan.md. Edit only when adding a new page or
> a new component to the shared shell.

## Numbering scheme — same numbers, every page

Every Woodfine marketing page uses the same component numbers. A
component that doesn't render on a given page simply isn't present;
its number isn't reused for something else.

| # | Component | Where it renders | Anchor / class | Notes |
|---|---|---|---|---|
| 1 | Logo / wordmark | All pages (header centre) | `header.topnav .wordmark` | Woodfine institutional SVG; centred between left and right nav |
| 2 | Disclaimer link (header) | All pages (header left, slot 1) | `header.topnav .left a[href="/page/disclaimer"]` | Same-tab — internal page |
| 3 | Contact us link (header) | All pages (header left, slot 2) | `header.topnav .left a[href="/page/contact"]` | Same-tab — internal page |
| 4 | Corporate link (header) | All pages (header right, slot 1) | `header.topnav .right a[href="https://corporate.woodfinegroup.com/"]` | `target="_blank"`; live subdomain (MediaWiki) |
| 5 | Projects link (header) | All pages (header right, slot 2) | `header.topnav .right a[href="https://projects.woodfinegroup.com/"]` | `target="_blank"`; live subdomain (MediaWiki) |
| 6 | Newsroom link (header) | All pages (header right, slot 3) | `header.topnav .right a[href="https://newsroom.woodfinegroup.com/"]` | `target="_blank"`; subdomain in build (Phase 4 of plan) |
| 7 | Page hero (H1 band) | Inner pages only (Disclaimer, Contact, Newsroom; not Landing) | `.page-hero` | All-caps display font, centred; one line |
| 8 | Subpage main (article surface) | Inner pages only | `main.subpage-main` | White card on canvas |
| 9 | Cities band (footer left) | All pages | `footer.footer .cities` | "Vancouver \| New York" — serif |
| 10 | Footer nav (footer right) | All pages | `footer.footer .footnav` | Contact us / Disclaimer — display font, small caps |
| 11 | Copyright line | All pages | `.copyright` | Below footer |
| 12 | Side ToC | Newsroom only (Phase 4) | TBD `.sidebar-ledger` | Right-rail; date / source / version filters |
| 13 | Date/Month filter | Newsroom only (Phase 4) | TBD `.newsroom-filter` | Above the press-release list |
| 14 | NewsArticle JSON-LD | Newsroom only (Phase 4) | `<script type="application/ld+json">` in `<head>` | Schema.org markup; copied from pointsav.github.io template |

## Same-tab vs new-tab

| Component | Behaviour | Reason |
|---|---|---|
| 2, 3 (Disclaimer, Contact us) | **same tab** | Inner pages of the same property; investors should be able to scan disclosure / contact without losing the marketing context |
| 4, 5, 6 (Corporate, Projects, Newsroom) | **new tab** (`target="_blank" rel="noopener"`) | Subdomain-isolated CMSes (MediaWiki, MediaWiki, RSS-reader); each is its own property; `↗` glyph appended via CSS to signal this |

## Header layout — logo-centre, on every page

```
┌──────────────────────────────────────────────────────────────────┐
│  DISCLAIMER  CONTACT US        [WOODFINE LOGO]        CORPORATE↗  PROJECTS↗  NEWSROOM↗
└──────────────────────────────────────────────────────────────────┘
```

- Grid: `1fr auto 1fr` — left links / wordmark / right links.
- Display font on links (Oswald, uppercase, 0.16em letter-spacing).
- Wordmark: 320 × 80 px at desktop; scales down to 160 × 40 px at the
  480px breakpoint per shell.css.

Sketches V2 (Ian Kiprono) showed two layouts (logo-centre on Landing,
logo-left on Corporate / Newsroom / Documentation). Operator chose
**logo-centre everywhere** 2026-05-09. The Arch-Linux-style
logo-left precedent the hand sketch cited works for distros, not for
capital-projects firms — ratified 2026-05-09.

## Footer — same on every page

```
┌──────────────────────────────────────────────────────────────────┐
│  Vancouver | New York                              CONTACT US     │
│                                                    DISCLAIMER     │
├──────────────────────────────────────────────────────────────────┤
│  © 2026 Woodfine Capital Projects Inc. All rights reserved.       │
└──────────────────────────────────────────────────────────────────┘
```

- Footer is the disclosure anchor — the BCSC posture line eventually
  belongs here too (planned for a follow-up sprint).
- Cities serif on left; footnav display-font on right.
- Copyright line beneath footer.

## What's *out* of this map

- Sub-product surfaces (GIS Location Intelligence, BIM Tokens) get
  their own utility shell — see Phase 5 of website-congruence-plan.md.
  They are *not* wrapped in this header/footer.
- PointSav landing (deployment 2) — separate cluster scope; PointSav
  pages mirror this map structurally but with PointSav tokens
  (slate-blue accent, etc.).

## Wireframe references

- `~/sandbox/inputs/project-marketing/website/Wireframe sketches V2- Ian Kiprono.pdf`
- `~/sandbox/inputs/project-marketing/website/www.woodfinegroup.com hand sketches.pdf`

## Implementation references

- `clones/project-marketing/templates/_shell-header.html`
- `clones/project-marketing/templates/_shell-footer.html`
- `clones/project-marketing/templates/shell.css`
- `clones/project-marketing/docs/website-congruence-plan.md` — full plan

## Component drafts (DTCG-substrate)

- `clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/components/component-marketing-topnav.draft.md` — header recipe (Phase 1b: rewrite to match this map)
- `clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/components/component-marketing-footer.draft.md` — footer recipe (Phase 1c: author)
- `clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/components/component-marketing-page-hero.draft.md` — page-hero recipe (Phase 1c: author)
- `clones/project-marketing/.claude/drafts-outbound/leapfrog-2030/assets/asset-woodfine-wordmark-svg.draft.md` — wordmark asset (Phase 1c: author)
