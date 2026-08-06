// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.


// P0.2 — visual token gallery. Flattens tokens.full.json's primitive/theme/extension
// tiers into renderable entries: color tokens get a swatch + contrast ratio, everything
// else gets a plain name/value row.
//
// Phase 5 fix (2026-07-04): this previously read `tokens/dtcg-bundle.json` — a
// knowledge-wiki-specific additive bundle (own $description: "PointSav design system —
// knowledge-wiki baseline... Adds knowledge-wiki primitives"), master-cosigned for that
// product, not this substrate's own generic primitives. `sync-design-tokens.sh` explicitly
// SKIPs dtcg-bundle.json from its canonical merge step — it was never meant to be the
// gallery's source. The actual merged, canonical bundle lives at `exports/tokens.full.json`.
// Its `primitive` tier's `primary-60: #234ed8` matches the ratified PointSav blue from
// `primitive.json` / `pointsav-brand.json` — confirming this is the correct source.
//
// Correction (2026-07-16 token-completeness audit): the line above previously called
// `tokens.full.json` "sync-design-tokens.sh's own output" — false. That script only
// COPIES this already-hand-edited file between deploy locations; nothing regenerates
// its `primitive`/`theme`/`paper`/`writing` tiers from the real granular source files
// (`primitive.json`, `pointsav-brand.json`, `paper/*`, `writing/*`). The real generator
// is `pointsav-design-system/bin/generate-tokens-export.py` (added same day, closes this
// gap) — run it after editing any source file, before committing `tokens.full.json`.
// Correction (2026-08-02 registry-reconciliation pass): the tier list below was missing
// "wcp" (the finance/wcp pillar, 25 tokens). Every count derived from this function had
// silently undercounted by exactly 25 (586 vs. 611) since wcp was added, and the gallery
// page never rendered those tokens at all. Added "wcp" below to fix both.
// Cleanup (2026-08-04, content audit): the tier list previously also carried two
// tier names retired in the 2026-08-02 de-branding pass, no longer present in
// `tokens.full.json`. The lookup below already skips any tier absent from the data,
// so this was dead code, not a live bug; removed for good since a stale tier name
// has zero purpose once the tier itself is gone.
// Retirement (2026-08-04, later same day): "wcp" removed again -- not a stale-name
// cleanup this time, the pillar itself was deleted. It was always a pure alias
// layer over paper.semantic.financial-report-layout.*, never consumed by any real
// code (confirmed against project-proforma, its originating archive), and the
// gallery never resolved DTCG aliases before rendering, so its 25 entries only ever
// showed as broken/empty rows. See pointsav-design-system's finance.tokens.json
// deletion commit for the full rationale.
use serde_json::Value;
use std::path::Path;

#[derive(serde::Serialize)]
pub struct TokenEntry {
    pub path: String,
    pub css_var: String,
    pub value: String,
    pub kind: String,
    pub description: Option<String>,
    pub contrast_vs_white: Option<String>,
}

#[derive(serde::Serialize)]
pub struct TokenGroup {
    pub name: String,
    pub entries: Vec<TokenEntry>,
}

#[derive(serde::Serialize)]
pub struct TokenTier {
    pub name: String,
    pub groups: Vec<TokenGroup>,
}

pub fn load_and_flatten(vault: &Path) -> Vec<TokenTier> {
    let path = vault.join("exports").join("tokens.full.json");
    let Ok(raw) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    let Ok(Value::Object(root)) = serde_json::from_str::<Value>(&raw) else {
        return Vec::new();
    };

    let mut tiers = Vec::new();
    for tier_name in ["primitive", "theme", "paper", "writing"] {
        let Some(Value::Object(tier_map)) = root.get(tier_name) else {
            continue;
        };
        let mut groups = Vec::new();
        for (group_name, group_val) in tier_map {
            if group_name.starts_with('$') {
                continue;
            }
            let mut entries = Vec::new();
            // Fable audit finding (2026-08-02): this used to seed the path with
            // `group_name` alone for every tier, silently dropping `tier_name` --
            // e.g. a paper-tier leaf got path "primitive.color.ink" instead of
            // "paper.primitive.color.ink". That broke the homepage Paper/Writing
            // preview (find_val() calls below use the real tier-inclusive paths, which
            // could never match), MCP alias resolution (196 of 248 real aliases are
            // tier-inclusive, e.g. "{paper.primitive.page.margin-standard}"), and
            // renames.json's path-alias branch. Only the "primitive" tier is meant to
            // stay bare/group-relative: generate-tokens-export.py's own alias_index
            // deliberately special-cases pillar=="primitive" to register a bare-path
            // alias alongside the full one (since every other tier's tokens reference
            // primitive values without the "primitive." prefix, e.g. theme's
            // "{color.neutral-100}") -- no such special case exists for any other
            // tier, so every other tier's real path includes its own tier name.
            let seed_path = if tier_name == "primitive" {
                group_name.clone()
            } else {
                format!("{tier_name}.{group_name}")
            };
            flatten(group_val, seed_path, None, &mut entries);
            if !entries.is_empty() {
                groups.push(TokenGroup {
                    name: group_name.clone(),
                    entries,
                });
            }
        }
        if !groups.is_empty() {
            tiers.push(TokenTier {
                name: tier_name.to_string(),
                groups,
            });
        }
    }
    tiers
}

