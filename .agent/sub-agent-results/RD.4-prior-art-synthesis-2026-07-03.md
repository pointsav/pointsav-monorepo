# Prior-Art Extraction: BB.13, BB.14, bim-token-strategy.md

Source files read in full:
- `/srv/foundry/clones/project-bim/.agent/sub-agent-results/BB.13-design-system-showcase-survey-2026-04-28.md`
- `/srv/foundry/clones/project-bim/.agent/sub-agent-results/BB.14-design-system-website-pick-2026-04-28.md`
- `/srv/foundry/clones/project-bim/.agent/plans/bim-token-strategy.md`

---

## PART A — BB.13: Design System Showcase Visual Upgrade Survey (2026-04-28)

**Brief origin:** Operator request for an Untitled UI 3-inspired redesign of `bim.woodfinegroup.com`.

### A.1 Sites compared / analyzed

The primary deep-dive subject was **Untitled UI** (Figma kit + open-source React library, MIT, Tailwind v4.2/React 19.2/TypeScript 5.9/React Aria). It was cross-walked against **Carbon, Material 3 (M3), Shopify Polaris, Atlassian Design**. AEC-vernacular sources were also surveyed: **xeokit, ThatOpen Engine, Speckle 3D Viewer, Bonsai (BlenderBIM), Revit 2024 (dark mode), ArchiCAD, BricsCAD**.

### A.2 Untitled UI specification details (what BB.13 catalogued)

- **Typography:** Inter (top blog recommendation, OFL 1.1). Inferred 8-step named scale from `heading-xxlarge` (48–60px/700) down to `text-size-small` (12px/400). No disclosed monospace typeface; standard Tailwind `font-mono` stack assumed.
- **Color system v6.0 (2025–2026):** More neutral, less blue-saturated gray. Gray scale `gray-25`→`gray-900` (11 steps, e.g. `gray-900 ≈ #101828`, `gray-600 ≈ #475467`, `gray-200 ≈ #EAECF0`, `gray-50 ≈ #F9FAFB`, `gray-25 ≈ #FCFCFD`). Configurable "primary" family (brand-agnostic slot), default appears mid-blue `#0066CC`–`#0057B7`. Semantic families each 11 steps: error (`#D92D20`@600), warning (`#DC6803`@600), success (`#039855`@600). Secondary accents: Indigo, Blue, Pink, Orange. Shadow tokens: xxsmall–xxlarge (7 sizes). WCAG 2.1 contrast metadata built into every color step (v6.0 differentiator).
- **Spacing:** 4px base grid, major stops 4/8/12/16/20/24/32/40/48/64/80/96/128px; named tokens `spacing-xxs`(2px) through `spacing-8xl`(80px).
- **Component anatomy:** Buttons — 5 sizes (xs/sm/md/lg/xl), variants primary/secondary-color/secondary-gray/tertiary-color/tertiary-gray/destructive, 8px radius at md/lg. Inputs — label+hint above, error below, 1px gray-300 border, 4px offset focus ring in primary-100, labels never float (matches Polaris/Carbon). Cards — white surface on gray-50 bg, 1px gray-200 border, rounded-xl (12px), md shadow, medium density (less compact than Carbon, less padded than Material). Modals — centered, max-width 640px, ~60% opacity backdrop. Nav — top nav for marketing, left sidebar (240–280px) for dashboard/app contexts. Tables — sticky headers, row hover, dense/comfortable variants. Badges — 3 sizes, semantic colors, outline+solid.
- **Icons:** Untitled UI's own set (24px default/20px compact, 2px stroke, $59 commercial). Open-source equivalent recommended: **Lucide** (MIT fork of Feather).
- **Hero treatment:** 48–72px headline, light/white bg, centered, two CTAs, product screenshot below fold, no dark hero sections in main body.

### A.3 Cross-walk table (universal DS conventions vs Untitled UI divergence)

Universal across Carbon/M3/Polaris/Atlassian/Untitled UI: semantic token layer over primitives, 4px base spacing grid, sidebar nav for dense app contexts, labels above inputs, destructive=red-same-shape, WCAG 2.1 AA minimum, focus rings, icon+label buttons.

Untitled UI diverges from all four enterprise systems via: rounder corners (8–12px vs squarer 0–4px), marketing-first templates alongside app patterns, less brand-locked primary color, more whitespace.

BB.13's abstracted "design system feel" definition: consistent 4px/8px spacing rhythm, semantic (not arbitrary) color roles, a typography scale of **exactly 6–9 steps** (not more), repeating component anatomy (same radius family/shadow tier/focus behavior), and documentation showing components in context (live preview + code).

### A.4 The "AEC industry vernacular" overlay (source of the 5 markers later adopted in BB.14)

BB.13 established these as the raw findings, from studying xeokit/ThatOpen/Speckle/Bonsai/Revit/ArchiCAD/BricsCAD:

1. **Viewport-dominated layout** — authoring tools give 60–80% of screen to the 3D/2D viewport; spatial tree/properties panel are narrow auxiliary rails.
2. **Dark UI for viewport, light UI for documentation** — Revit 2024 added dark mode for ribbon/canvas; convention is dark-or-neutral-dark viewport chrome, light property panel/spatial tree for text-data readability.
3. **Technical-data tables in Pset key/value format** — `Pset_WallCommon: FireRating = "REI 90"` style, group header + tight two-column layout (matches Revit Properties panel, ArchiCAD Info Box, Bonsai Properties Editor).
4. **IFC-anchor labels and classification chips** — displaying `IfcClass` (`IfcWall`, `IfcSlab`) and Uniclass code (`EF_25_10`) as small chips/inline labels — "alien to a generic design system but immediately recognizable to an AEC practitioner."
5. **IFC GUID display in monospace** — 22-char base64 GUIDs (e.g. `2O2Fr$t4X7Zf8NOew3FL_A`) shown small monospace — "a strong AEC identity signal."
6. **Storey-level navigation affordances** — spatial hierarchy navigator defaulting to Site/Building/Storey expand level, floor-plan thumbnail on hover (Revit/ArchiCAD pattern).
7. **AEC color conventions:** chrome/UI bg neutral gray `#2D2D2D`–`#3C3C3C` (dark) / `#F2F2F2`–`#FFFFFF` (light); selection highlight vivid blue `#0070C0` (Autodesk) or `#156EF5` (xeokit); clash/warning amber/orange `#F4A621`–`#FF8C00`; MEP elements cyan/teal; structural elements gray/ochre. Drafting-blueprint blue is medium, in the `#0A5494`–`#1F5E9B` range — NOT vivid SaaS blue.

### A.5 Concrete v0.0.2 upgrade spec proposed by BB.13 (superseded in parts by BB.14, but establishes the baseline)

