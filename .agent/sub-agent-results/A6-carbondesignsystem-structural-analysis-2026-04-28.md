---
agent: sub-agent-A6
task: carbondesignsystem.com structural analysis
authored: 2026-04-28
sources:
  - https://carbondesignsystem.com/
  - https://github.com/carbon-design-system/carbon-website (nav-items.yaml, gatsby-config.js, MDX sources)
  - https://github.com/carbon-design-system/gatsby-theme-carbon (component source)
  - https://medium.com/carbondesign/carbons-new-information-architecture-531c52207205
---

# Carbon Design System — Structural Analysis

Reverse-engineered from live site, GitHub source (carbon-website repo), and
gatsby-theme-carbon component source. Extracted 2026-04-28.

---

## Top Navigation Surface

**Composition (left to right):**
1. Carbon logo (wordmark) — links to homepage
2. Primary nav is NOT in the top header bar. The header carries: logo, search
   icon button (opens inline search), GitHub link (icon), an IBM "ecosystem
   switcher" (9-dot grid icon — opens a flyout panel listing IBM Design
   properties: IBM Brand Center, IBM Design Language, Carbon for IBM Products,
   Carbon for Cloud, Enterprise Design Thinking, IBM Accessibility, etc.)
3. No theme toggle in the top header. No version selector in the header.
4. GitHub link: present as a GitHub icon in the top-right header area, links to
   `github.com/carbon-design-system/carbon`.

**Stickiness:** The header is sticky (fixed to top of viewport, full width).

**Mobile collapse:** At breakpoints below ~1056px, the sidebar becomes a
drawer (overlay). The header remains. A hamburger/menu icon in the header opens
the sidebar drawer. The switcher icon and search remain in header.

**No primary nav items across the top.** All section navigation lives in the
left sidebar. The top bar is minimal: logo + search + GitHub + switcher.

---

## Left Sidebar

**Source:** `src/data/nav-items.yaml` (248 lines). 13 top-level categories,
106 total pages.

**Top-level categories (in order from the YAML):**

1. All about Carbon (4 pages: What is Carbon?, Who uses Carbon?, The Carbon
   ecosystem, Releases)
2. What's happening (2 pages: News and articles, Meetups) — has `hasDivider: true`
3. Designing (3 pages: Get started, Design kits, Other resources)
4. Developing (7 pages: Get started, Frameworks, Carbon MCP, Developer resources,
   React tutorial, Web components tutorial, Community frameworks)
5. Contributing (4 pages: Get started, Product Development Lifecycle, Component
   checklist, Documentation)
6. Migrating (2 pages: Guide, FAQs) — has `hasDivider: true`
7. Elements (8 pages: 2x Grid, Color, Icons, Pictograms, Motion, Spacing, Themes,
   Typography)
8. Guidelines (3 pages: Accessibility, Carbon for AI, Content)
9. Components (44 pages: Overview + 43 named components from Accordion through
   UI shell variants)
10. Patterns (18 pages: Overview + 17 pattern types)
11. Community assets (2 pages: Overview, Domain guidance)
12. Data visualization (10 pages: Getting started, chart types, palettes,
    axes, legends, dashboards)
13. Help (3 pages: FAQs, Certificate of Originality, Contact us)

**Sub-navigation pattern:** Expand/collapse accordion per category. The active
category auto-expands (`defaultExpanded={isActive}`). Inactive categories are
collapsed. Single-item categories render as a flat `SideNavLink`; multi-item
categories render as a `SideNavMenu` with child links.

**Active page indication:** CSS class `currentItem` applied to the matching nav
item. Active state is determined by comparing the first pathname segment to the
item's path slug.

**Sidebar width:** Uses Carbon's SideNav component — standard width is 256px
(16rem). Sticky, full-height, independent scroll from main content. Scroll
position is preserved on navigation via stored `scrollTop` state.

**Mobile behavior:** Below ~1056px, sidebar becomes an overlay drawer. Clicking
outside the drawer closes it. An "expanded" prop on `LeftNavWrapper` controls
visibility. Opening state is held in `NavContext` shared with the header.

