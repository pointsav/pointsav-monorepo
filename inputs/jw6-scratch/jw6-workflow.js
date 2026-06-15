export const meta = {
  name: 'jw6-10x-quality-pass',
  description: 'JW6 overnight 10x quality improvement: visual enrichment, language consistency, reorder, research augments',
  phases: [
    { title: 'Discovery' },
    { title: 'Visual Enrichment' },
    { title: 'Content Augments' },
    { title: 'Language Pass' },
    { title: 'Research Expansion' },
    { title: 'Merge & Reorder' },
    { title: 'Assembly' },
    { title: 'Verification' },
    { title: 'Commit' },
  ],
}

// ── PATHS ──
const BASE = '/srv/foundry/clones/project-orgcharts'
const JW5  = BASE + '/inputs/IT SUPPORT_PointSav_2026_06_14_Design Slides_JW5/IT SUPPORT_PointSav_2026_06_14_Design Slides_JW5.html'
const JW6D = BASE + '/inputs/IT SUPPORT_PointSav_2026_06_14_Design Slides_JW6'
const JW6  = JW6D + '/IT SUPPORT_PointSav_2026_06_14_Design Slides_JW6.html'
const SCR  = BASE + '/inputs/jw6-scratch'
const DG   = BASE + '/inputs/datagraph-pointsav'
const PDF  = BASE + '/inputs/IT SUPPORT_PointSav_2026_06_09_Design Slides_Pipeline/PointSav_Onboarding_Design Slides_V5.pdf'

// ── SHARED RULES ──
const CSS_AVAIL = '.cmp2 .cmp-col .cmp-head .cmp-item .cmp-rule .cmp-sub .cmp-rules .cmp-div .slab .vc-cyl .vc-stack .db .doc .arch .future-callout .canvas .wire .node .totebox .vbar .badge .backend .vmscale .vmstep .para .hexagon .diamond .cloud .cube .adminwin .inpanel .tbsvc-ex .vc-verdict .arch-compare .vs-sep .svc-ladder .rung .rn .arch-card'
const TOKENS    = '--blue:#0E7C84  --green:#54924E  --orange:#F15F22  --ink-100 (darkest) through --ink-600 (lightest)  --paper-100 (white) --paper-200 (off-white) --paper-300 (light grey)  --highlight-50:#FDE8DD'
const BCSC      = 'seL4 and Sovereign Data Foundation: PLANNED/INTENDED only — never presented as shipped. Every seL4 mention must include "planned" or "intended".'
const DUAL      = 'Every crumb and bullet works for both: 28-year-old Rust developer AND 55-year-old pension fund manager simultaneously — neither audience feels the slide was written for the other.'
const BANNED    = '"revolutionary" "cutting-edge" "AI-powered" "innovative" "state-of-the-art" "next-generation" "game-changer" "disruptive" "seamless" "robust" "leverage" (as verb) "unlock" "empower" "journey" "solution" (standalone noun)'
const NOTES_Q   = 'All .notes-strip questions must be SPECIFIC. Banned: "What is this?". Required: technical ownership or strategic implication questions. Example: "Which service here would you own first, and what would the first pull request change?"'

function slug(s) {
  return s.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '')
}

// ── CAT-TAG CANON ──
const CAT_TAG = {
  '01': 'Developer Platform',
  '02': 'Operator Workspace',
  '03': 'System of Record',
  '04': 'Integration & Data Portability',
  '05': 'Machine-Based Authorization',
  '06': 'Multi-Entity Consolidation',
  '07': 'Platform Foundation',
  'nav': 'Platform Overview',
}

// ── FINAL SLIDE ORDER ──
const ORDER = {
  nav: ['Cover','Contents','How We Select Contributors','Icon Index'],
  '01': ['Contributors','Design System','Color Values','Typographic Protocols','Naming Conventions','Repositories','Use Cases','SMB Environment','Deployment Scenarios','Licensing Model','Documentation Standards','Coding Guidelines'],
  '02': ['Interface OS','Input Machine','System Administration','Command Center','Interfaces','BIM Console','Maps Console','Building Preferences','Console Search'],
  '03': ['VM vs Containers','Toteboxes','Totebox Archive','WORM Record Format','Bookkeeping','Chart-of-Accounts Names','Partition Layout','IoT Data','Geo-spatial'],
  '04': ['Totebox Service Architecture','Totebox Service Catalog','Products & Services','Economic Model','Communications','Solid / Pods','GIS Catchment','Docs Navigation'],
  '05': ['Private Network','Pairing as Permissions','Authentication','Registry Token'],
  '06': ['Multiple Archives','Deployment Tiers Matrix','Scaling — Composition','Three-Layer Stack','Owner / Operator'],
  '07': ['On-prem','Leased Servers','Public Cloud','Hybrid Network','GIS Engine','OS Matrix','seL4 Kernel','GIS Stack','Cluster Method','Terminal Rendering','Co-location Thesis'],
}

