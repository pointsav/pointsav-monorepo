# Token / Component Intake Checklist — project-design

At session start: sweep outbox and any handoffs-outbound for DESIGN-* or ASSET-* drafts
routed here. For each, open a plan in `.agent/plans/` to track processing.

## Three repo routing rules

| Destination | Artifact types | Published at |
|---|---|---|
| `pointsav-design-system` | Generic DTCG tokens, component recipes, foundation docs, accessibility specs, research | design.pointsav.com |
| `pointsav-media-assets` | PointSav brand: SVG/PNG, CSS (--ps-*), YAML color/theme/linguistic tokens | none (GitHub only) |
| `woodfine-media-assets` | Woodfine brand: SVG/PNG, CSS (--wf-*), YAML color/theme/linguistic/architecture tokens | none (GitHub only) |

**Sorting question:** "Does this artifact belong to a specific tenant's brand identity?"
→ media-assets. "Is it a generic design token, component recipe, or research?"
→ pointsav-design-system.

## Complete unit schema by artifact type

### DESIGN-COMPONENT
Required files:
- `components/<name>/guide.md` — HTML + CSS + ARIA recipe
- `dtcg-vault/research/component-<name>.md` — AI-readable design rationale

Required frontmatter in draft:
- `language_protocol: DESIGN-COMPONENT`
- `component_metadata.component_name`
- `component_metadata.carbon_baseline` (or explicit "no Carbon baseline" justification)
- `component_metadata.accessibility_targets`
- 5 research-trail fields: `research_done_count`, `research_suggested_count`, `open_questions_count`, `research_provenance`, `research_inline`
- `paired_with` — ES language pair (required for public-audience content)

Rejection criteria: no research file, no ES pair for public content, no Carbon baseline
field, no `ai_consumption_hint` in research file.

### DESIGN-TOKEN-CHANGE
Required files:
- DTCG JSON patch to `tokens/dtcg-bundle.json` (or new token file under `tokens/`)
- `dtcg-vault/research/<topic>.md` — justification + downstream impact

Required frontmatter in draft:
- `language_protocol: DESIGN-TOKEN-CHANGE`
- `master_cosign:` — MANDATORY; do not commit without this field populated
- Downstream impact analysis in research trail

### DESIGN-RESEARCH
Required files:
- `dtcg-vault/research/<topic>.md`

Required frontmatter:
- `language_protocol: DESIGN-RESEARCH`
- 5 research-trail fields

Research files land in `dtcg-vault/research/` — NOT in `docs/`. They are the AI
consumption surface (codegen agents read these at request time via Doorman).

### ASSET (binary media — SVG, PNG, JPEG)
Required frontmatter:
- `language_protocol: ASSET`
- `target_repo` — one of: `pointsav/pointsav-media-assets`, `woodfine/woodfine-media-assets`
- `target_path` — directory within the repo
- `target_filename`
- `asset_type` — favicon | logo | badge | screenshot | illustration | photo | icon
- `asset_format` — svg | png | jpg | webp | pdf
- `state: asset-staged-pending-design-commit` (NOT pending-master; project-design handles directly)

Operator-capture assets (screenshots, photos) use `state: asset-capture-pending-operator`.
Add to outbox as operator-action item; do not auto-commit.

## Commit procedure per destination

**pointsav-design-system:**
1. Work in `clones/project-design/pointsav-design-system/` on `cluster/project-design` branch
2. Commit with `~/Foundry/bin/commit-as-next.sh`
3. Fast-forward local `main` → `git branch -f main cluster/project-design`
4. Force-push staging mirrors: `git push --force-with-lease origin-staging-j main && git push --force-with-lease origin-staging-p main`
5. Run `~/Foundry/bin/promote.sh` from `main` branch → promotes to canonical

**pointsav-media-assets / woodfine-media-assets:**
1. Work in the respective sub-clone under `clones/project-design/`
2. Commit with `~/Foundry/bin/commit-as-next.sh` (staging-tier identity)
3. Push directly: `git push origin main`
   - pointsav-media-assets origin: `git@github.com-pointsav-administrator:pointsav/pointsav-media-assets.git`
   - woodfine-media-assets origin: `git@github.com-woodfine-administrator:woodfine/woodfine-media-assets.git`
4. No promote.sh needed (no staging mirrors; admin SSH alias handles auth)

## Iteration loop

When a project-* archive produces new tokens or components, they route to this cluster
as DESIGN-TOKEN-CHANGE or DESIGN-COMPONENT drafts. The loop:

1. project-* drops draft in own drafts-outbound with target = project-design
2. project-design picks up at session start (sweep outbox + handoffs)
3. project-design reviews completeness against this checklist
4. If DESIGN-TOKEN-CHANGE: confirm master_cosign present before committing
5. Commit to pointsav-design-system → Stage 6
6. Ack to originating project-* outbox with SHA
7. design.pointsav.com rebuilds (or manual rebuild trigger for MVP static file service)
8. Other project-* archives can now curl design.pointsav.com/tokens.full.json for updated bundle
