export const meta = {
  name: 'jw7-mohammad-feedback',
  description: 'JW7: Apply 13 approved Mohammad discovery call recommendations to JW6 slides',
  phases: [
    { title: 'Patch Slides' },
    { title: 'New Slides' },
    { title: 'Assembly' },
    { title: 'Verification' },
    { title: 'Commit' },
  ],
}

// ── PATHS ──
const BASE = '/srv/foundry/clones/project-orgcharts'
const JW6  = BASE + '/inputs/IT SUPPORT_PointSav_2026_06_14_Design Slides_JW6/IT SUPPORT_PointSav_2026_06_14_Design Slides_JW6.html'
const JW7D = BASE + '/inputs/IT SUPPORT_PointSav_2026_06_14_Design Slides_JW7'
const JW7  = JW7D + '/IT SUPPORT_PointSav_2026_06_14_Design Slides_JW7.html'
const SCR  = BASE + '/inputs/jw7-scratch'

// ── SHARED RULES ──
const CSS   = '.cmp2 .cmp-col .cmp-head .cmp-item .cmp-rule .cmp-sub .cmp-rules .cmp-div .slab .vc-cyl .vc-stack .db .doc .arch .future-callout .canvas .wire .node .totebox .vbar .badge .backend .vmscale .vmstep .para .hexagon .diamond .cloud .cube .adminwin .inpanel .tbsvc-ex .vc-verdict .arch-compare .vs-sep .svc-ladder .rung .rn .arch-card'
const TOKENS = '--blue:#0E7C84  --green:#54924E  --orange:#F15F22  --ink-100 (darkest) through --ink-600 (lightest)  --paper-100/200/300  --highlight-50:#FDE8DD'
const BCSC   = 'seL4 and Sovereign Data Foundation: PLANNED/INTENDED only — never shipped.'
const DUAL   = 'Every word works for both: 28-year-old Rust developer AND 55-year-old pension fund manager.'
const BANNED = '"revolutionary" "cutting-edge" "AI-powered" "innovative" "seamless" "robust" "leverage" (verb) "unlock" "empower" "journey" "solution" (noun)'

// ── FINAL SLIDE ORDER ──
// Nav: Cover → Contents → How We Select Contributors → Plain Language Standard (R-14 NEW) → Icon Index
// Cat 01: Contributors → Design System → Color Values → Typographic Protocols → Naming Conventions
//         → Repositories → Use Cases → SMB Environment → Deployment Scenarios
//         → Licensing Model → Documentation Standards → Coding Guidelines → Testing & QA (R-09 NEW)
// Cat 02–07: unchanged from JW6

const NAV_ORDER = ['Cover', 'Contents', 'How We Select Contributors', 'Plain Language Standard', 'Icon Index']
const CAT01_ORDER = [
  'Contributors', 'Design System', 'Color Values', 'Typographic Protocols', 'Naming Conventions',
  'Repositories', 'Use Cases', 'SMB Environment', 'Deployment Scenarios',
  'Licensing Model', 'Documentation Standards', 'Coding Guidelines', 'Testing & QA',
]
const CAT02_ORDER = ['Interface OS','Input Machine','System Administration','Command Center','Interfaces','BIM Console','Maps Console','Building Preferences','Console Search']
const CAT03_ORDER = ['VM vs Containers','Toteboxes','Totebox Archive','WORM Record Format','Bookkeeping','Chart-of-Accounts Names','Partition Layout','IoT Data','Geo-spatial']
const CAT04_ORDER = ['Totebox Service Architecture','Totebox Service Catalog','Products & Services','Economic Model','Communications','Solid / Pods','GIS Catchment','Docs Navigation']
const CAT05_ORDER = ['Private Network','Pairing as Permissions','Authentication','Registry Token']
const CAT06_ORDER = ['Multiple Archives','Deployment Tiers Matrix','Scaling — Composition','Three-Layer Stack','Owner / Operator']
const CAT07_ORDER = ['On-prem','Leased Servers','Public Cloud','Hybrid Network','GIS Engine','OS Matrix','seL4 Kernel','GIS Stack','Cluster Method','Terminal Rendering','Co-location Thesis']

