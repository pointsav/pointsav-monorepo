---
schema: foundry-draft-v1
artifact: DESIGN-TOKEN-CHANGE
language_protocol: DESIGN-TOKEN-CHANGE
state: draft-pending-master-cosign
originating_cluster: project-workplace
created: 2026-06-02
to: project-design
master_cosign: command@claude-code 2026-06-02
research_trail:
  source_files:
    - app-workplace-http-prototype/src/assets/style.css
  commit: 6ae5e97c
  downstream_impact: >
    Tokens added to style.css only (prototype). Not yet in pointsav-design-system.
    Downstream: app-workplace-* Tauri Wave 1 apps will consume these tokens at
    native-parity pass. No existing consumers to migrate. Zero migration burden.
---

# DESIGN-TOKEN-CHANGE — wp-tokens — 2026-06-02

**Route:** project-design → `pointsav-design-system`
- DTCG JSON patch: `tokens/dtcg-bundle.json` (new `workplace` namespace)
- Research file: `dtcg-vault/research/workplace-tokens-2026-06-02.md`

**Blocked on:** `master_cosign` from Command Session (mandatory per token-intake-checklist.md
before committing to `pointsav-design-system`).

---

## Token namespace: `--wp-*`

The `--wp-*` CSS custom property namespace was introduced 2026-06-02 in
`app-workplace-http-prototype/src/assets/style.css` (commit `6ae5e97c`). It defines the
complete design token foundation for the `app-workplace-*` product family.

Current scope: HTTP prototype only. Native-parity pass for Tauri Wave 1 apps
(`app-workplace-memo`, `app-workplace-presentation`, `app-workplace-workbench`) will
consume these tokens directly.

---

## 16-token palette

### Background scale

```css
--wp-bg-base:    #1a1a1a;   /* page/window base */
--wp-bg-surface: #222222;   /* panel/card surface */
--wp-bg-raised:  #2a2a2a;   /* elevated panel */
--wp-bg-overlay: #333333;   /* overlay / hover state */
```

### Text scale

```css
--wp-text-primary:   #e6e4dc;  /* primary content */
--wp-text-secondary: #a09e98;  /* labels, captions */
--wp-text-muted:     #6a6866;  /* placeholders, disabled */
--wp-text-inverse:   #1a1a1a;  /* text on accent backgrounds */
```

### Accent — graphite bronze

```css
--wp-accent:       #c89a4a;              /* primary accent */
--wp-accent-hover: #d4aa5a;              /* hover state */
--wp-accent-muted: rgba(200,154,74,0.15); /* tinted background */
```

`--wp-accent: #c89a4a` replaces the VS Code derivative `#007acc` blue. Bronze is warm,
material, and professional — appropriate for construction and AEC work. It is visually
distinct from developer tooling (blue/purple) and productivity suites (blue/green).

### Semantic tokens

```css
--wp-border:  rgba(255,255,255,0.08);  /* dividers, outlines */
--wp-success: #4caf50;                 /* save confirmation, valid state */
--wp-danger:  #f44336;                 /* destructive action, error state */
--wp-warning: #ff9800;                 /* caution, non-blocking issue */
```

---

## 7-step spacing scale (base-4)

```css
--wp-space-1: 4px;
--wp-space-2: 8px;
--wp-space-3: 12px;
--wp-space-4: 16px;
--wp-space-5: 24px;
--wp-space-6: 32px;
--wp-space-7: 48px;
```

Base unit is 4px; scale follows 1×, 2×, 3×, 4×, 6×, 8×, 12× multiples. Covers
all common layout distances in a professional desktop application without fractional values.

---

## 6-step type scale (13px base, ratio ≈1.125)

```css
--wp-text-xs:   11px;
--wp-text-sm:   12px;
--wp-text-base: 13px;
--wp-text-md:   14px;
--wp-text-lg:   16px;
--wp-text-xl:   18px;
```

13px base matches VS Code and most professional desktop IDEs — the established legibility
floor for information-dense tooling interfaces. The scale provides adequate differentiation
between label, body, and heading sizes without requiring large display sizes.