const RENAMED = {
  'Totebox Services (Containers)': 'Totebox Service Architecture',
  'Totebox Services (Ladder)': 'Totebox Service Catalog',
}

const DIVIDERS = {
  '01': '01 Developer Platform',
  '02': '02 Operator Workspace',
  '03': '03 System of Record',
  '04': '04 Integration & Data Portability',
  '05': '05 Machine-Based Authorization',
  '06': '06 Multi-Entity Consolidation',
  '07': '07 Platform Foundation',
}

// ── WP-A: VISUAL ENRICHMENT TARGETS ──
const WPA = [
  { label: 'Documentation Standards', rfile: 'research-01-developer-platform.yaml',
    pattern: 'Add a flex row of 4 .badge elements (42x42px, border-radius:50%) ABOVE the existing .cmp-rules block. Labels and colors: UML (background:var(--blue), white text), C4 (background:var(--green), white text), 4+1 (background:var(--orange), white text), ISO 42010 (background:var(--ink-300), white text). Wrap row in: <div style="display:flex;gap:16px;margin-bottom:24px">. Font: 13px bold centered.' },
  { label: 'Chart-of-Accounts Names', rfile: 'research-03-system-of-record.yaml',
    pattern: 'Add a horizontal name-decomposition diagram as 6 .slab divs in a flex row (gap:3px; margin-bottom:20px). Each 145px wide, 54px tall. Labels centered 11px bold: ENTITY / ACCOUNT / DATE / DOCTYPE / VENDOR / DESCRIPTION. Alternate background: var(--paper-200) and var(--paper-300). Separate each pair with a small ">" character span styled ink-400.' },
  { label: 'Licensing Model', rfile: 'research-01-developer-platform.yaml',
    pattern: 'Add two .totebox boxes (width:210px; height:120px; display:inline-flex; flex-direction:column; align-items:center; justify-content:center; gap:8px) in a flex row (justify-content:center; gap:48px; margin:20px 0). Left: "Single Archive" h4 + "Free — no coordination cost" small. Right: "Multi-Archive" h4 + "Coordination layer" small. Below each add a .vc-verdict chip: left class="vc-verdict good" "No cost", right class="vc-verdict warn" "Subscription".' },
  { label: 'WORM Record Format', rfile: 'research-03-system-of-record.yaml',
    pattern: 'Add a comparison table using .arch-compare (5-column grid: 1fr auto 1fr auto 1fr becomes: label col | vs | DB col | vs | WORM col). Use two columns: "Relational DB" and "WORM Archive". For 4 properties show .vc-verdict chips: (1) Mutable records: DB=vc-verdict warn "Yes", WORM=vc-verdict good "No"; (2) Append-only: DB=warn "No", WORM=good "Yes"; (3) Audit trail: DB=warn "Config", WORM=good "Built-in"; (4) Regulatory: DB=warn "Add-on", WORM=good "Native".' },
  { label: 'Economic Model', rfile: 'research-04-integration-data-portability.yaml',
    pattern: 'Replace the main bullet content with 3 .future-callout divs (gap:12px). Each has class="future-callout" with a .fc-kick label and a concise body sentence. Callout 1: .fc-kick "Pillar 1 — Decentralization", body: a sentence about no central coordinator required. Callout 2: .fc-kick "Pillar 2 — Auth as Permissions", body: pairing IS the access grant. Callout 3: .fc-kick "Pillar 3 — Portability", body: EU Data Act compliance by design.' },
  { label: 'Solid / Pods', rfile: 'research-04-integration-data-portability.yaml',
    pattern: 'Add a .cmp2 two-column with a .cmp-div between. Left .cmp-col: .cmp-head with .ch-t "W3C Solid Pod" + .arch box showing "App A" and "App B" as .badge items pointing to a "Pod" .db box. Right .cmp-col: .cmp-head with .ch-t "PointSav Totebox" + .arch box showing "Console" and "Archive" as separate .badge items (illustrating the console/archive separation).' },
  { label: 'Communications', rfile: 'research-04-integration-data-portability.yaml',
    pattern: 'Add a .svc-ladder before the existing content. 3 .rung divs (height:60px each, border-bottom:2px solid var(--blue)). Each rung has: .rn element on left (30px, background:var(--blue), white text, font-size:11px) and label text. Rung 1: .rn "1", label "tmp/ — staging area". Rung 2: .rn "2", label "new/ — delivered". Rung 3: .rn "3", label "cur/ — processed". After ladder add a .cmp-sub span: "one file per message — append-only, non-erasable".' },
  { label: 'Partition Layout', rfile: 'research-03-system-of-record.yaml',
    pattern: 'Add a .vc-stack (display:flex; flex-direction:column; gap:3px; margin-bottom:20px; width:320px). Slabs from top to bottom (each is a .slab div): Recovery (height:52px, background:var(--paper-300), label "R — Recovery"), OS (height:72px, background:rgba(14,124,132,0.1), label "OS — Operating System"), S1 (height:46px, label "S1 — Service Store"), S2 (height:46px, label "S2"), S3 (height:46px, label "S3"), S4 (height:46px, label "S4 — Service Store"). Each slab centered 13px text.' },
  { label: 'Registry Token', rfile: 'research-05-machine-based-authorization.yaml',
    pattern: 'Add a .canvas div (position:relative; height:130px; margin:16px 0) containing: a .totebox (position:absolute; left:20px; top:40px; width:100px; height:60px) labelled "Device"; a .doc box (position:absolute; left:50%; transform:translateX(-50%); top:20px; width:140px) with .doc-t "Token Record" and .doc-meta "IP · timestamp · key"; an .arch box (position:absolute; right:20px; top:35px; width:110px; height:70px) labelled "Totebox Archive". Add a .wire SVG connecting them with two horizontal arrows.' },
  { label: 'Co-location Thesis', rfile: 'research-07-platform-foundation.yaml',
    pattern: 'Add a .vmscale element (position:relative; height:80px; margin:20px 0; border-bottom:2px solid var(--ink-300)) with 3 .vmstep markers. Each .vmstep has a .sq (14x14px colored square) and .vlabel (12px text below). Position at left:15%, 50%, 82%. Step 1: .sq background:var(--blue), .vlabel "5 km — Degree 1". Step 2: .sq background:var(--orange), .vlabel "25 km — Degree 2". Step 3: .sq background:var(--green), .vlabel "80 km — Degree 3".' },
]

