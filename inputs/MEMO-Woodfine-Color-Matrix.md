# MEMO 04: The Woodfine Color Matrix

**TARGET AUDIENCE:** Creative Design Contributors
**IDENTITY:** Institutional Private Equity & Real Estate Promoters

## 1. Visual Doctrine

Woodfine Capital Projects utilizes a high-trust, traditional institutional aesthetic. The palette is designed to signal multi-generational stability, fiduciary duty, and legacy asset management. Dark Mode is forbidden; the interface must replicate physical paper and premium corporate reports.

## 2. Surface tokens

* **Canvas (Background Outer):** `#F7F9FA` (Surgical off-white to eliminate screen glare).
* **Card (Document Background):** `#FFFFFF` (Pure White).
* **Typography (Primary):** `#111827` (Deep Slate). Pure black `#000000` is forbidden to prevent halation eye-strain.
* **Asset:** `Woodfine_Logo_Large.png` / `Woodfine_Logo_Small.png` (Requires `#FFFFFF` contrast background).

## 3. Brand palette

| Token | Hex | Tint (10% wash) | Role |
|---|---|---|---|
| `--wf-blue` | `#164679` | `#E8EFF7` | Accent & interactive; counsel / governance domain accent. **Primary brand colour.** |
| `--wf-green` | `#54924E` | `#EEF6EC` | Accounting / finance / operations domain accent. Tints to `--accent-2 #6BAE64` (+8% lightness) where a softer green is needed. |
| `--wf-orange` | `#F15F22` | `#FDE8DD` | Highlight / cross-cutting flag (e.g., Financial Officer asterisk, "Consolidation Recognition" callouts). |
| `--wf-red` | `#ED1B2F` | — | Reserved for alerts / required action. Use sparingly; never as a domain accent. |
| `--wf-purple` | `#7C468C` | `#EEE6F1` | Cross-border / regional / non-resident entity marker (e.g., MCorp Delaware in a consolidation chart). |
| `--wf-gold` | `#F57F17` | `#FFFDE7` | Investment-vehicle marker (e.g., GP / Administrator, Direct-Hold Solutions container). The established standard across the legacy chart corpus (Mar 28 – Apr 6 batch); the Apr-6 Canva-rendered Accounting Statements chart drifted to `#C89211` and is corrected on its re-author. |
| `--wf-black` | `#000000` | — | **Forbidden for typography.** Available only for use where pure black is structurally required (e.g., a hairline rule that must reproduce identically in print). Default to `--ink` (`#111827`) for all body text. |
| `--wf-grey` | `#E6E7E8` | — | Neutral fill for non-semantic boxes (placeholder TitleCos, undefined entities). |

## 4. Neutral scale (typography & rules)

| Token | Hex | Role |
|---|---|---|
| `--ink` | `#111827` | Body text (Deep Slate). The substitute for `#000000` per §1. |
| `--ink-2` | `#374151` | Secondary text — duties bullets, sub-labels. |
| `--ink-3` | `#6B7280` | Tertiary text — section labels, footnotes, kicker on tinted bars. |
| `--ink-4` | `#9CA3AF` | Quaternary text — entity tags (CA / US prefixes), disabled states. |
| `--paper` | `#FFFFFF` | Document background. |
| `--paper-2` | `#F7F9FA` | Outer canvas (per §2). |
| `--paper-3` | `#E6E7E8` | Tertiary surface, equal to `--wf-grey`. |
| `--rule` | `#E6E7E8` | Hairline separator (0.5 px). |
| `--rule-strong` | `#9CA3AF` | Bold separator (1 px). |

## 5. Per-chart accent rule

Each chart binds `--accent` to one of the brand-palette colours according to its domain. The accent then drives the kicker, the section-numeral colour, the strong-box fill, and the per-discipline header fill via the cascade.

| Domain | `--accent` | `--accent-tint` | Notes |
|---|---|---|---|
| Counsel / governance / corporate registration | `--wf-blue` | `#E8EFF7` | Default for counsel / legal / authority charts. |
| Accounting / finance / operations | `--wf-green` | `#EEF6EC` | Default for accounting / treasury / consolidation charts. |
| Cross-border / regional notation | `--wf-purple` | `#EEE6F1` | Use as a per-entity marker, not a chart-wide accent. |
| Investment-vehicle marker | `--wf-gold` | `#FFFDE7` | Use as a per-entity marker, not a chart-wide accent. |
| Highlight / cross-cutting callout | `--wf-orange` | `#FDE8DD` | Always paired with the chart's main accent; never the chart-wide accent on its own. |
| Alert / required action | `--wf-red` | — | Reserved; not used as accent on production charts. |

A chart should bind exactly one of `--wf-blue` or `--wf-green` to its `--accent`. The other palette colours are domain markers used inside the chart for specific entities (purple for non-domestic, gold for investment vehicles, orange for highlights). This rule is what makes the JW14 Accounting and Counsel charts a deliberate template series — single-line accent swap (`#54924E` → `#164679`) flips green-themed Accounting into blue-themed Counsel without any other change.

## 6. Boundaries and additions

The palette in §3 is closed for new top-level brand colours; any addition (new domain, new marker) returns through this MEMO before reaching production. Additions do not retroactively re-theme existing charts; they apply forward only.

Tints are deliberately uniform at ~10% wash. Custom tints belong in the chart that needs them (declared as a local CSS variable inside that chart's `<style>` block, not as a new global token). A bespoke tint that recurs across two or more charts is a candidate for promotion into this MEMO.

## 7. Provenance

Section 2 (surface tokens) is the original 2026 visual doctrine. Sections 3–5 were ratified 2026-04-28 from the Group B canonical chart series authored under the project-orgcharts cluster — specifically the Apr 15 JW14 Accounting / Counsel pair and the Apr 6 JW9 Accounting Statements consolidation chart. Those three charts established the brand palette and the per-chart accent-swap pattern that this MEMO now codifies as the canonical reference for all future Woodfine corporate chart authoring.

The `--wf-gold` value (`#F57F17 / #FFFDE7`) was reconciled against the established 9-chart majority across the legacy corpus: the Apr-6 JW9 Accounting Statements chart used a different gold (`#C89211 / #FAEFCC`) due to its Canva-template origin, while every other chart depicting Direct-Hold pills (JW7 Management Extended, JW9 + JW11 Access Fund Internal/External, JW9 Woodfine-Group, JW10 Mexico, JW13 Canada, JW19 Cross-Border-2, JW21 Cross-Border, JW15/JW18/JW29 Transactions) used `#F57F17 / #FFFDE7`. The 9-chart majority is taken as canon; the JW9 Canva default is corrected on re-author.
