---
schema: foundry-draft-v1
artifact: guide
language_protocol: PROSE-GUIDE
route_to: project-editorial
status: staged
created: 2026-06-05
author: totebox@project-orgcharts
cluster: project-orgcharts
deployment: <orgchart-deployment-instance>
tenant: woodfine
research_trail_source: inputs/README.md (naming convention); MANIFEST.md (deployment layout); manifest.md (cluster mission + sub-clone scope)
research_trail_method: operational — authoring workflow derived from canonical naming convention and deployment layout
research_trail_data: inputs/README.md; MANIFEST.md; manifest.md
research_trail_claims: Captures operational facts of the org-chart authoring workflow
research_trail_gaps: None — no additional research needed; this guide captures operational facts, not design decisions
---

# GUIDE-orgchart-authoring — Corporate Org Chart Authoring

> **Operational guide.** Describes how the operator authors and revises
> corporate org charts using the project-orgcharts Totebox cluster.
>
> Destination: `customer/woodfine-fleet-deployment/cluster-totebox-corporate/`
> via project-editorial gateway.

---

## 1. Overview

The project-orgcharts cluster produces corporate org charts and related
visualizations (SPV arrangement diagrams, governance charts, board structure
charts) for the customer's corporate group and its subsidiaries. The cluster
consists of:

- A Totebox Session running on the workspace VM at the
  `project-orgcharts` clone root
- A private deployment instance (gitignored; never pushed to GitHub)
- Three design-system sub-clones that receive component backfill as new
  visual patterns emerge from chart authoring

The operator supplies source files; the cluster session produces
rendered drafts and final outputs.

---

## 2. File-naming convention

Every file in `inputs/`, `working/`, and `outputs/` follows this pattern:

```
<DEPARTMENT>_<ENTITY>_<YYYY-MM-DD>_<chart-slug>_OP<n>_AI<n>_<STATUS>.<ext>
```

| Part | Format | Example |
|---|---|---|
| `<DEPARTMENT>` | UPPERCASE; spaces allowed | `INVESTOR RELATIONS`, `COMPLIANCE` |
| `<ENTITY>` | Short code | `MCorp`, `WCPI`, `PointSav` |
| `<YYYY-MM-DD>` | ISO 8601 | `2026-04-25` |
| `<chart-slug>` | kebab-case | `spv-arrangements`, `exec-team` |
| `OP<n>` | Operator's revision counter | `OP1`, `OP2` |
| `AI<n>` | Cluster-session iteration counter | `AI1`, `AI2` |
| `<STATUS>` | One of five words | `SOURCE`, `DRAFT`, `REVIEW`, `FINAL`, `ARCHIVE` |
| `<ext>` | File extension | `.html`, `.svg`, `.pdf` |

**OP counter:** Bumps every time the operator uploads a new source version.
**AI counter:** Resets to 1 when OP bumps; increments with every cluster-session
iteration off the current OP source.

### Status vocabulary

| Status | Owner | Folder |
|---|---|---|
| `SOURCE` | Operator | `inputs/` |
| `DRAFT` | Cluster session | `working/` |
| `REVIEW` | Cluster session | `working/` |
| `FINAL` | Cluster session (after operator approval) | `outputs/` |
| `ARCHIVE` | Either | Any |

---

## 3. Authoring workflow

### Step 1 — Prepare your source file

Upload the source file to `inputs/current-org-chart-html/` using the naming
convention above. The first upload for any chart is always `OP1_AI1_SOURCE`.

Acceptable source formats, in order of preference:
1. An existing rendered HTML, SVG, PDF, or PowerPoint-to-HTML export
2. A structured CSV (columns: `name`, `title`, `reports_to`, `department`)
3. An indented text outline (two spaces per hierarchy level)
4. Prose description (least preferred — use when no structured input exists)

Companion files (photos, notes, brand assets) share the same slug and counters,
with a suffix after `_SOURCE`: `_SOURCE_notes.md`, `_SOURCE_photo-name.jpg`.

### Step 2 — Open the cluster session

In Claude Code, navigate to the `project-orgcharts` clone root and start
a session. Say `startup` to initialize. The cluster session will read the manifest,
inbox, and session context.

### Step 3 — Request a draft

Ask the cluster session to produce a draft from the source file you uploaded. Specify
the chart slug and any requirements (branding, entity names, layout preferences,
design-system token constraints).

The cluster session will produce:
```
working/<DEPARTMENT>_<ENTITY>_<YYYY-MM-DD>_<chart-slug>_OP<n>_AI2_DRAFT.html
```

### Step 4 — Review and iterate

Review the draft in a browser. Provide feedback in the chat session. The cluster
session increments `AI<n>` on each revision. When a draft is ready for your
final review, the cluster session marks it `REVIEW`.

To make changes on your side (revisions to the source data, new entity names,
structural changes), upload a new source file with `OP<n+1>_AI1_SOURCE`. The cluster
session then drafts off the new source, resetting `AI` to `AI2`.

Do not edit the `working/` files directly — the cluster session is the owner of that
folder. Redirect changes through chat or a new source upload.

### Step 5 — Approve and finalize

When you approve a REVIEW file, tell the cluster session. The cluster session renames it to
`FINAL` status and moves it to `outputs/`.

```
outputs/<DEPARTMENT>_<ENTITY>_<YYYY-MM-DD>_<chart-slug>_OP<n>_AI<n>_FINAL.html
```

Final HTML can be opened locally in any browser and printed to PDF from there.

### Step 6 — Design-system backfill (optional)

If a new visual pattern emerged during this chart (a new node shape, connector
style, badge variant), the cluster session will stage a `DESIGN-COMPONENT` or
`DESIGN-TOKEN-CHANGE` draft to `.agent/drafts-outbound/` and route it to
`project-design`. This happens automatically at session shutdown when new
patterns are identified. No action required from the operator.

---

## 4. Corporate entity codes

| Short code | Full name |
|---|---|
| `MCorp` | Woodfine Management Corp. |
| `WCPI` | Woodfine Capital Projects Inc. |
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
| `inputs/` | Operator's source uploads | Operator writes; cluster session reads |
| `working/` | Cluster session in-progress drafts | Cluster session writes; operator reads |
| `outputs/` | Final approved artifacts | Cluster session writes; operator pulls |
| `inputs/current-org-chart-html/` | Latest source HTML files | Operator's primary upload folder |

All paths are relative to the private org-chart deployment instance root.

---

*Authored 2026-06-05 by totebox@project-orgcharts.
Source authority: `inputs/README.md` (naming convention) + `MANIFEST.md` (deployment layout).*
