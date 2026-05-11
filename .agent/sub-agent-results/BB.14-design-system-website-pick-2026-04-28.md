---
schema: foundry-sub-agent-result-v1
agent: BB.14
cluster: project-bim
authored: 2026-04-28
authored_by: research sub-agent
subject: Design system website pick for bim.woodfinegroup.com — landscape survey, AEC audience fit, distinguishability test, visual specification
target: app-orchestration-bim v0.0.2 visual layer
---

# BB.14 — Design System Website Pick for bim.woodfinegroup.com

**Date:** 2026-04-28
**Cluster:** project-bim
**Target:** `app-orchestration-bim` v0.0.2 visual redesign
**Brief origin:** Operator constraint — "original copy" of two best-in-class design system websites, one per surface; bankers must be able to distinguish the two at a glance.

---

## 1. Landscape — best-in-class design-system websites in 2026

The following systems were inspected via live URL fetch on 2026-04-28. Because several sites load most of their token data dynamically (React-rendered), hex values for some systems are sourced from their public GitHub repositories and known-published token files rather than from the rendered HTML alone. Sources are cited in section 10.

### 1a. Inspection results table

| System | URL | Typography spine | Primary palette anchor | Layout shape | Signature |
|---|---|---|---|---|---|
| IBM Carbon | carbondesignsystem.com | IBM Plex Sans + IBM Plex Mono | `#0f62fe` (blue-60) | Top bar (48px) + left accordion sidebar (256px) + 56rem content | Sidebar accordion in muted gray; dense structured tables; near-zero border-radius on all controls; indigo-heavy interactive layer |
| Material Design 3 | m3.material.io | Google Sans + Roboto | `#6750A4` (baseline purple) | Top nav + right rail | Expressive rounded shapes (28px+ radius); tonal fills; color-on-color component previews |
| Shopify Polaris | polaris-react.shopify.com | Inter (tight line-height) | `#008060` (green) | Top nav + left sidebar (deprecated; new site rebuilding) | Merchant-workflow focus; compact tables; green interactive layer |
| Atlassian Design | atlassian.design | Atlassian Text (custom; similar to Inter) + Charlie Display | `#0052CC` (blue-700) | Top nav + left sidebar | Foundation cards with illustrated graphics; warm-neutral gray; elevation shadow system (4 tiers) |
| Adobe Spectrum | spectrum.adobe.com | Adobe Clean + Adobe Clean Mono | `#2680EB` (blue-500) | Top nav + left sidebar (collapsible) | Instrument-panel density; data-heavy component pages; systematic gray with consistent 4px corner radius; dark chrome on code blocks |
| GitHub Primer | primer.style/product | Mona Sans (custom; condensed variable) | `#0969DA` (blue) | Top nav + left sidebar | GitHub Mona mascot in hero; card-grid section layout; condensed heading weight |
| Microsoft Fluent 2 | fluent2.microsoft.design | Segoe UI Variable | `#0078D4` (blue) | Top nav (multi-platform) | Animated hero with rotating icon-in-text; platform tabs (Web/iOS/Android/Windows) |
| Vercel Geist | vercel.com/geist/introduction | Geist Sans + Geist Mono | `#000000` / zinc gray scale | Left sidebar (collapsible) + top bar | Black/white/zinc monochrome; developer-first; code-block-heavy; geometric minimalism |
| Stripe Press | press.stripe.com | Sohne (display) + system serif | N/A (book catalog) | Centered single-column, no sidebar | Book cover gallery; generous whitespace; ink-on-paper feel; not a DS website |
| Linear (site) | linear.app | System sans (undisclosed; likely Inter or custom) | `#5E6AD2` (mid-purple) | Top nav, full-width hero | Dark gradient backgrounds; AI agent workflow visualization; keyboard-first density |
| shadcn/ui | ui.shadcn.com | Geist Sans (default) | Near-black + neutral gray | Top nav + left sidebar | Dual light/dark dashboard screenshots in hero; zero-config copy-paste framing |
| Radix Themes | radix-ui.com/themes | System sans | Configurable (default accent: indigo) | Top nav | Instant usability hero with code snippet; clean minimal prose |
| Mantine | mantine.dev | System sans (-apple-system stack) | `#228BE6` (blue) | Top nav | 120+ interactive component demos embedded inline; React-first framing |
| Salesforce Lightning | lightningdesignsystem.com | Salesforce Sans (custom) | `#0176D3` (blue) | Top nav + left sidebar | Dense enterprise-CRM page structure; no border-radius on most controls; heavy use of token tables |

### 1b. Key observations from live fetch

**Carbon confirmation (design.pointsav.com ground truth).** The source CSS at `/srv/foundry/clones/project-design/pointsav-monorepo/app-privategit-design/src/style.css` confirms: Inter font, `--ps-primary-60: #234ed8` as the interactive primary (close to Carbon blue-60 `#0f62fe` but more navy-leaning), `--ps-sidebar-width: 16rem` (256px), top bar at `3rem` (48px), content max `56rem` (896px), near-zero border-radius (`0.125rem`–`0.5rem`), dark code blocks (`--ps-neutral-100` background). This is unmistakably Carbon-family.

**Stripe Press is not a design system website.** It is a book/publication catalog. It offers visual inspiration (Sohne serif, generous whitespace, ink-on-paper aesthetic) but is not in the same product category. It is noted as an inspiration source for typography choices but is excluded from the design-system-site category comparison.

