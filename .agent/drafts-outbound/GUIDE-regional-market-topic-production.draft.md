---
schema: foundry-draft-v1
artifact_type: GUIDE
language_protocol: GUIDE
audience: editorial-team
bcsc_class: no-disclosure-implication
version: "1.0"
date: 2026-05-30
title: "Guide: Producing Regional Market TOPIC Articles"
routes_to: project-editorial
state: language-cleared
language_pass_date: 2026-06-02
language_pass_by: totebox@project-editorial
target_repo: woodfine-fleet-deployment
target_path: gateway-orchestration-gis-1/
target_filename: guide-regional-market-topic-production.md
---

# Guide: Producing Regional Market TOPIC Articles

This guide takes an editor from a raw `score-regional-markets.py` ranking
all the way to a finished TOPIC article ready for dispatch to
`project-editorial`. It assumes you have the GIS Totebox archive checked
out and can run Python 3 from the command line.

Plan on roughly 30–45 minutes per article once you have the workflow in
your hands. The first one will take longer.

---

## Prerequisites

You need:

- A clone of `project-gis` with `app-orchestration-gis/` populated.
- The current scoring output and AEC dataset:
  - `app-orchestration-gis/work/top400-na.json`
  - `app-orchestration-gis/work/top400-eu.json`
  - `app-orchestration-gis/work/DATA-aec-clusters.csv`
  - `app-orchestration-gis/regional-markets.json` (Phase 18+ build)
- Python 3 (any version Foundry currently ships with — no extra
  packages required; only `urllib`, `json`, and `csv` from the
  standard library are used).
- Internet access from the workstation (the Wikipedia REST API
  call requires outbound HTTPS).
- Familiarity with the page template specification at
  `.agent/drafts-outbound/DESIGN-regional-market-topic-template.draft.md` —
  the TOPIC's section order must match the template.

You do **not** need:

- Any pip install. The script in Step 3 is standard-library only.
- Access to the design system. The article is plain Markdown; the
  template is applied by the wiki platform at render time.

---

## Step 1: Run the scoring script

From the monorepo root:

```bash
cd pointsav-monorepo/app-orchestration-gis/
python3 score-regional-markets.py
```

This produces (or refreshes) two ranked JSON files under `work/`:

- `work/top400-na.json` — North-American Regional Markets, ranked
- `work/top400-eu.json` — European Regional Markets, ranked

If the script reports missing input files, the most recent cluster
build is stale or absent; ask the GIS pipeline owner to refresh
`regional-markets.json` before continuing.

---

## Step 2: Select a Regional Market

Open the appropriate ranking file:

```bash
less work/top400-na.json
# or
less work/top400-eu.json
```

Each entry in the JSON list has the shape:

```json
{
  "rank": 16,
  "rm_id": "RM-NA-0123",
  "market": "Plano, Texas",
  "country": "US",
  "centroid_lat": 33.0198,
  "centroid_lon": -96.6989,
  "cluster_ids": ["NA-12345", "NA-12378", ...],
  "score": 25.5,
  "t1": 3, "t2": 2, "t3": 1,
  "mkt_conf": "high",
  "components": { /* per-metric scoring breakdown */ }
}
```

### Selection criteria

Pick markets that satisfy all of these:

1. **`mkt_conf: "high"`** — the market boundary resolution is
   confident. Skip `medium` and `low` until the boundary lookup is
   refreshed.
2. **Single-city** — verify by eye that the centroid sits inside the
   named city's metropolitan footprint. If the centroid lands between
   two adjacent cities (a multi-city RM), skip it; those are a
   separate article archetype.
3. **`t1 > 0` preferred** for the first batch — Tier-1-bearing markets
   are the highest-readership category. Pure-T2 markets are valid
   targets but slot into a later batch.

### Bookkeeping

Note these values before moving on; you will reference them in every
subsequent step:

- `rm_id` (article slug source)
- `market` (article title)
- `centroid_lat`, `centroid_lon` (used by the Wikipedia thumbnail)
- `cluster_ids` (used for the AEC join in Step 4)
- `score` and `rank` (infobox)

---

## Step 3: Fetch Wikipedia pages

The Wikipedia REST summary API returns: a short text extract suitable
for the Overview section, a thumbnail URL, and the canonical article
URL. Standard library only — no `requests` import.

```python
import urllib.request
import urllib.parse
import json

def wp_summary(title):
    """Fetch a Wikipedia page summary as a dict."""
    slug = urllib.parse.quote(title.replace(" ", "_"), safe="_,()")
    url = f"https://en.wikipedia.org/api/rest_v1/page/summary/{slug}"
    req = urllib.request.Request(
        url,
        headers={"User-Agent": "regional-market-research/1.0 (jmwoodfine@gmail.com)"}
    )
    with urllib.request.urlopen(req, timeout=15) as r:
        return json.loads(r.read())

# Examples
# US city:        wp_summary("Plano, Texas")
# German city:    wp_summary("Krefeld")
# French city:    wp_summary("Toulouse")
# UK city:        wp_summary("Manchester")
```

The returned dict carries fields including:

- `extract` — plain-text summary (use this for the Overview)
- `extract_html` — HTML summary (use only if the Overview will be
  rendered as raw HTML rather than Markdown)
- `thumbnail.source` — image URL for the infobox thumbnail (may be
  absent for low-profile cities)
- `content_urls.desktop.page` — canonical Wikipedia URL (use this in
  the attribution footer)

### Which pages to fetch

For each Regional Market, fetch **2 or 3** Wikipedia pages:

1. **The city itself** — always. This is the Overview source.
2. **The county, district, or LAU** — usually. Adds administrative
   context.
3. **One civic anchor, optional** — the main university, the largest
   hospital system, or another institution that genuinely belongs in
   the Overview. Skip if forcing a third reference would be padding.

### Attribution discipline

The fetch step is the only point at which raw Wikipedia prose enters
your article. Mark every sentence that came from Wikipedia. The
attribution footer in Step 5 lists every page you fetched; if you
fetched a page you did not end up quoting, drop it from the footer
list — only attribute what is actually used.

---

## Step 4: Join AEC data

The AEC dataset is a flat CSV keyed by cluster_id. Filter it to the
clusters in your selected Regional Market:

```python
import csv

target_clusters = ["NA-12345", "NA-12378"]  # from the RM JSON

with open("work/DATA-aec-clusters.csv") as f:
    reader = csv.DictReader(f)
    aec = {row["cluster_id"]: row
           for row in reader
           if row["cluster_id"] in target_clusters}
```

The relevant fields (the template's Section 4 enumerates the full
set) include:

- `ashrae_zone` — US only
- `eu_climate_zone` — EU only
- `koppen`
- `wwf_ecoregion`
- `wwf_biome`
- `seismic_pga`
- `flood_100yr`

Multiple clusters in the same RM often share AEC values (because they
sit in the same climate zone). If all clusters in the RM share a
single value for a field, render the value once. If they disagree
(e.g. one cluster is Köppen Cfa, another Cfb), render the value with
the cluster count: "Cfa (3 clusters), Cfb (1 cluster)".

---

## Step 5: Write the TOPIC article

### Frontmatter

Use this template (replace bracketed values):

```yaml
---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC
audience: external-readers
bcsc_class: no-disclosure-implication
title: "[Market name] — Regional Market"
date: 2026-05-30
rm_id: "RM-NA-0123"
country: "US"
rank: 1
rank_total: 400
score: 25.5
best_tier: 1
cluster_count: 6
centroid_lat: 33.0198
centroid_lon: -96.6989
wikipedia_city: "https://en.wikipedia.org/wiki/Plano,_Texas"
wikipedia_sources:
  - "https://en.wikipedia.org/wiki/Plano,_Texas"
  - "https://en.wikipedia.org/wiki/Collin_County,_Texas"
wikipedia_thumbnail: "https://upload.wikimedia.org/..."   # optional
data_vintage: "2026-05-22"  # date of the cluster build the RM was scored against
paired_with: "topic-rm-[slug].es.md"  # if a Spanish sibling is planned
---
```

### Lead paragraph

Two or three sentences. Required content, in order:

1. Market name + rank ("Plano is the rank-1 Regional Market in North America…").
2. Cluster count + composition summary.
3. Distance from the nearest larger metro (helps reader place it).

Example:

> Plano, Texas is the rank-1 Regional Market in North America, containing
> six co-location clusters. It is situated 28 km north of Dallas within
> the Dallas–Fort Worth metroplex.

### Body sections (in order — match the template)

1. **Overview** — Wikipedia extract, attributed inline.
2. **Co-location Profile** — table, one row per cluster.
3. **Civic Infrastructure** — Medical / Academic / (optional) Other.
4. **AEC Data** — only fields that have values.
5. **Composite Score** — six components + total.
6. **Wikipedia References** — bullet list of pages fetched.

### Markdown patterns

The wiki platform renders standard CommonMark with table support.
Use plain Markdown tables; CSS classes from the template are applied
by the wiki renderer based on the surrounding `<section>` wrapper or
the frontmatter `artifact_type: TOPIC` value. You do not write
classes into the Markdown.

---

## Step 6: Language and attribution checks

Before saving the draft, do one pass for each of these:

### Internal-term scrub

Open `.agent/rules/journal-artifact-discipline.md` and find the
"Forbidden vocabulary" section. None of those terms may appear in the
TOPIC body. Common offenders to grep for:

```bash
grep -nE 'PointSav|Foundry|Totebox|Doorman|BCSC|Stage 6|F12' \
  .agent/drafts-outbound/TOPIC-rm-[slug].draft.md
```

If any hit, rewrite. Internal product / process names never appear
in reader-facing TOPICs.

### Wikipedia attribution

Every sentence sourced from Wikipedia carries an inline lead-in such
as: "Per Wikipedia, accessed [date], …". The footer (see Step 7 of
the design template) repeats the license and lists the source pages.

### Forward-looking hedge

Any sentence that describes a future state (planned data layers, an
upcoming rebuild, an intended methodology change) carries "planned",
"intended", "may", or "target". This applies to TOPIC articles
identically to all other Foundry artifacts.

### Region-specific data fields