---

## Component Detail Page Template

**URL pattern:** `/components/<component-name>/<tab>/`
- Example: `/components/button/usage/`, `/components/button/style/`,
  `/components/button/code/`, `/components/button/accessibility/`
- Tabs are URL-reflected (each tab is a distinct Gatsby page/route).
- Anchor patterns within pages: `#section-heading` (standard HTML anchors via
  the `AutolinkHeader` MDX component).

**Tabs per component page:**
The four canonical tabs are `['Usage', 'Style', 'Code', 'Accessibility']`.
These are declared in each MDX file's frontmatter under `tabs:`. The tab set
is component-level, not global — some components may omit a tab.

**Tab rendering:** `PageTabs` component renders tabs as a `<nav>/<ul>/<li>`
list. Tabs are NOT sticky — they sit below the page header. Tabs are URL-driven
(each tab is a separate page route). Active tab receives `selectedItem` CSS class.
Inter-tab navigation: each tab is an independent link; no in-page state required.

**PageHeader:** Each component page carries a page header with the component name
and a one-sentence description (from frontmatter `description:`). The
`A11yStatus` component renders a small accessibility-status card (conformance
badges) on Usage tabs, driven by a `components=` prop.

**Previous/Next navigation:** `NextPrevious` component at page bottom renders
sequential prev/next links with category labels, traversing the full nav tree
in order.

**Edit on GitHub:** `EditLink` component in the page — links to the MDX source
file in `carbon-design-system/carbon-website` on GitHub.

### Usage Tab Content Shape

MDX frontmatter: `title`, `description`, `tabs: ['Usage',...]`

H2 sections (Button example, representative):
1. Live demo (`StorybookDemo` component with `themeSelector` — 7 variants,
   embedded Storybook iframe)
2. Overview (H3: When to use, When not to use, Variants, Anatomy, Button sizes)
3. Formatting (H3: Emphasis, Alignment, Fixed/Fluid width, Button groups)
4. Content (H3: Main elements, Button label, Label alignment, RTL, Overflow)
5. Universal behaviors (H3: Focus, States, Interactions, Mouse, Keyboard,
   Loading)
6. Primary button (variant-specific section)
7. Secondary button
8. Tertiary button
9. Ghost button
10. Danger button
11. Modifiers (H3: Button with icon, Icon only buttons)
12. Related
13. References
14. Feedback

Custom MDX components used: `StorybookDemo`, `A11yStatus`, `PageDescription`,
`AnchorLinks`/`AnchorLink`, `Row`/`Column`, `DoDontRow`/`DoDont`, `Caption`,
`GifPlayer`, `InlineNotification`.

`DoDontRow`/`DoDont` pairs are the primary "do/don't" pattern comparison idiom.
`AnchorLinks` renders an in-page anchor jump list at the top of each tab.

### Style Tab Content Shape

H2 sections (Button example):
- Color (H3 per variant per state — very granular: "Primary button color",
  "Primary button interactive state color", etc.)
- Typography
- Structure (H3: Button structure, Ghost button structure, Button groups structure)
- Size
- Feedback

Token format: design tokens appear as inline code strings (backtick-wrapped)
within standard HTML tables. Example: `` `$text-on-color` ``, `` `$button-primary` ``,
`` `$spacing-05` ``. No dedicated `TokenTable` component in button style — plain
Markdown tables. `Tabs`/`Tab` components used for themed image switching.

### Code Tab Content Shape

The Code tab does NOT contain inline code snippets or a props table for Button.
It is structured as:
- H2: Documentation — four `ResourceCard` components linking to framework-
  specific Storybooks (React, Web Components, Angular [Community], Vue [Community])
- H2: Live demo — `StorybookDemo` component with seven variants

Framework support: React (primary/official), Web Components (official),
Angular (community), Vue (community). No inline framework selector — each
framework links out to its own Storybook.