const DIVIDERS = {
  '01': '01 Developer Platform',
  '02': '02 Operator Workspace',
  '03': '03 System of Record',
  '04': '04 Integration & Data Portability',
  '05': '05 Machine-Based Authorization',
  '06': '06 Multi-Entity Consolidation',
  '07': '07 Platform Foundation',
}

function slug(s) {
  return s.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '')
}

log('Phase 1: Setting up scratch directories...')
await agent(
  'mkdir -p "' + SCR + '" "' + JW7D + '" && echo done',
  { model: 'haiku', label: 'setup', phase: 'Patch Slides' }
)

// ══════════════════════════════════════════════════════
// PHASE 1: PATCH EXISTING SLIDES (8 agents in parallel)
// ══════════════════════════════════════════════════════
phase('Patch Slides')
log('Patching 8 existing slides with Mohammad feedback...')

await parallel([

  // ── R-01 + R-02 + R-11 + R-17(part 1): Contributors ──
  () => agent(`Apply 4 approved changes to the "Contributors" slide.

Step 1: Read "${JW6}" (full file). Extract <section data-label="Contributors">...</section>.

Apply ALL four changes to this ONE slide:

CHANGE 1 (R-01) — Add "What We're NOT Looking For" block:
Below the existing content, add a new .cmp-rules section with heading "What We're NOT Looking For"
using a small h4 or .cmp-sub label. Add 4 .cmp-rule items:
- "Not looking for agencies — the work must be personal, not delegated"
- "Not looking for solution vendors — we build the platform; we do not procure it"
- "Not looking for passive capital — Contributors hold equity in exchange for active contribution"
- "Not looking for companies in active M&A — timing and scope incompatible with a 6-call process"

CHANGE 2 (R-02) — Add "What to Bring" checklist panel:
In the right column (or below the existing columns if no right column), add a "What to Bring" section
as a .cmp-rules block with 8 .cmp-rule items. Use the .rk "✦" marker:
Website Home Page · Marketing Kit · Leadership Bios · Professional Credentials ·
Project Fact Sheet · Existing Case Studies · Company Information · Financial Overviews

CHANGE 3 (R-11) — Add M&A and Design Collaboration as discovery criteria:
In the existing contributor criteria section, add two new .cmp-rule bullets:
- "M&A and Material Change Reports — active M&A defers the engagement timing"
- "Design Collaboration — a formal contribution track alongside technical development"
(Add these where other contribution types/tracks are listed)

CHANGE 4 (R-17) — Update notes-strip questions:
Find the .notes-strip div. Replace the existing questions with these three SPECIFIC questions:
1. "What should I bring to the first call?"
2. "What disqualifies a Contributor?"
3. "What do I have at the end of the 6-call series?"

RULES:
- Only existing CSS classes: ${CSS}
- Design tokens: ${TOKENS}
- ${BCSC}
- ${DUAL}
- Banned: ${BANNED}
- Do NOT change data-label, data-screen-label, eyebrow, h2 heading
- Do NOT remove any existing content — ADD to it

Step 2: Write the COMPLETE revised <section>...</section> to:
"${SCR}/contributors.html"
Write ONLY the HTML. Return: one-sentence confirmation.`,
  { model: 'opus', label: 'patch-contributors', phase: 'Patch Slides' }),

  // ── R-03 + R-17(part 2): How We Select Contributors ──
  () => agent(`Apply 2 approved changes to the "How We Select Contributors" slide.

Step 1: Read "${JW6}" (full file). Extract <section data-label="How We Select Contributors">...</section>.

CHANGE 1 (R-03) — Add callout stating what the discovery series produces:
Add a .future-callout div with:
- .fc-kick label: "What You Walk Away With"
- Body: "At the end of 6 calls, the Contributor produces the White Paper — a scoped document
  defining the contribution domain, access level, and engagement terms."

CHANGE 2 (R-17) — Update notes-strip questions:
Find the .notes-strip div. Replace existing questions with:
1. "What should I bring to the first call?"
2. "What disqualifies a Contributor?"
3. "What do I have at the end of the 6-call series?"

RULES:
- Only existing CSS: ${CSS}
- ${BCSC} · ${DUAL} · Banned: ${BANNED}
- Do NOT change data-label, eyebrow, h2, crumb

Step 2: Write complete <section>...</section> to "${SCR}/how-we-select-contributors.html"
Return: one-sentence confirmation.`,
  { model: 'opus', label: 'patch-how-we-select', phase: 'Patch Slides' }),

  // ── R-04: Licensing Model ──
  () => agent(`Apply approved change to the "Licensing Model" slide.

Step 1: Read "${JW6}". Extract <section data-label="Licensing Model">...</section>.

CHANGE (R-04) — Add product-to-license mapping table:
Add a two-column product table using .arch-compare or .cmp2 structure.
For each product show: Product Name | License Type (as .vc-verdict chip)

Open-source products (use class="vc-verdict good"):
- Lightbox OS — Open
- Workspace OS — Open

Proprietary products (use class="vc-verdict warn"):
- Command Centre API — Proprietary
- Data Engine — Proprietary
- Data Marketplace — Proprietary
- BIM Server — Proprietary
- Building ID Server — Proprietary

Add a small label above the table: "License by product"
Format as a flex grid or .cmp2 with product name left, chip right.

RULES: Only existing CSS: ${CSS} · Tokens: ${TOKENS} · ${DUAL} · Banned: ${BANNED}

Step 2: Write complete <section>...</section> to "${SCR}/licensing-model.html"
Return: one-sentence confirmation.`,
  { model: 'opus', label: 'patch-licensing', phase: 'Patch Slides' }),

  // ── R-05: Documentation Standards ──
  () => agent(`Apply approved change to the "Documentation Standards" slide.

Step 1: Read "${JW6}". Extract <section data-label="Documentation Standards">...</section>.

CHANGE (R-05) — Present all 6 standards as an explicit numbered list:
Locate where the documentation standards are referenced. Replace or supplement with a
clearly enumerated numbered list using .cmp-rules. Each .cmp-rule should have:
- The .rk marker showing the number (1. 2. 3. etc.)
- The .rt text in format: "Standard Name — one-line description"

The 6 standards in order:
1. Wireframes — screen blueprints showing layout before visual design
2. UML 2.5.1 — component and sequence diagrams for system relationships
3. 4+1 View Model — logical, process, physical, development, and scenario perspectives
4. C4 Model — context, container, component, and code hierarchy diagrams
5. ISO/IEC 42010 — architecture description standard (the formal conformance framework)
6. User Guide / Wiki / Help — human-readable operational reference documentation

RULES: Only existing CSS: ${CSS} · ${DUAL} · Banned: ${BANNED}

Step 2: Write complete <section>...</section> to "${SCR}/documentation-standards.html"
Return: confirmation.`,
  { model: 'opus', label: 'patch-documentation-standards', phase: 'Patch Slides' }),

  // ── R-06: Design System ──
  () => agent(`Apply approved change to the "Design System" slide.

Step 1: Read "${JW6}". Extract <section data-label="Design System">...</section>.

CHANGE (R-06) — Add 3-tier repository structure diagram:
Add a .vc-stack (column of stacked elements) showing the three layers:

Layer 1 (top): Design System Repository
  - A .slab div (background: var(--paper-200)) with label "Design System Repository — PrivateGit"
  - Subtext: "versioned components, tokens, media assets"

Layer 2 (middle): Display Environment
  - A .slab div (background: rgba(14,124,132,0.08)) with label "Media Kit OS / Workspace OS"
  - Subtext: "display environment for platform surfaces"

Layer 3 (bottom): Component Apps
  - A .slab div (background: var(--paper-300)) with label "Component Apps"
  - Subtext: "app-layer surfaces consuming the design system"

Connect layers visually with a small down-arrow (↓) or border-bottom accent between slabs.
Width: ~320px, center-aligned or left-aligned in the content area.

RULES: Only existing CSS: ${CSS} · Tokens: ${TOKENS} · ${DUAL} · Banned: ${BANNED}
Do NOT change data-label, eyebrow, h2, crumb.

Step 2: Write complete <section>...</section> to "${SCR}/design-system.html"
Return: confirmation.`,
  { model: 'opus', label: 'patch-design-system', phase: 'Patch Slides' }),

  // ── R-07: Repositories ──
  () => agent(`Apply approved change to the "Repositories" slide.

Step 1: Read "${JW6}". Extract <section data-label="Repositories">...</section>.

CHANGE (R-07) — Add phase/tier label to each repository group:
The slide lists repositories. Add a phase badge or label to each repository (or group of repos):

S-Phase (Vendor/Source tier) — class="badge" style="background:var(--blue);color:#fff" "S":
  - pointsav/* repos (pointsav-monorepo, pointsav-design-system, etc.)

C-Phase (Customer tier) — class="badge" style="background:var(--green);color:#fff" "C":
  - woodfine/* repos (woodfine-fleet-deployment, woodfine-media-assets, etc.)

D-Phase (Deployment instances) — class="badge" style="background:var(--orange);color:#fff" "D":
  - ~/Foundry/deployments/* (local-only, gitignored)

Add a brief legend row above the repository list:
Three inline .badge spans: "S — Vendor source" | "C — Customer" | "D — Deployments"

RULES: Only existing CSS: ${CSS} · Tokens: ${TOKENS} · ${DUAL} · Banned: ${BANNED}

Step 2: Write complete <section>...</section> to "${SCR}/repositories.html"
Return: confirmation.`,
  { model: 'opus', label: 'patch-repositories', phase: 'Patch Slides' }),

  // ── R-08: Coding Guidelines ──
  () => agent(`Apply approved change to the "Coding Guidelines" slide.

Step 1: Read "${JW6}". Extract <section data-label="Coding Guidelines">...</section>.

CHANGE (R-08) — Add cross-platform scope line:
At the very top of the content area (before the first bullet or rule), add a .cmp-sub or
.para element containing:
"Scope: Rust · WASM · web surfaces. Platform-specific extensions (iOS, Android) are additive —
these rules are the primary standard across all environments."

Style: background: var(--paper-200); border-left: 3px solid var(--blue); padding: 8px 14px;
margin-bottom: 20px; font-size: 13px;

RULES: Only existing CSS: ${CSS} · Tokens: ${TOKENS} · ${DUAL} · Banned: ${BANNED}

Step 2: Write complete <section>...</section> to "${SCR}/coding-guidelines.html"
Return: confirmation.`,
  { model: 'opus', label: 'patch-coding-guidelines', phase: 'Patch Slides' }),

  // ── R-12: Use Cases ──
  () => agent(`Apply approved change to the "Use Cases" slide.

Step 1: Read "${JW6}". Extract <section data-label="Use Cases">...</section>.

CHANGE (R-12) — Ensure every diagram zone has an explicit label and definition:
Examine the slide content. For every visual zone or .node element:
- Add a visible text label if not present
- Add a one-line definition phrase below the label (in a .cmp-sub or small tag)

The three Use Case zones should be explicitly named (use the platform's established names):
- "Business Administration" — managing entity records, operators, and contracts
- "Record Keeping" — WORM document archives, chart-of-accounts, audit trail
- "Cyber-physical" — IoT sensors, geo-spatial data, building systems integration

If the slide already has these labels, ensure each has a one-line definition beneath it.
If using .canvas layout, add .cmp-sub beneath each .node.

RULES: Only existing CSS: ${CSS} · Tokens: ${TOKENS} · ${DUAL} · Banned: ${BANNED}

Step 2: Write complete <section>...</section> to "${SCR}/use-cases.html"
Return: confirmation.`,
  { model: 'opus', label: 'patch-use-cases', phase: 'Patch Slides' }),

])

