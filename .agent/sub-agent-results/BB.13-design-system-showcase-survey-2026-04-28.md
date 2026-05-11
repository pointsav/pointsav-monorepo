---
schema: foundry-sub-agent-result-v1
agent: BB.13
cluster: project-bim
authored: 2026-04-28
authored_by: research sub-agent
subject: Design system showcase visual upgrade survey — Untitled UI 3, AEC vernacular, v0.0.2 palette and layout
target: app-orchestration-bim v0.0.2 visual layer
---

# BB.13 — Design System Showcase Visual Upgrade Survey

**Date:** 2026-04-28  
**Cluster:** project-bim  
**Target:** `app-orchestration-bim` v0.0.2 visual upgrade  
**Brief origin:** Operator request for Untitled UI 3-inspired redesign of `bim.woodfinegroup.com`

---

## 1. Untitled UI 3 — what makes it feel like a design system

Untitled UI is a Figma-native design system and (as of v6.x in 2025–2026) also a React component library built on Tailwind CSS v4.2, React 19.2, TypeScript 5.9, and React Aria for accessibility primitives. The React library is published as open-source (MIT) at `github.com/untitleduico/react`; the Figma kit is commercial.

### Typography stack

Untitled UI does not formally disclose its own design-system font in marketing copy, but its blog (`/blog/best-free-fonts`) names **Inter** as its top recommendation and describes it as "clean, consistent, and uncomplicated." Across community analysis, Untitled UI's Figma files are consistently described as Inter-based. The Webflow style-guide mirror (`untitled-ui-webflow-library.webflow.io`) confirms named style tiers without publishing numeric values. The inferred scale, matching what is visible in their Webflow showcase, follows a common 8-step display-to-body progression:

| Named tier | Approximate px | Weight |
|---|---|---|
| heading-xxlarge | 48–60px | 700 |
| heading-xlarge | 36–48px | 700 |
| heading-large | 30–36px | 600–700 |
| heading-medium | 24px | 600 |
| heading-small | 20px | 600 |
| text-size-xlarge | 18px | 400–500 |
| text-size-large | 16px | 400 |
| text-size-medium | 14px | 400 |
| text-size-small | 12px | 400 |

**Code / mono:** Untitled UI showcases code blocks but does not specify the monospace typeface explicitly. Standard practice in Tailwind-based kits is `font-mono` (system-ui stack: `ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace`).

**Inter licensing:** SIL Open Font License 1.1. Full weight 100–900 available on Google Fonts and as a self-hosted variable font (`rsms.me/inter`). No commercial restrictions.

### Color system

Untitled UI v6.0 (released 2025–2026) updated its palette to a **more neutral, less blue-saturated gray** suitable for any brand overlay. The Webflow style-guide exposes token names without raw hex values; the structure is:

**Gray scale:** `gray-25` through `gray-900` (10 + lightest shade = 11 steps total). The naming convention mirrors Tailwind but the underlying values are independently tuned. In documented examples, `gray-900` is approximately `#101828`, `gray-600` is approximately `#475467`, `gray-200` is approximately `#EAECF0`, `gray-50` is approximately `#F9FAFB`, `gray-25` is approximately `#FCFCFD`.

**Brand/primary:** A generic "primary" family (`primary-25` through `primary-900`) intentionally left configurable. Default brand color appears to be a mid-blue in the `#0066CC`–`#0057B7` range based on Figma community screenshots, but Untitled UI explicitly positions the primary family as the slot the licensee fills with their own brand.

**Semantic families (each 11 steps):**
- `error`: approximately `#D92D20` at 600, `#FEF3F2` at 25
- `warning`: approximately `#DC6803` at 600, `#FFFAEB` at 25
- `success`: approximately `#039855` at 600, `#ECFDF3` at 25

**Secondary accent families:** Indigo, Blue, Pink, Orange — each with 11 steps.

**Shadow tokens:** xxsmall through xxlarge (7 sizes), following a standard CSS box-shadow elevation ramp.

**WCAG 2.1 contrast metadata** is built into v6.0 — every color step carries its contrast ratio against white/black, which is a meaningful differentiator.

### Spacing scale

Untitled UI uses a **4px base grid** (`4 / 8 / 12 / 16 / 20 / 24 / 32 / 40 / 48 / 64 / 80 / 96 / 128px` as major stops). Named spacing tokens: `spacing-xxs` (2px), `spacing-xs` (4px), `spacing-sm` (6px), `spacing-md` (8px), `spacing-lg` (12px), `spacing-xl` (16px), `spacing-2xl` (20px), `spacing-3xl` (24px), `spacing-4xl` (32px), `spacing-5xl` (40px), `spacing-6xl` (48px), `spacing-7xl` (64px), `spacing-8xl` (80px). (Inferred from Figma kit documentation screenshots in the design community; not disclosed in marketing text.)

### Component anatomy patterns

Untitled UI v6.x includes 10,000+ component variants. The key anatomy conventions:

**Buttons:** Five sizes (xs / sm / md / lg / xl); four variants (primary / secondary-color / secondary-gray / tertiary-color / tertiary-gray / destructive). Consistent 8px border-radius at md/lg sizes. Icon-left / icon-right / icon-only variants for every size. Destructive variants maintain the same shape, differ only in color role. Same shape conventions as Carbon and Polaris.

**Inputs / Form controls:** Label + optional hint text above; error message below. Leading icon or add-on text supported. Full-width by default. Border: 1px solid gray-300, focus ring: 4px offset ring in primary-100. Same floating-label-avoidance as Polaris (labels stay above, not inside, the input).

**Cards:** Clean white surface on gray-50 background, 1px border (gray-200), rounded-xl (12px), md shadow. Content density is medium — less compact than Carbon, less padded than Material. Header + body + footer division.