### Accessibility Tab Content Shape

H2 sections (Button example):
1. What Carbon provides (H3: Keyboard interactions, Behavior)
2. Design recommendations (H3: Labeling)
3. Development considerations

`A11yStatus` component with `layout="table"` and `components="Button"` — renders
a dynamic conformance table (WCAG level, IBM checkpoint, criteria).

Keyboard interactions described in narrative prose (Tab to focus, Space/Enter
to activate, icon-only buttons expose labels on focus). No separate keyboard
table for Button — narrative only.

---

## Foundation Pages (Elements)

**URL pattern:** `/elements/<slug>/<tab>/`
- Example: `/elements/color/overview/`, `/elements/typography/overview/`

**Tabs differ from component pages.** Color has: `['Overview', 'Usage', 'Tokens', 'Code']`.
Typography has: `['Overview', 'Style strategies', 'Type sets', 'Code']`.

Foundation pages are longer-form than component pages. They explain the design
rationale and system-level semantics before presenting implementation specifics.

### Color Page Structure

H2 sections:
1. Introduction
2. Color anatomy (H3: Layering model)
3. Implementing color
4. Themes (H3: Light themes, Dark themes, High contrast moments)
5. Tokens (H3: Token names, Core tokens, Component tokens)
6. Interaction states (H3: Hover, Active, Selected, Focus, Disabled)
7. Accessibility (H3: Contrast ratios)
8. Resources

Custom components: `ColorBlock` (individual swatch + hex), `ColorGrid` (family
palette), `KalturaVideo` (instructional video), `GifPlayer` (animated
light/dark comparison), `DoDontRow`/`DoDont`, `Tabs`/`Tab`, `ResourceCard`.

Token naming convention: `$<role>-<variant>` — e.g., `$button-primary`,
`$text-on-color`. Four named themes: White, Gray 10 (light), Gray 90, Gray 100
(dark).

### Typography Page Structure

H2 sections:
1. Type tokens and sets (H3: Productive and expressive type sets)
2. Typeface: IBM Plex (H3: Sans-serif, Serif, Mono font stacks)
3. Scale
4. Style (H3: Weights, Italic, Type color)
5. Resources

Custom components: `TypeScaleTable`, `TypeWeight`, `DoDontRow`/`DoDont`,
`ResourceCard`, `MdxIcon`.

Font: IBM Plex family (IBM proprietary open-source). Sans-serif, Serif, and
Mono variants. Mathematical modular scale. Two type sets: Productive (UI-focused,
tighter) and Expressive (marketing/editorial, larger scale).

---

## Guidelines Pages

**URL pattern:** `/guidelines/<slug>/<tab>/`
- Example: `/guidelines/accessibility/overview/`

Accessibility page has tabs: `['Overview', 'Color', 'Developers', 'Keyboard']`.

Content is long-form prose + structured tables, not the tightly-templated
component format. The overview page covers seven disability categories, each
with H3 sub-sections: "How they experience an interface", "What designers should
think about", "How this applies to everyone."

Custom components: `PageDescription`, `AnchorLinks`, `Title`, `Row`, `Column`,
`ResourceCard`. No `StorybookDemo`, no `DoDontRow` — prose-dominant.

---

## "All About Carbon" / About Pages

**URL pattern:** `/all-about-carbon/<slug>/`

No tabs on What is Carbon. Page is introductory/educational. Content types:
narrative prose, principle-based feature list (five guiding principles as bold
statements + descriptions), embedded `KalturaVideo`, `AnchorLinks`.

H2 sections: Overview (H3: Carbon is open source), Introduction to Carbon,
How Carbon works (H3: Our guiding principles, We maintain assets, We support
adoption), Carbon compliance at IBM, Contact us.

---

## Search

**Implementation:** Client-side, Lunr.js via `react-lunr` `useLunr` hook.
Pre-built search index at build time. Global scope (all pages).

**Index source:** Every page with `title` in frontmatter is indexed. `description`
frontmatter field improves result quality. Custom scoring/source nodes via
`lunrOptions` config.