// ══════════════════════════════════════════════════════
// PHASE 2: NEW SLIDES (R-09 + R-14)
// ══════════════════════════════════════════════════════
phase('New Slides')
log('Creating 2 new slides: Testing & QA (R-09) and Plain Language Standard (R-14)...')

await parallel([

  // ── R-09: NEW SLIDE — Testing & QA ──
  () => agent(`Create a new PointSav design slide: "Testing & QA".

This slide DOES NOT exist in JW6. Create it from scratch.

SLIDE SPEC:
- data-label="Testing & QA"
- data-screen-label="Developer Platform — Testing, QA, and Compliance Verification"
- class="scaling" (same class as all content slides)
- cat-tag: "Developer Platform"
- eyebrow: "Developer Platform"
- h2: "Testing & QA"
- crumb: "Every commit to the canonical ledger passes three verification layers before Stage 6
  promotion — developer-side testing, integration coverage, and compliance verification that
  the WORM audit trail is intact."

LAYOUT — .cmp2 two-column with .cmp-div divider:

LEFT COLUMN — "Developer Layer" (.cmp-head with .ch-t "Developer Layer"):
.cmp-rules with 3 .cmp-rule items:
- .rk "1" .rt "Unit tests — Rust cargo test; every public function has a corresponding test"
- .rk "2" .rt "Integration tests — end-to-end API calls; real filesystem, no mocks"
- .rk "3" .rt "Clippy gate — cargo clippy --all-targets -D warnings; zero allowed warnings"

RIGHT COLUMN — "Compliance Layer" (.cmp-head with .ch-t "Compliance Layer"):
.cmp-rules with 3 .cmp-rule items:
- .rk "✲" .rt "WORM integrity — every WORM record verified append-only before Stage 6 promotion"
- .rk "✲" .rt "Audit trail — chain of custody confirmed across contributor boundary crossings"
- .rk "✲" .rt "Stage 6 gate — Command Session promotion only after all checks pass"

NOTES-STRIP with 3 specific questions:
1. "Which layer would fail first in your current stack, and why?"
2. "Does your company already run clippy in CI, or would this be new discipline?"
3. "What does your team currently do for WORM or append-only record verification?"

Use ONLY existing CSS classes: ${CSS}
Design tokens: ${TOKENS}
${BCSC} · ${DUAL} · Banned: ${BANNED}

Write the COMPLETE <section>...</section> HTML to: "${SCR}/testing-and-qa.html"
Write ONLY the HTML. Return: confirmation.`,
  { model: 'opus', label: 'new-testing-qa', phase: 'New Slides' }),

  // ── R-14: NEW SLIDE — Plain Language Standard ──
  () => agent(`Create a new PointSav design slide: "Plain Language Standard".

This slide DOES NOT exist in JW6. Create it from scratch.

CONTEXT: During the October 2025 discovery call, Mohammad was shown Berkshire Hathaway and
SEC plain-language financial disclosures as a writing register benchmark. This slide
codifies that standard for all PointSav documentation and slide content.

SLIDE SPEC:
- data-label="Plain Language Standard"
- data-screen-label="Platform Overview — Writing Register and Plain Language Standard"
- class="scaling"
- cat-tag: "Platform Overview"
- eyebrow: "Platform Overview"
- h2: "Plain Language Standard"
- crumb: "Every sentence in this deck — and in every document the platform produces — holds to
  a single writing standard: as precise and as plain as a well-drafted SEC filing."

LAYOUT:
Use a .cmp2 two-column structure:

LEFT COLUMN (.cmp-head "The Standard"):
A blockquote-style .para element quoting Warren Buffett's 1997 Berkshire Hathaway letter
preface (his plain-English writing standard). Use this specific well-known passage:
"When writing Berkshire Hathaway's annual report, I pretend that I'm talking to my sisters.
They are smart women, but they are not accountants. I don't need to impress them;
I just need them to understand what I'm saying."
— Warren Buffett, 1997 Berkshire Hathaway Annual Report

Style the quote in italics with a left border (border-left: 3px solid var(--blue); padding-left: 16px;)

Below the quote, add a .cmp-sub: "Source: SEC Plain Language Handbook, 1998 (third printing,
adapted from Berkshire Hathaway shareholder letters)"

RIGHT COLUMN (.cmp-head "Applied Here"):
.cmp-rules with 3 .cmp-rule items:
- .rk "✦" .rt "Active voice — 'the system writes the record' not 'the record is written by the system'"
- .rk "✦" .rt "Precise nouns — 'WORM archive' not 'solution', 'Totebox' not 'platform instance'"
- .rk "✦" .rt "No marketing vocabulary — the banned list applies to every artifact, every slide, every commit message"

NOTES-STRIP with 3 specific questions:
1. "Which sentence on this slide would you rewrite first, and how?"
2. "Does your company have a house style guide, and where does it conflict with this standard?"
3. "Name one piece of documentation in your organization that fails this test."

Use ONLY existing CSS: ${CSS}
Tokens: ${TOKENS} · ${BCSC} · ${DUAL} · Banned: ${BANNED}

Write COMPLETE <section>...</section> to: "${SCR}/plain-language-standard.html"
Write ONLY the HTML. Return: confirmation.`,
  { model: 'opus', label: 'new-plain-language', phase: 'New Slides' }),

])

