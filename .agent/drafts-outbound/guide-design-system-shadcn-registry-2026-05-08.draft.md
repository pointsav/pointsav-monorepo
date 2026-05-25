---
schema: foundry-draft-v1
state: draft-pending-language-pass
language_protocol: PROSE-GUIDE
originating_cluster: project-design
target_repo: woodfine/woodfine-fleet-deployment
target_path: <tbd-by-project-editorial>
target_filename: guide-design-system-shadcn-registry.md
audience: customer-public
bcsc_class: current-fact
authored: 2026-05-08T00:00:00Z
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 2
research_suggested_count: 1
open_questions_count: 0
research_provenance: tacit
research_inline: true
notes_for_editor: |
  Vault stub is live at https://design.pointsav.com/developing/shadcn-registry/.
  Language pass: Bloomberg standard. "shadcn" is a lowercase proper noun — do
  not capitalise to "Shadcn". The component table in this draft lists the 12
  components present in the vendor vault at time of authoring; the operator
  should refresh this table when new components ship. v0, Cursor, Windsurf are
  product names — no trademark markers needed in editorial context but verify
  current brand capitalisation before publishing.
---

## Research trail

### Done — what informed this draft
- [tacit: vault stub developing/shadcn-registry.md] — skeleton content
- [tacit: nav.rs components list] — 12 confirmed components: badge, breadcrumb,
  button, checkbox, input-text, link, navigation-bar, notification, select,
  surface, switch, tab

### Suggested — what project-editorial should consult
- [external: ui.shadcn.com/docs/registry] — verify the registry JSON schema has
  not changed; the substrate's /r/ endpoint must remain compatible (medium priority)

---

# shadcn Registry

The substrate exposes a shadcn-compatible component registry. Code generators
and AI agents can pull component HTML and CSS directly from the registry without
importing a package or reading documentation.

---

## Endpoints

### Registry index

```
GET /r/registry.json
```

Returns a JSON array of registry entries. Each entry includes the component name,
a short description, and a link to the full component endpoint.

### Individual component

```
GET /r/{component}
```

Returns the full recipe for one component: the HTML structure, CSS class
definitions, ARIA attributes, and token references. The `{component}` parameter
is the component slug (see the name table below).

---

## Usage with code generators

Most AI-assisted editors that support shadcn-style registries accept a registry
URL in their configuration. Point them at your instance's `/r/registry.json`.

**Cursor** — in Settings → AI → Component sources, add the URL of your instance's
`/r/registry.json`. Cursor will index the registry at session start and offer
components from the vault alongside its built-in library.

**Windsurf** — add the registry URL in the component configuration panel. Windsurf
resolves component requests against the registry when generating UI.

**Claude Code** — pass the registry URL in your project's MCP configuration or
reference it in your system prompt. The agent fetches `/r/{component}` when
generating code that requires a specific component.

**v0** — in the project settings, add the registry URL to the component source
list. v0 will prefer vault components over its defaults when the component name
matches.

---

## Manual installation

To pull a single component without a code generator:

```bash
curl -sS https://design.pointsav.com/r/button | jq '{html, css}'
```

Copy the `html` field into your template and the `css` field into your stylesheet.
The CSS uses `--ps-*` custom properties; include the substrate's CSS variable
declarations or provide your own values for the properties referenced.

---

## Component names

Component slugs match the sidebar navigation. The following components are
available in the current vendor vault:

| Slug | Display name |
|---|---|
| `badge` | Badge |
| `breadcrumb` | Breadcrumb |
| `button` | Button |
| `checkbox` | Checkbox |
| `input-text` | Input (text) |
| `link` | Link |
| `navigation-bar` | Navigation bar |
| `notification` | Notification |
| `select` | Select |
| `surface` | Surface |
| `switch` | Switch |
| `tab` | Tab |

This table reflects the vendor vault at the time this guide was authored. Forked
instances may carry additional or different components. Fetch `/r/registry.json`
on your instance to see the current list programmatically.