---

## 6-level z-index map

```css
--wp-z-base:    0;
--wp-z-raised:  100;
--wp-z-overlay: 500;
--wp-z-modal:   1000;
--wp-z-toast:   2000;
--wp-z-top:     3000;
```

Six named levels prevent z-index collision across independently developed surface modules.
Gaps are intentional: surface-specific internal stacking can use integers between levels.

---

## Button system — `.wp-btn`

```css
.wp-btn            /* base: padding, border-radius, cursor, font */
.wp-btn--primary   /* background: var(--wp-accent); color: var(--wp-text-inverse) */
.wp-btn--ghost     /* background: transparent; border: 1px solid var(--wp-border) */
.wp-btn--danger    /* background: var(--wp-danger) */
```

Replaces four previously duplicated button class definitions scattered across surface HTML
files. All surfaces consume `.wp-btn` from `style.css`.

---

## Bronze rail signature

The active-surface indicator in the workbench uses:

```css
box-shadow: inset 0 3px 0 0 var(--wp-accent);
```

This "bronze rail" is the primary visual affordance distinguishing the active surface
from inactive surfaces in the workbench toolbar. It replaces the VS Code-derivative
`background: #094771` active state with a top-inset shadow in the accent colour,
which is lighter (doesn't fill the entire button background) and consistent with the
graphite bronze identity.

---

## DTCG JSON patch (for project-design to write)

The following DTCG-format token additions belong in `tokens/dtcg-bundle.json` under a
new `workplace` top-level group, or as a standalone `tokens/workplace.tokens.json` file:

```json
{
  "workplace": {
    "bg": {
      "base":    { "$value": "#1a1a1a", "$type": "color" },
      "surface": { "$value": "#222222", "$type": "color" },
      "raised":  { "$value": "#2a2a2a", "$type": "color" },
      "overlay": { "$value": "#333333", "$type": "color" }
    },
    "text": {
      "primary":   { "$value": "#e6e4dc", "$type": "color" },
      "secondary": { "$value": "#a09e98", "$type": "color" },
      "muted":     { "$value": "#6a6866", "$type": "color" },
      "inverse":   { "$value": "#1a1a1a", "$type": "color" }
    },
    "accent": {
      "default": { "$value": "#c89a4a", "$type": "color" },
      "hover":   { "$value": "#d4aa5a", "$type": "color" },
      "muted":   { "$value": "rgba(200,154,74,0.15)", "$type": "color" }
    },
    "semantic": {
      "border":  { "$value": "rgba(255,255,255,0.08)", "$type": "color" },
      "success": { "$value": "#4caf50", "$type": "color" },
      "danger":  { "$value": "#f44336", "$type": "color" },
      "warning": { "$value": "#ff9800", "$type": "color" }
    },
    "space": {
      "1": { "$value": "4px",  "$type": "dimension" },
      "2": { "$value": "8px",  "$type": "dimension" },
      "3": { "$value": "12px", "$type": "dimension" },
      "4": { "$value": "16px", "$type": "dimension" },
      "5": { "$value": "24px", "$type": "dimension" },
      "6": { "$value": "32px", "$type": "dimension" },
      "7": { "$value": "48px", "$type": "dimension" }
    },
    "text-size": {
      "xs":   { "$value": "11px", "$type": "dimension" },
      "sm":   { "$value": "12px", "$type": "dimension" },
      "base": { "$value": "13px", "$type": "dimension" },
      "md":   { "$value": "14px", "$type": "dimension" },
      "lg":   { "$value": "16px", "$type": "dimension" },
      "xl":   { "$value": "18px", "$type": "dimension" }
    },
    "z": {
      "base":    { "$value": "0",    "$type": "number" },
      "raised":  { "$value": "100",  "$type": "number" },
      "overlay": { "$value": "500",  "$type": "number" },
      "modal":   { "$value": "1000", "$type": "number" },
      "toast":   { "$value": "2000", "$type": "number" },
      "top":     { "$value": "3000", "$type": "number" }
    }
  }
}
```