// ══════════════════════════════════════════════════════
// PHASE 3: ASSEMBLY
// ══════════════════════════════════════════════════════
phase('Assembly')
log('Assembling JW7.html from JW6 with patched + new slides...')

const navStr    = JSON.stringify(NAV_ORDER)
const cat01Str  = JSON.stringify(CAT01_ORDER)
const cat02Str  = JSON.stringify(CAT02_ORDER)
const cat03Str  = JSON.stringify(CAT03_ORDER)
const cat04Str  = JSON.stringify(CAT04_ORDER)
const cat05Str  = JSON.stringify(CAT05_ORDER)
const cat06Str  = JSON.stringify(CAT06_ORDER)
const cat07Str  = JSON.stringify(CAT07_ORDER)
const divStr    = JSON.stringify(DIVIDERS)

await agent(
  `Assemble the PointSav JW7 slide deck.

SOURCE: "${JW6}" — read the full file using the Read tool.
SCRATCH PATCHES: "${SCR}/" — files here REPLACE their JW6 equivalent
OUTPUT: "${JW7}"

════ STEP 1: Extract CSS header and footer ════
CSS header = everything from line 1 through the line before the first <section> element.
Footer = everything after the final </section> tag.

════ STEP 2: Patches available in scratch dir ════
Check which files exist in "${SCR}/":
ls "${SCR}/"

Each file slug maps to a data-label:
contributors.html → "Contributors"
how-we-select-contributors.html → "How We Select Contributors"
licensing-model.html → "Licensing Model"
documentation-standards.html → "Documentation Standards"
design-system.html → "Design System"
repositories.html → "Repositories"
coding-guidelines.html → "Coding Guidelines"
use-cases.html → "Use Cases"
testing-and-qa.html → "Testing & QA"  ← NEW SLIDE (not in JW6)
plain-language-standard.html → "Plain Language Standard"  ← NEW SLIDE (not in JW6)

For patched slides: use the scratch version.
For all other slides: extract from JW6 using data-label.

════ STEP 3: Build in FINAL ORDER ════
Nav slides: ${navStr}
Then: <section data-label="01 Developer Platform"> (Cat 01 divider)
Cat 01 slides: ${cat01Str}
Then: <section data-label="02 Operator Workspace"> (Cat 02 divider)
Cat 02 slides: ${cat02Str}
Then: <section data-label="03 System of Record"> (Cat 03 divider)
Cat 03 slides: ${cat03Str}
Then: <section data-label="04 Integration & Data Portability"> (Cat 04 divider)
Cat 04 slides: ${cat04Str}
Then: <section data-label="05 Machine-Based Authorization"> (Cat 05 divider)
Cat 05 slides: ${cat05Str}
Then: <section data-label="06 Multi-Entity Consolidation"> (Cat 06 divider)
Cat 06 slides: ${cat06Str}
Then: <section data-label="07 Platform Foundation"> (Cat 07 divider)
Cat 07 slides: ${cat07Str}

DIVIDER DATA-LABELS: ${divStr}

For nav slides "Cover", "Contents", "Icon Index" — extract from JW6 verbatim.

SPECIAL CASES:
- "Plain Language Standard" → read from "${SCR}/plain-language-standard.html" (not in JW6)
- "Testing & QA" → read from "${SCR}/testing-and-qa.html" (not in JW6)
- "Solid / Pods" in JW6 may be "Solid / Pods" — search with both slash formats if needed

════ STEP 4: Write output ════
Concatenate: [CSS header] + newline + [all sections in order] + newline + [footer]
Write to: "${JW7}"

════ STEP 5: Verify and report ════
grep -c '<section' "${JW7}"
wc -c < "${JW7}"
Return: section count and file size.`,
  { model: 'opus', label: 'assemble-jw7', phase: 'Assembly' }
)

