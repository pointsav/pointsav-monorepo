# RD.6 — AEC-Native Precedent Survey (post-April-2026 check)

Date: 2026-07-03
Task: Re-check whether anything genuinely BIM/AEC-native has emerged (or was missed) as a
documentation/catalog-site design precedent since the April 2026 research (BB.13/BB.14,
synthesized in RD.4), which settled on Adobe-Spectrum chrome + 5 AEC-specific markers.
Method: live WebFetch/WebSearch of the named targets, plus direct inspection of published
CSS/npm packages where doc sites were JS-rendered (values below are grep'd from shipped
CSS, not recalled).

---

## Verdict up front

1. **The "no direct prior art for a BIM-data design system" conclusion still holds.**
   Nothing has launched since April 2026 that publishes a design system or DTCG token
   catalog *for built-environment data*. The DTCG spec itself reached its first stable
   version (2025.10, W3C community group) — which strengthens the timing argument — but
   no BIM/IFC-anchored token library exists in that ecosystem.
2. **However, the April survey had a real blind spot: AEC-vendor design systems for
   software UIs.** Bentley **iTwinUI**, Trimble **Modus**, and Autodesk **Weave** all
   exist, are public, and none appear in the BB.13/BB.14 survey lists (which covered
   generic enterprise DS + AEC authoring tools, but no AEC-vendor DS). iTwinUI in
   particular is the closest thing to a "BIM-native design system" in existence.
   **None of them displaces the Spectrum pick** (reasons below), but they change what we
   can safely claim publicly, and iTwinUI has two or three patterns worth borrowing.
3. bSDD, RealEstateCore, and Brick remain data sources / schema repos with utilitarian
   or generator-default front-ends — confirmed, no design identity worth studying.

---

## 1. buildingSMART / bSDD — no design precedent

- `search.bsdd.buildingsmart.org` ("bSDD Search") is a plain utilitarian search
  front-end — a search box over the dictionary, no distinct visual identity.
- The bSDD GitHub repo states explicitly that they do not publish the front-end of the
  website; the public surface is the search UI + API + a Manage portal for authors.
- `buildingsmart.org` and `bsdd.buildingsmart.org` both returned HTTP 403 to automated
  fetch; the accessible search interface and forum/tech-update trail show no evidence of
  a 2025–2026 redesign into a designed catalog site.
- **Status unchanged from April: bSDD is the data source the token catalog should link
  to (`$extensions.bsdd` URIs, IFC anchor links), not a visual reference.**

## 2. RealEstateCore — generator themes, no identity

Two documentation surfaces, both generic:
- `dev.realestatecore.io` — Jekyll + **Minimal Mistakes** theme (self-declared in
  footer). Top nav + left sidebar, default theme styling, minimal branding beyond a logo.
- `doc.realestatecore.io` — WIDOCO/pyLODE **auto-generated** ontology documentation for
  REC 2.x–4.0. Machine-generated ontology reference pages, no designed chrome.
- **Conclusion: schema repo with default-theme docs. Not a precedent.**

## 3. Brick Schema — modest, utilitarian

- `brickschema.org` is a technical documentation/landing site with modest design
  investment: horizontal nav, anchor-linked doc sections, custom logo, links out to
  GitHub and Read-the-Docs-style ontology/user docs. Professional but utilitarian;
  no catalog-site design language. **Not a precedent.**

## 4. Autodesk Platform Services docs + Tandem — real, but generic developer-portal grammar

- APS docs (`aps.autodesk.com/en/docs/...`) are a JS-rendered SPA; automated fetch sees
  little. What is verifiable from the served HTML: the site loads **Artifakt /
  ArtifaktElement / ArtifaktLegend** — Autodesk's proprietary brand font family — and
  uses standard REST-doc anatomy (top product nav, left API tree, Request/Response
  sections, Sketch-exported SVG icons). Palette values in the static HTML are limited
  (black `#000000`, gray-blue `#BBC2C8`); the chrome is the standard Autodesk
  developer-portal look: dark top band, white content, light-blue links.