- **ASHRAE zones apply to US markets only.** Omit the row for any
  EU / UK / non-US market.
- **EU Climate Zones apply to European markets only.** Omit for US,
  Canada, Mexico.
- The Köppen-Geiger class, WWF Ecoregion, and WWF Biome rows are
  global and always included if populated.

---

## Step 7: Dispatch to project-editorial

### Save the file

Filename pattern (lowercase, hyphenated market slug):

```
.agent/drafts-outbound/TOPIC-rm-[market-slug].draft.md
```

Examples:

```
.agent/drafts-outbound/TOPIC-rm-plano-tx.draft.md
.agent/drafts-outbound/TOPIC-rm-krefeld-de.draft.md
.agent/drafts-outbound/TOPIC-rm-toulouse.draft.md
```

### Update the artifact registry

Add an entry in `.agent/rules/artifact-registry.md` under
**A — Active / In-Progress**, with this shape:

```markdown
### A[N] — TOPIC: Regional Market — [Market Name]
- **File:** `TOPIC-rm-[slug].draft.md`
- **Status:** STAGED in drafts-outbound/ ([YYYY-MM-DD]) — awaiting dispatch
- **Destination:** project-editorial → content-wiki-documentation/topics/regional-markets/
- **Content:** Regional Market TOPIC for [Market Name] (rank [N], [country], [tier]).
  Sources: [N] Wikipedia pages, AEC join across [N] clusters, score-regional-markets.py vintage [YYYY-MM-DD].
```

### Post outbox message

Append to `.agent/outbox.md`:

```
---
from: project-gis (totebox)
to: project-editorial
re: TOPIC dispatch — Regional Market: [Market Name]
created: [ISO 8601 timestamp]
---

New TOPIC draft staged for editorial review and commit to
content-wiki-documentation:

- File: .agent/drafts-outbound/TOPIC-rm-[slug].draft.md
- Type: Regional Market TOPIC ([N] sections per design template)
- Wikipedia sources: [N] pages, all attributed; CC BY-SA 4.0 footer present
- Forbidden-vocabulary pass: clean
- Forward-looking hedge: applied
- Spanish sibling: [planned | not planned] (paired_with field [set | absent])

Message id: project-gis-[YYYYMMDD]-topic-rm-[slug]
```

Replace `[YYYYMMDD]` with today's date in compact ISO format. The
message id pattern matches the project-gis outbound convention used
for A1–A6.

---

## Data vintage note

The cluster dataset and the scoring output are rebuilt on a regular
cadence (Phase 22, Phase 23, and subsequent phases each refresh the
ranking). After every cluster rebuild:

1. Re-run `python3 score-regional-markets.py`.
2. Compare the new `top400-na.json` and `top400-eu.json` against the
   ranking that produced live TOPIC articles.
3. For each live TOPIC whose `rank` or `score` changed materially
   (more than three places, or score delta > 5), open a refresh
   ticket against the article.

The `data_vintage` frontmatter field is the article's promise to the
reader about which cluster build it was scored from. If the article's
content references specific counts ("four clusters", "rank 16 of
400"), those numbers should be updated on every refresh; if it
references only relative positioning ("a Tier-1 Regional Market"),
refresh is optional.

---

## Wikipedia data freshness

Wikipedia summary API calls fetch the page content at the moment the
TOPIC is produced. The `wikipedia_city` frontmatter URL points at the
live Wikipedia page, so a reader who clicks through always gets the
current version; the Overview section of the TOPIC, however, is a
snapshot.

The attribution footer must always show the date the summary was
fetched. When an editor refreshes the Overview section from a fresh
Wikipedia call, the footer date updates and the article is committed
as a new revision.

A periodic refresh pass (quarterly is a reasonable cadence) re-fetches
the Wikipedia summary for every live TOPIC and updates any sections
whose source content has materially changed. The footer date is the
audit trail.

---

## Quick-reference checklist

Before dispatch, confirm:

- [ ] Frontmatter populated, including `rm_id`, `rank`, `score`,
      `data_vintage`, and `wikipedia_sources` list.
- [ ] Lead paragraph names the market, rank, cluster count, and
      nearest larger metro.
- [ ] Overview section quotes Wikipedia with an inline attribution
      lead-in.
- [ ] Co-location Profile table has one row per cluster, ordered by
      tier then by descriptive name.
- [ ] Civic Infrastructure lists every medical and academic member;
      "Other" only when there is genuine other infrastructure.
- [ ] AEC Data omits unpopulated fields (no "N/A" rows).
- [ ] Composite Score table sums to the value in the infobox.
- [ ] Wikipedia References lists every page the article cites.
- [ ] CC BY-SA 4.0 attribution footer present with access date.
- [ ] Forbidden-vocabulary grep returns nothing.
- [ ] Forward-looking sentences carry a hedge word.
- [ ] Region-specific AEC fields (ASHRAE / EU Climate Zone) match
      the market's region.
- [ ] File saved under `.agent/drafts-outbound/TOPIC-rm-[slug].draft.md`.
- [ ] Entry added to `.agent/rules/artifact-registry.md`.
- [ ] Outbox message posted to `project-editorial`.
