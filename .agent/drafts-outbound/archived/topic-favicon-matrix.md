---
schema: foundry-draft-v1
state: draft-pending-editorial-pass
originating_cluster: project-design
source_repo: pointsav-media-assets (root — misplaced; removed 2026-05-07)
target_repo: content-wiki-documentation
target_path: content/governance/ (suggested)
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: TOPIC
authored: 2026-04-20T00:00:00Z
authored_by: ps-administrator
notes_for_editor: |
  This TOPIC file was misplaced in the root of pointsav-media-assets.
  Content is technically accurate but uses emoji headers and non-Bloomberg
  prose style. Needs editorial pass (banned-vocab cleanup, bilingual pair,
  Bloomberg standard) before routing to content-wiki-documentation.
  Content is short — straightforward editorial pass.
---

# TOPIC: Favicon Matrix & Tab Identity

The PointSav OS utilizes high-fidelity SVG data URIs for browser tab identification.

## Engineering rationale

By embedding the SVG directly into the `<link rel="icon">` header as a URL-encoded string:

1. Zero HTTP requests — eliminates a network call to the server for an icon file.
2. Infinite scaling — vector math ensures sharp rendering on high-DPI displays.

## Entity dichotomy

- Vendor (PointSav): Steel-Blue Square (`#869FB9`). Represents the infrastructure layer.
- Customer (Woodfine): Woodfine Blue Circle (`#164679`). Represents the operating enterprise.