- **Fonts:** Inter (sans, self-hosted `InterVariable.woff2` ~350KB) + JetBrains Mono (code/GUIDs, ~180KB), OFL 1.1 both.
- **8-step font scale:** 12/14/16/18/20/24/30/36px (`--bim-text-xs` through `--bim-text-4xl`). Line heights: tight 1.25, normal 1.55, relaxed 1.75.
- **Light-mode palette (full table):** `--bim-bg #FAFBFC`, `--bim-bg-surface #FFFFFF`, `--bim-bg-sidebar #F2F4F7`, `--bim-bg-code #F0F2F5`, `--bim-border #D0D5DD`, `--bim-border-subtle #E5E7EB`, `--bim-fg #101828`, `--bim-fg-secondary #344054`, `--bim-fg-muted #667085`, `--bim-fg-disabled #98A2B3`, `--bim-accent #1A4480` (drafting-document blue, chosen as "midpoint between US Navy Blueprint standard blue and NBS Uniclass documentation header color range," 4.6:1 contrast vs `#FAFBFC`), `--bim-accent-hover #133360`, `--bim-accent-subtle #E8EEF8`, `--bim-amber #B54708` / `--bim-amber-bg #FFF8ED` (clash/warning), `--bim-cyan #0E7490` / `--bim-cyan-bg #ECFEFF` (MEP/systems), `--bim-success #027A48` / `--bim-success-bg #ECFDF3` (IDS pass), `--bim-error #B42318` / `--bim-error-bg #FEF3F2` (violation).
- **Dark-mode (preview frames only, not full page):** `#1C2333` bg / `#242E42` surface / `#1A2030` sidebar / `#2E3D5A` border / `#E8EDF5` fg / `#4A90D9` accent. Described as "desaturated navy... reads as technical instrument rather than code editor."
- **Layout:** Sidebar-nav (not top-nav) above 768px, because "the current app has three nav links which will grow to ~20 as the component catalog fills. Sidebar-nav handles 20 items gracefully; a horizontal nav does not." 8px base grid. Content max-width **896px** (adjusted from 920px to land on grid). Sidebar **240px**. Gap 32px. Full desktop width: 240+32+896=1168px fits in 1200px viewport w/ 16px margins each side. Below 768px, sidebar collapses to hamburger.
- **Border radius:** sm 4px (inputs/badges), md 6px (buttons/cards), lg 8px (panels/modals) — "tighter than Untitled UI's 8–12px to signal 'technical' rather than 'consumer SaaS.'"
- **Component-recipe page anatomy** (example given: `bim-spatial-tree`): breadcrumb → page title (Inter 36px/700) → description → classification chip row (`IfcSpatialElement`, `Universal AEC`, `Uniclass SL`, `data-mode: workplace|console`) → sticky tab bar (Preview / recipe.html / recipe.css / aria.md / IFC mapping) → preview frame (light/dark toggle) → code blocks (JetBrains Mono 13px, copy button) → IFC mapping table (Token name | IFC Entity | IFC Anchor | Uniclass Code | Notes) → sticky right-sidebar TOC + related components.
- **Token bundle page:** 8 DTCG categories as a **grid of semantic cards** (2-col desktop/1-col mobile), each with category icon glyph, name, IFC anchor link, description, token-count badge, and a closed-by-default `<details>` disclosure containing the raw JSON (JetBrains Mono) — explicitly "no full JSON dump on page load."

### A.6 The 5 "distinct-from-project-design markers" as originally stated in BB.13 §4

1. IFC GUID display in monospace chrome — IFC class anchor label in JetBrains Mono 11px on every component page; sample GUID shown in properties panel header of previews.
2. Classification chips — IFC class + Uniclass code chip row on every component page, e.g. `[IfcBuiltElement]` `[EF_25_10 Walls]`.
3. Hero illustration: isometric building mass — homepage hero replaces the flat count block with a minimalist isometric building volume outline (storeys as horizontal lines), single inline SVG ~30 lines, 300px width, on `--bim-bg-sidebar` panel. "No 3D engine required... differentiates from the generic 'dashboard screenshot' hero used by Untitled UI, Mantine, and shadcn documentation sites."
4. Storey navigator breadcrumb — component-specific breadcrumb for SpatialTree etc. showing `Site / Building A / Ground Floor`, using the `═` storey icon rather than a generic chevron separator.
5. Dark viewport preview frame — Viewport3D (and optionally SpatialTree) page renders its preview inside a `data-preview-theme="dark"` frame (`#1C2333`), surrounding docs stay light — "familiar from Revit, xeokit, and ThatOpen's own documentation screenshots."

### A.7 Untitled UI licensing analysis

Three products: Figma kit (commercial $129–$349), React library (MIT open-source), icon set (commercial $59). Figma license prohibits creating competing UI kits/libraries/templates, prohibits redistribution/sale, prohibits exposing raw React source code, carries a **$10,000 USD per-breach enforcement clause**. BB.13's conclusion: taking visual *inspiration* (scale/palette logic/anatomy patterns) without copying the Figma file or React source does NOT trigger these restrictions, since visual conventions (radius, spacing grid, typography scale structure, sidebar layout) are not copyrightable. Caution flagged: never extract hex/spacing values verbatim from a purchased Figma file — BB.13's own palette was derived from public docs/community screenshots/OSS code, not the commercial file.

### A.8 Open-source alternatives table

shadcn/ui (MIT, closest match — neutral gray palette, Inter default, 8px radius, same `gray-50`→`gray-950` token structure), Radix Themes (MIT), Mantine 7.x (MIT), Park UI (MIT), Geist UI/Vercel (MIT), NextUI/HeroUI (MIT), Saas UI (partial MIT). **Recommendation: shadcn/ui's visual language as the cleanest open-source anchor**, but only as a CSS-variable-naming/visual-grammar reference — not an import — since the BIM showcase is server-rendered Rust/Axum with no React runtime.

### A.9 Implementation strategy (Options A–D)

- **Option A (recommended):** Stay SSR; expand `style.css` (43→400-600 lines) with CSS custom properties, named utility classes, two-column sidebar layout. No JS, no bundler.
- **Option B (recommended, alongside A):** Add one small inline `<script>` (~50–80 lines) for copy-to-clipboard, tab switching (`<details>`→`role="tablist"` if desired), and light/dark preview toggle. "These three features are the exact set where JS adds genuine value."
- **Option C (not recommended for v0.0.2):** Compile Tailwind 4 or a build-step CSS framework — breaks single-binary architecture.
- **Option D (not recommended):** Migrate to Yew/Leptos — appropriate for `app-workplace-bim`, not the showcase, since SSR is the showcase's correctness argument (screen-reader friendly, indexable, printable, offline-capable).
- **Final verdict: Option A + Option B together.** Total added JS <80 lines, no bundler, no new Rust dependencies.

### A.10 Cross-references to existing cluster artefacts