**Result display:** WAI-ARIA combobox pattern. Results dropdown below the search
input. Maximum 12 results (`MAX_RESULT_LIST_SIZE`). Keyboard navigation: arrow
keys traverse results, Enter navigates, Escape closes.

**Location:** Search is accessible from the top header (search icon). Not a
persistent search bar — icon opens inline search interaction.

**Faceted:** No. Flat ranked results only.

---

## Theme Toggle

**Carbon has no user-facing theme toggle on the documentation website itself.**
The top header does NOT contain a light/dark mode toggle for the site UI.

The Carbon design system defines four themes (White, Gray 10, Gray 90, Gray 100)
that product developers implement in their own applications. Some foundation pages
(e.g., Color overview) use `Tabs`/`Tab` components to switch between themed
image examples within the page, but this is not a site-wide toggle.

`StorybookDemo` components on Usage/Code tabs embed a Storybook instance which
has its own theme selector inside the iframe.

`gatsby-config.js` sets `themeHomepage: 'white'` — the homepage renders in the
White theme by default, static.

---

## GitHub Integration

**Header:** GitHub icon link to `github.com/carbon-design-system/carbon`.

**Per-page:** `EditLink` component renders an "Edit this page on GitHub" link
at the bottom of each page, linking to the specific MDX source file in
`carbon-design-system/carbon-website`.

**Previous/Next:** Not GitHub-linked — purely internal nav.

**Configured via:** `gatsby-config.js` `repository.baseUrl` and `repository.subDirectory`.

---

## URL Patterns

```
/                                   Homepage
/all-about-carbon/<slug>/           About section (no tabs)
/what-is-carbon/                    Redirects into /all-about-carbon/
/designing/<slug>/                  Designing section
/developing/<slug>/                 Developing section (frameworks, tutorials)
/contributing/<slug>/               Contributing section
/migrating/<slug>/                  Migration guides
/elements/<slug>/<tab>/             Foundation elements (color, typography, etc.)
  e.g. /elements/color/overview/
       /elements/color/tokens/
       /elements/typography/overview/
/guidelines/<slug>/<tab>/           Guidelines (accessibility, AI, content)
  e.g. /guidelines/accessibility/overview/
/components/<name>/<tab>/           Component pages
  e.g. /components/button/usage/
       /components/button/style/
       /components/button/code/
       /components/button/accessibility/
/patterns/<slug>/                   Pattern pages
/data-visualization/<slug>/         Data visualization section
/community/<slug>/                  Community assets
/help/<slug>/                       Help pages
```

Tabs are reflected in the URL as a path segment (not hash/query param). Each
tab is a distinct Gatsby page/route. Within a page, section anchors use `#slug`
fragment identifiers generated by `AutolinkHeader`.

---

## Footer

**Composition (two-column link grid + branding row):**

Column 1: Contact us, Privacy, Terms of use, Accessibility, IBM.com
Column 2: Medium, X (formerly Twitter)

Content area (centered): contact email (`carbon@us.ibm.com`), React Components
version number, build timestamp.

Bottom row: IBM logo (SVG), copyright `© <currentYear> IBM`.

Footer is customizable via `links`, `Content`, and `Logo` props. Build
timestamp injected via Gatsby GraphQL query at build time.

---

## Visual / Typographic Patterns

**Heading hierarchy:** IBM Plex Sans throughout. H1 used for page title
(PageHeader component). H2 for major tab sections. H3 for sub-sections within
H2. H4 rare (appears in complex tables).

**Font:** IBM Plex Sans (body, headings, UI), IBM Plex Mono (code), IBM Plex
Serif (expressive/editorial contexts). IBM Plex is an IBM open-source typeface
family.

**Spacing rhythm:** Based on a 2x base-8 grid. `$spacing-*` tokens: 02=2px,
03=4px, 04=8px, 05=16px, 06=24px, 07=32px, 08=40px, 09=48px, 10=64px, etc.
Body text lives on an 8px baseline grid.