- **Tandem has no separate doc site.** The Tandem Data API docs live inside the same APS
  chrome at `aps.autodesk.com/en/docs/tandem/v1/`, supplemented by a Postman collection
  and GitHub samples. Tandem's own visual identity is the *application* (dark viewer
  UI), not its documentation.
- **Judgment: APS/Tandem docs are competent generic developer-portal design with a
  proprietary brand font. There is no AEC-specific visual convention in the docs layer
  itself** (the AEC-ness lives in embedded viewer screenshots — consistent with the
  April finding that dark-viewport-in-light-docs is the convention). Nothing here
  improves on the Spectrum base; Artifakt is proprietary and unusable anyway.

## 5. The blind spot: AEC-vendor design systems (missed in April, pre-existing, public)

The April research surveyed 15 generic design-system sites and 7 AEC authoring
tools/viewers — but zero design systems *published by AEC software vendors*. Three exist:

### 5a. Bentley iTwinUI — the closest thing to "BIM-native design system" that exists

`itwinui.bentley.com` (docs), `github.com/iTwin/iTwinUI` (source), npm
`@itwin/itwinui-{css,variables,react}`. A design system explicitly for building web UIs
for **iTwin.js infrastructure digital-twin applications** — i.e., BIM-adjacent by design,
not by accident. Verified visual identity (from docs site + shipped
`@itwin/itwinui-variables` CSS):

- **Typography:** Noto Sans (body) + Noto Sans Mono. Font-size scale `--iui-font-size-0`
  through `-5`; weights light/normal/semibold/bold.
- **Color (light theme, from shipped CSS):** accent aliases to "informational" blue
  `#0071b8` (hover `#00568f`); text `#242424` on white; zebra bg `#fbfbfc`; hover
  `#f7f8f8`; backdrop `#eef0f1`. Status set: informational `#0071b8`, positive
  `#157e11`, warning `#8f6400`, negative `#c52b26`, each with hover + muted variants.
  Light, dark, **and high-contrast versions of both** — four themes total.
- **A named "soft" categorical background palette** unlike anything in the generic DS
  world: `skyblue #b5e1f2`, `celery #c3d57c`, `froly #fbafad`, `steelblue #9cbdd8`,
  `sunglow #ffd166`, `seabuckthorn #f9b371`, `montecarlo #99d6d2`, `poloblue #abc3de`,
  `bouquet #cbb3c9`, `ash #dedbd3`, `oak #ceb5a1`. This is an element/category
  color-coding substrate — the design-token formalization of the AEC habit of coloring
  model elements by discipline/category.
- **Component set (60+)** is visibly shaped by model-centric apps: **Tree**, data-dense
  **Table**, **Panels**, Side navigation, TransferList, Stepper, Tile — the
  spatial-tree/properties-panel grammar the cluster identified independently.
- **Tokens:** T-shirt spacing (3xs–3xl, m=1rem), border-radius-1 + round, elevation
  shadows 0–5, opacity + duration scales. The docs site deliberately does **not** print
  token values ("we purposefully do not expose the variables' values on this site" —
  values live in the raw CSS), an interesting contrast with our token-bundle-page
  approach of showing raw DTCG JSON behind a disclosure.
- **Docs-site chrome itself:** conventional — left component sidebar, top bar with
  GitHub link, light default, live demos. Clean but not distinctive.

**Why it does NOT displace the Spectrum pick:**
- Its thumbnail identity (white bg, mid-blue `#0071b8` accent, neutral sans) is exactly
  the Carbon-adjacent blue-sans-sidebar cluster the bankers' test penalizes. Scored on
  BB.14's axes it would land roughly Color 2–3 / Typography 3 / Chrome 2 — far below
  Spectrum's 14/15. Adopting it as the base would undo the distinguishability work.
- It is a design system for *application UIs*, not a documentation/catalog-site design
  precedent, and not a system for *BIM data*.

**Why it matters anyway (complementary):**
- **Validation:** an AEC vendor independently converged on the same primitives the
  cluster derived (tree + dense table + panels first-class; light docs / dark app;
  status color quartet). Good citable corroboration for `bim-aec-muscle-memory.md`.