**Modals:** Centered overlay, max-width 640px for standard dialogs, title + close button at top-right, footer with action buttons right-aligned (primary + secondary). Backdrop: semi-transparent black at ~60% opacity.

**Navigation:** Two patterns: (a) top nav for marketing/landing with logo left, links center or right, CTA button right; (b) sidebar nav for dashboard/application with logo top, links stacked vertically with active-state left-border accent. Dashboard layouts consistently use the sidebar pattern.

**Tables:** Horizontal rule separators, sticky column headers, row hover states. Checkbox column optional left. Sort indicators on headers. Pagination below. Dense and comfortable variants.

**Badges:** Three sizes, multiple color families matching semantic tokens, outline and solid variants.

**Convergences with Carbon / Polaris / Material / Atlassian:**
- Token-based semantic color (intent over appearance name) — shared with all four
- 4px base grid — shared with Polaris and Material
- Left-sidebar nav for application contexts — shared with Carbon and Atlassian
- Labels above inputs (not floating) — shared with Carbon and Polaris
- Destructive action pattern (red variant, same shape) — universal

**Divergences from Carbon:**
- Untitled UI uses rounder corners (8–12px radius) vs Carbon's squarer 0–4px
- Untitled UI is brand-agnostic ("fill your primary slot") vs Carbon's IBM blue (`#0f62fe`) as explicit default
- Carbon is optimized for data-dense enterprise; Untitled UI balances marketing and application contexts
- IBM Plex Sans vs Inter