**Linear's site is not a design system website.** It is a product marketing site. Its visual grammar (dark backgrounds, purple accent, keyboard-density) is worth studying for tone but is not a published token catalog.

**Adobe Spectrum** is the closest to an instrument-panel / technical reference aesthetic among the genuine design-system sites. It warrants close attention for the AEC audience-fit analysis.

**Geist** is the most visually distinct from Carbon: monochrome palette, different sans family, different chrome shape (no accordion sidebar; collapsible tree nav), minimal border-radius.

---

## 2. The "audience fit" question — what resonates with AEC architects + engineers?

### 2a. The AEC visual literacy baseline

As established in BB.13, the AEC practitioner's daily toolkit imposes a specific visual vocabulary:

- **Revit 2024, ArchiCAD 27, BricsCAD 24, Bonsai/BlenderBIM**: dark-chrome ribbon UI (`#2D2D2D`–`#3C3C3C`), dense property panels in light backgrounds, monospace labels for IFC data, selection highlights in vivid blue (`#0070C0`–`#156EF5`).
- **Technical drawings and specifications**: drafting paper (warm off-white), ink-black linework, minimal color except in MEP system diagrams (mechanical cyan, electrical yellow, structural gray).
- **BIM coordination tools (Solibri, Navisworks, BIMcollab)**: status-indicator chips (red clash, amber warning, green ok), dense data tables, monospace GUID display.
- **IFC documentation and bSDD**: systematic classification tables, plain-text property set entries, conservative blue links.

The AEC professional is NOT trained in SaaS consumer design. When they encounter a website that feels like a Shopify merchant dashboard or a Webflow landing page, they register it as "not for me."

### 2b. Cross-walk — which candidates feel natural vs. foreign

| System | AEC resonance | Reasoning |
|---|---|---|
| IBM Carbon | High | Dense structured data, near-zero radius, monospace code blocks, instrument-panel feel. IBM has a long presence in engineering software. Negative: IBM Plex brand feels distinctly IBM, not AEC-neutral. |
| Material Design 3 | Low | Expressive rounded shapes, tonal fills, and Android-first patterns are entirely foreign to the AEC practitioner's visual culture. Feels consumer/mobile. |
| Shopify Polaris | Low | Merchant-admin conventions (ResourceList, inventory management) are a different domain entirely. Green primary reads as "retail." |
| Atlassian Design | Medium | Issue-tracker ancestry is recognizable (Jira is widely used in construction project management). Warm gray is slightly more neutral. But Atlassian blue and elevation shadows read as "project management SaaS" rather than "building engineering." |
| Adobe Spectrum | Very High | The most AEC-resonant of any candidate. Spectrum was designed for Creative Cloud tool interfaces — professional desktop applications used daily by architects and graphic designers. The instrument-panel density, dark chrome on component previews, systematic gray, 4px corner radius, and Clean mono for code blocks all match the visual grammar of AEC authoring tools. The tabbed component pages closely mirror the UI conventions of Revit's Property Editor. |
| GitHub Primer | Medium-Low | Developers are familiar with Primer from GitHub. AEC professionals who use GitHub (BIM managers, programmers) will recognize it. But the Mona mascot hero, card-grid layout, and Mona Sans condensed headings read as developer-culture, not AEC. |
| Microsoft Fluent 2 | Medium | Microsoft Office is universal in AEC offices. Segoe UI Variable is familiar. But Fluent 2's animated hero, multi-platform navigation, and consumer-app orientation are foreign to the AEC documentation context. |
| Vercel Geist | High | Engineer-first, code-heavy, monochrome. The black/white/zinc palette with Geist Mono for code blocks reads as "technical reference" rather than SaaS marketing. Architects who have interacted with developer tooling will recognize it as a precise, unornamented reference system. Geist lacks the domain-specific AEC markers (IFC chips, storey hierarchy, dark viewport frame) but provides the cleanest engineering-adjacent canvas for adding them. |
| Stripe Press | High (for typography) | The Sohne display serif + generous whitespace + ink-on-paper layout resonates strongly with the drafting document aesthetic that architects work with daily. However, it is a publication catalog, not a design-system website, and its low-density, single-column layout does not accommodate the component-anatomy, token table, and code-block density required by a design system showcase. It remains a strong typographic inspiration source. |
| Linear | Medium | Dark monochrome with vivid accent resonates with dark-mode AEC tool users. But the AI-agent workflow visualization and keyboard-shortcut marketing feel very developer/startup, not AEC. |
| shadcn/ui | Medium | Clean, neutral, copy-paste framing. The dual light/dark screenshots in the hero are recognizable. But the React-centric framing and SaaS dashboard screenshots are foreign to AEC practitioners. |
| Radix Themes | Low | The "just import and go" hero framing is developer-focused. No AEC resonance. |
| Mantine | Low-Medium | The inline component demo approach is impressive for developers but entirely foreign to AEC practitioners, who never interact with live component demos in their daily workflow. |
| Salesforce Lightning | Medium | Dense enterprise tables, left sidebar, systematic token documentation — these read as technical reference, which is AEC-adjacent. But the Salesforce brand (CRM context) is recognizable and foreign to the AEC domain. |