// ── WP-B: CONTENT AUGMENT TARGETS ──
const WPB = [
  { label: 'GIS Engine', pages: '91-96,102-106,127-130', rfile: 'research-07-platform-foundation.yaml',
    inject: 'Full GIS build stack: PostGIS (spatial query engine) + QGIS (analysis and export) + OpenStreetMap (POI source, $0) + GeoNode (publication layer). O-D catchment data economics: commercial providers $50k–$150k/yr vs OSM $0. 11-dimension site-ranking matrix (tier score, civic anchor, accessibility, catchment size, transit proximity, employment density, etc.). Franklin TN as a worked T1/T2 cluster analysis example.' },
  { label: 'Pairing as Permissions', pages: '6,28', rfile: 'research-05-machine-based-authorization.yaml',
    inject: 'Registry Token mechanism: device pairing binds the device IP address cryptographically in an immutable Registry Token — the token IS the permission record; no separate credential store. Three economic model pillars: (1) decentralized orchestration — no central authority required, (2) auth as permissions — the pairing is the access grant, (3) freely transferable archives — move the archive, the access follows.' },
  { label: 'Design System', pages: '23,26,31,36-42', rfile: 'research-01-developer-platform.yaml',
    inject: 'Four documentation standards used by the platform: ISO/IEC 42010 (architecture description standard), C4 Model (context/container/component/code diagram hierarchy), 4+1 View Model (logical/process/physical/development/scenario), UML 2.5.1 (sequence and component diagrams). Power of 10 coding rules (NASA/JPL safety-critical discipline adapted for Rust). Media Assets Directory as a named output artifact of the design system.' },
  { label: 'Bookkeeping', pages: '68,88', rfile: 'research-03-system-of-record.yaml',
    inject: 'Concrete WORM filename example: BOOKKEEPING_TITLECO1_2025_03_07_Invoice_Property Management_Snow Removal.PDF — show the 6 filename segments (ENTITY / ACCOUNT / DATE / DOCTYPE / VENDOR / DESCRIPTION). The filename REPLACES a database record number: no separate DB lookup required because the file system IS the index. This is the flat-file WORM discipline made concrete.' },
  { label: 'Interface OS', pages: '85,86,110', rfile: 'research-02-operator-workspace.yaml',
    inject: 'F-keys console Search surface: structured filter fields (entity type, date range, record category), keyboard-driven navigation (no mouse required), CSV/JSON export. Proprietary interface stack sold as separate chargeable products: Command Centre API (orchestration access), Data Engine (analytics), Data Marketplace (licensed third-party data), BIM Server (building information models), Building ID Server (property registry). Each connects to the Totebox OS via the private network.' },
]

