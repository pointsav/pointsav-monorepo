---
schema: foundry-draft-v1
artifact: GUIDE
language_protocol: GUIDE
to: project-editorial
state: pending-editorial-review
created: 2026-06-05
author: totebox@project-orgcharts
cluster: project-orgcharts
deployment: cluster-totebox-corporate-1
tenant: woodfine
paired_with: ""
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: "inputs/README.md (naming convention); MANIFEST.md (deployment layout); manifest.md (cluster mission + sub-clone scope)"
research_inline: "Authoring workflow derived directly from the canonical naming convention in inputs/README.md. Deployment structure from MANIFEST.md. No additional research needed — this guide captures operational facts, not design decisions."
---

# GUIDE-orgchart-authoring — Woodfine Corporate Org Chart Authoring

> **Operational guide.** Describes how Jennifer Woodfine authors and revises
> corporate org charts using the project-orgcharts Totebox cluster.
>
> Destination: `customer/woodfine-fleet-deployment/cluster-totebox-corporate/`
> via project-editorial gateway.

---

## 1. Overview

The project-orgcharts cluster produces corporate org charts and related
visualizations (SPV arrangement diagrams, governance charts, board structure
charts) for Woodfine Capital Projects and its subsidiaries. The cluster
consists of:

- A Totebox Session running on the `foundry-workspace` VM at
  `~/Foundry/clones/project-orgcharts/`
- A private deployment instance at
  `~/Foundry/deployments/cluster-totebox-corporate-1/` (gitignored; never
  pushed to GitHub)
- Three design-system sub-clones that receive component backfill as new
  visual patterns emerge from chart authoring

Jennifer Woodfine (operator) supplies source files; Task Claude produces
rendered drafts and final outputs.

---

## 2. File-naming convention

Every file in `inputs/`, `working/`, and `outputs/` follows this pattern:

```
<DEPARTMENT>_<ENTITY>_<YYYY-MM-DD>_<chart-slug>_JW<n>_AI<n>_<STATUS>.<ext>
```

| Part | Format | Example |
|---|---|---|
| `<DEPARTMENT>` | UPPERCASE; spaces allowed | `INVESTOR RELATIONS`, `COMPLIANCE` |
| `<ENTITY>` | Short code | `MCorp`, `WCPI`, `PointSav` |
| `<YYYY-MM-DD>` | ISO 8601 | `2026-04-25` |
| `<chart-slug>` | kebab-case | `spv-arrangements`, `exec-team` |
| `JW<n>` | Jennifer's revision counter | `JW1`, `JW2` |
| `AI<n>` | Task Claude iteration counter | `AI1`, `AI2` |
| `<STATUS>` | One of five words | `SOURCE`, `DRAFT`, `REVIEW`, `FINAL`, `ARCHIVE` |
| `<ext>` | File extension | `.html`, `.svg`, `.pdf` |

**JW counter:** Bumps every time Jennifer uploads a new source version.
**AI counter:** Resets to 1 when JW bumps; increments with every Task Claude
iteration off the current JW source.

### Status vocabulary

| Status | Owner | Folder |
|---|---|---|
| `SOURCE` | Jennifer | `inputs/` |
| `DRAFT` | Task Claude | `working/` |
| `REVIEW` | Task Claude | `working/` |
| `FINAL` | Task Claude (after Jennifer approval) | `outputs/` |
| `ARCHIVE` | Either | Any |

---

## 3. Authoring workflow

### Step 1 — Prepare your source file

Upload the source file to `inputs/current-org-chart-html/` using the naming
convention above. The first upload for any chart is always `JW1_AI1_SOURCE`.

Acceptable source formats, in order of preference:
1. An existing rendered HTML, SVG, PDF, or PowerPoint-to-HTML export
2. A structured CSV (columns: `name`, `title`, `reports_to`, `department`)
3. An indented text outline (two spaces per hierarchy level)
4. Prose description (least preferred — use when no structured input exists)

Companion files (photos, notes, brand assets) share the same slug and counters,
with a suffix after `_SOURCE`: `_SOURCE_notes.md`, `_SOURCE_photo-name.jpg`.

### Step 2 — Open the cluster session

In Claude Code, navigate to `~/Foundry/clones/project-orgcharts/` and start
a session. Say `startup` to initialize. Task Claude will read the manifest,
inbox, and session context.

### Step 3 — Request a draft

Ask Task Claude to produce a draft from the source file you uploaded. Specify
the chart slug and any requirements (branding, entity names, layout preferences,
design-system token constraints).

Task Claude will produce:
```
working/<DEPARTMENT>_<ENTITY>_<YYYY-MM-DD>_<chart-slug>_JW<n>_AI2_DRAFT.html
```

### Step 4 — Review and iterate

Review the draft in a browser. Provide feedback in the chat session. Task
Claude increments `AI<n>` on each revision. When a draft is ready for your
final review, Task Claude marks it `REVIEW`.

To make changes on your side (revisions to the source data, new entity names,
structural changes), upload a new source file with `JW<n+1>_AI1_SOURCE`. Task
Claude then drafts off the new source, resetting `AI` to `AI2`.

Do not edit the `working/` files directly — Task Claude is the owner of that
folder. Redirect changes through chat or a new source upload.

### Step 5 — Approve and finalize

When you approve a REVIEW file, tell Task Claude. Task Claude renames it to
`FINAL` status and moves it to `outputs/`.

```
outputs/<DEPARTMENT>_<ENTITY>_<YYYY-MM-DD>_<chart-slug>_JW<n>_AI<n>_FINAL.html
```

Final HTML can be opened locally in any browser and printed to PDF from there.

### Step 6 — Design-system backfill (optional)

If a new visual pattern emerged during this chart (a new node shape, connector
style, badge variant), Task Claude will stage a `DESIGN-COMPONENT` or
`DESIGN-TOKEN-CHANGE` draft to `.agent/drafts-outbound/` and route it to
`project-design`. This happens automatically at session shutdown when new
patterns are identified. No action required from Jennifer.

---

## 4. Corporate entity codes

| Short code | Full name |
|---|---|
| `MCorp` | Woodfine Management Corp. |
| `WCPI` | Woodfine Capital Projects Inc. |
| `BPC` | Bencal Private Capital Inc. |
| `PointSav` | PointSav Digital Systems |

Use these codes consistently in filenames and in internal chart metadata.

---

## 5. Hard rules

1. **Never overwrite a file.** Always create a new file with the next counter.
2. **Never delete an active version.** Archive with `ARCHIVE` status instead.
3. **Never reuse a chart-slug for a different chart.** Each subject gets its own slug.
4. **One chart per source file.** Upload separate subjects separately.
5. **Internal references match the slug.** HTML `<title>`, IDs, and class names
   use the same kebab-case slug as the filename.

---

## 6. Where things live

| Path | Contents | Access |
|---|---|---|
| `inputs/` | Jennifer's source uploads | Jennifer writes; Task Claude reads |
| `working/` | Task Claude in-progress drafts | Task Claude writes; Jennifer reads |
| `outputs/` | Final approved artifacts | Task Claude writes; Jennifer pulls |
| `inputs/current-org-chart-html/` | Latest source HTML files | Jennifer's primary upload folder |

All paths are relative to
`~/Foundry/deployments/cluster-totebox-corporate-1/`.

---

*Authored 2026-06-05 by totebox@project-orgcharts.
Source authority: `inputs/README.md` (naming convention) + `MANIFEST.md` (deployment layout).*