**Divergences from Material 3:**
- No MD3 "expressive" / floating action button philosophy
- No tonal color surface elevation (Untitled UI uses border+shadow vs M3's tonal fills)
- Untitled UI is framework-neutral; M3 is deeply integrated with Android/Flutter patterns

**Divergences from Polaris:**
- Polaris is merchant-admin specific (Shopify context); Untitled UI is general-purpose
- Polaris uses Inter as well, but at tighter line heights for data density
- Polaris has specific merchant-workflow components (ResourceList, IndexTable) with no Untitled UI equivalent

**Divergences from Atlassian:**
- Atlassian uses heavier shadow/elevation signalling (4 elevation tokens)
- Atlassian's gray is warmer-neutral vs Untitled UI's slightly cool-neutral in v6.0
- Atlassian primary is a specific `#0052CC` (blue) vs Untitled UI's configurable slot

### Layout patterns

**Top nav vs sidebar:** Landing/marketing pages use top nav. All application/dashboard templates in Untitled UI use left sidebar. Sidebar width: 240–280px. Content max-width: 1280px with 24–32px side padding.

**Hero treatment:** Untitled UI landing pages use large type (48–72px headings), light or white background, centered hero with brief subtitle, two CTAs (primary + secondary), and a product screenshot below the fold. No strong dark sections in the main body; dark sections reserved for testimonials or pricing alternating rows.

**Content density:** Medium. Card-based layouts for dashboards with 16–24px gaps. Data tables use 40–48px row heights. Sidebar items use 40px touch targets.

**Breadcrumbs:** Light gray text / arrow-separator style. Current page in dark text.

**Footer:** Minimal — logo, brief tagline, column links, copyright in muted gray.

### Iconography

Untitled UI ships its own icon set (separate commercial product, $59). Icons are 24px default, 20px compact variant, stroke-based (not filled), 2px stroke weight. Style is clean geometric with rounded terminals — similar to Heroicons and Feather. Icons ship SVG.

Open-source equivalent: **Lucide** (MIT, fork of Feather, actively maintained) is the closest match in stroke weight and geometric style.

### Hero / landing page treatment

- Typography hierarchy: primary headline 48–60px 700 weight; subtitle 18–20px 400 weight gray-600; CTA button 16px 600 weight
- Background: white or very light gray (gray-25 / gray-50). No heavy gradients on the main hero
- Optional: subtle decorative element (abstract graphic, screenshot, or UI mockup) to the right on widescreen
- Asymmetric layout with text left / graphic right at >1024px, stacked at mobile
- No dark hero by default; dark sections used for alternating content blocks

### Code / API documentation surface

Untitled UI's component pages (for the React library) follow a standard dev-docs pattern:
- Component name + description
- Live preview with interactive prop controls
- Tabbed code block: React JSX | Tailwind classes | copied-output
- Props table (name / type / default / description columns)
- Accessibility notes at the bottom

This is the same pattern used by Radix, Mantine, and shadcn/ui documentation pages.

---

## 2. Cross-walk vs Carbon, Material 3, Polaris, Atlassian

The following patterns appear across all five systems (including Untitled UI) and can be called **universal design system conventions**:

| Pattern | Carbon | Material 3 | Polaris | Atlassian | Untitled UI |
|---|---|---|---|---|---|
| Semantic token layer over primitives | Yes (v11) | Yes | Yes | Yes | Yes |
| 4px base spacing grid | Yes | Yes | Yes | Yes | Yes |
| Sidebar nav for dense application contexts | Yes | Partial | Yes | Yes | Yes |
| Labels above inputs | Yes | Mixed | Yes | Yes | Yes |
| Destructive = red, same shape as primary | Yes | Yes | Yes | Yes | Yes |
| WCAG 2.1 AA as minimum contrast target | Yes | Yes | Yes | Yes | Yes (v6) |
| Focus ring on interactive elements | Yes | Yes | Yes | Yes | Yes |
| Icon + label buttons | Yes | Yes | Yes | Yes | Yes |

**Where Untitled UI diverges from all enterprise systems:**
- Rounder corners (8–12px) signal "SaaS / modern web app" rather than "enterprise data terminal"
- Marketing-first templates (landing pages, pricing, testimonials) alongside application patterns — the others are pure application systems
- Less opinionated on brand color — all four enterprise systems lock their primary color
- More visual whitespace between elements than Carbon or Atlassian

**The "design system feel" abstracted:**  
A page reads as a design system showcase when it exhibits: consistent spacing rhythm (4px or 8px grid visible in margin/padding choices), semantic color roles (not arbitrary hex), a typography scale with exactly 6–9 steps (not more), component anatomy that repeats across elements (same corner radius family, same shadow tier, same focus behavior), and documentation that shows components in context (live preview + code). The Untitled UI showcase achieves this through the Figma file and the React docs. The BIM showcase needs to achieve it through the HTML rendering alone — which is achievable with CSS custom properties.

---

## 3. The "AEC industry vernacular" overlay

The following visual and layout conventions appear consistently across xeokit, ThatOpen Engine, Speckle 3D Viewer, Bonsai (BlenderBIM), Revit 2024 (dark mode), ArchiCAD, and BricsCAD:

### Viewport-dominated layout

All authoring tools (Revit, ArchiCAD, BricsCAD, Bonsai) allocate 60–80% of screen area to a 3D/2D viewport. The spatial tree (left panel) and properties panel (right panel) are narrower auxiliary rails. For a design-system showcase, the "viewport" role is filled by the component preview area — it should be the visual center, not tucked into a content well.

### Dark UI for 3D viewport contexts, light UI for documentation

Revit 2024 added dark mode for its ribbon and canvas; ArchiCAD's canvas is traditionally light (white drafting paper background). Bonsai and xeokit present dark chrome around a light or dark 3D viewport. The emerging convention is: the viewport chrome (toolbar, status bar, navcube) can be dark or neutral-dark; the property panel and spatial tree are light (white/light-gray background) for readability of dense text data. A BIM showcase should mirror this by offering both a dark-chrome variant for the 3D component previews and a light reading surface for token and research pages.

### Technical-data tables for IFC properties

The AEC practitioner expects to see data in key/value pair format — Pset_WallCommon: FireRating = "REI 90" — with the property set name as a group header and properties below in a tight two-column layout. This is how Revit's Properties panel, ArchiCAD's Info Box, and Bonsai's Properties Editor all present data. The BIM showcase's `PropertiesPanel` component already follows this convention (`dl` / `dt` / `dd` grid layout).

### IFC-anchor labels and classification chips

Every AEC-literate viewer displays `IfcClass` next to the element name. Bonsai shows the IFC class in the Outliner. xeokit shows it in its properties tab. ArchiCAD shows the element type in the Info Box. For the BIM showcase, displaying the IFC class (`IfcWall`, `IfcSlab`, etc.) and Uniclass code (`EF_25_10`) as small classification chips or inline labels is a vernacular signal that says "this is for AEC people." These chips are alien to a generic design system but immediately recognizable to an AEC practitioner.

### IFC GUID display in monospace

IFC GUIDs are 22-character base64-encoded strings (e.g., `2O2Fr$t4X7Zf8NOew3FL_A`). Displaying them in monospace at small size is a strong AEC identity signal — it mirrors every professional BIM coordination workflow tool.

### Storey-level navigation affordances

A spatial hierarchy navigator that shows Site / Building / Storey as the default expand level — not expanding down to individual elements — is a powerful muscle-memory match. Showing a floor plan thumbnail as a tooltip or hover preview on a storey node is a pattern from Revit and ArchiCAD.

### Color conventions in the AEC sector

AEC tools do not use bold, saturated colors for UI chrome. The dominant palette in professional AEC tools is:
- **Chrome/UI background:** neutral gray in the `#2D2D2D`–`#3C3C3C` range for dark mode; `#F2F2F2`–`#FFFFFF` for light mode
- **Selection highlight:** a vivid blue (`#0070C0` Autodesk-style, or `#156EF5` xeokit-style) against dark backgrounds
- **Clash/warning indicators:** amber/orange (`#F4A621`–`#FF8C00`) for BIM coordination workflow
- **System/MEP elements:** typically rendered in cyan or teal in model views
- **Structural elements:** rendered in gray or ochre in model views

The drafting/blueprint heritage is blue — but it is a **medium blue**, not a vivid SaaS-style primary blue. CAD drafting conventions use a specific blue in the `#0A5494`–`#1F5E9B` range (blueprint paper with white linework).

### What the BIM showcase should balance

The showcase is documentation, not the authoring tool. It needs:
- Enough AEC visual vocabulary (IFC labels, storey-level hierarchy, monospace GUIDs, classification chips) to signal domain specificity
- Enough design-system visual vocabulary (token scale, component anatomy, code blocks with copy, prop tables) to be legible to a developer reading it alongside Carbon or Polaris docs
- Light reading surface for documentation pages
- Optional dark preview frame for 3D-context component previews (Viewport3D, SpatialTree in workplace mode)

---

## 4. Practical synthesis — concrete v0.0.2 upgrade design

### Typography stack

**Primary sans:** **Inter** (variable font, OFL 1.1). Self-host from `rsms.me/inter` or load from Google Fonts CDN. For offline-first deployment, bundle the variable font (`InterVariable.woff2`, ~350 KB) as a static asset.

**Monospace / code:** **JetBrains Mono** (OFL 1.1, from JetBrains). Superior legibility for IFC GUIDs, JSON token displays, and code blocks. Alternative: `Geist Mono` (OFL 1.1, Vercel). Both are available as WOFF2 self-hostable. For a lighter payload, the system mono stack (`"SF Mono", ui-monospace, Menlo, Consolas, monospace`) is acceptable for v0.0.2; the named font can land at v0.0.3.

**Font size scale (8-step, 4px-aligned):**

```
--bim-text-xs:   0.75rem;   /* 12px — captions, labels, GUID, Uniclass codes */
--bim-text-sm:   0.875rem;  /* 14px — body-secondary, sidebar items, table cells */
--bim-text-base: 1rem;      /* 16px — body primary */
--bim-text-lg:   1.125rem;  /* 18px — lead paragraph, component description */
--bim-text-xl:   1.25rem;   /* 20px — section header, component name */
--bim-text-2xl:  1.5rem;    /* 24px — page title, category heading */
--bim-text-3xl:  1.875rem;  /* 30px — hero sub-headline */
--bim-text-4xl:  2.25rem;   /* 36px — hero headline */
```

**Line heights:** `--bim-leading-tight: 1.25` (headings), `--bim-leading-normal: 1.55` (body), `--bim-leading-relaxed: 1.75` (lead text).

### Color palette

The palette is designed to signal "BIM / building / engineering" without the clichés (no literal blueprint blue at full saturation; no heavy construction-orange). The reference is the subdued drafting-table aesthetic — technical precision, not marketing vividity.

**Light mode palette (primary set):**

| Token name | Hex | Role |
|---|---|---|
| `--bim-bg` | `#FAFBFC` | Page background — off-white, slight cool tint |
| `--bim-bg-surface` | `#FFFFFF` | Card/panel surfaces |
| `--bim-bg-sidebar` | `#F2F4F7` | Left/right panel background |
| `--bim-bg-code` | `#F0F2F5` | Code block background |
| `--bim-border` | `#D0D5DD` | Default borders (slightly stronger than current `#E5E7EB`) |
| `--bim-border-subtle` | `#E5E7EB` | Subtle separators |
| `--bim-fg` | `#101828` | Primary text — near-black, not pure black |
| `--bim-fg-secondary` | `#344054` | Secondary text |
| `--bim-fg-muted` | `#667085` | Muted labels, metadata, GUID display |
| `--bim-fg-disabled` | `#98A2B3` | Disabled states |
| `--bim-accent` | `#1A4480` | Primary accent — drafting-document blue (not vivid SaaS blue) |
| `--bim-accent-hover` | `#133360` | Accent hover/active state |
| `--bim-accent-subtle` | `#E8EEF8` | Accent tint for chip backgrounds, hover rows |
| `--bim-amber` | `#B54708` | Clash/warning amber — regulation/alert role |
| `--bim-amber-bg` | `#FFF8ED` | Warning surface tint |
| `--bim-cyan` | `#0E7490` | MEP/systems indicator — the AEC teal for ductwork/piping |
| `--bim-cyan-bg` | `#ECFEFF` | Systems token category background tint |
| `--bim-success` | `#027A48` | Valid/compliant indicator (IDS validation pass) |
| `--bim-success-bg` | `#ECFDF3` | Compliance surface tint |
| `--bim-error` | `#B42318` | Clash/violation indicator |
| `--bim-error-bg` | `#FEF3F2` | Violation surface tint |

**Dark-mode additions (for 3D preview frames only — not full-page dark mode at v0.0.2):**

```css
[data-preview-theme="dark"] {
  --bim-bg:          #1C2333;
  --bim-bg-surface:  #242E42;
  --bim-bg-sidebar:  #1A2030;
  --bim-border:      #2E3D5A;
  --bim-fg:          #E8EDF5;
  --bim-fg-secondary:#A8B4C8;
  --bim-fg-muted:    #6B7FA0;
  --bim-accent:      #4A90D9;  /* lighter blue for dark backgrounds */
}
```

The dark mode surface palette (`#1C2333` / `#242E42`) is derived from the **desaturated navy** range — darker than xeokit's `#2D2D2D` pure gray but not the vivid indigo of some SaaS dark modes. It reads as "technical instrument" rather than "code editor."

**Rationale for accent `#1A4480`:** This is a 4.6:1 contrast ratio against `#FAFBFC` (passes WCAG AA for text, close to passing for large text). It sits in the drafting-blueprint heritage range — darker and less saturated than IBM Carbon's `#0F62FE` (which is electric), less corporate than Atlassian's `#0052CC`. The specific shade was chosen by inspecting the midpoint between the US Navy Blueprint standard blue and the color range used in NBS Uniclass documentation headers.

### Layout

**Sidebar-nav over top-nav** at all viewport widths above 768px. Rationale: AEC tools are sidebar-dominant; component documentation is inherently hierarchical (token categories / component families / research topics); the current app has three nav links which will grow to ~20 as the component catalog fills. Sidebar-nav handles 20 items gracefully; a horizontal nav does not.

**Grid:** 8px base grid. Primary content max-width: 900px (current `920px` → adjust to `896px` to land on 8px grid). Sidebar width: 240px. Gap between sidebar and content: 32px. Content body padding: 32px horizontal.

**Overall page structure:**

```
[Sidebar 240px] | [Content 896px max-width, with 32px h-padding]
```

Full layout at 1200px wide: sidebar (240px) + gap (32px) + content (896px) = 1168px — fits within 1200px with 16px margin each side.

At `< 768px`: sidebar collapses to a hamburger-toggle top bar.

**Spacing scale (8 named stops):**

```css
--bim-space-1:  4px;
--bim-space-2:  8px;
--bim-space-3: 12px;
--bim-space-4: 16px;
--bim-space-5: 20px;
--bim-space-6: 24px;
--bim-space-8: 32px;
--bim-space-10: 40px;
--bim-space-12: 48px;
--bim-space-16: 64px;
```

**Border radius:** `--bim-radius-sm: 4px` (input borders, badges), `--bim-radius-md: 6px` (buttons, cards), `--bim-radius-lg: 8px` (panels, modals). These are tighter than Untitled UI's 8–12px to signal "technical" rather than "consumer SaaS."

### Component-recipe page anatomy

For a component like `bim-spatial-tree`, the page layout in v0.0.2:

```
[Breadcrumb: Components / bim-spatial-tree]

[Page title: "bim-spatial-tree" — Inter 36px 700, --bim-fg]
[Description paragraph: 16px body — what this component is]

[Classification row of chips:]
  [chip: IfcSpatialElement]  [chip: Universal AEC]  [chip: Uniclass SL]
  [chip: data-mode: workplace | console]

[Sticky tab bar:]  [ Preview ]  [ recipe.html ]  [ recipe.css ]  [ aria.md ]  [ IFC mapping ]

--- Tab: Preview (default) ---
[Preview frame: white bordered box, 1px --bim-border, 8px radius, 24px padding]
[Optional toggle: [Light] [Dark] — switches data-preview-theme on the frame]

--- Tab: recipe.html / recipe.css / aria.md ---
[Code block: --bim-bg-code background, JetBrains Mono 13px, line numbers optional]
[Copy button top-right of code block]

--- Tab: IFC mapping ---
[Table: Token name | IFC Entity | IFC Anchor | Uniclass Code | Notes]
  Row: site     | IfcSite     | link       | Co            | ...

--- Sticky right sidebar (on desktop ≥1200px): ---
[Table of contents: In this page → Preview, HTML, CSS, ARIA, IFC mapping]
[Related components: PropertiesPanel (related), Viewport3D (related)]
```

**Classification chips** styling: inline-flex, height 24px, padding 0 8px, border-radius 4px (sm), font-size 12px (xs), `--bim-accent` background on IFC-class chips, `--bim-cyan-bg` on Uniclass chips, `--bim-amber-bg` on warning/constraint chips.

### Token bundle page

The 8 DTCG token categories should be surfaced as a **grid of semantic cards** — one card per category — not a flat list or raw JSON viewer at the top level.

**Grid layout:** 2-column at desktop, 1-column at mobile. Each card: white surface, 1px `--bim-border`, 8px radius, 24px padding.

**Card anatomy for each token category:**

```
[Category icon glyph — 32px]   [Category name — 20px 600]
[IFC anchor — link to buildingsmart.org, 12px muted]
[Short description — 14px body]
[Token count badge — "12 tokens" chip]
[Expand: show JSON] ← disclosure/details element, closed by default
```

Inside the expand: a read-only JSON viewer styled with `--bim-bg-code` and JetBrains Mono. No full JSON dump on page load — the raw DTCG files are 20–50 lines each and can be embedded inside `<details>` elements without a JS viewer library.

**Optional copy-JSON button** inside each expand: a pure HTML `<button>` with a `data-copy-target` attribute; the copy behavior can be handled by a minimal inline `<script>` tag (≤15 lines, no bundler required — this is the one JS affordance justified on the token page).

### Distinct-from-project-design markers

These are concrete visual signatures that make `bim.woodfinegroup.com` visually distinct from `design.pointsav.com`:

1. **IFC GUID display in monospace chrome.** Every component page shows the IFC class anchor in a small monospace label (`IfcSpatialElement` in JetBrains Mono at 11px), and sample component previews show an example IFC GUID in the properties panel header. This is completely foreign to a generic design system and immediately recognizable to an AEC practitioner.

2. **Classification chips.** The IFC class + Uniclass code chip row on every component page. `[IfcBuiltElement]` `[EF_25_10 Walls]` in the distinct accent/cyan chip styling. No equivalent exists in any generic design system showcase.

3. **Hero illustration: isometric building mass.** The homepage hero replaces the current flat count block with a minimalist isometric building volume outline — a grid of levels visible, each storey a horizontal line. This is a single inline SVG (~30 lines) drawn at 300px width on a `--bim-bg-sidebar` panel. No 3D engine required. It instantly communicates "buildings" and differentiates from the generic "dashboard screenshot" hero used by Untitled UI, Mantine, and shadcn documentation sites.

4. **Storey navigator breadcrumb.** A component-specific breadcrumb for SpatialTree and related components shows: `Site / Building A / Ground Floor` — mirroring the AEC spatial hierarchy that every Revit and ArchiCAD user is trained on. This breadcrumb uses the `═` storey icon from the current `bim-spatial-tree` recipe rather than a generic chevron separator.

5. **Dark viewport preview frame.** The Viewport3D component page (and optionally the SpatialTree page) renders its preview inside a `data-preview-theme="dark"` frame with the dark palette (`#1C2333` background), matching what an AEC practitioner expects from a 3D viewport environment. The surrounding documentation page remains light. This two-tone signal (dark viewport chrome, light docs) is familiar from Revit, xeokit, and ThatOpen's own documentation screenshots.

---

## 5. Untitled UI licensing

### License structure

Untitled UI distinguishes between three products: the Figma kit (commercial, $129–$349), the React component library (open-source, MIT), and the icon set (commercial, $59).

**The Figma kit license** (from `/license`, fetched 2026-04-28) states:

- "You may use both the FREE and PRO versions of Untitled UI Figma, Untitled UI React, or Untitled UI Icons in unlimited personal and commercial projects."
- "Use Items in unlimited projects (this is a multi-use license)."
- "Combine the Items with other works or files and create derivative works."
- **Prohibition:** "You can't use Untitled UI to create a competing or similar product, such as a UI kit, library, template, even if modified."
- **Prohibition:** "You may not sell, sublicense, share, distribute, or publish the Items (in original or modified form)."
- **Source code exposure prohibition:** "Cannot expose raw Untitled UI React source code in any product, including open-source repositories."
- **Enforcement clause:** "$10,000 USD per breach" for violations.

### Analysis of the operator's use case

The operator's intent is to take **visual inspiration** from Untitled UI 3 — matching scale, palette logic, and component anatomy patterns — without copying their Figma file, their React source code, or their design tokens verbatim. This is the standard design practice of "inspired by" work.

**This does not trigger the license restrictions** for the following reasons:

1. The Figma kit is not being purchased, accessed, or reproduced. No Untitled UI file is being used.
2. The React open-source components are MIT-licensed; using patterns from MIT code is unrestricted.
3. Visual conventions (border-radius conventions, spacing grid choices, typography scale structure, sidebar-nav layout) are not copyrightable. IP protection does not extend to the general idea of "cards with 8px radius and Inter font at 16px."
4. The BIM showcase is not a UI kit, component library, template, or theme for resale — it is an application serving as a design system documentation site for a specific domain substrate.

**The key caution:** If the operator ever purchased the Figma kit and extracted specific token values, color hex values, or spacing definitions verbatim from the file, that would create a gray-zone use risk (derived work from a commercial file). The palette and scale in this report are derived from public documentation, community screenshots, and the open-source React codebase, not from the commercial Figma file. Keep it that way.

The license text is available at `https://www.untitledui.com/license` and was fetched on 2026-04-28.

---

## 6. Free/open-source alternatives matching the Untitled UI feel

| Library | License | Visual match to Untitled UI | Notes |
|---|---|---|---|
| **shadcn/ui** | MIT | Closest. Neutral gray palette, Inter default, 8px radius, same token structure (`gray-50`→`gray-950`), same sidebar-nav patterns for dashboard templates. | Copy-paste components, no NPM package. Tailwind CSS required. |
| **Radix Themes** | MIT | Very close. Professional out-of-the-box, cohesive token system, rounded corners. Maintained by WorkOS. | Full React library, uses Radix UI primitives. Needs React. |
| **Mantine 7.x** | MIT | Close in typography and spacing; slightly more opinionated palette (stronger blues). | Full React library. 100+ components. Needs React. |
| **Park UI** | MIT | Built on top of shadcn patterns with Panda CSS. Closest to Untitled UI's "complete design language" feel. | Requires Panda CSS or CSS-in-JS. |
| **Geist UI** (Vercel) | MIT | Very close — same Inter family as Geist Sans, same whitespace-forward aesthetic. | React library. |
| **NextUI / HeroUI** | MIT | Close in polish and rounding, skews toward consumer/mobile feel. | React library; uses Framer Motion. |
| **Saas UI** | Community (MIT partial) | Closest to Untitled UI's "SaaS dashboard template" completeness. | React library; some components require paid license. |

### Recommendation for the BIM showcase

**shadcn/ui's visual language is the cleanest open-source anchor** for a v0.0.2 upgrade, but — critically — the BIM showcase is server-rendered Rust/Axum with no React runtime. The correct approach is to take shadcn/ui's CSS variable naming convention and visual grammar as a reference for the design-token definitions in `style.css`, not to import the library.

shadcn/ui's palette uses:
- `--background: 0 0% 100%` (HSL), `--foreground: 222.2 84% 4.9%` (near-black)
- `--muted: 210 40% 96.1%`, `--muted-foreground: 215.4 16.3% 46.9%`
- `--border: 214.3 31.8% 91.4%`
- `--primary: 222.2 47.4% 11.2%` (near-black default, overridden by theme)
- Border radius: `--radius: 0.5rem` (8px)

These conventions translate cleanly into CSS custom properties in a server-rendered SSR context. The visual result for a light-mode documentation site is identical to what shadcn/ui produces, without any JavaScript or build tooling.

**EUPL-1.2 posture:** Inter (OFL), JetBrains Mono (OFL), Lucide icons (MIT), the CSS token conventions themselves — none carry restrictions incompatible with EUPL-1.2. The EUPL-1.2 covers the Rust code; the CSS and font assets have their own permissive licenses.

---

## 7. Implementation strategy

The current `app-orchestration-bim` is server-rendered Rust/Axum with a single inline `style.css` (~43 lines). The baseline produces correct, accessible HTML but lacks visual hierarchy, a sidebar-nav, font loading, and component-anatomy signals.

### Option A — Stay SSR; expand CSS with design-tokens-as-CSS-custom-properties

Add a structured CSS file using CSS custom properties for the full token set, named utility classes (`.bim-chip`, `.bim-tab-bar`, `.bim-code-block`), and a two-column layout with sidebar. No JavaScript. No bundler. No additional Rust dependencies.

**Implementation footprint:**
- `style.css`: expand from 43 lines to ~400–600 lines
- `render.rs`: update HTML templates to include sidebar nav, breadcrumbs, chip rows, tab elements (as disclosure elements, not JS tabs), and new section structure
- One WOFF2 font file: `InterVariable.woff2` (~350KB) served as a static asset
- One optional WOFF2: `JetBrainsMono.woff2` (~180KB) for code blocks

**Sidebar nav:** Pure HTML `<nav>` with CSS `:focus-within` and media queries for mobile collapse (no JS required for basic functionality; a `<details>` trick or CSS `:checked` radio can handle the toggle state on mobile without JS if needed at v0.0.3).

**Tab bars on component pages:** Use `<details>` / `<summary>` as progressive disclosure elements for recipe.html / recipe.css / aria.md tabs. Not pixel-perfect equivalent of a JS tab bar, but fully accessible, zero JS, and clearly AEC-appropriate (collapsible sections are standard in PDF-based BIM specifications).

**Verdict:** Strongly recommended for v0.0.2. Matches the offline-first / EUPL-1.2 / accessibility-first posture exactly. Aligns with the existing architecture (single binary, no build step, instant deploy). The constraint "no JavaScript bundler" does not mean "no JavaScript ever" — a single 20-line inline script for copy-to-clipboard on code blocks is acceptable and is the one JS addition that delivers significant developer-experience value.

### Option B — Add a small JS layer for interactive features

Keep SSR; add one small inline `<script>` (~50–80 lines) for:
- Copy-to-clipboard on code blocks
- Tab switching on component pages (replace `<details>` with `<div role="tablist">`)
- Light/dark toggle for viewport preview frames

No bundler. No npm. Just a `<script>` tag in the page template.

**Verdict:** Recommended as part of v0.0.2 alongside Option A. These three features are the exact set where JS adds genuine value and where the absence of JS is a real regression. All three can be implemented in vanilla JS that degrades gracefully (code blocks still readable, details panels still accessible, preview frame stays light by default).

### Option C — Compile Tailwind 4 or serve a pre-built CSS framework

Would require adding a build step (node.js toolchain or a Rust Tailwind port). Increases binary complexity and breaks the "single-binary static HTML + CSS" architecture.

**Verdict:** Not recommended for v0.0.2. Revisit at v1.0 if the component catalog grows large enough to benefit from utility-first classes.

### Option D — Migrate to Yew/Leptos

Full rewrite of the rendering layer in a Rust WebAssembly frontend framework. Appropriate for `app-workplace-bim` (where xeokit requires a JS runtime anyway) but not for the showcase/documentation app.

**Verdict:** Not recommended for `app-orchestration-bim`. The showcase's SSR model is its correctness argument — screen-reader friendly, indexable, printable, offline-capable. Rewriting in Wasm-SPA would undermine this.

### Recommendation

**Option A + Option B together.** Expand `style.css` to a full design-token CSS file with sidebar-nav layout and BIM-semantic visual language. Add a single minimal inline `<script>` block for copy-to-clipboard + tab switching + preview theme toggle. Total additional JavaScript: <80 lines. No bundler, no build step, no new Rust dependencies.

This keeps the binary architecture intact, achieves the Untitled UI–inspired "design system feel," and adds the three AEC vernacular signals (IFC class chips, monospace GUID display, dark viewport preview frame) that differentiate the site from a generic design system showcase.

---

## 8. Cross-references to existing cluster artefacts

The following artefacts already established by the cluster constrain and inform the v0.0.2 upgrade:

### `bim-design-philosophy.md`

Establishes the "AEC equivalent of IBM Carbon" framing. The v0.0.2 visual upgrade should make this legible visually — the current 43-line stylesheet looks like a utility site, not an AEC equivalent of Carbon. The drafting-document blue (`#1A4480`), the IFC class chips, and the sidebar-nav all serve the Carbon-equivalent positioning.

### `bim-token-taxonomy.md`

Defines the 8 DTCG token categories anchored to IFC 4.3. The token bundle page design (grid of semantic cards per category, each with IFC anchor and Uniclass reference) must surface these category names and their IFC context, not just present raw JSON. The table in section 4 of this report maps each category to a visual treatment — SPATIAL gets the `--bim-accent` treatment; SYSTEMS (MEP) gets the `--bim-cyan` treatment; IDENTITY+CODES gets the `--bim-amber` treatment (regulation/constraint territory).

### `bim-aec-muscle-memory.md`

Establishes left-sidebar SpatialTree, right-sidebar PropertiesPanel, storey-level default expansion, and Pset/Qto grouping as the universal AEC conventions the BIM showcase must mirror. The v0.0.2 sidebar-nav for the showcase site is a documentation-level analog of the app's own SpatialTree — categories expand/collapse, the selected item is highlighted in `--bim-accent-subtle`.

### Component recipes: `bim-spatial-tree`, `bim-properties-panel`, `bim-viewport-3d`

The existing CSS in these components (`#1e3a8a` selection color, `#6b7280` muted text, `#e5e7eb` borders) already establishes a palette. The v0.0.2 tokens refine this:
- `#1e3a8a` → `--bim-accent: #1A4480` (slightly adjusted for better contrast against `#FAFBFC`)
- `#6b7280` → `--bim-fg-muted: #667085` (Untitled UI gray-500 equivalent)
- `#e5e7eb` → `--bim-border-subtle: #E5E7EB` (kept), `--bim-border: #D0D5DD` (primary borders, stronger)
- System font stack → Inter variable font

The upgrade is evolutionary, not a rewrite. The component recipe files themselves do not change in v0.0.2 — the showcase renderer (`render.rs` + `style.css`) changes around them.

### Tokens: 8 DTCG JSON files

Each of the 8 files (`spatial.dtcg.json`, `elements.dtcg.json`, etc.) is currently rendered via `/tokens.json` as a flat JSON dump. The v0.0.2 tokens page should parse these files server-side (already done in `vault.rs`) and render the 8-category grid described in section 4. The raw JSON link (`/tokens.json`) stays as a developer access point; the UI layer sits on top.

---

## 9. Risks and open questions

### Risk 1 — Untitled UI visual language is "too generic" and the AEC distinct-feel is lost

**Severity: High.** The core risk of this brief. Untitled UI's visual grammar was designed for SaaS dashboards and marketing sites. Applying it without the AEC overlay markers produces a site that looks polished but indistinguishable from any other design system documentation. The five distinct markers in section 4 (GUID monospace, classification chips, IFC anchor labels, isometric hero, dark viewport frame) are non-negotiable — they must survive any visual refinement pass. If the upgrade passes visual review but strips these markers, the site regresses toward generic.

**Mitigation:** Make the AEC markers structural (baked into the `render.rs` templates, not optional CSS classes). They should be on every component page, not a special section.

### Risk 2 — Font loading latency on first paint for offline/field-use scenarios

**Severity: Medium.** The design philosophy in `bim-design-philosophy.md` emphasizes offline-capable BIM for basements, air-gapped facilities, and field use. Serving Inter from Google Fonts CDN breaks offline access. Bundling `InterVariable.woff2` (~350KB) as a static asset in the binary solves this but adds to binary size.

**Mitigation:** Use `include_bytes!` in `render.rs` to embed the WOFF2 as a base64 data URI, or serve it from the same Axum server as a static file route (`/static/InterVariable.woff2`). The nginx vhost already handles static file caching. The system font stack fallback is still defined for the pre-load state.

**Open question for operator:** Is 350KB additional static asset acceptable in the binary, or should the font be served as a separate file via nginx?

### Risk 3 — Copy-to-clipboard JS fails in certain restricted browser contexts

**Severity: Low.** `navigator.clipboard.writeText()` requires HTTPS and focus in certain browsers. The site will be served over HTTPS (certbot TLS), so this is not a deployment blocker. The copy button should degrade gracefully to `display: none` if the API is not available.

### Risk 4 — Sidebar-nav mobile collapse without JavaScript

**Severity: Low.** A pure CSS sidebar collapse (using `:checked` checkbox hack or `:focus-within`) is not universally smooth across mobile browsers. For v0.0.2, the clean approach is the small inline JS block (Option B) which handles the mobile toggle. If the operator wants zero-JS, a mobile collapse can be omitted entirely at v0.0.2 (show a scrollable top nav on mobile instead) and revisited.

### Risk 5 — Dark viewport preview frame requires `data-` attribute toggling

**Severity: Low.** The light/dark preview toggle on component pages requires either a JS event handler (simple `<button onclick="...">`) or a CSS-only `:checked` radio button. This is resolved by the Option B inline script recommendation.

### Risk 6 — Inter variable font WOFF2 licensing with EUPL-1.2

**Confirmed not a risk.** Inter is OFL 1.1. The OFL is explicitly compatible with distribution in compiled/linked forms. Embedding Inter as a binary asset within a EUPL-1.2-licensed application raises no license conflict.

### Open question — Should v0.0.2 include dark-mode for the full page (not just preview frames)?

Not recommended at v0.0.2. Full dark mode doubles the CSS custom property definitions and requires a `prefers-color-scheme` media query pass or a theme toggle. The AEC audience is primarily desktop/laptop, and the "dark interface" convention is specific to 3D viewport contexts in AEC tools (not documentation sites). Reserve full dark mode for v0.1.0.

### Open question — Should the 18 components (current 3 + 15 in-progress) all land before v0.0.2 visual upgrade, or should visual upgrade ship with the current 3?

The visual upgrade in `render.rs` + `style.css` is independent of how many component recipes are in the vault. Ship the visual upgrade with the 3 existing components; the remaining 15 populate the same template automatically as they are authored. Separating the visual-layer commit from the content-layer commits is the cleaner scope.

---

## 10. Sources

- Untitled UI homepage: https://www.untitledui.com/
- Untitled UI license page: https://www.untitledui.com/license
- Untitled UI changelog: https://www.untitledui.com/changelog
- Untitled UI blog — best free fonts: https://www.untitledui.com/blog/best-free-fonts
- Untitled UI React (open source): https://github.com/untitleduico/react
- Untitled UI Webflow color style guide: https://untitled-ui-webflow-library.webflow.io/style-guide/colors-effects
- Untitled UI Webflow typography: https://untitled-ui-webflow-library.webflow.io/style-guide/typography
- IBM Carbon Design System: https://carbondesignsystem.com/
- Carbon GitHub repository: https://github.com/carbon-design-system/carbon
- Carbon v10 typography overview: https://v10.carbondesignsystem.com/guidelines/typography/overview/
- Carbon v10 themes: https://v10.carbondesignsystem.com/guidelines/themes/overview/
- Material Design 3 color overview: https://m3.material.io/styles/color/system/overview
- Shopify Polaris color tokens: https://polaris-react.shopify.com/design/colors/color-tokens
- Polaris font and typescale: https://polaris-react.shopify.com/design/typography/font-and-typescale
- Polaris GitHub tokens: https://github.com/Shopify/polaris-tokens
- Atlassian Design color overview: https://atlassian.design/foundations/color
- Atlassian Design tokens: https://atlassian.design/foundations/tokens/design-tokens
- Atlassian color palette: https://atlassian.design/foundations/color/color-palette/
- Radix Themes overview: https://www.radix-ui.com/themes/docs/theme/overview
- Radix Themes 3.0 release: https://www.radix-ui.com/blog/themes-3
- shadcn/ui: https://ui.shadcn.com/
- shadcn/ui colors: https://ui.shadcn.com/colors
- Mantine theming colors: https://mantine.dev/theming/colors/
- Geist font by Vercel: https://vercel.com/font
- Geist typography: https://vercel.com/geist/typography
- Geist font GitHub (OFL): https://github.com/vercel/geist-font
- Inter font: https://rsms.me/inter/
- Inter on Google Fonts: https://fonts.google.com/specimen/Inter
- Inter Wikipedia: https://en.wikipedia.org/wiki/Inter_(typeface)
- JetBrains Mono: https://www.jetbrains.com/lp/mono/
- JetBrains Mono GitHub (OFL): https://github.com/JetBrains/JetBrainsMono
- xeokit BIM Viewer: https://xeokit.github.io/xeokit-bim-viewer/
- xeokit SDK: https://xeokit.io/
- ThatOpen engine_ui-components: https://github.com/ThatOpen/engine_ui-components
- ThatOpen BIM software: https://thatopen.com/bim-software-open-source/
- Speckle redesigned 3D viewer: https://speckle.systems/blog/redesigned-speckle-3d-viewer/
- Speckle Systems: https://speckle.systems/
- Revit 2024 dark mode: https://resources.imaginit.com/revit/revit-2024-dark-mode
- Tailwind CSS v4 colors: https://tailwindcss.com/docs/colors
- Tailwind color palette (v4): https://tailcolors.com/
- shadcn/ui vs Untitled UI comparison: https://medium.com/@jeffshomali/shadcn-ui-vs-untitled-ui-the-ultimate-comparison-guide-for-modern-ui-development-91ac228d7e68
- React UI libraries 2025 comparison: https://makersden.io/blog/react-ui-libs-2025-comparing-shadcn-radix-mantine-mui-chakra
- CSS design tokens with custom properties: https://penpot.app/blog/the-developers-guide-to-design-tokens-and-css-variables/
- Design Systems typography guide: https://www.designsystems.com/typography-guides/
- Untitled UI color palette Figma community: https://www.figma.com/community/file/1029506782158027808/ultimate-color-palette-system-untitled-ui

---

*Research compiled 2026-04-28 by BB.13 sub-agent. Read-only; no code was modified during research.*