// ── WP-C: LANGUAGE PASS CATEGORIES ──
const WPC = [
  { id: '01', name: 'Developer Platform', rfile: 'research-01-developer-platform.yaml' },
  { id: '02', name: 'Operator Workspace', rfile: 'research-02-operator-workspace.yaml' },
  { id: '03', name: 'System of Record', rfile: 'research-03-system-of-record.yaml' },
  { id: '04', name: 'Integration & Data Portability', rfile: 'research-04-integration-data-portability.yaml' },
  { id: '05', name: 'Machine-Based Authorization', rfile: 'research-05-machine-based-authorization.yaml' },
  { id: '06', name: 'Multi-Entity Consolidation', rfile: 'research-06-multi-entity-consolidation.yaml' },
  { id: '07', name: 'Platform Foundation', rfile: 'research-07-platform-foundation.yaml' },
]

// ── WP-D: RESEARCH EXPANSION TARGETS ──
const WPD = [
  { topic: 'Competitive Positioning', out: 'research-competitive-positioning.yaml',
    scope: 'Find 8-12 authoritative sources: (1) W3C Solid Protocol vs data portability architectures (portability comparison), (2) seL4 formally-verified microkernel vs other verified OS projects: Fiasco.OC (TU Dresden), Genode OS Framework, L4Linux — comparative status and regulated-industry adoption, (3) open-core vs dual-license enterprise models (GitLab MIT/EE, HashiCorp MPL/BSL, Elastic Apache/SSPL) — what triggers the commercial boundary, (4) analyst reports on verified OS adoption in regulated industries (aviation, medical, financial).' },
  { topic: 'Customer TAM', out: 'research-customer-tam.yaml',
    scope: 'Find 8-12 authoritative sources: (1) pension fund and institutional asset manager digital transformation budgets (McKinsey, Deloitte, EY), (2) commercial real estate PropTech market size 2024-2028 (JLL, CBRE, Gartner, IDC), (3) regulated-industry compliance software spending (financial services, legal, property management), (4) Gartner sovereign cloud IaaS $37B-$169B forecast original source and any vertical breakdown, (5) SMB cloud adoption spending in property management.' },
  { topic: 'Regulatory Timelines', out: 'research-regulatory-timelines.yaml',
    scope: 'Find 8-12 authoritative sources: (1) EU Data Act 2023/2854 application date, grace periods by obligation type, Arts 23-31 cloud switching rights enforcement, (2) GDPR Art.20 data portability enforcement history and DPA guidance, (3) SEC Rule 17a-4 amendment 2022 effective date and WORM technical requirements for broker-dealers, (4) NIST SP 800-207 publication date and US federal agency adoption mandate, (5) CISA Zero Trust Maturity Model v2.0 phases and timelines.' },
]

// ══════════════════════════════════════════════════
// PHASE 1: DISCOVERY + DIRECTORY SETUP
// ══════════════════════════════════════════════════
phase('Discovery')
log('Phase 1: Extracting JW5 structure and creating scratch directories...')

const DISC_SCHEMA = {
  type: 'object',
  required: ['slide_count', 'slides'],
  properties: {
    slide_count: { type: 'number' },
    slides: {
      type: 'array',
      items: {
        type: 'object',
        required: ['position', 'data_label', 'category', 'is_divider'],
        properties: {
          position: { type: 'number' },
          data_label: { type: 'string' },
          category: { type: 'string', description: 'nav|01|02|03|04|05|06|07|divider' },
          is_divider: { type: 'boolean' },
          has_notes_strip: { type: 'boolean' },
          visual_complexity: { type: 'string', enum: ['none', 'low', 'high'] },
          cat_tag: { type: 'string' },
        }
      }
    }
  }
}

const [disc, _dirs] = await parallel([
  () => agent(
    `Read the PointSav JW5 deck at "${JW5}" (use Read tool, load the full file).

Do two things:

TASK 1 — Write the CSS header to disk:
Everything from <!DOCTYPE html> through the last line before the very first <section> element (this includes all <style> blocks, CSS custom property definitions, and the deck-stage.js <script> block). Preserve EXACTLY character for character.
Write to: "${SCR}/css-header.html"

Also write everything after the final </section> tag (the HTML footer, typically </div></body></html>) to:
"${SCR}/html-footer.html"

TASK 2 — Return slide manifest:
For each <section data-label="..."> element in the file, record:
- position: 1-based index of this section in the file
- data_label: value of data-label attribute
- category: which TOC category this slide belongs to. Determine by scanning from the top: nav=Cover/Contents/How We Select Contributors/Icon Index; divider=a section that is a category chapter header (data-label starts with "01 "/"02 "/" etc.); after each divider, slides belong to that category until the next divider.
- is_divider: true if this section is a category chapter header (not a content slide)
- has_notes_strip: true if the section HTML contains class="notes-strip"
- visual_complexity: "high" if has SVG or class="canvas" or class="cube" or class="hexagon"; "low" if has class="cmp-rules" without SVG; "none" otherwise
- cat_tag: text of <span class="cat-tag"> element, or empty string

Return all slides in the structured output.`,
    { schema: DISC_SCHEMA, model: 'sonnet', label: 'discover-jw5', phase: 'Discovery' }
  ),
  () => agent(
    `Create required directories for the JW6 build:
mkdir -p "${SCR}/visual"
mkdir -p "${SCR}/content"
mkdir -p "${SCR}/language"
mkdir -p "${SCR}/merge"
mkdir -p "${JW6D}"
echo "Directories ready"`,
    { model: 'haiku', label: 'setup-dirs', phase: 'Discovery' }
  ),
])

