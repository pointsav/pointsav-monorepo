---
schema: foundry-draft-v1
state: draft-pending-design-review
originating_cluster: project-gis
target_repo: vendor/pointsav-design-system
target_path: research/
target_filename: DESIGN-RESEARCH-bento-merged-zones-disclosure.md
audience: internal-design
bcsc_class: internal
language_protocol: DESIGN-RESEARCH
authored: 2026-05-08
authored_by: project-gis Task Claude
authored_with: claude-opus-4-7
research_done_count: 3
research_suggested_count: 1
open_questions_count: 0
research_provenance: |
  Derived from GIS Sprint 9 Phase 3 (BentoBox 0.15 km dedup transparency). Operator
  constraint: "Show retailers collapsed under 0.15km of the same names in the box,
  for 100% transparency." Sprint 9 audit identified silent suppression as the issue
  (deduplicate_clusters dropped 1,172 candidates with no audit trail).
  Implementation: generate-rankings.py:167-185, index.html:823-887.
research_inline: false
notes_for_editor: |
  Route to project-design for design-system integration.
  Pattern is dataset-agnostic; document it as a reusable transparency primitive.
---

# DESIGN RESEARCH: Silent-Dedup Transparency Disclosure

**Surface:** Inspector panel showing data that has passed through a deduplication step.
**Component:** Disclosure line beneath the survivor entity.
**Status:** Shipped GIS Sprint 9 (`7e92013`) for the cluster co-location BentoBox.

---

## The Problem

Many data pipelines collapse near-duplicate entries before presentation: same-zone retail clusters within 150 m of each other, same-name records within a 200 m radius, same-coordinate POIs from multiple sources. A naive dedup discards the suppressed entries entirely. The presented entity carries no signal that it represents multiple inputs.

This produces three trust issues:

1. **Silent suppression** — a user looking at one survivor cluster has no way to know that their search term ("Costco at Edmonton South Common") might have been folded into a sibling ("Home Depot at Edmonton South Common") with no notification.
2. **Recovery impossible** — once suppressed, the data is dropped from the manifest; the survivor cluster carries only its own anchor identity.
3. **Audit unfriendly** — a methodology reviewer cannot reconstruct the dedup decision without re-running the pipeline.

## The Decision

The deduplication step **annotates** the survivor with the suppressed entries' identifying fields rather than discarding them. The inspector panel surfaces the annotation as a disclosure line.

### Pipeline-side annotation

```python
# generate-rankings.py — deduplicate_clusters
if survivor_idx is not None:
    survivor = kept_features[survivor_idx]
    p = feat["properties"]
    zones = survivor["properties"].setdefault("merged_zones", [])
    zones.append({
        "anchor": p.get("anchor_label") or p.get("primary_anchor") or "",
        "cluster_id": p.get("cluster_id") or "",
    })
    continue
```

Each suppressed entity contributes a short record (anchor name + identifier) to the survivor's `merged_zones` array. No spatial geometry is preserved — only enough to name the suppressed entity for human review.

### Frontend disclosure

The inspector panel surfaces `merged_zones` as a single-line italic note beneath the primary anchor pill:

```html
{mz.length > 0 ? `<div style="font-size:10px;color:var(--text-muted);
                       width:100%;margin-top:6px;font-style:italic;">
   Co-located within 150 m: ${mz.map(z => z.anchor).join(' · ')}
 </div>` : ''}
```

No expand/collapse control. No chrome. The line is short, italicised, muted — it is acknowledgement, not feature.

## Reusable Pattern

**When a deduplication step compresses inputs to a single survivor:**

1. Treat dedup as **annotation**, not deletion. The survivor carries an array of suppressed identifiers.
2. Annotation fields should be the minimum needed to **name** the suppressed entries for human review (typically: name + stable id). Geometry and full payload can stay dropped.
3. Surface the annotation in the inspector panel as a low-chrome, low-font, italic note. Plain text. No toggle.
4. Choose disclosure language that names the **threshold** (e.g., "within 150 m"); the threshold is the audit signal.
5. Render only when the array is non-empty. Don't introduce a placeholder.

## Why low-chrome

A toggle or expand/collapse would imply this is a feature surface — somewhere users go to discover something. It isn't. It's an audit trail. The right design language is "I notice you noticed; here is what happened." That is one italic line.

This also keeps the inspector panel scannable. The primary anchor is the headline; the dedup disclosure is a footnote.

## Research Trail

### Done
1. Sprint 9 audit (2026-05-08) confirmed `deduplicate_clusters()` was silently dropping 1,172 of 7,594 candidates — 15% of pipeline output unaudited.
2. Annotation-not-deletion approach evaluated against the alternative of writing a separate `clusters-dedup-log.json` audit file. The annotation approach won: keeps the audit trail with the survivor, requires no separate fetch, no separate sync to the deployment.
3. Live BentoBox shipped. 1,162 of 6,422 surviving clusters now carry `merged_zones`; 18% of clusters disclose at least one consolidation. Feedback was that the disclosure feels honest, not noisy.

### Suggested for next editor
1. Cross-cluster reuse: project-bookkeeping is exploring deduplication of vendor records; same pattern (annotation + low-chrome disclosure) may apply.

## Implementation Reference

| File | Line range | Purpose |
|---|---|---|
| `pointsav-monorepo/app-orchestration-gis/generate-rankings.py` | 167–185 | `deduplicate_clusters` annotates survivor |
| `pointsav-monorepo/app-orchestration-gis/build-tiles.py` | 320 | `mz` field carried through clusters-meta |
| `pointsav-monorepo/app-orchestration-gis/www/index.html` | 823–887 | Inspector renders the disclosure line |

## See Also

- DESIGN-RESEARCH-tier-naming-accessibility.draft.md (Sprint 9 tier rebrand)
- DESIGN-RESEARCH-zoom-prefetch-pattern.draft.md (Sprint 9 zoom transition)
- topic-cluster-deduplication-threshold.md (customer-facing TOPIC, already in pipeline)