fn flatten(val: &Value, path: String, inherited_type: Option<String>, out: &mut Vec<TokenEntry>) {
    let Value::Object(map) = val else {
        return;
    };
    let own_type = map
        .get("$type")
        .and_then(|v| v.as_str())
        .map(String::from)
        .or(inherited_type.clone());

    if let Some(v) = map.get("$value") {
        let value = match v {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        };
        let description = map
            .get("$description")
            .and_then(|v| v.as_str())
            .map(String::from);
        let kind = own_type.unwrap_or_else(|| "unknown".to_string());
        let contrast_vs_white = if kind == "color" {
            contrast_ratio_vs_white(&value)
        } else {
            None
        };
        // Correction (2026-08-02 MCP functional-test finding): this used to reconstruct
        // css_var from the DTCG path (`--{path with dots as dashes}`), which does NOT
        // match the token's real canonical id -- the one actually emitted into
        // exports/tokens.css by generate-tokens-export.py, and the one get_token callers
        // see in real rendered CSS. A caller pasting the real variable name they see on
        // the page got a false "not found" from get_token, the opposite of what an
        // AI-agent-consumable registry is supposed to guarantee. Read the real canonical
        // id from $extensions when present; only fall back to the reconstructed form for
        // entries that somehow lack it (there shouldn't be any -- the generator's own
        // migrate-canonical-ids.py stamps every real leaf).
        let css_var = map
            .get("$extensions")
            .and_then(|e| e.get("com.pointsav.tokens"))
            .and_then(|t| t.get("id"))
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| format!("--{}", path.replace('.', "-")));
        out.push(TokenEntry {
            css_var,
            path,
            value,
            kind,
            description,
            contrast_vs_white,
        });
        return;
    }

    for (k, v) in map {
        if k.starts_with('$') {
            continue;
        }
        flatten(v, format!("{path}.{k}"), own_type.clone(), out);
    }
}

// Real count, not a hardcoded literal (2026-08-02 registry-reconciliation pass — the
// homepage previously hardcoded this as "6", stale against the real 10 families in
// `paper.semantic`). Counts top-level non-`$`-prefixed keys directly from the same
// `tokens.full.json` `load_and_flatten` already reads, so it can't drift independently.
pub fn paper_family_count(vault: &Path) -> usize {
    let path = vault.join("exports").join("tokens.full.json");
    let Ok(raw) = std::fs::read_to_string(&path) else {
        return 0;
    };
    let Ok(Value::Object(root)) = serde_json::from_str::<Value>(&raw) else {
        return 0;
    };
    let Some(Value::Object(paper)) = root.get("paper") else {
        return 0;
    };
    let Some(Value::Object(semantic)) = paper.get("semantic") else {
        return 0;
    };
    semantic.keys().filter(|k| !k.starts_with('$')).count()
}

fn contrast_ratio_vs_white(hex: &str) -> Option<String> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    // Fable-audit finding (2026-08-02): `hex.len()` is a byte count, but a `$value`
    // like "#aéaaa" is 6 bytes with a multi-byte char straddling index 2 -- the old
    // `&hex[0..2]` byte-slice would panic ("byte index 2 is not a char boundary") on
    // that input. `load_and_flatten()` runs on every page render, so one malformed
    // vault token would 500 every request. Vault content is operator-controlled, not
    // user input, but a typo is still one edit away; `.get()` returns `None` instead
    // of panicking on a bad boundary or bad UTF-8 slice, same as an unparseable hex
    // digit already does via `.ok()?` below.
    let r = u8::from_str_radix(hex.get(0..2)?, 16).ok()?;
    let g = u8::from_str_radix(hex.get(2..4)?, 16).ok()?;
    let b = u8::from_str_radix(hex.get(4..6)?, 16).ok()?;
    let lum = relative_luminance(r, g, b);
    let ratio = (1.0 + 0.05) / (lum + 0.05);
    Some(format!("{ratio:.2}:1"))
}

fn relative_luminance(r: u8, g: u8, b: u8) -> f64 {
    fn chan(c: u8) -> f64 {
        let c = c as f64 / 255.0;
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * chan(r) + 0.7152 * chan(g) + 0.0722 * chan(b)
}