if (!disc) {
  log('FATAL: Discovery agent returned null.')
  return { error: 'discovery_failed' }
}
log('Discovery: ' + disc.slide_count + ' slides found in JW5.')

// ══════════════════════════════════════════════════
// PHASE 2: ALL 26 WP AGENTS IN PARALLEL
// ══════════════════════════════════════════════════

log('Phase 2: Launching all 26 WP agents simultaneously (WP-A×10, WP-B×5, WP-C×7, WP-D×3, WP-E×1)...')

await parallel([

  // ─── WP-A: Visual Enrichment (10 agents) ───
  ...WPA.map(t => () => agent(
    `Improve slide "${t.label}" in the PointSav JW5 deck with a CSS visual diagram.

Step 1: Read "${JW5}" using Read tool. Find <section data-label="${t.label}"> and extract it completely.
Step 2: Read "${DG}/${t.rfile}" for domain context (optional — helps with accurate terminology).

ADD THIS VISUAL (use ONLY these existing CSS classes — ${CSS_AVAIL}):
${t.pattern}

HARD RULES:
1. NO new CSS classes. Zero. Only existing classes listed above.
2. Preserve .head div content EXACTLY (eyebrow, h2, crumb text — no changes).
3. Preserve existing .notes-strip EXACTLY if present.
4. Do not remove any existing content — ADD the visual to it.
5. ${BCSC}
6. ${DUAL}
7. Banned vocabulary: ${BANNED}
8. Design tokens for inline styles if needed: ${TOKENS}

Step 3: Write the COMPLETE revised <section>...</section> HTML to:
"${SCR}/visual/${slug(t.label)}.html"
Write ONLY the HTML — no markdown, no explanation in the file.

Return one sentence confirming what visual was added.`,
    { model: 'opus', label: 'wpa-' + slug(t.label), phase: 'Visual Enrichment' }
  )),

  // ─── WP-B: Content Augments (5 agents) ───
  ...WPB.map(t => () => agent(
    `Augment slide "${t.label}" with content from V5 PDF source material.

Step 1: Read "${JW5}" using Read tool. Extract <section data-label="${t.label}"> completely.
Step 2: Read V5 PDF pages ${t.pages} from "${PDF}" using Read tool with pages parameter.
Step 3: Read "${DG}/${t.rfile}" for research context and terminology.

CONTENT TO INJECT from the V5 source pages:
${t.inject}

RULES:
1. Only existing CSS classes: ${CSS_AVAIL}
2. Design tokens: ${TOKENS}
3. ${BCSC}
4. ${DUAL}
5. Banned vocabulary: ${BANNED}
6. ${NOTES_Q}
7. Integrate content naturally into the slide — do not just append a raw list.
8. Update the crumb sentence to reflect the enriched content.
9. Improve .notes-strip questions to be specific to the new content injected.
10. Slide must remain self-contained (no dependency on adjacent slides).

Step 4: Write complete revised <section>...</section> to:
"${SCR}/content/${slug(t.label)}.html"
Write ONLY the HTML. Return: one-sentence summary of what was injected.`,
    { model: 'opus', label: 'wpb-' + slug(t.label), phase: 'Content Augments' }
  )),

  // ─── WP-C: Language + Cat-tag + Notes-strip Pass (7 agents) ───
  ...WPC.map(c => () => agent(
    `Apply Bloomberg-standard language discipline to all slides in Category ${c.id} — ${c.name}.

SLIDES TO PROCESS: ${JSON.stringify(ORDER[c.id])}

Step 1: Read "${JW5}" using Read tool. For each slide in the list above, extract its <section data-label="[label]"> block.
Step 2: Read "${DG}/${c.rfile}" for authoritative terminology and regulatory language context.

Apply ALL rules to EVERY slide in this category:

RULE 1 — BANNED VOCABULARY: Replace every instance of: ${BANNED}
Replace with precise direct language. Example: "seamless integration" → "direct filesystem access without middleware"

RULE 2 — BCSC: ${BCSC}

RULE 3 — DUAL AUDIENCE: ${DUAL}

RULE 4 — CAT-TAG: Set class="cat-tag" content to exactly "${CAT_TAG[c.id]}" for all content slides.

RULE 5 — NOTES-STRIP: ${NOTES_Q}
• If .notes-strip is MISSING: add one with 3 specific questions.
• If questions are GENERIC: replace with specific technical or strategic questions.

RULE 6 — CRUMB QUALITY: Each crumb = one sentence, active voice, present tense, specific.
Must answer: "Why does this matter to someone evaluating PointSav right now?"

DO NOT CHANGE: eyebrow text, h2 heading, SVG elements, data-label attribute, data-screen-label attribute, CSS class structure, visual layout.

For EACH slide, write the revised <section>...</section> HTML to:
"${SCR}/language/[SLUG].html"
where [SLUG] = the data-label lowercased with spaces and special characters replaced by hyphens, leading/trailing hyphens removed.
Examples: "Interface OS" → interface-os.html, "VM vs Containers" → vm-vs-containers.html, "Solid / Pods" → solid-pods.html

Write ONLY the HTML to each file. Return: list of slides processed and main change type per slide.`,
    { model: 'opus', label: 'wpc-cat' + c.id, phase: 'Language Pass' }
  )),

  // ─── WP-D: Research Expansion (3 agents) ───
  ...WPD.map(t => () => agent(
    `Research and document authoritative sources for the PointSav slide knowledge base.

TOPIC: ${t.topic}
SCOPE: ${t.scope}

Find 8-12 high-quality, authoritative sources (official regulatory texts, academic papers, analyst reports, vendor documentation — NOT Wikipedia as primary source).

For each source record:
- url: direct link
- title: exact document title
- year: publication year
- publisher: organization name
- relevance: 2-3 sentences on why this source matters for PointSav design slides
- slide_context: which slide title(s) this enriches (from: ${Object.values(ORDER).flat().join(', ')})
- key_stat: the single most quotable fact, figure, or date from this source

Write results to "${DG}/${t.out}" in this YAML format (create the file):
---
topic: ${t.topic}
generated: 2026-06-14
schema: foundry-datagraph-flat-v1
---
sources:
  - url: "https://..."
    title: "..."
    year: "YYYY"
    publisher: "..."
    relevance: "..."
    slide_context: "..."
    key_stat: "..."

Write the file now. Return: top 3 most impactful findings.`,
    { model: 'opus', label: 'wpd-' + slug(t.topic), phase: 'Research Expansion' }
  )),

  // ─── WP-E: Merge Scaling—Tiers + Deployment Tiers (1 agent) ───
  () => agent(
    `Merge two PointSav slides into one combined visual slide.

Step 1: Read "${JW5}" using Read tool.
Extract: <section data-label="Scaling — Tiers"> (the old slide with .vmscale axis diagram)
Extract: <section data-label="Deployment Tiers"> (the new slide with commercial packaging text)

Step 2: Create a merged slide called "Deployment Tiers Matrix":
- data-label="Deployment Tiers Matrix"
- data-screen-label="Deployment Tiers Matrix — Hosting Models and Deployment Scale"
- class="scaling" (same as other slides)
- .head div: cat-tag "Multi-Entity Consolidation", eyebrow "Multi-Entity Consolidation", h2 "Deployment Tiers Matrix", crumb: one precise sentence about why hosting model × deployment tier selection matters for a property-management operator.

LAYOUT (use ONLY: ${CSS_AVAIL}):
Use .cmp2 two-column with .cmp-div:
Left .cmp-col: The .vmscale axis visual from "Scaling — Tiers" (retain .vmstep markers showing scale progression)
Right .cmp-col: Commercial packaging comparison from "Deployment Tiers" (SaaS/PaaS/IaaS × Managed/Hosted/On-Prem/Self-Hosted) formatted as .cmp-rules or comparison rows

Design tokens: ${TOKENS}
${BCSC}
${DUAL}
Banned: ${BANNED}

Include .notes-strip with 3 SPECIFIC questions about deployment tier and hosting model decision criteria.

Step 3: Write complete <section>...</section> HTML to:
"${SCR}/merge/deployment-tiers-matrix.html"
Write ONLY the HTML. Return: confirmation.`,
    { model: 'opus', label: 'wpe-merge', phase: 'Merge & Reorder' }
  ),

])