// ══════════════════════════════════════════════════════
// PHASE 4: VERIFICATION
// ══════════════════════════════════════════════════════
phase('Verification')
log('Verifying JW7...')

const verifyResult = await agent(
  `Verify the assembled JW7 PointSav deck at "${JW7}".

CHECK 1 — Section count (expect >= 74 — JW6 had 72 + 2 new slides):
grep -c '<section' "${JW7}"

CHECK 2 — New slides present:
echo "Plain Language Standard:"; grep -c '"Plain Language Standard"' "${JW7}"
echo "Testing & QA:"; grep -c '"Testing & QA"' "${JW7}"
EXPECT: both >= 1

CHECK 3 — Plain Language Standard in Nav (before Cat 01 divider):
grep -n '"Plain Language Standard"\\|"01 Developer Platform"' "${JW7}" | head -4
EXPECT: Plain Language Standard line < 01 Developer Platform line

CHECK 4 — Testing & QA after Coding Guidelines (both in Cat 01):
grep -n '"Coding Guidelines"\\|"Testing & QA"' "${JW7}" | head -4
EXPECT: Coding Guidelines line < Testing & QA line

CHECK 5 — Cat-tag audit (no ad-hoc values):
grep 'class="cat-tag">' "${JW7}" | grep -vE 'Developer Platform|Operator Workspace|System of Record|Integration|Machine-Based|Multi-Entity|Platform Foundation|Platform Overview' | wc -l
EXPECT: 0

CHECK 6 — BCSC posture:
grep -i 'sel4' "${JW7}" | grep -iv 'planned\\|intended\\|klein\\|sosp\\|acm\\|arxiv\\|darpa\\|provers\\|formally.verified\\|lionsOS\\|hacms\\|verified mitig' | wc -l
EXPECT: 0

CHECK 7 — Banned vocabulary:
grep -ic 'revolutionary\\|cutting-edge\\|game-changer\\|seamless' "${JW7}"
EXPECT: 0

CHECK 8 — deck-stage.js preserved:
grep -c 'class DeckStage' "${JW7}"
EXPECT: >= 1

CHECK 9 — File size:
wc -c < "${JW7}"
EXPECT: > 300000

CHECK 10 — Notes-strip on new slides:
grep -A5 '"Plain Language Standard"' "${JW7}" | grep -c 'notes-strip'
grep -A5 '"Testing & QA"' "${JW7}" | grep -c 'notes-strip'
EXPECT: both >= 1 (within their sections)

For each check: show actual result and PASS/FAIL. If any FAIL, describe specifically what broke.`,
  { model: 'sonnet', label: 'verify-jw7', phase: 'Verification' }
)