**Color roles (token naming convention):**
- Background tiers: `$background`, `$layer-01`, `$layer-02`, `$layer-03`
- Text: `$text-primary`, `$text-secondary`, `$text-placeholder`, `$text-disabled`
- Interactive: `$interactive`, `$focus`, `$highlight`
- Status: `$support-error`, `$support-warning`, `$support-success`, `$support-info`
- Border: `$border-subtle-01`, `$border-strong-01`
- Component-specific: `$button-primary`, `$button-secondary`, etc.

**Layout:** 2-column at lg+ (sidebar ~256px fixed, content ~1fr). Single column
on mobile. Content area max-width constrained. Grid via Carbon's `cds--grid` /
`cds--row` / `cds--col-lg-*` classes.

**Code blocks:** Syntax-highlighted via Carbon's `Code` MDX component (PrismJS
under the hood). Dark background regardless of page theme. Copy button present.

---

## Machine-Readable / API Surface

**Tokens — npm packages published:**
- `@carbon/tokens` — SCSS + JS token exports. Format: SCSS variables
  (`$button-primary: ...`), JS named exports. As of 2025, Carbon is migrating
  toward DTCG-compatible JSON output but the primary published format remains
  SCSS/JS. No dedicated DTCG `.tokens.json` endpoint surfaced on the live site.
- `@carbon/colors` — color primitives (Blue 60, Gray 90, etc.)
- `@carbon/themes` — theme objects (White, G10, G90, G100)
- `@carbon/layout` — spacing + grid tokens
- `@carbon/type` — typography tokens
- `@carbon/react` — React component library (v1.105.0 as of late 2025)
- `@carbon/styles` — CSS/SCSS stylesheet bundle (v1.100.0 as of late 2025)
- `@carbon/web-components` — Web Components library

**Carbon MCP:** The nav YAML lists "Carbon MCP" under Developing → Frameworks.
This is Carbon's Model Context Protocol integration — a documentation/tooling
integration point for AI assistants, not a token API endpoint. No public MCP
server URL surfaced from this analysis.

**Storybook:** Live component playground at `react.carbondesignsystem.com`
(React) and `web-components.carbondesignsystem.com`. Each component's Code tab
links to its Storybook story.

**GitHub source:** All MDX documentation source at
`github.com/carbon-design-system/carbon-website`. All component implementations
at `github.com/carbon-design-system/carbon`.

---

## Key Structural Observations for Re-Implementation

1. **The header is minimal.** No primary nav in the header — everything is in
   the left sidebar. Header = logo + search icon + GitHub icon + ecosystem
   switcher icon.

2. **Tabs are separate routes, not in-page state.** `/components/button/usage/`
   and `/components/button/style/` are distinct server-renderable pages. The
   `tabs:` frontmatter array drives `PageTabs` rendering with Gatsby `Link`
   components.

3. **MDX is the content model.** Every page is an MDX file. The frontmatter
   `tabs:` field declares which sibling pages form a tab group. The component
   doc framework is file-system-driven, not database-driven.

4. **Custom MDX components are the design system vocabulary.** `DoDontRow`,
   `StorybookDemo`, `AnchorLinks`, `ResourceCard`, `ColorBlock`, `TypeScaleTable`
   etc. are the building blocks. Prose content is light; the custom components
   carry the structural weight.

5. **Accordion sidebar, not flat list.** 13 top-level categories, most with
   multiple sub-pages. Active category auto-expands. This is the primary IA
   mechanism — the sidebar is the navigation spine of the whole site.

6. **No site-wide dark mode toggle.** Theme is product-developer-facing
   documentation, not a dark-mode-first consumer site.

7. **Previous/Next at every page bottom.** Traverses the full nav tree in
   YAML order. Combined with the sidebar, this creates two navigation paths:
   spatial (sidebar) and sequential (prev/next).

8. **Footer is thin.** 5 links + 2 social + email + version + build date +
   logo + copyright. Not a mega-footer.
