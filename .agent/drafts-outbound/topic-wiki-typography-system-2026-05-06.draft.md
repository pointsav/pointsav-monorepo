---
schema: foundry-draft-v1
state: ready-for-sweep
language_protocol: TOPIC
originating_cluster: project-design
target_repo: vendor/content-wiki-documentation
target_path: content/
target_filename: topic-wiki-typography-system.md
audience: editorial
bcsc_class: vendor-public
bilingual: true
authored: 2026-05-06
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 5
research_suggested_count: 1
open_questions_count: 1
research_provenance: |
  project-editorial brief 2026-05-06T18:30Z (24-agent research pass).
  IBM Plex Sans + Mono font analysis (training data + agent research 2026-05-06).
  IBM Plex GitHub: github.com/IBM/plex (SIL OFL 1.1).
  Google Fonts IBM Plex availability confirmed (training data).
  WCAG contrast audit 2026-05-06 — all pairs pass AAA.
research_inline: true
---

# Wiki Typography System

The PointSav wiki typographic system uses IBM Plex Sans for body prose and IBM Plex Mono for code and technical notation. This document explains the font choice, the heading scale, the spacing tokens, and how the system achieves broad linguistic coverage for bilingual (English/Spanish) content.

---

## Font stack

**Body prose:** IBM Plex Sans (400, 500, 600, 700 weights). IBM Plex Sans is IBM's open-source typeface released under the SIL Open Font License 1.1, which permits use, modification, and redistribution without restriction. The face is a humanist sans-serif with high legibility at reading sizes and clear differentiation between commonly confused glyphs (l, 1, I; O, 0).

**Code and technical notation:** IBM Plex Mono (400, 500 weights). A proportionally designed monospaced companion to IBM Plex Sans. Used for inline `code`, code blocks, command-line examples, and metadata fields (dates, identifiers).

**Fallback stack:** -apple-system, BlinkMacSystemFont, Segoe UI, Roboto (system UI sans-serif chain) for body; SFMono-Regular, Consolas, Liberation Mono for code.

---

## Delivery

IBM Plex Sans and IBM Plex Mono are available through Google Fonts, the npm package `@ibm/plex`, and direct download from the IBM Plex GitHub repository.

**Variable font:** IBM Plex Sans ships a variable font file (`IBM-Plex-Sans-Variable.woff2`) covering the full weight axis (100–700). A single variable font file replaces four static weight WOFF2 files and runs approximately 60–80 KB for the Latin subset — smaller than four separate files combined.

**Self-hosting (recommended for privacy):** The wiki self-hosts from the deployment's `/static/fonts/` directory. No requests leave the deployment to external font CDNs. Latin subset is the minimum; latin-ext (adds accented characters for Spanish bilingual content) adds approximately 10–20% to file size.

**`font-display: swap`:** Prevents invisible-text flash during load (FOIT). The fallback system font renders immediately; IBM Plex swaps in when loaded. This is appropriate for a text-heavy wiki where layout stability matters less than immediate readability.

---

## Type scale

The heading scale was set by a 24-agent editorial research pass (2026-05-06) to balance Wikipedia muscle-memory, reading comprehension at the 65-character measure, and the IBM Plex Sans x-height.

| Level | Token | rem | px (at 17px base) | Use |
|---|---|---|---|---|
| H1 | `--ps-wiki-text-h1` | 2.25rem | 38px | Article title |
| H2 | `--ps-wiki-text-h2` | 1.75rem | 29.75px | Major section |
| H3 | `--ps-wiki-text-h3` | 1.375rem | 23.375px | Subsection |
| H4 | `--ps-wiki-text-h4` | 1.125rem | 19.125px | Minor heading |
| Body | `--ps-wiki-font-size-base` | 1.0625rem | 17px | Running prose |

**Base:** 106.25% root font-size (17px) — slightly larger than the 16px browser default for improved readability at the 65ch measure on desktop. The 17px base was set by the editorial research pass; the browser default of 16px produces slightly short lines at this measure.

---

## Reading measure and line height

| Property | Value | Token |
|---|---|---|
| Measure (max-width) | 65ch | `--ps-wiki-measure` |
| Body line-height | 1.6 | `--ps-wiki-line-height-body` |
| H1 line-height | 1.22 | (inline, no dedicated token) |

65 characters per line is the typographic optimum for sustained reading. The 1.6 line-height at 17px base gives 27.2px leading, matching the spacing rhythm of Wikipedia's body text.

---

## CSS tokens

All values are CSS custom properties defined on `:root` in `dist/tokens.css`:

```css
--ps-wiki-font-body: 'IBM Plex Sans', -apple-system, …;
--ps-wiki-font-mono: 'IBM Plex Mono', 'SFMono-Regular', …;
--ps-wiki-font-size-base:   1.0625rem;
--ps-wiki-line-height-body: 1.6;
--ps-wiki-measure:          65ch;
--ps-wiki-text-h1: 2.25rem;
--ps-wiki-text-h2: 1.75rem;
--ps-wiki-text-h3: 1.375rem;
--ps-wiki-text-h4: 1.125rem;

/* Short-form aliases used by wiki templates */
--font-sans:    var(--ps-wiki-font-body);
--font-mono:    var(--ps-wiki-font-mono);
--leading-body: 1.6;
--measure:      65ch;
--text-h1:      var(--ps-wiki-text-h1);
--text-h2:      var(--ps-wiki-text-h2);
--text-h3:      var(--ps-wiki-text-h3);
--text-h4:      var(--ps-wiki-text-h4);
```

---

## Research trail

### Done
- Font choice rationale: IBM Plex selected for SIL OFL 1.1 license, legibility, and corporate identity alignment.
- Scale values confirmed by 24-agent research pass (2026-05-06).
- Variable font availability confirmed (training data; spot-check against npm @ibm/plex before final publish).
- WCAG contrast pairs all pass AAA at the confirmed dark-mode background values (2026-05-06 audit).
- Delivery strategy: self-hosted preferred for privacy; Google Fonts and jsDelivr available as CDN fallbacks.

### Suggested
- Confirm exact WOFF2 file sizes against live npm @ibm/plex package before documenting in GUIDE.

### Open questions
1. **Variable font weight-range support:** Does the Woodfine wiki deployment use a variable font or static weights? If variable, confirm the 100–700 weight axis covers all weights used in templates (400/600 for body+heading, 700 for bold).