- **Borrowable pattern #1 — categorical soft palette:** a named muted-background
  palette for discipline/category chips (our IFC class / Uniclass / MEP chips currently
  use 3 ad-hoc chip backgrounds; iTwinUI shows what the systematized 11-step version
  looks like). Worth considering when chip families grow past the current three.
- **Borrowable pattern #2 — four-theme accessibility posture:** light/dark each with a
  high-contrast variant is a strong precedent for a field-conditions story (site
  offices, sunlight) if full dark mode lands in v0.1.0.
- **Positioning constraint:** see §6.

### 5b. Trimble Modus — construction-vendor DS, Bootstrap-flavored

`modus.trimble.com` ("Modus 2.0 Blueprint", OneTrimble Design System); open source at
`trimble-oss`. Verified from shipped `@trimble-oss/modus-bootstrap@2.3.2` CSS:
**Open Sans**; Trimble blue `#0063a3` (light) / `#019aeb` (dark accent); dark body
`#171c1e`, light body white with text `#252a2e`; Bootstrap 5 base. The doc site is a
JS-rendered styleguide. **Judgment: corporate Bootstrap-derived system for Trimble's
product fleet (construction/geospatial). Generic visual language; no BIM-data
conventions; not a precedent to borrow from — but it is a third data point that every
major AEC vendor now runs a public-ish design system.**

### 5c. Autodesk Weave — public Storybook, not a designed doc site

`storybook.weave.autodesk.com`, npm `@weave-design/*`, GitHub `Autodesk/hig`
("Autodesk's unified design system"), used by Forma (the Forma Site Design API's design
guidelines on APS point to Weave). Public surface is a Storybook instance + npm theme
data (8 themes, ES/JSON/SCSS) — a component reference, not a designed documentation
site. ArtifaktElement typography, Autodesk brand palette. **Not a catalog-site
precedent; relevant only as ecosystem context.**

## 6. Positioning consequence (recommend relaying to the strategy brief)

The RD.4/BB-era framing "no design system by/for AEC exists" is **falsifiable as
worded** — iTwinUI, Modus, and Weave are all public AEC-vendor design systems. The claim
that survives scrutiny, and should be the only one used in public/BCSC-reviewable
copy, is the narrower one:

> No one publishes a design system for **built-environment data** — an IFC-anchored,
> DTCG-format token catalog. Existing AEC design systems (Bentley iTwinUI, Trimble
> Modus, Autodesk Weave) style the *software*; nothing styles/structures the *building
> data itself*.

That distinction also strengthens the pitch: three major vendors each built a private
software-UI design system, and none of them extended it downward into the data layer —
the gap is real and now precisely nameable.

## 7. Net effect on the settled design direction

- **Spectrum-chrome + Source Serif 4/Geist + `#1A4480` + 5 AEC markers: unchanged.**
  Nothing found is a better base; iTwinUI would actively hurt distinguishability.
- **Additions worth queuing (small):** (a) cite iTwinUI in `bim-aec-muscle-memory.md`
  as vendor corroboration; (b) consider an iTwinUI-style named soft categorical palette
  if chip families grow; (c) note iTwinUI's high-contrast theme pair as the precedent
  for a future field-use accessibility mode; (d) tighten public positioning language
  per §6.
- **bSDD/RealEstateCore/Brick:** remain data/bridge targets only, exactly as the token
  strategy doc treats them. No visual follow-up needed.

---

Sources (fetched 2026-07-03): itwinui.bentley.com/docs + /docs/variables;
unpkg.com/@itwin/itwinui-variables (CSS grep); github.com/iTwin/iTwinUI;
modus.trimble.com; unpkg.com/@trimble-oss/modus-bootstrap@2.3.2 (CSS grep);
storybook.weave.autodesk.com + github.com/Autodesk/hig (via search);
aps.autodesk.com/en/docs/tandem/v1/ + /en/docs/viewer/v7 (HTML grep: Artifakt fonts);
search.bsdd.buildingsmart.org; github.com/buildingSMART/bSDD; dev.realestatecore.io;
doc.realestatecore.io; brickschema.org; designtokens.org (DTCG 2025.10 stable);
buildingsmart.org (403 to automated fetch — association site not directly inspected).