log('All 26 WP agents complete. Building final slide order...')

// ══════════════════════════════════════════════════
// PHASE 3: SLIDE ORDER MANIFEST
// ══════════════════════════════════════════════════
phase('Merge & Reorder')

const finalOrder = []
for (const label of ORDER.nav) {
  finalOrder.push(label)
}
for (const cat of ['01','02','03','04','05','06','07']) {
  finalOrder.push('__DIVIDER_' + cat + '__')
  for (const label of ORDER[cat]) {
    finalOrder.push(label)
  }
}

log('Final order: ' + finalOrder.length + ' entries (content slides + 7 dividers)')

await agent(
  'Write this JSON to "' + SCR + '/jw6-slide-order.json":\n' + JSON.stringify(finalOrder, null, 2),
  { model: 'haiku', label: 'write-order', phase: 'Merge & Reorder' }
)

// ══════════════════════════════════════════════════
// PHASE 4: ASSEMBLY
// ══════════════════════════════════════════════════
phase('Assembly')
log('Phase 4: Assembling JW6.html...')

const renamedStr = JSON.stringify(RENAMED, null, 2)
const dividersStr = JSON.stringify(DIVIDERS, null, 2)
const finalOrderStr = JSON.stringify(finalOrder, null, 2)

await agent(
  `Assemble the complete JW6 PointSav slide deck from all sources.

════ STEP 1: Read source files ════
Read the CSS header: "${SCR}/css-header.html"
Read the HTML footer: "${SCR}/html-footer.html"

If either file is missing or empty, extract from JW5 directly:
- CSS header = everything in "${JW5}" from line 1 through the line BEFORE the first <section> element
- HTML footer = everything AFTER the last </section> tag

════ STEP 2: List revised slides available ════
Run:
ls "${SCR}/visual/" 2>/dev/null
ls "${SCR}/content/" 2>/dev/null
ls "${SCR}/language/" 2>/dev/null
ls "${SCR}/merge/" 2>/dev/null

This tells you which slugs have been improved. Slug = data-label lowercased, non-alphanumeric→hyphens, trimmed.
Examples: "Interface OS"→interface-os  "WORM Record Format"→worm-record-format  "Solid / Pods"→solid-pods  "Scaling — Composition"→scaling-composition  "seL4 Kernel"→sel4-kernel

════ STEP 3: Process each slide in FINAL ORDER ════
${finalOrderStr}

For each entry:

If "__DIVIDER_NN__": Extract the category divider section from JW5.
  Category divider data-labels in JW5: ${dividersStr}
  Find <section data-label="[divider label]"> in JW5 and use it verbatim.

If a content slide label:
  PRIORITY: visual/ > content/ > language/ > JW5 original
  1. Check if "${SCR}/visual/[SLUG].html" exists — if yes, read and use it
  2. Else check "${SCR}/content/[SLUG].html" — if yes, read and use it
  3. Else check "${SCR}/language/[SLUG].html" — if yes, read and use it
  4. Else extract <section data-label="[original-label]"> from JW5

RENAMED SLIDES (look for OLD label in JW5, output with NEW data-label):
${renamedStr}
When extracting from JW5 under an old label, replace data-label="old" with data-label="new" in the output HTML.

SPECIAL CASES:
- "Deployment Tiers Matrix": read from "${SCR}/merge/deployment-tiers-matrix.html"
- "Scaling — Tiers" is NOT in the final order (merged away) — skip if encountered
- "Deployment Tiers" is NOT in the final order (merged away) — skip if encountered
- "Console Search": if found in language/ use it; else extract from JW5 (original was in Cat 04 but now placed in Cat 02 order — the slide content is unchanged, position is new)

════ STEP 4: Assemble and write ════
Concatenate exactly:
[contents of css-header.html]
[newline]
[all section HTML blocks in final order, each separated by a newline]
[newline]
[contents of html-footer.html]

Write the result to: "${JW6}"

CRITICAL — MUST NOT:
- Modify any JavaScript in deck-stage.js
- Change any CSS custom property value
- Add any new CSS class definitions
- Alter any data-label or data-screen-label beyond the explicit renames above

════ STEP 5: Report ════
After writing, run:
grep -c '<section' "${JW6}"
wc -c < "${JW6}"
Return: section count and file size in bytes.`,
  { model: 'opus', label: 'assemble-jw6', phase: 'Assembly' }
)

