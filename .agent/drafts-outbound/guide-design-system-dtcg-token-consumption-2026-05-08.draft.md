---
schema: foundry-draft-v1
state: draft-pending-language-pass
language_protocol: PROSE-GUIDE
originating_cluster: project-design
target_repo: woodfine/woodfine-fleet-deployment
target_path: <tbd-by-project-editorial>
target_filename: guide-design-system-dtcg-token-consumption.md
audience: customer-public
bcsc_class: current-fact
authored: 2026-05-08T00:00:00Z
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 3
research_suggested_count: 1
open_questions_count: 0
research_provenance: tacit
research_inline: true
notes_for_editor: |
  Vault stub is live at https://design.pointsav.com/developing/dtcg-tokens/.
  Language pass: Bloomberg standard. Toolchain examples (style-dictionary,
  token-transformer) are correct as of 2026-05; verify package names and APIs
  have not changed before publishing. The /api/tokens/{theme}.dtcg.json endpoint
  is declared in vault.rs but should be verified live before this guide ships.
---

## Research trail

### Done — what informed this draft
- [tacit: vault stub developing/dtcg-tokens.md] — skeleton content
- [tacit: vault.rs build_tokens_bundle()] — how the bundle is assembled
  (primitive layer + theme layer merged under "primitive" and "theme" keys)
- [external: w3c.github.io/design-tokens/] — DTCG `$value`, `$type`, `$description`
  field semantics (tacit recall; W3C spec confirmed in prior session)

### Suggested — what project-editorial should consult
- [external: styledictionary.amzn.github.io] — verify style-dictionary config
  for DTCG format (medium priority; toolchain version may have changed)

---

# DTCG Token Consumption

The substrate exposes design tokens in W3C Design Tokens Community Group (DTCG)
format. This guide covers the available endpoints, the structure of the token
bundle, and how to consume tokens in a build toolchain.

---

## The DTCG format

DTCG is a W3C specification for expressing design tokens as a portable JSON
structure. Each token is an object with three standard fields:

- `$value` — the resolved value (`"#234ed8"`, `"1rem"`, `"250ms"`)
- `$type` — the semantic type (`"color"`, `"dimension"`, `"duration"`)
- `$description` — optional human-readable explanation

Tokens are organised into a nested object hierarchy, with path segments acting as
a namespace. A token at `color.primary-60.$value` is accessed by traversing the
`color` → `primary-60` → `$value` path.

The two-layer model used here:

**Primitive layer** — raw named values. Every colour, spacing step, timing value,
and radius appears once with a stable name and a direct value. No aliases.

**Semantic layer** — named roles that alias into the primitive layer. The semantic
token `interactive.primary.$value` is a reference like `{color.primary-60}` rather
than a hex literal. Tools that resolve aliases (Tokens Studio, style-dictionary)
follow the reference to find the raw value.

---

## Endpoints

### Full bundle

```
GET /tokens.json
```

Returns the complete token bundle as a JSON object with two top-level keys:

```json
{
  "primitive": { ... },
  "theme":     { ... }
}
```

`primitive` contains the raw value layer. `theme` contains semantic overrides for
the active tenant. Both layers must be consumed together to resolve all aliases.

Inspect the live bundle:

```bash
curl -sS https://design.pointsav.com/tokens.json | jq 'keys'
# ["primitive", "theme"]

curl -sS https://design.pointsav.com/tokens.json \
  | jq '.primitive.color | keys'
```

### Per-theme typed bundle

```
GET /api/tokens/{theme}.dtcg.json
```

Returns a typed DTCG bundle scoped to one theme. The `{theme}` parameter is the
filename in `vault/themes/` without the `.json` extension. For the vendor instance:
`pointsav-brand`.

### shadcn registry

```
GET /r/registry.json
```

A shadcn-compatible component index. Individual component entries are at
`/r/{component}`. See the [shadcn registry guide](/developing/shadcn-registry/)
for how to consume this in a code generator.

---

## CSS custom properties

Each primitive token maps to a CSS custom property loaded on every page of the
substrate. The naming convention is `--ps-{path-segments-joined-by-hyphen}`:

```css
--ps-primary-60: #234ed8;
--ps-neutral-10: #f5f6f8;
--ps-space-5:    1.25rem;
--ps-speed-2:    250ms;
```

The full variable set is visible in the page source of any substrate page.
Components in the recipe layer reference these custom properties rather than raw
values; changing a primitive token propagates to all components without a rebuild.

---

## Build toolchain integration

### style-dictionary

style-dictionary reads a DTCG-formatted source and outputs platform-specific files
(CSS, SCSS, iOS Swift, Android XML, etc.).

Fetch the token bundle and write it to a local file:

```bash
curl -sS https://design.pointsav.com/tokens.json > tokens/tokens.json
```

Configure style-dictionary to read from the `primitive` key:

```javascript
// style-dictionary.config.js
module.exports = {
  source: ['tokens/tokens.json'],
  platforms: {
    css: {
      transformGroup: 'css',
      prefix: 'ps',
      buildPath: 'dist/',
      files: [{ destination: 'tokens.css', format: 'css/variables' }],
    },
  },
};
```

### Token-transformer (Tokens Studio)

If you sync tokens from your design editor via Tokens Studio, the output is
already in DTCG format. Pass it through `token-transformer` to resolve aliases
before feeding to style-dictionary:

```bash
npx token-transformer tokens/tokens.json tokens/tokens-flat.json
```

---

## Alias resolution

When a tool encounters a value like `{color.primary-60}`, it must traverse the
primitive layer to find the resolved value. Tools that do not resolve aliases will
emit the reference string literally — verify your toolchain handles DTCG aliases
before production use.

The substrate resolves aliases server-side when building the CSS custom property
declarations. If you are building a static export, the alias resolution step
belongs in your build pipeline.