- `bim-design-philosophy.md` — establishes "AEC equivalent of IBM Carbon" framing; v0.0.2 upgrade should make this visually legible.
- `bim-token-taxonomy.md` — defines the (at the time) **8 DTCG token categories** anchored to IFC 4.3; token bundle page must surface category names + IFC context.
- `bim-aec-muscle-memory.md` — establishes left-sidebar SpatialTree / right-sidebar PropertiesPanel / storey-level default expansion / Pset-Qto grouping as universal AEC conventions to mirror.
- Existing component recipe CSS values being refined: `#1e3a8a`→`--bim-accent:#1A4480`; `#6b7280`→`--bim-fg-muted:#667085`; `#e5e7eb` kept as `--bim-border-subtle`, new `--bim-border:#D0D5DD`.

### A.11 Risks flagged (BB.13 §9)

1. **High severity — "Untitled UI visual language is too generic and AEC distinct-feel is lost."** The 5 markers are called "non-negotiable — they must survive any visual refinement pass... Make the AEC markers structural (baked into `render.rs` templates, not optional CSS classes). They should be on every component page, not a special section."
2. Medium — font loading latency for offline/field-use (mitigated via `include_bytes!`/static route).
3. Low — clipboard API HTTPS requirement (non-blocker, site is HTTPS).
4. Low — sidebar mobile collapse without JS (resolved via Option B).
5. Low — dark preview toggle needs `data-` attribute JS handling (resolved via Option B).
6. Confirmed not a risk — Inter OFL 1.1 vs EUPL-1.2 compiled-binary distribution, no conflict.
7. Open question — full-page dark mode: **not recommended at v0.0.2**, reserved for v0.1.0.
8. Open question — should all planned components land before visual upgrade ships: **no**, visual upgrade should ship independent of catalog size ("the visual upgrade in `render.rs` + `style.css` is independent of how many component recipes are in the vault... the remaining 15 populate the same template automatically").

---

## PART B — BB.14: Design System Website Pick for bim.woodfinegroup.com (2026-04-28)

**Brief origin:** Operator constraint — "original copy" of two best-in-class design system websites, one per surface (design.pointsav.com and bim.woodfinegroup.com); **bankers must be able to distinguish the two at a glance.**

### B.1 Full landscape survey — 14 sites inspected via live URL fetch

| System | URL | Typography spine | Primary palette anchor | Layout shape | Signature |
|---|---|---|---|---|---|
| IBM Carbon | carbondesignsystem.com | IBM Plex Sans + IBM Plex Mono | `#0f62fe` (blue-60) | Top bar 48px + left accordion sidebar 256px + 56rem content | Dense structured tables; near-zero radius; indigo-heavy interactive layer |
| Material Design 3 | m3.material.io | Google Sans + Roboto | `#6750A4` (baseline purple) | Top nav + right rail | Expressive rounded shapes (28px+ radius); tonal fills |
| Shopify Polaris | polaris-react.shopify.com | Inter (tight line-height) | `#008060` (green) | Top nav + left sidebar (deprecated; rebuilding) | Merchant-workflow focus; compact tables |
| Atlassian Design | atlassian.design | Atlassian Text (Inter-like) + Charlie Display | `#0052CC` (blue-700) | Top nav + left sidebar | Foundation cards w/ illustrations; warm-neutral gray; 4-tier elevation shadow |
| Adobe Spectrum | spectrum.adobe.com | Adobe Clean + Adobe Clean Mono | `#2680EB` (blue-500) | Top nav + left sidebar (collapsible) | Instrument-panel density; systematic gray; consistent 4px radius; dark code chrome |
| GitHub Primer | primer.style/product | Mona Sans (condensed variable) | `#0969DA` (blue) | Top nav + left sidebar | Mona mascot hero; card-grid layout; condensed heading weight |
| Microsoft Fluent 2 | fluent2.microsoft.design | Segoe UI Variable | `#0078D4` (blue) | Top nav (multi-platform) | Animated hero; platform tabs (Web/iOS/Android/Windows) |
| Vercel Geist | vercel.com/geist/introduction | Geist Sans + Geist Mono | `#000000`/zinc scale | Left sidebar (collapsible) + top bar | Monochrome; developer-first; code-heavy |
| Stripe Press | press.stripe.com | Sohne + system serif | N/A (book catalog) | Centered single-column | Book gallery; ink-on-paper feel; NOT a DS site |
| Linear | linear.app | Undisclosed sans | `#5E6AD2` (mid-purple) | Top nav, full-width hero | Dark gradient bg; AI-agent workflow viz; NOT a DS site (marketing site) |
| shadcn/ui | ui.shadcn.com | Geist Sans (default) | Near-black + neutral gray | Top nav + left sidebar | Dual light/dark screenshots in hero; copy-paste framing |
| Radix Themes | radix-ui.com/themes | System sans | Configurable (default indigo) | Top nav | Instant-usability hero w/ code snippet |
| Mantine | mantine.dev | System sans (-apple-system) | `#228BE6` (blue) | Top nav | 120+ interactive component demos embedded inline |
| Salesforce Lightning | lightningdesignsystem.com | Salesforce Sans (custom) | `#0176D3` (blue) | Top nav + left sidebar | Dense enterprise-CRM structure; zero radius |

**Ground-truth Carbon confirmation (from reading `design.pointsav.com`'s actual CSS):** Inter font, `--ps-primary-60: #234ed8` interactive primary (navy-leaning vs Carbon's `#0f62fe`), `--ps-sidebar-width: 16rem` (256px), top bar `3rem` (48px), content max `56rem` (896px), near-zero radius (0.125–0.5rem), dark code blocks (`--ps-neutral-100`).

### B.2 AEC audience resonance — full scoring table

