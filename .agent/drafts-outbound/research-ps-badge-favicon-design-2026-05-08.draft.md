---
schema: foundry-draft-v1
state: draft-pending-design-pass
language_protocol: DESIGN-RESEARCH
originating_cluster: project-design
target_repo: pointsav/pointsav-design-system
target_path: research/
target_filename: ps-badge-favicon-design.md
audience: vendor-internal
bcsc_class: current-fact
authored: 2026-05-08T00:00:00Z
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 4
research_suggested_count: 2
open_questions_count: 1
research_provenance: tacit
research_inline: true
notes_for_designer: |
  This is a design decision record, not a proposal — the PS badge is already
  live at design.pointsav.com (committed a02027e). The record exists so the
  decision is traceable and can be revisited when the broader brand-token layer
  for the Design System is formally authored.
  Open question for project-design: hex literal vs CSS custom property for the
  fill value — see research trail below.
  The SVG file itself is staged as a separate ASSET draft:
  asset-favicon-ps-badge-svg-2026-05-08.draft.md
---

## Research trail

### Done — what informed this design
- [app-privategit-design/src/render.rs:30] — production implementation
  (inline SVG data URI embedded in the <link rel="icon"> tag)
- [tacit: render.rs prior session] — initial iteration used `y='72'`; operator
  reported the glyph appeared malformed (bottom-heavy) in the browser tab;
  iterated to `y='50'` + `dominant-baseline='central'`
- [tacit: W3C SVG dominant-baseline spec] — `central` aligns the midpoint of
  the em box to the specified y coordinate; this centres the cap-height visually
  regardless of OS font metric differences
- [vault/tokens/primitive.json color.primary-60] — `#234ed8` is the confirmed
  PointSav Blue primitive token value at time of design

### Suggested — what project-design should consult before finalising
- [pointsav-media-assets/tokens/] — verify `#234ed8` is still the canonical
  primary-60 value after the admin-tier token restructure (commit 30fefe6);
  the colour must stay in sync (high priority)
- [external: developer.apple.com/library/archive/documentation/
  AppleApplications/Reference/SafariWebContent/pinnedTabs/pinnedTabs.html] —
  Safari pinned-tab mask icon uses a separate `<link rel="mask-icon">` SVG
  with monochrome rules; the PS badge may need a mask-icon variant (low priority)

### Open questions — for future passes
- Should the SVG fill value `#234ed8` be kept as a literal hex in the data URI,
  or noted as an alias for `--ps-primary-60`? Data URIs cannot resolve CSS custom
  properties at runtime, so the hex is required in context. However, if
  `--ps-primary-60` changes, the favicon hex must be updated manually. This
  coupling should be documented as a maintenance note in the substrate.
  → project-design decision on how to handle the maintenance note.

---

# PS Badge — Favicon Design Decision Record

## What was designed

The PointSav Design System substrate (`app-privategit-design`) uses an inline SVG
data URI as its favicon: a blue square with white "PS" lettering.

The production data URI (embedded at `render.rs` line 30):

```
data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><rect width='100' height='100' rx='12' fill='%23234ed8'/><text x='50' y='50' dominant-baseline='central' text-anchor='middle' fill='%23ffffff' font-family='Arial,sans-serif' font-weight='700' font-size='40'>PS</text></svg>
```

The unencoded SVG source:

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
  <rect width="100" height="100" rx="12" fill="#234ed8"/>
  <text x="50" y="50" dominant-baseline="central" text-anchor="middle"
        fill="#ffffff" font-family="Arial,sans-serif"
        font-weight="700" font-size="40">PS</text>
</svg>
```

## Design parameters and rationale

**Format: inline SVG data URI**
SVG scales to any DPI without a separate high-resolution asset. The data URI
eliminates an additional HTTP request per page load and removes a file that
would need to be provisioned at every deployment. No build step, no static
file path, no CDN dependency.

**viewBox: 0 0 100 100**
A square canvas in round-number units simplifies the geometry. All measurements
(rx, font-size, x, y) are expressed as percentages of 100, making the design
intent legible without a design tool.

**rect rx="12"**
A 12% corner radius produces a rounded-square shape consistent with iOS
app icon conventions and contemporary favicon practice. The radius is large
enough to read at 16×16 px (the minimum browser tab size) without disappearing.

**fill="#234ed8" (PointSav Blue)**
`#234ed8` is the value of `color.primary-60` in the primitive token bundle.
This is `--ps-interactive-primary` in the semantic layer — the same blue used
for primary interactive elements across the design system. The favicon shares
its colour with the system's primary action affordance; there is no second blue.

**text y="50" dominant-baseline="central"**
The initial implementation used `y="72"` as the text baseline. At this position
the top of the letterforms was approximately centred but the descender area
extended below the visual midpoint, making the badge appear bottom-heavy in the
browser tab. The `dominant-baseline="central"` attribute instructs the browser to
align the midpoint of the em box (not the baseline) to `y="50"`. This produces
optical centring across the major desktop and mobile operating systems, regardless
of system font metric differences.

**font-size="40"**
At 40 units in a 100-unit box, the "PS" text occupies approximately 70% of the
badge width with ~15 units of horizontal margin on each side. An earlier value
of 44 brought the glyphs within ~6 units of the `rx=12` corner curve, producing
a cramped appearance at small sizes. 40 is the largest value that maintains
breathing room at 16×16 px.

**font-family="Arial,sans-serif"**
Data URI SVGs cannot reference web fonts or system-ui tokens that resolve
differently across platforms. Arial is present on all major desktop and mobile
operating systems and produces consistent letterforms at small sizes.
`system-ui` was considered and rejected: it resolves to different typefaces on
macOS (SF Pro), Windows (Segoe UI), and Android (Roboto), producing different
"PS" proportions and weights across platforms.

**font-weight="700"**
Bold weight at 40px on a 100-unit canvas. At 16×16 px the strokes read at
roughly 2 device pixels each on a non-retina display; lighter weights would
drop below the readable threshold.

## Maintenance note

The fill value `#234ed8` is a literal hex in the data URI. CSS custom properties
(`--ps-primary-60`) cannot be referenced inside data URIs because they are not
in a CSS parsing context. If `color.primary-60` changes in the primitive token
bundle, the favicon hex must be updated manually in `src/render.rs` line 30 at
the next rebuild. This is a known coupling.

## Status

Live at design.pointsav.com as of commit `a02027e` (2026-05-08).
The standalone SVG file is staged for commit to `pointsav-media-assets/icons/`
pending master-tier access decision (see outbox 2026-05-08).