log('Verification complete.')

// ══════════════════════════════════════════════════════
// PHASE 5: COMMIT
// ══════════════════════════════════════════════════════
phase('Commit')
log('Committing JW7...')

await agent(
  `Commit the JW7 slide deck.

1. cd "${BASE}" && git status

2. Stage specific files only:
cd "${BASE}" && git add \\
  "inputs/IT SUPPORT_PointSav_2026_06_14_Design Slides_JW7/" \\
  "inputs/jw7-scratch/"

3. Commit using commit-as-next.sh (NEVER git commit directly):
cd "${BASE}" && ~/Foundry/bin/commit-as-next.sh "feat(slides): JW7 — 13 Mohammad discovery call feedback items applied (R-01–R-09, R-11, R-12, R-14, R-17)"

4. Verify:
cd "${BASE}" && git log --oneline -3

5. Prepend outbox message to "${BASE}/.agent/outbox.md":
Read the current file, then prepend this message AFTER the yaml frontmatter header (the ---...--- block) and BEFORE any existing messages:

---
from: totebox@project-orgcharts
to: command@claude-code
re: Stage 6 READY — JW7 Mohammad discovery call feedback committed
created: 2026-06-15T00:00:00Z
status: pending
priority: high
msg-id: project-orgcharts-20260615-stage6-jw7
---

JW7 PointSav design slides complete. 13 Mohammad discovery call feedback items applied.

Changes vs JW6:
- Contributors slide: "What We're NOT Looking For" (R-01), discovery documentation checklist (R-02), M&A + Design Collaboration criteria (R-11), notes-strip questions updated (R-17 part 1)
- How We Select Contributors: output callout added — "produces the White Paper" (R-03), notes-strip updated (R-17 part 2)
- Licensing Model: product-to-license mapping table, Open vs Proprietary per product (R-04)
- Documentation Standards: all 6 standards enumerated name-first (R-05)
- Design System: 3-tier repository structure diagram (R-06)
- Repositories: S/C/D phase labels (R-07)
- Coding Guidelines: cross-platform scope line added (R-08)
- Use Cases: all diagram zones explicitly labelled (R-12)
- NEW SLIDE: Testing & QA — Cat 01 after Coding Guidelines (R-09)
- NEW SLIDE: Plain Language Standard — Nav (R-14)

Stage 6 pending: JW7 + all prior pending commits.

---

Write the updated outbox.md back to disk.

6. Stage and commit the outbox:
cd "${BASE}" && git add .agent/outbox.md && ~/Foundry/bin/commit-as-next.sh "ops(outbox): Stage 6 READY for JW7 Mohammad feedback"

7. Show final log:
cd "${BASE}" && git log --oneline -4

Return: commit hashes.`,
  { model: 'sonnet', label: 'commit-jw7', phase: 'Commit' }
)

log('JW7 complete.')

return {
  status: 'complete',
  approved_items: ['R-01','R-02','R-03','R-04','R-05','R-06','R-07','R-08','R-09','R-11','R-12','R-14','R-17'],
  new_slides: ['Testing & QA', 'Plain Language Standard'],
  verification: verifyResult,
}