| System | AEC resonance | Reasoning |
|---|---|---|
| IBM Carbon | **High** | Dense data, near-zero radius, monospace code, instrument-panel feel; IBM has long engineering-software presence. Negative: IBM Plex feels distinctly IBM, not AEC-neutral. |
| Material Design 3 | **Low** | Expressive rounded shapes, tonal fills, Android-first patterns "entirely foreign to AEC visual culture." |
| Shopify Polaris | **Low** | Merchant-admin conventions (ResourceList, inventory) are a different domain; green reads as "retail." |
| Atlassian Design | **Medium** | Jira ancestry recognizable in construction PM context; warm gray more neutral. But blue+elevation shadows read as "project-management SaaS" not "building engineering." |
| **Adobe Spectrum** | **Very High** | "The most AEC-resonant of any candidate." Designed for Creative Cloud desktop tools (Photoshop/Illustrator/Premiere/InDesign) used at same organizational tier as Revit/ArchiCAD. Instrument-panel density, dark preview chrome, systematic gray, 4px radius, Clean Mono all match AEC authoring-tool grammar. Tabbed component pages mirror Revit's Property Editor conventions. |
| GitHub Primer | **Medium-Low** | Developers/BIM managers recognize it, but Mona mascot + card-grid reads as developer-culture, not AEC. |
| Microsoft Fluent 2 | **Medium** | MS Office universal in AEC offices, Segoe familiar — but animated hero + consumer-app orientation foreign. |
| **Vercel Geist** | **High** | "Engineer-first, code-heavy, monochrome... reads as 'technical reference' rather than SaaS marketing." Lacks domain markers but provides "cleanest engineering-adjacent canvas" to add them onto. |
| Stripe Press | High (typography only) | Sohne + whitespace + ink-on-paper resonates with drafting-document aesthetic, but it's a publication catalog, not a DS site — low-density single-column can't accommodate component/token/code density needed. Kept only as typographic inspiration. |
| Linear | Medium | Dark monochrome resonates with dark-mode AEC tool users; AI-agent marketing feels startup, not AEC. |
| shadcn/ui | Medium | Clean/neutral copy-paste framing recognizable; React-centric SaaS dashboard screenshots foreign to AEC. |
| Radix Themes | Low | "Just import and go" hero is developer-focused; no AEC resonance. |
| Mantine | Low-Medium | Inline component demos impressive to devs but "entirely foreign" — AEC practitioners never use live component demos daily. |
| Salesforce Lightning | Medium | Dense enterprise tables/sidebar/token docs read as technical reference (AEC-adjacent), but Salesforce/CRM brand recognizable and foreign. |

**Top three AEC-resonant candidates named explicitly: Adobe Spectrum (first), Vercel Geist (second), IBM Carbon (third — but already taken by design.pointsav.com).**

### B.3 The bankers' distinguishability test — full methodology and scoring

**Methodology:** design.pointsav.com (Carbon-shape) and bim.woodfinegroup.com (candidate) opened side-by-side in browser tabs at **thumbnail scale (~200×150 pixels)**. Three axes scored 1–5 each (1 = nearly identical to Carbon at thumbnail; 5 = unmistakably different), summed to a **/15 total**:

**Carbon-shape reference characteristics at thumbnail scale (what's being distinguished from):** distinctive indigo `#234ed8` in sidebar active states/interactive elements; white page bg; muted gray sidebar; Inter typography (geometric, neutral, slightly wide); 3rem top bar + 16rem left sidebar accordion; near-zero radius everywhere; visible sidebar-content separation line.

| Candidate | Color identity (1–5) | Typography spine (1–5) | Chrome shape (1–5) | Total /15 | Notes |
|---|---|---|---|---|---|
| IBM Carbon | 1 | 1 | 1 | **3** | identical, disqualified |
| Material Design 3 | 4 | 3 | 5 | **12** | very distinct, but wrong audience |
| Shopify Polaris | 3 | 2 | 2 | **7** | both use Inter; similar sidebar |
| Atlassian Design | 3 | 2 | 3 | **8** | similar blue family; warmer gray |
| **Adobe Spectrum** | **5** | **4** | **5** | **14** | very distinct: dark chrome, Adobe Clean, tabbed density |
| GitHub Primer | 3 | 5 | 3 | **11** | Mona Sans condensed clearly different; same blue family |
| Microsoft Fluent 2 | 3 | 3 | 3 | **9** | similar horizontal nav |
| **Vercel Geist** | **5** | **4** | **4** | **13** | monochrome vs indigo; Geist Sans vs Inter; different chrome |
| shadcn/ui | 3 | 3 | 3 | **9** | both use Geist/Inter; similar sidebar structure |
| Mantine | 2 | 2 | 3 | **7** | both system-sans; similar nav structure |
| Salesforce Lightning | 4 | 3 | 3 | **10** | distinct Salesforce blue; different heading weight |

**Distinguishability leaders confirmed: Adobe Spectrum (14/15), Vercel Geist (13/15).** Both score far above the Carbon baseline (3/15) and are stated to be "mutually distinguishable from Carbon at thumbnail scale." (This confirms the "14/15" figure referenced in the extraction brief — it is Adobe Spectrum's score, with the breakdown Color 5 + Typography 4 + Chrome 5 = 14.)

### B.4 The two picks

**For design.pointsav.com:** Confirmed keep IBM Carbon-family (already shipped). Rationale: Carbon was designed for the same "developer + creative designer at regulated business" audience; token layer matches developer mental model; Inter is the industry-reference DS sans; dark code blocks signal "technical reference, not marketing." Only note: keep the current `#234ed8` (not migrate to Carbon's exact `#0f62fe`) — differentiates PointSav from IBM while staying in the Carbon family.

**For bim.woodfinegroup.com: Adobe Spectrum — specifically "the instrument-panel aesthetic... specifically the Spectrum 2 documentation site's visual grammar."** Recommendation: use Spectrum as the "original copy" source, mirroring structural grammar (layout, chrome shape, component-page anatomy) while substituting Woodfine brand identity and AEC domain markers.

**5 observable differences between Spectrum and Carbon at thumbnail scale (BB.14 §4b, verbatim substance):**
1. Adobe Clean (humanist, wider letterforms, open counters) vs IBM Plex Sans/Inter — visible heading weight/letter-proportion difference at thumbnail.
2. Spectrum primary `#2680EB` (lighter sky-blue) vs Carbon `#0f62fe` electric indigo / PointSav `#234ed8` navy.
3. Spectrum's darker sidebar/panel bg (`#F5F5F5`–`#E8E8E8` light mode; `#1D1D1D` dark mode) with visible panel-border, vs Carbon's light-gray sidebar.
4. Spectrum's tabbed panel anatomy (Overview/API/Examples) with darker tab-bar background creating a visible horizontal chrome band — absent from Carbon's lighter accordion.
5. Spectrum code blocks use Adobe Clean Mono on dark backgrounds — visually different from Carbon's SFMono.

**Spectrum's licensing posture:** Apache 2.0 for the Spectrum Web Components library, MIT for many token assets. Structural grammar (layout, anatomy, spacing, chrome) is not copyrightable and freely borrowable. **Adobe Clean font itself is proprietary and must be substituted** (done via Source Serif 4 + Geist Sans/Mono — see B.6).

### B.5 "Original copy" strategy — patterns mirrored exactly vs. patterns deviated from

**Mirrored exactly (structural grammar, not copyrightable):**
- Left sidebar ~272px wide, categorical tree nav
- Top bar 48px, logo left, search+theme-toggle right
- Content area 896px max-width, 32px horizontal padding
- Full desktop total: 272+16+896 = 1184px
- Component page anatomy: version/status badge near component name → horizontal tab bar with slight dark-panel bg (tabs: Preview / recipe.html / recipe.css / aria.md / IFC mapping) → preview frame with light/dark toggle → token reference table below → accessibility notes as collapsible `<details>` at bottom
- Chrome: darker sidebar bg than page bg (`#EFEFEF`–`#E5E5E5` light mode, Spectrum uses `#E8E8E8`), visible 1px sidebar/content border, **4px corner radius on all interactive elements** (not 0px like Carbon, not 8px+ like shadcn/ui), tab bar `#F0F0F0`–`#E8E8E8` panel bg with bottom border
- Code blocks: dark bg `#1A1A1A`–`#222222`, mono 13px, copy-to-clipboard top-right, "visible syntax-region differentiation even without a highlight library (foreground at 80%/60% opacity creates sufficient keyword/string/comment contrast without full syntax highlighting)"
- Navigation: categorical collapsible tree, active item 2px left-border accent, category headers small-caps/heavier weight, sidebar search at top (filterable, pure CSS or small JS)
- Prose: h2 24px, h3 18px, body 16px/1.6 line-height, subtle h2 border-bottom separator

**Deviated from (Spectrum→Woodfine brand-swap table):**

| Spectrum original | bim.woodfinegroup.com replacement |
|---|---|
| Adobe Clean font | Source Serif 4 (display headings) + Geist Sans (UI/body) + Geist Mono (code) |
| `#2680EB` sky blue primary | `#1A4480` drafting blue |
| Adobe logo/wordmark | Woodfine wordmark + "BIM Design System" descriptor |
| Adobe favicon | Woodfine logomark / geometric building-section SVG |
| "Spectrum 2" version badge | IFC class chip (`IfcWall`, `IfcSlab`) — "the BIM analog of a version/status badge" |
| "Accessibility" tab label | "aria.md" tab label |
| Spectrum blue left-border active indicator | `#1A4480` drafting blue active indicator |
| Spectrum sky-blue chip backgrounds | `#E8EEF8` (IFC class) / `#ECFEFF` (Uniclass/MEP) / `#FFF8ED` (warning/constraint) |

### B.6 The 5 AEC-specific structural markers under the Spectrum pick (BB.14 §5c) — full list

BB.14 explicitly restates and layers all 5 BB.13 markers on top of the Spectrum chrome:

1. **IFC GUID display in monospace.** Every component preview shows an example GUID (`2O2Fr$t4X7Zf8NOew3FL_A`) in **Geist Mono** (updated from JetBrains Mono in BB.13) at 11px, `--bim-fg-muted` color; in the Properties Panel preview, GUID appears in the header row. "These belong in the preview frame, not in the documentation prose."
2. **Classification chips.** Component page header row, immediately below component name: `[IfcBuiltElement]` in `--bim-accent-subtle` bg, `[EF_25_10 Walls]` in `--bim-cyan-bg`, and where applicable `[IDS 1.0 constraint]` in `--bim-amber-bg`. These **replace Spectrum's version/status badge positions**. "At thumbnail scale, these chips are visible as small colored rectangles in a row — distinguishable from Carbon's approach of a single text breadcrumb."
3. **IFC anchor labels on token categories.** On the tokens overview page, each of the 8 DTCG category cards carries a small IFC anchor link in 11px Geist Mono (`IfcSpatialElement`, `IfcBuiltElement`, etc.) linking to buildingsmart.org — "visible as a monospace label beneath each category name on the grid."
4. **Isometric building-mass hero SVG.** Homepage hero uses an inline SVG isometric building-mass outline — simple geometric stack of floor plates with grid-like facade pattern, ~280×240px, `--bim-accent` stroke on `--bim-bg-sidebar` fill. Replaces the v0.0.1 substrate-marketing count-block. "No other design system homepage uses axonometric architectural geometry."
5. **Dark viewport preview frame.** Viewport3D page (optionally SpatialTree) renders preview inside `data-preview-theme="dark"` frame (`#1C2333` bg); surrounding docs stay light. Under the Spectrum pick this is noted as "already structurally supported by Spectrum's own light/dark preview toggle pattern on component pages — the mechanism is borrowed, the content is AEC-specific."

### B.7 Full concrete visual specification table (BB.14 §6 — verbatim values)

| Element | Concrete value |
|---|---|
| Sans — display/heading | **Source Serif 4** (Google Fonts, OFL 1.1), h1–h2 only; weight 400 for page titles, 600 for section headings. Rationale: "technical-publication lineage... aligns precisely with the AEC practitioner's familiarity with engineering standards documents, building code PDFs, and NBS specification formats." |
| Sans — UI/body | **Geist Sans** (Vercel/Basement Studio, OFL 1.1), 16px body. "Reads as 'precision engineering' rather than 'SaaS web app.'" Fallback: `'Geist', -apple-system, BlinkMacSystemFont, 'Segoe UI Variable', sans-serif`. |
| Mono | **Geist Mono** (OFL 1.1) — same family as Geist Sans for visual cohesion; legible at 11–13px for GUIDs/token values. Fallback: `'GeistMono', 'SFMono-Regular', Menlo, Consolas, monospace`. |
| Serif | Source Serif 4, h1/h2 only — "the single most effective differentiator from Carbon-shape (zero serif) and shadcn/ui/Geist (sans throughout)." Cites NBS Specification, CIBSE Guides, Approved Documents as using serif headings + sans body. |
| Primary anchor | `#1A4480` drafting-document blue — "range used by US federal engineering documentation standards (FHWA manuals, ASHRAE handbooks) and NBS Uniclass color scheme." Darker/more authoritative than Spectrum's `#2680EB`, less electric than Carbon's `#0f62fe`. Contrast vs `#FAFBFC`: **~7.8:1** (exceeds WCAG AA all text sizes). |
| Surface neutrals | `--bim-bg: #FAFBFC`; `--bim-bg-surface: #FFFFFF`; `--bim-bg-sidebar: #EFEFEF` (Spectrum-match, darker than BB.13's `#F2F4F7`); `--bim-bg-panel: #E8E8E8` (tab bar/preview header); `--bim-bg-code: #1A1A1A` (dark, Spectrum-match) |
| Amber (warning) | `#B54708`, WCAG AA vs `#FFF8ED` bg |
| Cyan (MEP) | `#0E7490`, used for Uniclass MEP-family chips (`SL_`, `SL_25_`) |
| Green (validation) | `#027A48`, "IDS 1.0: PASS" status chips |
| Font size scale | `--bim-text-xs: 0.6875rem` (11px, GUIDs/Uniclass labels/chip text); `--bim-text-sm: 0.8125rem` (13px, code/sidebar/table cells); `--bim-text-base: 1rem` (16px, body); `--bim-text-lg: 1.125rem` (18px, lead paragraph); `--bim-text-xl: 1.25rem` (20px, h3); `--bim-text-2xl: 1.5rem` (24px, h2); `--bim-text-3xl: 1.875rem` (30px, h1 sans); `--bim-text-display: 2.25rem` (36px, hero headline in Source Serif 4) |
| Spacing scale | 4px base, 10 stops: 4/8/12/16/20/24/32/40/48/64px |
| Border radius | none (0 — table cells/property-panel rows, "AEC data-table convention"); sm 4px (chips/badges/inputs — mirrors Spectrum); md 6px (buttons/cards); lg 8px (preview frames/panels/modals) |
| Layout grid | Sidebar 272px; content max 896px; gap 16px; page padding 32px horizontal; full desktop (1280px+): 272+16+896+48px margins each side |
| Chrome flavor | Left sidebar (sticky) + top bar 48px (sticky). Logo top-left. Search input w/ Ctrl+K hint (pure HTML `<input type="search">`, no JS framework). Sidebar categorical tree, collapsible `<details>` per category, 2px left-border active indicator in `--bim-accent`. No hamburger at full desktop; appears `<768px`. |

### B.8 Direct comparison table: design.pointsav.com (Carbon) vs bim.woodfinegroup.com (Spectrum)

| Axis | design.pointsav.com | bim.woodfinegroup.com |
|---|---|---|
| Sans | Inter | Geist Sans |
| Mono | SFMono-Regular/system | Geist Mono |
| Serif | None | Source Serif 4 (h1–h2 only) |
| Primary anchor | Indigo `#234ed8` | Drafting blue `#1A4480` |
| Surface | White `#ffffff` page, `#f5f6f8` sidebar | Warm off-white `#FAFBFC` page, `#EFEFEF` sidebar panel |
| Sidebar width | 16rem (256px) | 272px |
| Content max | 56rem (896px) | 896px |
| Corner radius | Near-zero (0.125–0.5rem) | 4px chip / 6px button / 8px panel |
| Code blocks | Dark `#0e0f12`, SFMono | Dark `#1A1A1A`, Geist Mono |
| Hero | Headline + component count stats block | Serif display headline + isometric building-mass SVG + minimal count row |
| Tab bar | Light, bottom-border active indicator | `#E8E8E8` panel bg (darker, instrument-panel) |
| AEC markers | None — generic DS | IFC class chips, Uniclass MEP chips, IFC GUID monospace, storey navigator breadcrumb, dark viewport frame |

### B.9 Explicit statements on nav structure / card density / assumed catalog size

**This is the key finding relevant to the 24-category growth question.** BB.14 does NOT independently re-derive nav sizing assumptions — it explicitly inherits and confirms BB.13's sidebar rationale (see A.5 above: "the current app has three nav links which will grow to ~20 as the component catalog fills. Sidebar-nav handles 20 items gracefully; a horizontal nav does not"). BB.14's only numeric change to BB.13's layout is the sidebar width (240px→272px, purely a Spectrum-chrome match) and confirms content-max stays 896px "same, different derivation."

BB.14 §8 implementation-delta section explicitly states under "Sidebar width adjustment": *"BB.13 used 240px; this report specifies 272px (Spectrum-match). `render.rs` sidebar HTML generation is unchanged; only the CSS variable `--bim-sidebar-width` changes... No layout logic changes."* — i.e., **BB.14 did not revisit or re-test the categorical-tree/collapsible-`<details>`-per-category nav pattern against a larger catalog size; it assumed the same ~20-item scale BB.13 assumed.**

On card density: the only card-density spec present is BB.13's token-bundle-page card grid (2-col desktop/1-col mobile, one card per DTCG category) — this was designed against **8 DTCG token categories** (explicit: "The 8 DTCG token categories should be surfaced as a grid of semantic cards"). BB.14 does not revise this card grid or its assumed category count; it inherits it verbatim (§9 open question about Uniclass code format references "the 8 DTCG token categories" implicitly via cross-reference to `bim-token-taxonomy.md`, and BB.14's own body text never updates the count).

**Conclusion for the redesign planning:** Both documents' nav and card-density assumptions were built for a catalog of **~20 nav items and 8 token categories** (BB.13's explicit target — "will grow to ~20"). Neither document was written against, or tested at, the current 24-category scale. The sidebar-vs-top-nav *decision* (sidebar wins because it "handles 20 items gracefully; a horizontal nav does not") should still hold directionally at 24 categories, but the **specific 2-column token-category card grid** (designed for 8 cards) and the flat categorical-tree sidebar pattern (designed for ~20 leaf items) were not stress-tested at 24+ categories and may need collapsing/grouping/sub-navigation treatment BB.13/BB.14 never specified. This is a genuine gap for the redesign round to address, not an oversight to be inferred from silence — it's an explicit "no update from BB.13's ~20-item assumption" in BB.14's own delta section.

### B.10 Implementation deltas from BB.13 (BB.14 §8, full list)

1. **Font loading:** Inter+JetBrains Mono (BB.13, ~530KB) replaced by Geist Sans (~280KB) + Geist Mono (~180KB) + Source Serif 4 (~420KB, weights 200–900) = **~880KB total**, a ~350KB increase. Mitigations offered: Latin-extended subsetting (30–40% reduction each) or CDN-fallback for Source Serif 4 only while self-hosting Geist. Operator's offline-first preference favors self-hosting all three with subsetting.
2. **Sidebar width:** 240px (BB.13) → 272px (BB.14, Spectrum-match). CSS-variable-only change.
3. **Tab bar styling:** light (BB.13) → `#E8E8E8` panel bg with 1px border-bottom (BB.14). CSS-only; HTML structure (`<details>`/`<summary>`) unchanged; JS toggle (Option B) unchanged.
4. **Source Serif 4 for h1–h2:** requires `render.rs` heading templates to carry a class (e.g. `bim-display-heading`) distinguishing h1/h2 (serif) from component/token `<h3>` names (stay Geist Sans). Estimated ~10 lines of change.
5. **Isometric SVG hero:** replaces the `counts` `<dt>`/`<dd>` block in `render.rs`'s homepage hero; ~30–40 lines of inline SVG path data, static, compile-time embedded, uses `var(--bim-accent)`/`var(--bim-bg-sidebar)`.
6. **Nothing else changes:** SSR architecture (Axum port 9096), `vault.rs`, `main.rs` route table, systemd unit, nginx vhost all unaffected. **"Verdict: BB.13's Option A + Option B recommendation stands. The Spectrum pick is a CSS and font-asset change, not an architecture change."**

### B.11 Risks (BB.14 §9, full list)

1. **Medium — Source Serif 4 at display sizes may feel unexpected.** Mitigation: serif confined to h1/h2 only; single CSS-rule revert available if operator rejects it on first render.
2. **Low — Geist Sans less universally recognized than Inter.** Framed as a *feature* for distinguishability ("visibly not Inter = visibly not Carbon-family"). Licensing confirmed OFL 1.1, EUPL-1.2 compatible.
3. **Low-Medium — Spectrum's actual site is React/Lit-rendered**; structural patterns require SSR/HTML translation. Base structure said to degrade gracefully; only cosmetic interaction details (animated tab transitions) may not translate perfectly.
4. **Low — font payload increase (~880KB vs ~530KB).** Subsetting reduces to ~350KB combined; CDN fallback option noted for Source Serif 4 only.
5. **Low — "Spectrum-shape" may read as "Adobe" to a designer-literate banker.** Mitigation: AEC markers (chips, isometric hero, wordmark, drafting blue) should override; if concern persists, increase visual weight of the serif+Geist typographic signature over the chrome shape.
6. **Open question — hero headline serif size:** recommend Source Serif 4 at 36px display size at the homepage hero itself (not just internal h1/h2) — "the most visible position for establishing typographic identity... reverting to Geist Sans at the hero is a one-line CSS change" if operator dislikes it on first render.
7. **Open question — Uniclass code format** (`SL_25_10` vs `SL 25-10`) — a content question, not styling; must match production bSDD URI format used in `service-codes`, confirm against `service-codes` NEXT.md.

---

## PART C — bim-token-strategy.md (2026-05-17) — non-branding/non-domain content only

Per the task scope, branding and domain decisions (Parts II, IV, VII, VIII, X on Woodfine/PointSav naming, governance foundation structure, revenue trajectory, consortium path, "who publishes" decision) are **out of scope** for this extraction and are omitted except where they directly touch the requested workflow-primitive / phased-rollout / product-feel content below.

### C.1 Library-first, then CMS — phased rollout logic (Part VI, full detail)

**Recommendation stated verbatim in substance:** "open library first (Phase 1), CMS second (Phase 2), only if Phase 1 achieves ≥100 firm adoption."

Three models compared:

| Model | Description | Risk |
|---|---|---|
| A — Open library only | `@pointsav/bim-tokens`, Apache 2.0, npm, Style Dictionary transforms | No commercial capture; community risk without governance |
| B — CMS only | Proprietary token management platform, tokens as entry-point | Zero adoption; no open ecosystem to pull from |
| **C — Both (recommended)** | Open library establishes format; CMS provides operational workflow surface | Requires discipline not to build CMS before library is proven |

**Phase 1 (open library) — product surface:**
- Publish `@pointsav/bim-tokens` on npm, Apache 2.0, IFC 4.3 anchored, DTCG 2025.10 format
- Ship reference serializers: DTCG → IFC PropertySet, DTCG → DTDL, DTCG → Brick instance
- Ship a **Revit add-in**: JSON token catalog → Shared Parameters `.txt` file + IFC mapping table
- Register token identifiers with bSDD as a custom dictionary ("not a fork — a publication")
- Engage OSArch, Speckle, That Open Company communities

**Phase 2 (hosted dashboard — free, lightweight CMS) — product surface:**
- Web UI for browsing/searching the token catalog
- Hosted per-project token override files (tenant fit-out variant of base token set)
- IDS validation: upload an IFC file, validate against the token set, get a report
- Targets small-firm/single-project adoption (the "tail" of the AEC market)

**Phase 3 (productized CMS — paid) — product surface:**
- Full workflow: token lifecycle management, approval gating, audit trail
- Portfolio rollup for ESG reporting (Scope 3 Cat 13 — described as "the money")
- Multi-tenant architecture: one token catalog per landlord, token override files per tenant
- Gated explicitly on Phase 1 reaching ≥100 firm adoption

**Critical anti-patterns called out:**
- Do not lead with the CMS pitch — the open library must prove itself first
- Do not build a closed token catalog — it must be openly forkable for adoption
- Must explicitly position relative to bSDD — "not a competing catalog, a DTCG consumption layer"

### C.2 SaaS-justifying workflow primitives (Part X, full list with detail)

The document frames these explicitly as what "justif[ies] SaaS pricing," with a pricing comparable cited: **Supernova-comparable $49–$99/editor/month team tier; enterprise $20K–$100K+ ACV.** The six primitives, each given a one-line functional description in the source:

1. **Multi-stakeholder review workflows for token changes** — approval/review process gating changes to the token set (implied: multiple parties — e.g., landlord, tenant, BIM consultant — must sign off before a token change is committed/published).
2. **IDS export** — "the same token set that defines tokens validates IFC files" — i.e., the token catalog doubles as an Information Delivery Specification (IDS) file that can be run against an uploaded IFC model to check compliance. (Cross-referenced again in Phase 2 scope as "upload an IFC file, validate against the token set, get a report.")
3. **IFC 4.3 anchoring validation service** — a validation service confirming each token's IFC anchor (the `$extensions` reference to an IFC entity/Pset property, per Part VII's positioning statement) is correctly mapped/valid against the IFC 4.3 schema.
4. **Automated DTCG → IFC property set mapping** — automated (not manual) generation of the mapping from a DTCG token definition to its corresponding IFC PropertySet, described elsewhere (Part V) as "the core bridge" among six platform bridges (also DTCG→DTDL for Azure Digital Twins, DTCG→RDF/Turtle for Brick/RealEstateCore via SPARQL transform, DTCG→USD attributes for NVIDIA Omniverse, DTCG→geodatabase attributes for ESRI ArcGIS Indoors).
5. **WORM provenance audit trail** — described as "BCSC-defensible" (i.e., built to withstand BC Securities Commission scrutiny) — a write-once-read-many audit log of token changes/provenance, positioned as part of `os-privategit`'s horizontal infrastructure (private git with WORM provenance, F12 console, sovereignty posture) that `app-privategit-bim` (the BIM vertical) sits on top of.
6. **Private token overlays on top of the open base set (tenant fit-out variants)** — the mechanism by which a landlord's private/proprietary token customizations (e.g., a specific tenant's fit-out variant) layer on top of the public open base token catalog without forking it. This is the same mechanism referenced in Phase 2 ("hosted per-project token override files") and in Part II's core use case (tenant fit-out delivered "as a token override file," triggering the digital twin to "recompose the affected spaces" so IWMS/BMS/ESG-reporting systems all pick up the change from one source).

