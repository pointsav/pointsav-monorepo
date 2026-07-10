---
schema: foundry-draft-v1
draft_id: topic-corporate-office-leasehold-fraction
language_protocol: PROSE-TOPIC
state: ready-for-sweep
target_path: vendor/content-wiki-projects/building-design/corporate-office-leasehold-fraction.md
created: 2026-07-10T00:00:00Z
author: task@project-bim
cites: []
research_done_count: 1
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Direct inspection of woodfine-bim-library/tokens/bim/floor-plate-standards.dtcg.json,
  tile-system.dtcg.json, key-plans.dtcg.json, tenant-mix.dtcg.json,
  building-width-calculator.dtcg.json, and app-privategit-bim/src/render/catalog.rs, Round 5
  background workflow wf_9c7ae20f-be0 (2026-07-10).
research_inline: false
---

# Corporate Office: A Leasehold Fraction, Not a Furniture Composition

## Lede

The five Corporate Office entries on this site — an eighth, a quarter, a third, a half, and a full floor — render with a distinct "Leasehold sized as a fraction of the Floor Plate" note instead of a furniture plan. This is not a data gap awaiting authorship. It is a correct, deliberate, structurally different kind of catalog entry, and this TOPIC explains why.

---

## Two different kinds of Composition

Every other Composition category on this site — Private Office, Medical, Business, Laboratory, Academic, Civic — is a **furniture-level Composition**: a spatial programme assembled from real furniture placement, with named rooms, occupancy counts, and (where the catalog carries a matching product) a linked parts list. These Compositions describe what an architect would actually draw and furnish inside a leased space.

Corporate Office is a **leasehold-area Composition** instead. It describes a fraction of a Floor Plate — one-eighth, one-quarter, one-third, one-half, or a full floor — as a leasing unit, without prescribing what goes inside it. Woodfine's own token data states this design decision explicitly: dimensions for Corporate Office are "a proportion of the Floor Plate; independent Key Plan dimensions are not defined at this stage," with the architect expected to size the interior against confirmed Floor Plate dimensions once a specific tenant and fit-out are known. The companion token file governing floor-scale tile fractions describes Corporate Office tenants as free to occupy any fraction up to a full floor — the defining fact about a Corporate Office leasehold is its size relative to the floor, not its interior layout.

## Why no furniture data exists

This is confirmed at three independent levels, not merely inferred: the Key Plan token schema's own description states Corporate Office dimensions are deliberately undefined at the catalog stage; the Building Width Calculator's zone-depth registry — the authoritative source for every other category's three-zone data — has no Corporate Office entry to be missing; and every Tile containing a Corporate Office unit represents it as a single, unsubdivided leasehold-area line, unlike Tiles built from Private or Professional Office units, which list multiple furnished sub-units individually. A Corporate Office tenant designs their own interior fit-out with their own architect — Woodfine's Key Plan system, by design, never touches it.

## Why the honest-pending state is correct, not a placeholder

Fabricating furniture-level zone data for Corporate Office would misrepresent the design methodology itself: it would attribute an interior layout to Woodfine's architects that they have explicitly not authored, for a leasehold type where authoring it is contractually and practically the tenant's own architect's job. The "Leasehold sized as a fraction of the Floor Plate — tenant designs interior layout" note is this site's way of stating that fact plainly, rather than leaving a visitor to guess whether the entry is simply incomplete.

---

## References

- Woodfine internal token library, `floor-plate-standards.dtcg.json`, `tile-system.dtcg.json`, `key-plans.dtcg.json` (unpublished primary source)