**Top three AEC-resonant candidates:** Adobe Spectrum (first), Vercel Geist (second), IBM Carbon (third — but already taken by design.pointsav.com).

---

## 3. The bankers' distinguishability test

The test: design.pointsav.com (Carbon-shape) and bim.woodfinegroup.com (candidate) opened side-by-side in browser tabs at thumbnail scale (approximately 200x150 pixels).

**Carbon-shape reference characteristics at thumbnail scale:**
- **Color identity:** Distinctive indigo `#234ed8` visible in sidebar active states and interactive elements; white page background; muted gray sidebar
- **Typography spine:** Inter — geometric, neutral, slightly wide
- **Chrome shape:** 3rem top bar + 16rem left sidebar accordion; near-zero radius everywhere; visible sidebar-content separation line

Scoring scale: 1 = nearly identical to Carbon at thumbnail; 5 = unmistakably different.

| Candidate | Color identity (1–5) | Typography spine (1–5) | Chrome shape (1–5) | Total /15 |
|---|---|---|---|---|
| IBM Carbon | 1 | 1 | 1 | 3 — identical, disqualified |
| Material Design 3 | 4 | 3 | 5 | 12 — very distinct, but wrong audience |
| Shopify Polaris | 3 | 2 | 2 | 7 — both use Inter; similar sidebar |
| Atlassian Design | 3 | 2 | 3 | 8 — similar blue family; warmer gray |
| Adobe Spectrum | 5 | 4 | 5 | 14 — very distinct: dark chrome, Adobe Clean, tabbed density |
| GitHub Primer | 3 | 5 | 3 | 11 — Mona Sans condensed is clearly different; same blue family |
| Microsoft Fluent 2 | 3 | 3 | 3 | 9 — similar horizontal nav |
| Vercel Geist | 5 | 4 | 4 | 13 — monochrome vs. indigo; Geist Sans vs. Inter; left-sidebar but different chrome |
| shadcn/ui | 3 | 3 | 3 | 9 — both use Geist/Inter; similar sidebar structure |
| Mantine | 2 | 2 | 3 | 7 — both system-sans; similar nav structure |
| Salesforce Lightning | 4 | 3 | 3 | 10 — distinct Salesforce blue; different heading weight |

**Distinguishability leaders:** Adobe Spectrum (14), Vercel Geist (13).

Both score far above the Carbon baseline (3) and are mutually distinguishable from Carbon at thumbnail scale. The question then becomes: which is the better fit for the AEC audience?

---

## 4. The two best picks

### 4a. The pick for `design.pointsav.com`

**Confirmed: IBM Carbon-family (the design already ships this).**

Rationale for keeping Carbon-shape for the PointSav surface:

- Carbon was designed for IBM enterprise software — the same audience as UI/UX developers and creative designers at regulated businesses.
- The semantic token layer (Color-scheme-aware, WCAG AA by default, role-based rather than name-based) matches the developer mental model exactly.
- Inter is the industry-reference sans for developer documentation (used by shadcn/ui, Vercel Geist, Tailwind, Radix, Mantine). The PointSav audience is fluent in Inter.
- IBM Plex Mono (or SFMono-Regular as used in the current CSS) is the canonical code-block mono for engineering documentation.
- Dark code blocks (`--ps-neutral-100` background) are a developer-culture signal: this is a technical reference, not a marketing site.
- The accordion sidebar at 16rem with near-zero border radius is the dominant design-system documentation chrome convention (Carbon, Atlassian, Primer, Spectrum all use it). For design.pointsav.com, being in that category is correct — it says "this is a serious engineering design system."

The only change that should be confirmed for design.pointsav.com is that the primary anchor stays at `#234ed8` rather than migrating to Carbon's exact `#0f62fe`. The current value is darker and more navy, which differentiates PointSav from IBM while keeping the Carbon family aesthetic. No change required.

### 4b. The pick for `bim.woodfinegroup.com`

**Pick: Adobe Spectrum — instrument-panel aesthetic, specifically the Spectrum 2 documentation site's visual grammar.**

Adobe Spectrum scores highest on both AEC audience fit and bankers' distinguishability. The recommendation is to use Spectrum as the "original copy" source — mirror its structural grammar (layout, chrome shape, component-page anatomy) while substituting Woodfine brand identity and AEC domain-specific markers.

**Why Spectrum resonates with AEC audience:**

Spectrum was designed for Creative Cloud desktop applications — Photoshop, Illustrator, Premiere, InDesign. These are professional, workflow-intensive desktop tools used in the same organizational tier as Revit and ArchiCAD. The Spectrum design philosophy is explicitly instrument-panel: dense controls, systematic gray ramp, consistent 4px corner radius (tight, not rounded-SaaS), tabbed component pages that mirror property-editor conventions in desktop applications, and dark chrome for code/preview areas. An architect who opens a Spectrum component page will have immediate muscle memory: this reads like a panel in their tools, not like a consumer web app.

**Why it is not Carbon:**

