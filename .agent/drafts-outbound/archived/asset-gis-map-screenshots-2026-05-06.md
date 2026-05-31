---
schema: foundry-draft-v1
state: asset-capture-pending
language_protocol: ASSET
originating_cluster: project-gis
target_repo: woodfine/woodfine-media-assets
target_path: gis/screenshots/2026-05-06/
audience: internal-design
bcsc_class: internal
authored: 2026-05-06
authored_by: project-gis Task Claude
authored_with: claude-sonnet-4-6
research_done_count: 0
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Asset capture brief only. Screenshots to be taken from live URL by operator.
research_inline: false
notes_for_editor: |
  This is a capture brief, not a finished asset. Operator to take screenshots at
  gis.woodfinegroup.com and file to woodfine-media-assets at the target_path.
  Naming convention: gis-{region}-{feature}-{YYYY-MM-DD}.png
---

# Asset Capture Brief — GIS Map Screenshots (May 6, 2026)

Capture the following screenshots from https://gis.woodfinegroup.com for the media asset archive. These document the state of the platform after the Nordic/UK coverage expansion and the chain search feature launch.

---

## Required Captures

### 1. Norway overview — new T2 clusters
- Switch to EU region
- Zoom to Norway (Bergen / Oslo / Trondheim corridor)
- Toggle to 3km radius
- Expected: amber T2 dots now visible at Obs Coop / Obs Bygg sites
- Filename: `gis-eu-norway-t2-clusters-2026-05-06.png`

### 2. UK overview — B&Q T2 unlocked
- Stay in EU region
- Zoom to UK (London / Midlands / Edinburgh)
- Toggle to 1km radius
- Expected: T2 dots at IKEA / B&Q co-locations
- Filename: `gis-eu-uk-t2-clusters-2026-05-06.png`

### 3. Sweden detail — Bauhaus/IKEA co-location
- EU region, zoom to Stockholm or Gothenburg
- Click a T2 cluster (amber/teal dot pair)
- Expected: bento shows region (Stockholm/Gothenburg), Market Grade T2, IKEA anchor + Bauhaus co-tenant
- Filename: `gis-eu-sweden-bento-t2-2026-05-06.png`

### 4. Chain search active — Costco light-up
- Switch to NA region
- Type "Costco" in the search bar
- Expected: Costco clusters glow amber with ring; rest dim
- Filename: `gis-na-chain-search-costco-2026-05-06.png`

### 5. Chain search active — B&Q light-up (EU)
- Switch to EU region, type "B&Q"
- Expected: B&Q-containing clusters highlighted in UK; rest dim
- Filename: `gis-eu-chain-search-bq-2026-05-06.png`

### 6. Bento box — full hierarchy
- NA region, click any T3 Apex cluster
- Expected: shows Market Region → Market Grade · Score → Anchor → Co-Tenants → Nearby Services → sel-el
- Filename: `gis-na-bento-full-hierarchy-2026-05-06.png`

---

## Dimensions

All screenshots: 1440 × 900 px (desktop viewport). Panel visible on left. Map fills remainder.

For the bento captures (3, 6): ensure the panel is open and all sections visible. Scroll the panel if necessary to capture the full hierarchy.