log('Assembly complete. Running verification...')

// ══════════════════════════════════════════════════
// PHASE 5: VERIFICATION
// ══════════════════════════════════════════════════
phase('Verification')

const verifyResult = await agent(
  `Verify the assembled JW6 PointSav deck. Run each check with Bash. Report PASS or FAIL for each.

FILE: "${JW6}"

CHECK 1 — Section count (expect >= 72):
grep -c '<section' "${JW6}"

CHECK 2 — Merge: old slides absent, new present:
echo "Scaling-Tiers removed:"; grep -c 'data-label="Scaling' "${JW6}" || echo 0
echo "Deployment Tiers Matrix present:"; grep -c '"Deployment Tiers Matrix"' "${JW6}"
EXPECT: first=0 (or only matching "Scaling — Composition" which is kept), second>=1

CHECK 3 — Cat 02 reorder (Interface OS before Input Machine):
grep -n 'data-label="Interface OS"\\|data-label="Input Machine"' "${JW6}" | head -4
EXPECT: Interface OS line < Input Machine line

CHECK 4 — Cat-tag audit (no numeric or ad-hoc values):
grep 'class="cat-tag">' "${JW6}" | grep -vE 'Developer Platform|Operator Workspace|System of Record|Integration|Machine-Based|Multi-Entity|Platform Foundation|Platform Overview' | wc -l
EXPECT: 0

CHECK 5 — Notes-strip coverage:
grep -c 'class="notes-strip"' "${JW6}"
EXPECT: >= 58

CHECK 6 — BCSC posture (seL4 always with planned/intended):
grep -i 'sel4' "${JW6}" | grep -iv 'planned\\|intended' | wc -l
EXPECT: 0

CHECK 7 — Banned vocabulary:
grep -ic 'revolutionary\\|cutting-edge\\|game-changer' "${JW6}"
EXPECT: 0

CHECK 8 — deck-stage.js preserved:
grep -c 'class DeckStage' "${JW6}"
EXPECT: >= 1

CHECK 9 — File size:
wc -c < "${JW6}"
EXPECT: > 250000

For each check: show actual output and state PASS or FAIL. If any FAIL, describe specifically what to fix.`,
  { model: 'sonnet', label: 'verify-jw6', phase: 'Verification' }
)