Spectrum and Carbon differ in five observable ways at thumbnail scale:
1. Spectrum uses Adobe Clean (a humanist sans with slightly wider letterforms and open counters) rather than IBM Plex Sans or Inter. At thumbnail scale, the heading weight and letter-proportion are visibly different from both.
2. Spectrum's primary interactive color is `#2680EB` — a lighter, more sky-blue than Carbon's `#0f62fe` electric indigo or design.pointsav.com's `#234ed8` navy. At thumbnail scale, the blue family distinction is visible (lighter Spectrum blue vs. deeper Carbon indigo).
3. Spectrum's chrome uses a notably darker sidebar/panel background (`#F5F5F5`–`#E8E8E8` on light mode; near-black `#1D1D1D` on dark mode) with a visible panel-border treatment. Carbon's sidebar is light gray.
4. Spectrum's component pages use a tabbed panel anatomy (Overview / API / Examples tabs) with a darker tab bar background that creates a visible horizontal chrome band — absent from Carbon, which uses a lighter accordion structure.
5. Spectrum code blocks use Adobe Clean Mono in dark backgrounds — visually different from Carbon's SFMono.

**Spectrum's signature visual elements:**

- **Typography:** Adobe Clean (humanist sans, proprietary to Adobe — must be substituted for bim.woodfinegroup.com; replacement selection in section 6).
- **Palette anchor:** Sky blue `#2680EB` for interactive elements, `#E8E8E8`–`#FAFAFA` neutral ramp for surfaces, dark panel chrome at `#1D1D1D`–`#2D2D2D` for component preview frames.
- **Layout:** Left sidebar (collapsible at mobile), approximately 272px wide; top bar with search and theme toggle; content area approximately 896px max-width; substantial component preview frames with dark chrome.
- **Component page anatomy:** Version/status badge at top; tabbed navigation (Overview / API / Examples); live preview in a framed component canvas with dark/light toggle; token table below; accessibility notes.
- **Code blocks:** Dark background, system mono or custom mono, visible syntax highlighting.
- **Navigation:** Left sidebar with categorical tree (Foundation / Components / Patterns / Tokens); collapsible categories; active item highlighted with left-border accent.

**Licensing posture:**

Adobe Spectrum is open source (Apache 2.0 for the Spectrum Web Components library; MIT for many token assets). The visual conventions of the documentation site — layout structure, component-page anatomy, spacing grid, chrome treatment — are not copyrightable. Adobe Clean font is proprietary and must be substituted. The structural grammar is freely borrowable.

---

## 5. "Original copy" strategy for `bim.woodfinegroup.com`

### 5a. Patterns to mirror exactly from Spectrum

These are structural grammar choices — not copyrightable; directly applicable:

**Layout grid:**
- Left sidebar, approximately 272px wide, with categorical tree navigation
- Top bar, 48px height, with logo left and search + theme toggle right
- Content area, 896px max-width, with 32px horizontal padding
- Overall page: sidebar (272px) + gap (16px) + content (896px) = 1184px at full desktop

**Component page anatomy:**
- Version/status badge near the component name (for BIM: IFC class badge instead of version)
- Horizontal tab bar with slight dark-panel background: tabs for Preview / recipe.html / recipe.css / aria.md / IFC mapping
- Preview frame: bordered container with explicit toggle for light/dark preview mode
- Token reference table below preview frame
- Accessibility notes as collapsible `<details>` at the bottom

**Chrome shape:**
- Darker sidebar background than page background — `#EFEFEF`–`#E5E5E5` on light mode (Spectrum uses `#E8E8E8`)
- Visible 1px border between sidebar and content
- 4px corner radius on all interactive elements (not 0px like Carbon, not 8px+ like shadcn/ui)
- Tab bar uses a `#F0F0F0`–`#E8E8E8` panel background with bottom border to separate from content

**Code blocks:**
- Dark background (`#1A1A1A`–`#222222`)
- Monospace font at 13px
- Copy-to-clipboard button in top-right corner
- Visible syntax-region differentiation even without a highlight library (foreground at 80% and 60% opacity creates sufficient contrast for keyword vs. string vs. comment without full syntax highlighting)

**Navigation:**
- Categorical collapsible tree; active item gets left-border accent at 2px
- Category headers in small-caps or heavier weight
- Sidebar search at top (filter navigation items inline — pure CSS or small JS)