**Additional actionable detail an implementer would need, drawn from elsewhere in the document:**
- The single highest-leverage technical deliverable (Part III, confirmed by two separate research agents) is explicitly named: **"a Revit add-in that converts the JSON token catalog into a Shared Parameters `.txt` file + IFC mapping table."** Quoted rationale: "This is the bridge between the open standard and the authoring-tool world practitioners actually inhabit. Without it, the token catalog is documentation; with it, it is tooling." This add-in is also listed as a Part IX action item (#6, "Phase 1 tooling decision") and a Part VIII "condition for success" (#5: "The Revit add-in ships with Phase 1... Without it, the token catalog is documentation.").
- The bSDD positioning mechanism (Part VII): "Every token in `pointsav-bim-system` that has an equivalent IFC Pset property should carry the bSDD URI as a canonical identifier in its `$extensions.bsdd` field." This is the concrete technical mechanism underpinning primitive #4 above.
- Platform bridge table (Part V) — the six bridge targets and their status, useful context for what "DTCG↔IFC mapping" is expected to interoperate with beyond IFC itself:

| Platform | DTCG token → platform format | Bridge status |
|---|---|---|
| Azure Digital Twins | DTCG → DTDL | DTDL is JSON-LD; one schema conversion |
| Autodesk Tandem | DTCG → classification templates | Compatible template model; no public bridge exists yet |
| Brick/RealEstateCore | DTCG → RDF/Turtle Brick instances | RealEstateCore is reference; bridge is a SPARQL transform |
| NVIDIA Omniverse (USD) | DTCG → USD attributes | NVIDIA encoding IFC in USD; DTCG → IFC PropertySet → USD attribute |
| ESRI ArcGIS Indoors | DTCG → geodatabase attributes | ArcGIS BIM File to Geodatabase tool is intermediate |
| IFC files | DTCG → IFC PropertySet | "The core bridge; IDS validation from the same token set" |

### C.3 What makes the PRODUCT (not the brand) feel like "Carbon for BIM" vs. just a docs site

This is the most directly relevant content to the redesign brief's positioning question. Key statements, extracted and organized:

**The Carbon analogy is explicitly corrected/limited (Part VIII):** "IBM does not monetize Carbon directly. Carbon is Apache 2.0, fully funded by IBM's design organization as overhead — not a profit center. There is no IBM Carbon support contract, no 'Carbon Enterprise' tier... Carbon's commercial value to IBM is: internal cost reduction at scale (47% faster to build with Carbon than from scratch per Sparkbox study), talent acquisition signal, and indirect alignment of IBM Cloud products." Direct implication stated: **"PointSav should not expect Apache 2.0 publication... to generate direct revenue. The Carbon analogy works only if PointSav has a portfolio of products benefiting from a common BIM token vocabulary (which it does)... But the revenue model is *not* IBM/Carbon."** Closer commercial analogs named: Confluent/Kafka, RealEstateCore, Brick Consortium.

**What makes it a genuine SaaS *product* rather than a docs site** (this is the operative distinction for the redesign — the workflow primitives in C.2 ARE the answer to this question, restated directly from the source text): *"`app-privategit-bim` is the BIM-vertical application layer. Market it as 'PointSav BIM CMS' or 'PointSav BIM Tokens Enterprise.' Workflow primitives that justify SaaS pricing... [the six listed in C.2]."* In other words, the document's explicit answer to "what makes the product feel like Carbon-for-BIM rather than just a docs site" is: **the presence of stateful, multi-party, validated workflow operations (review gates, IDS validation runs, IFC anchoring checks, provenance-audited overlay commits) layered on top of the static open token catalog** — a docs site can show you the tokens; the product lets you *operate* on them (review, validate, override, audit) at portfolio scale.

**The "spec is the funnel, running the spec at scale is the moat" framing (Part VIII, Layer 2):** *"Closed-source SaaS that runs `pointsav-bim-system` at portfolio scale: token lifecycle, IFC anchoring service, multi-stakeholder governance, WORM provenance, deployment to fleet. Pricing model: per-building per-month + per-anchored-commit + per-tenant-seat for tenant-facing applications... This is the Confluent play: spec is the funnel, running the spec at scale is the moat."*

**Architecture parallel that legitimizes this as more than branding (Part X, kept because it describes product architecture not brand):** *"The design token CMS market (Supernova, Tokens Studio Pro, Backlight, Specify) has converged on the same pattern: neutral open standard + branded CMS with different names. The successful ones (Supernova, Tokens Studio) are CMS companies that *defer to* neutral token formats (DTCG, Figma Variables, Style Dictionary) rather than publishing their own. The failed one (Specify) branded its own 'SDTF' format — format never achieved standard status, company shut down."* — i.e., the product-feel lesson is that the CMS layer must sit on top of and defer to the neutral open format, not invent its own proprietary vocabulary, or it fails the way Specify failed.

**Explicit statement that `os-privategit` (the WORM-provenance / F12-console / sovereignty infrastructure) is horizontal, not BIM-branded** — kept here because it is architecture, not branding: *"`os-privategit` is correctly positioned as a horizontal PointSav infrastructure product — managed private git with WORM provenance, F12 console, and sovereignty posture. It should not carry BIM branding; that preserves future `app-privategit-*` verticals (legal, healthcare, scientific)."* This confirms `app-privategit-bim` is the BIM-specific vertical application built on that horizontal substrate — the "product feel" comes from what that vertical adds (the six workflow primitives), not from the substrate itself.

### C.4 Score-card context (kept as it frames why this workflow-primitive layer matters, not branding)

Two operator questions and their scores (included briefly for context, since they frame *why* the workflow-primitive layer in C.2 matters commercially — not itself a branding claim):
- Q1 (Woodfine tenant token use case): **7/10** valid — value is landlord-side (asset intelligence/ESG data/fit-out governance), not a tenant fee model.
- Q2 (industry-wide BIM token gap PointSav should fill): **8/10** — gap is real; no DTCG-format BIM token library exists anywhere; EU implementing-act timing window closes ~2026–2027.

---

## Summary of what this means for the redesign brief

1. **Visual direction is settled and detailed:** BB.14 supersedes BB.13's Untitled-UI-derived palette/type choices with an Adobe-Spectrum-chrome + Source-Serif-4/Geist-Sans/Geist-Mono typography + `#1A4480` drafting-blue system, scoring **14/15** on the bankers' distinguishability test (Color 5 + Typography 4 + Chrome 5), against Vercel Geist's 13/15 as runner-up and Carbon's 3/15 baseline (disqualified as too similar to design.pointsav.com).
2. **The 5 AEC markers (GUID monospace, classification chips, IFC anchor labels on token cards, isometric building-mass hero SVG, dark viewport preview frame) are called structurally non-negotiable** and must be baked into templates, not left as optional styling.
3. **Nav/card-density assumptions are stale relative to current scale:** both documents were built assuming ~20 sidebar nav items and 8 token-category cards in a 2-column grid — neither was tested against the now-24-category catalog. This is a genuine open gap, not something either document addressed, and BB.14's own delta section confirms it made no changes to BB.13's nav-scale assumptions beyond a cosmetic width change (240px→272px).
4. **Implementation architecture is settled and lightweight:** SSR Rust/Axum stays; CSS-custom-properties expansion + a single ~60–80 line inline JS block (copy/tab/theme toggle) is the full technical lift; no bundler, no React, no Wasm migration.
5. **On the product-not-just-branding question:** the token-strategy document's answer is that genuine SaaS/"Carbon-for-BIM" product feel comes from the six workflow primitives (multi-stakeholder review, IDS export/validation, IFC anchoring validation, automated DTCG↔IFC mapping, WORM provenance audit trail, private token overlays) operating on top of the open catalog — not from visual design alone. A redesign that nails the Spectrum-chrome/AEC-marker visual language but never surfaces these operational primitives in the product would still read as "just a docs site," per the source document's own framing.