log('Verification done.')

// ══════════════════════════════════════════════════
// PHASE 6: COMMIT
// ══════════════════════════════════════════════════
phase('Commit')
log('Phase 6: Committing JW6 and research files...')

await agent(
  `Commit the JW6 slide deck to project-orgcharts. Run in order:

1. cd "${BASE}" && git status

2. Stage specific files ONLY (never git add . or -A):
   cd "${BASE}" && git add \\
     "inputs/IT SUPPORT_PointSav_2026_06_14_Design Slides_JW6/" \\
     "inputs/jw6-scratch/" \\
     "inputs/datagraph-pointsav/research-competitive-positioning.yaml" \\
     "inputs/datagraph-pointsav/research-customer-tam.yaml" \\
     "inputs/datagraph-pointsav/research-regulatory-timelines.yaml"

3. Commit using commit-as-next.sh (NEVER git commit directly — it is blocked by pre-commit gate):
   cd "${BASE}" && ~/Foundry/bin/commit-as-next.sh "feat(slides): JW6 — 10x quality pass (visual enrichment, language consistency, reorder, research augments)"

4. Verify:
   cd "${BASE}" && git log --oneline -3

5. Prepend outbox message: Read "${BASE}/.agent/outbox.md" to get current content.
   Prepend the following message block IMMEDIATELY after the YAML frontmatter header (the first --- block ending with ---), before any existing messages:

---
from: totebox@project-orgcharts
to: command@claude-code
re: Stage 6 READY — JW6 overnight quality pass committed
created: 2026-06-15T00:00:00Z
status: pending
priority: high
msg-id: project-orgcharts-20260615-stage6-jw6
---

JW6 overnight 10x quality pass complete.

Changes vs JW5 (94256550):
- Visual enrichment: 10 text-heavy slides now have CSS diagrams
- Content augments: 5 existing slides enriched from V5 source (GIS Engine, Pairing as Permissions, Design System, Bookkeeping, Interface OS)
- Language pass: all slides Bloomberg-standard, BCSC posture, dual-audience
- Reorder: Cat 02 (Interface OS first, Console Search added), Cat 03 (VM vs Containers first, WORM after Archive, Chart-of-Accounts + Partition after Bookkeeping), Cat 04 (Totebox Services renamed x2, Console Search moved to Cat 02), Cat 06 (Scaling—Tiers + Deployment Tiers merged into Deployment Tiers Matrix)
- Cat-tag: all canonical
- Notes-strip: all content slides with specific questions
- Research expansion: 3 new datagraph YAML files (competitive-positioning, customer-tam, regulatory-timelines)

Stage 6 pending for this commit + all prior pending commits.

---

   Write the updated content back to "${BASE}/.agent/outbox.md"

6. Stage and commit the outbox:
   cd "${BASE}" && git add .agent/outbox.md && ~/Foundry/bin/commit-as-next.sh "ops(outbox): Stage 6 READY for JW6 overnight quality pass"

7. Show final log:
   cd "${BASE}" && git log --oneline -5

Return the two commit hashes.`,
  { model: 'sonnet', label: 'commit-jw6', phase: 'Commit' }
)

log('JW6 overnight run COMPLETE.')

return {
  status: 'complete',
  final_order_length: finalOrder.length,
  visual_enrichment_targets: WPA.length,
  content_augment_targets: WPB.length,
  language_pass_categories: WPC.length,
  research_files: WPD.length,
  verification: verifyResult,
}