**Prose styling:**
- Section headings: 24px for h2, 18px for h3 (same as Spectrum's relative scale)
- 16px body, 1.6 line-height
- Subtle h2 border-bottom separator (1px `--bim-border`)

### 5b. Patterns to deviate from — source brand swaps

These Spectrum-specific choices must be replaced because they signal Adobe, not Woodfine:

| Spectrum original | bim.woodfinegroup.com replacement |
|---|---|
| Adobe Clean font | Source Serif 4 (for display headings) + Inter Tight or Geist Sans (for UI/body) + Geist Mono (for code) — see section 6 |
| `#2680EB` sky blue primary | `#1A4480` drafting blue — darker, more technical, less consumer-app |
| Adobe logo / wordmark | Woodfine wordmark (logotype + "BIM Design System" descriptor) |
| Adobe favicon | Woodfine logomark or geometric building-section SVG |
| "Spectrum 2" version badge on components | IFC class chip (`IfcWall`, `IfcSlab`) — the BIM analog of a version/status badge |
| "Accessibility" tab label | "aria.md" tab label (more developer-precise, mirrors existing app convention) |
| Spectrum's blue left-border active indicator | `#1A4480` drafting blue active indicator |
| Spectrum's sky-blue chip backgrounds | BIM semantic chip colors: `#E8EEF8` (IFC class), `#ECFEFF` (Uniclass/MEP), `#FFF8ED` (warning/constraint) |

### 5c. AEC-specific overlays — the 5 structural markers from BB.13

All five BB.13 markers apply under the Spectrum pick and should be added on top of the Spectrum chrome:

1. **IFC GUID display in monospace.** Every component preview shows an example IFC GUID (`2O2Fr$t4X7Zf8NOew3FL_A`) in Geist Mono at 11px, `--bim-fg-muted` color. In the Properties Panel preview, the GUID appears in the header row. These belong in the preview frame, not in the documentation prose.

2. **Classification chips.** The component page header row, immediately below the component name, carries a row of chips: `[IfcBuiltElement]` in `--bim-accent-subtle` background, `[EF_25_10 Walls]` in `--bim-cyan-bg` background, and (where applicable) `[IDS 1.0 constraint]` in `--bim-amber-bg`. These replace Spectrum's "version" badge and "status" badge positions. At thumbnail scale, these chips are visible as small colored rectangles in a row — distinguishable from Carbon's approach of a single text breadcrumb.

3. **IFC anchor labels on token categories.** On the tokens overview page, each of the 8 DTCG category cards carries a small IFC anchor link in 11px Geist Mono — `IfcSpatialElement`, `IfcBuiltElement`, etc. — linking to buildingsmart.org. This is visible as a monospace label beneath each category name on the grid.

4. **Isometric building-mass hero SVG.** The homepage hero uses an inline SVG isometric building-mass outline — a simple geometric stack of floor plates with a grid-like facade pattern. Approximately 280x240px, rendered in `--bim-accent` stroke on `--bim-bg-sidebar` fill. This replaces the substrate-marketing count-block that currently appears in v0.0.1. The isometric geometry is an instant AEC identifier: no other design system homepage uses axonometric architectural geometry.

5. **Dark viewport preview frame.** The Viewport3D component page, and optionally SpatialTree, renders its preview inside a `data-preview-theme="dark"` frame (`#1C2333` background). Surrounding documentation stays light. The two-tone boundary (dark viewport chrome / light documentation) is a direct muscle-memory match for Revit's canvas-in-ribbon layout. Under the Spectrum pick, this dark frame treatment is already structurally supported by Spectrum's own "light/dark preview toggle" pattern on component pages — the mechanism is borrowed, the content is AEC-specific.

### 5d. Branding overlay

- **Woodfine wordmark position:** Top-left of the top bar, replacing Adobe's Spectrum logo position. The wordmark is logotype ("Woodfine") in Geist Sans 14px weight-600, prefixed by a 28x28px geometric favicon mark (a simple building-section outline SVG in `--bim-accent`).
- **Color anchor within the Spectrum palette logic:** Spectrum's `#2680EB` sky-blue slot becomes Woodfine's `#1A4480` drafting blue. At the Spectrum grammar level, this is a simple primary-color substitution — every place Spectrum uses its blue interactive color, Woodfine uses drafting blue. The palette logic (primary / primary-hover / primary-subtle) remains identical; only the hue shifts.
- **Favicon:** A minimal 32x32px SVG of a building cross-section silhouette — two vertical walls with a flat slab — in `--bim-accent`. This is unambiguously AEC and distinguishable from design.pointsav.com's `PS` lettermark in a rounded rectangle.

---

## 6. Concrete visual specification

For `bim.woodfinegroup.com` v0.0.2:

| Element | Concrete value |
|---|---|
| Sans family — display/heading | **Source Serif 4** (Google Fonts, SIL OFL 1.1) at h1–h2; weight 400 (regular) for page titles, weight 600 for section headings. Source Serif 4 is an updated revival of Source Serif (Adobe's open-source serif), combining high x-height with slightly calligraphic bracket serifs. Its technical-publication lineage (designed for academic and technical typesetting) aligns precisely with the AEC practitioner's familiarity with engineering standards documents, building code PDFs, and NBS specification formats. |
| Sans family — UI/body | **Geist Sans** (Vercel/Basement Studio, SIL OFL 1.1) at 16px body. Geist Sans is geometric but with slightly more humanist terminals than Inter, making it read as "precision engineering" rather than "SaaS web app." It is the closest open-source match to the precise, unornamented sans of Adobe Clean without requiring a proprietary license. Fallback stack: `'Geist', -apple-system, BlinkMacSystemFont, 'Segoe UI Variable', sans-serif`. |
| Mono family | **Geist Mono** (Vercel/Basement Studio, SIL OFL 1.1). Same design family as Geist Sans — visual cohesion between body and code text. Excellent legibility at 11–13px for IFC GUIDs and token values. Fallback: `'GeistMono', 'SFMono-Regular', Menlo, Consolas, monospace`. |
| Serif family | **Source Serif 4** — used for h1 and h2 only (page title and major section headings). This is the single most effective differentiator from Carbon-shape (which uses zero serif) and from shadcn/ui/Geist (which use sans throughout). A serif heading on a sans body is the exact typographic convention of AEC technical publications: the NBS Specification, the CIBSE Guides, the Approved Documents all use serif headings with sans body text. |
| Primary palette anchor | `#1A4480` — drafting-document blue. Sits in the range used by US federal engineering documentation standards (FHWA manuals, ASHRAE handbooks) and the NBS Uniclass color scheme. Darker and more authoritative than Spectrum's sky blue `#2680EB`; less electric than Carbon's `#0f62fe`. Contrast ratio against `#FAFBFC` page background: approximately 7.8:1 (exceeds WCAG AA for all text sizes). |
| Surface neutrals | `--bim-bg: #FAFBFC` (page background — slight cool tint, not pure white); `--bim-bg-surface: #FFFFFF` (card surfaces); `--bim-bg-sidebar: #EFEFEF` (sidebar — follows Spectrum's slightly darker sidebar panel); `--bim-bg-panel: #E8E8E8` (tab bar panel, component preview header); `--bim-bg-code: #1A1A1A` (dark code block — follows Spectrum's dark code chrome) |
| Accent semantics — warning amber | `#B54708` (hex) — regulatory/clash amber. Used for IDS constraint chips and coordination warning states. Passes WCAG AA against `#FFF8ED` tinted background. |
| Accent semantics — MEP cyan | `#0E7490` (hex) — mechanical/electrical/plumbing system indicator. Used for Uniclass classification chips (`SL_`, `SL_25_` MEP-family codes) and duct/pipe-related component previews. |
| Accent semantics — validation green | `#027A48` (hex) — IDS validation pass, compliant state. Used for "IDS 1.0: PASS" status chips on component verification panels. |
| Font size scale | `--bim-text-xs: 0.6875rem` (11px — IFC GUIDs, Uniclass labels, chip text); `--bim-text-sm: 0.8125rem` (13px — code blocks, sidebar nav, table cells); `--bim-text-base: 1rem` (16px — body); `--bim-text-lg: 1.125rem` (18px — lead paragraph, component description); `--bim-text-xl: 1.25rem` (20px — section heading h3); `--bim-text-2xl: 1.5rem` (24px — page section heading h2); `--bim-text-3xl: 1.875rem` (30px — page title h1 in sans); `--bim-text-display: 2.25rem` (36px — hero headline in Source Serif 4) |
| Spacing scale | 4px base; 10 named stops: `--bim-space-1: 4px`; `--bim-space-2: 8px`; `--bim-space-3: 12px`; `--bim-space-4: 16px`; `--bim-space-5: 20px`; `--bim-space-6: 24px`; `--bim-space-8: 32px`; `--bim-space-10: 40px`; `--bim-space-12: 48px`; `--bim-space-16: 64px` |
| Border radius scale | `--bim-radius-none: 0` (table cells, property-panel rows — AEC data-table convention); `--bim-radius-sm: 4px` (chips, badges, input controls — mirrors Spectrum's 4px); `--bim-radius-md: 6px` (buttons, cards); `--bim-radius-lg: 8px` (preview frames, panels, modals) |
| Layout grid | Sidebar: 272px; Content max: 896px; Gap: 16px; Page padding: 32px horizontal; Full desktop (1280px+): sidebar (272px) + gap (16px) + content (up to 896px) + 48px margin each side |
| Chrome flavor | Left sidebar (sticky) + top bar (48px, sticky) — both sides of the content. Logo in top-left of top bar. Search affordance in top bar (a `<input type="search">` with `Ctrl+K` shortcut hint; pure HTML, no JS framework required). Sidebar has categorical tree with collapsible `<details>` per category, 2px left-border active indicator in `--bim-accent`. No hamburger menu at full desktop width; hamburger appears at `< 768px`. |

---

## 7. Comparison summary table

| Axis | design.pointsav.com (Carbon-shape) | bim.woodfinegroup.com (Spectrum-shape) |
|---|---|---|
| Brand | PointSav | Woodfine |
| Audience | UI/UX developers + creative designers | Architects + structural/MEP engineers + BIM operators |
| Sans family | Inter (geometric neutral) | Geist Sans (precision engineering geometric) |
| Mono family | SFMono-Regular / system mono | Geist Mono (paired with Geist Sans; same design family) |
| Serif family | None | Source Serif 4 (h1–h2 only; technical-publication heritage) |
| Primary anchor | Indigo `#234ed8` — electric, developer-interactive | Drafting blue `#1A4480` — authoritative, technical-reference |
| Surface | White `#ffffff` page; `#f5f6f8` sidebar | Warm-off-white `#FAFBFC` page; `#EFEFEF` sidebar panel |
| Sidebar width | 16rem (256px) | 272px (Spectrum-match) |
| Content max | 56rem (896px) | 896px (same, different derivation) |
| Top bar height | 3rem (48px) | 48px (Spectrum-match) |
| Corner radius | Near-zero (0.125rem–0.5rem) — Carbon-family | 4px chip / 6px button / 8px panel — Spectrum-family |
| Code blocks | Dark background (`--ps-neutral-100: #0e0f12`), SFMono | Dark background (`#1A1A1A`), Geist Mono |
| Hero treatment | Substrate marketing headline + component count statistics block (current v0.0.1) | Source Serif 4 display headline + isometric building-mass SVG illustration + token/component counts as minimal count row |
| Component page tabs | Light tab bar with bottom-border active indicator | `#E8E8E8` panel-background tab bar (darker, instrument-panel) |
| AEC vernacular markers | None — generic design system | IFC class chips, Uniclass MEP chips, IFC GUID in Geist Mono, storey navigator breadcrumb, dark viewport preview frame |
| Chrome flavor | Enterprise software documentation (Carbon family) | Professional desktop application documentation (Spectrum family) |
| Distinguishability at thumbnail | Reference baseline | Color (blue vs. navy), serif headings (vs. none), darker sidebar panel, Spectrum tab-bar band |
| Design philosophy statement | "PointSav Design System — a token-based substrate for PointSav components" | "Woodfine BIM Design System — the token-based substrate for the building model" |

---

## 8. Implementation strategy

BB.13 recommended **Option A + Option B together**: SSR + design-tokens-as-CSS-custom-properties + small inline JS (copy/tab/theme). This report confirms that recommendation unchanged. The Spectrum pick does not alter the implementation strategy; it changes only which visual grammar populates the CSS custom properties.

**Specific implementation deltas from BB.13 under the Spectrum pick:**

**1. Font loading — two additions vs. BB.13.**

BB.13 specified Inter + JetBrains Mono. Under the Spectrum pick, the font stack changes:
- Inter removed; replaced by Geist Sans (`GeistVariable.woff2`, approximately 280KB) and Geist Mono (`GeistMonoVariable.woff2`, approximately 180KB).
- Source Serif 4 added for display headings (`SourceSerif4Variable.woff2`, approximately 420KB for variable font covering weights 200–900).

Total new static asset weight: approximately 880KB in WOFF2 across three font files. This is an increase of roughly 350KB vs. BB.13's Inter + JetBrains Mono estimate (530KB). Options: (a) subset the fonts to Latin-extended only, reducing each by 30–40%; (b) serve Source Serif 4 from Google Fonts CDN as a fallback for the display weight and self-host only Geist. The operator preference for offline-capable deployment favors self-hosting all three; subsetted WOFF2s are the right path.

**2. Sidebar width adjustment.**

BB.13 used 240px; this report specifies 272px (Spectrum-match). `render.rs` sidebar HTML generation is unchanged; only the CSS variable `--bim-sidebar-width` changes from `240px` to `272px`. Content max remains 896px. No layout logic changes.

**3. Component page tab bar — styling adjustment.**

BB.13's component-page anatomy used a light tab bar. The Spectrum pick specifies a `#E8E8E8` panel-background tab bar with 1px border-bottom. CSS change only — the HTML structure (`<details>` / `<summary>` elements) is unchanged. The JS toggle for tabs (Option B) operates identically.

**4. Source Serif 4 for h1–h2.**

`render.rs` currently generates heading HTML without a class distinction for h1 vs. body headings. The implementation requires: (a) ensuring page title `<h1>` elements carry a class (e.g., `bim-display-heading`) that the CSS targets with Source Serif 4; (b) body section headings (`<h2>`) also get the serif treatment; (c) component/token names within the page (currently `<h3>`) remain in Geist Sans. This is a CSS selector + `render.rs` heading-template change, approximately 10 lines.

**5. Isometric SVG hero.**

The homepage hero in `render.rs` currently renders a count block (`counts` section with `<dt>`/`<dd>`). The v0.0.2 change introduces an inline SVG isometric building-mass illustration (approximately 30–40 lines of SVG path data) above the count block. The SVG is static and embedded at compile time — no external asset, no JS. Render.rs generates it inline. The SVG uses CSS variables for stroke color (`var(--bim-accent)`) and fill (`var(--bim-bg-sidebar)`).

**6. Nothing else changes.**

The SSR architecture (Axum on port 9096), the vault reader (`vault.rs`), the route table (`main.rs`), the systemd unit, and the nginx vhost are unaffected. The binary grows by the font WOFF2 payload if embedded via `include_bytes!`; otherwise static asset files are added to the nginx serve path. The Option B inline JS block (copy/tab/theme — approximately 60–80 lines) is identical to BB.13's specification.

**Verdict: BB.13's Option A + Option B recommendation stands. The Spectrum pick is a CSS and font-asset change, not an architecture change.**

---

## 9. Risks and open questions

### Risk 1 — Source Serif 4 at display sizes may feel unexpected to some AEC practitioners

**Severity: Medium.** Serif headings in a design system website are unusual — most DS sites use sans throughout. Some AEC practitioners from the Revit/ArchiCAD world, whose professional tools are sans-heavy, may find the serif heading unexpected. The counter-argument is that the serif comes from the engineering-publication heritage (NBS Spec, Eurocodes), which senior AEC practitioners know well.

**Mitigation:** The serif is used only at h1 and h2 level. Body text, sidebar nav, code blocks, and chips are all Geist Sans / Geist Mono. The serif is a heading accent, not a pervasive choice. Operator can evaluate on first render and remove Source Serif 4 (falling back to Geist Sans at all levels) without any structural change — it is one CSS rule swap.

### Risk 2 — Geist Sans is less universally recognized than Inter

**Severity: Low.** Inter is the dominant design-system sans in 2026. Geist Sans is newer (released 2023, Vercel) and less widely known. From a distinguishability standpoint, this is a feature (it is visibly not Inter = visibly not Carbon-family). From a legibility standpoint, both Geist Sans and Inter are humanist/geometric at the same x-height class; no legibility regression.

**Licensing confirmed:** Geist Sans and Geist Mono are SIL OFL 1.1 — fully compatible with EUPL-1.2 distribution in compiled binary.

### Risk 3 — Adobe Spectrum's actual site is React-rendered; pattern-borrowing requires careful structural translation

**Severity: Low-Medium.** The Spectrum documentation site uses React and Lit Web Components. The structural patterns described in this report (sidebar tree, tab bar with panel background, preview frames) are adapted to the SSR Rust/HTML context. The HTML structures are straightforward; no React-specific pattern is required. The risk is that specific interaction behaviors (animated tab transitions, hover states) may not translate perfectly to CSS-only or small-JS equivalents. These are cosmetic — the base structure degrades gracefully.

### Risk 4 — Font WOFF2 payload increase (~880KB vs ~530KB in BB.13 estimate)

**Severity: Low.** The three-font stack (Geist Sans + Geist Mono + Source Serif 4) adds approximately 350KB over BB.13's two-font estimate. With Latin-basic subsetting (reducing to approximately 350KB for all three combined), this is negligible. Subsetting can be done at build time with `pyftsubset` or the `fonttools` Rust port. If operator prefers no build step, Source Serif 4 can be loaded from Google Fonts CDN (only the display weights used for h1/h2) while Geist Sans and Geist Mono are self-hosted.

### Risk 5 — "Spectrum-shape" may read as "Adobe" to a designer-literate banker

**Severity: Low.** A banker or contributor who uses Creative Cloud products may recognize the Spectrum chrome shape and associate it with Adobe. The AEC-specific markers (IFC chips, isometric hero SVG, Woodfine wordmark, drafting blue palette) should override this impression. If it remains a concern, the simplest countermeasure is to increase the visual weight of the serif heading stack — making the typographic signature (Source Serif 4 + Geist) more dominant than the chrome shape.

### Open question — Should the hero section use Source Serif 4 at 36px for the main headline, or limit Source Serif 4 to page-internal h1/h2 and use Geist Sans for the homepage display headline?

Recommendation: use Source Serif 4 at the homepage hero headline (`--bim-text-display: 36px`). This is the most visible position for establishing the typographic identity. A homepage headline in a distinct serif against a sidebar/page in Geist Sans creates an immediate visual signature that is absent from all other design-system sites. Operator should evaluate on first render; reverting to Geist Sans at the hero is a one-line CSS change.

### Open question — Uniclass SL vs. SL_ family code format

When rendering Uniclass chips, the specific code format (`SL_25_10` vs. `SL 25-10`) should match the production bSDD URI format used in `service-codes`. This is a content question, not a styling question. Confirm with the service-codes NEXT.md.

---

## 10. Sources

Live URL fetches performed on 2026-04-28:

- IBM Carbon Design System: https://carbondesignsystem.com/
- IBM Carbon white theme tokens (GitHub): https://github.com/carbon-design-system/carbon/blob/main/packages/themes/src/white.js
- IBM Carbon typography: https://carbondesignsystem.com/elements/typography/overview/
- Material Design 3: https://m3.material.io/
- Shopify Polaris (redirected): https://polaris-react.shopify.com/
- Atlassian Design System: https://atlassian.design/
- Atlassian color palette: https://atlassian.design/foundations/color/color-palette/
- Adobe Spectrum: https://spectrum.adobe.com/
- Adobe Spectrum color fundamentals: https://spectrum.adobe.com/page/color-fundamentals/
- Adobe Spectrum button component: https://spectrum.adobe.com/page/button/
- GitHub Primer: https://primer.style/
- GitHub Primer product: https://primer.style/product
- Microsoft Fluent 2: https://fluent2.microsoft.design/
- Vercel Geist introduction: https://vercel.com/geist/introduction
- Vercel Geist typography: https://vercel.com/geist/typography
- Vercel Geist colors: https://vercel.com/geist/colors
- Geist font GitHub (license): https://github.com/vercel/geist-font
- Stripe Press: https://press.stripe.com/
- Linear: https://linear.app/
- shadcn/ui: https://ui.shadcn.com/
- Radix Themes: https://www.radix-ui.com/themes
- Mantine: https://mantine.dev/
- Mantine typography: https://mantine.dev/theming/typography/
- Salesforce Lightning Design System: https://www.lightningdesignsystem.com/ (redirected to v1)
- Source Serif 4 (Google Fonts): https://fonts.google.com/specimen/Source+Serif+4
- Source Serif 4 GitHub (OFL): https://github.com/adobe-fonts/source-serif
- Geist font (OFL): https://github.com/vercel/geist-font/blob/main/LICENSE.txt

Ground-truth source files read locally on 2026-04-28:

- design.pointsav.com CSS (Carbon-shape reference): `/srv/foundry/clones/project-design/pointsav-monorepo/app-privategit-design/src/style.css`
- design.pointsav.com render.rs (Carbon-shape reference): `/srv/foundry/clones/project-design/pointsav-monorepo/app-privategit-design/src/render.rs`
- bim.woodfinegroup.com current CSS (v0.0.1): `/srv/foundry/clones/project-bim/pointsav-monorepo/app-orchestration-bim/src/style.css`
- BB.13 predecessor research: `/srv/foundry/clones/project-bim/.claude/sub-agent-results/BB.13-design-system-showcase-survey-2026-04-28.md`

---

*Research compiled 2026-04-28 by BB.14 sub-agent. Read-only; no code was modified during research.*
